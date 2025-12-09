make test-examples-rust 🧪 [36mRunning Rust examples (direct
HTTP/WebSocket)...[0m Finished `dev` profile [unoptimized + debuginfo] target(s)
in 0.11s Running `target/debug/examples/simple_crud` ✓ Authentication successful

=== Insert Document === Inserted: Object {"id":
String("Kzth1WDpz-FcgmladsSrv1byE8Lfk03INF0vys5Tr0NXc7yhRC3pfod6Wwn-DD6i8YpT27vdqHcBjT-S_SvnaQ")}

=== Find by ID === Found: Object {"active": Object {"type": String("Boolean"),
"value": Bool(true)}, "id":
String("Kzth1WDpz-FcgmladsSrv1byE8Lfk03INF0vys5Tr0NXc7yhRC3pfod6Wwn-DD6i8YpT27vdqHcBjT-S_SvnaQ"),
"name": Object {"type": String("String"), "value": String("Test Record")},
"value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query === Found documents: Array [Object {"active": Object
{"type": String("Boolean"), "value": Bool(true)}, "id":
String("Kzth1WDpz-FcgmladsSrv1byE8Lfk03INF0vys5Tr0NXc7yhRC3pfod6Wwn-DD6i8YpT27vdqHcBjT-S_SvnaQ"),
"name": Object {"type": String("String"), "value": String("Test Record")},
"value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document === Updated: Object {"active": Object {"type":
String("Boolean"), "value": Bool(true)}, "id":
String("Kzth1WDpz-FcgmladsSrv1byE8Lfk03INF0vys5Tr0NXc7yhRC3pfod6Wwn-DD6i8YpT27vdqHcBjT-S_SvnaQ"),
"name": Object {"type": String("String"), "value": String("Updated Record")},
"value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully Finished `dev` profile [unoptimized
+ debuginfo] target(s) in 0.09s Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data === ✓ Inserted test record:
"-StpvC0i8Gt6eyenL3QP9h3rW2r0I9qfrnNaba88DIxHpZPxfUglMnSlcwVMvT1ZWYpBf50k510Fiy7LaTQCaQ"

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"-StpvC0i8Gt6eyenL3QP9h3rW2r0I9qfrnNaba88DIxHpZPxfUglMnSlcwVMvT1ZWYpBf50k510Fiy7LaTQCaQ",
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
"ixEVvXIr0txswnrIQNnaQqnjFql6pwfXEiM1mgGWY50yz5cmmYA_8xWEIDUI7Th_D9LBSRwfg5TLPHdXqT40ow"

=== List Collections === Total collections: 4 Sample collections:
["demo_collection", "test_collection", "websocket_test", "batch_users"]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully Finished `dev`
profile [unoptimized + debuginfo] target(s) in 0.09s Running
`target/debug/examples/document_ttl` ✓ Authentication successful

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
"OGvjERbR05hstchFc_nffMYGHJpJmJluwEWYk0pfl4uWYV5CaG6atCedzApmffj37gGxRWbvkJaFtFrIuhuenA"

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: null

=== Query Documents === ✓ Found 1 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

✓ All document TTL operations completed successfully Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.11s Running
`target/debug/examples/websocket_ttl` ✓ Authentication successful

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
String("D4lY92RaZpJKPEbZp-KTNdLNCKI04w5-APYLgfHE2JaiL6niFV4n-u65NwEWzvm7menUVcMZhcy8dgQnCVbcIg")

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.09s Running
`target/debug/examples/transactions` ✓ Authentication successful

=== Setup: Creating Test Accounts === Created Alice: $1000 - ID:
uu3Jp1e27OxPUhCu-YRvSw3v2re5-rtT1GIRwTt-kNr1G_wvK39kLA-S-9ioTQnIEaHKaMv4QfyzcgWPnaiAig
Created Bob: $500 - ID:
-J-BNMR1s6p6rDTO7lVb6ij2XCRaO_lAdyRBLUT--aR7GEILrCO7tXPGiY0BjLMR44J_RaZ2FH7N3W1q52Ap8Q

=== Example 1: Begin Transaction === Transaction ID:
843d2818-7029-44cf-815e-492ebaad11f8

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: "Active" Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: {"type":"Integer","value":800} Bob:
{"type":"Integer","value":700}

=== Example 5: Rollback === New transaction:
29b16ba4-d118-4b6f-8d2f-2d1eaf51ef5e Updated Bob: $700 → $600 (in transaction) ✓
Transaction rolled back Bob after rollback: {"type":"Integer","value":700}

=== Cleanup === ✓ Deleted test accounts

✓ All transaction examples completed ✅ [32mRust direct examples complete![0m 🛠️
[36mBuilding client library...[0m cargo build -p ekodb_client Finished `dev`
profile [unoptimized + debuginfo] target(s) in 0.08s ✅ [32mClient build
complete![0m 🧪 [36mRunning Rust client library examples...[0m Finished `dev`
profile [unoptimized + debuginfo] target(s) in 0.10s Running
`target/debug/examples/client_simple_crud` ✓ Client created (token exchange
happens automatically)

=== Insert Document === Inserted: Record { fields: {"id":
String("3aA3_utKWW8plZ3WHgJccIEMqQ7wBegHRGeN31JwnhsaTnNcgOfK1g6OkLXenRCIZZKxrKdcKRSWmQJvdkA_NQ")}
}

=== Find by ID === Found: Record { fields: {"id":
String("3aA3_utKWW8plZ3WHgJccIEMqQ7wBegHRGeN31JwnhsaTnNcgOfK1g6OkLXenRCIZZKxrKdcKRSWmQJvdkA_NQ"),
"name": Object({"type": String("String"), "value": String("Test Record")}),
"value": Object({"value": Integer(42), "type": String("Integer")}), "active":
Object({"value": Boolean(true), "type": String("Boolean")})} }

=== Find with Query === Found documents: [Record { fields: {"id":
String("3aA3_utKWW8plZ3WHgJccIEMqQ7wBegHRGeN31JwnhsaTnNcgOfK1g6OkLXenRCIZZKxrKdcKRSWmQJvdkA_NQ"),
"value": Object({"type": String("Integer"), "value": Integer(42)}), "name":
Object({"type": String("String"), "value": String("Test Record")}), "active":
Object({"type": String("Boolean"), "value": Boolean(true)})} }]

=== Update Document === Updated: Record { fields: {"value": Object({"value":
Integer(100), "type": String("Integer")}), "id":
String("3aA3_utKWW8plZ3WHgJccIEMqQ7wBegHRGeN31JwnhsaTnNcgOfK1g6OkLXenRCIZZKxrKdcKRSWmQJvdkA_NQ"),
"active": Object({"type": String("Boolean"), "value": Boolean(true)}), "name":
Object({"type": String("String"), "value": String("Updated Record")})} }

=== Delete Document === Deleted document

=== Cleanup === ✓ Deleted collection

✓ All CRUD operations completed successfully Finished `dev` profile [unoptimized
+ debuginfo] target(s) in 0.09s Running
`target/debug/examples/client_batch_operations` ✓ Client created

=== Batch Insert (via multiple inserts) === ✓ Inserted 5 records ✓ Verified:
Found 5 total records in collection

=== Update Records === ✓ Updated 3 records

=== Delete Records === ✓ Deleted 3 records

=== Cleanup === ✓ Deleted collection

✓ All batch operations completed successfully Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.08s Running
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
debuginfo] target(s) in 0.08s Running
`target/debug/examples/client_collection_management` ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
"bVbD08FkL4FQRLyrZc5xr8fHbhgkLoTf9ME0d9V0wZeuZbOUVm-APBeN_dUrfVCw1UzsP5d_BSVZooeuwgSI7Q"

=== List Collections === Total collections: 7 Sample collections:
["test_collection", "ws_ttl_test", "ttl_cache", "websocket_test", "batch_users"]

=== Count Documents === Document count: 1

=== Check Collection Exists === Collection exists: true

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully Finished `dev`
profile [unoptimized + debuginfo] target(s) in 0.09s Running
`target/debug/examples/client_document_ttl` ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
"ryzlrojvJGnniFLQQyYZcTEhQ_Yi5I4c2af9RKsgOaBJXWhfbTMmVV_PAk1zPQZLhqXAllG4i5r3PL6MKvq_DQ"

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
Some(String("t31K0qBTe4HrrFIO5ry5uhrEFddfDHu3DkCLGBUny9bzdpoGe7nRqJFP2G7aZE4cYoTTCeGHgrTlR5MFdQNg3Q"))

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
Running `target/debug/examples/client_simple_websocket` ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
nvh_umHoqI8oYjk3LH0qGtewWuu5LveCGrWEftKGeBOxecay6Op5kY-vd1hwYX1qXxVEkjgId0vpNlf4mWM4Rg

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully Finished `dev` profile [unoptimized +
debuginfo] target(s) in 0.09s Running
`target/debug/examples/client_websocket_ttl` ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
Some(String("rt_VJBsxfxck_tX-BIKkzv3FSXASsL9tV_z8u2bFs3t3z804OI0gAdtEArwaIW9nENDFyVu5sa-PE6m8Luq4rw"))

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
Running `target/debug/examples/client_query_builder` === ekoDB Query Builder
Example ===

=== Inserting Sample Data === ✓ Inserted 5 users

=== Simple Equality Query === ✓ Found 3 active users

- Some(Object({"value": String("David"), "type": String("String")}))
- Some(Object({"type": String("String"), "value": String("Bob")}))
- Some(Object({"type": String("String"), "value": String("Alice")}))

=== Range Query (age >= 28 AND age < 35) === ✓ Found 3 users in age range

- Some(Object({"value": String("David"), "type": String("String")}))
- Some(Object({"type": String("String"), "value": String("Eve")}))
- Some(Object({"type": String("String"), "value": String("Bob")}))

=== IN Operator === ✓ Found 4 users with status active or pending

- Some(Object({"type": String("String"), "value": String("David")}))
- Some(Object({"value": String("Eve"), "type": String("String")}))
- Some(Object({"value": String("Bob"), "type": String("String")}))
- Some(Object({"value": String("Alice"), "type": String("String")}))

=== NOT IN Operator === ✓ Found 4 users not inactive

=== String Pattern Matching === ✓ Found 5 users with @example.com email

=== Regex Query === ✓ Found 0 users with names starting with A-C

=== Complex Query (active AND age >= 28 AND score > 1500) === ✓ Found 2 users
matching all conditions

- Some(Object({"value": String("David"), "type": String("String")}))
- Some(Object({"value": String("Bob"), "type": String("String")}))

=== OR Query === ✓ Found 2 users with age < 28 OR age > 32

- Some(Object({"value": String("Alice"), "type": String("String")}))
- Some(Object({"type": String("String"), "value": String("Charlie")}))

=== Sorted Query (by score descending) === ✓ Top 3 users by score:

1. Some(Object({"value": String("Bob"), "type": String("String")}))
2. Some(Object({"value": String("David"), "type": String("String")}))
3. Some(Object({"type": String("String"), "value": String("Alice")}))

=== Pagination (page 2, size 2) === ✓ Page 2 results:

- Some(Object({"value": String("Charlie"), "type": String("String")}))
- Some(Object({"type": String("String"), "value": String("David")}))

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
   ["title", "title.value", "description.value", "description"]
2. Score: 2.0000 Title: Some(String("Python for Data Science")) Matched:
   ["description", "description.value"]
3. Score: 1.0000 Title: Some(String("Rust Programming")) Matched:
   ["description.value", "description"]
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
Some(String("LNDqQcD6J3p_4KxXnAIHRlzKS3bmL9GOa-SaIeFFjEwT38YjRS9zUju8zYBtewuUs3WV05JBZ5MKeGB_RmTzoQ"))
✓ Inserted user 2:
Some(String("Fg2Ms71vce50FMtvCILvD-4SsHlisbYshbqpJCoPtJO1rex6kczQ8VJXgao6g0j1xo-Qw9aX3IZihH1xniyHWA"))

=== Getting Schema === ✓ Schema for schema_client_rust:

- age: Integer
- status: String
- email: String (required)
- title: String (required)

=== Listing Collections === ✓ Total collections: 7 Sample: ["test_collection",
"schema_client_rust", "ws_ttl_test", "ttl_cache", "websocket_test"]

=== Cleanup === ✓ Deleted collection

✓ All schema management operations completed successfully Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.09s Running
`target/debug/examples/client_joins` === ekoDB Rust Client - Join Operations
Example ===

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

✓ Join operations example completed successfully Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.09s Running
`target/debug/examples/client_chat_basic` === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
YkiDVUICmI0KGpwjWBz6CkhCmH570quCFqsURqHvH-deVCrMTx6Hp0qv3b3MepmPdkUcfSUdnEOABuONpyYBKQ

=== Sending Chat Message === Message ID:
6ywuiQwGCRIwEw8VtaENcL2f4LA5WmGk3WVhcY52ESVwfhIl4bigPQWMhcD1LoNVEBe0UyV1kZy3z-L6qdUCiQ

=== AI Response === Response 1: I'm sorry, but the provided context does not
contain any information about ekoDB and its features. Could you provide more
details or context?

=== Context Used (3 snippets) ===

Snippet 1: Collection: client_chat_basic_rust Score: 0.6333 Matched Fields:
["category", "title", "content"] Record: Object {"category": Object {"type":
String("String"), "value": String("features")}, "content": Object {"type":
String("String"), "value": String("The chat feature allows you to query your
database using natural language and get AI-powered responses with relevant
context.")}, "id":
String("SRHmQkU0pFwWh7Kdi435sKxSvmsJmJHYQITdWFts6OENBEwZz180iXKvca4h5Um_zzMPHahckFYbLaryrW-Abw"),
"title": Object {"type": String("String"), "value": String("AI Chat
Integration")}}

Snippet 2: Collection: client_chat_basic_rust Score: 0.5222 Matched Fields:
["content", "category", "title"] Record: Object {"category": Object {"type":
String("String"), "value": String("features")}, "content": Object {"type":
String("String"), "value": String("ekoDB supports full-text search, vector
search, and hybrid search with automatic context retrieval.")}, "id":
String("MTJuq6wjEcYTYGvV9-39UDCm-Wmp24XqJ_DpFKZjTqQCAN6FGvYF061WJtMtKkc-E1tu3yBtM8Lhvlrvw9ELng"),
"title": Object {"type": String("String"), "value": String("Search Features")}}

Snippet 3: Collection: client_chat_basic_rust Score: 0.5222 Matched Fields:
["content", "title"] Record: Object {"category": Object {"type":
String("String"), "value": String("documentation")}, "content": Object {"type":
String("String"), "value": String("ekoDB is a high-performance database with
intelligent caching, real-time capabilities, and AI integration.")}, "id":
String("-zQUIEi5zfadPRXd4al4lNs0_heBZcTMhHxfpMIFppRh0Z4VXEx0GLMltQKBP4m2y_xe8bG8J411ZLA7s_phhw"),
"title": Object {"type": String("String"), "value": String("Introduction to
ekoDB")}}

Execution Time: 1617ms

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully Finished `dev` profile [unoptimized + debuginfo]
target(s) in 0.09s Running `target/debug/examples/client_chat_sessions` ===
ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
7BDD-zJBVBWMsH1TZJ9mpV8ZjmliM9Il_h-MDHYIRwB09rF5oK14CqlOiemb2KwvAOhtymFAKWv5RYXVLn9LGg
=== Sending Messages === ✓ Message 1 sent Response: I'm sorry, but the context
provided does not contain specific information about any available products.

✓ Message 2 sent Response: I'm sorry, but there's no information provided about
the price of any product in the context given. Could you please specify which
product's price you're asking about?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
8-ETwzylsTed_4RwJaoCkuHWQEckgc0FUDKBCC3mk5D5isoC2O3RPn398EqL7iTHi5azx7RcXqzhl7AaHjFscQ
Parent:
7BDD-zJBVBWMsH1TZJ9mpV8ZjmliM9Il_h-MDHYIRwB09rF5oK14CqlOiemb2KwvAOhtymFAKWv5RYXVLn9LGg

=== Listing Sessions === ✓ Found 3 sessions Session 1:
8-ETwzylsTed_4RwJaoCkuHWQEckgc0FUDKBCC3mk5D5isoC2O3RPn398EqL7iTHi5azx7RcXqzhl7AaHjFscQ
(Untitled) Session 2:
7BDD-zJBVBWMsH1TZJ9mpV8ZjmliM9Il_h-MDHYIRwB09rF5oK14CqlOiemb2KwvAOhtymFAKWv5RYXVLn9LGg
(Untitled) Session 3:
YkiDVUICmI0KGpwjWBz6CkhCmH570quCFqsURqHvH-deVCrMTx6Hp0qv3b3MepmPdkUcfSUdnEOABuONpyYBKQ
(Untitled)

=== Getting Session Details === ✓ Session details retrieved Messages: 4

=== Deleting Branch Session === ✓ Deleted branch session:
8-ETwzylsTed_4RwJaoCkuHWQEckgc0FUDKBCC3mk5D5isoC2O3RPn398EqL7iTHi5azx7RcXqzhl7AaHjFscQ

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully Finished `dev`
profile [unoptimized + debuginfo] target(s) in 0.09s Running
`target/debug/examples/client_chat_advanced` === ekoDB Advanced Chat Features
Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
1ubai-hidVvVetd7OtAkKdNgex-ATrOR_xKL3HQNNK7D5gR2E8S24LfyeVOgHk1j6KuKlAEDy6YE2P62fUyRQg

=== Sending Initial Message === ✓ Message sent Response: Apologies for the
inconvenience, but the provided context does not contain specific information
about available products. Could you please provide more details?

✓ Second message sent === Feature 1: Regenerate AI Response === ✓ Message
regenerated New response: Answer: I'm sorry, but the context provided does not
include information about the price of ekoDB. You may need to check the official
website or contact the seller for the most accurate and current price.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
B0IRmVduPyhJcej9WW3Kz3ykpu3orY3VUi_OYpThxcXTAAnvs_tOFH5m72d4njdGdiWzc0kZ-wkLVPly6-AHOg
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
bGF-Dq1Ov94k8tmymzxRG1xbAkktQdXSh-48jaiQQ36-Bd1lIiIOosRSyFC4au1Fazn6ddf5YWRz4Sk3xwPwpQ
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved:
cb29n8NZ_Uq1o1WDI2ppV14cZyGB53mfGv6xVtW5jmiX3Xtr5ZbpfVWhdcJffl39Ihh5Kpy2uQ-pzQtmZZ03sw
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved:
q0QawUKP_HrtmGgPC6_znCtXn6CG0kb6oAIZHHpZqfgVPNc1hbwj4rKaFqiIZTaNleFg6catmB9x6wpdUaEt_g
📊 Statistics: 2 groups

📝 Example 4: Script Management

📋 Total scripts: 3 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed! ✅ [32mRust client examples complete![0m
