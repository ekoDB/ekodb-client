make test-examples 🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s Running
`target/debug/examples/simple_crud` ✓ Authentication successful

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

✅ All examples completed! ✅ [32mRust client examples complete![0m ✅ [32mAll
Rust integration tests complete![0m 🧪 [36mRunning Python examples (direct
HTTP/WebSocket)...[0m ✓ Authentication successful

=== Insert Document === Inserted: {'id':
'rfImyx0Q4L30WQ-kPr9UhtQu8jZxXLQpiSt233mQHm5P1hwFeTrh_MPZ7vqfkD9tbLZZ3sILPgTL2Wis9UYLtQ'}

=== Find by ID === Found: {'active': {'value': True, 'type': 'Boolean'}, 'id':
'rfImyx0Q4L30WQ-kPr9UhtQu8jZxXLQpiSt233mQHm5P1hwFeTrh_MPZ7vqfkD9tbLZZ3sILPgTL2Wis9UYLtQ',
'name': {'type': 'String', 'value': 'Test Record'}, 'value': {'value': 42,
'type': 'Integer'}}

=== Find with Query === Found documents: 1

=== Update Document === Updated: {'name': {'type': 'String', 'value': 'Updated
Record'}, 'id':
'rfImyx0Q4L30WQ-kPr9UhtQu8jZxXLQpiSt233mQHm5P1hwFeTrh_MPZ7vqfkD9tbLZZ3sILPgTL2Wis9UYLtQ',
'value': {'type': 'Integer', 'value': 100}, 'active': {'value': True, 'type':
'Boolean'}}

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully ✓ Authentication successful

=== Inserting Test Data === ✓ Inserted test record:
7mvDvjLu-3eoQZwjlZon8ia6SymRSNNQo0VlDw_zXmr7rkofCdE4da0lfkWdQKmCUTscj8culXnsEWFRpq3cCw

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"-StpvC0i8Gt6eyenL3QP9h3rW2r0I9qfrnNaba88DIxHpZPxfUglMnSlcwVMvT1ZWYpBf50k510Fiy7LaTQCaQ",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"7mvDvjLu-3eoQZwjlZon8ia6SymRSNNQo0VlDw_zXmr7rkofCdE4da0lfkWdQKmCUTscj8culXnsEWFRpq3cCw",
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
cache:product:2: {'name': 'Product 2', 'price': 39.989999999999995}
cache:product:3: {'name': 'Product 3', 'price': 49.989999999999995}

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Authentication successful

=== Create Collection (via insert) === Collection created with first record:
OC2B1SQkhxB_xDPI7p6I5H-ezEpLAdT9cTBS41M5_STySOdSz5DNCb1eL6pyP5vruBMLV4gsceH-gH9WXwDxsw

=== List Collections === Total collections: 11 Sample collections: ['users',
'demo_collection', 'test_collection', 'ws_ttl_test', 'ttl_cache']

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: False

✓ All collection management operations completed successfully ✓ Authentication
successful

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
cnEjeWZi6R6hvDC-H782GwtR5YK_nh0P8rjoPXsNSe3vLcFexb0fkzDF-crj-vRUc6DjK-t1Sggsz0LpcHGy4Q

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully ✓ Authentication successful

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
CFNm2sX4w_8UkQ0i_HSJtOzEke-1BpY8B7yUW6s374NGFQJm9YDQVLtLCoxJhsSy36Z_QyzO703DMw5E64cBfw

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 2 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully ✓ Authentication successful

📋 Getting original configuration... Original durable_operations: False

============================================================ 🔥 TEST 1: Original
Config (durable=False)
============================================================

=== Setup: Creating Test Accounts === Created Alice: $1000 - ID:
b8cAtsZTYmGb3-TFcTkKANIt0ErZTC6HAsDNDzIov43yj07E33YCNHrJiTABuJcPzt2u2p2Ux74IsCoyDrEvOA
Created Bob: $500 - ID:
b2L7d7Nyp3TpZqk0vPUiJZJ4X40eu5IBJ22Wh5nreFnQqmKjQAU5yjUYK6Gy5cT806PveWPsHWeStS_k6enJ-A

=== Example 1: Begin Transaction === Transaction ID:
122b3d9a-645f-4eb7-9bdc-f095a4eca2a6

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: Active Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: ${'value': 800, 'type': 'Integer'} Bob: ${'type':
'Integer', 'value': 700}

=== Example 5: Rollback === New transaction:
8e42790a-0da9-4e99-90da-d27366a5fdff Updated Bob: $700 → $600 (in transaction) ✓
Transaction rolled back Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup === ✓ All transaction examples completed === Setup: Creating Test
Accounts === Created Alice: $1000 - ID:
i14q4WzI6M1xSo1dPIXwIRhqOv2kt9GURwLKCrOAnBocuecACFtYUelgVM91YAUB2DMFZJEN9uY-FJL3DBBqWA
Created Bob: $500 - ID:
gTsBA9gtf3EfB3FSohw_dzMlRIgt8Zcuy_9Sn8Z-cjx7ouqD4nAzrLtdk1D8ZLBIVyk2DeZ2lGTcdBbL3BnKIg

=== Example 1: Begin Transaction === Transaction ID:
f91b19f2-a9eb-4a65-a608-1b3349c72282

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: ${'value': 800, 'type': 'Integer'} Bob: ${'value':
700, 'type': 'Integer'}

=== Cleanup === ✓ Deleted test accounts

🔄 Switching to NON-DURABLE mode... ✓ Config updated: durable_operations=false

============================================================ 🔥 TEST 2:
Non-Durable Mode (durable=false)
============================================================

=== Setup: Creating Test Accounts === Created Alice: $1000 - ID:
ZkWU-CuJt69FzCUr7Cz277PyRTXN7ZIpRMhkeyzAfPEzi005V2r6olZN9YH3gp_x-Crr4kfyzxCGqG72LXTTUg
Created Bob: $500 - ID:
64PvEGrc-YSB8KNBDSA226TRJg2OEmwtqmmT7iIxpkpMfrPqyIBGJ2EyhtjVlLlI7RojRo1R6kZs_oIbVduvXw

=== Example 1: Begin Transaction === Transaction ID:
1e684921-985d-40a8-96e9-b073fdf20dd4

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: Active Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: ${'value': 800, 'type': 'Integer'} Bob: ${'value':
700, 'type': 'Integer'}

=== Example 5: Rollback === New transaction:
4e65ffba-9915-4ccc-b6e1-bf83c4799b24 Updated Bob: $700 → $600 (in transaction) ✓
Transaction rolled back Bob after rollback: ${'value': 700, 'type': 'Integer'}

=== Cleanup === ✓ All transaction examples completed === Setup: Creating Test
Accounts === Created Alice: $1000 - ID:
rFu44nbmAUEcTSM6cTzr1-30AdFOtSTHZOjRPnwdk0msTlPBEXFRrq0UrcIj30T_u8FT9gV3Sd65RFSVSZ45OQ
Created Bob: $500 - ID:
Nof5flxJ9tNedj1hVURTzt8H9OMRjeSS9yuhcAfBYEKCyXmFd7l7FYdZC2nuDFYjOuZMrEKrQ5pf2Jp-FST_jw

=== Example 1: Begin Transaction === Transaction ID:
8e527684-e4e0-40a9-8bf2-0e4683beb7a4

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
JfQqyQPxBTh4qnjABL*fvT6LdWZI7N6Ncmgfq0H2gOPO7KIH2zCgakEnyGGTGK8j9kl6n9f2NrzGb7lEL5JsaQ
Created Bob: $500 - ID:
qgqxZGc20yJLt3kt-\_rML6yA8jMBK8w8qMddvNBPw_TQ4tQyJa0*-vOoJsAKPMpa-70S3M5htl_edd-KKOzGgg

=== Example 1: Begin Transaction === Transaction ID:
71c80dad-0c47-4f81-ac9f-d9341687f536

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: Active Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: ${'value': 800, 'type': 'Integer'} Bob: ${'value':
700, 'type': 'Integer'}

=== Example 5: Rollback === New transaction:
700c754e-e2eb-4683-b5f0-b2e898dbd849 Updated Bob: $700 → $600 (in transaction) ✓
Transaction rolled back Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup === ✓ All transaction examples completed === Setup: Creating Test
Accounts === Created Alice: $1000 - ID:
XTrIsqzLWObvZLdyT4xaVDW13BTdQvaszAfTWAR-76LGjQFwUwJMqQ1Ca4jtW2wza5AAh0xTvQ-tDBK_ecno9A
Created Bob: $500 - ID:
14v1NOmvBZcSoIX4AeGLsP1NAekGfUoYyg9E-9uQziGRSgj3UMGMYqB6oyHK0Tk0A5CUcquMrTZSU8zMnmAdfQ

=== Example 1: Begin Transaction === Transaction ID:
4295f880-3549-43c8-8bb6-24c3b18d81db

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: ${'type': 'Integer', 'value': 800} Bob: ${'value':
700, 'type': 'Integer'}

=== Cleanup === ✓ Deleted test accounts

🔄 Restoring original configuration... ✓ Config restored:
durable_operations=False

============================================================ ✅ ALL TESTS
PASSED - Transactions successful
============================================================

[34m ╔════════════════════════════════════════╗[0m [34m║ ekoDB Python Examples
Test Suite ║[0m [34m╚════════════════════════════════════════╝[0m [34m ===
Checking Server Connection ===[0m [32m✓ Server is ready[0m [34m === Getting
Authentication Token ===[0m [32m✓ Authentication successful[0m [33m === Running
8 Examples ===[0m [34m === Running simple_crud.py ===[0m [32m✓ simple_crud.py
completed successfully[0m [34m === Running simple_websocket.py ===[0m [32m✓
simple_websocket.py completed successfully[0m [34m === Running
batch_operations.py ===[0m [32m✓ batch_operations.py completed successfully[0m
[34m === Running kv_operations.py ===[0m [32m✓ kv_operations.py completed
successfully[0m [34m === Running collection_management.py ===[0m [32m✓
collection_management.py completed successfully[0m [34m === Running
document_ttl.py ===[0m [32m✓ document_ttl.py completed successfully[0m [34m ===
Running websocket_ttl.py ===[0m [32m✓ websocket_ttl.py completed successfully[0m
[34m === Running transactions.py ===[0m [32m✓ transactions.py completed
successfully[0m [34m ╔════════════════════════════════════════╗[0m [34m║ Test
Summary ║[0m [34m╚════════════════════════════════════════╝[0m [34mTotal: 8[0m
[32mPassed: 8[0m [32mFailed: 0[0m ✅ [32mPython direct examples complete![0m 🐍
[36mBuilding Python client package...[0m 🍹 Building a mixed python/rust project
🔗 Found pyo3 bindings with abi3 support 🐍 Not using a specific python
interpreter 💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by
default Finished `release` profile [optimized] target(s) in 0.08s 📦 Built wheel
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
'DuHXjjvqL-VtNeslR1ghBDgEaPHUQqADYHDJuI5a9agIwBVAUG-fk2f2NAQnz4dY7Ce5s93XMG3YBgxhpQi8zA'}

=== Find by ID === Found: {'id':
'DuHXjjvqL-VtNeslR1ghBDgEaPHUQqADYHDJuI5a9agIwBVAUG-fk2f2NAQnz4dY7Ce5s93XMG3YBgxhpQi8zA',
'name': {'type': 'String', 'value': 'Test Record'}, 'active': {'value': True,
'type': 'Boolean'}, 'value': {'type': 'Integer', 'value': 42}}

=== Find with Query === Found documents: 1

=== Update Document === Updated: {'id':
'DuHXjjvqL-VtNeslR1ghBDgEaPHUQqADYHDJuI5a9agIwBVAUG-fk2f2NAQnz4dY7Ce5s93XMG3YBgxhpQi8zA',
'value': {'type': 'Integer', 'value': 100}, 'active': {'value': True, 'type':
'Boolean'}, 'name': {'type': 'String', 'value': 'Updated Record'}}

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
"EkqwpueNc_UgUWKmPi3uNQR2Htjupj3yu7LGbxkVUm1rml2f0OpSxjZVCAzA0IHRqEuQe_FZdf71VAza5kWigA"

=== List Collections === Total collections: 11 Sample collections: ['users',
'test_collection', 'ws_ttl_test', 'ttl_cache', 'websocket_test']

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: False

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
12KXBZYllJpfsRMGjw6esftpX453eOPZh-7Nf28DetBmgeHEckG7VOT9i8eFmwdThCopividxi0KaO9jNFpwWQ

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
9T9n_OEibzvPaaBLAZ1hx0LyTN1SKJavksbMT4CdXj8w5RZvfXJCJ-up1ceE976sXOEN3ZRlR1GAm0UeNMftpg

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
8P1yBpYt_f0ek3k-7leIy8smD9kQ3NEMb3GStCRQRwfGp1Hh7gzL9VMcOJob8wFyP77JIt4O4HAeHjAENU0KxQ

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
pWroJEYL5-lwaPFE5PYG_Oiz5wot7bsOLrWpKmM8FEAVHFalMfoYJOKs_QZyZ627lRn4iEAGUwzW6yoYvSHlsA

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
1. Score: 25.740, Matched: email.value, email, name, name.value
1. Score: 12.540, Matched: name.value, name

1. Fuzzy search (typo tolerance): Found 4 results with fuzzy matching
1. Score: 26.400, Matched: title, bio, title.value, bio.value
1. Score: 26.400, Matched: title, title.value, bio, bio.value
1. Score: 26.400, Matched: bio, title.value, title, bio.value
1. Score: 26.400, Matched: title.value, bio.value, title, bio

1. Search with field weights: Found 4 results with weighted fields
1. Score: 39.600, Matched: bio.value, title, title.value, bio
1. Score: 39.600, Matched: bio, bio.value, title, title.value
1. Score: 39.600, Matched: title, title.value, bio, bio.value
1. Score: 39.600, Matched: title.value, title, bio.value, bio

1. Search with minimum score threshold: Found 2 results with score >= 0.3
1. Score: 13.200, Matched: bio.value, bio
1. Score: 13.200, Matched: bio, bio.value

1. Search with stemming and exact match boosting: Found 2 results (matches: run,
   running, runs)
1. Score: 13.200, Matched: bio, bio.value
1. Score: 13.200, Matched: bio.value, bio

1. Vector search (semantic search): Found 3 semantically similar documents
1. Score: 0.764
1. Score: 0.764
1. Score: 0.745

1. Hybrid search (text + vector): Found 3 results using hybrid search (text +
   vector)
1. Score: 1.722, Matched: title, title.value, content, content.value
1. Score: 1.135, Matched: content.value, title.value, content, title
1. Score: 0.535, Matched:

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

- Alice Johnson: Engineering
- Bob Smith: Sales

2. Join with filtering: Found 1 users in Engineering:

- Alice Johnson: Building A

3. Join with user profiles: Found 2 users with profile data:

- Alice Johnson: Senior Software Engineer
- Bob Smith: Sales Manager

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
\_ZfZ-i61nrfaPM1Ql5QjKywbDAUwP0ApPNSm8aSneEByuytGCny8Dec49cx3-gRpg03AvoP_26-ID7_KtQfCLw

=== Sending Chat Message === Message ID:
IGBbxIDtjFOpl_BCM69QWCgqE2hqWF1lpMAcKOXa_44c59gVemZjDeYnU3DWPdk6Gzv0rbiFp2lFB8cp8KafQQ

=== AI Response === I'm sorry, but the provided context does not contain
specific information about available products or their prices. Could you please
provide additional details or context?

Execution Time: 2026ms

=== Token Usage === Prompt tokens: 317 Completion tokens: 29 Total tokens: 346

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
17Zok24kRKL0JCbQzJ2miG8bWcWIQ3fqG7mD3aIpuLjAMu0V_Qv5aoQvN1lmqhrl44_BhaVGQWOZHxhvfOpnRA

=== Sending Messages === ✓ Message 1 sent Response: I'm sorry, but the provided
context does not include specific information about any available products.
Could you please provide more details or a different context?

✓ Message 2 sent Response: Apologies, but the context provided does not include
information about the price of a product. Could you please specify which product
you're referring to so I can provide the accurate price?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
Vid2OjfaNug9XN5oIOmJyAO9yhSaIX_29Bhur4OjfwKD41BeqzhdkXGsLzioIBMlbfEVi8X5rx1Hqdm4aH3tiw
Parent:
17Zok24kRKL0JCbQzJ2miG8bWcWIQ3fqG7mD3aIpuLjAMu0V_Qv5aoQvN1lmqhrl44_BhaVGQWOZHxhvfOpnRA

=== Listing Sessions === ✓ Found 6 sessions Session 1:
Vid2OjfaNug9XN5oIOmJyAO9yhSaIX_29Bhur4OjfwKD41BeqzhdkXGsLzioIBMlbfEVi8X5rx1Hqdm4aH3tiw
(Untitled) Session 2:
17Zok24kRKL0JCbQzJ2miG8bWcWIQ3fqG7mD3aIpuLjAMu0V_Qv5aoQvN1lmqhrl44_BhaVGQWOZHxhvfOpnRA
(Untitled) Session 3:
\_ZfZ-i61nrfaPM1Ql5QjKywbDAUwP0ApPNSm8aSneEByuytGCny8Dec49cx3-gRpg03AvoP_26-ID7_KtQfCLw
(Untitled) Session 4:
B0IRmVduPyhJcej9WW3Kz3ykpu3orY3VUi_OYpThxcXTAAnvs_tOFH5m72d4njdGdiWzc0kZ-wkLVPly6-AHOg
(Untitled) Session 5:
7BDD-zJBVBWMsH1TZJ9mpV8ZjmliM9Il_h-MDHYIRwB09rF5oK14CqlOiemb2KwvAOhtymFAKWv5RYXVLn9LGg
(Untitled) Session 6:
YkiDVUICmI0KGpwjWBz6CkhCmH570quCFqsURqHvH-deVCrMTx6Hp0qv3b3MepmPdkUcfSUdnEOABuONpyYBKQ
(Untitled)

=== Deleting Branch Session === ✓ Deleted branch session:
Vid2OjfaNug9XN5oIOmJyAO9yhSaIX_29Bhur4OjfwKD41BeqzhdkXGsLzioIBMlbfEVi8X5rx1Hqdm4aH3tiw

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
WCg1KYLRUW1M91O-oLeLJtZan_OaKNtWXG_2g_jDnTZ1ja3C3d4Mo2jflrPF7BEWRt1yzuK_CVk5qredt3oFvQ

=== Sending Initial Message === ✓ Message sent Response: I'm sorry, but the
context provided does not include any specific product information. Could you
please provide more details or context?

✓ Second message sent

Debug: Found 4 messages Debug: First message keys: dict_keys(['id',
'created_at', 'content', 'role', 'token_usage', 'updated_at', 'chat_id']) Debug:
First message role: {'type': 'String', 'value': 'assistant'} === Feature 1:
Regenerate AI Response === ✓ Message regenerated New response: Answer: I'm
sorry, but the context provided does not include information on the price of
ekoDB.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
48IeSr-b5OvgoPOo5UeudSGqoPBWM9t1iupVu29o83S7xz239syKzCaUsRhQY4gO-CBRPK3vUG5Fu8m694PnmA
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
tW1o_iiQQ-5cg9d0KX_a_vvsxJUWiWIH1Zjah5uRZTGaVU9j2xWkObIIGATHK4OtuNskUMuDHte-VvtFXyrE8Q
📊 Found 20 active users

📝 Example 2: Parameterized Script

✅ Script saved:
fNZKoLQgLgZfLC8t2quBrgRnibZlHStNWZYsMVscp7ILNVyWzYz3xyO-AAU0Hs6Mpeoea9FQmwN5Fjm2ZdRisw
📊 Found 20 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved:
E1NARJ_03PorHZZco1JygYza-8f113bMd_12qtAIf2QB6mZIfFCMY07tk4EFaZ5HTNsg4zswU8fjI74M-LkjTQ
📊 Statistics: 2 groups {'avg_score': 60.0, 'count': 10, 'status': 'active'}

{'avg_score': 50.0, 'count': 10, 'status': 'inactive'}

📝 Example 4: Script Management

📋 Total scripts: 5 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed! 🚀 ekoDB Python Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data... ✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved:
vJb2ZQuyPEYr76krW27pULXPvySogU160hvFSiA1bkJSmQMivDun6zmNI0AB9oeR9dJdrzgLssqBxRuMiigU7g
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
Connection ===[0m [32m✓ Server is ready[0m [33m === Running 8 Examples ===[0m
[34m === Running simple_crud.go ===[0m ✓ Authentication successful

=== Insert Document === Inserted:
map[id:GvgANMNQNZm9XybNJ8lwYGTv3nR1mAXCdhV8IJPtcu-a-u2yES-hamumfm0C_yXP4t8h71IraZ7EewMUmm5nXQ]

=== Find by ID === Found: map[active:map[type:Boolean value:true]
id:GvgANMNQNZm9XybNJ8lwYGTv3nR1mAXCdhV8IJPtcu-a-u2yES-hamumfm0C_yXP4t8h71IraZ7EewMUmm5nXQ
name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query === Found 1 documents

=== Update Document === Updated: map[active:map[type:Boolean value:true]
id:GvgANMNQNZm9XybNJ8lwYGTv3nR1mAXCdhV8IJPtcu-a-u2yES-hamumfm0C_yXP4t8h71IraZ7EewMUmm5nXQ
name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully [32m✓ simple_crud.go completed
successfully[0m [34m === Running simple_websocket.go ===[0m ✓ Authentication
successful

=== Inserting Test Data === ✓ Inserted test record:
Xmfr_Z2upSSFQAcp-owmbvissITLMYTXOLA0QCeJe1vZiQq6v5nK8U39FZycJMcGfHfeCzslbFfsriw4sfw7VQ

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"Xmfr_Z2upSSFQAcp-owmbvissITLMYTXOLA0QCeJe1vZiQq6v5nK8U39FZycJMcGfHfeCzslbFfsriw4sfw7VQ",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"-StpvC0i8Gt6eyenL3QP9h3rW2r0I9qfrnNaba88DIxHpZPxfUglMnSlcwVMvT1ZWYpBf50k510Fiy7LaTQCaQ",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"7mvDvjLu-3eoQZwjlZon8ia6SymRSNNQo0VlDw_zXmr7rkofCdE4da0lfkWdQKmCUTscj8culXnsEWFRpq3cCw",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } } ] }, "type": "Success" } ✓ Retrieved 3
record(s) via WebSocket

✓ WebSocket example completed successfully [32m✓ simple_websocket.go completed
successfully[0m [34m === Running batch_operations.go ===[0m ✓ Authentication
successful

=== Batch Insert === Batch insert response: map[failed:[]
successful:[z0o0Ei8fUPLl0rvBEwZIjXkhOt5OJwBpVPEh2sbYa1m2L7NKoA6zw_jTW0E62jZnggcAWFMTVgNQdUXyf5OziQ
bsXe0YONh-A2IUX8BVHN8MRtld-EATFWHMOzNG58UvHuSxVQmbf6RPnYnFsPpNx7ScBqypg3kRhBY4pq0BC2QQ
gmAiilxsHIaUGrtxzb90F5EZyZrvJ1yy7KveZh9gkiXf0Q_YUNlbkEcBEvj50_YeXrYgFPEHUIsS4GeayZiMsg
LBy1UF3fQiaB4jPDXcKhTGbsHC1fiaUhD4LpJEHD_e0Sm18RMOmB5eXTOD8R1ucyEXAVoWHAVnd_GTuulvCLdA
eFHmxkQSQAaqWtS5W6PHOT8rb4SZ5e-rbmpMUDzV-FBr53U4Ct4a1gzTd9LlvR25KDQcZE1rrX_FOfLf17YdhQ]]
✓ Batch insert completed

=== Creating test records for update/delete === Created 3 test records

=== Batch Update === Batch update response: map[failed:[]
successful:[vK0aT5DYD8VJ8KYs5G8sqokrdBYWLQ4hppR2mj7JRWG_pJBldE4QEWaUHpYKXbpkD79jwkOIzI5jwxMPCJ-OoA
IdKj1toTfp_l9Ai5_LYegTZ_AkMyCY5MqSE_NtK40a9mUzBZv2RGw-VPwiLveFPidYnfZNiBT0-Pu4kxiNrH4g
5dySS49DoOsAUx1ye24ZCwm2mrq8yZz7seKP2_2vDaPwhv9kfAOZxfhTMPDev-e8fFwHSMvu5-YLprS6lOOi-Q]]
✓ Batch update completed

=== Batch Delete === Batch delete response: map[failed:[]
successful:[vK0aT5DYD8VJ8KYs5G8sqokrdBYWLQ4hppR2mj7JRWG_pJBldE4QEWaUHpYKXbpkD79jwkOIzI5jwxMPCJ-OoA
IdKj1toTfp_l9Ai5_LYegTZ_AkMyCY5MqSE_NtK40a9mUzBZv2RGw-VPwiLveFPidYnfZNiBT0-Pu4kxiNrH4g
5dySS49DoOsAUx1ye24ZCwm2mrq8yZz7seKP2_2vDaPwhv9kfAOZxfhTMPDev-e8fFwHSMvu5-YLprS6lOOi-Q]]
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
6lEHVutAilPuK8TrejV0HlRbSPrHKeDvsC3AUIvIFEDuKnN_qcuOmIze0K6OPK2t5BuHZnd9lehZheOIuvXr6g

=== List Collections === Total collections: 15 Sample collections: [users
schema_documents_client_py demo_collection test_collection ws_ttl_test ttl_cache
schema_employees_client_py websocket_test batch_users
chat_configurations__ek0_bench test_accounts schema_products_client_py
chat_messages__ek0_bench scripts__ek0_bench schema_users_client_py]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully [32m✓
collection_management.go completed successfully[0m [34m === Running
document_ttl.go ===[0m ✓ Authentication successful

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
DopmsKdQD7VBvrrX93SF7ukzm6AlKhctGv5TGbqSQA7cqTDx_iGUseZ7orkZNJV1UbQfpraWHyJQwsc7ZfzdNA

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 3 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully [32m✓ document_ttl.go
completed successfully[0m [34m === Running websocket_ttl.go ===[0m ✓
Authentication successful

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
be_4hkQ3Nc-yAdX67XrvRpx_Df_EtybFg2mo6Iv0-3wmfb607AbLgyB0TxQUrfufc1e6mSDoreBlTrNbZqXxRQ

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 3 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully [32m✓ websocket_ttl.go completed
successfully[0m [34m === Running transactions.go ===[0m ✓ Authentication
successful

=== Setup: Creating Test Accounts === Created Alice: $1000 - ID:
27WL\_\_ZEW_jUGfcs7IPRNTpDeFF9WsTiVaQXjTR0UxKGWDi9BZeNhkUgdnrkpHMY7iyGqlLoIzHCJbYNj_gmIw
Created Bob: $500 - ID:
Yi6MFNtIWxVDi4mWYKJG_H_Fp_FVZBsAet5CPhqL2LbbnFQO_9I0qE9zdtgVqSTWj-xPVmN8RD2j8OoUkI_txA

=== Example 1: Begin Transaction === Transaction ID:
847b9471-93dd-4dfa-9260-7ad8f6588383

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: Active Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: map[type:Integer value:800] Bob: map[type:Integer
value:700]

=== Example 5: Rollback === New transaction:
8aa608bb-38ae-426c-a994-1027f12c7ed3 Updated Bob: $700 → $600 (in transaction) ✓
Transaction rolled back Bob after rollback: map[type:Integer value:700]

=== Cleanup === ✓ Deleted test accounts

✓ All transaction examples completed [32m✓ transactions.go completed
successfully[0m [34m ╔════════════════════════════════════════╗[0m [34m║ Test
Summary ║[0m [34m╚════════════════════════════════════════╝[0m [34mTotal: 8[0m
[32mPassed: 8[0m [32mFailed: 0[0m ✅ [32mGo direct examples complete![0m 🧪
[36mRunning Go client library examples...[0m ✓ Client created (token exchange
happens automatically)

=== Insert Document === Inserted:
map[id:Ss0KiaLcOf49co0_rk3AKtQ18FUu1w83gSK7Zgq6EhkHl3x3KpYEp6IDycpBLx1njLctn6C-LGNkjuDDxMzSRQ]

=== Find by ID === Found: map[active:map[type:Boolean value:true]
id:Ss0KiaLcOf49co0_rk3AKtQ18FUu1w83gSK7Zgq6EhkHl3x3KpYEp6IDycpBLx1njLctn6C-LGNkjuDDxMzSRQ
name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query === Found documents: 1

=== Update Document === Updated: map[active:map[type:Boolean value:true]
id:Ss0KiaLcOf49co0_rk3AKtQ18FUu1w83gSK7Zgq6EhkHl3x3KpYEp6IDycpBLx1njLctn6C-LGNkjuDDxMzSRQ
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
OnCthxy44hBF-aCBQEgkK3NtKrybNJjP6MRqoJ8yHb29zE-17TsOhfYKBkuv7Cij7aWlbhqmfEaKogHXlXs4aQ

=== List Collections === Total collections: 15 Sample collections: [users
schema_documents_client_py test_collection ws_ttl_test ttl_cache]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
UmSyTu0CJiyh163YPeqQwpyNT4HVpIui6lUy3iXzwJE4U-ayVnrh3eexshdGIGiikzD2KnIktIm6ZP2RqsS2uQ

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
1U0l8PKF0y1xDpg0Xsyml82eHhOCOXySskHnsGnsqvTVwzHTU3dOU7TxdpXvijxxes-eD6WlV6dzUA_OMvxviA

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
pxXtdKyyjXizzAIbWBia6nfJo6sjS9-3c26ITJlfxl60wF8VtRSs_rhRGp6swzs_5T8oNSHtWrAD1Mx4c-J0Bg

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
qlMZ6FSExMqqiPK2KTmDLWvjFJsdR4A5rnInbhP5jzyMH5GCgy9-3ZuykrFsoSiHx7md7hMK27l9hn3mv4OYXw

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
XqylP4HFwIW9Ahwo41V5fM1SLJvXFqcbPoqoN5ayJ0TxY8OwX3yRTzKyRL45_3UXctd83YgI8BvXUCJYdIWkmg

=== Sending Chat Message === Message ID:
vxL2T4Jskmh5ciat6caPEZMyrhg9p36IMWzXzhJklPGRgOz7UDESX7VNgdzC5wLgpjDlaCJ3QlUKY6_ADsAZUg

=== AI Response === I'm sorry, but the provided context doesn't contain specific
information about any products or their prices. Could you please provide more
details or a different context?

=== Context Used (3 snippets) === Snippet 1: map[collection:client_chat_basic_go
matched_fields:[description] record:map[description:map[type:String
value:Enterprise edition product with advanced features]
id:jo9FSKofqTDAwLw3I-rLv69n6V7SQ0zQgB4zqoQ2A5yYA235f4ZF-oGppX-S1dMKrSIsmcs7jYvf500jy2RGJA
name:map[type:String value:ekoDB Pro] price:map[type:Integer value:299]]
score:0.1111111111111111] Snippet 2: map[collection:client_chat_basic_go
matched_fields:[description] record:map[description:map[type:String value:A
high-performance database product with AI capabilities]
id:Y1yCuMRM5BFCP6Qp09w5d7qgLSQwdBGJeIVNVbZV3SYkiR_ZkwFSVLqqVedj55zntzcAOPqXS9CjQMwOw6pvXQ
name:map[type:String value:ekoDB] price:map[type:Integer value:99]]
score:0.1111111111111111] Snippet 3: map[collection:client_chat_basic_go
matched_fields:[description] record:map[description:map[type:String value:Fully
managed cloud database service product]
id:eAqvSK1_J3BY3gqHHgG-KCi-MeK3xZJyM70CKWC54ai58rLCnyg6qKx6mBg51uEv4iDjri8t3bE4-j5Xx8xapA
name:map[type:String value:ekoDB Cloud] price:map[type:Integer value:499]]
score:0.1111111111111111]

Execution Time: 1135ms

=== Token Usage === Prompt tokens: 322 Completion tokens: 31 Total tokens: 353

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
mGV91EHKMf7NwRPDOedF5F6YMIqJ9Ud_AnWeX7HXYmkc_K-KO_y59NL7bIxkhwpaHLRWeYyQSHYXSndXJAj1Sw

=== Sending Messages === ✓ Message 1 sent Response: I'm sorry, but the provided
context does not contain specific information about the available products.
Could you please provide more details or a specific product category you are
interested in?

✓ Message 2 sent Response: I'm sorry, but the context provided doesn't include
information about the price of any product. Could you please provide more
details or specify which product's price you're asking about?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
d9-4S03Q1d7m4wWBGRyGtY4qTSdqWcIksBM-adlhJvdhAFBHXZhmaN4c5LyWavMq0zi2mGFVCJQStCWqkEKfZg
Parent:
mGV91EHKMf7NwRPDOedF5F6YMIqJ9Ud_AnWeX7HXYmkc_K-KO_y59NL7bIxkhwpaHLRWeYyQSHYXSndXJAj1Sw

=== Listing Sessions === ✓ Found 9 sessions Session 1:
d9-4S03Q1d7m4wWBGRyGtY4qTSdqWcIksBM-adlhJvdhAFBHXZhmaN4c5LyWavMq0zi2mGFVCJQStCWqkEKfZg
(Untitled) Session 2:
mGV91EHKMf7NwRPDOedF5F6YMIqJ9Ud_AnWeX7HXYmkc_K-KO_y59NL7bIxkhwpaHLRWeYyQSHYXSndXJAj1Sw
(Untitled) Session 3:
XqylP4HFwIW9Ahwo41V5fM1SLJvXFqcbPoqoN5ayJ0TxY8OwX3yRTzKyRL45_3UXctd83YgI8BvXUCJYdIWkmg
(Untitled) Session 4:
48IeSr-b5OvgoPOo5UeudSGqoPBWM9t1iupVu29o83S7xz239syKzCaUsRhQY4gO-CBRPK3vUG5Fu8m694PnmA
(Untitled) Session 5:
17Zok24kRKL0JCbQzJ2miG8bWcWIQ3fqG7mD3aIpuLjAMu0V_Qv5aoQvN1lmqhrl44_BhaVGQWOZHxhvfOpnRA
(Untitled) Session 6:
\_ZfZ-i61nrfaPM1Ql5QjKywbDAUwP0ApPNSm8aSneEByuytGCny8Dec49cx3-gRpg03AvoP_26-ID7_KtQfCLw
(Untitled) Session 7:
B0IRmVduPyhJcej9WW3Kz3ykpu3orY3VUi_OYpThxcXTAAnvs_tOFH5m72d4njdGdiWzc0kZ-wkLVPly6-AHOg
(Untitled) Session 8:
7BDD-zJBVBWMsH1TZJ9mpV8ZjmliM9Il_h-MDHYIRwB09rF5oK14CqlOiemb2KwvAOhtymFAKWv5RYXVLn9LGg
(Untitled) Session 9:
YkiDVUICmI0KGpwjWBz6CkhCmH570quCFqsURqHvH-deVCrMTx6Hp0qv3b3MepmPdkUcfSUdnEOABuONpyYBKQ
(Untitled)

=== Getting Session Details === ✓ Session details retrieved Messages: 4

=== Deleting Branch Session === ✓ Deleted branch session:
d9-4S03Q1d7m4wWBGRyGtY4qTSdqWcIksBM-adlhJvdhAFBHXZhmaN4c5LyWavMq0zi2mGFVCJQStCWqkEKfZg

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
wfjfrsAshzmiPQAaQPLc8pBmTZhWPkDS-7vw6qvwTn2W4Ql6WKm2NhwNHqs5dg-k3uyrPtuNHZwIKNgYM-m0GA

=== Sending Initial Message === ✓ Message sent Response: I'm sorry, but the
provided context doesn't contain any specific information about available
products. Could you please provide more details or clarify your request?

✓ Second message sent

=== Feature 1: Regenerate AI Response === ✓ Message regenerated New response:
Answer: I'm sorry, but the context provided does not include information on the
price of ekoDB.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
p_OflSDoLgj1I23BWhnbC3uURTcqR7G2QxFlCkVn03E1yvKx4CnEyAErMUHT8A0_3vjzc5lc2OkcgBJPSgd6XQ
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
1. Score: 0.771
1. Score: 0.760
1. Score: 0.726

1. Hybrid search (text + vector): Found 3 results using hybrid search (text +
   vector)
1. Score: 2.704
1. Score: 1.508
1. Score: 0.290

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

- Laptop ($0) by Alice Johnson
- Mouse ($0) by Alice Johnson

5. Complex join with multiple conditions: Found 2 users with example.com emails

- Alice Johnson (alice@example.com): Building A
- Bob Smith (bob@example.com): Building B

=== Cleanup === ✅ Deleted test collections

✅ Join operations examples completed! 🚀 ekoDB Scripts Example (Go Client)

✅ Client initialized

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved:
iCm_foCPqKXhLxRCaRWfh14SnlVDTY4idpS6xVujUwLZSw88VPA-ZzK1p091e6B0Gd1x51CPLv6eueLP1LSuTw
📊 Found 30 records ⏱️ Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved 📊 Found 30 users (limited) ⏱️ Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved 📊 Statistics: 2 groups map[avg_score:60 count:15 status:active]
map[avg_score:50 count:15 status:inactive] ⏱️ Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 7 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

📝 Example 5: Multi-Stage Pipeline

✅ Multi-stage script saved 📊 Pipeline executed 2 stages ⏱️ Total execution
time: 0ms 📈 Stage breakdown:

📝 Example 6: Count Users

✅ Count script saved 📊 Total user count: 30 ⏱️ Execution time: 0ms

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

added 1 package, removed 1 package, and audited 13 packages in 701ms

3 packages are looking for funding run `npm fund` for details

found 0 vulnerabilities ✓ Client created (token exchange happens automatically)

=== Insert Document === Inserted: { id:
'RXdtfsZ94Dxb9-z12wCLbFsuBBJkx-wPpgz7zuLDrVvanK2jiQifcQg4GOLoALtutB5ghkGj4rksin5J8A76bw'
}

=== Find by ID === Found: { id:
'RXdtfsZ94Dxb9-z12wCLbFsuBBJkx-wPpgz7zuLDrVvanK2jiQifcQg4GOLoALtutB5ghkGj4rksin5J8A76bw',
name: { type: 'String', value: 'Test Record' }, active: { type: 'Boolean',
value: true }, value: { value: 42, type: 'Integer' } }

=== Find with Query === Found documents: 1

=== Update Document === Updated: { id:
'RXdtfsZ94Dxb9-z12wCLbFsuBBJkx-wPpgz7zuLDrVvanK2jiQifcQg4GOLoALtutB5ghkGj4rksin5J8A76bw',
value: { type: 'Integer', value: 100 }, active: { type: 'Boolean', value: true
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

=== Get Multiple Keys === cache:product:1: { name: 'Product 1', price: 29.99 }
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
FuqRhbbQCFoaIwFldBup6B2qN8sAFuZSj9D5OXHK10UrJG8DAN8EW2ObXOjEPUAyUuUN0EBm6aY1fYy17L1Hnw

=== List Collections === Total collections: 18 Sample collections:
schema_documents_client_py,schema_documents_client_go,client_collection_management_ts,test_collection,ws_ttl_test

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
LybzDjEbkKWFENXuVTH84Hgg9as5_jlsAb_1ZFgrv5GV7KAcKUW27ExQkd7yEAKpvOlqyfUp4zQQmt3QdUj90g

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
neQ0oEpWFvxRBxvvj9R6d8Mb3CPhn1NM45S8IFcTxXgy6FUjH7qK1BlSg7HjLQsIgizdQIqzjJS0Zq0g-xSDmA

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
sUVbX6c7Kx6IkAWXwaULIGFS6RG2_T7V22l1jMbI_rwfpM687ZOFcAkdfSe7SuP_5BvbuVjaaTGQlitAmja3bg

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
-jkFUCiwT8ALcjxKZauWCPEsmLhk3e7JZ4ZoWAUs_2HI4QTW_PzlZMJdLhFDTe3ksd8MsLOXf5Nmq2rbKMJmVA

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
qkbLEE_1fFuGTX3Ok9Vxhk4tDPDOldzO7UZubV_3jacMhA-MkdDcfhMpn4IrPvkhZ2ZOEPMhDTevhlaRwv3hKw

=== Sending Chat Message === Message ID:
j-TVIaT0zlou4J-me60aIH2gn2lI_PncBpWkx00FoVXxWKc_wUQatXHcaATtzChTBeypetmoxtFZxyX7mW0J7w

=== AI Response === I'm sorry, but the provided context does not contain
specific information about the product names and their corresponding prices.
Could you please provide more details or context?

=== Context Used (3 snippets) === Snippet 1: { collection:
'client_chat_basic_ts', record: { name: { type: 'String', value: 'ekoDB Cloud'
}, id:
'oeSO7bII1BemmnXQnEspwai-i5yHv4UTf0CYoXa70fYkXAUdV_N_UR4eizsDYhg1vYOd4Ke02958U018CTWk1Q',
description: { value: 'Fully managed cloud database service product', type:
'String' }, price: { type: 'Integer', value: 499 } }, score: 0.1111111111111111,
matched_fields: [ 'description' ] } Snippet 2: { collection:
'client_chat_basic_ts', record: { name: { value: 'ekoDB', type: 'String' },
description: { type: 'String', value: 'A high-performance database product with
AI capabilities' }, price: { value: 99, type: 'Integer' }, id:
'lowNpdwCcR-HASV-KyaDZA1G4PqecajFtpg4SPimxvGoomG9P0d3LFvPIz-IlC-L0NjkLZYJ7NtoUP1yNjzVSw'
}, score: 0.1111111111111111, matched_fields: [ 'description' ] } Snippet 3: {
collection: 'client_chat_basic_ts', record: { price: { type: 'Integer', value:
299 }, name: { type: 'String', value: 'ekoDB Pro' }, id:
'kkHX14KdXjOK37-QlZm5sPLlBTHAemdeSfz3lq5cZlqAgeEGVeKiHCcpcWq268xB3VYnUbfqFmlOBBBXfSRTpg',
description: { value: 'Enterprise edition product with advanced features', type:
'String' } }, score: 0.1111111111111111, matched_fields: [ 'description' ] }

Execution Time: 1236ms

=== Token Usage === Prompt tokens: 316 Completion tokens: 31 Total tokens: 347

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
PW3qaGhiyvmk\_-RTKqBXfQCiFa_D30yeGTDITy1ACfJPvWTYTrchMhoucgCYaJpThCHu2WZWyAxX8pilfX3CSw

=== Sending Messages === ✓ Message 1 sent Response: I'm sorry, but the context
provided doesn't contain specific information about any products. Could you
please provide more details or clarify your request?

✓ Message 2 sent Response: Apologies, but the context provided doesn't include
information about a specific product or its price. Could you please provide more
details or specify the product you're asking about?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
E6kRbN*F9ci7Cw1TfEwJu1FwytFxrqqgPm-4LCqSmFTQmn0o-mxwsbjfIjW6ULaXFM5Jrze-aLrLHbyUJSUiUA
Parent:
PW3qaGhiyvmk*-RTKqBXfQCiFa_D30yeGTDITy1ACfJPvWTYTrchMhoucgCYaJpThCHu2WZWyAxX8pilfX3CSw

=== Listing Sessions === ✓ Found 10 sessions Session 1:
E6kRbN*F9ci7Cw1TfEwJu1FwytFxrqqgPm-4LCqSmFTQmn0o-mxwsbjfIjW6ULaXFM5Jrze-aLrLHbyUJSUiUA
(Untitled) Session 2:
PW3qaGhiyvmk*-RTKqBXfQCiFa_D30yeGTDITy1ACfJPvWTYTrchMhoucgCYaJpThCHu2WZWyAxX8pilfX3CSw
(Untitled) Session 3:
qkbLEE_1fFuGTX3Ok9Vxhk4tDPDOldzO7UZubV_3jacMhA-MkdDcfhMpn4IrPvkhZ2ZOEPMhDTevhlaRwv3hKw
(Untitled) Session 4:
p_OflSDoLgj1I23BWhnbC3uURTcqR7G2QxFlCkVn03E1yvKx4CnEyAErMUHT8A0_3vjzc5lc2OkcgBJPSgd6XQ
(Untitled) Session 5:
mGV91EHKMf7NwRPDOedF5F6YMIqJ9Ud_AnWeX7HXYmkc_K-KO_y59NL7bIxkhwpaHLRWeYyQSHYXSndXJAj1Sw
(Untitled) Session 6:
XqylP4HFwIW9Ahwo41V5fM1SLJvXFqcbPoqoN5ayJ0TxY8OwX3yRTzKyRL45_3UXctd83YgI8BvXUCJYdIWkmg
(Untitled) Session 7:
48IeSr-b5OvgoPOo5UeudSGqoPBWM9t1iupVu29o83S7xz239syKzCaUsRhQY4gO-CBRPK3vUG5Fu8m694PnmA
(Untitled) Session 8:
17Zok24kRKL0JCbQzJ2miG8bWcWIQ3fqG7mD3aIpuLjAMu0V_Qv5aoQvN1lmqhrl44_BhaVGQWOZHxhvfOpnRA
(Untitled) Session 9:
\_ZfZ-i61nrfaPM1Ql5QjKywbDAUwP0ApPNSm8aSneEByuytGCny8Dec49cx3-gRpg03AvoP_26-ID7_KtQfCLw
(Untitled) Session 10:
B0IRmVduPyhJcej9WW3Kz3ykpu3orY3VUi_OYpThxcXTAAnvs_tOFH5m72d4njdGdiWzc0kZ-wkLVPly6-AHOg
(Untitled)

=== Getting Session Details === ✓ Session details retrieved Messages: 4

=== Deleting Branch Session === ✓ Deleted branch session:
E6kRbN_F9ci7Cw1TfEwJu1FwytFxrqqgPm-4LCqSmFTQmn0o-mxwsbjfIjW6ULaXFM5Jrze-aLrLHbyUJSUiUA

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
MWRilXZZgRSI1BREH-M41tJsZNVi8NNOBtwrb2K45pil2rUVFUNxV2O6ogc9fS1XWeVgzEdMTDw9_Fwti16y6Q

=== Sending Initial Message === ✓ Message sent Response: I'm sorry, but the
context provided doesn't contain specific information about any available
products. Could you please provide more detailed information?

✓ Second message sent

=== Feature 1: Regenerate AI Response === ✓ Message regenerated New response:
Answer: I'm sorry, but the context provided does not include information on the
price of ekoDB.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
U875fq5jYR34Tng6NKbvpO2-bkyWIcONOOJYo27ABn_EyWs41NJKHZ50DczKNCcUU3Rftl8geJM4hvXHdToV6g
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
1. Score: 25.740, Matched: name.value, email.value, email, name
1. Score: 12.540, Matched: name, name.value

1. Fuzzy search (typo tolerance): Found 4 results with fuzzy matching
1. Score: 26.400, Matched: bio.value, bio, title.value, title
1. Score: 26.400, Matched: title, title.value, bio.value, bio
1. Score: 26.400, Matched: title, bio, bio.value, title.value
1. Score: 26.400, Matched: title, bio.value, bio, title.value

1. Search with field weights: Found 4 results with weighted fields
1. Score: 39.600, Matched: title, bio.value, bio, title.value
1. Score: 39.600, Matched: title, bio, bio.value, title.value
1. Score: 39.600, Matched: title, bio, bio.value, title.value
1. Score: 39.600, Matched: bio.value, title.value, title, bio

1. Search with minimum score threshold: Found 2 results with score >= 0.3
1. Score: 13.200, Matched: bio, bio.value
1. Score: 13.200, Matched: bio, bio.value

1. Search with stemming and exact match boosting: Found 1 results (matches:
   work, working, worked)
1. Score: 13.200, Matched: bio, bio.value

1. Vector search (semantic search): Found 3 semantically similar documents
1. Score: 0.755, Matched:
1. Score: 0.740, Matched:
1. Score: 0.734, Matched:

1. Hybrid search (text + vector): Found 3 results using hybrid search (text +
   vector)
1. Score: 2.694, Matched: title.value, content.value, title, content
1. Score: 1.502, Matched: title, content, content.value, title.value
1. Score: 0.296, Matched:

1. Case-sensitive search: Found 1 results (case-sensitive)
1. Score: 26.400, Matched: bio.value, bio, skills, skills.value

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
QEb-TbhM6G-SzdYfVI4CDOq4Cck3sRJNSFaHljU5st1yVZZNLNWykjRfA3Z8I5uwAKKl_fC_dmJU4ld2NnDNFA
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved:
aPfDt5pP8jimkqtX3p0HrvY9XSiMcFlN4lQx_8SKP88b0sh1e3bKyYBNpg0Tr8ryuMD0A_nwJo0rcFeM5Zxt5w
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved:
RaelLf-KnbL0aRK86_iy1scz_7CoYp-Z85PG71R6Xh-OpeKbH6V46czX0RXpJdopLzedJwhxrVHb95wkp4h6iQ
📊 Statistics: 2 groups {"status":"active","count":5,"avg_score":60}
{"count":5,"avg_score":50,"status":"inactive"}

📝 Example 4: Script Management

📋 Total scripts: 11 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed! 🚀 ekoDB TypeScript Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data... ✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved:
uv4vxAOpHjRFVn6vCZtIUVO7LQVmk6r4mK61p2taXLHfFg0SrZWHagpL5lpsLa3k38VlmeGuX24vz4rfdTrQcg
📊 Found 2 product groups
{"avg_price":575.6666666666666,"category":"Electronics","count":3}
{"category":"Furniture","count":2,"avg_price":474} ⏱️ Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved 📊 Found 5 products ⏱️ Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved 📊 Found 2 categories {"count":2,"category":"Furniture"}
{"category":"Electronics","count":3} ⏱️ Execution time: 0ms

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

added 1 package, removed 1 package, and audited 9 packages in 651ms

1 package is looking for funding run `npm fund` for details

found 0 vulnerabilities [34m ╔════════════════════════════════════════╗[0m [34m║
ekoDB JavaScript Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m [34m === Checking Server
Connection ===[0m (node:26319) [DEP0040] DeprecationWarning: The `punycode`
module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) [32m✓
Server is ready[0m [34m === Getting Authentication Token ===[0m [32m✓
Authentication successful[0m [33m === Running 9 Examples ===[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_crud.js
===[0m ✓ Authentication successful

=== Insert Document === (node:26322) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) Inserted:
{ id:
'yXnIhKW4M_h0JtyJESZyji4e79a6wBTmsyl_V0T7SEjYWOcQhg4W5zpUtYKxJjEjYVI3BU0ZMm9FdUPp29YU3A'
}

=== Find by ID === Found: { name: { value: 'Test Record', type: 'String' },
value: { value: 42, type: 'Integer' }, active: { type: 'Boolean', value: true },
id:
'yXnIhKW4M_h0JtyJESZyji4e79a6wBTmsyl_V0T7SEjYWOcQhg4W5zpUtYKxJjEjYVI3BU0ZMm9FdUPp29YU3A'
}

=== Find with Query === Found documents: 1

=== Update Document === Updated: { active: { type: 'Boolean', value: true },
value: { type: 'Integer', value: 100 }, name: { value: 'Updated Record', type:
'String' }, id:
'yXnIhKW4M_h0JtyJESZyji4e79a6wBTmsyl_V0T7SEjYWOcQhg4W5zpUtYKxJjEjYVI3BU0ZMm9FdUPp29YU3A'
}

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully [32m✓ simple_crud.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_websocket.js
===[0m ✓ Authentication successful

=== Inserting Test Data === (node:26335) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Inserted
test record:
A1pbDsJbqPKfY4-SKfOeuctbX-rKBkMv1luw6vpzCdlq26DFb0MNtnJVrugvGVKbhpTLoXcTh58sJ_vU4QSS1g

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"7mvDvjLu-3eoQZwjlZon8ia6SymRSNNQo0VlDw_zXmr7rkofCdE4da0lfkWdQKmCUTscj8culXnsEWFRpq3cCw",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"A1pbDsJbqPKfY4-SKfOeuctbX-rKBkMv1luw6vpzCdlq26DFb0MNtnJVrugvGVKbhpTLoXcTh58sJ_vU4QSS1g",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"Xmfr_Z2upSSFQAcp-owmbvissITLMYTXOLA0QCeJe1vZiQq6v5nK8U39FZycJMcGfHfeCzslbFfsriw4sfw7VQ",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"-StpvC0i8Gt6eyenL3QP9h3rW2r0I9qfrnNaba88DIxHpZPxfUglMnSlcwVMvT1ZWYpBf50k510Fiy7LaTQCaQ",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } } ] }, "type": "Success" } ✓ Retrieved 4
record(s) via WebSocket

✓ WebSocket example completed successfully [32m✓ simple_websocket.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/batch_operations.js
===[0m ✓ Authentication successful

=== Batch Insert === (node:26342) [DEP0040] DeprecationWarning: The `punycode`
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

=== KV Set === (node:26345) [DEP0040] DeprecationWarning: The `punycode` module
is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Set key:
session:user123

=== KV Get === Retrieved value: { loginTime: '2025-12-09T05:40:14.187Z', userId:
123, username: 'john_doe' }

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: { name: 'Product 1', price: 29.99 }
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully [32m✓ kv_operations.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/collection_management.js
===[0m ✓ Authentication successful

=== Create Collection (via insert) === (node:26352) [DEP0040]
DeprecationWarning: The `punycode` module is deprecated. Please use a userland
alternative instead. (Use `node --trace-deprecation ...` to show where the
warning was created) Collection created with first record:
u5VtRZhI7H1xR2G_uBbGoixV18NGoSC8_hebKgdhoUoziaYE3ZM2xDjX5RyqVqdxMSqoFZrzNUX1XvIPEro7dA

=== List Collections === Total collections: 23 Sample collections: [
'schema_products_client_ts', 'users', 'schema_documents_client_py',
'schema_documents_client_go', 'demo_collection' ]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully [32m✓
collection_management.js completed successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/document_ttl.js
===[0m ✓ Authentication successful

=== Insert Document with TTL (1 hour) === (node:26355) [DEP0040]
DeprecationWarning: The `punycode` module is deprecated. Please use a userland
alternative instead. (Use `node --trace-deprecation ...` to show where the
warning was created) ✓ Inserted document:
fjFQ_EEs9KunXb3ssFc9MLnh-1x16A1VhcZyqjlmUYdCK3k3XWw0tlxuVvlXN61mxmWWrCbpYIV8VCML_75lkg

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 4 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully [32m✓ document_ttl.js
completed successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/websocket_ttl.js
===[0m ✓ Authentication successful

=== Insert Test Data with TTL === (node:26362) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Inserted
document with TTL:
KQV9PawhitGmg3NXwL2_UgEUfs0F_WpQz0RMjhYM28CQIGEkmBkaTP-rNkNWpnQBZ41gUneN8Be3CZ9dxQq-yA

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 4 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully [32m✓ websocket_ttl.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/http_functions.js
===[0m 🚀 ekoDB Scripts Example (JavaScript/HTTP)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function

✅ Script saved:
BcsESSowQ1HTy\_-T6kyT7x9AfiUDPJzhGuFjmUe8yKaTJ98tcxg-kyh7uyK3otWXKeXXu1YUPD3iowGsbflONw
📊 Found 20 active users

📝 Example 2: Parameterized Function

✅ Script saved:
Oop9mbvL2-tbDgzHepw0Q1MiXIbP_pLlt6MvRm6cUyEGilh1Lnwx8TxiH0g6w606mVjZE7oOq-NWmpwBWKR5QA
📊 Found 20 users (limited)

📝 Example 3: Aggregation Function

✅ Script saved:
EzNksXVaQ5dMKlPoDR_kLKuXViTTqgZRuYjfy3DAE3R_kZfgOqagxyYGmGfICkem-JBLiPxiyw6AdATFn2XqVg
📊 Statistics: 2 groups {"status":"inactive","count":10,"avg_score":50}
{"status":"active","avg_score":60,"count":10}

📝 Example 4: Function Management

📋 Total scripts: 13 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

✅ All examples completed! [32m✓ http_functions.js completed successfully[0m
[34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/transactions.js
===[0m ✓ Authentication successful

=== Setup: Creating Test Accounts === Created Alice: $1000 - ID:
Bwweft8xib18fG1fZPRfV8n_gc0FfcKDWOZllgSeU9y2rLkw99EDgB66KlLbDRBAzAwHLJd3SdOt1NBP3xnROQ
Created Bob: $500 - ID:
5z1am5csUiYq0_NQ6dijpV0aAH5rBw-n98aj59BkS3tGT6k_CFk0bhuQFiMhshgkN272RDpIIftlhnprQD7nDQ

=== Example 1: Begin Transaction === Transaction ID:
f5335fe4-d9c2-48fb-85df-39178b611847

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: Active Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: {"type":"Integer","value":800} Bob:
{"type":"Integer","value":700}

=== Example 5: Rollback === New transaction:
7af84d93-e71c-40a5-9c72-e88e4f78caaf Updated Bob: $700 → $600 (in transaction) ✓
Transaction rolled back Bob after rollback: {"type":"Integer","value":700}

=== Cleanup === ✓ Deleted test accounts

✓ All transaction examples completed [32m✓ transactions.js completed
successfully[0m [34m ╔════════════════════════════════════════╗[0m [34m║ Test
Summary ║[0m [34m╚════════════════════════════════════════╝[0m [34mTotal: 9[0m
[32mPassed: 9[0m [32mFailed: 0[0m ✅ [32mJavaScript direct examples complete![0m
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.3.0 prepare npm run build

> @ekodb/ekodb-client@0.3.0 build tsc

up to date, audited 7 packages in 1s

found 0 vulnerabilities

> @ekodb/ekodb-client@0.3.0 build tsc

✅ [32mTypeScript client built![0m 🧪 [36mRunning JavaScript client library
examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 740ms

1 package is looking for funding run `npm fund` for details

found 0 vulnerabilities ✓ Client created (token exchange happens automatically)

=== Insert Document === Inserted: { id:
'Z35cPTqTpyzwuAXBHTG80wSTbQi4uI6b-MItF4ZzQMDmGEg9ykMT5GZvTUwfu1C_qEzzUu8sK7VWnQwSeOtZUA'
}

=== Find by ID === Found: { name: { type: 'String', value: 'Test Record' }, id:
'Z35cPTqTpyzwuAXBHTG80wSTbQi4uI6b-MItF4ZzQMDmGEg9ykMT5GZvTUwfu1C_qEzzUu8sK7VWnQwSeOtZUA',
value: { value: 42, type: 'Integer' }, active: { type: 'Boolean', value: true }
}

=== Find with Query === Found documents: 1

=== Update Document === Updated: { active: { type: 'Boolean', value: true },
value: { type: 'Integer', value: 100 }, name: { type: 'String', value: 'Updated
Record' }, id:
'Z35cPTqTpyzwuAXBHTG80wSTbQi4uI6b-MItF4ZzQMDmGEg9ykMT5GZvTUwfu1C_qEzzUu8sK7VWnQwSeOtZUA'
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

=== Get Multiple Keys === cache:product:1: { name: 'Product 1', price: 29.99 }
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
4pvlDp5r6DxEo4Sprica6aWqAaoxUrLvtSb9irj6P5lK6TIVWR2Mi17DJ4nNikjoxEkUDI6udtvQvsS_R0oUbA

=== List Collections === Total collections: 23 Sample collections:
schema_products_client_ts,users,schema_documents_client_py,schema_documents_client_go,schema_users_client_ts

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
BgqvCu9lVggY8LDhgug1cYKWpKvIh2CnmMAHcZIyMBkSsG4PilLNOFby1iXXNS62FV9skyLQuxPDVt5cshfXvw

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
t4AShukRaFzk4pOKwrImk0Es9fV5HE1VCOaAIzcc1u4ZCR8UMNLYtkpfAZzU6H2QKnNufDBFPptgwnBEjNLKoA

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
amYwdF8YzC-a9JyaZ5ddHm79tOlUZMMgrXndzMDp*ktkBvHc3aW76*-Dfr5mk8b7zAF2WNnmpyZ7eHwNJxjCXA

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
l7xt9-MtDJRIZkK3vPbCE92B94P3F35eWSniO4FLgfbfFdRSGCdJgkWoJ3fy6MK6CiG-1lpzSuCSp6YvC7uCnQ

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration 🚀 ekoDB Scripts Example (JavaScript/HTTP)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function

✅ Script saved:
c7BdNiK5_AnbNuc7KoTg2a51OEuoi6UWXBS_nCLyRyvG4XArXNEBClzilgLxRfIR5jhv7xf4VVV6lM1kggFdFQ
📊 Found 30 active users

📝 Example 2: Parameterized Function

✅ Script saved:
qlYrzyVNxYA7kklGdcpDGZA1JQwmiSlgN1iQZb3OmLeP8TixbAUKz0WxZfqO65Ptj-NBccmi1SGKjRxQ4cGaIw
📊 Found 30 users (limited)

📝 Example 3: Aggregation Function

✅ Script saved:
fcb4A2jNJN_P9M49PeYP_1xjBPChMTAmlWDTYiLbp1Ed8QYQ6bU6FohuyaT_cFuNC6tNnO8I4d6cWt5AwSUVjQ
📊 Statistics: 2 groups {"count":15,"avg_score":50,"status":"inactive"}
{"avg_score":60,"count":15,"status":"active"}

📝 Example 4: Function Management

📋 Total scripts: 15 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

✅ All examples completed! 🚀 ekoDB Scripts Example (JavaScript Client)

✅ Client initialized (token exchange automatic)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved:
\_VvQsHpCuw075Mow0zJE33mrtjW0Vs05EXtl2v001oyl-J8-AXCrK2QnEzuIYy2zQHQKdhVXPgLfKe67aSRXYw
📊 Found 40 records ⏱️ Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved 📊 Found 40 users (limited) ⏱️ Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved 📊 Statistics: 2 groups
{"avg_score":50,"count":20,"status":"inactive"}
{"avg_score":60,"count":20,"status":"active"} ⏱️ Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 17 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

📝 Example 5: Multi-Stage Pipeline

✅ Multi-stage script saved 📊 Pipeline executed 2 stages ⏱️ Total execution
time: 0ms 📈 Stage breakdown:

📝 Example 6: Count Users

✅ Count script saved 📊 Total user count: 40 ⏱️ Execution time: 0ms

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

✅ Script saved 📊 Category analysis (2 categories): Electronics: Products: 6 |
Stock: 232 | Avg Rating: ⭐4.52 Furniture: Products: 4 | Stock: 43 | Avg Rating:
⭐4.26

⏱️ Total execution time: 0ms 📈 Pipeline stages:

📝 Example 5: Project Specific Fields

✅ Script saved 📊 Product summaries (10 items, showing first 3):

1.  Desk Chair - $349 (⭐4.2)
2.  Monitor 27" - $399 (⭐4.6)
3.  Standing Desk - $599 (⭐4.7) ⏱️ Execution time: 0ms

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
2.  User 6 (user6@example.com) - Score: 60
3.  User 8 (user8@example.com) - Score: 80
4.  User 1 (user1@example.com) - Score: 10
5.  User 2 (user2@example.com) - Score: 20 ⏱️ Execution time: 0ms

📝 Example 5: User Summary Statistics

✅ Script saved 📊 User summary (2 groups): active undefineds: 7 users, Total
Score: 370 inactive undefineds: 3 users, Total Score: 180 ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Cleanup complete

✅ All CRUD script examples completed! 🚀 ekoDB Search Scripts Example

📋 Setting up test data... ✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Script saved 📊 Found 5 documents

1.  Vector Databases Explained (Database)
2.  Introduction to Machine Learning (AI)
3.  Database Design Principles (Database)
4.  Natural Language Processing (AI)
5.  Getting Started with ekoDB (Database) ⏱️ Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved 📊 Documents by category: AI: 2 documents Database: 3 documents
⏱️ Execution time: 0ms

📝 Example 3: Select Specific Fields

✅ Script saved 📊 Document titles (5 docs):

1.  Vector Databases Explained
2.  Introduction to Machine Learning
3.  Database Design Principles
4.  Natural Language Processing
5.  Getting Started with ekoDB ⏱️ Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved 📊 Projected documents (showing first 3):

1.  Vector Databases Explained
2.  Introduction to Machine Learning
3.  Database Design Principles ⏱️ Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved 📊 All documents (5 total, showing first 2):

1.  Vector Databases Explained (Database)
2.  Introduction to Machine Learning (AI) ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Cleanup complete

✅ All search script examples completed! 🚀 ekoDB AI Scripts Example

⚠️ Note: These examples require AI API credentials (OpenAI, etc.)

📋 Setting up test data... ✅ Created 3 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved 🤖 AI Response:

1. Scalability: Vector databases can efficiently handle large datasets due to
   their spatial indexing capabilities.

2. High Precision: They provide high accuracy and detail, as vector data is
   represented in points, lines, and polygons, which maintains the preciseness
   of the data.

3. Less Space: Vector databases require less storage space compared to raster
   databases.

4. Analytical Capabilities: They support complex spatial analyses, such as
   network analysis and proximity analysis.

5. Visualization: Vector data is easier to visualize and interpret, providing
   clear representations of geographical features. ⏱️ Execution time: 0ms

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
Record(fields={id=StringValue(value=DBMi7robOq2yk1h_VhsiIoP3URs_EJ2SRSyijL089hOY_oKelbkZnu89KDLZ_5AWB1jzTW8jg0i-S_S469DJKw)})
User ID:
DBMi7robOq2yk1h_VhsiIoP3URs_EJ2SRSyijL089hOY_oKelbkZnu89KDLZ_5AWB1jzTW8jg0i-S_S469DJKw

=== Read === ✓ Found user by ID:
Record(fields={id=StringValue(value=DBMi7robOq2yk1h_VhsiIoP3URs_EJ2SRSyijL089hOY_oKelbkZnu89KDLZ_5AWB1jzTW8jg0i-S_S469DJKw),
name=ObjectValue(value={value=StringValue(value=Alice Johnson),
type=StringValue(value=String)}),
age=ObjectValue(value={value=IntegerValue(value=28),
type=StringValue(value=Integer)}),
email=ObjectValue(value={value=StringValue(value=alice@example.com),
type=StringValue(value=String)}),
active=ObjectValue(value={type=StringValue(value=Boolean),
value=BooleanValue(value=true)})})

=== Update === ✓ Updated user:
Record(fields={age=ObjectValue(value={type=StringValue(value=Integer),
value=IntegerValue(value=29)}),
city=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=San Francisco)}),
id=StringValue(value=DBMi7robOq2yk1h_VhsiIoP3URs_EJ2SRSyijL089hOY_oKelbkZnu89KDLZ_5AWB1jzTW8jg0i-S_S469DJKw),
active=ObjectValue(value={type=StringValue(value=Boolean),
value=BooleanValue(value=true)}),
email=ObjectValue(value={value=StringValue(value=alice@example.com),
type=StringValue(value=String)}),
name=ObjectValue(value={value=StringValue(value=Alice Johnson),
type=StringValue(value=String)})})

=== Query === ✓ Found 1 users matching query

- Record(fields={city=ObjectValue(value={value=StringValue(value=San Francisco),
  type=StringValue(value=String)}),
  email=ObjectValue(value={value=StringValue(value=alice@example.com),
  type=StringValue(value=String)}),
  name=ObjectValue(value={value=StringValue(value=Alice Johnson),
  type=StringValue(value=String)}),
  active=ObjectValue(value={type=StringValue(value=Boolean),
  value=BooleanValue(value=true)}),
  id=StringValue(value=DBMi7robOq2yk1h_VhsiIoP3URs_EJ2SRSyijL089hOY_oKelbkZnu89KDLZ_5AWB1jzTW8jg0i-S_S469DJKw),
  age=ObjectValue(value={value=IntegerValue(value=29),
  type=StringValue(value=Integer)})})

=== Delete === ✓ Deleted user with ID:
DBMi7robOq2yk1h_VhsiIoP3URs_EJ2SRSyijL089hOY_oKelbkZnu89KDLZ_5AWB1jzTW8jg0i-S_S469DJKw

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
DDuw1GWURg73wlFIrvL4qiAmvuwK7BmflKOirM1aGRRlvx6EESGZulzAagtiOlZAEjN8LlujYjqfbKzfvC4gBA,
mUD1XRGdILa852QqKNh5DfaHnvpZE4dQgLNkyU6ZcYfVU8vRL6FrfEsdesYUsKmINGWnleVJwLb3ta9enwhTLA,
hqvbKdfE3KDzbt643g1brbU1j64hWb1-nIW605ZXQUrSbkf7WO50z8YpLz4Mn3Z4jN0V_flVMOwE6rrn49_w3g...

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

- Score: ObjectValue(value={type=StringValue(value=Integer),
  value=IntegerValue(value=95)})
- Score: ObjectValue(value={type=StringValue(value=Integer),
  value=IntegerValue(value=92)})
- Score: ObjectValue(value={value=IntegerValue(value=88),
  type=StringValue(value=Integer)})

=== Query 4: Complex (score > 80 AND age >= 25) === ✓ Found 4 high-scoring
adults

=== Query 5: IN (city IN ['NYC', 'LA']) === ✓ Found 4 records in NYC or LA

=== Query 6: Pagination (skip 2, limit 2) === ✓ Page 2 (2 records):

- ObjectValue(value={value=StringValue(value=Charlie),
  type=StringValue(value=String)})
- ObjectValue(value={type=StringValue(value=String),
  value=StringValue(value=Diana)})

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
{"role":"admin","email":"alice@example.com","name":"Alice"}

=== KV Set with TTL === ✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key === ✓ Session value:
{"created_at":1765258857427,"user_id":"123"} (Will expire in 10 seconds)

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

- schema_products_client_ts
- schema_documents_client_py
- schema_documents_client_go
- schema_users_client_ts
- test_collection

=== Check Collection Existence === Collection 'kotlin_collection_example'
exists: false

=== Create Collection with Schema === ✓ Created collection with schema:
kotlin_collection_example

=== Get Collection Schema === ✓ Schema:
{"fields":{"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2025-12-09T05:41:01.657846Z","last_modified":"2025-12-09T05:41:01.657846Z","bypass_ripple":false,"primary_key_alias":"id"}

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
yl-wJSu97elnSTfcgRolZPSTTq2r3YBoSAmvXPVlE00BnlOiqSgCSs729iO3T94fxKqZVvK1yD9rGPp7wHFt_A

=== Verify Document Exists === ✓ Document found: [created_at, ttl_duration,
session_id, user_id, id]

=== Insert with Longer TTL === ✓ Inserted document with 1h TTL Document ID:
6tiBON2jg2RokB1849zVZXyq6mjtcs-JJoqrbf9OGJDd5kJiEyb06LthXVaROAUwZvlz01TneF7K9W4G29K1Dg

=== TTL Expiration === ✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document === ✓ Long TTL document still exists: [id,
cache_key, ttl_duration, value]

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
{"payload":{"data":[{"id":"3oEdzIzOSD5TlZaBHxWflCL2fLt8YDqYwjxlWdzxHewT_uNsM-6e4qFJLzk8fi3wy50ZwMzJZrn-vX78_NKXZQ","name":{"type":"String","value":"Test
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
StringValue(value=8Eziy2_mDlGPgmOK07VVepHCLVH_Jv8ScUPfxUVnLcbTINubzjELPnEPdOlmMbdlMfYF_E7ZfbyhoACHm7Gkgw)
✓ Inserted user 2:
StringValue(value=Km2JQq6OLThBhl7XWFmu0zXlrMU3jo4EjtVEH-RpocHlb8QklryrCTeaEvhAvzLAdKtWbh-G1CQfZp40ZSjcVQ)

=== Getting Schema === ✓ Schema for kotlin_schema_example: Fields:
{"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections === ✓ Total collections: 22 Sample:
[schema_products_client_ts, schema_documents_client_py,
schema_documents_client_go, kotlin_schema_example, schema_users_client_ts]

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
{"results":[{"record":{"views":21,"tags":"programming,rust,tutorial","title":"Rust
Programming","description":"Learn Rust programming language with hands-on
examples and best
practices.","id":"sjX-PgZlkkthLg2HQXGGVAYRv4AWlUbjSMTfoq_2-lPWW0MMbo4g19sAb8e2avgTXwWOnQnkWwvXm1hmx0ivVg"},"score":19.8,"matched_fields":["description","tags","title"]},{"record":{"views":800,"title":"JavaScript
Web Development","description":"Build modern web applications using JavaScript,
React, and
Node.js.","tags":"programming,javascript,web","id":"DZ2_4lDPuV1DimgURCAaQ9bl39aDP6rSAww7aQbZlnLZHHVu7WzMIfCZi3XH7ReWnGg9YjVaRinqgKrU2S3GTQ"},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"description":"Master
Python for data analysis, machine learning, and
visualization.","tags":"programming,python,data-science","id":"cqXU50FZ90ovbQmVdMKUETeNm406nIDMiBjEUPbgdkoQE1RAfiwjPW6rVTcivHGMR5QZAhh3DX24ISp-fSO9qA","title":"Python
for Data
Science","views":435},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' === ✓ Found results for 'machine learning'
{"results":[{"record":{"tags":"ai,machine-learning,python","id":"OJbmP3OTYitZQgCxZGJ3v9FZ0n0GhwCGz8EBK5q07HyjAv1qfR8f-17nq5MTjXLqtR-qtN7DZfCTrx0xaacMFA","description":"Introduction
to machine learning algorithms and neural
networks.","views":419,"title":"Machine Learning
Basics"},"score":2.7,"matched_fields":["description","title","tags"]},{"record":{"title":"Python
for Data Science","views":435,"description":"Master Python for data analysis,
machine learning, and
visualization.","tags":"programming,python,data-science","id":"cqXU50FZ90ovbQmVdMKUETeNm406nIDMiBjEUPbgdkoQE1RAfiwjPW6rVTcivHGMR5QZAhh3DX24ISp-fSO9qA"},"score":1.0,"matched_fields":["description"]},{"record":{"id":"mZ7Lf0i3Pi6A8b-viX0fc7mVMeXpP1eeA8ENkL8K33mvVOPvWSuBVJNB_UBl5SQ-rzf4fSwhDLeNiNIMqJ4r8A","description":"Learn
database design principles, normalization, and query
optimization.","views":639,"tags":"database,design,sql","title":"Database
Design"},"score":0.5,"matched_fields":["description"]},{"record":{"tags":"programming,rust,tutorial","id":"sjX-PgZlkkthLg2HQXGGVAYRv4AWlUbjSMTfoq_2-lPWW0MMbo4g19sAb8e2avgTXwWOnQnkWwvXm1hmx0ivVg","title":"Rust
Programming","description":"Learn Rust programming language with hands-on
examples and best
practices.","views":21},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

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
StringValue(value=3vjQ9_EkRpbppTtAFiI60BoinPcaYhhSNlgb_X4v1zWq0TekVQ53BJoOPLoiPFTiIQ1hsy8ZWT6FnOLfM5eQIg)

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved data via
WebSocket:
{"payload":{"data":[{"created_at":{"type":"Integer","value":1765258882772},"id":"3vjQ9_EkRpbppTtAFiI60BoinPcaYhhSNlgb_X4v1zWq0TekVQ53BJoOPLoiPFTiIQ1hsy8ZWT6FnOLfM5eQIg","name":{"type":"String","value":"WebSocket
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
> Kotlin Client - Basic Chat Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
vkK-IPDpRaOkF4_S-ne1PCGvc2Lg-D3C-3NIe9e-vcnsFTDKllYynudFWiecXHHtja83hQ9FWUpG55nj5qFrVQ

=== Sending Chat Message === ✓ Chat response: Message ID:
"htI9Y82FdaLkd-zxlq-xGi_2i0KplSH9x8Xl0tJPqNZ5tTft-z13SG7fBCoE2Ao1ICYfczrm9mmfHeoSv6weBg"
Responses: ["I'm sorry, but the provided context doesn't contain any specific
information about ekoDB or its features. However, ekoDB is typically a database,
and databases usually have features like data storage, data management, data
retrieval, and security features. It would be best if you refer to the specific
documentation of ekoDB for a comprehensive list of its features."]

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
sgYZC8swkGOQl9EC-lDbQQy6Z-XChJXbXi1fNhrA-QAxJvpU85wZVrxQSIcrKwtkM7o-JE6QVmR52_ZKanDeFw

=== Sending Messages === ✓ Message 1 sent Responses: ["I'm sorry, but the
context provided doesn't contain any specific information about available
products. Could you please provide more details or a different context?"]

✓ Message 2 sent Responses: ["Apologies, but the context provided does not
include information about the price of a specific product. Could you please
mention the product you're interested in?"]

=== Getting Message History === ✓ Retrieved message history Total messages:
[{"chat*id":{"type":"String","value":"sgYZC8swkGOQl9EC-lDbQQy6Z-XChJXbXi1fNhrA-QAxJvpU85wZVrxQSIcrKwtkM7o-JE6QVmR52_ZKanDeFw"},"content":{"type":"String","value":"What
products are
available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":{"type":"String","value":"A
high-performance database product with AI
capabilities"},"id":"feYOzhXKE2broMytoD2Sm3vthuJ7TnOpkL6KNMHR-4WznCigYHiQwcBHyGb8Mu-EtqO_iSSJ6bBUnonzrx2_SA","price":{"type":"Integer","value":99},"product":{"type":"String","value":"ekoDB"}},"score":0.25}]},"created_at":{"type":"DateTime","value":"2025-12-09T05:41:40.082368Z"},"id":"K6Me49vQfDW7gBS4l6CydhI3YVZ57AxPYS8qH--cVE1u*-bIExfu13V1x1CVaagvHsZsh45D6jwlbI8w_becZg","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":29,"prompt_tokens":150,"total_tokens":179}},"updated_at":{"type":"DateTime","value":"2025-12-09T05:41:40.082368Z"}},{"chat_id":{"type":"String","value":"sgYZC8swkGOQl9EC-lDbQQy6Z-XChJXbXi1fNhrA-QAxJvpU85wZVrxQSIcrKwtkM7o-JE6QVmR52_ZKanDeFw"},"content":{"type":"String","value":"I'm
sorry, but the context provided doesn't contain any specific information about
available products. Could you please provide more details or a different
context?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":{"type":"String","value":"A
high-performance database product with AI
capabilities"},"id":"feYOzhXKE2broMytoD2Sm3vthuJ7TnOpkL6KNMHR-4WznCigYHiQwcBHyGb8Mu-EtqO_iSSJ6bBUnonzrx2_SA","price":{"type":"Integer","value":99},"product":{"type":"String","value":"ekoDB"}},"score":0.25}]},"created_at":{"type":"DateTime","value":"2025-12-09T05:41:40.088651Z"},"id":"zYdfeCekwcLKiCkOhe6vctyCdIwqy7--prQcDAIOoS72jQyu0dp3I9kWpYsu1DzSksSxgeORFiLC6yfKJb83FA","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":29,"prompt_tokens":150,"total_tokens":179}},"updated_at":{"type":"DateTime","value":"2025-12-09T05:41:40.088651Z"}},{"chat_id":{"type":"String","value":"sgYZC8swkGOQl9EC-lDbQQy6Z-XChJXbXi1fNhrA-QAxJvpU85wZVrxQSIcrKwtkM7o-JE6QVmR52_ZKanDeFw"},"content":{"type":"String","value":"What
is the
price?"},"created_at":{"type":"DateTime","value":"2025-12-09T05:41:41.718482Z"},"id":"CPRPn18qy18ERfZxBb2U6YrXs928-bZZyABzIGuBrYJXTFcT2Mp78OwbW8qwFdwQVgZhma_0IRF-StjRNrQEsg","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":30,"prompt_tokens":70,"total_tokens":100}},"updated_at":{"type":"DateTime","value":"2025-12-09T05:41:41.718482Z"}},{"chat_id":{"type":"String","value":"sgYZC8swkGOQl9EC-lDbQQy6Z-XChJXbXi1fNhrA-QAxJvpU85wZVrxQSIcrKwtkM7o-JE6QVmR52_ZKanDeFw"},"content":{"type":"String","value":"Apologies,
but the context provided does not include information about the price of a
specific product. Could you please mention the product you're interested
in?"},"created_at":{"type":"DateTime","value":"2025-12-09T05:41:41.719368Z"},"id":"ZRHA5LB-cDQEk87w5ssSsISwnb0DFRuCCjmCgBiIj7dbQzaP7hETGRqDRGtVq0hOA239JHNGOwPvaKeUKpwtaA","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":30,"prompt_tokens":70,"total_tokens":100}},"updated_at":{"type":"DateTime","value":"2025-12-09T05:41:41.719368Z"}}]

=== Updating Session === ✓ Updated session system prompt

=== Listing Sessions === ✓ Total sessions: 13

=== Branching Session === ✓ Created branched session:
NZ0oeQ20I3YPCBzyFl5U3Tyrp7kP0ZJyb6XCw4UXuPiDXjQ3JWz2Aoqpuhd8aIKesKGW2DNJV_1MDDofITVgPQ

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
hjk0IjJSsS1-zr-3_dW0JcE60OA1XbdoDMy7q1LSjhERjKdFVjXkJlo4eznmbT60TesLKchwCl1zJtiCg_Eqgg

=== Sending Initial Message === ✓ Message sent Responses: ["I'm sorry, but the
provided context does not include specific information about available products.
Could you please provide more details or context?"]

✓ Second message sent

=== Regenerating AI Response === ✓ AI response regenerated New responses: ["I'm
sorry, but the context provided does not contain any specific product
information. Can you provide more details or context?"]

=== Updating Message === ✓ Updated message content

=== Toggling Forgotten Status === ✓ Marked message as forgotten (excluded from
context)

=== Creating Second Session for Merge === ✓ Created second session:
F1ax2mm2phQmWFqOO7INRlkwNqg1aXGqFn3Io6-C_aopbQX-Nm_Q2CwIBBVnBPAdnr-PCcN95EQfRsiEywy1rw

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
i6bN2N6ZHn4_5QFllaWN5Ek_UleP2INxXcNw8FBF8O_YdHixDSSvmY81-gTF_XHVSTXcrgwBYwGlcbMOGNe6Dg
📊 Found 10 records ⏱️ Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved 📊 Found 10 users (limited) ⏱️ Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved 📊 Statistics: 2 groups
{"avg_score":60.0,"status":"active","count":5}
{"avg_score":50.0,"count":5,"status":"inactive"} ⏱️ Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 21 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
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
Stage builders • Built-in error handling

BUILD SUCCESSFUL in 3s 2 actionable tasks: 1 executed, 1 up-to-date ✅
[32mKotlin client examples complete![0m 🟣 [33mKotlin Transactions...[0m
/bin/sh: kotlinc: command not found Kotlin example (requires kotlinc installed)
✅ [32mAll Kotlin integration tests complete![0m
