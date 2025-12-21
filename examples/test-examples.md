make test-examples 🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.28s Running
`target/debug/examples/simple_crud` ✓ Authentication successful

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

✅ All examples completed! ✅ [32mRust client examples complete![0m ✅ [32mAll
Rust integration tests complete![0m 🧪 [36mRunning Python examples (direct
HTTP/WebSocket)...[0m ✓ Authentication successful

=== Insert Document === Inserted: {'id':
'Sgu5HrD4eF-wLJ1nZ3ggvsPNUCewPkWIFbAYh9gySVTtsjYRtTGeHLvP3jKJJ1JiSnJ4TNhq8nE3dC_poeu1Yg'}

=== Find by ID === Found: {'id':
'Sgu5HrD4eF-wLJ1nZ3ggvsPNUCewPkWIFbAYh9gySVTtsjYRtTGeHLvP3jKJJ1JiSnJ4TNhq8nE3dC_poeu1Yg',
'value': {'value': 42, 'type': 'Integer'}, 'name': {'value': 'Test Record',
'type': 'String'}, 'active': {'type': 'Boolean', 'value': True}}

=== Find with Query === Found documents: 1

=== Update Document === Updated: {'value': {'type': 'Integer', 'value': 100},
'id':
'Sgu5HrD4eF-wLJ1nZ3ggvsPNUCewPkWIFbAYh9gySVTtsjYRtTGeHLvP3jKJJ1JiSnJ4TNhq8nE3dC_poeu1Yg',
'active': {'type': 'Boolean', 'value': True}, 'name': {'type': 'String',
'value': 'Updated Record'}}

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully ✓ Authentication successful

=== Inserting Test Data === ✓ Inserted test record:
qQFo7AuYWumBE-nyEtkv0vnDprHbBQs6MyvSoxLr6x1dPGKK912zpqx7rJPxDOhxF1hw2rZdIWEl5BK9pSW0TA

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"ZcYBdW4BqNkDzf5RbWUTc1jIg4MNTZSmLAIkKJV-qyVNSV9E7XHRzPgh3hNqd7aO-Wc1hgdTsZeNeSg32CNDwg",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"qQFo7AuYWumBE-nyEtkv0vnDprHbBQs6MyvSoxLr6x1dPGKK912zpqx7rJPxDOhxF1hw2rZdIWEl5BK9pSW0TA",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } } ] }, "type": "Success" } ✓ Retrieved 2
record(s) via WebSocket

✓ WebSocket example completed successfully ✓ Authentication successful

=== Batch Insert === ✓ Batch inserted 5 records ✓ Verified: Found 10 total
records in collection

=== Creating test records for update/delete === Created 3 test records

=== Batch Update === ✓ Batch updated 3 records ✓ Verified: Record updated with
status="{'type': 'String', 'value': 'active'}"

=== Batch Delete === ✓ Batch deleted 3 records ✓ Verified: Records successfully
deleted (not found)

✓ All batch operations completed successfully ✓ Authentication successful

=== KV Set === ✓ Set key: session:user123

=== KV Get === Retrieved value: {'userId': 123, 'username': 'john_doe'}

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: {'name': 'Product 1', 'price': 29.99}
cache:product:2: {'name': 'Product 2', 'price': 39.989999999999995}
cache:product:3: {'price': 49.989999999999995, 'name': 'Product 3'}

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Authentication successful

=== Create Collection (via insert) === Collection created with first record:
bng5vlD9qS3H3bVMplnxDynTte9IK4swlyOE-eWkCxT4fsvip3JNKt7xONMAg2f5BUvqahGFvaaVp8LxbtoTjg

=== List Collections === Total collections: 11 Sample collections:
['websocket_test', 'demo_collection', 'test_accounts', 'scripts__ek0_bench',
'test_collection']

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: False

✓ All collection management operations completed successfully ✓ Authentication
successful

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
GkJwQWqmN-qt4jQuEDmPlLid01HMX8MY4rORU9JeuNcKincTgyaQNvtf7zJ7GjtRZAOc5w6XnQwUaAyNPETX4Q

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully ✓ Authentication successful

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
wykJw6OoGbwdGsE6_BGVpep7fITbUArm4FNPXSbllHkZbe1YltufxNtNOXtrXIo03dcY_xrxnqqApm3lxhGXkg

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 2 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully 🚀 ekoDB Scripts Example
(Python/HTTP)

✓ Authentication successful

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved:
KCUaKvDGQAM1a5S3buVFfvzulkJ6scx8cKVRqTVUVquEDHy7r4d8Hs9_LT1eSh2iI2N1Z4uq_OxJPDriJK6zPQ
📊 Found 30 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved:
\_7tms6mo7-cd5pW3gUBPgZ4_dccXirPs4qSXPExnFRh0WrRGYe9gv2isFX6QV28SV5lsnJZoENZez5a4RhUmEA
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved:
-NAQowjzjOcComOWvfpQjIdHjexJ1rxlHo24aIALnLHb3bJm_DfTpT61nxDu7pHXVGRl-DXmjd4CB7YjDN6ovg
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
{'avg_score': 50.0, 'count': 15, 'max_score': 90, 'status': 'inactive'}
{'count': 15, 'status': 'active', 'avg_score': 60.0, 'max_score': 100}

📝 Example 4: Function Management

📋 Total scripts: 8 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

✅ All examples completed! ✓ Authentication successful

📋 Getting original configuration... Original durable_operations: False

============================================================ 🔥 TEST 1: Original
Config (durable=False)
============================================================

=== Setup: Creating Test Accounts === Created Alice: $1000 - ID:
oW3c1V0M4eb3a7zJTAYYONhsighdduLEsdT0FAACkDZQZogPieNMVP3-ZBj8-aEsxlNcy_ln6qra6H7PzghpuA
Created Bob: $500 - ID:
yob07jtjPF9kEQlPiXv7EpwHw4ESluC2tpHQPcSaEr5zh2VSzZ6iP2lXFKGXfieXxC1g3Qf_YmObCnaVqetT3g

=== Example 1: Begin Transaction === Transaction ID:
62485035-eda9-470d-8197-231ccd964204

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: Active Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: $800 Bob: $700

=== Example 5: Rollback === New transaction:
508efd9b-340f-4fe3-b538-f46063aab94c Updated Bob: $700 → $600 (in transaction) ✓
Transaction rolled back Bob after rollback: $700

=== Cleanup === ✓ All transaction examples completed === Setup: Creating Test
Accounts === Created Alice: $1000 - ID:
L_o19ky7yJ9VHJONlP_xEJ18Qpgcuzgc8LM0nz9eR12KEYq4YfrgYSTm7XAh_5ghMZSMT4Xp-eWUkxwIkrAhyg
Created Bob: $500 - ID:
gBVrfMzeiRmSkM4NL4JhJMXpMTT6CizFapz1n0lJiPuy8sxVMcCBFpgHdaRWI1h9veRmhABFEsAP0MzJlp8mTQ

=== Example 1: Begin Transaction === Transaction ID:
e9215966-7258-4d83-83e3-f37d3bdc0d8e

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: $800 Bob: $700

=== Cleanup === ✓ Deleted test accounts

🔄 Switching to NON-DURABLE mode... ✓ Config updated: durable_operations=false

============================================================ 🔥 TEST 2:
Non-Durable Mode (durable=false)
============================================================

=== Setup: Creating Test Accounts === Created Alice: $1000 - ID:
9hCrWPJ0wNXxa-0ot2G_fwRoa16o-vfxUqViRB4bA4agmbqNvu-IWW9iiczRIt1n6i4HeqrguZ1y56Rgkctcyw
Created Bob: $500 - ID:
KgYSi0Y_dmuq4_55ueF7pVlBz3Aeo3ahEk4hoxJZ7yfXdh2FJLsL0du3Z2gxGKb9XxcjWT0JK4uyCM6fXrjLHw

=== Example 1: Begin Transaction === Transaction ID:
741ce6a5-9e2d-45da-a23e-b9f565bdd977

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: Active Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: $800 Bob: $700

=== Example 5: Rollback === New transaction:
e019ae88-fe84-4f6b-9672-9b37d0551978 Updated Bob: $700 → $600 (in transaction) ✓
Transaction rolled back Bob after rollback: $700

=== Cleanup === ✓ All transaction examples completed === Setup: Creating Test
Accounts === Created Alice: $1000 - ID:
2H5fj8f4utQS4HCbQmqC7oFWlCZDJn-Q06352hzNlKJWS8jta_xhIP-UNNdi5caf1WLEp2v5i7JmNj2A38X3Lg
Created Bob: $500 - ID:
XM0RfBZIeSFhOTsG-LIyGz_S4tvPOUEByHKyXH-IdY1VDy7X_3zFTUoqkhvhWakANIm4kQiogguEXOYdlblUsw

=== Example 1: Begin Transaction === Transaction ID:
fc93b75c-8153-4f59-910e-b0504d6e5132

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: $800 Bob: $700

=== Cleanup === ✓ Deleted test accounts

🔄 Switching to DURABLE mode... ✓ Config updated: durable_operations=true

============================================================ 🔥 TEST 3: Durable
Mode (durable=true) ============================================================

=== Setup: Creating Test Accounts === Created Alice: $1000 - ID:
\_OyCtQm3VvLCGVKrGX4E0nnLVCvJ6edOcSx5TtT98YUbVSegolYcfCTquuBp7CegUt7ljSMjEIdk1bqQRPmE9g
Created Bob: $500 - ID:
D5DM_iqXmF96XRSi-vLcfA-5-7ApxfUgoFSlg9V1Ncbffy-wEsOFNvJwUA6_tHza-MHvTsfTdrQ9XKE1FnOMIg

=== Example 1: Begin Transaction === Transaction ID:
561b37af-7d67-49da-b2c7-64884fa8338a

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: Active Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: $800 Bob: $700

=== Example 5: Rollback === New transaction:
e2f056d8-a2be-4692-b70c-ab4e44112441 Updated Bob: $700 → $600 (in transaction) ✓
Transaction rolled back Bob after rollback: $700

=== Cleanup === ✓ All transaction examples completed === Setup: Creating Test
Accounts === Created Alice: $1000 - ID:
zE2cAhfLoYzKC7xXYllGur6VD-5do4jhDYXLQ6C60q1Yl8b1qjDzPXofh0TPGPNMFi3CPQEoWKDh3aC8Z6vN1g
Created Bob: $500 - ID:
e0rlnPhOg2hHZd5PJlSAoOlP7HtNqqh-fKeCpi6Qw5AD9O-hi5QGZSgzUQbDmn7Y4Z4RvCyDz1ce8oT3r9vzyg

=== Example 1: Begin Transaction === Transaction ID:
fcd8ee4d-8c43-4164-b3d5-b76a4bcf2092

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: $800 Bob: $700

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
LLA9inGkXisNwy3XpzoeuOaF6vHB7Z5aNQADm1eF1Gb3aM10qEFaDs6QJgaAOAJXLXj5LWaO09bRzZC80RaOlA

2️⃣ Calling Script (Insert + Verify)... ✅ Script executed: 2 Functions ⏱️
Execution time: 0ms

3️⃣ Verification Results: ✅ Found 1 record(s) 📋 User ID:
bVxqldxn_UhA7eXlltdsPw8T57ruRw3cMG6GyDr0WNUWW6W4OAsmyxwN8fUP4PS8hpNE94urn_ZQ0pBmEsSCFw
📋 Name: {'value': 'Alice Smith', 'type': 'String'} 📋 Email: {'value':
'alice@example.com', 'type': 'String'} 📋 Status: {'value': 'pending', 'type':
'String'} 📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================ 📝 Script 2:
Query + Update + Verify
============================================================

1️⃣ Saving Script... ✅ Script saved:
FmiAgxuOUY-Qi1gclZjEyMU5XfCBkRC_gb0-tTAAu6be1N1ImoIe1GG\_\_KdftXkAFKdWoO24fM32AySOXRgmLg

2️⃣ Calling Script (Query + Update + Verify)... ✅ Script executed: 3 Functions
⏱️ Execution time: 0ms

3️⃣ Verification Results: ✅ Found 1 record(s) 📋 Status updated to: {'value':
'active', 'type': 'String'} 📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================ 📝 Script 3:
Query + Update Credits + Verify
============================================================

1️⃣ Saving Script... ✅ Script saved:
Ri4wlBGH1SkE4Tat3FLLZOQaY1NP_xnItMjm19g_R5fIMQZUpHYy9MYbZrwBV1wMDuQ_J20gobUXmLTuJd7V5g

2️⃣ Calling Script (Query + Update Credits + Verify)... ✅ Script executed: 3
Functions ⏱️ Execution time: 0ms

3️⃣ Verification Results: ✅ Found 1 record(s) 📋 Credits updated to: {'value':
0, 'type': 'Integer'} 📋 Status: {'value': 'active', 'type': 'String'} 📋 Name:
{'value': 'Alice Smith', 'type': 'String'}

============================================================ 📝 Script 4: Query
Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script... ✅ Script saved:
nqSQToD8hm3Njg6d3fUcdeTUklagpi1GMcKuuR2ZVJcwj3aO6gD92mSiJiHU8H_hzqCfiyNnr61PvnYdGcAf2w

2️⃣ Calling Script (Query + Delete + Verify)... ✅ Script executed: 3 Functions
⏱️ Execution time: 0ms

3️⃣ Verification Results: 📊 Before delete: Record existed (verified by first
Query) ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================ 🧹 Cleanup
============================================================ ✅ Deleted script:
LLA9inGkXisNwy3Xpzoe... ✅ Deleted script: FmiAgxuOUY-Qi1gclZjE... ✅ Deleted
script: Ri4wlBGH1SkE4Tat3FLL... ✅ Deleted script: nqSQToD8hm3Njg6d3fUc... ✅
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
default Finished `release` profile [optimized] target(s) in 0.16s 📦 Built wheel
for abi3 Python ≥ 3.8 to
/Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.4.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m Processing
./ekodb-client-py/target/wheels/ekodb_client-0.4.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client Attempting uninstall: ekodb-client
Found existing installation: ekodb_client 0.4.0 Uninstalling ekodb_client-0.4.0:
Successfully uninstalled ekodb_client-0.4.0 Successfully installed
ekodb-client-0.4.0 ✅ [32mPython client package built and installed![0m 🧪
[36mRunning Python client library examples...[0m ✓ Client created (token
exchange happens automatically)

=== Insert Document === Inserted: {'id':
'OIpbl5lfka9L5F-8D6yh93oscIEP323DXVLlcjGv6jcsbLT6yroILgbb2AsiXYfkGorfpAuNYBOkwCETaUh33g'}

=== Find by ID === Found: {'value': {'type': 'Integer', 'value': 42}, 'name':
{'type': 'String', 'value': 'Test Record'}, 'id':
'OIpbl5lfka9L5F-8D6yh93oscIEP323DXVLlcjGv6jcsbLT6yroILgbb2AsiXYfkGorfpAuNYBOkwCETaUh33g',
'active': {'value': True, 'type': 'Boolean'}}

=== Find with Query === Found documents: 1

=== Update Document === Updated: {'active': {'value': True, 'type': 'Boolean'},
'value': {'value': 100, 'type': 'Integer'}, 'name': {'value': 'Updated Record',
'type': 'String'}, 'id':
'OIpbl5lfka9L5F-8D6yh93oscIEP323DXVLlcjGv6jcsbLT6yroILgbb2AsiXYfkGorfpAuNYBOkwCETaUh33g'}

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
"ZNuL0gE-Ju38JEYHeD1vzgU-F2Qcn39i8q5v345727YGtjYCAPYF8aWhANOLkdI4bGupMBMi9YAJhXfx3N4VdA"

=== List Collections === Total collections: 10 Sample collections:
['websocket_test', 'test_accounts', 'scripts__ek0_bench', 'test_collection',
'ttl_cache']

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: False

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
jCkXFplMxGQPc1zQ_TNr04QLZVbS-UhtZwYFyo7okqB5R6NKac6wvK8hzIBGqCAt_7tZsVNXJ0ALKgJihmSxWQ

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
cBuYWY9TiAsRSkC7OzR84Ch4dJ9i-ppGozhJJLpbUpDsVA0dlZ-JSQq40WjGIrLTbNzNvx9692Y-7EgIq6V9HQ

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
JQllzjvfNYe6uH9LAueZ_5--RS4piMwmYbXTR82gRupWHxpXF1i6hlI1LD_q53I5E3keqqiIfyjCUPgmHreHZw

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
DZ5HH2PrUA6L_Q2cVxwNlGAsZAboV5VxryKDZ22ijEMZC4nOLXY2argr5FDLs-J-su0jzyrO3SZRx50b0hqCQQ

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
1. Score: 25.740, Matched: name, name.value, email.value, email
1. Score: 12.540, Matched: name, name.value

1. Fuzzy search (typo tolerance): Found 4 results with fuzzy matching
1. Score: 26.400, Matched: bio.value, title, bio, title.value
1. Score: 26.400, Matched: title, bio, title.value, bio.value
1. Score: 26.400, Matched: title.value, bio, bio.value, title
1. Score: 26.400, Matched: bio, title.value, title, bio.value

1. Search with field weights: Found 4 results with weighted fields
1. Score: 39.600, Matched: title, bio, title.value, bio.value
1. Score: 39.600, Matched: bio.value, title.value, bio, title
1. Score: 39.600, Matched: bio.value, bio, title, title.value
1. Score: 39.600, Matched: title.value, title, bio, bio.value

1. Search with minimum score threshold: Found 2 results with score >= 0.3
1. Score: 13.200, Matched: bio.value, bio
1. Score: 13.200, Matched: bio.value, bio

1. Search with stemming and exact match boosting: Found 2 results (matches: run,
   running, runs)
1. Score: 13.200, Matched: bio.value, bio
1. Score: 13.200, Matched: bio, bio.value

1. Vector search (semantic search): Found 3 semantically similar documents
1. Score: 0.746
1. Score: 0.732
1. Score: 0.710

1. Hybrid search (text + vector): Found 3 results using hybrid search (text +
   vector)
1. Score: 1.697, Matched: title, content, title.value, content.value
1. Score: 1.122, Matched: content.value, title.value, content, title
1. Score: 0.513, Matched:

1. Case-sensitive search: Found 1 results (case-sensitive)
1. Score: 13.200, Matched: title, title.value

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

✅ Join operations examples completed! === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
LCHZxuXi5of0QvhmSm0wm2f0WqfUiFUL6umnxwqTLFma71kgbt2sXS9UnwTggcBS-gSdQZqtf466qGxqUAT_hg

=== Sending Chat Message === Message ID:
mnpB0g-rJvXOy2kWuPYghDlgaWWBn1suxLD_owkIdaXU6HpQsPLfDcdg2CksAguScjVB2sI30LspYHhI1LGFqQ

=== AI Response === I'm sorry, but the context provided does not include
specific information about the names of the products or their prices. Could you
please provide more information or clarify your question?

Execution Time: 1226ms

=== Token Usage === Prompt tokens: 324 Completion tokens: 34 Total tokens: 358

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
Kw60gQSthrKhDTfMuRtxqAQIyQTjGlsO4MUe8y65_FJHpsNu1vRm7nOJP10OKVfro3MzZkmxpCCQUhBStMMfhg

=== Sending Messages === ✓ Message 1 sent Response: I'm sorry, but the context
provided does not contain specific information about any available products.
Could you please provide more details?

✓ Message 2 sent Response: Apologies, but the context provided doesn't contain
information about the price of any product. Could you please specify the product
you're inquiring about?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
CQjUNeaSdiKP1ShvAlRIwPDt1I4W8xk4gcaoHLC0dJjsF2OlERdFi7HLpWj7w9WLGUzkb9IbIrnDN4xUWhGtug
Parent:
Kw60gQSthrKhDTfMuRtxqAQIyQTjGlsO4MUe8y65_FJHpsNu1vRm7nOJP10OKVfro3MzZkmxpCCQUhBStMMfhg

=== Listing Sessions === ✓ Found 6 sessions Session 1:
CQjUNeaSdiKP1ShvAlRIwPDt1I4W8xk4gcaoHLC0dJjsF2OlERdFi7HLpWj7w9WLGUzkb9IbIrnDN4xUWhGtug
(Untitled) Session 2:
Kw60gQSthrKhDTfMuRtxqAQIyQTjGlsO4MUe8y65_FJHpsNu1vRm7nOJP10OKVfro3MzZkmxpCCQUhBStMMfhg
(Untitled) Session 3:
LCHZxuXi5of0QvhmSm0wm2f0WqfUiFUL6umnxwqTLFma71kgbt2sXS9UnwTggcBS-gSdQZqtf466qGxqUAT_hg
(Untitled) Session 4:
pC_Hvla-W6X4dKvY4WkJu4fHKzUoGMR1u2f-5FM1qJvi3iON0PEl6jCJjAEIWgpAThKQIjskKUCCOt0Ad51qMQ
(Untitled) Session 5:
LxDb9FFuDweBcT_fv86mFw6VBNhoehESDjYzen5_WpbOU7Dk8scFwalnDWwRWmY4Jd35oW7KoM_EF8KG739cig
(Untitled) Session 6:
W9Ku4NfJAWIGjBvCJsMP2kNZAFEEvTkH8RenBjwisN141_0i-7ueIMCLe8CpfAgLZfO24z6cAVpzbYybitCyHA
(Untitled)

=== Deleting Branch Session === ✓ Deleted branch session:
CQjUNeaSdiKP1ShvAlRIwPDt1I4W8xk4gcaoHLC0dJjsF2OlERdFi7HLpWj7w9WLGUzkb9IbIrnDN4xUWhGtug

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
mfdanfZKSENFyTjCNKYTs8qxBOYrCLy9NhV27zEp-CJokU7s1eZZoZvtjLD_PX4qEwr1Fua3Nqb_cqvxzDsU2g

=== Sending Initial Message === ✓ Message sent Response: I'm sorry, but the
context provided doesn't contain specific information about any products. Could
you please provide more details?

✓ Second message sent

Debug: Found 4 messages Debug: First message keys: dict_keys(['updated_at',
'id', 'token_usage', 'created_at', 'role', 'content', 'chat_id']) Debug: First
message role: {'type': 'String', 'value': 'assistant'} === Feature 1: Regenerate
AI Response === ✓ Message regenerated New response: Answer: I'm sorry, but the
context provided does not include information on the price of ekoDB. You might
want to check the official website or contact the sellers for this information.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
j0ZNc8BrbxhRUDgb2Fwn4wHDCMGkwyehQjCVIE_5utey3QYCVumADwRrDHTrdEJPdutXXUHL_TIEvjy9tas1ig
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
qA6PohHSLGP5gU0p5jW0qxuO8HhjI-0uwezEZ9tJIK-9p55zM_b3vsr9GnJfjn1617Pp7r44We8uZwpDAGRaAQ
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved:
lXH9rYIVT7_nRuHuw6LIIr_AvyHgm_ZmKW-F6XIwgrOd7wPVKJM1kmJhUNxHLa0PyrmfRECrbQSxNokCDMUZdw
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved:
SD4wF7xWiUBl0oV5oUPa6RyjtkbulsUGvc5qQ9mcBmpI9TBld5ttbHLxlqaegozamc9AFzZkwF6_X-YqX4ecFw
📊 Statistics: 2 groups {'avg_score': 60.0, 'count': 5, 'status': 'active'}

{'avg_score': 50.0, 'count': 5, 'status': 'inactive'}

📝 Example 4: Script Management

📋 Total scripts: 10 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed! 🚀 ekoDB Python Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data... ✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved:
JLkVkUHAeu5iT4SG2mX46Birk_xQxw4FVjjlyGT0biy2GsYrWY3Fl7SQ6xkGSqT_WVDQMghA19iTyiw49_skUg
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
map[id:tlk6WdYqK5gukJ805WuUoHuZIGbBQLjFaQaJnursOBjyth7jaD7NSR-UBi3aFu5Ee_36-w8h-jyjhO0q2FTYBQ]

=== Find by ID === Found: map[active:map[type:Boolean value:true]
id:tlk6WdYqK5gukJ805WuUoHuZIGbBQLjFaQaJnursOBjyth7jaD7NSR-UBi3aFu5Ee_36-w8h-jyjhO0q2FTYBQ
name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query === Found 1 documents

=== Update Document === Updated: map[active:map[type:Boolean value:true]
id:tlk6WdYqK5gukJ805WuUoHuZIGbBQLjFaQaJnursOBjyth7jaD7NSR-UBi3aFu5Ee_36-w8h-jyjhO0q2FTYBQ
name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully [32m✓ simple_crud.go completed
successfully[0m [34m === Running simple_websocket.go ===[0m ✓ Authentication
successful

=== Inserting Test Data === ✓ Inserted test record:
UAcR9Yc_QZABIRubc0RoEp8eawAI-On9ZRl3kxmnfsYBDHLvLFzM9aWMaEkGLksmTlL4s2AxDw-B-UvQT1QkXQ

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"ZcYBdW4BqNkDzf5RbWUTc1jIg4MNTZSmLAIkKJV-qyVNSV9E7XHRzPgh3hNqd7aO-Wc1hgdTsZeNeSg32CNDwg",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"qQFo7AuYWumBE-nyEtkv0vnDprHbBQs6MyvSoxLr6x1dPGKK912zpqx7rJPxDOhxF1hw2rZdIWEl5BK9pSW0TA",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"UAcR9Yc_QZABIRubc0RoEp8eawAI-On9ZRl3kxmnfsYBDHLvLFzM9aWMaEkGLksmTlL4s2AxDw-B-UvQT1QkXQ",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } } ] }, "type": "Success" } ✓ Retrieved 3
record(s) via WebSocket

✓ WebSocket example completed successfully [32m✓ simple_websocket.go completed
successfully[0m [34m === Running batch_operations.go ===[0m ✓ Authentication
successful

=== Batch Insert === Batch insert response: map[failed:[]
successful:[czuLU_qVD5ikwGaX1RidQ4r_E9W4wX_-B8I-qhTPiewib0jvEh_JihuPAhw3Qdo3Om2TfKkTlsgEaJpDOkFIBQ
OKlAA7yaFleduor83CE_FjDr2unGgnSbWiCsLtWxVuOUMQJ9-G3wR841xHotBkkiOvAX0wg27g1JSd0ZmDlxHA
6sFsFliuYPOEjcxZunySYkv8PR8526U1OIHFFdQB6seGvpQgcLlgtBhTrR9lPumhrQojVuQGVijZmtbn-O5f-w
CbR2HbLZJ80Hk30dpUM0voXgkSuq30jY7YdWLd96lwKpmU14wbGSoof0_7GjcaS0ek-iXQLOoEd8zilN2FqidQ
fkhqG0QNiwUWt6TTTitTCuCmBnDPPSh4BScaV5aTj-z0Gf2Dj8H7tdswtX-2le9myAQIwYHMuaGHALQAh-UkoQ]]
✓ Batch insert completed

=== Creating test records for update/delete === Created 3 test records

=== Batch Update === Batch update response: map[failed:[]
successful:[sTrToZfyqMlw7pJLBP15EHR8wYQnU0ghY5CGk4hxhUDGfmeoHYBQQXUPKQ-f9zsDdudPYJH9mEUjELQExH5Weg
pfmbVCsKEotyzmgOBo-eeZfRU_bge-BhKTV6YshIcTMXvjpGxvpAoaDt635rtzNF3RkBnY8Td1iJ0fjlnixycA
CcTJzis4GCZCxzU18Pimt9tokMTq49DuyVV2VA9EZ0MGqvpiFLh0xiTnhiUo_r8sVEzptgD95s_PSbdyimVUFA]]
✓ Batch update completed

=== Batch Delete === Batch delete response: map[failed:[]
successful:[sTrToZfyqMlw7pJLBP15EHR8wYQnU0ghY5CGk4hxhUDGfmeoHYBQQXUPKQ-f9zsDdudPYJH9mEUjELQExH5Weg
pfmbVCsKEotyzmgOBo-eeZfRU_bge-BhKTV6YshIcTMXvjpGxvpAoaDt635rtzNF3RkBnY8Td1iJ0fjlnixycA
CcTJzis4GCZCxzU18Pimt9tokMTq49DuyVV2VA9EZ0MGqvpiFLh0xiTnhiUo_r8sVEzptgD95s_PSbdyimVUFA]]
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
UPhEvynFquapjgme1PTyaxdlOEfj0mfw0g8oew0Ut3YV-1F13ZfS-wR6XzPQijoKz4HQfrgh76DfggkevoeTBQ

=== List Collections === Total collections: 15 Sample collections:
[schema_documents_client_py websocket_test schema_employees_client_py
demo_collection test_accounts schema_users_client_py scripts__ek0_bench
schema_products_client_py test_collection users ttl_cache batch_users
ws_ttl_test chat_messages__ek0_bench chat_configurations__ek0_bench]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully [32m✓
collection_management.go completed successfully[0m [34m === Running
document_ttl.go ===[0m ✓ Authentication successful

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
d1f2JEeheqSZS9biJCU9kR51FAev70aGiQLeCl0RxLMNL7DrVv5re8IkrTvxVdwfYIcBaVuqs9c8a5G-4Z0_1A

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 3 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully [32m✓ document_ttl.go
completed successfully[0m [34m === Running websocket_ttl.go ===[0m ✓
Authentication successful

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
s_f-XMdIkaZd2d89o453lIFUpr46WMRB3XghjKPoGWPTdpIktLBXue4PVnHm9mfKemrEPv4m3kXRlRKvsdd4Xw

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 3 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully [32m✓ websocket_ttl.go completed
successfully[0m [34m === Running http_functions.go ===[0m 🚀 ekoDB Scripts
Example (Go/HTTP)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved:
yul-hOhIdRJGIjm6OrEU3Xc6rTfMBKHDTbBEVrmueLqVHuoOL-kmYj4Nlt3t_XC-EhrSuDruN98Nxe-IQawaeA
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved:
XcK0o97R6gTXbd6lwNnfgZx_xQEIdzMLkPSPudtjS82YM4umPFH9IIuSDpRgSkT2WPK0cqBXMnFu6Bjr4GjpFg
📊 Page 1: Found 3 users (limit=3, skip=0) 📊 Page 2: Found 3 users (limit=3,
skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved:
7_kVCaDPplmViw3G9dvTwdbl5d7niF4lz19wbzgqPcc53Xm4JSjeKImxAtxfavfH3I8uq_wybRwrrfsnZ-3C4g
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
{"avg_score":50,"count":10,"max_score":90,"status":"inactive"}
{"avg_score":60,"count":10,"max_score":100,"status":"active"}

📝 Example 4: Function Management

📋 Total scripts: 12 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

✅ All examples completed! [32m✓ http_functions.go completed successfully[0m
[34m === Running transactions.go ===[0m ✓ Authentication successful

=== Setup: Creating Test Accounts === Created Alice: $1000 - ID:
dRvjfo_eayWPAM5agAC3TA_9k8W3WRmkhVz7DgmxNmABxAcG_xBfB2BK080E64_U1JCYynbnXjsy1UKOnqYL_A
Created Bob: $500 - ID:
WXobgCOQPhrtPnOXCEBUcu-jkPokg30DyxSTyAeKUYLNYDwZegwlDX7XXS4UTCHL1wmt3L7uZ8FE0HL6vicXXw

=== Example 1: Begin Transaction === Transaction ID:
160f5791-ff61-4533-beec-d940d9db1bf3

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: Active Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: 800 Bob: 700

=== Example 5: Rollback === New transaction:
f845326f-5ac7-4e0b-9afa-f2d68426eab9 Updated Bob: $700 → $600 (in transaction) ✓
Transaction rolled back Bob after rollback: 700

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
UVuX_U382_XG6KrPrVZlxpGJUspRtlteuyeeDqgn7F2LTVYdPPEkd147J7B-ehq0fgzbmoEYVAH2f8TPybnzfg

2️⃣ Calling Script (Insert + Verify)... ✅ Script executed: 2 Functions ⏱️
Execution time: 0ms

3️⃣ Verification Results: ✅ Found 1 record(s) 📋 User ID:
pZQZ58jttHPNuM_fgGlEf5D41HRE64h8OZMYEjvQ-ZVMMBXDIatkBOXvX3mhw-QZZAvb3SUmjEC2SEeB3yBkVA
📋 Name: map[type:String value:Alice Smith] 📋 Email: map[type:String
value:alice@example.com] 📋 Status: map[type:String value:pending] 📋 Credits:
map[type:Integer value:0]

============================================================ 📝 Script 2:
Query + Update + Verify
============================================================

1️⃣ Saving Script... ✅ Script saved:
VFGqO7RyZbmfS0Jx5_twdoLo10REtlFoMbn6r6Ek_LJH9KhZpGllZYkwAnnXo4KRSQabwxBAflAar792Oe9XfQ

2️⃣ Calling Script (Query + Update + Verify)... ✅ Script executed: 3 Functions
⏱️ Execution time: 0ms

3️⃣ Verification Results: ✅ Found 1 record(s) 📋 Status updated to:
map[type:String value:active] 📋 Name: map[type:String value:Alice Smith]

============================================================ 📝 Script 3:
Query + Update Credits + Verify
============================================================

1️⃣ Saving Script... ✅ Script saved:
KP7cHKVbXQWNXDRc5OX4oRANn5yG221bxUZu0ME6lS3ZZXduElJ2pu93pH_oWhf8FCcv0exbahpdBzZO5oj7mw

2️⃣ Calling Script (Query + Update Credits + Verify)... ✅ Script executed: 3
Functions ⏱️ Execution time: 0ms

3️⃣ Verification Results: ✅ Found 1 record(s) 📋 Credits updated to:
map[type:Integer value:0] 📋 Status: map[type:String value:active] 📋 Name:
map[type:String value:Alice Smith]

============================================================ 📝 Script 4: Query
Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script... ✅ Script saved:
sr3E6AaI_6G_JAdjgDcIIaTfRMeU9fGlPrmvq_5yJjvSggaZwq2rhUjHsnbF4-BvOLQiqKTsl6wZeTQsCjC3Cw

2️⃣ Calling Script (Query + Delete + Verify)... ✅ Script executed: 3 Functions
⏱️ Execution time: 0ms

3️⃣ Verification Results: 📊 Before delete: Record existed (verified by first
Query) ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================ 🧹 Cleanup
============================================================ ✅ Deleted script:
UVuX_U382_XG6KrPrVZl... ✅ Deleted script: VFGqO7RyZbmfS0Jx5_tw... ✅ Deleted
script: KP7cHKVbXQWNXDRc5OX4... ✅ Deleted script: sr3E6AaI_6G_JAdjgDcI... ✅
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
map[id:ZwRgCZbE_QvJg8XwZdOVTmAgKCUcuWvSK5OiSx-JW0f15q2rsW6hyPmPa43d3PSrGldGEVgNtOzdonoTfdeqyg]

=== Find by ID === Found: map[active:map[type:Boolean value:true]
id:ZwRgCZbE_QvJg8XwZdOVTmAgKCUcuWvSK5OiSx-JW0f15q2rsW6hyPmPa43d3PSrGldGEVgNtOzdonoTfdeqyg
name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query === Found documents: 1

=== Update Document === Updated: map[active:map[type:Boolean value:true]
id:ZwRgCZbE_QvJg8XwZdOVTmAgKCUcuWvSK5OiSx-JW0f15q2rsW6hyPmPa43d3PSrGldGEVgNtOzdonoTfdeqyg
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
xyiW2AXRcRjioxHqOiWMzxruOxy_dYSW98GVzY-axs7xN_reAlq8mb4p5KbKCXOT_QhaJwDLtMB1kA_JNnhjHA

=== List Collections === Total collections: 14 Sample collections:
[schema_documents_client_py websocket_test schema_employees_client_py
test_accounts schema_users_client_py]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
sd5SHGyW6fMQWQo62eQ8mVyIRUPM4b0hUua4SMwuk0iRg26OsisBXaBcAcEzGR2wr8oShsOBw0dQuUypZjHSlA

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
byZRNr87Jy7EMSLyaveD8eCZs2qwlzVouDQWVBO_jW_LgUf7j5NJeLIBu7-leR7YlTZiv5YFlowOyiUYBhnj9w

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
JNRBjOzFkwn3S485zRxwdNW0pW8j-4Bq3-3UwA0j9Y6RXnOjVfGbpiiRsM4hXqTKe01kxsyZpvhfeeS7Sq5fjQ

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
0EtnXjbB_gWAumjDJtigIpAyjG6WpvxdrayIceTKBMSBAUz0AIeQJ9VCBwvo6Wm4imcvVxQ_ssv2nvavkDbMJg

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
U6ghpVmbIClTPbe95ajQzICrBvWDayyiU9jU-ksm_xwEYCjPl5G4va407cIuGPo_Qgko-e_P09vaBE52QSd46Q

=== Sending Chat Message === Message ID:
fUp9r3vfJiPnJa_ETNIA6a7UdxwNyzsqPn98B8LpRyYecmd0PIaRFtVYE8-hUqAyLuLAte_2WrZz9A1940TCUA

=== AI Response === I'm sorry, but the provided context doesn't include specific
information about product names and their prices. Could you please provide more
information or clarify your request?

=== Context Used (3 snippets) === Snippet 1: map[collection:client_chat_basic_go
matched_fields:[description] record:map[description:map[type:String value:A
high-performance database product with AI capabilities]
id:KM7XcloKZuj58S0AWD3WVrPxqd9UjrEQZp7EfYblSERtx5SdgQ-9btYNs5b2qF0lb8JMRvfHKEPXZfhsQf2XAQ
name:map[type:String value:ekoDB] price:map[type:Integer value:99]]
score:0.1111111111111111] Snippet 2: map[collection:client_chat_basic_go
matched_fields:[description] record:map[description:map[type:String value:Fully
managed cloud database service product]
id:8XcJoMrDEZ8LStuYHIj4jtfAYQu1ezU7pM0OtUPyKCN5K5ehOR_bCLKsYoQdjc769bS5bbQrITEQUvdilakwYw
name:map[type:String value:ekoDB Cloud] price:map[type:Integer value:499]]
score:0.1111111111111111] Snippet 3: map[collection:client_chat_basic_go
matched_fields:[description] record:map[description:map[type:String
value:Enterprise edition product with advanced features]
id:vIYFZgKtwJN4uFOxRQjZ2O1Fty5e76TSUHGm8HgMxdCgJCuXtO5XFWJBnq5jC7wp8sHzM2eVUHx55alacw7CVg
name:map[type:String value:ekoDB Pro] price:map[type:Integer value:299]]
score:0.1111111111111111]

Execution Time: 1750ms

=== Token Usage === Prompt tokens: 324 Completion tokens: 31 Total tokens: 355

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
l2TKmXvg8rdbBtJammgCyq4Jy3yVgirnt4-pk6LvMOAAockMGXCmZRC6WNzREj_pUUnlVPXFu7miAzmx039jNw

=== Sending Messages === ✓ Message 1 sent Response: I'm sorry, but the context
provided doesn't include specific information about available products. Could
you please provide more details or clarify your question?

✓ Message 2 sent Response: I'm sorry, but the context provided does not include
information about the price of any specific product. Could you please provide
more details or specify the product you're interested in?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
ZezuvgwoCLk15vaTxQ7xiSavnWEr8f04kng1Jt1rwJpzjUKiRQ-aIJ1BjoGgQPr7iaHpUtcspq7SrJWPZZPFyA
Parent:
l2TKmXvg8rdbBtJammgCyq4Jy3yVgirnt4-pk6LvMOAAockMGXCmZRC6WNzREj_pUUnlVPXFu7miAzmx039jNw

=== Listing Sessions === ✓ Found 9 sessions Session 1:
ZezuvgwoCLk15vaTxQ7xiSavnWEr8f04kng1Jt1rwJpzjUKiRQ-aIJ1BjoGgQPr7iaHpUtcspq7SrJWPZZPFyA
(Untitled) Session 2:
l2TKmXvg8rdbBtJammgCyq4Jy3yVgirnt4-pk6LvMOAAockMGXCmZRC6WNzREj_pUUnlVPXFu7miAzmx039jNw
(Untitled) Session 3:
U6ghpVmbIClTPbe95ajQzICrBvWDayyiU9jU-ksm_xwEYCjPl5G4va407cIuGPo_Qgko-e_P09vaBE52QSd46Q
(Untitled) Session 4:
j0ZNc8BrbxhRUDgb2Fwn4wHDCMGkwyehQjCVIE_5utey3QYCVumADwRrDHTrdEJPdutXXUHL_TIEvjy9tas1ig
(Untitled) Session 5:
Kw60gQSthrKhDTfMuRtxqAQIyQTjGlsO4MUe8y65_FJHpsNu1vRm7nOJP10OKVfro3MzZkmxpCCQUhBStMMfhg
(Untitled) Session 6:
LCHZxuXi5of0QvhmSm0wm2f0WqfUiFUL6umnxwqTLFma71kgbt2sXS9UnwTggcBS-gSdQZqtf466qGxqUAT_hg
(Untitled) Session 7:
pC_Hvla-W6X4dKvY4WkJu4fHKzUoGMR1u2f-5FM1qJvi3iON0PEl6jCJjAEIWgpAThKQIjskKUCCOt0Ad51qMQ
(Untitled) Session 8:
LxDb9FFuDweBcT_fv86mFw6VBNhoehESDjYzen5_WpbOU7Dk8scFwalnDWwRWmY4Jd35oW7KoM_EF8KG739cig
(Untitled) Session 9:
W9Ku4NfJAWIGjBvCJsMP2kNZAFEEvTkH8RenBjwisN141_0i-7ueIMCLe8CpfAgLZfO24z6cAVpzbYybitCyHA
(Untitled)

=== Getting Session Details === ✓ Session details retrieved Messages: 4

=== Deleting Branch Session === ✓ Deleted branch session:
ZezuvgwoCLk15vaTxQ7xiSavnWEr8f04kng1Jt1rwJpzjUKiRQ-aIJ1BjoGgQPr7iaHpUtcspq7SrJWPZZPFyA

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
QLnnGjpvAK589MM2W8RNav-8B4Mlr3nHidNWK55KPXEfV60IBlLoYUzLcPYy8nFRXxjrefuhbYPpzNW0QXKigw

=== Sending Initial Message === ✓ Message sent Response: I'm sorry, but the
provided context does not include any specific information about available
products. Could you please provide more details or specify the type of product
you are interested in?

✓ Second message sent

=== Feature 1: Regenerate AI Response === ✓ Message regenerated New response:
Answer: I'm sorry, but the context provided does not include information on the
price of ekoDB. You may need to check the official website or contact the
sellers for this information.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
zkkyfdmFnYypkAqEgfCEOohsXEn_9nMMpSt8BSnC6ndXSyLLB5M6XsYAkvgkWHqVWOs9he7TmMsMqBQAyNFq_w
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
1. Score: 0.761
1. Score: 0.756
1. Score: 0.730

1. Hybrid search (text + vector): Found 3 results using hybrid search (text +
   vector)
1. Score: 2.692
1. Score: 1.504
1. Score: 0.303

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
Ugx3JnvHkAVlP_fYnfQpZr8OT042FH0bOGSmCxmXz6ltbxXMtkoESKJovciFYkxzzheYSQ7ASowj-7faH4lS1Q
📊 Found 10 records ⏱️ Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved 📊 Found 10 users (limited) ⏱️ Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved 📊 Statistics: 2 groups map[avg_score:50 count:5
status:inactive] map[avg_score:60 count:5 status:active] ⏱️ Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 14 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
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

> @ekodb/ekodb-client@0.4.0 prepare npm run build

> @ekodb/ekodb-client@0.4.0 build tsc

up to date, audited 7 packages in 1s

found 0 vulnerabilities

> @ekodb/ekodb-client@0.4.0 build tsc

✅ [32mTypeScript client built![0m 🧪 [36mRunning TypeScript client library
examples...[0m

added 1 package, removed 1 package, and audited 13 packages in 812ms

3 packages are looking for funding run `npm fund` for details

found 0 vulnerabilities ✓ Client created (token exchange happens automatically)

=== Insert Document === Inserted: { id:
'ry4yL6LgBfo1LDEodz_QvouL6M14P3aBOfs2glCt34rWznw4j0feq9xRZvu60sa02uNq8p861S9tRzn-HL10zA'
}

=== Find by ID === Found: { id:
'ry4yL6LgBfo1LDEodz_QvouL6M14P3aBOfs2glCt34rWznw4j0feq9xRZvu60sa02uNq8p861S9tRzn-HL10zA',
value: { type: 'Integer', value: 42 }, active: { type: 'Boolean', value: true },
name: { type: 'String', value: 'Test Record' } }

=== Find with Query === Found documents: 1

=== Update Document === Updated: { id:
'ry4yL6LgBfo1LDEodz_QvouL6M14P3aBOfs2glCt34rWznw4j0feq9xRZvu60sa02uNq8p861S9tRzn-HL10zA',
active: { type: 'Boolean', value: true }, name: { type: 'String', value:
'Updated Record' }, value: { type: 'Integer', value: 100 } }

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

=== Get Multiple Keys === cache:product:1: { name: 'Product 1', price: 29.99 }
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
bEzryU5Dj9rXuSINkMWdA4N5AKW0cP9KDNKgqDtxGO1Jql4W903w5vcc8r\_-c0MuTOJETZ7fcvOADYoxO8U2xw

=== List Collections === Total collections: 18 Sample collections:
schema_documents_client_py,websocket_test,schema_employees_client_py,client_collection_management_ts,schema_documents_client_go

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
oTp5Xnnz4VmDMheWCzStQYIj_9hHuIqIIFrDIxmbX7OA-YW3duqO9e-SmeMvEOtx1ew9xzl57i5fTUFiyGWAVQ

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
cG3FFbvAAxYVTGHyr15Fh31Pn07n-vHSkacgfeMV8uEkeolNiZ0QJ00HRE_kOYxX5IpxT4lMXwvoXAm-U5Bhdw

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
\_BQr0wipSXpCriBsRESwcnvZ1tuLErhD0h6yq6kkgabXTSjJXJyWrvA12-CnNghdbwUgGwswt7O20BFeTN_uhA

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
XSLhSkPG02NAkKTR1vp8zfQTl0ysxNwTpJM8V67NAAZhQoTJw34A9JLSWsT5iC8QnkQneNjS1XAk8nUImjhOlw

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
UsEFZtdDz-pNk9IjXHv5abWj5i_NokGKR6LGHOkqavIaMjdYm_9NkF9jarqHmI4ZF_0Hd01q7vwo1yhCVG71rA

=== Sending Chat Message === Message ID:
WnUOVkbAJfwStxpBCZsLrE_U5AKXSr_51pxrfOg0XKhf_Gk5YgbHB96CUdn7lZFt64bxNz1gq1uSyXMpYGgUTw

=== AI Response === I'm sorry, but the context provided does not include
specific product names or their prices. Could you please provide more
information?

=== Context Used (3 snippets) === Snippet 1: { collection:
'client*chat_basic_ts', record: { price: { value: 299, type: 'Integer' }, id:
'SjlFTw199zPrxlEHLH2obrQJIINQM1rlVqsaGWRejTrEH*-EmKPviMtyXRDdZHMBlVP1G3tarv4DYf-sVpYjEA',
description: { value: 'Enterprise edition product with advanced features', type:
'String' }, name: { type: 'String', value: 'ekoDB Pro' } }, score:
0.1111111111111111, matched_fields: [ 'description' ] } Snippet 2: { collection:
'client_chat_basic_ts', record: { id:
'6ioduVPc1ShCrLjoEiJwUeqV-NZPJDuy7xbFyaLi3XH8nDuE4VG0oIHolXSILICcD8JzxF7iDdSkVUfaOBiXqw',
price: { type: 'Integer', value: 499 }, description: { value: 'Fully managed
cloud database service product', type: 'String' }, name: { value: 'ekoDB Cloud',
type: 'String' } }, score: 0.1111111111111111, matched_fields: [ 'description' ]
} Snippet 3: { collection: 'client_chat_basic_ts', record: { name: { value:
'ekoDB', type: 'String' }, price: { type: 'Integer', value: 99 }, id:
'CEGpniyINJEpHc2aFeeRU8UaxO9YvhPB464O8WbRPckDoqgd8auwMQEYNnOc1Qk-jekTZBm9Qo6lIcpUkdeu4w',
description: { type: 'String', value: 'A high-performance database product with
AI capabilities' } }, score: 0.1111111111111111, matched_fields: [ 'description'
] }

Execution Time: 1465ms

=== Token Usage === Prompt tokens: 313 Completion tokens: 25 Total tokens: 338

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
qdxCGX6CQqVL0KOp5IvBZ1DOfwFllzXUE3ruqjh3qa76afit-K-znu7nje5LoYiBvqmoBFShqNrRWegsuKiDWQ

=== Sending Messages === ✓ Message 1 sent Response: I'm sorry, but the context
provided does not contain any specific product information. Could you please
provide more details or specify the type of products you are interested in?

✓ Message 2 sent Response: Apologies, but the context provided does not include
information about a specific product or its price. Could you please provide more
details or clarify which product you're interested in?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
OMq9gymjNDHDtGiBjvQ8Wzp-WM9_CujYJ0nG1iEQtB7h8raxRtZOIF8BhxnosocbsTohPgkAV4qSxVyJcbyQbQ
Parent:
qdxCGX6CQqVL0KOp5IvBZ1DOfwFllzXUE3ruqjh3qa76afit-K-znu7nje5LoYiBvqmoBFShqNrRWegsuKiDWQ

=== Listing Sessions === ✓ Found 10 sessions Session 1:
OMq9gymjNDHDtGiBjvQ8Wzp-WM9_CujYJ0nG1iEQtB7h8raxRtZOIF8BhxnosocbsTohPgkAV4qSxVyJcbyQbQ
(Untitled) Session 2:
qdxCGX6CQqVL0KOp5IvBZ1DOfwFllzXUE3ruqjh3qa76afit-K-znu7nje5LoYiBvqmoBFShqNrRWegsuKiDWQ
(Untitled) Session 3:
UsEFZtdDz-pNk9IjXHv5abWj5i_NokGKR6LGHOkqavIaMjdYm_9NkF9jarqHmI4ZF_0Hd01q7vwo1yhCVG71rA
(Untitled) Session 4:
zkkyfdmFnYypkAqEgfCEOohsXEn_9nMMpSt8BSnC6ndXSyLLB5M6XsYAkvgkWHqVWOs9he7TmMsMqBQAyNFq_w
(Untitled) Session 5:
l2TKmXvg8rdbBtJammgCyq4Jy3yVgirnt4-pk6LvMOAAockMGXCmZRC6WNzREj_pUUnlVPXFu7miAzmx039jNw
(Untitled) Session 6:
U6ghpVmbIClTPbe95ajQzICrBvWDayyiU9jU-ksm_xwEYCjPl5G4va407cIuGPo_Qgko-e_P09vaBE52QSd46Q
(Untitled) Session 7:
j0ZNc8BrbxhRUDgb2Fwn4wHDCMGkwyehQjCVIE_5utey3QYCVumADwRrDHTrdEJPdutXXUHL_TIEvjy9tas1ig
(Untitled) Session 8:
Kw60gQSthrKhDTfMuRtxqAQIyQTjGlsO4MUe8y65_FJHpsNu1vRm7nOJP10OKVfro3MzZkmxpCCQUhBStMMfhg
(Untitled) Session 9:
LCHZxuXi5of0QvhmSm0wm2f0WqfUiFUL6umnxwqTLFma71kgbt2sXS9UnwTggcBS-gSdQZqtf466qGxqUAT_hg
(Untitled) Session 10:
pC_Hvla-W6X4dKvY4WkJu4fHKzUoGMR1u2f-5FM1qJvi3iON0PEl6jCJjAEIWgpAThKQIjskKUCCOt0Ad51qMQ
(Untitled)

=== Getting Session Details === ✓ Session details retrieved Messages: 4

=== Deleting Branch Session === ✓ Deleted branch session:
OMq9gymjNDHDtGiBjvQ8Wzp-WM9_CujYJ0nG1iEQtB7h8raxRtZOIF8BhxnosocbsTohPgkAV4qSxVyJcbyQbQ

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
GtwYLWVvRmo3_PULzuGdjXxy1YBB9JqJIP8P9HML35Ir-x1UC1UAFiLQ_u0541_Ijm8QY9PCqVER33Of77NBKQ

=== Sending Initial Message === ✓ Message sent Response: I'm sorry, but the
context provided doesn't give information about any specific product. Could you
please provide more details or specify the type of product you are interested
in?

✓ Second message sent

=== Feature 1: Regenerate AI Response === ✓ Message regenerated New response:
Answer: I'm sorry, but the context provided does not include information on the
price of ekoDB.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
rS0mxn8V7loz_E6U8ApkdhM_fvNusgU3fGTn0ndjtJ9loLKlpvV6fYhgoVTYkizoh7P3eX-NyWIDxDu0q5YyTg
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
1. Score: 26.400, Matched: title, bio, bio.value, title.value
1. Score: 26.400, Matched: title.value, title, bio, bio.value
1. Score: 26.400, Matched: title.value, bio.value, bio, title
1. Score: 26.400, Matched: title.value, bio, bio.value, title

1. Search with field weights: Found 4 results with weighted fields
1. Score: 39.600, Matched: bio.value, title.value, title, bio
1. Score: 39.600, Matched: title.value, bio, title, bio.value
1. Score: 39.600, Matched: bio.value, title, title.value, bio
1. Score: 39.600, Matched: bio, title.value, bio.value, title

1. Search with minimum score threshold: Found 2 results with score >= 0.3
1. Score: 13.200, Matched: bio, bio.value
1. Score: 13.200, Matched: bio, bio.value

1. Search with stemming and exact match boosting: Found 1 results (matches:
   work, working, worked)
1. Score: 13.200, Matched: bio, bio.value

1. Vector search (semantic search): Found 3 semantically similar documents
1. Score: 0.748, Matched:
1. Score: 0.742, Matched:
1. Score: 0.740, Matched:

1. Hybrid search (text + vector): Found 3 results using hybrid search (text +
   vector)
1. Score: 2.697, Matched: content.value, title, content, title.value
1. Score: 1.496, Matched: content, title, content.value, title.value
1. Score: 0.299, Matched:

1. Case-sensitive search: Found 1 results (case-sensitive)
1. Score: 26.400, Matched: bio, skills, skills.value, bio.value

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
atDYknsuaAZaWyOq0KhJL0r-jn2IORLVG6txGBdCVR0j9bQc0DqXQGy6ou2VbOLOtUqnr4Oxsj3_f7yyaClknQ
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved:
t9H06CcU0DxlpQ7rdeszfz9C8HXrZ5wHRwQxtWH9b6SOb6tDhX2pQeMSqsEVkxWqkgruX3FgFuPdJRlFgvlhHw
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved:
L0zBh-no8aViN1lsYAg_t7iMoZbx8q1HoOMrven6XZ-aTOPEQ6kyj6WPs_ENi0nnewWftpKL1lZCpyqeivPjyw
📊 Statistics: 2 groups {"count":5,"status":"active","avg_score":60}
{"status":"inactive","count":5,"avg_score":50}

📝 Example 4: Script Management

📋 Total scripts: 18 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed! 🚀 ekoDB TypeScript Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data... ✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved:
4tf-2IQ9eLvHj7GxOO8gSPW9oep8YBh0pn1na9wnjvOJCf_OqwI05taJ5MtgPeq5eYbd27R_pGJ-j_VJoKy76A
📊 Found 2 product groups
{"count":3,"category":"Electronics","avg_price":575.6666666666666}
{"avg_price":474,"category":"Furniture","count":2} ⏱️ Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved 📊 Found 5 products ⏱️ Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved 📊 Found 2 categories {"category":"Electronics","count":3}
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

added 1 package, removed 1 package, and audited 9 packages in 790ms

1 package is looking for funding run `npm fund` for details

found 0 vulnerabilities [34m ╔════════════════════════════════════════╗[0m [34m║
ekoDB JavaScript Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m [34m === Checking Server
Connection ===[0m (node:63994) [DEP0040] DeprecationWarning: The `punycode`
module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) [32m✓
Server is ready[0m [34m === Getting Authentication Token ===[0m [32m✓
Authentication successful[0m [33m === Running 10 Examples ===[0m [34m ===
Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_crud.js
===[0m ✓ Authentication successful

=== Insert Document === (node:63997) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) Inserted:
{ id:
'FQopvvtve-kJBuBgvgoB-yYDjLyrJOqwQ1pGz7xvl_acFnquFebroKQSZXcRu2XQYrYf5dCKLW-Vafz2nOEk1Q'
}

=== Find by ID === Found: { value: { type: 'Integer', value: 42 }, name: {
value: 'Test Record', type: 'String' }, id:
'FQopvvtve-kJBuBgvgoB-yYDjLyrJOqwQ1pGz7xvl_acFnquFebroKQSZXcRu2XQYrYf5dCKLW-Vafz2nOEk1Q',
active: { type: 'Boolean', value: true } }

=== Find with Query === Found documents: 1

=== Update Document === Updated: { id:
'FQopvvtve-kJBuBgvgoB-yYDjLyrJOqwQ1pGz7xvl_acFnquFebroKQSZXcRu2XQYrYf5dCKLW-Vafz2nOEk1Q',
value: { type: 'Integer', value: 100 }, active: { type: 'Boolean', value: true
}, name: { type: 'String', value: 'Updated Record' } }

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully [32m✓ simple_crud.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_websocket.js
===[0m ✓ Authentication successful

=== Inserting Test Data === (node:64000) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Inserted
test record:
JcFanAYz014N6pzJG3QUhCU3QaejJx9ZcgC7FIPzG67u0P5jgsCWecU6GF3PYE39p-6KAyb-zX24QL7w87F9rQ

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"ZcYBdW4BqNkDzf5RbWUTc1jIg4MNTZSmLAIkKJV-qyVNSV9E7XHRzPgh3hNqd7aO-Wc1hgdTsZeNeSg32CNDwg",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"qQFo7AuYWumBE-nyEtkv0vnDprHbBQs6MyvSoxLr6x1dPGKK912zpqx7rJPxDOhxF1hw2rZdIWEl5BK9pSW0TA",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"UAcR9Yc_QZABIRubc0RoEp8eawAI-On9ZRl3kxmnfsYBDHLvLFzM9aWMaEkGLksmTlL4s2AxDw-B-UvQT1QkXQ",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"JcFanAYz014N6pzJG3QUhCU3QaejJx9ZcgC7FIPzG67u0P5jgsCWecU6GF3PYE39p-6KAyb-zX24QL7w87F9rQ",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } } ] }, "type": "Success" } ✓ Retrieved 4
record(s) via WebSocket

✓ WebSocket example completed successfully [32m✓ simple_websocket.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/batch_operations.js
===[0m ✓ Authentication successful

=== Batch Insert === (node:64013) [DEP0040] DeprecationWarning: The `punycode`
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

=== KV Set === (node:64017) [DEP0040] DeprecationWarning: The `punycode` module
is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Set key:
session:user123

=== KV Get === Retrieved value: { loginTime: '2025-12-21T00:49:32.912Z', userId:
123, username: 'john_doe' }

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: { price: 29.99, name: 'Product 1' }
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully [32m✓ kv_operations.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/collection_management.js
===[0m ✓ Authentication successful

=== Create Collection (via insert) === (node:64020) [DEP0040]
DeprecationWarning: The `punycode` module is deprecated. Please use a userland
alternative instead. (Use `node --trace-deprecation ...` to show where the
warning was created) Collection created with first record:
VVooCLD_QXAC1fkb-SUpcxixiTQyC-TxpGU62Yo6Fo1j0zrt6IkrGRNdJID7xWTTBt9DSm075iusSmmpvYj4wg

=== List Collections === Total collections: 23 Sample collections: [
'schema_documents_client_py', 'websocket_test', 'schema_employees_client_py',
'demo_collection', 'schema_documents_client_go' ]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully [32m✓
collection_management.js completed successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/document_ttl.js
===[0m ✓ Authentication successful

=== Insert Document with TTL (1 hour) === (node:64033) [DEP0040]
DeprecationWarning: The `punycode` module is deprecated. Please use a userland
alternative instead. (Use `node --trace-deprecation ...` to show where the
warning was created) ✓ Inserted document:
0tfzsFT_XSLvsXPfj3JuN6A8CCP-lxnk9b7Uoenkm8sFHYztZAppPZ3g7K_zaKvK8xtsJns5WLpzu4rjvG9SzA

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 4 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully [32m✓ document_ttl.js
completed successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/websocket_ttl.js
===[0m ✓ Authentication successful

=== Insert Test Data with TTL === (node:64036) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Inserted
document with TTL:
BWHtkjYcF7JQltQs7ig8y-Cu6Ta8zSsJNG7qltmZE1Nw7suC8Jz7eOq2nsRh8yHRi5dNbEsguFM_ICT3BTQTxg

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 4 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully [32m✓ websocket_ttl.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/http_functions.js
===[0m 🚀 ekoDB Scripts Example (JavaScript/HTTP)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved:
TwbA-YCUH9gM-BIcW9jqQqcOO6q5VVq5np58FYGlQiuhqz0cwhJaLNTR0lOGLEsR9okn7fLYQ5iPd_0MXvuxmA
📊 Found 20 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved:
h4tFVxWW6F2WbJY6wBmZi3H2wH_oFtdkebcSV9NZck_Zb9ZsL273ewQ8IopmI4s5VZWTkSCKmwa2YM9XuoIJpw
📊 Page 1: Found 3 users (limit=3, skip=0) 📊 Page 2: Found 3 users (limit=3,
skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved:
DeoAs9IpaLhB5KLqPW6RPicijapUBRkHbkgPHxXLSoUSfVWM2Kom6u_PbskILIJG-4652YnYq_tm-tTcJ4aUZQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
{"avg_score":60,"max_score":100,"count":10,"status":"active"}
{"status":"inactive","avg_score":50,"max_score":90,"count":10}

📝 Example 4: Function Management

📋 Total scripts: 20 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

✅ All examples completed! [32m✓ http_functions.js completed successfully[0m
[34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/transactions.js
===[0m ✓ Authentication successful

=== Setup: Creating Test Accounts === Created Alice: $1000 - ID:
2dK6LvJlI2264a-ai1HCkDEGQuSudptuuiZ9jj4kIVT23-gqBOvqJW4EFSsTv46AYx9eD0_9YImaZf8yCElQuQ
Created Bob: $500 - ID:
QLkBPRRSWOqXYDCkiUM6Ql6V1FqSBBU43rQNFBbGPROrX_maunfIW3pWs13FMnwNsihgJButTXFZjTy8sTT2-w

=== Example 1: Begin Transaction === Transaction ID:
78bc9a67-af0c-4c56-9a89-493a59048b57

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: Active Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: 800 Bob: 700

=== Example 5: Rollback === New transaction:
4faea1b4-5b3d-4b96-87ec-c84966fa178c Updated Bob: $700 → $600 (in transaction) ✓
Transaction rolled back Bob after rollback: 700

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
hxupRm0IXCvZcl2RrZBz7KlnkgRfCdXJeaeMskrwf96w0qRylncWWEuf-RzwysmtUXIHca2MfzPs6G-FwOxu5Q

2️⃣ Calling Script (Insert + Verify)... ✅ Script executed: 2 Functions ⏱️
Execution time: 0ms

3️⃣ Verification Results: ✅ Found 1 record(s) 📋 Name: [object Object] 📋 Email:
{"type":"String","value":"alice@example.com"} 📋 Status: [object Object] 📋
Credits: [object Object]

============================================================ 📝 Script 2:
Query + Update + Verify
============================================================

1️⃣ Saving Script... ✅ Script saved:
\_YoPgd4rux1ReycSVLXwObCMhZPEfIhAK6GcjoW-i9ZB_yIfzSEPKrMPKBKvrtl4vaKQdFczSUCN6NrlKvXrgQ

2️⃣ Calling Script (Query + Update + Verify)... ✅ Script executed: 3 Functions
⏱️ Execution time: 0ms

3️⃣ Verification Results: ✅ Found 1 record(s) 📋 Status updated to:
{"type":"String","value":"active"} 📋 Name: {"type":"String","value":"Alice
Smith"}

============================================================ 📝 Script 3:
Query + Update Credits + Verify
============================================================

1️⃣ Saving Script... ✅ Script saved:
ILCY5aHf--hggnvnseSBsYxq0TI-bblZlda8fbrlq-9P1H96uaK7pB-1BlSJspv2E6nNHguW_oon8nKTEUyWyw

2️⃣ Calling Script (Query + Update Credits + Verify)... ✅ Script executed: 3
Functions ⏱️ Execution time: 0ms

3️⃣ Verification Results: ✅ Found 1 record(s) 📋 Credits updated to:
{"value":0,"type":"Integer"} 📋 Status: {"value":"active","type":"String"} 📋
Name: {"type":"String","value":"Alice Smith"}

============================================================ 📝 Script 4: Query
Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script... ✅ Script saved:
VGR7tb6S51WiMhb64sFD92bDLZmgz3GKF1IpFQkaV5FI49i-r23Ss7-6BRdNC-ywo0MAApQbZuvzP0PFvuUvvQ

2️⃣ Calling Script (Query + Delete + Verify)... ✅ Script executed: 3 Functions
⏱️ Execution time: 0ms

3️⃣ Verification Results: 📊 Before delete: Record existed (verified by first
Query) ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================ 🧹 Cleanup
============================================================ ✅ Deleted script:
hxupRm0IXCvZcl2RrZBz... ✅ Deleted script: \_YoPgd4rux1ReycSVLXw... ✅ Deleted
script: ILCY5aHf--hggnvnseSB... ✅ Deleted script: VGR7tb6S51WiMhb64sFD... ✅
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

> @ekodb/ekodb-client@0.4.0 prepare npm run build

> @ekodb/ekodb-client@0.4.0 build tsc

up to date, audited 7 packages in 1s

found 0 vulnerabilities

> @ekodb/ekodb-client@0.4.0 build tsc

✅ [32mTypeScript client built![0m 🧪 [36mRunning JavaScript client library
examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 783ms

1 package is looking for funding run `npm fund` for details

found 0 vulnerabilities ✓ Client created (token exchange happens automatically)

=== Insert Document === Inserted: { id:
'wMY6lw0l2-EWlDcSgmG9SvQVqEgunCEwcon5lwLv0Hx3-1YVWIKcM3_2Y-eQisgCykg_atM3vptpHHFbqvV_UQ'
}

=== Find by ID === Found: { name: { value: 'Test Record', type: 'String' }, id:
'wMY6lw0l2-EWlDcSgmG9SvQVqEgunCEwcon5lwLv0Hx3-1YVWIKcM3_2Y-eQisgCykg_atM3vptpHHFbqvV_UQ',
value: { value: 42, type: 'Integer' }, active: { value: true, type: 'Boolean' }
}

=== Find with Query === Found documents: 1

=== Update Document === Updated: { id:
'wMY6lw0l2-EWlDcSgmG9SvQVqEgunCEwcon5lwLv0Hx3-1YVWIKcM3_2Y-eQisgCykg_atM3vptpHHFbqvV_UQ',
value: { value: 100, type: 'Integer' }, active: { value: true, type: 'Boolean'
}, name: { type: 'String', value: 'Updated Record' } }

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
OCvTBXqIjbEbRK1A518wiAm70cS9B85VYdWYY3C7VRiF4kFogk-qiMc7G9WmAVF_Xmno0Euw8xJ4VIyRRJaYlA

=== List Collections === Total collections: 22 Sample collections:
schema_documents_client_py,websocket_test,schema_employees_client_py,schema_documents_client_go,schema_products_client_go

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
ErCcw5gT0GL24Tc5rsfFQNBT3y8KFM7SFq5d2OPfuCnTtdbQK6W38vdKZJx_MKpHssczjum55ur4LrntYdlJ-A

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
SpfB16ISZuXEsl-eIQ6iZ8fIhxXyERGY-oTBEDCNAde5q_jNs1IZYLE-d1sm6jQO_m0pfGSIy34NmM-zmwvOkQ

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
pjxjDCEgEPJzabFvRinkTT3XEsuiKuSG12QPrRuzejYRr8HmHH6Sc5DmLTZw11YxxsJnbaBdEwDQFTf5AGDZ2g

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
h9-8xkV5uIIkr_YZHFeVk_rsUxVkqJ_xjdx6OJDvT2iOFCd9k5wU_6hovnsb6UmOSFOiAZWBOEhmVX-CcO_MtQ

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration 🚀 ekoDB Scripts Example (JavaScript/HTTP)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved:
OMVywZrcSgn6P3uk8V_v7Q0S2I-oMZBHnZAtIybu4Ub5GpEYh_KozJxCZx6kDPreenqjozsD-9V1WaK9pu9ztg
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved:
anjLadVtxO7rOvcFGFBsdhs1VWzqUCwGZiIOqvbOPHYzkH0blqtc1bjjxvNC0yVzfc6z0b5CJE2_6kiMf24T2Q
📊 Page 1: Found 3 users (limit=3, skip=0) 📊 Page 2: Found 2 users (limit=3,
skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved:
Xx-5HnsisvzTtRk6kMRpdIRUVeeX8cocmYTfdB3pACdn3mb0-kQOAf0HQql4gNLj_Dy7YPF-zpTT3CVCyBxYPg
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
{"max_score":100,"avg_score":60,"count":5,"status":"active"}
{"status":"inactive","count":5,"max_score":90,"avg_score":50}

📝 Example 4: Function Management

📋 Total scripts: 22 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

✅ All examples completed! 🚀 ekoDB Scripts Example (JavaScript Client)

✅ Client initialized (token exchange automatic)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved:
EBZVIH9OCJ3pLhPero5WIzcva0S1ziouWPY-Ho1hRIhN0J0hSOoGtvH7-4JOOsmjfkhywOq6vXfWsks0b2MBBg
📊 Found 20 records ⏱️ Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved 📊 Found 20 users (limited) ⏱️ Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved 📊 Statistics: 2 groups
{"count":10,"status":"inactive","avg_score":50}
{"count":10,"avg_score":60,"status":"active"} ⏱️ Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 24 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
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

1.  Desk Chair - $349 (⭐4.2)
2.  Standing Desk - $599 (⭐4.7)
3.  Keyboard - $89 (⭐4.4) ⏱️ Execution time: 0ms

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

1.  User 6 (user6@example.com) - Score: 60
2.  User 4 (user4@example.com) - Score: 40
3.  User 8 (user8@example.com) - Score: 80
4.  User 2 (user2@example.com) - Score: 20
5.  User 1 (user1@example.com) - Score: 10 ⏱️ Execution time: 0ms

📝 Example 5: User Summary Statistics

✅ Script saved 📊 User summary (2 groups): inactive undefineds: 3 users, Total
Score: 180 active undefineds: 7 users, Total Score: 370 ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Cleanup complete

✅ All CRUD script examples completed! 🚀 ekoDB Search Scripts Example

📋 Setting up test data... ✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Script saved 📊 Found 5 documents

1.  Getting Started with ekoDB (Database)
2.  Introduction to Machine Learning (AI)
3.  Natural Language Processing (AI)
4.  Database Design Principles (Database)
5.  Vector Databases Explained (Database) ⏱️ Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved 📊 Documents by category: Database: 3 documents AI: 2 documents
⏱️ Execution time: 0ms

📝 Example 3: Select Specific Fields

✅ Script saved 📊 Document titles (5 docs):

1.  Getting Started with ekoDB
2.  Introduction to Machine Learning
3.  Natural Language Processing
4.  Database Design Principles
5.  Vector Databases Explained ⏱️ Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved 📊 Projected documents (showing first 3):

1.  Getting Started with ekoDB
2.  Introduction to Machine Learning
3.  Natural Language Processing ⏱️ Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved 📊 All documents (5 total, showing first 2):

1.  Getting Started with ekoDB (Database)
2.  Introduction to Machine Learning (AI) ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Cleanup complete

✅ All search script examples completed! 🚀 ekoDB AI Scripts Example

⚠️ Note: These examples require AI API credentials (OpenAI, etc.)

📋 Setting up test data... ✅ Created 3 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved 🤖 AI Response:

1. Efficient Data Management: Vector databases allow efficient storage,
   management, and retrieval of spatial data.

2. Scalability: They can handle large amounts of data without compromising
   performance.

3. High Precision: Vector databases provide high precision due to the use of
   coordinates and points to represent data.

4. Superior Analysis Capabilities: They offer superior spatial analysis
   capabilities like network analysis, proximity analysis, overlay operations,
   etc.

5. Visual Appeal: Vector data models can represent data more realistically,
   creating visually appealing maps.

6. Data Integration: They allow seamless integration of different types of data
   (point, line, polygon).

7. Less Storage: Compared to raster databases, vector databases often require
   less storage space. ⏱️ Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding script saved 📊 Generated embeddings for 3 articles

1.  "Getting Started with ekoDB" - 1536D vector
2.  "Advanced Query Patterns" - 1536D vector
3.  "Draft Article" - 1536D vector ⏱️ Execution time: 0ms

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

BUILD SUCCESSFUL in 4s 2 actionable tasks: 2 up-to-date ✅ [32mKotlin client
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
Record(fields={id=StringValue(value=DYLDzoSxXtf2Se0dxlK22sEMk9rNl9pUEObwn6C6foszoOtDQ-CgwseRWHV2UH1tIt5FrSahUbVPBukz6zCMmA)})
User ID:
DYLDzoSxXtf2Se0dxlK22sEMk9rNl9pUEObwn6C6foszoOtDQ-CgwseRWHV2UH1tIt5FrSahUbVPBukz6zCMmA

=== Read === ✓ Found user by ID:
Record(fields={age=ObjectValue(value={type=StringValue(value=Integer),
value=IntegerValue(value=28)}),
name=ObjectValue(value={value=StringValue(value=Alice Johnson),
type=StringValue(value=String)}),
email=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=alice@example.com)}),
active=ObjectValue(value={type=StringValue(value=Boolean),
value=BooleanValue(value=true)}),
id=StringValue(value=DYLDzoSxXtf2Se0dxlK22sEMk9rNl9pUEObwn6C6foszoOtDQ-CgwseRWHV2UH1tIt5FrSahUbVPBukz6zCMmA)})

=== Update === ✓ Updated user:
Record(fields={city=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=San Francisco)}),
name=ObjectValue(value={value=StringValue(value=Alice Johnson),
type=StringValue(value=String)}),
email=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=alice@example.com)}),
active=ObjectValue(value={type=StringValue(value=Boolean),
value=BooleanValue(value=true)}),
age=ObjectValue(value={type=StringValue(value=Integer),
value=IntegerValue(value=29)}),
id=StringValue(value=DYLDzoSxXtf2Se0dxlK22sEMk9rNl9pUEObwn6C6foszoOtDQ-CgwseRWHV2UH1tIt5FrSahUbVPBukz6zCMmA)})

=== Query === ✓ Found 1 users matching query

- Record(fields={email=ObjectValue(value={type=StringValue(value=String),
  value=StringValue(value=alice@example.com)}),
  city=ObjectValue(value={type=StringValue(value=String),
  value=StringValue(value=San Francisco)}),
  active=ObjectValue(value={value=BooleanValue(value=true),
  type=StringValue(value=Boolean)}),
  name=ObjectValue(value={value=StringValue(value=Alice Johnson),
  type=StringValue(value=String)}),
  age=ObjectValue(value={type=StringValue(value=Integer),
  value=IntegerValue(value=29)}),
  id=StringValue(value=DYLDzoSxXtf2Se0dxlK22sEMk9rNl9pUEObwn6C6foszoOtDQ-CgwseRWHV2UH1tIt5FrSahUbVPBukz6zCMmA)})

=== Delete === ✓ Deleted user with ID:
DYLDzoSxXtf2Se0dxlK22sEMk9rNl9pUEObwn6C6foszoOtDQ-CgwseRWHV2UH1tIt5FrSahUbVPBukz6zCMmA

✓ Confirmed user was deleted

=== Cleanup === ✓ Deleted collection: kotlin_users_example

=== Example Complete ===

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
> Kotlin Client - Batch Operations Example ===

=== Batch Insert === ✓ Inserted 5 records IDs:
Iv9qPC9jhprV2XojPYJBh1Oxa3VoNvH8OTPt4woi3hXI7FUjbpxVCbrcDQGJxvDjq4Syyd7qnTH1kS26m4hPSQ,
jX_MjHC4VQEnFqK92G5xVH7Xv3D79zkVnvMdgX6cuCMUshxGqRkGHiNjVXtrU1PU0UPqH-dZkSlhU6GrzSt9oA,
xwIBFxpaNKGWv4v_9zhW3caFcF8ydX3sweaXX6Qmy_oFb2dLQH0Ro9pppQ0hxbF1c0apTG3eRMoAO8ths5mp9A...

=== Batch Update === ✓ Updated 3 records

=== Batch Delete === ✓ Deleted 2 records

=== Cleanup === ✓ Deleted collection: kotlin_batch_example

=== Example Complete ===

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
> Kotlin Client - Query Builder Example ===

=== Inserting Sample Data === ✓ Inserted 5 records

=== Query 1: Equality (city = 'NYC') === ✓ Found 2 records in NYC

=== Query 2: Range (age >= 25 AND age < 32) === ✓ Found 3 records with age 25-31

=== Query 3: Sort by score (descending) === ✓ Top 3 scores:

- Score: ObjectValue(value={value=IntegerValue(value=95),
  type=StringValue(value=Integer)})
- Score: ObjectValue(value={type=StringValue(value=Integer),
  value=IntegerValue(value=92)})
- Score: ObjectValue(value={type=StringValue(value=Integer),
  value=IntegerValue(value=88)})

=== Query 4: Complex (score > 80 AND age >= 25) === ✓ Found 4 high-scoring
adults

=== Query 5: IN (city IN ['NYC', 'LA']) === ✓ Found 4 records in NYC or LA

=== Query 6: Pagination (skip 2, limit 2) === ✓ Page 2 (2 records):

- ObjectValue(value={type=StringValue(value=String),
  value=StringValue(value=Charlie)})
- ObjectValue(value={type=StringValue(value=String),
  value=StringValue(value=Diana)})

=== Query 7: Contains (name contains 'a') === ✓ Found 2 names containing 'a'

=== Cleanup === ✓ Deleted collection: kotlin_query_example

=== Example Complete ===

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
> Kotlin Client - KV Operations Example ===

=== KV Set === ✓ Set key: user:123

=== KV Get === ✓ Retrieved value:
{"role":"admin","name":"Alice","email":"alice@example.com"}

=== KV Set with TTL === ✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key === ✓ Session value:
{"created_at":1766278221922,"user_id":"123"} (Will expire in 10 seconds)

=== Set Multiple Keys === ✓ Set 3 configuration keys

=== KV Delete === ✓ Deleted key: user:123

=== Verify Deletion === ✓ Confirmed key was deleted

=== Cleanup === ✓ Cleaned up test keys

=== Example Complete ===

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
> Kotlin Client - Collection Management Example ===

=== List Collections === ✓ Found 21 collections

- schema_documents_client_py
- websocket_test
- schema_employees_client_py
- schema_documents_client_go
- schema_products_client_go

=== Check Collection Existence === Collection 'kotlin_collection_example'
exists: false

=== Create Collection with Schema === ✓ Created collection with schema:
kotlin_collection_example

=== Get Collection Schema === ✓ Schema:
{"fields":{"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2025-12-21T00:50:26.882476Z","last_modified":"2025-12-21T00:50:26.882476Z","bypass_ripple":false,"primary_key_alias":"id"}

=== Cleanup === ✓ Deleted collection: kotlin_collection_example

=== Example Complete ===

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
> Kotlin Client - Document TTL Example ===

=== Insert with TTL === ✓ Inserted document with 10s TTL Document ID:
Rhkz3_qZO13s0RY_nShvFPtcJ7f8eWIvR757qY425U0rnjSDnOaln2S08z5VXuF3tqA-aikPVK1T9IYhnE3hFQ

=== Verify Document Exists === ✓ Document found: [created_at, id, session_id,
user_id, ttl_duration]

=== Insert with Longer TTL === ✓ Inserted document with 1h TTL Document ID:
K_yScS0KTgMieE5F-4TKdVPg-yfKC3RkWNoSBOrzM-Zilic4oKzPxhSmHd57NCtX8VGFiwESPJK6DVHRgs6YUA

=== TTL Expiration === ✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document === ✓ Long TTL document still exists: [cache_key,
ttl_duration, id, value]

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection: kotlin_ttl_example

=== Example Complete ===

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
> Kotlin Client - Simple WebSocket Example ===

=== Create WebSocket Client === ✓ WebSocket client created

=== Connect to WebSocket === ✓ Connected to WebSocket

=== Insert Test Record === ✓ Inserted test record

=== Find All via WebSocket === ✓ WebSocket findAll result:
{"payload":{"data":[{"id":"j16MA2iaMDaF1UaZ-CygKE9mKfmdl2TZ00YetpHQBKa7C7tF8qF3FOVr3TXI6aL6bq-lvnJAMr4YvIk2pO2d6A","name":{"type":"String","value":"Test
User"},"status":{"type":"String","value":"active"}}]},"type":"Success"}

=== Close WebSocket === ✓ WebSocket closed

=== Cleanup === ✓ Deleted collection: kotlin_websocket_example

=== Example Complete ===

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
> Kotlin Client - Schema Management Example ===

=== Creating Collection with Schema === ✓ Created collection
'kotlin_schema_example' with schema

=== Inserting Valid Documents === ✓ Inserted user 1:
StringValue(value=Va-RKxvWmuYlgRHSSmNgQhngwwqxEB2bgamLVvfSqXVi4jiFTZ_SDmhXhFZV5Kd90-\_V_ce_cMXFJakwkdnMaQ)
✓ Inserted user 2:
StringValue(value=q5NOZVDFPSq5LTPId7klddKfELyH2g4B4gGVt7MxCO0NavyG-d5RANLVlIkMQSOUiHNNWF3GuZqBZ01ekVk0Hw)

=== Getting Schema === ✓ Schema for kotlin_schema_example: Fields:
{"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections === ✓ Total collections: 22 Sample:
[schema_documents_client_py, websocket_test, schema_employees_client_py,
schema_products_client_go, schema_documents_client_go]

=== Cleanup === ✓ Deleted collection: kotlin_schema_example

✓ All schema management operations completed successfully

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
> Kotlin Client - Search Example ===

=== Inserting Sample Documents === ✓ Inserted 5 sample documents

=== Basic Text Search === ✓ Search results for 'programming':
{"results":[{"record":{"description":"Learn Rust programming language with
hands-on examples and best
practices.","views":365,"tags":"programming,rust,tutorial","id":"t-pnMBiLjq99KXBFsoRO9brBMu-CEYNY0aDd_5m198OtpLHEErUPPA-OWm25xtlksfLJ8PHIVEAbLt6DqxAVYg","title":"Rust
Programming"},"score":19.8,"matched_fields":["tags","title","description"]},{"record":{"id":"Orh2u_I_J4DzNsn4xdRNk0QU7-dzBnQvpD2q0g43WxiCJsr-0b4B_sX-i8YEtS6XHjJYpYItHam1VvnnKVr1IQ","title":"JavaScript
Web Development","description":"Build modern web applications using JavaScript,
React, and
Node.js.","tags":"programming,javascript,web","views":759},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"views":162,"title":"Python
for Data Science","description":"Master Python for data analysis, machine
learning, and
visualization.","id":"QBJiHtQ521LuWUqSENoOZehB8H1tAtdioQFblez-FQ3rZn10nQcEFDUWBpo4ViGjEP0cu8ot9OnQpzx30s2qsw","tags":"programming,python,data-science"},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' === ✓ Found results for 'machine learning'
{"results":[{"record":{"description":"Introduction to machine learning
algorithms and neural
networks.","id":"YZah-ArpamKEqiM2zQsBO9kd_tkS_x8UvVTr2n7F1EfPb3Qeo2Xn0xC6Ymz4PkYM7Rj7y7UvkeYba_HuFR0sAw","tags":"ai,machine-learning,python","views":790,"title":"Machine
Learning
Basics"},"score":2.7,"matched_fields":["tags","title","description"]},{"record":{"tags":"programming,python,data-science","id":"QBJiHtQ521LuWUqSENoOZehB8H1tAtdioQFblez-FQ3rZn10nQcEFDUWBpo4ViGjEP0cu8ot9OnQpzx30s2qsw","views":162,"description":"Master
Python for data analysis, machine learning, and visualization.","title":"Python
for Data
Science"},"score":1.0,"matched_fields":["description"]},{"record":{"description":"Learn
Rust programming language with hands-on examples and best
practices.","views":365,"tags":"programming,rust,tutorial","title":"Rust
Programming","id":"t-pnMBiLjq99KXBFsoRO9brBMu-CEYNY0aDd_5m198OtpLHEErUPPA-OWm25xtlksfLJ8PHIVEAbLt6DqxAVYg"},"score":0.5,"matched_fields":["description"]},{"record":{"views":488,"id":"cbZ0uiEWRzUO6tr6ZAITYxheiAPF26pGlW2-8Jbk3QqVxYfOx8ibzY1Lr2mzKBVEVeelddYax5Kxh_A7BXI31A","title":"Database
Design","description":"Learn database design principles, normalization, and
query
optimization.","tags":"database,design,sql"},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

=== Cleanup === ✓ Deleted collection: kotlin_search_example

✓ All search operations completed successfully

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
> Kotlin Client - WebSocket TTL Example ===

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
StringValue(value=JfHgmInJrPi0tTZx-KOAoWq8CivhOC-RRCAFfddbuHwRvkhbYTEhjCrmMg7vTms2fLK3wy18ilZvlLbjjVUdyQ)

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved data via
WebSocket:
{"payload":{"data":[{"created_at":{"type":"Integer","value":1766278251460},"id":"JfHgmInJrPi0tTZx-KOAoWq8CivhOC-RRCAFfddbuHwRvkhbYTEhjCrmMg7vTms2fLK3wy18ilZvlLbjjVUdyQ","name":{"type":"String","value":"WebSocket
TTL
Test"},"ttl_duration":{"type":"String","value":"1h"},"value":{"type":"Integer","value":42}}]},"type":"Success"}

✓ WebSocket closed

=== Cleanup === ✓ Deleted collection: kotlin_websocket_ttl_example

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration

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
> Kotlin Client - Join Operations Example ===

=== Setting up sample data === ✓ Sample data created

=== Example 1: Single collection join (users with departments) === ✓ Found 2
users with department data

- Alice Johnson: Engineering
- Bob Smith: Sales

=== Example 2: Join with filtering === ✓ Found 1 users in Engineering

- Alice Johnson: Building A

=== Example 3: Join with user profiles === ✓ Found 2 users with profile data

- Alice Johnson: Senior Software Engineer
- Bob Smith: Sales Manager

=== Example 4: Join orders with user data === ✓ Found 2 completed orders

- Laptop ($1200) by Alice Johnson
- Mouse ($25) by Alice Johnson

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
c9d3Y8ewnf9MimH-TdhlUSTuu0jBCGvNFPABwxWqMwXHCkAq7fPz1tG0Pvj0zhyKlkLhdIJ7KkN7CVLqN4g1hw

=== Sending Chat Message === ✓ Chat response: Message ID:
"pTaBV29wFeCPpS-LT_MD4IOd4cXt8Y6ABZZxrqJRx0MBWJQw8uH1428HBOYJ-tPa_rxGhWjlmjGt63t6H_JcsA"
Responses: ["I'm sorry, but the provided context doesn't contain any specific
information about ekoDB or its features. Could you please provide more
information or context?"]

=== Cleanup === ✓ Deleted chat session ✓ Deleted collection:
kotlin_chat_basic_example

✓ Basic chat example completed successfully

BUILD SUCCESSFUL in 28s 2 actionable tasks: 1 executed, 1 up-to-date To honour
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
21tc0ZL5xevd9DvKaU5he5n9dYIeW8e5sYN8hk5iuO-bh2C0FtwR9wBut5byshTcws2lmzrIpkjM2BtoiOGFsg

=== Sending Messages === ✓ Message 1 sent Responses: ["I'm sorry, but the
context provided does not contain specific information about any available
products. Could you please provide more details or context?"]

✓ Message 2 sent Responses: ["I'm sorry, but I can't provide the information
you're looking for because the context doesn't specify which product's price
you're asking about. Could you please provide more details or specify the
product?"]

=== Getting Message History === ✓ Retrieved message history Total messages:
[{"chat*id":{"type":"String","value":"21tc0ZL5xevd9DvKaU5he5n9dYIeW8e5sYN8hk5iuO-bh2C0FtwR9wBut5byshTcws2lmzrIpkjM2BtoiOGFsg"},"content":{"type":"String","value":"What
products are
available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":{"type":"String","value":"A
high-performance database product with AI
capabilities"},"id":"z5nvL6Bja9ayvUYpSap-KHmRokENKiYCsuiZfh5IZJPlq-w1wvpAzyTuVzW49inXYdIqGqX9kmLvn0yZBB8N1g","price":{"type":"Integer","value":99},"product":{"type":"String","value":"ekoDB"}},"score":0.25}]},"created_at":{"type":"DateTime","value":"2025-12-21T00:51:32.545516Z"},"id":"10Y48OXtQ_uwHEfuCQODnqEYWoUOGM-hpZ6AJBOENrz9BQB-GVii8sxBSyM3W-nMTXguSol6--XdBF3ITEjf4Q","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":27,"prompt_tokens":155,"total_tokens":182}},"updated_at":{"type":"DateTime","value":"2025-12-21T00:51:32.545516Z"}},{"chat_id":{"type":"String","value":"21tc0ZL5xevd9DvKaU5he5n9dYIeW8e5sYN8hk5iuO-bh2C0FtwR9wBut5byshTcws2lmzrIpkjM2BtoiOGFsg"},"content":{"type":"String","value":"I'm
sorry, but the context provided does not contain specific information about any
available products. Could you please provide more details or
context?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":{"type":"String","value":"A
high-performance database product with AI
capabilities"},"id":"z5nvL6Bja9ayvUYpSap-KHmRokENKiYCsuiZfh5IZJPlq-w1wvpAzyTuVzW49inXYdIqGqX9kmLvn0yZBB8N1g","price":{"type":"Integer","value":99},"product":{"type":"String","value":"ekoDB"}},"score":0.25}]},"created_at":{"type":"DateTime","value":"2025-12-21T00:51:32.549352Z"},"id":"fj_az8roYBQK9sl4UH7Oaa-OqcqGpVpK8bdnomCQVWC6dCLQBb0ogeAl-Si7EjUUTUve9aPt2Hd*-eo2h18cig","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":27,"prompt_tokens":155,"total_tokens":182}},"updated_at":{"type":"DateTime","value":"2025-12-21T00:51:32.549352Z"}},{"chat_id":{"type":"String","value":"21tc0ZL5xevd9DvKaU5he5n9dYIeW8e5sYN8hk5iuO-bh2C0FtwR9wBut5byshTcws2lmzrIpkjM2BtoiOGFsg"},"content":{"type":"String","value":"What
is the
price?"},"created_at":{"type":"DateTime","value":"2025-12-21T00:51:34.186469Z"},"id":"Wrv4I9CNgyIF-yp6yf6UT1ZRDIMj5T2tlFJ-qopVyRjKeTxEe4ud8N4mRJCYj6CHo8AP-xnN3fLC0BK7wFsMXQ","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":41,"prompt_tokens":70,"total_tokens":111}},"updated_at":{"type":"DateTime","value":"2025-12-21T00:51:34.186469Z"}},{"chat_id":{"type":"String","value":"21tc0ZL5xevd9DvKaU5he5n9dYIeW8e5sYN8hk5iuO-bh2C0FtwR9wBut5byshTcws2lmzrIpkjM2BtoiOGFsg"},"content":{"type":"String","value":"I'm
sorry, but I can't provide the information you're looking for because the
context doesn't specify which product's price you're asking about. Could you
please provide more details or specify the
product?"},"created_at":{"type":"DateTime","value":"2025-12-21T00:51:34.189195Z"},"id":"eciWUq5HpTWWpCUQ-d9V98DrhmrsDFJaMc9cYz9yV_dOVMZwolzR3XYwDs5CRfMwh6Cy6ctiXzmOSNmz97wPdA","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":41,"prompt_tokens":70,"total_tokens":111}},"updated_at":{"type":"DateTime","value":"2025-12-21T00:51:34.189195Z"}}]

=== Updating Session === ✓ Updated session system prompt

=== Listing Sessions === ✓ Total sessions: 13

=== Branching Session === ✓ Created branched session:
3EW1hxDZdhdIpFl3rOLPuRZxazXVtyjaa0poSZtKREEMZ7vzIlkNgwBj5Hg2Go9Ff8aEOGVRm15gyN6JGh8WaQ

=== Cleanup === ✓ Deleted chat sessions ✓ Deleted collection:
kotlin_chat_sessions_example

✓ Chat session management example completed successfully

BUILD SUCCESSFUL in 7s 2 actionable tasks: 1 executed, 1 up-to-date To honour
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
kX1v2DYNJHfXjaLO-q9j9FZh81RLW3F9ULSqpOnmvnXklN8xAOt3ZtflMeXT1ft3mf8MGkkKp6IHGW33m671mA

=== Sending Initial Message === ✓ Message sent Responses: ["I'm sorry, but the
context provided does not contain any specific information about available
products. Could you please provide more details or try again?"]

✓ Second message sent

=== Regenerating AI Response === ✓ AI response regenerated New responses: ["I'm
sorry, but the context provided does not include information about any specific
products. Could you please provide more details or clarify what type of products
you're interested in?"]

=== Updating Message === ✓ Updated message content

=== Toggling Forgotten Status === ✓ Marked message as forgotten (excluded from
context)

=== Creating Second Session for Merge === ✓ Created second session:
loMignSlsWBaEn-oJlJY7DiwlfXYHWQVoJWvnP1qdF_qrae_f0VGbMMueyf9KXUYqLmyDSF_1PNgUtDb4-T0rQ

=== Merging Sessions === ✓ Merged sessions Merged session ID: null

=== Deleting Message === ✓ Deleted message

=== Cleanup === ✓ Deleted chat sessions ✓ Deleted collection:
kotlin_chat_advanced_example

✓ Advanced chat features example completed successfully

BUILD SUCCESSFUL in 8s 2 actionable tasks: 1 executed, 1 up-to-date To honour
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
5Ppjw4l7d\_\_5fhNv-zUcQn-FM2_Vs8L0NPg98bHVB8xMlYtOZWRrk1cShfn6WTkg5AyxBP7VWQx9Xs6SnDcoLw
📊 Found 10 records ⏱️ Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved 📊 Found 10 users (limited) ⏱️ Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved 📊 Statistics: 2 groups
{"status":"active","avg_score":60.0,"count":5}
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

BUILD SUCCESSFUL in 4s 2 actionable tasks: 1 executed, 1 up-to-date ✅
[32mKotlin client examples complete![0m 🟣 [33mKotlin Transactions...[0m
/bin/sh: kotlinc: command not found Kotlin example (requires kotlinc installed)
✅ [32mAll Kotlin integration tests complete![0m

🤖 [36mRAG Conversation System Examples[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m

[33mPrerequisites:[0m

1. ekoDB server running (make run in ekodb/)
2. OPENAI_API_KEY set in server environment
3. API_BASE_URL and API_BASE_KEY exported in your shell

[36mBuilding Rust client library...[0m ✓ Rust client built

[36mBuilding Python client bindings...[0m 🍹 Building a mixed python/rust
project 🔗 Found pyo3 bindings with abi3 support 🐍 Not using a specific python
interpreter 💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by
default 📦 Built wheel for abi3 Python ≥ 3.8 to
/Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.4.0-cp38-abi3-macosx_11_0_arm64.whl
✓ Python client built and installed

[36mBuilding TypeScript client library...[0m ✓ TypeScript client built

[36mInstalling TypeScript client in examples...[0m ✓ TypeScript client installed

[36mBuilding TypeScript example...[0m ✓ TypeScript example built

[36mBuilding Go client library...[0m ✓ Go client built

[36mBuilding Go RAG example...[0m ✓ Go example built

[36mBuilding Kotlin client library...[0m ✓ Kotlin client built

[36mBuilding Kotlin RAG example...[0m ✓ Kotlin example built

[36mRunning Rust RAG Example...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m === ekoDB RAG
Conversation System ===

This example shows how ekoDB can power a self-improving AI system that learns
from its own conversation history.

=== Step 1: Building Conversation History === Storing previous conversations
with embeddings...

→ Calling ekoDB embed() helper... • Using model: text-embedding-3-small • Text
length: 34 characters • Behind the scenes: Creating temp Function with Embed
operation ✓ Generated embedding: 1536 dimensions in 655.381459ms • Function
auto-cleaned up by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 169 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 900.51375ms • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 33
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 810.318292ms • Function auto-cleaned up
by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 230 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 412.315625ms • Function auto-cleaned up by client ✓ Stored Rust
programming conversation (4 messages) → Calling ekoDB embed() helper... • Using
model: text-embedding-3-small • Text length: 31 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 898.599208ms • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 217
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 707.353916ms • Function auto-cleaned up
by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 33 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 314.216375ms • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 232
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 321.235084ms • Function auto-cleaned up
by client ✓ Stored database design conversation (4 messages) → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 36
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 272.687583ms • Function auto-cleaned up
by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 178 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 473.736ms • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 37
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 303.668542ms • Function auto-cleaned up
by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 213 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 337.012875ms • Function auto-cleaned up by client ✓ Stored
performance optimization conversation (4 messages)

=== Step 2: New User Question with Context Retrieval === User asks: "How do I
write memory-safe high-performance database code?"

=== Step 3: Searching Related Context === Using hybrid search to find relevant
messages from all conversations...

→ Generating embedding for user question... → Calling ekoDB embed() helper... •
Using model: text-embedding-3-small • Text length: 58 characters • Behind the
scenes: Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 319.341042ms • Function auto-cleaned up by client

→ Executing hybrid_search()... • Collection: rag_messages • Query text: "How do
I write memory-safe high-performance database code?" • Vector dimensions: 1536 •
Limit: 5 results • Search type: Semantic (vector) + Keyword (text) • Server
combines both scores for relevance ranking ✓ Search completed in 121.122459ms

✓ Found 5 related messages across all conversations:

1. From conv_database_design Use NoSQL when you need: flexible schemas,
   horizontal scaling, high write throughput, or when working with unstructured
   data. SQL is better for complex queries, ACID transactions, and structured
   data with well-defined relationships.

2. From conv_database_design What is database normalization?

3. From conv_database_design Database normalization is the process of organizing
   data to reduce redundancy and improve data integrity. It involves dividing
   large tables into smaller ones and defining relationships between them using
   foreign keys.

4. From conv_database_design When should I use NoSQL over SQL?

5. From conv_performance How can I optimize database queries?

=== Step 4: Generating Context-Aware Response === ✓ AI Response (with context
from 3 conversations):

Answer: Writing memory-safe, high-performance database code involves a
combination of good practices in both programming and database management. Here
are some tips:

1. **Normalization**: As discussed in Context 2 and 3, database normalization
   can help reduce redundancy and improve data integrity. This can also lead to
   performance benefits because it reduces the amount of data that needs to be
   loaded into memory.

2. **Optimize Queries**: As per Context 5, optimizing your database queries is
   crucial for performance. This could involve using indexes effectively,
   avoiding unnecessary columns in SELECT statements, leveraging the WHERE
   clause to limit data retrieval, and using JOINs effectively.

3. **Choosing the Right Database Type**: Depending on the nature of your data
   and the operations you need to perform, you might choose SQL or NoSQL
   databases (Context 1 and 4). For instance, if you're working with
   unstructured data and require high write throughput, a NoSQL database might
   be more suitable. On the other hand, SQL databases are better for complex
   queries and ACID transactions, which might be important for maintaining data
   integrity and consistency in memory-intensive applications.

4. **Use Efficient Data Structures and Algorithms**: In your code, make sure to
   use efficient data structures and algorithms that minimize memory usage and
   maximize performance. For instance, avoid using data structures that have a
   high memory overhead.

5. **Memory Management**: Depending on the programming language you are using,
   you might need to manually manage memory (e.g., in C or C++) or it might be
   automatically managed for you (e.g., in Python or Java). If you are manually
   managing memory, make sure to free up memory that is no longer needed to
   avoid memory leaks.

6. **Concurrency Control**: If multiple processes or threads are accessing and
   modifying your database, you need to implement proper concurrency control
   mechanisms to avoid conflicts and ensure data integrity.

7. **Caching**: Implement caching to save and quickly retrieve frequently
   accessed data, reducing the need for expensive database operations.

Remember, the best practices can vary depending on the specific use case,
database system, and programming language used.

=== Step 5: Storing New Conversation === → Calling ekoDB embed() helper... •
Using model: text-embedding-3-small • Text length: 58 characters • Behind the
scenes: Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 371.825792ms • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 2261
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 415.602667ms • Function auto-cleaned up
by client ✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search === Searching for messages about
'ownership' across ALL conversations...

→ Executing text_search()... • Collection: rag_messages • Query: "ownership
system" • Limit: 3 results • Search method: Full-text with fuzzy matching &
stemming • No vector embeddings needed - pure keyword search ✓ Text search
completed in 69.043875ms

✓ Found 3 messages mentioning ownership:

1. From conv_performance: Rust's ownership system provides zero-cost memory
   management. Use Box for heap allocation, Rc/Arc for shared ownership, and
   avoid cloning large data structures. The compiler optimizes away unnecessary
   allocations.

2. From conv_rust_programming: Rust's key features include: memory safety
   without garbage collection, zero-cost abstractions, ownership system,
   powerful type system, and excellent concurrency support.

3. From conv_rust_programming: The borrow checker enforces Rust's ownership
   rules at compile time. It ensures that references don't outlive the data they
   point to and prevents data races by allowing either multiple immutable
   references or one mutable reference.

=== System Statistics ===

→ Querying database statistics... • Using find_all() helper - simplified query
API

📊 Database Statistics: • Total conversations: 4 • Total messages stored: 14 •
All messages indexed for vector search ✓ • All messages indexed for text search
✓ • All messages queryable by metadata ✓

=== Step 8: Dynamic Search Configuration === Each conversation can have its own
search config...

💡 Conversations can store custom search configurations: • Search type: hybrid,
text, or vector • Relevance thresholds • Filter by tags or metadata •
Collection-specific settings • Per-conversation AI behavior

This enables context-aware search tuned to each conversation's needs!

=== Cleanup === Deleting example collections...

✅ All done! RAG system demonstrated successfully.

✓ Using search results to enhance AI responses (RAG) ✓ Building a self-improving
knowledge base ✓ Dynamic search configurations per conversation

ekoDB provides everything needed for AI-powered applications: • Vector search
(semantic similarity) • Text search (keyword matching) • Hybrid search (best of
both) • AI functions (Chat, Embed) • Flexible querying and filtering • All in
one database - no external dependencies!

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m [36mRunning
Python RAG Example...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m === ekoDB RAG
Conversation System ===

This example shows how ekoDB can power a self-improving AI system that learns
from its own conversation history.

=== Step 1: Building Conversation History === Storing previous conversations
with embeddings...

→ Calling ekoDB embed() helper... • Using model: text-embedding-3-small • Text
length: 34 characters • Behind the scenes: Creating temp Function with Embed
operation ✓ Generated embedding: 1536 dimensions in 0.280s • Function
auto-cleaned up by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 169 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.498s • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 33
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.317s • Function auto-cleaned up by
client → Calling ekoDB embed() helper... • Using model: text-embedding-3-small •
Text length: 230 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.568s • Function
auto-cleaned up by client ✓ Stored Rust programming conversation (4 messages) →
Calling ekoDB embed() helper... • Using model: text-embedding-3-small • Text
length: 31 characters • Behind the scenes: Creating temp Function with Embed
operation ✓ Generated embedding: 1536 dimensions in 1.068s • Function
auto-cleaned up by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 217 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.279s • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 33
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.290s • Function auto-cleaned up by
client → Calling ekoDB embed() helper... • Using model: text-embedding-3-small •
Text length: 232 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.209s • Function
auto-cleaned up by client ✓ Stored database design conversation (4 messages) →
Calling ekoDB embed() helper... • Using model: text-embedding-3-small • Text
length: 36 characters • Behind the scenes: Creating temp Function with Embed
operation ✓ Generated embedding: 1536 dimensions in 0.350s • Function
auto-cleaned up by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 178 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.243s • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 37
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.312s • Function auto-cleaned up by
client → Calling ekoDB embed() helper... • Using model: text-embedding-3-small •
Text length: 213 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.291s • Function
auto-cleaned up by client ✓ Stored performance optimization conversation (4
messages)

=== Step 2: New User Question with Context Retrieval === User asks: "How do I
write memory-safe high-performance database code?"

=== Step 3: Searching Related Context === Using hybrid search to find relevant
messages from all conversations...

→ Generating embedding for user question... → Calling ekoDB embed() helper... •
Using model: text-embedding-3-small • Text length: 58 characters • Behind the
scenes: Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.545s • Function auto-cleaned up by client

→ Executing hybrid_search()... • Collection: rag_messages • Query text: "How do
I write memory-safe high-performance database code?" • Vector dimensions: 1536 •
Limit: 5 results • Search type: Semantic (vector) + Keyword (text) • Server
combines both scores for relevance ranking ✓ Search completed in 0.091s

✓ Found 5 related messages across all conversations:

1. [Score: 0.000] From conv_database_design Database normalization is the
   process of organizing data to reduce redundancy and improve data integrity.
   It involves dividing large tables into smaller ones and defining
   relationships between them using foreign keys.

2. [Score: 0.000] From conv_database_design Use NoSQL when you need: flexible
   schemas, horizontal scaling, high write throughput, or when working with
   unstructured data. SQL is better for complex queries, ACID transactions, and
   structured data with well-defined relationships.

3. [Score: 0.000] From conv_database_design What is database normalization?

4. [Score: 0.000] From conv_database_design When should I use NoSQL over SQL?

5. [Score: 0.000] From conv_performance How can I optimize database queries?

=== Step 4: Generating Context-Aware Response === ✓ AI Response (with context
from 3 conversations):

Answer: Writing memory-safe, high-performance database code involves a number of
strategies:

1. Normalization: As mentioned in Context 1, database normalization can help
   improve data integrity and reduce redundancy. This can lead to more efficient
   use of memory.

2. Choosing the right database type: Depending on your specific needs and the
   nature of your data, you may want to use SQL or NoSQL. As indicated in
   Context 2, NoSQL is ideal for flexible schemas, high write throughput,
   horizontal scaling, and working with unstructured data. SQL, on the other
   hand, is better for complex queries, ACID transactions, and structured data
   with well-defined relationships.

3. Query Optimization: One key aspect of high-performance database code is
   optimizing your queries. This can involve strategies such as using indexes
   effectively, avoiding full table scans by using WHERE clauses, and minimizing
   the use of wildcards.

4. Efficient Memory Management: To write memory-safe code, always ensure to free
   up any resources that are no longer needed. Avoid memory leaks by ensuring
   every allocated byte of memory is eventually freed.

5. Use of Prepared Statements: Prepared statements can be used to execute the
   same or similar database statements repeatedly with high efficiency. They are
   also beneficial from a security perspective as they assist in preventing SQL
   injection attacks.

6. Regular Monitoring and Tuning: Regularly monitor your database performance
   and tune it as needed. This could involve hardware upgrades, adjusting
   configuration settings, or modifying your database schema.

Remember, the specific methods and techniques for writing high-performance,
memory-safe code can vary depending on the specific database system being used.

=== Step 5: Storing New Conversation === → Calling ekoDB embed() helper... •
Using model: text-embedding-3-small • Text length: 58 characters • Behind the
scenes: Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.501s • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 1747
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.389s • Function auto-cleaned up by
client ✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search === Searching for messages about
'ownership' across ALL conversations...

✓ Found 3 messages mentioning ownership:

1. From conv_performance: Rust's ownership system provides zero-cost memory
   management. Use Box for heap allocation, Rc/Arc for shared ownership, and
   avoid cloning large data structures. The compiler optimizes away unnecessary
   allocations.

2. From conv_rust_programming: Rust's key features include: memory safety
   without garbage collection, zero-cost abstractions, ownership system,
   powerful type system, and excellent concurrency support.

3. From conv_rust_programming: The borrow checker enforces Rust's ownership
   rules at compile time. It ensures that references don't outlive the data they
   point to and prevents data races by allowing either multiple immutable
   references or one mutable reference.

=== System Statistics === Total conversations: 4 Total messages stored: 14 All
messages are indexed for vector search ✓ All messages are indexed for text
search ✓ All messages are queryable by metadata ✓

=== Step 8: Dynamic Search Configuration === Each conversation can have its own
search config...

💡 Conversations can store custom search configurations: • Search type: hybrid,
text, or vector • Relevance thresholds • Filter by tags or metadata •
Collection-specific settings • Per-conversation AI behavior

This enables context-aware search tuned to each conversation's needs!

=== Cleanup === ✓ Cleanup complete

=== 📚 Summary: What This Example Showed ===

🔧 ekoDB Native Capabilities Used: ✓ Functions with Embed operation (AI
integration) ✓ Hybrid Search (text + vector combined) ✓ Text Search (full-text
with stemming) ✓ Automatic embedding generation ✓ Cross-collection queries

🚀 New Client Helper Methods: • client.embed(text, model) - Generate embeddings
• client.hybrid_search() - Semantic + keyword search • client.text_search() -
Full-text search • client.find_all() - Query all documents

💡 Key Takeaways:

1. ekoDB handles AI Functions natively - no external services needed
2. One-line embedding generation with auto-cleanup
3. Hybrid search combines semantic understanding + keyword matching
4. Perfect for RAG: store, search, and retrieve context
5. All AI capabilities accessible through simple client methods

🎯 Build production RAG systems with ekoDB! → Set OPENAI_API_KEY in your ekoDB
server environment → Use these client helpers to make AI integration simple →
Scale to millions of documents with native indexing

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m [36mRunning
TypeScript RAG Example...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m === ekoDB RAG
Conversation System ===

This example shows how ekoDB can power a self-improving AI system that learns
from its own conversation history.

=== Step 1: Building Conversation History === Storing previous conversations
with embeddings...

→ Calling ekoDB embed() helper... • Using model: text-embedding-3-small • Text
length: 34 characters • Behind the scenes: Creating temp Function with Embed
operation ✓ Generated embedding: 1536 dimensions in 0.452s • Function
auto-cleaned up by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 169 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.239s • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 33
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.270s • Function auto-cleaned up by
client → Calling ekoDB embed() helper... • Using model: text-embedding-3-small •
Text length: 230 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.253s • Function
auto-cleaned up by client ✓ Stored Rust programming conversation (4 messages) →
Calling ekoDB embed() helper... • Using model: text-embedding-3-small • Text
length: 31 characters • Behind the scenes: Creating temp Function with Embed
operation ✓ Generated embedding: 1536 dimensions in 0.207s • Function
auto-cleaned up by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 217 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.306s • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 33
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.298s • Function auto-cleaned up by
client → Calling ekoDB embed() helper... • Using model: text-embedding-3-small •
Text length: 232 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.479s • Function
auto-cleaned up by client ✓ Stored database design conversation (4 messages) →
Calling ekoDB embed() helper... • Using model: text-embedding-3-small • Text
length: 36 characters • Behind the scenes: Creating temp Function with Embed
operation ✓ Generated embedding: 1536 dimensions in 0.262s • Function
auto-cleaned up by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 178 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.272s • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 37
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.259s • Function auto-cleaned up by
client → Calling ekoDB embed() helper... • Using model: text-embedding-3-small •
Text length: 213 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.244s • Function
auto-cleaned up by client ✓ Stored performance optimization conversation (4
messages)

=== Step 2: New User Question with Context Retrieval === User asks: "How do I
write memory-safe high-performance database code?"

=== Step 3: Searching Related Context === Using hybrid search to find relevant
messages from all conversations...

→ Generating embedding for user question... → Calling ekoDB embed() helper... •
Using model: text-embedding-3-small • Text length: 58 characters • Behind the
scenes: Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.208s • Function auto-cleaned up by client

→ Executing hybridSearch()... • Collection: rag_messages • Query text: "How do I
write memory-safe high-performance database code?" • Vector dimensions: 1536 •
Limit: 5 results • Search type: Semantic (vector) + Keyword (text) • Server
combines both scores for relevance ranking ✓ Search completed in 0.090s ✓ Found
5 related messages across all conversations:

1. [Score: 0.000] From conv_database_design Database normalization is the
   process of organizing data to reduce redundancy and improve data integrity.
   It involves dividing large tables into smaller ones and defining
   relationships between them using foreign keys.

2. [Score: 0.000] From conv_database_design Use NoSQL when you need: flexible
   schemas, horizontal scaling, high write throughput, or when working with
   unstructured data. SQL is better for complex queries, ACID transactions, and
   structured data with well-defined relationships.

3. [Score: 0.000] From conv_database_design What is database normalization?

4. [Score: 0.000] From conv_database_design When should I use NoSQL over SQL?

5. [Score: 0.000] From conv_performance How can I optimize database queries?

=== Step 4: Generating Context-Aware Response === ✓ AI Response (with context
from 3 conversations):

Answer: Writing memory-safe, high-performance database code involves a
combination of good design principles, effective query optimization, and
appropriate database selection:

1. Database Design: As mentioned earlier, database normalization is a key factor
   for data integrity and reduction of redundancy. A well-structured database
   can greatly enhance both performance and safety.

2. Query Optimization: Optimizing your database queries is fundamental for high
   performance. This might involve using indexes wisely, minimizing the use of
   joins, limiting the returned data by using SELECT instead of SELECT \*, and
   avoiding nested queries as much as possible.

3. Database Selection: Depending on your needs, you might choose SQL or NoSQL.
   SQL databases are typically better for complex queries and structured data
   with well-defined relationships, while NoSQL databases are better suited for
   flexible schemas, high write throughput, and unstructured data.

4. Memory Safety: To ensure memory safety, avoid operations that can cause
   buffer overflow or underflow, null pointer dereferencing, and memory leaks.
   In a high-level language, these are usually handled for you, but in
   lower-level languages, you may need to manage memory manually.

5. Caching: Implement caching strategies to reduce the load on your database,
   this can significantly improve performance and reduce cost.

6. Use Database Features: Most databases come with features to improve
   performance and ensure safety. These may include stored procedures, triggers,
   and views in SQL databases, or sharding and replication in NoSQL databases.

Remember, the choice of database and the specific techniques used will largely
depend on your specific use case and the nature of the data you're working with.

=== Step 5: Storing New Conversation === → Calling ekoDB embed() helper... •
Using model: text-embedding-3-small • Text length: 58 characters • Behind the
scenes: Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.246s • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 1763
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.280s • Function auto-cleaned up by
client ✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search === Searching for messages about
'ownership' across ALL conversations...

→ Executing textSearch()... • Collection: rag_messages • Query: "ownership
system" • Limit: 3 results • Search method: Full-text with fuzzy matching &
stemming • No vector embeddings needed - pure keyword search ✓ Text search
completed in 0.052s ✓ Found 3 messages mentioning ownership:

1. From conv_performance: Rust's ownership system provides zero-cost memory
   management. Use Box for heap allocation, Rc/Arc for shared ownership, and
   avoid cloning large data structures. The compiler optimizes away unnecessary
   allocations.

2. From conv_rust_programming: Rust's key features include: memory safety
   without garbage collection, zero-cost abstractions, ownership system,
   powerful type system, and excellent concurrency support.

3. From conv_rust_programming: The borrow checker enforces Rust's ownership
   rules at compile time. It ensures that references don't outlive the data they
   point to and prevents data races by allowing either multiple immutable
   references or one mutable reference.

=== System Statistics ===

→ Querying database statistics... • Using findAllWithLimit() helper - simplified
query API

📊 Database Statistics: • Total conversations: 4 • Total messages stored: 14 •
All messages indexed for vector search ✓ • All messages indexed for text search
✓ • All messages queryable by metadata ✓

=== Step 8: Dynamic Search Configuration === Each conversation can have its own
search config...

💡 Conversations can store custom search configurations: • Search type: hybrid,
text, or vector • Relevance thresholds • Filter by tags or metadata •
Collection-specific settings • Per-conversation AI behavior

This enables context-aware search tuned to each conversation's needs!

=== Cleanup === ✓ Cleanup complete

=== 📚 Summary: What This Example Showed ===

🔧 ekoDB Native Capabilities Used: ✓ Functions with Embed operation (AI
integration) ✓ Hybrid Search (text + vector combined) ✓ Text Search (full-text
with stemming) ✓ Automatic embedding generation ✓ Cross-collection queries

🚀 New Client Helper Methods: • client.embed(text, model) - Generate embeddings
• client.hybridSearch() - Semantic + keyword search • client.textSearch() -
Full-text search • client.findAllWithLimit() - Query all documents

💡 Key Takeaways:

1. ekoDB handles AI Functions natively - no external services needed
2. One-line embedding generation with auto-cleanup
3. Hybrid search combines semantic understanding + keyword matching
4. Perfect for RAG: store, search, and retrieve context
5. All AI capabilities accessible through simple client methods

🎯 Build production RAG systems with ekoDB! → Set OPENAI_API_KEY in your ekoDB
server environment → Use these client helpers to make AI integration simple →
Scale to millions of documents with native indexing

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m [36mRunning Go
RAG Example...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m === ekoDB RAG
Conversation System ===

This example shows how ekoDB can power a self-improving AI system that learns
from its own conversation history.

=== Step 1: Building Conversation History === Storing previous conversations
with embeddings...

→ Calling ekoDB Embed() helper... • Using model: text-embedding-3-small • Text
length: 34 characters • Behind the scenes: Creating temp Function with Embed
operation ✓ Generated embedding: 1536 dimensions in 0.327s • Function
auto-cleaned up by client → Calling ekoDB Embed() helper... • Using model:
text-embedding-3-small • Text length: 169 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.384s • Function auto-cleaned up by client → Calling ekoDB
Embed() helper... • Using model: text-embedding-3-small • Text length: 33
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.229s • Function auto-cleaned up by
client → Calling ekoDB Embed() helper... • Using model: text-embedding-3-small •
Text length: 230 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.234s • Function
auto-cleaned up by client ✓ Stored Rust programming conversation (4 messages) →
Calling ekoDB Embed() helper... • Using model: text-embedding-3-small • Text
length: 31 characters • Behind the scenes: Creating temp Function with Embed
operation ✓ Generated embedding: 1536 dimensions in 0.259s • Function
auto-cleaned up by client → Calling ekoDB Embed() helper... • Using model:
text-embedding-3-small • Text length: 217 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.273s • Function auto-cleaned up by client → Calling ekoDB
Embed() helper... • Using model: text-embedding-3-small • Text length: 33
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.243s • Function auto-cleaned up by
client → Calling ekoDB Embed() helper... • Using model: text-embedding-3-small •
Text length: 232 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.286s • Function
auto-cleaned up by client ✓ Stored database design conversation (4 messages) →
Calling ekoDB Embed() helper... • Using model: text-embedding-3-small • Text
length: 36 characters • Behind the scenes: Creating temp Function with Embed
operation ✓ Generated embedding: 1536 dimensions in 0.309s • Function
auto-cleaned up by client → Calling ekoDB Embed() helper... • Using model:
text-embedding-3-small • Text length: 178 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.469s • Function auto-cleaned up by client → Calling ekoDB
Embed() helper... • Using model: text-embedding-3-small • Text length: 37
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 1.015s • Function auto-cleaned up by
client → Calling ekoDB Embed() helper... • Using model: text-embedding-3-small •
Text length: 213 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.294s • Function
auto-cleaned up by client ✓ Stored performance optimization conversation (4
messages)

=== Step 2: New User Question with Context Retrieval === User asks: "How do I
write memory-safe high-performance database code?"

=== Step 3: Searching Related Context === Using hybrid search to find relevant
messages from all conversations...

→ Generating embedding for user question... → Calling ekoDB Embed() helper... •
Using model: text-embedding-3-small • Text length: 58 characters • Behind the
scenes: Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.256s • Function auto-cleaned up by client

→ Executing HybridSearch()... • Collection: rag_messages • Query text: "How do I
write memory-safe high-performance database code?" • Vector dimensions: 1536 •
Limit: 5 results • Search type: Semantic (vector) + Keyword (text) • Server
combines both scores for relevance ranking ✓ Search completed in 0.085s

✓ Found 5 related messages across all conversations:

1. [Score: 0.000] From conv_database_design Database normalization is the
   process of organizing data to reduce redundancy and improve data integrity.
   It involves dividing large tables into smaller ones and defining
   relationships between them using foreign keys.

2. [Score: 0.000] From conv_database_design What is database normalization?

3. [Score: 0.000] From conv_database_design Use NoSQL when you need: flexible
   schemas, horizontal scaling, high write throughput, or when working with
   unstructured data. SQL is better for complex queries, ACID transactions, and
   structured data with well-defined relationships.

4. [Score: 0.000] From conv_database_design When should I use NoSQL over SQL?

5. [Score: 0.000] From conv_performance How can I optimize database queries?

=== Step 4: Generating Context-Aware Response === ✓ AI Response (with context
from 3 conversations):

Writing memory-safe, high-performance database code involves a variety of
practices. Here are a few key ones:

1. Database Normalization: As mentioned in Context 1, normalization helps to
   organize data, reduce redundancy, and improve data integrity. This can lead
   to improved performance and memory usage as it avoids unnecessary data
   duplication.

2. Correct Database Choice: Depending on your needs, you might choose between
   SQL and NoSQL databases (as discussed in Context 3 and 4). For instance, if
   you're dealing with highly structured data and complex queries, an SQL
   database might be more suitable. On the other hand, for unstructured data and
   high write throughput, a NoSQL database could be more efficient.

3. Query Optimization: Context 5 talks about optimizing database queries, which
   is vital for high-performance code. This could involve techniques such as
   using indexes effectively, avoiding unnecessary columns in SELECT statements,
   minimizing the use of wildcards, and using JOIN operations instead of
   multiple queries when possible.

4. Managing Connections: Open and close connections judiciously to avoid memory
   leaks. Use connection pools to manage connections efficiently and prevent
   system resources from being overwhelmed.

5. Caching: Cache frequently accessed data to avoid unnecessary database hits.
   This can significantly improve performance and reduce memory usage.

Remember, writing memory-safe, high-performance database code is a broad topic
and these are just a few points to get started. Depending on the specific
database system and programming language you are using, there might be more
specific strategies that you can employ.

=== Step 5: Storing New Conversation === → Calling ekoDB Embed() helper... •
Using model: text-embedding-3-small • Text length: 58 characters • Behind the
scenes: Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.307s • Function auto-cleaned up by client → Calling ekoDB
Embed() helper... • Using model: text-embedding-3-small • Text length: 1666
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.604s • Function auto-cleaned up by
client ✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search === Searching for messages about
'ownership' across ALL conversations...

→ Executing TextSearch()... • Collection: rag_messages • Query: "ownership
system" • Limit: 3 results • Search method: Full-text with fuzzy matching &
stemming • No vector embeddings needed - pure keyword search ✓ Text search
completed in 0.056s

✓ Found 3 messages mentioning ownership:

1. From conv_performance: Rust's ownership system provides zero-cost memory
   management. Use Box for heap allocation, Rc/Arc for shared ownership, and
   avoid cloning large data structures. The compiler optimizes away unnecessary
   allocations.

2. From conv_rust_programming: Rust's key features include: memory safety
   without garbage collection, zero-cost abstractions, ownership system,
   powerful type system, and excellent concurrency support.

3. From conv_rust_programming: The borrow checker enforces Rust's ownership
   rules at compile time. It ensures that references don't outlive the data they
   point to and prevents data races by allowing either multiple immutable
   references or one mutable reference.

=== System Statistics ===

→ Querying database statistics... • Using FindAll() helper - simplified query
API

📊 Database Statistics: • Total conversations: 4 • Total messages stored: 14 •
All messages indexed for vector search ✓ • All messages indexed for text search
✓ • All messages queryable by metadata ✓

=== Step 8: Dynamic Search Configuration === Each conversation can have its own
search config...

💡 Conversations can store custom search configurations: • Search type: hybrid,
text, or vector • Relevance thresholds • Filter by tags or metadata •
Collection-specific settings • Per-conversation AI behavior

This enables context-aware search tuned to each conversation's needs!

=== Cleanup === ✓ Cleanup complete

=== 📚 Summary: What This Example Showed ===

🔧 ekoDB Native Capabilities Used: ✓ Functions with Embed operation (AI
integration) ✓ Hybrid Search (text + vector combined) ✓ Text Search (full-text
with stemming) ✓ Automatic embedding generation ✓ Cross-collection queries

🚀 New Client Helper Methods: • client.Embed(text, model) - Generate embeddings
• client.HybridSearch() - Semantic + keyword search • client.TextSearch() -
Full-text search • client.FindAll() - Query all documents

💡 Key Takeaways:

1. ekoDB handles AI Functions natively - no external services needed
2. One-line embedding generation with auto-cleanup
3. Hybrid search combines semantic understanding + keyword matching
4. Perfect for RAG: store, search, and retrieve context
5. All AI capabilities accessible through simple client methods

🎯 Build production RAG systems with ekoDB! → Set OPENAI_API_KEY in your ekoDB
server environment → Use these client helpers to make AI integration simple →
Scale to millions of documents with native indexing

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m [36mRunning
Kotlin RAG Example...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m === ekoDB RAG
Conversation System ===

This example shows how ekoDB can power a self-improving AI system that learns
from its own conversation history.

SLF4J(W): No SLF4J providers were found. SLF4J(W): Defaulting to no-operation
(NOP) logger implementation SLF4J(W): See
https://www.slf4j.org/codes.html#noProviders for further details. === Step 1:
Building Conversation History === Storing previous conversations with
embeddings...

→ Calling ekoDB embed() helper... • Using model: text-embedding-3-small • Text
length: 34 characters • Behind the scenes: Creating temp Function with Embed
operation ✓ Generated embedding: 1536 dimensions in 0.35s • Function
auto-cleaned up by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 169 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.223s • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 33
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.351s • Function auto-cleaned up by
client → Calling ekoDB embed() helper... • Using model: text-embedding-3-small •
Text length: 230 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.306s • Function
auto-cleaned up by client ✓ Stored Rust programming conversation (4 messages) →
Calling ekoDB embed() helper... • Using model: text-embedding-3-small • Text
length: 31 characters • Behind the scenes: Creating temp Function with Embed
operation ✓ Generated embedding: 1536 dimensions in 0.324s • Function
auto-cleaned up by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 217 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.293s • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 33
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.29s • Function auto-cleaned up by
client → Calling ekoDB embed() helper... • Using model: text-embedding-3-small •
Text length: 232 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.291s • Function
auto-cleaned up by client ✓ Stored database design conversation (4 messages) →
Calling ekoDB embed() helper... • Using model: text-embedding-3-small • Text
length: 36 characters • Behind the scenes: Creating temp Function with Embed
operation ✓ Generated embedding: 1536 dimensions in 0.503s • Function
auto-cleaned up by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 178 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.419s • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 37
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.336s • Function auto-cleaned up by
client → Calling ekoDB embed() helper... • Using model: text-embedding-3-small •
Text length: 213 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.296s • Function
auto-cleaned up by client ✓ Stored performance optimization conversation (4
messages)

=== Step 2: New User Question with Context Retrieval === User asks: "How do I
write memory-safe high-performance database code?"

=== Step 3: Searching Related Context === Using hybrid search to find relevant
messages from all conversations...

→ Generating embedding for user question... → Calling ekoDB embed() helper... •
Using model: text-embedding-3-small • Text length: 58 characters • Behind the
scenes: Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.329s • Function auto-cleaned up by client

→ Executing hybridSearch()... • Collection: rag_messages • Query text: "How do I
write memory-safe high-performance database code?" • Vector dimensions: 1536 •
Limit: 5 results • Search type: Semantic (vector) + Keyword (text) • Server
combines both scores for relevance ranking ✓ Search completed in 0.119s

✓ Found 5 related messages across all conversations:

1. [Score: 0.000] From conv_performance How can I optimize database queries?

2. [Score: 0.000] From conv_database_design Use NoSQL when you need: flexible
   schemas, horizontal scaling, high write throughput, or when working with
   unstructured data. SQL is better for complex queries, ACID transactions, and
   structured data with well-defined relationships.

3. [Score: 0.000] From conv_database_design When should I use NoSQL over SQL?

4. [Score: 0.000] From conv_database_design What is database normalization?

5. [Score: 0.000] From conv_database_design Database normalization is the
   process of organizing data to reduce redundancy and improve data integrity.
   It involves dividing large tables into smaller ones and defining
   relationships between them using foreign keys.

=== Step 4: Generating Context-Aware Response === ✓ Context prepared from search
results ✓ AI would use this context to generate comprehensive response

=== Step 5: Storing New Conversation === → Calling ekoDB embed() helper... •
Using model: text-embedding-3-small • Text length: 58 characters • Behind the
scenes: Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.698s • Function auto-cleaned up by client ✓ New conversation
stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search === Searching for messages about
'ownership' across ALL conversations...

→ Executing textSearch()... • Collection: rag_messages • Query: "ownership
system" • Limit: 3 results • Search method: Full-text with fuzzy matching &
stemming • No vector embeddings needed - pure keyword search ✓ Text search
completed in 0.055s

✓ Found 3 messages mentioning ownership:

1. From conv_performance: Rust's ownership system provides zero-cost memory
   management. Use Box for heap allocation, Rc/Arc for shared ownership, and
   avoid cloning large data structures. The compiler optimizes away unnecessary
   allocations.

2. From conv_rust_programming: Rust's key features include: memory safety
   without garbage collection, zero-cost abstractions, ownership system,
   powerful type system, and excellent concurrency support.

3. From conv_rust_programming: The borrow checker enforces Rust's ownership
   rules at compile time. It ensures that references don't outlive the data they
   point to and prevents data races by allowing either multiple immutable
   references or one mutable reference.

=== System Statistics ===

→ Querying database statistics... • Using findAllWithLimit() helper - simplified
query API

📊 Database Statistics: • Total conversations: 4 • Total messages stored: 13 •
All messages indexed for vector search ✓ • All messages indexed for text search
✓ • All messages queryable by metadata ✓

=== Step 8: Dynamic Search Configuration === Each conversation can have its own
search config...

💡 Conversations can store custom search configurations: • Search type: hybrid,
text, or vector • Relevance thresholds • Filter by tags or metadata •
Collection-specific settings • Per-conversation AI behavior

This enables context-aware search tuned to each conversation's needs!

=== Cleanup === ✓ Cleanup complete

=== 📚 Summary: What This Example Showed ===

🔧 ekoDB Native Capabilities Used: ✓ Functions with Embed operation (AI
integration) ✓ Hybrid Search (text + vector combined) ✓ Text Search (full-text
with stemming) ✓ Automatic embedding generation ✓ Cross-collection queries

🚀 New Client Helper Methods: • client.embed(text, model) - Generate embeddings
• client.hybridSearch() - Semantic + keyword search • client.textSearch() -
Full-text search • client.findAllWithLimit() - Query all documents

💡 Key Takeaways:

1. ekoDB handles AI Functions natively - no external services needed
2. One-line embedding generation with auto-cleanup
3. Hybrid search combines semantic understanding + keyword matching
4. Perfect for RAG: store, search, and retrieve context
5. All AI capabilities accessible through simple client methods

🎯 Build production RAG systems with ekoDB! → Set OPENAI_API_KEY in your ekoDB
server environment → Use these client helpers to make AI integration simple →
Scale to millions of documents with native indexing

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m ✅ [32mRAG
Examples Complete![0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m

[32mWhat you just saw across 5 languages:[0m ✓ Embeddings generated via ekoDB
Functions ✓ Hybrid search (semantic + keyword) ✓ Text search with stemming ✓
Cross-conversation context retrieval ✓ Simple client helpers wrapping powerful
AI

[36mMission: AI for All 🚀[0m - Making RAG accessible to everyone!

✅ [32mAll RAG examples complete! Output saved to test-examples-rag.md[0m
