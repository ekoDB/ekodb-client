make test-examples-rust 🧪 [36mRunning Rust examples (direct
HTTP/WebSocket)...[0m Finished `dev` profile [unoptimized + debuginfo] target(s)
in 0.28s Running `target/debug/examples/simple_crud` ✓ Authentication successful

=== Insert Document === Inserted: Object {"id":
String("uKzIQ7icrwktVPAABDxZ9_jrSWgYrUOcZL_X0qORPOa3-mVb_JnYIebXhfzcsElXXFdfRESW0GI4L7yVkBX73A")}

=== Find by ID === Found: Object {"active": Object {"type": String("Boolean"),
"value": Bool(true)}, "id":
String("uKzIQ7icrwktVPAABDxZ9_jrSWgYrUOcZL_X0qORPOa3-mVb_JnYIebXhfzcsElXXFdfRESW0GI4L7yVkBX73A"),
"name": Object {"type": String("String"), "value": String("Test Record")},
"value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query === Found documents: Array [Object {"active": Object
{"type": String("Boolean"), "value": Bool(true)}, "id":
String("uKzIQ7icrwktVPAABDxZ9_jrSWgYrUOcZL_X0qORPOa3-mVb_JnYIebXhfzcsElXXFdfRESW0GI4L7yVkBX73A"),
"name": Object {"type": String("String"), "value": String("Test Record")},
"value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document === Updated: Object {"active": Object {"type":
String("Boolean"), "value": Bool(true)}, "id":
String("uKzIQ7icrwktVPAABDxZ9_jrSWgYrUOcZL_X0qORPOa3-mVb_JnYIebXhfzcsElXXFdfRESW0GI4L7yVkBX73A"),
"name": Object {"type": String("String"), "value": String("Updated Record")},
"value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully Finished `dev` profile [unoptimized
+ debuginfo] target(s) in 0.10s Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data === ✓ Inserted test record:
"ZcYBdW4BqNkDzf5RbWUTc1jIg4MNTZSmLAIkKJV-qyVNSV9E7XHRzPgh3hNqd7aO-Wc1hgdTsZeNeSg32CNDwg"

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"ZcYBdW4BqNkDzf5RbWUTc1jIg4MNTZSmLAIkKJV-qyVNSV9E7XHRzPgh3hNqd7aO-Wc1hgdTsZeNeSg32CNDwg",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } } ] }, "type": "Success" } ✓ Retrieved 1
record(s) via WebSocket

✓ WebSocket example completed successfully Finished `dev` profile [unoptimized +
debuginfo] target(s) in 0.10s Running `target/debug/examples/batch_operations` ✓
Authentication successful

=== Batch Insert === ✓ Batch inserted 5 records ✓ Verified: Found 5 total
records in collection

=== Creating test records for update/delete === Created 3 test records

=== Batch Update === ✓ Batch updated 3 records ✓ Verified: Record updated with
status="active"

=== Batch Delete === ✓ Batch deleted 3 records ✓ Verified: Records successfully
deleted (not found)

✓ All batch operations completed successfully Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.10s Running
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
debuginfo] target(s) in 0.10s Running
`target/debug/examples/collection_management` ✓ Authentication successful

=== Create Collection (via insert) === Collection created with first record:
"nFD1rvaBYp_R1VcB4-PDu8rdcmgTB6fpZ8qZSlYPZZEg0pzWOK-k3SabN3a_kZnPtzl7DwDKdxgHMJ2aTDjRwg"

=== List Collections === Total collections: 4 Sample collections:
["websocket_test", "demo_collection", "test_collection", "batch_users"]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully Finished `dev`
profile [unoptimized + debuginfo] target(s) in 0.11s Running
`target/debug/examples/document_ttl` ✓ Authentication successful

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
"KRwkpnI_QDOvoi0jE7FFwkNVMPT37Ov2TtQ14pED7tEoMjnIjcy9W_wGbtHHBZbY6a5_4C-a3A4yaNQ57jqDKQ"

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: null

=== Query Documents === ✓ Found 1 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

✓ All document TTL operations completed successfully Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.10s Running
`target/debug/examples/websocket_ttl` ✓ Authentication successful

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
String("mYATAvpvEzhyl9jBlfHGwVqhW9nRRBqqkAfQ5v2W2ukJo3UkhoES-iJ5K1AzsA9ojpjcaipiv7t_tehHkirDfQ")

=== Query via WebSocket === ✓ WebSocket connected

