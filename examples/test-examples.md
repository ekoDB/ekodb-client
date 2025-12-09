make test-examples 🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s Running
`target/debug/examples/simple_crud` ✓ Authentication successful

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

✅ All examples completed! ✅ [32mRust client examples complete![0m ✅ [32mAll
Rust integration tests complete![0m 🧪 [36mRunning Python examples (direct
HTTP/WebSocket)...[0m ✓ Authentication successful

=== Insert Document === Inserted: {'id':
'g37V-LwIE-D39vDQNH7q_TksP6sDufBFxhi1ECioxyR88XpT6QWqNCw1IWVX85pSVlCVUlgUpPI3dnV157xT7w'}

=== Find by ID === Found: {'value': {'value': 42, 'type': 'Integer'}, 'name':
{'type': 'String', 'value': 'Test Record'}, 'id':
'g37V-LwIE-D39vDQNH7q_TksP6sDufBFxhi1ECioxyR88XpT6QWqNCw1IWVX85pSVlCVUlgUpPI3dnV157xT7w',
'active': {'type': 'Boolean', 'value': True}}

=== Find with Query === Found documents: 1

=== Update Document === Updated: {'id':
'g37V-LwIE-D39vDQNH7q_TksP6sDufBFxhi1ECioxyR88XpT6QWqNCw1IWVX85pSVlCVUlgUpPI3dnV157xT7w',
'value': {'type': 'Integer', 'value': 100}, 'name': {'value': 'Updated Record',
'type': 'String'}, 'active': {'type': 'Boolean', 'value': True}}

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully ✓ Authentication successful

=== Inserting Test Data === ✓ Inserted test record:
p0WGdKAIQtu4s5ajCoE5oCqbhiPonpADBDtzNzVo2gEYwqeaFyLY3JmHjuNrJ0ywk4YSqv-4LoaE9RFYLVD3ww

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"ChsE4hhtNg8BkHvjsfyGVK_RCbVHQx8IaoOx1Vo0vx5l_0yuETorylmHzekR4I61txQOtNqmD1ntnmwC4zO4Nw",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"p0WGdKAIQtu4s5ajCoE5oCqbhiPonpADBDtzNzVo2gEYwqeaFyLY3JmHjuNrJ0ywk4YSqv-4LoaE9RFYLVD3ww",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } } ] }, "type": "Success" } ✓ Retrieved 2
record(s) via WebSocket

✓ WebSocket example completed successfully ✓ Authentication successful

=== Batch Insert === ✓ Batch inserted 5 records ✓ Verified: Found 10 total
records in collection

=== Creating test records for update/delete === Created 3 test records

=== Batch Update === ✓ Batch updated 3 records ✓ Verified: Record updated with
status="None"

=== Batch Delete === ✓ Batch deleted 3 records ✓ Verified: Records successfully
deleted (not found)

✓ All batch operations completed successfully ✓ Authentication successful

=== KV Set === ✓ Set key: session:user123

=== KV Get === Retrieved value: {'username': 'john_doe', 'userId': 123}

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: {'price': 29.99, 'name': 'Product 1'}
cache:product:2: {'price': 39.989999999999995, 'name': 'Product 2'}
cache:product:3: {'name': 'Product 3', 'price': 49.989999999999995}

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Authentication successful

=== Create Collection (via insert) === Collection created with first record:
TsOFnLpHldLLbxKFjoygZUcqyhT8ltw_ZFuuzuq6A6NLb96OWd4p69StAXq7Z-bx15F1P4O2_tjcXSscTQJygA

=== List Collections === Total collections: 11 Sample collections:
['test_collection', 'chat_configurations__ek0_bench', 'demo_collection',
'chat_messages__ek0_bench', 'ws_ttl_test']

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: False

✓ All collection management operations completed successfully ✓ Authentication
successful

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
6UIDPf2Z_63HksMFonfY3_sR8bN9fXpx-1NBbcgSg9fGSy-62xizTdFNP4jy3NbNKGvUqRInbdH0_eP_zGGKmA

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully ✓ Authentication successful

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
bnYTF9_MDGLwyhD3PuNL7uPnDXrEkhSmGsExG6rRjGPPowa7XRxQF9UGCIfbaxcz8LFay7IRED20_LFkI_zHCQ

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 2 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully 🚀 ekoDB Scripts Example
(Python/HTTP)

✓ Authentication successful

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved:
fWIApn-Dnv9HpF3G5VUZ6o1OxbrMHyHwA86wh0HGF4tD8fyPE_vWFhwJqnNZ6P67SuRatVMiX6L_9Z8Qcoa3jA
📊 Found 30 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved:
SVJ2gSFfaIyFE5xeql2qqNrOwh4_iOuzelDTv2Ryfm_jvYwbzjrfqhOvLPU7hRW7wSNEkG7P6IoySBb61ItF1g
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved:
eURngPV3SI7qyakXhhH8du5dHzAJuctMRUbZyVQnCLYanuh7tK2_ZtI4mkGobaCV-GwrT5msizo2kO4MAzsHew
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups {'count':
15, 'max_score': 100, 'avg_score': 60.0, 'status': 'active'} {'max_score': 90,
'count': 15, 'status': 'inactive', 'avg_score': 50.0}

📝 Example 4: Function Management

📋 Total scripts: 7 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

✅ All examples completed! ✓ Authentication successful

📋 Getting original configuration... Original durable_operations: False

============================================================ 🔥 TEST 1: Original
Config (durable=False)
============================================================

=== Setup: Creating Test Accounts === Created Alice: $1000 - ID:
kGZ8ZwqW2pZR5qjVIJxN9yxEWVvrWoNwG2WEmtYzPMy06uNMCXvpH_2WflhpEqTzhKX0hJxKdqfy7mxbc8KvWg
Created Bob: $500 - ID:
DFqwBv9OYNLaZYClum3AK38UIWgL722Fw8gcy9-5SwSH_m_M-7bCESOeI5tXocW3KW23VQCcSNMKzdxtAUqYtA

=== Example 1: Begin Transaction === Transaction ID:
8cafb4a2-bbb5-4bc9-9502-6a213a1dc991

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: Active Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: ${'type': 'Integer', 'value': 800} Bob: ${'type':
'Integer', 'value': 700}

=== Example 5: Rollback === New transaction:
44d40a41-5298-40cc-9e90-5ad97b690de7 Updated Bob: $700 → $600 (in transaction) ✓
Transaction rolled back Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup === ✓ All transaction examples completed === Setup: Creating Test
Accounts === Created Alice: $1000 - ID:
MQ8DUqux8MwUeeURelb0W2N76pZEWUwCHBSgYqIrceNHnFpBYB7S6RL-\_7z6KosunnMwH07j_mdNCyturRS36A
Created Bob: $500 - ID:
8rgHgxVtQRIKupq4xZsdPttNYBgLHokMFX7G6A20X0FILWpVigZshQBvswsMBBWg7vknpJITY5Chf-9WNb5Qww

=== Example 1: Begin Transaction === Transaction ID:
d77248bf-79ac-4a7e-a6fb-a26bbb32df52

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: ${'type': 'Integer', 'value': 800} Bob: ${'type':
'Integer', 'value': 700}

=== Cleanup === ✓ Deleted test accounts

🔄 Switching to NON-DURABLE mode... ✓ Config updated: durable_operations=false

============================================================ 🔥 TEST 2:
Non-Durable Mode (durable=false)
============================================================

=== Setup: Creating Test Accounts === Created Alice: $1000 - ID:
wS5CMC4hZhyPjPyrIJcRV66TGvE74Aal0o_HErf8OckEKSdF5fJLvUpJ7B61PhyXH7i3ZUXo-jVR-g6KVM41fQ
Created Bob: $500 - ID:
mKnLbXH-1Fdq5RSCLvGK7cuq_RNzxL7cm4Wp-tyq7KT-wJEyPL2MThk9oZzYsR3lj5ZcaYJHagPWGuoUCCz0TA

=== Example 1: Begin Transaction === Transaction ID:
444194d3-1cbf-4b6e-bd1e-bd8c2beb14e9

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: Active Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: ${'value': 800, 'type': 'Integer'} Bob: ${'value':
700, 'type': 'Integer'}

=== Example 5: Rollback === New transaction:
29725547-2659-4dfc-bcc3-c4c3c935820f Updated Bob: $700 → $600 (in transaction) ✓
Transaction rolled back Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup === ✓ All transaction examples completed === Setup: Creating Test
Accounts === Created Alice: $1000 - ID:
R7Lm8mE4Mtol54_kX3_FphjQLJSUK4yg51pvPWJO6Ubo472XiZbRr_H2lWESHgtjTSdZLTvvz-rDk_KPiGTj7w
Created Bob: $500 - ID:
GXQHGhuwe9hqR-bsWYpKL0RQ0DFj0SRkF2P8_etGuf2i5x8AjmYHOr6xIfA0bVQY5CXV6cXnNaRuSHgKYNOkbw

=== Example 1: Begin Transaction === Transaction ID:
67d29b0f-5a9a-43b9-bfde-d53cccec9fa6

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: ${'type': 'Integer', 'value': 800} Bob: ${'type':
'Integer', 'value': 700}

=== Cleanup === ✓ Deleted test accounts

🔄 Switching to DURABLE mode... ✓ Config updated: durable_operations=true

============================================================ 🔥 TEST 3: Durable
Mode (durable=true) ============================================================

=== Setup: Creating Test Accounts === Created Alice: $1000 - ID:
cmC8f6gZKXDm2pwNw2jzy4LuuEfsfcGWo12mEGeOX-q6KAN6ZmWOR1d9W6lYwfpXSwzV29w9kPnMTbVesjQrpg
Created Bob: $500 - ID:
UOdhmHGU7dyVoLylfJOdRnd392rKfi-6gRrys2JxanszDTXbhxZnaulj8KicZ1uGqJr0V9I_NDA4psqCwVV5_Q

=== Example 1: Begin Transaction === Transaction ID:
fd4031b8-e673-4705-98e1-2df12d500ac6

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: Active Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: ${'type': 'Integer', 'value': 800} Bob: ${'type':
'Integer', 'value': 700}

=== Example 5: Rollback === New transaction:
2d72b69c-8da6-48f7-8d2b-64e6072e7c39 Updated Bob: $700 → $600 (in transaction) ✓
Transaction rolled back Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup === ✓ All transaction examples completed === Setup: Creating Test
Accounts === Created Alice: $1000 - ID:
Wq9Ty-v6LTne-ieYAI2Ac7wLU3HS9GPUViYvPHGbm3B5lrC014f4eKYNb8peLmr3-l1Qo9nKs0u_xM62ypveBg
Created Bob: $500 - ID:
9RxB0J8ZvaGh3hV94y42gDhZuSs6YmEWYTMrUGWy6Bw0IRUOrej6W0Em_2MgfAjJZCM7Ef3MvJodwHwsRaI63g

=== Example 1: Begin Transaction === Transaction ID:
4725a319-8905-4fff-a86b-2e297475a731

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: ${'value': 800, 'type': 'Integer'} Bob: ${'value':
700, 'type': 'Integer'}

=== Cleanup === ✓ Deleted test accounts

🔄 Restoring original configuration... ✓ Config restored:
durable_operations=False

============================================================ ✅ ALL TESTS
PASSED - Transactions successful
============================================================

# 🚀 ekoDB Complete CRUD Scripts Example

Demonstrates: • Insert + Verify (using Query) • Query + Update Status + Verify •
Query + Update Credits + Verify • Query Before Delete + Delete + Verify Gone

# Each Script shows Functions chaining with proper verification

============================================================ 📝 Script 1:
Insert + Verify ============================================================

1️⃣ Saving Script... ✅ Script saved:
SMO6yZ-GGUsb9VaJQJOkgLG9JesyyDAv3vYphWXDE1jXt1iWUshlRLEGECLao8Nf37U48QvjGeDE19Qsj1n6jg

2️⃣ Calling Script (Insert + Verify)... ✅ Script executed: 2 Functions ⏱️
Execution time: 0ms

3️⃣ Verification Results: ✅ Found 1 record(s) 📋 User ID:
lzdksKZsnQdMQcsGeOxEmtLC4du25MKLv-pB3Lyvto2xfWUXqMlHTbjNY59jswWWZ7UH4VmAq1VJVQI5dGbfKg
📋 Name: {'type': 'String', 'value': 'Alice Smith'} 📋 Email: {'value':
'alice@example.com', 'type': 'String'} 📋 Status: {'value': 'pending', 'type':
'String'} 📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================ 📝 Script 2:
Query + Update + Verify
============================================================

1️⃣ Saving Script... ✅ Script saved:
HRN6ZsjKiJ2kcUwojlgaEfzd5AzD-v-x6iRVABbmaiTh0Wm5Fyjem4_ITAfsQ9zQxjL4h4E1dywQ3SrqY3gYjQ

2️⃣ Calling Script (Query + Update + Verify)... ✅ Script executed: 3 Functions
⏱️ Execution time: 0ms

3️⃣ Verification Results: ✅ Found 1 record(s) 📋 Status updated to: {'type':
'String', 'value': 'active'} 📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================ 📝 Script 3:
Query + Update Credits + Verify
============================================================

1️⃣ Saving Script... ✅ Script saved:
7JHHSmPvEsLPvGNtLVfbiQY4T-EewxV6_2ryTngZ_9mOycZHabpE-aCbrm1U9iKFHxO6UyyAexC0buVg5wqssg

2️⃣ Calling Script (Query + Update Credits + Verify)... ✅ Script executed: 3
Functions ⏱️ Execution time: 0ms

3️⃣ Verification Results: ✅ Found 1 record(s) 📋 Credits updated to: {'value':
0, 'type': 'Integer'} 📋 Status: {'type': 'String', 'value': 'active'} 📋 Name:
{'type': 'String', 'value': 'Alice Smith'}

============================================================ 📝 Script 4: Query
Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script... ✅ Script saved:
8vUuVAzws5SA5m6bsaoYyaujFiRh5bXgbl4S-YW4xAGmMBsgRZzW6MBS5AflrqNLZh_ZosPjvUNwoXlGtvaQQg

2️⃣ Calling Script (Query + Delete + Verify)... ✅ Script executed: 3 Functions
⏱️ Execution time: 0ms

3️⃣ Verification Results: 📊 Before delete: Record existed (verified by first
Query) ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================ 🧹 Cleanup
============================================================ ✅ Deleted script:
SMO6yZ-GGUsb9VaJQJOk... ✅ Deleted script: HRN6ZsjKiJ2kcUwojlga... ✅ Deleted
script: 7JHHSmPvEsLPvGNtLVfb... ✅ Deleted script: 8vUuVAzws5SA5m6bsaoY... ✅
Deleted collection: users

============================================================ ✅ Complete CRUD
Scripts Example Finished!
============================================================

💡 Key Takeaways: ✅ Scripts chain Functions together ✅ Each Script
demonstrates operation + verification ✅ Parameters make Scripts reusable ✅
Verification is built into the Script itself ✅ Complete CRUD lifecycle in 4
focused Scripts [34m ╔════════════════════════════════════════╗[0m [34m║ ekoDB
Python Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m [34m === Checking Server
Connection ===[0m [32m✓ Server is ready[0m [34m === Getting Authentication Token
===[0m [32m✓ Authentication successful[0m [33m === Running 10 Examples ===[0m
[34m === Running simple_crud.py ===[0m [32m✓ simple_crud.py completed
successfully[0m [34m === Running simple_websocket.py ===[0m [32m✓
simple_websocket.py completed successfully[0m [34m === Running
batch_operations.py ===[0m [32m✓ batch_operations.py completed successfully[0m
[34m === Running kv_operations.py ===[0m [32m✓ kv_operations.py completed
successfully[0m [34m === Running collection_management.py ===[0m [32m✓
collection_management.py completed successfully[0m [34m === Running
document_ttl.py ===[0m [32m✓ document_ttl.py completed successfully[0m [34m ===
Running websocket_ttl.py ===[0m [32m✓ websocket_ttl.py completed successfully[0m
[34m === Running http_functions.py ===[0m [32m✓ http_functions.py completed
successfully[0m [34m === Running transactions.py ===[0m [32m✓ transactions.py
completed successfully[0m [34m === Running crud_scripts.py ===[0m [32m✓
crud_scripts.py completed successfully[0m [34m
╔════════════════════════════════════════╗[0m [34m║ Test Summary ║[0m
[34m╚════════════════════════════════════════╝[0m [34mTotal: 10[0m [32mPassed:
10[0m [32mFailed: 0[0m ✅ [32mPython direct examples complete![0m 🐍
[36mBuilding Python client package...[0m 🍹 Building a mixed python/rust project
🔗 Found pyo3 bindings with abi3 support 🐍 Not using a specific python
interpreter 💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by
default Finished `release` profile [optimized] target(s) in 0.09s 📦 Built wheel
for abi3 Python ≥ 3.8 to
/Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.3.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m Processing
./ekodb-client-py/target/wheels/ekodb_client-0.3.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client Attempting uninstall: ekodb-client
Found existing installation: ekodb_client 0.3.0 Uninstalling ekodb_client-0.3.0:
Successfully uninstalled ekodb_client-0.3.0 Successfully installed
ekodb-client-0.3.0 ✅ [32mPython client package built and installed![0m 🧪
[36mRunning Python client library examples...[0m ✓ Client created (token
exchange happens automatically)

=== Insert Document === Inserted: {'id':
'0mJiOObBA7_QjVOJ3oqjV6odAmEfMIsDuL1c1Iezrvlccx7MD1XRqK8SDABIs9ZJLoRzKy3fkB9sz_s8pWIOFg'}

=== Find by ID === Found: {'active': {'type': 'Boolean', 'value': True}, 'id':
'0mJiOObBA7_QjVOJ3oqjV6odAmEfMIsDuL1c1Iezrvlccx7MD1XRqK8SDABIs9ZJLoRzKy3fkB9sz_s8pWIOFg',
'name': {'value': 'Test Record', 'type': 'String'}, 'value': {'type': 'Integer',
'value': 42}}

=== Find with Query === Found documents: 1

=== Update Document === Updated: {'name': {'type': 'String', 'value': 'Updated
Record'}, 'value': {'type': 'Integer', 'value': 100}, 'active': {'value': True,
'type': 'Boolean'}, 'id':
'0mJiOObBA7_QjVOJ3oqjV6odAmEfMIsDuL1c1Iezrvlccx7MD1XRqK8SDABIs9ZJLoRzKy3fkB9sz_s8pWIOFg'}

=== Delete Document === Deleted document

=== Cleanup === ✓ Deleted collection

✓ All CRUD operations completed successfully ✓ Client created

=== Batch Insert === ✓ Batch inserted 5 records ✓ Verified: Found 5 total
records in collection

=== Batch Update === ✓ Batch updated 3 records

=== Batch Delete === ✓ Batch deleted 3 records

=== Cleanup === ✓ Deleted collection

✓ All batch operations completed successfully ✓ Client created

=== KV Set === ✓ Set key: session:user123

=== KV Get === Retrieved value: {'value':
'{"userId":123,"username":"john_doe"}'}

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: {'value': '{"name":"Product
1","price":29.99}'} cache:product:2: {'value': '{"name":"Product
2","price":39.989999999999995}'} cache:product:3: {'value': '{"name":"Product
3","price":49.989999999999995}'}

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
"PlrAHMapjVapOkUY8aVp6Kwhg1oaWwvj3yFyKdwrj6cs4xyMngmzirqLrJ-WMrnjrYRq0DoyWPmMivEGagLrqg"

=== List Collections === Total collections: 10 Sample collections:
['test_collection', 'chat_configurations__ek0_bench',
'chat_messages__ek0_bench', 'ws_ttl_test', 'ttl_cache']

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: False

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
6x45LnlGQ3IpwXHVGOuCpFEd5eegrSYKH02Cc-4EqqAqdZcy-N2hwGO4HnBChuwVExys6uRyGcQRy2ijR9zs4g

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
CNQtHMvJZjrYYlipqsH299sRuaKh-xSvYPjwoafKnPldlQzNZ1BWBOUZIs-Di1mTZ5gJMuFfer9Q88-Xf-HC7g

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
n1xNz7rB8SCMaamaCMM0xPG6JD9A9fwBApVEocGVLUpMn3dNlj1lr8AiSaRbTiZTzunIs8MseQBscCMqvetfPQ

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
vAAOMt0-K99WTtgGmhNf9sAlPxt8sJKMQVsWS9-uTkM6HuvVZkUEoXbzJeXyzC4DlpQqZxfaYLDrL2v6b40fAQ

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration === Query Builder Examples ===

Setting up test data... ✅ Test data created

1. Simple equality query: Found 2 active users

2. Range query with sorting: Found 3 users aged 18-65

3. String operations: Found 2 users with @example.com emails

4. IN operator: Found 2 privileged users

5. Complex query with multiple conditions: Found 1 active US users over 21

6. Pagination: Page 1: 2 users

7. NOT IN operator: Found 2 valid users

8. Using bypass flags: Found 2 users (bypassed cache)

=== Cleanup === ✅ Deleted test collection

✅ Query Builder examples completed! === Search Examples ===

Setting up test data... ✅ Test data created

1. Basic full-text search: Found 2 results
1. Score: 25.740, Matched: email, name, email.value, name.value
1. Score: 12.540, Matched: name.value, name

1. Fuzzy search (typo tolerance): Found 4 results with fuzzy matching
1. Score: 26.400, Matched: title.value, bio, title, bio.value
1. Score: 26.400, Matched: title.value, bio, title, bio.value
1. Score: 26.400, Matched: bio, bio.value, title, title.value
1. Score: 26.400, Matched: bio, bio.value, title, title.value

1. Search with field weights: Found 4 results with weighted fields
1. Score: 39.600, Matched: title, bio, bio.value, title.value
1. Score: 39.600, Matched: bio.value, bio, title, title.value
1. Score: 39.600, Matched: bio, title, bio.value, title.value
1. Score: 39.600, Matched: bio.value, title.value, title, bio

1. Search with minimum score threshold: Found 2 results with score >= 0.3
1. Score: 13.200, Matched: bio, bio.value
1. Score: 13.200, Matched: bio, bio.value

1. Search with stemming and exact match boosting: Found 2 results (matches: run,
   running, runs)
1. Score: 13.200, Matched: bio.value, bio
1. Score: 13.200, Matched: bio.value, bio

1. Vector search (semantic search): Found 3 semantically similar documents
1. Score: 0.761
1. Score: 0.744
1. Score: 0.727

1. Hybrid search (text + vector): Found 3 results using hybrid search (text +
   vector)
1. Score: 1.709, Matched: content, title, title.value, content.value
1. Score: 1.121, Matched: content, content.value, title.value, title
1. Score: 0.533, Matched:

1. Case-sensitive search: Found 1 results (case-sensitive)
1. Score: 13.200, Matched: title.value, title

=== Cleanup === ✅ Deleted test collections

✅ Search examples completed! === Schema Management Examples ===

1. Creating user schema with basic fields: ✅ User schema created

2. Creating product schema with text index: ✅ Product schema with indexes
   created

3. Creating document schema with vector index: ✅ Document schema with vector
   index created

4. Retrieving collection schema: Schema fields: ['age', 'email', 'name',
   'status'] Schema version: 1

5. Retrieving collection metadata: Collection has 4 fields

6. Creating employee schema with all constraint types: ✅ Employee schema with
   all constraints created

✅ Schema management examples completed! === Join Operations Examples ===

Setting up sample data... ✅ Sample data created

1. Single collection join (users with departments): Found 2 users with
   department data:

- Bob Smith: Sales
- Alice Johnson: Engineering

2. Join with filtering: Found 1 users in Engineering:

- Alice Johnson: Building A

3. Join with user profiles: Found 2 users with profile data:

- Bob Smith: Sales Manager
- Alice Johnson: Senior Software Engineer

4. Join orders with user data: Found 2 completed orders:

- Mouse ($25) by Alice Johnson
- Laptop ($1200) by Alice Johnson

5. Complex join with multiple conditions: Found 2 users with example.com emails:

- Alice Johnson (alice@example.com): Building A
- Bob Smith (bob@example.com): Building B

=== Cleanup === ✅ Deleted test collections

✅ Join operations examples completed! === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
3A1UH696f5qnpJrqcP7cydw8RytBVZgW2EDAYDoXRrH67_UAfeowZ81CpQI1Yw53fBOly5lNPvxJdQCq9e4CWg

=== Sending Chat Message === Message ID:
fkIvaqkYU5XWnJoa-Wfj2z0jvHfpgqmzVN7ED5FfRPUVVPfxjmfo-No1xoOQ0OMS04y4uJ-v_SRRbDZlkoWxeQ

=== AI Response === I'm sorry, but the context provided does not include
specific product names or their prices. Could you please provide more
information or a different context?

Execution Time: 1396ms

=== Token Usage === Prompt tokens: 314 Completion tokens: 29 Total tokens: 343

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
1UuS6qtnGEBHlOxafBToV60ep85KytJDuw0GFHNxBGABOD450flTEEhQrbZ_j0TEgP8LxGmfoB_EL2KxTqShMQ

=== Sending Messages === ✓ Message 1 sent Response: I'm sorry, but the context
provided does not include specific information about any available products. Can
you provide more details or clarify your request?

✓ Message 2 sent Response: Apologies, but I can't provide the information you're
looking for because the context does not mention any specific product or its
price. Could you please specify the product you're interested in?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
L7MaATzIiWydZp3Z4ltujVgD_qKK2hWjOmFdZGj8kwXRTtRWTWOKARsoRcTsCkAAnT1JqYCZ-x8Z26xl5kOwuw
Parent:
1UuS6qtnGEBHlOxafBToV60ep85KytJDuw0GFHNxBGABOD450flTEEhQrbZ_j0TEgP8LxGmfoB_EL2KxTqShMQ

=== Listing Sessions === ✓ Found 6 sessions Session 1:
L7MaATzIiWydZp3Z4ltujVgD_qKK2hWjOmFdZGj8kwXRTtRWTWOKARsoRcTsCkAAnT1JqYCZ-x8Z26xl5kOwuw
(Untitled) Session 2:
1UuS6qtnGEBHlOxafBToV60ep85KytJDuw0GFHNxBGABOD450flTEEhQrbZ_j0TEgP8LxGmfoB_EL2KxTqShMQ
(Untitled) Session 3:
3A1UH696f5qnpJrqcP7cydw8RytBVZgW2EDAYDoXRrH67_UAfeowZ81CpQI1Yw53fBOly5lNPvxJdQCq9e4CWg
(Untitled) Session 4:
Y9oeEuI0Dmbz-mFP_WrCEsZDbKYoKGSpek0DIqaEDWl5ia3L5pUftCBVIKBw4dtX5sMlhoqqhoK5Dj-TltDRgg
(Untitled) Session 5:
Ols-WO2aHQOC7MeahkVVK-T9Tc-ADE9GzB9ZhrbriEtpGVorP3vLjzQyHBQ_eURfu6VZqSeNJB5cO1Ge2zKbGg
(Untitled) Session 6:
pya1NJ3sd_plhrdNkxjV2UR64G7fwYPWoMMYOLD9nKmXIYUpiHjhWAWx9vDsNgnjfiUhdknf53UZWjg0-GZM8Q
(Untitled)

=== Deleting Branch Session === ✓ Deleted branch session:
L7MaATzIiWydZp3Z4ltujVgD_qKK2hWjOmFdZGj8kwXRTtRWTWOKARsoRcTsCkAAnT1JqYCZ-x8Z26xl5kOwuw

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
3EDVfbbWHgsPJN_XjedWrLfWpxxvu_ERmBn6mGon1k2HGx5KKYdtyyyjtiI-J45RZLOLTioqYwwpVGEqgnaONg

=== Sending Initial Message === ✓ Message sent Response: I'm sorry, but the
context provided doesn't include specific information about any available
products. Could you please provide more details or specific criteria you're
looking for in a product?

✓ Second message sent

Debug: Found 4 messages Debug: First message keys: dict_keys(['role',
'token_usage', 'id', 'chat_id', 'updated_at', 'created_at', 'content']) Debug:
First message role: {'value': 'assistant', 'type': 'String'} === Feature 1:
Regenerate AI Response === ✓ Message regenerated New response: Answer: I'm
sorry, but the context provided does not include information on the price of
ekoDB. I recommend checking the product's official website or contacting their
customer service for the most accurate and up-to-date pricing information.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
Cba_JGR3M-Emd84AjyvP6hxbHaewbJeQAOh-i20QRBPQACBK8Irp8dY1A6TPc5I9I56o0Qa25EiDAL1kGr_vXQ
✓ Sent message in second session ✓ Sessions merged successfully Total messages
in merged session: 4

=== Feature 5: Delete Message === ✓ Message deleted

✓ Messages remaining: 3

=== Cleanup === ✓ Deleted session ✓ Deleted collection

✓ All advanced chat features demonstrated successfully! 🚀 ekoDB Scripts Example
(Python)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved:
jU5BZeOaQRYJACIQ_5EXwytkFCU_6jEIjMxoza6egUOyKbFyxgOMF0mypdzZ1_foOI_RJ6h4lA96N1FXHFjMmw
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved:
xXddHwoNkWJu5VfOsLl4W8-DSBRyzTQQKM4pkP-gdhFxxKwTQWXhkjJC0dDl1sCRpp0S2uISe3GaqaAZ-L7Eyg
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved:
qT6LvSgG9Tbc8bPCrN3svW-9U1IvPok5AW0IKvSN2pbGGge2C6eKld0b0PVkIlhycwOKCx3EQd0Qvti3Vx-EWg
📊 Statistics: 2 groups {'avg_score': 60.0, 'count': 5, 'status': 'active'}

{'avg_score': 50.0, 'count': 5, 'status': 'inactive'}

📝 Example 4: Script Management

📋 Total scripts: 9 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed! 🚀 ekoDB Python Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data... ✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved:
zVgS42drtKzMvKQ0dejWfQ7pMfzd0o1_6PbE_pCfVDdRZwFuxPc1mSXUn73qfkW97F-vBS5WU15mMyF5osZrqw
📊 Found 2 product groups {'avg_price': 474.0, 'category': 'Furniture', 'count':
2} {'avg_price': 575.6666666666666, 'category': 'Electronics', 'count': 3} ⏱️
Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved 📊 Found 5 products ⏱️ Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved 📊 Found 2 categories {'category': 'Electronics', 'count': 3}
{'category': 'Furniture', 'count': 2} ⏱️ Execution time: 0ms

📝 Example 4: High Rating Products

✅ Script saved 📊 Found 5 products ⏱️ Execution time: 0ms

📝 Example 5: Script with Parameter Definition

✅ Script saved 📊 Found 5 products ⏱️ Execution time: 0ms

📝 Example 6: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Script saved 📊 Pipeline executed 3 stages ⏱️ Total execution time: 0ms

🧹 Cleaning up... ✅ Cleanup complete

✅ All complete script examples finished!

💡 This example demonstrates ekoDB's Script system: ✅ FindAll operations ✅
Group aggregations (Count, Average) ✅ Multi-stage pipelines (FindAll → Group →
Count) ✅ Parameter definitions ✅ Script management (save, call, delete) ✅
[32mPython client examples complete![0m ✅ [32mAll Python integration tests
complete![0m 🧪 [36mRunning Go examples (direct HTTP/WebSocket)...[0m [34m
╔════════════════════════════════════════╗[0m [34m║ ekoDB Go Examples Test Suite
║[0m [34m╚════════════════════════════════════════╝[0m [34m === Checking Server
Connection ===[0m [32m✓ Server is ready[0m [33m === Running 10 Examples ===[0m
[34m === Running simple_crud.go ===[0m ✓ Authentication successful

=== Insert Document === Inserted:
map[id:T95qIyv8edVjTioqNNyL25H3KP12M4OGGTr4uviLjz3mMiR1DxBe5IHcwQptQfx4wtJDTaMnoBV5N03IpRVqDQ]

=== Find by ID === Found: map[active:map[type:Boolean value:true]
id:T95qIyv8edVjTioqNNyL25H3KP12M4OGGTr4uviLjz3mMiR1DxBe5IHcwQptQfx4wtJDTaMnoBV5N03IpRVqDQ
name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query === Found 1 documents

=== Update Document === Updated: map[active:map[type:Boolean value:true]
id:T95qIyv8edVjTioqNNyL25H3KP12M4OGGTr4uviLjz3mMiR1DxBe5IHcwQptQfx4wtJDTaMnoBV5N03IpRVqDQ
name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully [32m✓ simple_crud.go completed
successfully[0m [34m === Running simple_websocket.go ===[0m ✓ Authentication
successful

=== Inserting Test Data === ✓ Inserted test record:
G2EZWyV1Cqti1eOKOJQtnPuxXRm0BHD5rChCO4pYktf6M8SCa1YvUpMpN3sRzJctGTa8GF9hzyPmiwJZfj0q_w

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"G2EZWyV1Cqti1eOKOJQtnPuxXRm0BHD5rChCO4pYktf6M8SCa1YvUpMpN3sRzJctGTa8GF9hzyPmiwJZfj0q_w",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"ChsE4hhtNg8BkHvjsfyGVK_RCbVHQx8IaoOx1Vo0vx5l_0yuETorylmHzekR4I61txQOtNqmD1ntnmwC4zO4Nw",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"p0WGdKAIQtu4s5ajCoE5oCqbhiPonpADBDtzNzVo2gEYwqeaFyLY3JmHjuNrJ0ywk4YSqv-4LoaE9RFYLVD3ww",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } } ] }, "type": "Success" } ✓ Retrieved 3
record(s) via WebSocket

✓ WebSocket example completed successfully [32m✓ simple_websocket.go completed
successfully[0m [34m === Running batch_operations.go ===[0m ✓ Authentication
successful

=== Batch Insert === Batch insert response: map[failed:[]
successful:[XzvLEtLFkD-ETKaWN6luyss6-bsTILoeL8hm2c0WUCCMKDAHVw-rHM9Hl7Jh8YqVu69u8yr-u-PylOE93D4wnQ
82OnQMjw_VDqkiFwV6Oc1B2-HvICGW3GeJ2nc2-h8Wdu9jvOvrnEHk6EkdLSpq7R32sabVcKrYTThph8X9LDww
pAl8yHgPztfkOgwo2pbtooMtfaKvV-lsERQxEBaPKdrU1rFe7Y6boRar7EAkmfvyR4i-cjTbrbrVIaERlsDJGg
Cl0AeYhUyeGqFEjRB0_b4ron-gv83-uqXwICYxVR8i3cottlE4NVYngl_PkP2vfzJ2Yu7RkTv4tP5R37BlX-3w
kVjCIvPKb2rO_vVEvjiguXFfA_o56aKpfwnrgPIhg2a-ZaBl8FdNa-rtMI3qAXfc-sqDoGyGxLf9WoxI-Wsudg]]
✓ Batch insert completed

=== Creating test records for update/delete === Created 3 test records

=== Batch Update === Batch update response: map[failed:[]
successful:[y8L6ucIYTojCnr0MdqOGvT5jVOoapu7yDtA0JVhMpxuSeik04SZkJbKqoJwhdfGhR8flUcNfHmV6LND2R2OCcw
9Mq4KQMDQ44BC9_BqylNvfThWGa30oZYTTqpdKlzi_ZT9eZdFvdoucksbQtWzpBSaIqtkx39ijD48ZT05JdXlA
LtXn6oql4KAdu7M2gWuBwaG4Sj6o5WqUznSwRX84WMZdY1clD3hkQIpchrIZx7QCsS9p35mAWefcxC4xyqcgag]]
✓ Batch update completed

=== Batch Delete === Batch delete response: map[failed:[]
successful:[y8L6ucIYTojCnr0MdqOGvT5jVOoapu7yDtA0JVhMpxuSeik04SZkJbKqoJwhdfGhR8flUcNfHmV6LND2R2OCcw
9Mq4KQMDQ44BC9_BqylNvfThWGa30oZYTTqpdKlzi_ZT9eZdFvdoucksbQtWzpBSaIqtkx39ijD48ZT05JdXlA
LtXn6oql4KAdu7M2gWuBwaG4Sj6o5WqUznSwRX84WMZdY1clD3hkQIpchrIZx7QCsS9p35mAWefcxC4xyqcgag]]
✓ Batch delete completed

✓ All batch operations completed successfully [32m✓ batch_operations.go
completed successfully[0m [34m === Running kv_operations.go ===[0m ✓
Authentication successful

=== KV Set === ✓ Set key: session:user123

=== KV Get === Retrieved value: map[userId:123 username:john_doe]

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: map[name:Product 1 price:29.99]
cache:product:2: map[name:Product 2 price:39.989999999999995] cache:product:3:
map[name:Product 3 price:49.989999999999995]

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully [32m✓ kv_operations.go completed
successfully[0m [34m === Running collection_management.go ===[0m ✓
Authentication successful

=== Create Collection (via insert) === Collection created with first record:
pCFdwpydWGMkXM93wqG_oOM4B25jvClHQV_R-8odatn5lK1V9pgLqB3Ea7OhBtJCCZ1yTuv_nyLIloK-XPQGQQ

=== List Collections === Total collections: 15 Sample collections:
[schema_users_client_py test_collection schema_employees_client_py
chat_configurations__ek0_bench demo_collection chat_messages__ek0_bench
ws_ttl_test schema_documents_client_py schema_products_client_py ttl_cache
websocket_test scripts__ek0_bench users test_accounts batch_users]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully [32m✓
collection_management.go completed successfully[0m [34m === Running
document_ttl.go ===[0m ✓ Authentication successful

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
5fiYJsNhrrMpWP2s70bFffLZAAZADdtbSDISIZRIhU3gSvnbazKZQmvUC1iauiLSkF0GB8mnc4lXEcGEKqNe4Q

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 3 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully [32m✓ document_ttl.go
completed successfully[0m [34m === Running websocket_ttl.go ===[0m ✓
Authentication successful

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
2tpliUqsn3hbbKsXrNbae33Dxnd28Wy-VeaWNuRYaL6N79WojWTBn-Xo8Xaoq4WFlVyrkLVz1pWXgcuZ5wvI4g

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 3 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully [32m✓ websocket_ttl.go completed
successfully[0m [34m === Running http_functions.go ===[0m 🚀 ekoDB Scripts
Example (Go/HTTP)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved:
46lR-JrTzivD6dVHbc611zwl06hcORIXqPRyN-Ui3yKZdSsU_u8r9ISk0jcrogLf77dZCIoEuCwfB_UK1Hea8Q
📊 Found 20 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved:
x-qnWD6p_yv9xeY0AcAtyMNGRmQ6nGhvdVOgo7WknWDQ_RcMOKWF_wLTCs88-RaHIWYMrcVkeCYcAH7Uo-VYFg
📊 Page 1: Found 3 users (limit=3, skip=0) 📊 Page 2: Found 3 users (limit=3,
skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved:
LcX4wusSKDq2EQAxcXi0jRRW6IrzKK5yKMMjxWZDdwWK8P-1K2XqqDc7Vh8sZAjW8kJWjwTkVDV8jyXiwH_L6A
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
{"avg_score":60,"count":10,"max_score":100,"status":"active"}
{"avg_score":50,"count":10,"max_score":90,"status":"inactive"}

📝 Example 4: Function Management

📋 Total scripts: 11 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

✅ All examples completed! [32m✓ http_functions.go completed successfully[0m
[34m === Running transactions.go ===[0m ✓ Authentication successful

=== Setup: Creating Test Accounts === Created Alice: $1000 - ID:
ULqhackg0-4B36Q3JKTnKZUUM5e9SmOVwwwn1FJncLcSIpe7k5oCtwRvG-KTXJdRkVSDkOhG9SuPWc7VMWUw6w
Created Bob: $500 - ID:
hSMvp2VbsfyNlrhOi621VNV417fphwl-Oj1dpxQfgBr2bglQ6WLbUueYY31tDS77jQRIPS2YGzFuig0Rrxy87A

=== Example 1: Begin Transaction === Transaction ID:
560c1d10-76ab-4a74-af6a-86b727bb7f3c

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: Active Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: map[type:Integer value:800] Bob: map[type:Integer
value:700]

=== Example 5: Rollback === New transaction:
6c284b56-1526-4ce8-a310-9d07ceda219e Updated Bob: $700 → $600 (in transaction) ✓
Transaction rolled back Bob after rollback: map[type:Integer value:700]

=== Cleanup === ✓ Deleted test accounts

✓ All transaction examples completed [32m✓ transactions.go completed
successfully[0m [34m === Running crud_scripts.go ===[0m 🚀 ekoDB Complete CRUD
Scripts Example ============================================================
Demonstrates: • Insert + Verify (using Query) • Query + Update Status + Verify •
Query + Update Credits + Verify • Query Before Delete + Delete + Verify Gone

# Each Script shows Functions chaining with proper verification

============================================================ 📝 Script 1:
Insert + Verify ============================================================

1️⃣ Saving Script... ✅ Script saved:
17Ijex6FKRz3xDx2G_ttoZ6P7YLYUTfDtBqaNs-i4448Mi7m8sj43giI4NpQPx5MunPm2PyookcBusEd6yrl8Q

2️⃣ Calling Script (Insert + Verify)... ✅ Script executed: 2 Functions ⏱️
Execution time: 0ms

3️⃣ Verification Results: ✅ Found 1 record(s) 📋 User ID:
zMwoCiQtXsLoHC1_Pqkr_4Uc7HAez1Zmj8xBm7eBWnoDb3jpvJykniMJg6AsAaSslPZTDWAjVits8oVaUXiI5A
📋 Name: map[type:String value:Alice Smith] 📋 Email: map[type:String
value:alice@example.com] 📋 Status: map[type:String value:pending] 📋 Credits:
map[type:Integer value:0]

============================================================ 📝 Script 2:
Query + Update + Verify
============================================================

1️⃣ Saving Script... ✅ Script saved:
urcgVTjXpWAQz0WrZOnNJ8kRs7GGntbQFTRJ_eYsgenGcnu4nURYlkRoahvPC2UAxN4_nMNDvHKUModfjrpqyQ

2️⃣ Calling Script (Query + Update + Verify)... ✅ Script executed: 3 Functions
⏱️ Execution time: 0ms

3️⃣ Verification Results: ✅ Found 1 record(s) 📋 Status updated to:
map[type:String value:active] 📋 Name: map[type:String value:Alice Smith]

============================================================ 📝 Script 3:
Query + Update Credits + Verify
============================================================

1️⃣ Saving Script... ✅ Script saved:
tHz1VFjh0_Ny5Fp89IlujIlKLo7blRViJ6rVcvYTA_RVmnV8HYfQnb2ZSYqt9E-JOg2WwiDvzcPe_P2-rJE0Rg

2️⃣ Calling Script (Query + Update Credits + Verify)... ✅ Script executed: 3
Functions ⏱️ Execution time: 0ms

3️⃣ Verification Results: ✅ Found 1 record(s) 📋 Credits updated to:
map[type:Integer value:0] 📋 Status: map[type:String value:active] 📋 Name:
map[type:String value:Alice Smith]

============================================================ 📝 Script 4: Query
Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script... ✅ Script saved:
rcDQW6nVXhRQccMOD-LomIKBhSMqtv_uWDiDBTyOBhRbkXnYzsWlWqV-AK4TDKzsWYHssfE2AtdaX9Y4SBUkSw

2️⃣ Calling Script (Query + Delete + Verify)... ✅ Script executed: 3 Functions
⏱️ Execution time: 0ms

3️⃣ Verification Results: 📊 Before delete: Record existed (verified by first
Query) ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================ 🧹 Cleanup
============================================================ ✅ Deleted script:
17Ijex6FKRz3xDx2G_tt... ✅ Deleted script: urcgVTjXpWAQz0WrZOnN... ✅ Deleted
script: tHz1VFjh0_Ny5Fp89Ilu... ✅ Deleted script: rcDQW6nVXhRQccMOD-Lo... ✅
Deleted collection: users

============================================================ ✅ Complete CRUD
Scripts Example Finished!
============================================================

💡 Key Takeaways: ✅ Scripts chain Functions together ✅ Each Script
demonstrates operation + verification ✅ Parameters make Scripts reusable ✅
Verification is built into the Script itself ✅ Complete CRUD lifecycle in 4
focused Scripts [32m✓ crud_scripts.go completed successfully[0m [34m
╔════════════════════════════════════════╗[0m [34m║ Test Summary ║[0m
[34m╚════════════════════════════════════════╝[0m [34mTotal: 10[0m [32mPassed:
10[0m [32mFailed: 0[0m ✅ [32mGo direct examples complete![0m 🧪 [36mRunning Go
client library examples...[0m ✓ Client created (token exchange happens
automatically)

=== Insert Document === Inserted:
map[id:1pu1uYPXZHMOygMSY1Jji9XETeCgvsly6lGlccLv6FylRjYGQDES8wE6wa_cQxFF8QaNPHnAQ7LPf5k0HOf4gA]

=== Find by ID === Found: map[active:map[type:Boolean value:true]
id:1pu1uYPXZHMOygMSY1Jji9XETeCgvsly6lGlccLv6FylRjYGQDES8wE6wa_cQxFF8QaNPHnAQ7LPf5k0HOf4gA
name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query === Found documents: 1

=== Update Document === Updated: map[active:map[type:Boolean value:true]
id:1pu1uYPXZHMOygMSY1Jji9XETeCgvsly6lGlccLv6FylRjYGQDES8wE6wa_cQxFF8QaNPHnAQ7LPf5k0HOf4gA
name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document === Deleted document

=== Cleanup === ✓ Deleted collection

✓ All CRUD operations completed successfully ✓ Client created

=== Batch Insert === ✓ Batch inserted 5 records ✓ Verified: Found 5 total
records in collection

=== Batch Update === ✓ Batch updated 3 records

=== Batch Delete === ✓ Batch deleted 3 records

=== Cleanup === ✓ Deleted collection

✓ All batch operations completed successfully ✓ Client created

=== KV Set === ✓ Set key: session:user123

=== KV Get === Retrieved value: map[userId:123 username:john_doe]

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: map[name:Product 1 price:29.99]
cache:product:2: map[name:Product 2 price:39.989999999999995] cache:product:3:
map[name:Product 3 price:49.989999999999995]

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
byfPbsXUJsOPktvTwbKLzrinbo2AGMs9sQzN9dth67tqViAVTccQNP-RYYmyDL0qI3xMzO1CB58LmAUaTPvozQ

=== List Collections === Total collections: 14 Sample collections:
[schema_users_client_py test_collection schema_employees_client_py
chat_configurations__ek0_bench chat_messages__ek0_bench]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
HH7Tt8eTeH31Ca7nFoSZeX9MFxxrj0Z1o9anhRDXEy959n9X_wzGqjd9VQ3iCirgMVWY2vbL6Tsb0BNnbA987g

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
Kh66pNx98uTYkkTER1a3a7Csxnf9cyWI4J8vBBAbuUWSuopOPMFG8DlvCrRjNeOHNuJqhdrfWT86xJxEKWfaeQ

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
0szuOjU564mG7FNVH2B0enwrIiO8sbM_762EBVeOU_bbqc7uNhNEoi-QCur6oCioL--qSb7DtXmEQdbCeIumgA

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
5kTcvbQ5sDY7C-c5Lhi4uyezTwp7nQuA1Ed51VYxWfQyXXhU4-4keLoYYhe-P-pefzPMQnxlroQVnxhi6zKUCg

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
o5YWLghBe-uU6P90uarQF7o_A3m8Nfh1xrlX2qTK7f1VPTkTotbZc-1gxSPn36OqJiIh4jVLqfaPsgfU9S0cLg

=== Sending Chat Message === Message ID:
HS8otHnBjt7Hwm2MUkMzlFsD8eUVomLc8dKZInG8KVteIDwQgoUK9KyY2lUMTOFZ0ZfXHEZifwqacCDiwPDy1A

=== AI Response === I'm sorry, but the provided context does not include
specific product names or their prices. Could you please provide more detailed
information?

=== Context Used (3 snippets) === Snippet 1: map[collection:client_chat_basic_go
matched_fields:[description] record:map[description:map[type:String
value:Enterprise edition product with advanced features]
id:fNxvIwF896cjwOolZRTLCJOXr0NCMgIG-LajrZAzCsTwreCmdUpA2_1DtiGDSvouP2xltU1_w1F_YjHzmaqWYw
name:map[type:String value:ekoDB Pro] price:map[type:Integer value:299]]
score:0.1111111111111111] Snippet 2: map[collection:client_chat_basic_go
matched_fields:[description] record:map[description:map[type:String value:A
high-performance database product with AI capabilities]
id:hZ1kuK6X0_IT-9nKmjF9X99dmV-zlvVkc3lF7RN8CsnYWB_rzMnJ7j_fpj6n1oJTrWTswoZ4ssY2MhaZ_u6R5w
name:map[type:String value:ekoDB] price:map[type:Integer value:99]]
score:0.1111111111111111] Snippet 3: map[collection:client_chat_basic_go
matched_fields:[description] record:map[description:map[type:String value:Fully
managed cloud database service product]
id:bx1-r-pBfliVNFGWwghRR6VhtEg6K4BzkxcncvdMkiWurziUbJ-RgrhjKUXzrYtNGr76TPpLl1Mwm4MvVK80Qg
name:map[type:String value:ekoDB Cloud] price:map[type:Integer value:499]]
score:0.1111111111111111]

Execution Time: 2792ms

=== Token Usage === Prompt tokens: 318 Completion tokens: 26 Total tokens: 344

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
BlUuNrxrPcg7DT7XH5RyCQlSIP904lBzfFMBc4clDioYQu4G-WqYuSz1vsRACAucqG9_n8ALXwofDGG2gtFfPQ

=== Sending Messages === ✓ Message 1 sent Response: I'm sorry, but the context
provided does not include specific information about any products. Could you
please provide more details or check the data?

✓ Message 2 sent Response: I'm sorry, but there's no information provided about
the price in the given context. Could you please specify the product you're
interested in?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
JR9hMLtXyybMfaee_8v1X788bNT7mGuiLLBBw_BiMp6160VsIz2iqZ3ysTOMFj22dAtNI9UMkh9oN6S9XzKzjA
Parent:
BlUuNrxrPcg7DT7XH5RyCQlSIP904lBzfFMBc4clDioYQu4G-WqYuSz1vsRACAucqG9_n8ALXwofDGG2gtFfPQ

=== Listing Sessions === ✓ Found 9 sessions Session 1:
JR9hMLtXyybMfaee_8v1X788bNT7mGuiLLBBw_BiMp6160VsIz2iqZ3ysTOMFj22dAtNI9UMkh9oN6S9XzKzjA
(Untitled) Session 2:
BlUuNrxrPcg7DT7XH5RyCQlSIP904lBzfFMBc4clDioYQu4G-WqYuSz1vsRACAucqG9_n8ALXwofDGG2gtFfPQ
(Untitled) Session 3:
o5YWLghBe-uU6P90uarQF7o_A3m8Nfh1xrlX2qTK7f1VPTkTotbZc-1gxSPn36OqJiIh4jVLqfaPsgfU9S0cLg
(Untitled) Session 4:
Cba_JGR3M-Emd84AjyvP6hxbHaewbJeQAOh-i20QRBPQACBK8Irp8dY1A6TPc5I9I56o0Qa25EiDAL1kGr_vXQ
(Untitled) Session 5:
1UuS6qtnGEBHlOxafBToV60ep85KytJDuw0GFHNxBGABOD450flTEEhQrbZ_j0TEgP8LxGmfoB_EL2KxTqShMQ
(Untitled) Session 6:
3A1UH696f5qnpJrqcP7cydw8RytBVZgW2EDAYDoXRrH67_UAfeowZ81CpQI1Yw53fBOly5lNPvxJdQCq9e4CWg
(Untitled) Session 7:
Y9oeEuI0Dmbz-mFP_WrCEsZDbKYoKGSpek0DIqaEDWl5ia3L5pUftCBVIKBw4dtX5sMlhoqqhoK5Dj-TltDRgg
(Untitled) Session 8:
Ols-WO2aHQOC7MeahkVVK-T9Tc-ADE9GzB9ZhrbriEtpGVorP3vLjzQyHBQ_eURfu6VZqSeNJB5cO1Ge2zKbGg
(Untitled) Session 9:
pya1NJ3sd_plhrdNkxjV2UR64G7fwYPWoMMYOLD9nKmXIYUpiHjhWAWx9vDsNgnjfiUhdknf53UZWjg0-GZM8Q
(Untitled)

=== Getting Session Details === ✓ Session details retrieved Messages: 4

=== Deleting Branch Session === ✓ Deleted branch session:
JR9hMLtXyybMfaee_8v1X788bNT7mGuiLLBBw_BiMp6160VsIz2iqZ3ysTOMFj22dAtNI9UMkh9oN6S9XzKzjA

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
AV-72n2n5RYSKuMQx5FRUibAwyfyTzWqHJqQmjMEkcqAkzh2bMNEhJ7pgiOmrJopv5mO25XOFkTw1F0N23drOQ

=== Sending Initial Message === ✓ Message sent Response: I'm sorry, but the
context provided does not contain information about any available products.
Could you please provide more details or another context?

✓ Second message sent

=== Feature 1: Regenerate AI Response === ✓ Message regenerated New response:
Answer: I'm sorry, but the context provided doesn't include information about
the price of ekoDB.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
lpIFDaYbl2PzBY3_7ZdF0F17T3jmH-N9SWDwgE-kGVnbK3WYJqXh90SMIs9iFktkkQx2yWwW29t6dOdULEkAVQ
✓ Sent message in second session ✓ Sessions merged successfully Total messages
in merged session: 4

=== Feature 5: Delete Message === ✓ Message deleted

✓ Messages remaining: 3

=== Cleanup === ✓ Deleted session ✓ Deleted collection

✓ All advanced chat features demonstrated successfully! === Query Builder
Examples ===

Setting up test data... ✅ Test data created

1. Simple equality query: Found 2 active users

2. Range query with sorting: Found 3 users aged 18-65

3. String operations: Found 2 users with @example.com emails

4. IN operator: Found 2 privileged users

5. Complex query with multiple conditions: Found 1 active US users over 21

6. Pagination: Page 1: 2 users

7. NOT IN operator: Found 3 valid users

8. Using bypass flags: Found 2 users (bypassed cache)

=== Cleanup === ✅ Deleted test collection

✅ Query Builder examples completed! === Search Examples ===

Setting up test data... ✅ Test data created

1. Basic full-text search: Found 2 results
1. Score: 25.740
1. Score: 12.540

1. Fuzzy search (typo tolerance): Found 4 results with fuzzy matching
1. Score: 26.400
1. Score: 26.400
1. Score: 26.400
1. Score: 26.400

1. Search with field weights: Found 4 results with weighted fields
1. Score: 39.600
1. Score: 39.600
1. Score: 39.600
1. Score: 39.600

1. Search with minimum score threshold: Found 2 results with score >= 0.3
1. Score: 13.200
1. Score: 13.200

1. Search with stemming and exact match boosting: Found 1 results (matches:
   work, working, worked)
1. Score: 13.200

1. Vector search (semantic search): Found 3 semantically similar documents
1. Score: 0.766
1. Score: 0.765
1. Score: 0.728

1. Hybrid search (text + vector): Found 3 results using hybrid search (text +
   vector)
1. Score: 2.691
1. Score: 1.506
1. Score: 0.306

1. Case-sensitive search: Found 1 results (case-sensitive)
1. Score: 26.400

=== Cleanup === ✅ Deleted test collections

✅ Search examples completed! === Schema Management Examples ===

1. Creating user schema with basic fields: ✅ User schema created

2. Creating product schema with text index: ✅ Product schema with indexes
   created

3. Creating document schema with vector index: ✅ Document schema with vector
   index created

4. Retrieving collection schema: Schema fields: 4 fields Schema version: 1

5. Retrieving collection metadata: Collection has 4 fields

6. Creating employee schema with all constraint types: ✅ Employee schema with
   all constraints created

✅ Schema management examples completed! === Join Operations Examples ===

Setting up sample data... ✅ Sample data created

1. Single collection join (users with departments): Found 2 users with
   department data

- Bob Smith: Sales
- Alice Johnson: Engineering

2. Join with filtering: Found 1 users in Engineering

- Alice Johnson: Building A

3. Join with user profiles: Found 2 users with profile data

- Bob Smith: Sales Manager
- Alice Johnson: Senior Software Engineer

4. Join orders with user data: Found 2 completed orders

- Mouse ($0) by Alice Johnson
- Laptop ($0) by Alice Johnson

5. Complex join with multiple conditions: Found 2 users with example.com emails

- Alice Johnson (alice@example.com): Building A
- Bob Smith (bob@example.com): Building B

=== Cleanup === ✅ Deleted test collections

✅ Join operations examples completed! 🚀 ekoDB Scripts Example (Go Client)

✅ Client initialized

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved:
GaWOjnjd-uRTGPNYP5zuV-WQcQu9_Gf0VEsFejAqqiL0uC3t6abeQltAit91rQamoL65_bIiyxrAlCKcmcPsGg
📊 Found 10 records ⏱️ Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved 📊 Found 10 users (limited) ⏱️ Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved 📊 Statistics: 2 groups map[avg_score:50 count:5
status:inactive] map[avg_score:60 count:5 status:active] ⏱️ Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 13 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

📝 Example 5: Multi-Stage Pipeline

✅ Multi-stage script saved 📊 Pipeline executed 2 stages ⏱️ Total execution
time: 0ms 📈 Stage breakdown:

📝 Example 6: Count Users

✅ Count script saved 📊 Total user count: 10 ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Deleted collection ✅ Deleted test scripts

✅ All examples completed successfully!

💡 Key Advantages of Using the Client: • Automatic token management • Type-safe
Stage builders • Built-in error handling ✅ [32mGo client examples complete![0m
✅ [32mAll Go integration tests complete![0m 📦 [36mBuilding TypeScript client
library...[0m

> @ekodb/ekodb-client@0.3.0 prepare npm run build

> @ekodb/ekodb-client@0.3.0 build tsc

up to date, audited 7 packages in 1s

found 0 vulnerabilities

> @ekodb/ekodb-client@0.3.0 build tsc

✅ [32mTypeScript client built![0m 🧪 [36mRunning TypeScript client library
examples...[0m

added 1 package, removed 1 package, and audited 13 packages in 654ms

3 packages are looking for funding run `npm fund` for details

found 0 vulnerabilities ✓ Client created (token exchange happens automatically)

=== Insert Document === Inserted: { id:
'zu-bx-dsnN87SMFoBFtB9zxnOjkBdHIxMDC4Rf8CHjRvDE9b2sFlN-KS8F1lHABxvZ2r6-AhcOMne03NkDf_KA'
}

=== Find by ID === Found: { value: { value: 42, type: 'Integer' }, id:
'zu-bx-dsnN87SMFoBFtB9zxnOjkBdHIxMDC4Rf8CHjRvDE9b2sFlN-KS8F1lHABxvZ2r6-AhcOMne03NkDf_KA',
active: { value: true, type: 'Boolean' }, name: { type: 'String', value: 'Test
Record' } }

=== Find with Query === Found documents: 1

=== Update Document === Updated: { name: { type: 'String', value: 'Updated
Record' }, id:
'zu-bx-dsnN87SMFoBFtB9zxnOjkBdHIxMDC4Rf8CHjRvDE9b2sFlN-KS8F1lHABxvZ2r6-AhcOMne03NkDf_KA',
active: { value: true, type: 'Boolean' }, value: { type: 'Integer', value: 100 }
}

=== Delete Document === Deleted document

=== Cleanup === ✓ Deleted collection

✓ All CRUD operations completed successfully ✓ Client created

=== Batch Insert === ✓ Batch inserted 5 records ✓ Verified: Found 5 total
records in collection

=== Batch Update === ✓ Batch updated 3 records

=== Batch Delete === ✓ Batch deleted 3 records

=== Cleanup === ✓ Deleted collection

✓ All batch operations completed successfully ✓ Client created

=== KV Set === ✓ Set key: session:user123

=== KV Get === Retrieved value: { userId: 123, username: 'john_doe' }

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: { price: 29.99, name: 'Product 1' }
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
edVt267TB09S3atm3PVFoeGHdnAI3eoMlElQCxp5gBIz7bAWZYM-LUCwnF8bjrQEIVnf0py5KC0AkBcEsxpObg

=== List Collections === Total collections: 18 Sample collections:
schema_users_client_py,schema_employees_client_py,test_collection,chat_configurations**ek0_bench,chat_messages**ek0_bench

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
IyDZXQemCyEhN8GXGdJRQTIJX7OCikRuTftLU4GXhdVa4r02JF1ZLsls1B7SsYpJiE1L7h8fmynLx7cCIKXrIg

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
V1e7PX33fcol4riEDY9_4dIQZPmcTx0JvmGtXyxJyP2KaikWP5uwrtWlI-lujq-oTvq7VKYaQ8I4WyJAyGr1ig

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
XgigwdqKDkzpbFre26Z9Dz7OEy0gUYsI-1eexgLUTfIY49aJQRHI1wGmx_ZnFRFI0mznQOovwUbKO8jbXRXMyw

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
0koT1SAbEioOUcPC-YPHD0lFhT78JwNM6t9VoTU1VktIIOKtTkzfnniQIe2eo-dDc9MlDlRmGLx0r0_9jgk_9w

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
X9rqox4icxg1eHIMX3le_ALFb5WkGaoLzeXuDub5dQmtts3RjPkPogfYABVPsPWsCosoNwlJqt-5qcVHKG63vA

=== Sending Chat Message === Message ID:
MiSbqRf7Q9wM44qR4JfqIjFZCzvL7cfRNLdD9YCyudrXHlOpGb21KGKuN6xc1jQbC3NkXGE_vBMYnPHvomiFHw

=== AI Response === I'm sorry, but the provided context does not include
specific product names or their prices. Could you please provide more
information or a more specific query?

=== Context Used (3 snippets) === Snippet 1: { collection:
'client_chat_basic_ts', record: { price: { type: 'Integer', value: 99 }, id:
'2FLvbZUDCwXQk3BBjvA4e8WysdQOiLB_OdcD_ZfFcU7TvmopbcRj4G0n_6zVu6ynSWWo7rmCvwS-a4HtNxlcvQ',
description: { value: 'A high-performance database product with AI
capabilities', type: 'String' }, name: { type: 'String', value: 'ekoDB' } },
score: 0.1111111111111111, matched_fields: [ 'description' ] } Snippet 2: {
collection: 'client_chat_basic_ts', record: { price: { type: 'Integer', value:
299 }, description: { type: 'String', value: 'Enterprise edition product with
advanced features' }, id:
'tbDiPHcHZk3aDesFih7mWEa1MAbHpjc60JJGq11SdXriTih_Swry2NnOBJ22ABifWHPyaZz-Y8TFKsVlZG17vw',
name: { value: 'ekoDB Pro', type: 'String' } }, score: 0.1111111111111111,
matched_fields: [ 'description' ] } Snippet 3: { collection:
'client_chat_basic_ts', record: { name: { value: 'ekoDB Cloud', type: 'String'
}, id:
'Nzbz4jaEmakggY6HVRofgOjoaHsUUbh6vxjU5DJyjz4AjCs3zagBoqpGEsUyt_l6Tnrs-SeI2NCoJP_3uSZt2Q',
description: { value: 'Fully managed cloud database service product', type:
'String' }, price: { type: 'Integer', value: 499 } }, score: 0.1111111111111111,
matched_fields: [ 'description' ] }

Execution Time: 1298ms

=== Token Usage === Prompt tokens: 315 Completion tokens: 30 Total tokens: 345

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
HuW1e8hufSqm7oAJcImllRsc-UCNwpEocV8vvtBEhsyOBC1t3rtGOxmi8L0f_m18yVEdyZiL6Fde73wHYGrgVA

=== Sending Messages === ✓ Message 1 sent Response: I'm sorry, but the provided
context does not contain any specific product information. Could you please
provide more details or a different context?

✓ Message 2 sent Response: Apologies, but the context provided doesn't include
information about the price of a product. Could you please provide more details
or specify the product you're inquiring about?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
RAu4uYfELQ0ynJEfZVwizs7opJD4HWx2hRjCjKEqhL7ChHVLaefQ0Em1n8EOe9Oy8AoemZK3Rue763a8mNe0lQ
Parent:
HuW1e8hufSqm7oAJcImllRsc-UCNwpEocV8vvtBEhsyOBC1t3rtGOxmi8L0f_m18yVEdyZiL6Fde73wHYGrgVA

=== Listing Sessions === ✓ Found 10 sessions Session 1:
RAu4uYfELQ0ynJEfZVwizs7opJD4HWx2hRjCjKEqhL7ChHVLaefQ0Em1n8EOe9Oy8AoemZK3Rue763a8mNe0lQ
(Untitled) Session 2:
HuW1e8hufSqm7oAJcImllRsc-UCNwpEocV8vvtBEhsyOBC1t3rtGOxmi8L0f_m18yVEdyZiL6Fde73wHYGrgVA
(Untitled) Session 3:
X9rqox4icxg1eHIMX3le_ALFb5WkGaoLzeXuDub5dQmtts3RjPkPogfYABVPsPWsCosoNwlJqt-5qcVHKG63vA
(Untitled) Session 4:
lpIFDaYbl2PzBY3_7ZdF0F17T3jmH-N9SWDwgE-kGVnbK3WYJqXh90SMIs9iFktkkQx2yWwW29t6dOdULEkAVQ
(Untitled) Session 5:
BlUuNrxrPcg7DT7XH5RyCQlSIP904lBzfFMBc4clDioYQu4G-WqYuSz1vsRACAucqG9_n8ALXwofDGG2gtFfPQ
(Untitled) Session 6:
o5YWLghBe-uU6P90uarQF7o_A3m8Nfh1xrlX2qTK7f1VPTkTotbZc-1gxSPn36OqJiIh4jVLqfaPsgfU9S0cLg
(Untitled) Session 7:
Cba_JGR3M-Emd84AjyvP6hxbHaewbJeQAOh-i20QRBPQACBK8Irp8dY1A6TPc5I9I56o0Qa25EiDAL1kGr_vXQ
(Untitled) Session 8:
1UuS6qtnGEBHlOxafBToV60ep85KytJDuw0GFHNxBGABOD450flTEEhQrbZ_j0TEgP8LxGmfoB_EL2KxTqShMQ
(Untitled) Session 9:
3A1UH696f5qnpJrqcP7cydw8RytBVZgW2EDAYDoXRrH67_UAfeowZ81CpQI1Yw53fBOly5lNPvxJdQCq9e4CWg
(Untitled) Session 10:
Y9oeEuI0Dmbz-mFP_WrCEsZDbKYoKGSpek0DIqaEDWl5ia3L5pUftCBVIKBw4dtX5sMlhoqqhoK5Dj-TltDRgg
(Untitled)

=== Getting Session Details === ✓ Session details retrieved Messages: 4

=== Deleting Branch Session === ✓ Deleted branch session:
RAu4uYfELQ0ynJEfZVwizs7opJD4HWx2hRjCjKEqhL7ChHVLaefQ0Em1n8EOe9Oy8AoemZK3Rue763a8mNe0lQ

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
YNYXc9UjaG2DyEHUK52J0UtomAq29sWZGDIi_dJgV8V9oGU5a5zND3FJnGYtrk11rZXdUrp3TAI2vJFiBGIISQ

=== Sending Initial Message === ✓ Message sent Response: Answer: The context
does not provide specific information about available products. Could you please
provide more details?

✓ Second message sent

=== Feature 1: Regenerate AI Response === ✓ Message regenerated New response:
Answer: I'm sorry, but the context provided doesn't include information about
the price of ekoDB.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
vJGopzlrJHB8R88ifgEYe4u4nCdsxLN-aOMSIq8krTjbnLqflnw7l7cSfxRVcvFc4W6t5yTf4ck9B3jW8kNcNw
✓ Sent message in second session ✓ Sessions merged successfully Total messages
in merged session: 4

=== Feature 5: Delete Message === ✓ Message deleted

✓ Messages remaining: 3

=== Cleanup === ✓ Deleted session ✓ Deleted collection

✓ All advanced chat features demonstrated successfully! === Query Builder
Examples ===

Setting up test data... ✅ Test data created

1. Simple equality query: Found 2 active users

2. Range query with sorting: Found 3 users aged 18-65

3. String operations: Found 2 users with @example.com emails

4. IN operator: Found 2 privileged users

5. Complex query with multiple conditions: Found 1 active US users over 21

6. Pagination: Page 1: 2 users

7. NOT IN operator: Found 3 valid users

8. Using bypass flags: Found 2 users (bypassed cache)

=== Cleanup === ✅ Deleted test collection

✅ Query Builder examples completed! === Search Examples ===

Setting up test data... ✅ Test data created

1. Basic full-text search: Found 2 results
1. Score: 25.740, Matched: name, email, email.value, name.value
1. Score: 12.540, Matched: name, name.value

1. Fuzzy search (typo tolerance): Found 4 results with fuzzy matching
1. Score: 26.400, Matched: title.value, title, bio.value, bio
1. Score: 26.400, Matched: title.value, bio.value, title, bio
1. Score: 26.400, Matched: bio, title.value, title, bio.value
1. Score: 26.400, Matched: bio.value, title.value, bio, title

1. Search with field weights: Found 4 results with weighted fields
1. Score: 39.600, Matched: bio, title, bio.value, title.value
1. Score: 39.600, Matched: bio.value, title.value, title, bio
1. Score: 39.600, Matched: title.value, title, bio.value, bio
1. Score: 39.600, Matched: title, bio, bio.value, title.value

1. Search with minimum score threshold: Found 2 results with score >= 0.3
1. Score: 13.200, Matched: bio.value, bio
1. Score: 13.200, Matched: bio, bio.value

1. Search with stemming and exact match boosting: Found 1 results (matches:
   work, working, worked)
1. Score: 13.200, Matched: bio, bio.value

1. Vector search (semantic search): Found 3 semantically similar documents
1. Score: 0.760, Matched:
1. Score: 0.753, Matched:
1. Score: 0.748, Matched:

1. Hybrid search (text + vector): Found 3 results using hybrid search (text +
   vector)
1. Score: 2.701, Matched: title.value, title, content.value, content
1. Score: 1.504, Matched: title.value, title, content, content.value
1. Score: 0.299, Matched:

1. Case-sensitive search: Found 1 results (case-sensitive)
1. Score: 26.400, Matched: skills.value, bio.value, bio, skills

=== Cleanup === ✅ Deleted test collections

✅ Search examples completed! === Schema Management Examples ===

1. Creating user schema with basic fields: ✅ User schema created

2. Creating product schema with text index: ✅ Product schema with indexes
   created

3. Creating document schema with vector index: ✅ Document schema with vector
   index created

4. Retrieving collection schema: Schema fields: [ 'age', 'email', 'name',
   'status' ] Schema version: 1

5. Retrieving collection metadata: Collection has 4 fields

6. Creating employee schema with all constraint types: ✅ Employee schema with
   all constraints created

✅ Schema management examples completed! === Join Operations Examples ===

Setting up sample data... ✅ Sample data created

1. Single collection join (users with departments): Found 2 users with
   department data:

- Alice Johnson: Engineering
- Bob Smith: Sales

2. Join with filtering: Found 1 users in Engineering:

- Alice Johnson: Building A

3. Join with user profiles: Found 2 users with profile data:

- Alice Johnson: Senior Software Engineer
- Bob Smith: Sales Manager

4. Join orders with user data: Found 2 completed orders:

- Laptop ($1200) by Alice Johnson
- Mouse ($25) by Alice Johnson

5. Complex join with multiple conditions: Found 2 users with example.com emails:

- Alice Johnson (alice@example.com): Building A
- Bob Smith (bob@example.com): Building B

=== Cleanup === ✅ Deleted test collections

✅ Join operations examples completed! 🚀 ekoDB Scripts Example (TypeScript)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved:
oFjVOpckAMclxm*0dTQ-Tf1SnU-R98Vq4Eydy7pydxU*-Gzgz8rGQ48tDPmLl90DCwVJeGqJ1i_G6J3pzzxekg
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved:
ktaZhVTgvm1wpQqTpsBvktGXkh522neYgePfA7ThKmfJdjIguZz_yKbsrnz9ir4OH2M_vPGBphEvWFknmLh1Cw
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved:
Ul4g8MKXNSgjkyH2lvXbByYpzXp1sbQr0yCFZ-XZaqzG_A2ByQKPvq1sO4D0ZeviBnRk_ctvdzVF-BY_WVWH_A
📊 Statistics: 2 groups {"status":"inactive","avg_score":50,"count":5}
{"count":5,"avg_score":60,"status":"active"}

📝 Example 4: Script Management

📋 Total scripts: 17 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed! 🚀 ekoDB TypeScript Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data... ✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved:
mCCzbyBjwXJY2NLQvVIKgnbOhuxraunjMDpuD6i8XmGyQfcOj3W5EwoJJY90DZFjhdXwAlS47VQk9VRSMDeZbQ
📊 Found 2 product groups
{"count":3,"category":"Electronics","avg_price":575.6666666666666}
{"avg_price":474,"category":"Furniture","count":2} ⏱️ Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved 📊 Found 5 products ⏱️ Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved 📊 Found 2 categories {"count":3,"category":"Electronics"}
{"count":2,"category":"Furniture"} ⏱️ Execution time: 0ms

📝 Example 4: High Rating Products

✅ Script saved 📊 Found 5 products ⏱️ Execution time: 0ms

📝 Example 5: Script with Parameter Definition

✅ Script saved 📊 Found 5 products ⏱️ Execution time: 0ms

📝 Example 6: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Script saved 📊 Pipeline executed 3 stages ⏱️ Total execution time: 0ms

🧹 Cleaning up... ✅ Cleanup complete

✅ All complete script examples finished!

💡 This example demonstrates ekoDB's Script system: ✅ FindAll operations ✅
Group aggregations (Count, Average) ✅ Multi-stage pipelines (FindAll → Group →
Count) ✅ Parameter definitions ✅ Script management (save, call, delete) ✅
[32mTypeScript client examples complete![0m ✅ [32mAll TypeScript integration
tests complete![0m 🧪 [36mRunning JavaScript examples (direct
HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 638ms

1 package is looking for funding run `npm fund` for details

found 0 vulnerabilities [34m ╔════════════════════════════════════════╗[0m [34m║
ekoDB JavaScript Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m [34m === Checking Server
Connection ===[0m (node:2706) [DEP0040] DeprecationWarning: The `punycode`
module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) [32m✓
Server is ready[0m [34m === Getting Authentication Token ===[0m [32m✓
Authentication successful[0m [33m === Running 10 Examples ===[0m [34m ===
Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_crud.js
===[0m ✓ Authentication successful

=== Insert Document === (node:2709) [DEP0040] DeprecationWarning: The `punycode`
module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) Inserted:
{ id:
'MarFFkwYBtYZj9Knl4GErjJgd1K9HknX9EwTLsER7PXmhPVkkmNUk5zJIbsYbqTVwa8-XyKjR_acVFNR7VNOAQ'
}

=== Find by ID === Found: { id:
'MarFFkwYBtYZj9Knl4GErjJgd1K9HknX9EwTLsER7PXmhPVkkmNUk5zJIbsYbqTVwa8-XyKjR_acVFNR7VNOAQ',
value: { value: 42, type: 'Integer' }, active: { type: 'Boolean', value: true },
name: { type: 'String', value: 'Test Record' } }

=== Find with Query === Found documents: 1

=== Update Document === Updated: { active: { type: 'Boolean', value: true },
value: { value: 100, type: 'Integer' }, id:
'MarFFkwYBtYZj9Knl4GErjJgd1K9HknX9EwTLsER7PXmhPVkkmNUk5zJIbsYbqTVwa8-XyKjR_acVFNR7VNOAQ',
name: { value: 'Updated Record', type: 'String' } }

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully [32m✓ simple_crud.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_websocket.js
===[0m ✓ Authentication successful

=== Inserting Test Data === (node:2716) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Inserted
test record:
4yx8jCkldYJ3cSCgMHvAGndl1C9n2sQflx-zuD3OFEyAU1RMURnPMjjzhkHV3UpH03NR9TifKjAABtwxqQVI5Q

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"ChsE4hhtNg8BkHvjsfyGVK_RCbVHQx8IaoOx1Vo0vx5l_0yuETorylmHzekR4I61txQOtNqmD1ntnmwC4zO4Nw",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"4yx8jCkldYJ3cSCgMHvAGndl1C9n2sQflx-zuD3OFEyAU1RMURnPMjjzhkHV3UpH03NR9TifKjAABtwxqQVI5Q",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"G2EZWyV1Cqti1eOKOJQtnPuxXRm0BHD5rChCO4pYktf6M8SCa1YvUpMpN3sRzJctGTa8GF9hzyPmiwJZfj0q_w",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"p0WGdKAIQtu4s5ajCoE5oCqbhiPonpADBDtzNzVo2gEYwqeaFyLY3JmHjuNrJ0ywk4YSqv-4LoaE9RFYLVD3ww",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } } ] }, "type": "Success" } ✓ Retrieved 4
record(s) via WebSocket

✓ WebSocket example completed successfully [32m✓ simple_websocket.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/batch_operations.js
===[0m ✓ Authentication successful

=== Batch Insert === (node:2720) [DEP0040] DeprecationWarning: The `punycode`
module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Batch
inserted 5 records ✓ Verified: Found 20 total records in collection

=== Creating test records for update/delete === Created 3 test records

=== Batch Update === ✓ Batch updated 3 records ✓ Verified: Record updated with
status="active"

=== Batch Delete === ✓ Batch deleted 3 records ✓ Verified: Records successfully
deleted (not found)

✓ All batch operations completed successfully [32m✓ batch_operations.js
completed successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/kv_operations.js
===[0m ✓ Authentication successful

=== KV Set === (node:2727) [DEP0040] DeprecationWarning: The `punycode` module
is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Set key:
session:user123

=== KV Get === Retrieved value: { loginTime: '2025-12-09T18:40:53.513Z', userId:
123, username: 'john_doe' }

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: { price: 29.99, name: 'Product 1' }
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully [32m✓ kv_operations.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/collection_management.js
===[0m ✓ Authentication successful

=== Create Collection (via insert) === (node:2730) [DEP0040] DeprecationWarning:
The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
Collection created with first record:
CJl22S2mK56Kf-IC8yoA-oqn-vaA5uhwMOFeXNrzhctEliroSH3Y7cIo1vsdFP4iyu2DANTCbzgyic9gja6juw

=== List Collections === Total collections: 23 Sample collections: [
'schema_users_client_py', 'schema_employees_client_py', 'test_collection',
'chat_configurations__ek0_bench', 'demo_collection' ]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully [32m✓
collection_management.js completed successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/document_ttl.js
===[0m ✓ Authentication successful

=== Insert Document with TTL (1 hour) === (node:2737) [DEP0040]
DeprecationWarning: The `punycode` module is deprecated. Please use a userland
alternative instead. (Use `node --trace-deprecation ...` to show where the
warning was created) ✓ Inserted document:
jtzib-fbYAKvn3e7fzZiJIOEtTSYbChhqXoEdTf7b7bIxdoj46kV5PxSoCuvAv0SXDj8cdaPapBhKnuZnlEkNA

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 4 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully [32m✓ document_ttl.js
completed successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/websocket_ttl.js
===[0m ✓ Authentication successful

=== Insert Test Data with TTL === (node:2742) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Inserted
document with TTL:
fn18xskx_cL3kMQCen4v_H4KlB3nqgiEC-5Kc93pBdWtb2mgDj2YD9FLEidkEnzTXs3SPHMqk_p_vhyQ8smhPQ

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 4 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully [32m✓ websocket_ttl.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/http_functions.js
===[0m 🚀 ekoDB Scripts Example (JavaScript/HTTP)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved:
5Yn0rEKxTQE20J50PZe6CuDuX4WZEC28hvo8MTOpz6tLrHMaYiWC-dXephSxugBfnuaFDzYA4GMYOVD0y-C5Sg
📊 Found 20 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved:
n9cPNC9uAbRY7ud_FIJheELjTlJhUs5mmVkJiR46CCdo1kQu1kgQNO5lppx5Ky0C-iGFeYreHxEXqZitHqpqJA
📊 Page 1: Found 3 users (limit=3, skip=0) 📊 Page 2: Found 3 users (limit=3,
skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved:
vR_P70Z_kvWwesv6DN5w_rVWti81iYfByiHj9Yz4-aFmiFfF-XtXsOMfqAjtvZp1deckLsujfSF73iJKGIgcXQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
{"count":10,"status":"inactive","avg_score":50,"max_score":90}
{"max_score":100,"count":10,"avg_score":60,"status":"active"}

📝 Example 4: Function Management

📋 Total scripts: 19 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

✅ All examples completed! [32m✓ http_functions.js completed successfully[0m
[34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/transactions.js
===[0m ✓ Authentication successful

=== Setup: Creating Test Accounts === Created Alice: $1000 - ID:
B_D7s0bCsL8RXJzNuwDeWcX74TFTH2JyDxJqf8hnaYvcza_AHhJ-48TX1MtuZOve2ygVHTWkLwiiwkl2qojWXg
Created Bob: $500 - ID:
NN-6qZNFQoQzcjDDo5B4V9BLxKGsVfcT9H0IxnsYMRLqWHZny5ABXMHCoimWXWwkTn3A2vTx526B3irCj8wMdA

=== Example 1: Begin Transaction === Transaction ID:
839afbf9-09d8-4324-aa79-e373acf511a2

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: Active Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: {"type":"Integer","value":800} Bob:
{"value":700,"type":"Integer"}

=== Example 5: Rollback === New transaction:
a05094b5-3218-4ef1-879b-ff1400b7b89c Updated Bob: $700 → $600 (in transaction) ✓
Transaction rolled back Bob after rollback: {"type":"Integer","value":700}

=== Cleanup === ✓ Deleted test accounts

✓ All transaction examples completed [32m✓ transactions.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/crud_scripts.js
===[0m 🚀 ekoDB Complete CRUD Scripts Example
============================================================ Demonstrates: •
Insert + Verify (using Query) • Query + Update Status + Verify • Query + Update
Credits + Verify • Query Before Delete + Delete + Verify Gone

# Each Script shows Functions chaining with proper verification

============================================================ 📝 Script 1:
Insert + Verify ============================================================

1️⃣ Saving Script... ✅ Script saved:
\_6HT1SHkhAMYkBy4oBy6cwVXJJkCXae5CJH36pC-Yhg-GyE6dGUucx5O5T_PlOpqZr7aN7sLngaHEd4UqwfbvQ

2️⃣ Calling Script (Insert + Verify)... ✅ Script executed: 2 Functions ⏱️
Execution time: 0ms

3️⃣ Verification Results: ✅ Found 1 record(s) 📋 Name: [object Object] 📋 Email:
{"value":"alice@example.com","type":"String"} 📋 Status: [object Object] 📋
Credits: [object Object]

============================================================ 📝 Script 2:
Query + Update + Verify
============================================================

1️⃣ Saving Script... ✅ Script saved:
cIGU2qG0yA-knbnRFZ5rFLIV0YHy_vLVXVTEHwBMYQMpEQ2aufLmLLNCNv_wCts5qSYgz-kHieLjNWSsMMT8mg

2️⃣ Calling Script (Query + Update + Verify)... ✅ Script executed: 3 Functions
⏱️ Execution time: 0ms

3️⃣ Verification Results: ✅ Found 1 record(s) 📋 Status updated to:
{"value":"pending","type":"String"} 📋 Name: {"value":"Alice
Smith","type":"String"}

============================================================ 📝 Script 3:
Query + Update Credits + Verify
============================================================

1️⃣ Saving Script... ✅ Script saved:
Yrh-NR96dElUrUqnhQXA6hxwCh-PrbB6Tfxn3bwInELbZMZXQxgVINewnz5vUZXIbRtP7gTAiqbA19IWubKbeg

2️⃣ Calling Script (Query + Update Credits + Verify)... ✅ Script executed: 3
Functions ⏱️ Execution time: 0ms

3️⃣ Verification Results: ✅ Found 1 record(s) 📋 Credits updated to:
{"value":0,"type":"Integer"} 📋 Status: {"value":"active","type":"String"} 📋
Name: {"type":"String","value":"Alice Smith"}

============================================================ 📝 Script 4: Query
Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script... ✅ Script saved:
0P767uZZOpbh9jK4czeMSDKerSP69vrCt6RXnU7PhsU1lLpSkXJHTmC7vnHIhDrIDOzEiZbDHVGYgDZuVplZ4A

2️⃣ Calling Script (Query + Delete + Verify)... ✅ Script executed: 3 Functions
⏱️ Execution time: 0ms

3️⃣ Verification Results: 📊 Before delete: Record existed (verified by first
Query) ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================ 🧹 Cleanup
============================================================ ✅ Deleted script:
\_6HT1SHkhAMYkBy4oBy6... ✅ Deleted script: cIGU2qG0yA-knbnRFZ5r... ✅ Deleted
script: Yrh-NR96dElUrUqnhQXA... ✅ Deleted script: 0P767uZZOpbh9jK4czeM... ✅
Deleted collection: users

============================================================ ✅ Complete CRUD
Scripts Example Finished!
============================================================

💡 Key Takeaways: ✅ Scripts chain Functions together ✅ Each Script
demonstrates operation + verification ✅ Parameters make Scripts reusable ✅
Verification is built into the Script itself ✅ Complete CRUD lifecycle in 4
focused Scripts [32m✓ crud_scripts.js completed successfully[0m [34m
╔════════════════════════════════════════╗[0m [34m║ Test Summary ║[0m
[34m╚════════════════════════════════════════╝[0m [34mTotal: 10[0m [32mPassed:
10[0m [32mFailed: 0[0m ✅ [32mJavaScript direct examples complete![0m 📦
[36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.3.0 prepare npm run build

> @ekodb/ekodb-client@0.3.0 build tsc

up to date, audited 7 packages in 2s

found 0 vulnerabilities

> @ekodb/ekodb-client@0.3.0 build tsc

✅ [32mTypeScript client built![0m 🧪 [36mRunning JavaScript client library
examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 632ms

1 package is looking for funding run `npm fund` for details

found 0 vulnerabilities ✓ Client created (token exchange happens automatically)

=== Insert Document === Inserted: { id:
'FNX8_AImcx6vh7s3UHa0V_GBATR1HCN0phXdIOS0iwDgNVKgeiob3WJbScnQi6NyDSA0jPwnuzEElDyNCla3QQ'
}

=== Find by ID === Found: { active: { type: 'Boolean', value: true }, value: {
value: 42, type: 'Integer' }, id:
'FNX8_AImcx6vh7s3UHa0V_GBATR1HCN0phXdIOS0iwDgNVKgeiob3WJbScnQi6NyDSA0jPwnuzEElDyNCla3QQ',
name: { type: 'String', value: 'Test Record' } }

=== Find with Query === Found documents: 1

=== Update Document === Updated: { name: { type: 'String', value: 'Updated
Record' }, id:
'FNX8_AImcx6vh7s3UHa0V_GBATR1HCN0phXdIOS0iwDgNVKgeiob3WJbScnQi6NyDSA0jPwnuzEElDyNCla3QQ',
active: { type: 'Boolean', value: true }, value: { type: 'Integer', value: 100 }
}

=== Delete Document === Deleted document

=== Cleanup === ✓ Deleted collection

✓ All CRUD operations completed successfully ✓ Client created

=== Batch Insert === ✓ Batch inserted 5 records ✓ Verified: Found 5 total
records in collection

=== Batch Update === ✓ Batch updated 3 records

=== Batch Delete === ✓ Batch deleted 3 records

=== Cleanup === ✓ Deleted collection

✓ All batch operations completed successfully ✓ Client created

=== KV Set === ✓ Set key: session:user123

=== KV Get === Retrieved value: { username: 'john_doe', userId: 123 }

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: { name: 'Product 1', price: 29.99 }
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
1HgNJJ_KETfBECFaIJ3q8bczaqfc9AtoJnP9i8Ym4ASeXK0d3KEso8eKda3WsCU-jsmdNgvI5rxGEJQDXouY7Q

=== List Collections === Total collections: 22 Sample collections:
schema_users_client_py,schema_employees_client_py,test_collection,chat_configurations**ek0_bench,chat_messages**ek0_bench

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
arO3JyCQJuMb_CFS87TDLYa1foZP4SdAJmDgZHJOeA4Uxnkt0jJhhSZw6m5krrkSy5OdgPNw_5waz-iQ5h5AlA

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
JfChkOlXjz6zU3H60JIesTeDN0_MdXRJ57S-4CCMG6B69myLCyCp8JdhXfc-PEqbMc8xLb7NX2D0D5x8z5w_AQ

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
YbZxqUCuk_AymEM0iPH_IgM5h7f3BN4UX2deTKufp7qhSkGDLxi8Al-ikxPZircGk95kbExAhE_9MSFDQ7SFWg

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
2wyhy76s94P5BSmsgZ9UfN-B_grbfSBkNyBMm1A6Kz_b9Sdc6MEWOewPL1X7rGghMzyvHJRo0seUpsN6f67dzw

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration 🚀 ekoDB Scripts Example (JavaScript/HTTP)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved:
uATiYQnploA4ykurX-Akng8m1mgP3FL_Q_4faeBjenTlWt2M5vE_vuHrsCo8Ve-e6gymwovkbhR2b7MFozV_Cw
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved:
xK0tbUJ8ZCy3bMtUaCofSCl_bUtFt4CkyNx8e3kwC59yhl8F5fJcu0ZWA_EJM_FMbcoo1qVjctctMqRcqjGZCw
📊 Page 1: Found 3 users (limit=3, skip=0) 📊 Page 2: Found 2 users (limit=3,
skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved:
ozKAxzwwsWKdQmt_RYD7s9EvNz7h7oOhNYI09wVk8CrxNJIZjON0DHrs54stsBPwSlRliFosnvoNGaUiFhMrAQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
{"max_score":100,"status":"active","count":5,"avg_score":60}
{"max_score":90,"status":"inactive","avg_score":50,"count":5}

📝 Example 4: Function Management

📋 Total scripts: 21 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

✅ All examples completed! 🚀 ekoDB Scripts Example (JavaScript Client)

✅ Client initialized (token exchange automatic)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved:
0c4EoSXXWu5f302EWkwRKPYCT7fEYP9m_90TsLAFhhUV9mX15ZzZbhTaSSTWdpmqap-68Bj_T0kmCUv4R08HdQ
📊 Found 20 records ⏱️ Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved 📊 Found 20 users (limited) ⏱️ Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved 📊 Statistics: 2 groups
{"count":10,"avg_score":50,"status":"inactive"}
{"status":"active","count":10,"avg_score":60} ⏱️ Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 23 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

📝 Example 5: Multi-Stage Pipeline

✅ Multi-stage script saved 📊 Pipeline executed 2 stages ⏱️ Total execution
time: 0ms 📈 Stage breakdown:

📝 Example 6: Count Users

✅ Count script saved 📊 Total user count: 20 ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Deleted collection ✅ Deleted test scripts

✅ All examples completed successfully!

💡 Key Advantages of Using the Client: • Automatic token management • Type-safe
Stage builders • ChatMessage helpers • Cleaner, more maintainable code •
Built-in error handling 🚀 ekoDB Advanced Scripts Example

📋 Setting up test data... ✅ Created 10 products

📝 Example 1: List All Products

✅ Script saved 📊 Found 10 products ⏱️ Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Script saved 📊 Found 2 categories Furniture: 4 items (avg $294.00)
Electronics: 6 items (avg $325.67) ⏱️ Execution time: 0ms

📝 Example 3: Count All Products

✅ Script saved 📊 Total products: 10 ⏱️ Execution time: 0ms

📝 Example 4: Multi-Stage Aggregation

✅ Script saved 📊 Category analysis (2 categories): Furniture: Products: 4 |
Stock: 43 | Avg Rating: ⭐4.26 Electronics: Products: 6 | Stock: 232 | Avg
Rating: ⭐4.52

⏱️ Total execution time: 0ms 📈 Pipeline stages:

📝 Example 5: Project Specific Fields

✅ Script saved 📊 Product summaries (10 items, showing first 3):

1.  Office Lamp - $79 (⭐4.05)
2.  Wireless Mouse - $29 (⭐4.5)
3.  Monitor 27" - $399 (⭐4.6) ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Cleanup complete

✅ All advanced script examples completed! 🚀 ekoDB CRUD Scripts Example

📋 Setting up test data... ✅ Created 10 test users

📝 Example 1: List All Users

✅ Script saved 📊 Found 10 users ⏱️ Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Script saved 📊 User counts by status: inactive: 3 users active: 7 users ⏱️
Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Script saved 📊 Average scores by role: admin: 20.0 (3 users) user: 70.0 (7
users) ⏱️ Execution time: 0ms

📝 Example 4: Top Users by Score

✅ Script saved 📊 Users (showing first 5 of 10):

1.  User 5 (user5@example.com) - Score: 50
2.  User 4 (user4@example.com) - Score: 40
3.  User 2 (user2@example.com) - Score: 20
4.  User 8 (user8@example.com) - Score: 80
5.  User 10 (user10@example.com) - Score: 100 ⏱️ Execution time: 0ms

📝 Example 5: User Summary Statistics

✅ Script saved 📊 User summary (2 groups): active undefineds: 7 users, Total
Score: 370 inactive undefineds: 3 users, Total Score: 180 ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Cleanup complete

✅ All CRUD script examples completed! 🚀 ekoDB Search Scripts Example

📋 Setting up test data... ✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Script saved 📊 Found 5 documents

1.  Natural Language Processing (AI)
2.  Introduction to Machine Learning (AI)
3.  Vector Databases Explained (Database)
4.  Getting Started with ekoDB (Database)
5.  Database Design Principles (Database) ⏱️ Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved 📊 Documents by category: AI: 2 documents Database: 3 documents
⏱️ Execution time: 0ms

📝 Example 3: Select Specific Fields

✅ Script saved 📊 Document titles (5 docs):

1.  Natural Language Processing
2.  Introduction to Machine Learning
3.  Vector Databases Explained
4.  Getting Started with ekoDB
5.  Database Design Principles ⏱️ Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved 📊 Projected documents (showing first 3):

1.  Natural Language Processing
2.  Introduction to Machine Learning
3.  Vector Databases Explained ⏱️ Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved 📊 All documents (5 total, showing first 2):

1.  Natural Language Processing (AI)
2.  Introduction to Machine Learning (AI) ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Cleanup complete

✅ All search script examples completed! 🚀 ekoDB AI Scripts Example

⚠️ Note: These examples require AI API credentials (OpenAI, etc.)

📋 Setting up test data... ✅ Created 3 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved 🤖 AI Response:

1. Scalability: Vector databases can handle massive amounts of data and scale up
   easily.
2. Performance: They offer high-speed data processing and efficient querying
   capabilities.
3. Flexibility: They can store and manage different types of data including
   spatial, temporal, and multimedia data.
4. Precision: They maintain the highest level of detail and precision, without
   data degradation.
5. Interactivity: They facilitate interactive operations like point-in-polygon
   or network analysis.
6. Integration: They can be easily integrated with other systems or data types.
7. Security: They provide robust security features to protect sensitive data. ⏱️
   Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding script saved 📊 Generated embeddings for 3 articles

1.  "Draft Article" - 1536D vector
2.  "Getting Started with ekoDB" - 1536D vector
3.  "Advanced Query Patterns" - 1536D vector ⏱️ Execution time: 0ms

📝 Example 3: List All Articles

✅ Script saved 📊 Found 3 articles ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Cleanup complete

✅ All AI examples completed! ✅ [32mJavaScript client examples complete![0m ✅
[32mAll JavaScript integration tests complete![0m 🟣 [36mBuilding Kotlin client
library...[0m To honour the JVM settings for this build a single-use Daemon
process will be forked. For more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE Task :jar UP-TO-DATE Task :assemble UP-TO-DATE Task
> :compileTestKotlin NO-SOURCE Task :compileTestJava NO-SOURCE Task
> :processTestResources NO-SOURCE Task :testClasses UP-TO-DATE Task :test
> NO-SOURCE Task :check UP-TO-DATE Task :build UP-TO-DATE

BUILD SUCCESSFUL in 3s 2 actionable tasks: 2 up-to-date ✅ [32mKotlin client
built![0m 🧪 [36mRunning Kotlin client library examples...[0m To honour the JVM
settings for this build a single-use Daemon process will be forked. For more on
this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Simple CRUD Example ===

=== Create === ✓ Inserted user:
Record(fields={id=StringValue(value=QP0_syT_o0ZeTKuy9dpqvm44l12fP4AffzH5P9ywb-YHsiBJsa1fvCUvptBO9JFtMe3ZYmdth9STp9YrqKhjeg)})
User ID:
QP0_syT_o0ZeTKuy9dpqvm44l12fP4AffzH5P9ywb-YHsiBJsa1fvCUvptBO9JFtMe3ZYmdth9STp9YrqKhjeg

=== Read === ✓ Found user by ID:
Record(fields={age=ObjectValue(value={type=StringValue(value=Integer),
value=IntegerValue(value=28)}),
id=StringValue(value=QP0_syT_o0ZeTKuy9dpqvm44l12fP4AffzH5P9ywb-YHsiBJsa1fvCUvptBO9JFtMe3ZYmdth9STp9YrqKhjeg),
name=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=Alice Johnson)}),
email=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=alice@example.com)}),
active=ObjectValue(value={type=StringValue(value=Boolean),
value=BooleanValue(value=true)})})

=== Update === ✓ Updated user:
Record(fields={name=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=Alice Johnson)}),
email=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=alice@example.com)}),
city=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=San Francisco)}),
active=ObjectValue(value={type=StringValue(value=Boolean),
value=BooleanValue(value=true)}),
id=StringValue(value=QP0_syT_o0ZeTKuy9dpqvm44l12fP4AffzH5P9ywb-YHsiBJsa1fvCUvptBO9JFtMe3ZYmdth9STp9YrqKhjeg),
age=ObjectValue(value={value=IntegerValue(value=29),
type=StringValue(value=Integer)})})

=== Query === ✓ Found 1 users matching query

- Record(fields={id=StringValue(value=QP0_syT_o0ZeTKuy9dpqvm44l12fP4AffzH5P9ywb-YHsiBJsa1fvCUvptBO9JFtMe3ZYmdth9STp9YrqKhjeg),
  active=ObjectValue(value={type=StringValue(value=Boolean),
  value=BooleanValue(value=true)}),
  name=ObjectValue(value={type=StringValue(value=String),
  value=StringValue(value=Alice Johnson)}),
  email=ObjectValue(value={type=StringValue(value=String),
  value=StringValue(value=alice@example.com)}),
  city=ObjectValue(value={value=StringValue(value=San Francisco),
  type=StringValue(value=String)}),
  age=ObjectValue(value={value=IntegerValue(value=29),
  type=StringValue(value=Integer)})})

=== Delete === ✓ Deleted user with ID:
QP0_syT_o0ZeTKuy9dpqvm44l12fP4AffzH5P9ywb-YHsiBJsa1fvCUvptBO9JFtMe3ZYmdth9STp9YrqKhjeg

✓ Confirmed user was deleted

=== Cleanup === ✓ Deleted collection: kotlin_users_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Batch Operations Example ===

=== Batch Insert === ✓ Inserted 5 records IDs:
G6asODFMfC8H2ehnaXFjz0CXZZ0SQ7hmgPmNj_W5PrIKaqMN7zRU4xWvwWdvyggWGR0et80rk70WIBqmBCaGXQ,
F-tr0FvvI1yocSAI6BsVkkVm_rfObFJC74IpboyhD28GiO7WAJVYRQEdzon9jYYuA7CrbJXHfT_Z5b5hDrkQMg,
G_K04uAkVvSzhfHzQnz1ybdV7leEZSUJVhdAZvuivU1lDENnF-oaojKp7L-DUeuv4suvZwn1HvWHiMM3I8i9Ag...

=== Batch Update === ✓ Updated 3 records

=== Batch Delete === ✓ Deleted 2 records

=== Cleanup === ✓ Deleted collection: kotlin_batch_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Query Builder Example ===

=== Inserting Sample Data === ✓ Inserted 5 records

=== Query 1: Equality (city = 'NYC') === ✓ Found 2 records in NYC

=== Query 2: Range (age >= 25 AND age < 32) === ✓ Found 3 records with age 25-31

=== Query 3: Sort by score (descending) === ✓ Top 3 scores:

- Score: ObjectValue(value={value=IntegerValue(value=95),
  type=StringValue(value=Integer)})
- Score: ObjectValue(value={value=IntegerValue(value=92),
  type=StringValue(value=Integer)})
- Score: ObjectValue(value={type=StringValue(value=Integer),
  value=IntegerValue(value=88)})

=== Query 4: Complex (score > 80 AND age >= 25) === ✓ Found 4 high-scoring
adults

=== Query 5: IN (city IN ['NYC', 'LA']) === ✓ Found 4 records in NYC or LA

=== Query 6: Pagination (skip 2, limit 2) === ✓ Page 2 (2 records):

- ObjectValue(value={value=StringValue(value=Charlie),
  type=StringValue(value=String)})
- ObjectValue(value={value=StringValue(value=Diana),
  type=StringValue(value=String)})

=== Query 7: Contains (name contains 'a') === ✓ Found 2 names containing 'a'

=== Cleanup === ✓ Deleted collection: kotlin_query_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - KV Operations Example ===

=== KV Set === ✓ Set key: user:123

=== KV Get === ✓ Retrieved value:
{"email":"alice@example.com","name":"Alice","role":"admin"}

=== KV Set with TTL === ✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key === ✓ Session value:
{"user_id":"123","created_at":1765305699887} (Will expire in 10 seconds)

=== Set Multiple Keys === ✓ Set 3 configuration keys

=== KV Delete === ✓ Deleted key: user:123

=== Verify Deletion === ✓ Confirmed key was deleted

=== Cleanup === ✓ Cleaned up test keys

=== Example Complete ===

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Collection Management Example ===

=== List Collections === ✓ Found 21 collections

- schema_users_client_py
- schema_employees_client_py
- test_collection
- chat_configurations\_\_ek0_bench
- chat_messages\_\_ek0_bench

=== Check Collection Existence === Collection 'kotlin_collection_example'
exists: false

=== Create Collection with Schema === ✓ Created collection with schema:
kotlin_collection_example

=== Get Collection Schema === ✓ Schema:
{"fields":{"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2025-12-09T18:41:44.165390Z","last_modified":"2025-12-09T18:41:44.165390Z","bypass_ripple":false,"primary_key_alias":"id"}

=== Cleanup === ✓ Deleted collection: kotlin_collection_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Document TTL Example ===

=== Insert with TTL === ✓ Inserted document with 10s TTL Document ID:
vWWXJTa8Ldsj5a_meYGXbskwO2tAubBQwyK6Cg77v5oDJAfyD-uS08F8JA0yj139iF05sKSHwxYt7qX_lZxg9g

=== Verify Document Exists === ✓ Document found: [id, user_id, session_id,
created_at, ttl_duration]

=== Insert with Longer TTL === ✓ Inserted document with 1h TTL Document ID:
4ZJLa15K1x4_afd147CNTPE8RXxAnwXXL0DhmxHYS-nxnwbdzpwuNAkQfmBFZlvCLc7pnozuRK_rGOyn1gd1Iw

=== TTL Expiration === ✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document === ✓ Long TTL document still exists:
[ttl_duration, cache_key, id, value]

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection: kotlin_ttl_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Simple WebSocket Example ===

=== Create WebSocket Client === ✓ WebSocket client created

=== Connect to WebSocket === ✓ Connected to WebSocket

=== Insert Test Record === ✓ Inserted test record

=== Find All via WebSocket === ✓ WebSocket findAll result:
{"payload":{"data":[{"id":"Yy2_0jfmuoMgpUv1zsPEln1i-VLDTIGOorCMmaXZh1EbJTQP_oweRWnxHF8dstQj-P5jvN03kfkWUbfe8W7X7A","name":{"type":"String","value":"Test
User"},"status":{"type":"String","value":"active"}}]},"type":"Success"}

=== Close WebSocket === ✓ WebSocket closed

=== Cleanup === ✓ Deleted collection: kotlin_websocket_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Schema Management Example ===

=== Creating Collection with Schema === ✓ Created collection
'kotlin_schema_example' with schema

=== Inserting Valid Documents === ✓ Inserted user 1:
StringValue(value=NgP3LAVPCUT7dpu80P8L2kxyTKXK-VT2Ay9bjAkbD04C3El6u75jjS1PfKvEsZw2MeilSqaGxI8HbfDpYhhVLg)
✓ Inserted user 2:
StringValue(value=quJbDJzsaUeIW4TLZeV3HDHQl1-Gt-mY58z-b3qeYDEMRBDDf0Wo_8tKq7Z_XB36y4yxgo4Eqhm8HQa5GUA37Q)

=== Getting Schema === ✓ Schema for kotlin_schema_example: Fields:
{"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections === ✓ Total collections: 22 Sample:
[schema_users_client_py, schema_employees_client_py, test_collection,
chat_configurations__ek0_bench, chat_messages__ek0_bench]

=== Cleanup === ✓ Deleted collection: kotlin_schema_example

✓ All schema management operations completed successfully

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Search Example ===

=== Inserting Sample Documents === ✓ Inserted 5 sample documents

=== Basic Text Search === ✓ Search results for 'programming':
{"results":[{"record":{"description":"Learn Rust programming language with
hands-on examples and best practices.","views":8,"title":"Rust
Programming","id":"0Zb_pBgRyJKECfTfYXKod729WSetU-FDn4AIGA4qLdckhLISjShXskZwZYWoWP-2fBxSsjz6xHfW5rUMiGAjrQ","tags":"programming,rust,tutorial"},"score":19.8,"matched_fields":["description","title","tags"]},{"record":{"views":425,"tags":"programming,python,data-science","title":"Python
for Data Science","description":"Master Python for data analysis, machine
learning, and
visualization.","id":"mxLo-wBUymJXeJ08G2USv_FsoI0CbsD944Sro_ikJ75OlwAuXrWE6WVyGlCaDSaN1snWdotamOrnpkmQ64Gdww"},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"tags":"programming,javascript,web","views":854,"description":"Build
modern web applications using JavaScript, React, and
Node.js.","title":"JavaScript Web
Development","id":"MoyzlJavmItaTZxzSlUQBfdusesHNAbvuTAxUry6q1AH5oRy5AnkT5Qa6PlBs9lw1MBToCV6itdnRXa4d-osRg"},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' === ✓ Found results for 'machine learning'
{"results":[{"record":{"views":850,"id":"\_9ZOm1cjVM9d33t_UlSfJ1Tigssguo5a63UQfjVpc0vR_uf2nCN5XsTG7BgFETRCapGwMmltClpjN5SnB1qevQ","tags":"ai,machine-learning,python","title":"Machine
Learning Basics","description":"Introduction to machine learning algorithms and
neural
networks."},"score":2.7,"matched_fields":["tags","title","description"]},{"record":{"description":"Master
Python for data analysis, machine learning, and visualization.","title":"Python
for Data
Science","tags":"programming,python,data-science","id":"mxLo-wBUymJXeJ08G2USv_FsoI0CbsD944Sro_ikJ75OlwAuXrWE6WVyGlCaDSaN1snWdotamOrnpkmQ64Gdww","views":425},"score":1.0,"matched_fields":["description"]},{"record":{"views":8,"tags":"programming,rust,tutorial","description":"Learn
Rust programming language with hands-on examples and best
practices.","title":"Rust
Programming","id":"0Zb_pBgRyJKECfTfYXKod729WSetU-FDn4AIGA4qLdckhLISjShXskZwZYWoWP-2fBxSsjz6xHfW5rUMiGAjrQ"},"score":0.5,"matched_fields":["description"]},{"record":{"id":"oMEAz5Zi3HxKX1mxeM9ufIB_dhMBQbKFQ_yW-Y5zbbOnozfJrZdhSXib1gaBevQLa-V8tbdLAchbMG10ujdVaA","views":2,"title":"Database
Design","description":"Learn database design principles, normalization, and
query
optimization.","tags":"database,design,sql"},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

=== Cleanup === ✓ Deleted collection: kotlin_search_example

✓ All search operations completed successfully

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - WebSocket TTL Example ===

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
StringValue(value=Jd03U9uygmL9cH-ksajF0jDI4vow2_iNtI6X0OMjaM6yiDfHN2EvZX1CKN11K0KZBc_2leTZZV5ubEBHWcCMIw)

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved data via
WebSocket:
{"payload":{"data":[{"created_at":{"type":"Integer","value":1765305725205},"id":"Jd03U9uygmL9cH-ksajF0jDI4vow2_iNtI6X0OMjaM6yiDfHN2EvZX1CKN11K0KZBc_2leTZZV5ubEBHWcCMIw","name":{"type":"String","value":"WebSocket
TTL
Test"},"ttl_duration":{"type":"String","value":"1h"},"value":{"type":"Integer","value":42}}]},"type":"Success"}

✓ WebSocket closed

=== Cleanup === ✓ Deleted collection: kotlin_websocket_ttl_example

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Join Operations Example ===

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

✓ Join operations example completed successfully

BUILD SUCCESSFUL in 4s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Basic Chat Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
hEkr3R1pmJ9cykn2mYps7s2tFBXqFse1LHeDi2_vGzRog6AtFplh8lwVLsjwAJV-FEYg442W-DACwwZxNxWlmA

=== Sending Chat Message === ✓ Chat response: Message ID:
"azRlZP8zIldwt-LgyLMu0XDrhWvGcXWcAiBNHVftmyDGWRFZRXzmyVwBbw9SGNGnGbKH41b5KYBbYI_IpLZumA"
Responses: ["I'm sorry, but the provided context does not contain any
information about what ekoDB is or what features it has."]

=== Cleanup === ✓ Deleted chat session ✓ Deleted collection:
kotlin_chat_basic_example

✓ Basic chat example completed successfully

BUILD SUCCESSFUL in 5s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
pOijMiS-nMN0LYt--ZiOsf2I84L3ISe_UdJsK2XP-chgEGWGEM5ByOYeVnoNS-lMX50goi6yFsRcXpfaTFIVjQ

=== Sending Messages === ✓ Message 1 sent Responses: ["I'm sorry, but the
context provided does not contain specific information about any available
products. Could you please provide more details or a specific product you're
interested in?"]

✓ Message 2 sent Responses: ["Apologies, but there's no information provided
about the product's price in the context. Could you please provide more details
or specify which product you're asking about?"]

=== Getting Message History === ✓ Retrieved message history Total messages:
[{"chat_id":{"type":"String","value":"pOijMiS-nMN0LYt--ZiOsf2I84L3ISe_UdJsK2XP-chgEGWGEM5ByOYeVnoNS-lMX50goi6yFsRcXpfaTFIVjQ"},"content":{"type":"String","value":"What
products are
available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":{"type":"String","value":"A
high-performance database product with AI
capabilities"},"id":"qPzuw8kyMdIRzPRoNPaMihmn4erUSygYd5lrVKTtckGyarUtEl7lfSXjRBq3LDDO6Xso-kz4-qiK4MlaIgIiLQ","price":{"type":"Integer","value":99},"product":{"type":"String","value":"ekoDB"}},"score":0.25}]},"created_at":{"type":"DateTime","value":"2025-12-09T18:42:22.458209Z"},"id":"wHIOF4yTNdnqrcRxU7Kdqzk72T9uE7kJXGfZsEes8nI98ADyeUQ9gJ07_DCUJNcIFrFQgCW7bK0vm7kHks1aBQ","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":33,"prompt_tokens":153,"total_tokens":186}},"updated_at":{"type":"DateTime","value":"2025-12-09T18:42:22.458209Z"}},{"chat_id":{"type":"String","value":"pOijMiS-nMN0LYt--ZiOsf2I84L3ISe_UdJsK2XP-chgEGWGEM5ByOYeVnoNS-lMX50goi6yFsRcXpfaTFIVjQ"},"content":{"type":"String","value":"I'm
sorry, but the context provided does not contain specific information about any
available products. Could you please provide more details or a specific product
you're interested
in?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":{"type":"String","value":"A
high-performance database product with AI
capabilities"},"id":"qPzuw8kyMdIRzPRoNPaMihmn4erUSygYd5lrVKTtckGyarUtEl7lfSXjRBq3LDDO6Xso-kz4-qiK4MlaIgIiLQ","price":{"type":"Integer","value":99},"product":{"type":"String","value":"ekoDB"}},"score":0.25}]},"created_at":{"type":"DateTime","value":"2025-12-09T18:42:22.463572Z"},"id":"mTQPPQ-Ig-ItzUGmH8d9UoKR5Fd9KqD9wBsiSIGc0DFaQHIAWejm7kbC-JueYqm5pHB4j7Slo5azE8aEugrhZA","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":33,"prompt_tokens":153,"total_tokens":186}},"updated_at":{"type":"DateTime","value":"2025-12-09T18:42:22.463572Z"}},{"chat_id":{"type":"String","value":"pOijMiS-nMN0LYt--ZiOsf2I84L3ISe_UdJsK2XP-chgEGWGEM5ByOYeVnoNS-lMX50goi6yFsRcXpfaTFIVjQ"},"content":{"type":"String","value":"What
is the
price?"},"created_at":{"type":"DateTime","value":"2025-12-09T18:42:24.605268Z"},"id":"ysTlOHJXsB-b0RYii8iNYrv6NNHw66Gs2PiD3hbDxqxiBQSt3ZN0W2x4f-bfjeQNIV-zAg0tCHF7KGhH14EY4w","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":33,"prompt_tokens":70,"total_tokens":103}},"updated_at":{"type":"DateTime","value":"2025-12-09T18:42:24.605268Z"}},{"chat_id":{"type":"String","value":"pOijMiS-nMN0LYt--ZiOsf2I84L3ISe_UdJsK2XP-chgEGWGEM5ByOYeVnoNS-lMX50goi6yFsRcXpfaTFIVjQ"},"content":{"type":"String","value":"Apologies,
but there's no information provided about the product's price in the context.
Could you please provide more details or specify which product you're asking
about?"},"created_at":{"type":"DateTime","value":"2025-12-09T18:42:24.608980Z"},"id":"QCRgoHNg0V2DdkvFqspSKlt0j6wYDgQn53RF1dEJOV-3oKYY3mTgXSyQSRsEFYXiCxTTylfxKG_C7qxmc6hvVQ","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":33,"prompt_tokens":70,"total_tokens":103}},"updated_at":{"type":"DateTime","value":"2025-12-09T18:42:24.608980Z"}}]

=== Updating Session === ✓ Updated session system prompt

=== Listing Sessions === ✓ Total sessions: 13

=== Branching Session === ✓ Created branched session:
CEsDz7CLwMDwZhoS6d8WetD9frzGVKdXbh-thQUE4UqGSUkuscMWnq8G_zu6_MeX7Gh66hbINNukDXrQeKvP3A

=== Cleanup === ✓ Deleted chat sessions ✓ Deleted collection:
kotlin_chat_sessions_example

✓ Chat session management example completed successfully

BUILD SUCCESSFUL in 8s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to
> no-operation (NOP) logger implementation SLF4J(W): See
> https://www.slf4j.org/codes.html#noProviders for further details. === ekoDB
> Kotlin Client - Advanced Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
QyrffLSJVIb09z5dBcwP_OFkhNSpG3sYClkRfd-1l8i4yLAG948U00nF1hb4quvzXqP0O9Sq5O0s_BjOF3Wm4Q

=== Sending Initial Message === ✓ Message sent Responses: ["I'm sorry, but the
context provided does not include any information about the available products.
Could you please provide more details?"]

✓ Second message sent

=== Regenerating AI Response === ✓ AI response regenerated New responses: ["I'm
sorry, but the context provided doesn't contain any specific product
information. Could you please provide more details or clarify what kind of
products you're interested in?"]

=== Updating Message === ✓ Updated message content

=== Toggling Forgotten Status === ✓ Marked message as forgotten (excluded from
context)

=== Creating Second Session for Merge === ✓ Created second session:
KS_EdkIzOo9gg3gEof0UfmrOFm6qYnv6lt7RNYs6yumCLaWK5VjfrSM5Qi_tsvIJ4IOjVlHZKvaZK-Zc1lCILg

=== Merging Sessions === ✓ Merged sessions Merged session ID: null

=== Deleting Message === ✓ Deleted message

=== Cleanup === ✓ Deleted chat sessions ✓ Deleted collection:
kotlin_chat_advanced_example

✓ Advanced chat features example completed successfully

BUILD SUCCESSFUL in 10s 2 actionable tasks: 1 executed, 1 up-to-date To honour
the JVM settings for this build a single-use Daemon process will be forked. For
more on this, please refer to
https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon
in the Gradle documentation. Daemon will be stopped at the end of the build

> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED Task :compileKotlin
> UP-TO-DATE Task :compileJava NO-SOURCE Task :processResources NO-SOURCE Task
> :classes UP-TO-DATE

> Task :run 🚀 ekoDB Scripts Example (Kotlin Client)

SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to no-operation
(NOP) logger implementation SLF4J(W): See
https://www.slf4j.org/codes.html#noProviders for further details. ✅ Client
initialized

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved:
9eRim0u4tfo-fzQb2i88ACi4ytZUzGyEtFoSfhvGsoOtwBoS0Cesif-wUNOjTl1R8k13uUuIlTvcyEqAUt7nVw
📊 Found 10 records ⏱️ Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved 📊 Found 10 users (limited) ⏱️ Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved 📊 Statistics: 2 groups
{"count":5,"avg_score":60.0,"status":"active"}
{"avg_score":50.0,"status":"inactive","count":5} ⏱️ Execution time: 0ms

📝 Example 4: Script Management

📋 Script listing skipped (some scripts contain parameter placeholders) 🔍
Retrieved script: Get Active Users ✏️ Script updated 🗑️ Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

📝 Example 5: Multi-Stage Pipeline

✅ Multi-stage script saved 📊 Pipeline executed 2 stages ⏱️ Total execution
time: 0ms 📈 Stage breakdown:

📝 Example 6: Count Users

✅ Count script saved 📊 Total user count: 10 ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Deleted collection ⚠️ Script cleanup skipped (some scripts
contain parameter placeholders)

✅ All examples completed successfully!

💡 Key Advantages of Using the Client: • Automatic token management • Type-safe
Stage builders • Built-in error handling

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date ✅
[32mKotlin client examples complete![0m 🟣 [33mKotlin Transactions...[0m
/bin/sh: kotlinc: command not found Kotlin example (requires kotlinc installed)
✅ [32mAll Kotlin integration tests complete![0m