✓ WebSocket TTL example completed successfully Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.10s Running
`target/debug/examples/http_functions` 🚀 ekoDB Scripts Example (Rust/HTTP)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved:
oPrLTmK1VBavTQsFtzZxYsrQwq2xp0NWB5Pdt6axqQ5juOS5N7hOgEQ43dARKUsAhnzW_ukV5yFe8Z9QqlfySQ
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved:
"N7wxvCZ8VWMFqKLaLtiYCkT-XQpkyxqdoPxsD9cNBQmBNrEU1Wyq65v2igQ7BPGRF7NawwDxT7T6iZZC5q8j9g"
📊 Page 1: Found 3 users (limit=3, skip=0) 📊 Page 2: Found 2 users (limit=3,
skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Script saved:
CZDaxrXR3J1a1TWW89hXKEC7zxNTLbkwbte3dGmbgwqUAmLPmxW-C6tX5PPZ3f6QtbI2zKgOxhJRAVNedXB6FQ
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved:
0tyq9G4MXqJhHAMVzt3ixlRJ35qs1TcadCK0i7mMjqgdqjj8sBiS7Z-mAUdTMqnruCE-0KuUXGZgcEwaHeVeng
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
{"avg_score":50.0,"count":5,"max_score":90,"status":"inactive"}
{"avg_score":60.0,"count":5,"max_score":100,"status":"active"}

📝 Example 4: Function Management

📋 Total scripts: 4 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

✅ All examples completed! Finished `dev` profile [unoptimized + debuginfo]
target(s) in 0.11s Running `target/debug/examples/transactions` ✓ Authentication
successful

=== Setup: Creating Test Accounts === Created Alice: $1000 - ID:
U1uk_Zniu2CetVEVgUPIhICLkhFvUDTaQOlew9M6Knyis8XqryT2g-qzFx-tc3idEYs5q6WZcJWoqrVSNitxVA
Created Bob: $500 - ID:
Z8o4MkrwClEq_L_ultHYPRFfJOtQbKzoEf0sUTR1PDspuiUod-OO5e_GMywIo60eD1P9Fo2VPLsLkRp5ZsYcZA

=== Example 1: Begin Transaction === Transaction ID:
bc620080-ee9f-47c1-a934-465f8da87009

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: "Active" Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: 800 Bob: 700

=== Example 5: Rollback === New transaction:
3b432f67-59bb-4ce5-9dab-707377cfa4ac Updated Bob: $700 → $600 (in transaction) ✓
Transaction rolled back Bob after rollback: 700

=== Cleanup === ✓ Deleted test accounts

✓ All transaction examples completed ✅ [32mRust direct examples complete![0m 🛠️
[36mBuilding client library...[0m cargo build -p ekodb_client Finished `dev`
profile [unoptimized + debuginfo] target(s) in 0.15s ✅ [32mClient build
complete![0m 🧪 [36mRunning Rust client library examples...[0m Finished `dev`
profile [unoptimized + debuginfo] target(s) in 0.10s Running
`target/debug/examples/client_simple_crud` ✓ Client created (token exchange
happens automatically)

=== Insert Document === Inserted: Record { fields: {"id":
String("aaAyMGqR_9PWOvGvlFpErsEv1qew5_JSbu4ga4CSIj4Sq1rjne6MXko1K7zxijt6Ez6Ij-eIVG5nLnaaS2O59g")}
}

=== Find by ID === Found: Record { fields: {"active": Object({"type":
String("Boolean"), "value": Boolean(true)}), "id":
String("aaAyMGqR_9PWOvGvlFpErsEv1qew5_JSbu4ga4CSIj4Sq1rjne6MXko1K7zxijt6Ez6Ij-eIVG5nLnaaS2O59g"),
"name": Object({"value": String("Test Record"), "type": String("String")}),
"value": Object({"value": Integer(42), "type": String("Integer")})} }

=== Find with Query === Found documents: [Record { fields: {"name":
Object({"value": String("Test Record"), "type": String("String")}), "value":
Object({"type": String("Integer"), "value": Integer(42)}), "active":
Object({"type": String("Boolean"), "value": Boolean(true)}), "id":
String("aaAyMGqR_9PWOvGvlFpErsEv1qew5_JSbu4ga4CSIj4Sq1rjne6MXko1K7zxijt6Ez6Ij-eIVG5nLnaaS2O59g")}
}]

=== Update Document === Updated: Record { fields: {"active": Object({"value":
Boolean(true), "type": String("Boolean")}), "name": Object({"type":
String("String"), "value": String("Updated Record")}), "id":
String("aaAyMGqR_9PWOvGvlFpErsEv1qew5_JSbu4ga4CSIj4Sq1rjne6MXko1K7zxijt6Ez6Ij-eIVG5nLnaaS2O59g"),
"value": Object({"value": Integer(100), "type": String("Integer")})} }

=== Delete Document === Deleted document

=== Cleanup === ✓ Deleted collection

✓ All CRUD operations completed successfully Finished `dev` profile [unoptimized
+ debuginfo] target(s) in 0.11s Running
`target/debug/examples/client_batch_operations` ✓ Client created

=== Batch Insert (via multiple inserts) === ✓ Inserted 5 records ✓ Verified:
Found 5 total records in collection

=== Update Records === ✓ Updated 3 records

=== Delete Records === ✓ Deleted 3 records

=== Cleanup === ✓ Deleted collection

✓ All batch operations completed successfully Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.11s Running
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
debuginfo] target(s) in 0.10s Running
`target/debug/examples/client_collection_management` ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
"llwMi1HMD9Cj61lHayTt0pS2Nj1iPxGH1ad1GwruDz2CDeoTgod1nSGXYcoi6Ua4_0QvO6YR38A78hTkZ3EwBA"

=== List Collections === Total collections: 9 Sample collections:
["websocket_test", "client_collection_management_rust", "test_accounts",
"scripts__ek0_bench", "test_collection"]

=== Count Documents === Document count: 1

=== Check Collection Exists === Collection exists: true

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully Finished `dev`
profile [unoptimized + debuginfo] target(s) in 0.11s Running
`target/debug/examples/client_document_ttl` ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
"p-EFxKmgf1AtUB_YniZORCWEXz95CRckT9sPhEif2EbSAVkgoO4ybXSpxIvROZO20qCekPdu1NtkzwAeDrs3ag"

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
Some(String("MHG4YcX0kJM1Rv1fDR2GZPNTzLbnzeoBjTu8v14vhottdWOwIE3lhOQ7qIQaW_e4T7ACVwv8EGUmTSn0fc4W5w"))

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
Running `target/debug/examples/client_simple_websocket` ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
1GxFMa7X10Hje07puEL-wAKD97edLuqIeFwje-CQfOJmq9ProiRe-5s3_LvExF6dxLkcZwgSb5mbM9D1fwJjsA

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully Finished `dev` profile [unoptimized +
debuginfo] target(s) in 0.11s Running
`target/debug/examples/client_websocket_ttl` ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
Some(String("eK_YVBdmcVsCYJLO9TSZZuUAra_iX_M1EUTe_ugnxdEZ_XVVb7n1sPbh7a8i_pxOZ1D0lTITcZwYPufzWeXMrw"))

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
Running `target/debug/examples/client_query_builder` === ekoDB Query Builder
Example ===

=== Inserting Sample Data === ✓ Inserted 5 users

=== Simple Equality Query === ✓ Found 3 active users

- Some(Object({"type": String("String"), "value": String("Bob")}))
- Some(Object({"value": String("David"), "type": String("String")}))
- Some(Object({"type": String("String"), "value": String("Alice")}))

=== Range Query (age >= 28 AND age < 35) === ✓ Found 3 users in age range

- Some(Object({"value": String("Eve"), "type": String("String")}))
- Some(Object({"type": String("String"), "value": String("Bob")}))
- Some(Object({"value": String("David"), "type": String("String")}))

=== IN Operator === ✓ Found 4 users with status active or pending

- Some(Object({"value": String("Bob"), "type": String("String")}))
- Some(Object({"type": String("String"), "value": String("Alice")}))
- Some(Object({"value": String("David"), "type": String("String")}))
- Some(Object({"value": String("Eve"), "type": String("String")}))

=== NOT IN Operator === ✓ Found 4 users not inactive

=== String Pattern Matching === ✓ Found 5 users with @example.com email

=== Regex Query === ✓ Found 0 users with names starting with A-C

=== Complex Query (active AND age >= 28 AND score > 1500) === ✓ Found 2 users
matching all conditions

- Some(Object({"type": String("String"), "value": String("Bob")}))
- Some(Object({"value": String("David"), "type": String("String")}))

=== OR Query === ✓ Found 2 users with age < 28 OR age > 32

- Some(Object({"type": String("String"), "value": String("Charlie")}))
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
[unoptimized + debuginfo] target(s) in 0.11s Running
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
   ["title", "title.value", "description.value", "description"]
2. Score: 2.0000 Title: Some(String("Python for Data Science")) Matched:
   ["description", "description.value"]
3. Score: 1.0000 Title: Some(String("Rust Programming")) Matched:
   ["description.value", "description"]
4. Score: 1.0000 Title: Some(String("Database Design")) Matched:
   ["description.value", "description"]

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
`dev` profile [unoptimized + debuginfo] target(s) in 0.11s Running
`target/debug/examples/client_schema_management` === ekoDB Schema Management
Example ===

=== Creating Collection with Schema === ✓ Created collection
'schema_client_rust' with schema

=== Inserting Valid Documents === ✓ Inserted user 1:
Some(String("denbK-4RR8bfRHjztoESpWlyQ8UnhJlbyiVRFSrq4oSMWOGdFCB7RxguXEnwt5iZIM1H310VaqSONcNOzWpvig"))
✓ Inserted user 2:
Some(String("7m5zx4IYCV89rKLRNLSXzg0mvx5WrIjjW1Znkta1soK_Rf4WyutPzZH-j2l4dTBRTiKMdDa4Ld-EDU0TlG-06A"))

=== Getting Schema === ✓ Schema for schema_client_rust:

- email: String (required)
- age: Integer
- status: String
- title: String (required)

=== Listing Collections === ✓ Total collections: 9 Sample: ["websocket_test",
"schema_client_rust", "test_accounts", "scripts__ek0_bench", "test_collection"]

=== Cleanup === ✓ Deleted collection

✓ All schema management operations completed successfully Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.11s Running
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
[unoptimized + debuginfo] target(s) in 0.11s Running
`target/debug/examples/client_chat_basic` === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
W9Ku4NfJAWIGjBvCJsMP2kNZAFEEvTkH8RenBjwisN141_0i-7ueIMCLe8CpfAgLZfO24z6cAVpzbYybitCyHA

=== Sending Chat Message === Message ID:
7R9NRxpKrQ5BMjCHAjThZ_PQVDYU7hVT1Um20xWgNGIoY20qqqRuNNQXdjOWO4_uKGt9xxt1zrx1DL6SNKFLPQ

=== AI Response === Response 1: I'm sorry, but the provided context does not
contain information about what ekoDB is or the features it has.

=== Context Used (3 snippets) ===

Snippet 1: Collection: client_chat_basic_rust Score: 0.6333 Matched Fields:
["category", "title", "content"] Record: Object {"category": Object {"type":
String("String"), "value": String("features")}, "content": Object {"type":
String("String"), "value": String("The chat feature allows you to query your
database using natural language and get AI-powered responses with relevant
context.")}, "id":
String("18rp03-1I6sFDFK7yy4UqzVJ5V7co9k6EP05_6IW4T_YsQ0lYGKKT23MqnaZ1_6IyXQWPB8xS3cUh9yyTefRTw"),
"title": Object {"type": String("String"), "value": String("AI Chat
Integration")}}

Snippet 2: Collection: client_chat_basic_rust Score: 0.5222 Matched Fields:
["content", "title"] Record: Object {"category": Object {"type":
String("String"), "value": String("documentation")}, "content": Object {"type":
String("String"), "value": String("ekoDB is a high-performance database with
intelligent caching, real-time capabilities, and AI integration.")}, "id":
String("Qifwl_FKz06OF5-TOcCI1DPyhewjjhlg70qBUtogR69LbEdplW_E_KunBvsJueRv4N3WHpVxuQqwyBGK5e-7CA"),
"title": Object {"type": String("String"), "value": String("Introduction to
ekoDB")}}

Snippet 3: Collection: client_chat_basic_rust Score: 0.5222 Matched Fields:
["category", "title", "content"] Record: Object {"category": Object {"type":
String("String"), "value": String("features")}, "content": Object {"type":
String("String"), "value": String("ekoDB supports full-text search, vector
search, and hybrid search with automatic context retrieval.")}, "id":
String("flI2zkhLSADL6y15ELJwDT3v6Bxle58bdYe1LtN7QPFwF2Z8qfUCfF1m6lau7H5nas7ehPrHlBY6lBVKPLK4Bg"),
"title": Object {"type": String("String"), "value": String("Search Features")}}

Execution Time: 1596ms

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully Finished `dev` profile [unoptimized + debuginfo]
target(s) in 0.11s Running `target/debug/examples/client_chat_sessions` ===
ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
LxDb9FFuDweBcT_fv86mFw6VBNhoehESDjYzen5_WpbOU7Dk8scFwalnDWwRWmY4Jd35oW7KoM_EF8KG739cig
=== Sending Messages === ✓ Message 1 sent Response: I'm sorry, but the provided
context doesn't contain specific information about any available products. Could
you please provide more details or context?

✓ Message 2 sent Response: I'm sorry, but the context provided does not contain
information about the price of a product. Could you please provide more details
or specify the product you're asking about?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
QUrWfxoB7VNuuUgvHL4QftXzeEru3xlYwkGbGuYHqrtm3OJFONNtiQsq8ycdKxm-39mttVwj_zbcrEmTXCAUew
Parent:
LxDb9FFuDweBcT_fv86mFw6VBNhoehESDjYzen5_WpbOU7Dk8scFwalnDWwRWmY4Jd35oW7KoM_EF8KG739cig

=== Listing Sessions === ✓ Found 3 sessions Session 1:
QUrWfxoB7VNuuUgvHL4QftXzeEru3xlYwkGbGuYHqrtm3OJFONNtiQsq8ycdKxm-39mttVwj_zbcrEmTXCAUew
(Untitled) Session 2:
LxDb9FFuDweBcT_fv86mFw6VBNhoehESDjYzen5_WpbOU7Dk8scFwalnDWwRWmY4Jd35oW7KoM_EF8KG739cig
(Untitled) Session 3:
W9Ku4NfJAWIGjBvCJsMP2kNZAFEEvTkH8RenBjwisN141_0i-7ueIMCLe8CpfAgLZfO24z6cAVpzbYybitCyHA
(Untitled)

=== Getting Session Details === ✓ Session details retrieved Messages: 4

=== Deleting Branch Session === ✓ Deleted branch session:
QUrWfxoB7VNuuUgvHL4QftXzeEru3xlYwkGbGuYHqrtm3OJFONNtiQsq8ycdKxm-39mttVwj_zbcrEmTXCAUew

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully Finished `dev`
profile [unoptimized + debuginfo] target(s) in 0.11s Running
`target/debug/examples/client_chat_advanced` === ekoDB Advanced Chat Features
Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
XHnTqoGF7l9cSJpTXHDS9A27aY4ISByFSkEZ1sIIwT58epKEIxQZukVyHfBGBEyxZoL0CLSY7M1vZ_ZzktgAhg

=== Sending Initial Message === ✓ Message sent Response: I'm sorry, but the
context provided does not include information about any specific products. Could
you please provide more details or context?

✓ Second message sent === Feature 1: Regenerate AI Response === ✓ Message
regenerated New response: Apologies, but the provided context doesn't contain
information about the price of ekoDB. I recommend checking the official website
or contacting their sales department to get the most accurate and current
pricing information.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
pC_Hvla-W6X4dKvY4WkJu4fHKzUoGMR1u2f-5FM1qJvi3iON0PEl6jCJjAEIWgpAThKQIjskKUCCOt0Ad51qMQ
✓ Sent message in second session ✓ Sessions merged successfully Total messages
in merged session: 4

=== Feature 5: Delete Message === ✓ Message deleted

✓ Messages remaining: 3

=== Cleanup === ✓ Deleted session ✓ Deleted collection

✓ All advanced chat features demonstrated successfully! Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.11s Running
`target/debug/examples/client_functions` === ekoDB Rust Client - Scripts Example
===

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved:
hVWM8BazJ68ynI5j_bilm0lGW6sBe2szHpjq7YnmcZcKsJemez6dsU-S3SNFtRy8CHJJFsF9sakiKtn1z9r5rA
📊 Found 20 active users

📝 Example 2: Parameterized Script

✅ Script saved:
5Yy-q4F7K7W_Owd5EKgKsXzUS3r6DHyhjabM7rKVdPpGSYqj1Dz8zvV9xDluTd5NWzUjjkxeeaMyBdRZ2P34UQ
📊 Found 20 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved:
77wIHAJtNcTTdFvCKGUlrWHVENk2AFGOv41AmR8i05lPpuGwtF8NydBiAvC5Rf82wKzs4LvLpGl33cdkcVqTRA
📊 Statistics: 2 groups

📝 Example 4: Script Management

📋 Total scripts: 6 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed! ✅ [32mRust client examples complete![0m
