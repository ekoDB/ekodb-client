make test-examples-rust 🧪 [36mRunning Rust examples (direct
HTTP/WebSocket)...[0m Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.90s Running
`target/debug/examples/simple_crud` ✓ Authentication successful

=== Insert Document === Inserted: Object {"id":
String("T4PBWB6fnGERXv6G5Dpy5woRjLfeM84HP1HLTtwB1Ei6GcooqULzteAyUoDnd1LbEAFIQm3g7l06LusMGF0VgA")}

=== Find by ID === Found: Object {"active": Object {"type": String("Boolean"),
"value": Bool(true)}, "id":
String("T4PBWB6fnGERXv6G5Dpy5woRjLfeM84HP1HLTtwB1Ei6GcooqULzteAyUoDnd1LbEAFIQm3g7l06LusMGF0VgA"),
"name": Object {"type": String("String"), "value": String("Test Record")},
"value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query === Found documents: Array [Object {"active": Object
{"type": String("Boolean"), "value": Bool(true)}, "id":
String("T4PBWB6fnGERXv6G5Dpy5woRjLfeM84HP1HLTtwB1Ei6GcooqULzteAyUoDnd1LbEAFIQm3g7l06LusMGF0VgA"),
"name": Object {"type": String("String"), "value": String("Test Record")},
"value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document === Updated: Object {"active": Object {"type":
String("Boolean"), "value": Bool(true)}, "id":
String("T4PBWB6fnGERXv6G5Dpy5woRjLfeM84HP1HLTtwB1Ei6GcooqULzteAyUoDnd1LbEAFIQm3g7l06LusMGF0VgA"),
"name": Object {"type": String("String"), "value": String("Updated Record")},
"value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.75s Running
`target/debug/examples/simple_websocket` ✓ Authentication successful

=== Inserting Test Data === ✓ Inserted test record:
"-g3odd0twQrSsVSLVsISeu0mVnyzhoYUmFFRGNOwE63t_8qQwJM0iIFFDVgCcXfWKD0yRacu6KH6p4ml-i7KgQ"

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"-g3odd0twQrSsVSLVsISeu0mVnyzhoYUmFFRGNOwE63t_8qQwJM0iIFFDVgCcXfWKD0yRacu6KH6p4ml-i7KgQ",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } } ] }, "type": "Success" } ✓ Retrieved 1
record(s) via WebSocket

✓ WebSocket example completed successfully Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.38s Running
`target/debug/examples/batch_operations` ✓ Authentication successful

=== Batch Insert === ✓ Batch inserted 5 records ✓ Verified: Found 5 total
records in collection

=== Creating test records for update/delete === Created 3 test records

=== Batch Update === ✓ Batch updated 3 records ✓ Verified: Record updated with
status="active"

=== Batch Delete === ✓ Batch deleted 3 records ✓ Verified: Records successfully
deleted (not found)

✓ All batch operations completed successfully Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.36s Running
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

✓ All KV operations completed successfully Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.36s Running
`target/debug/examples/collection_management` ✓ Authentication successful

=== Create Collection (via insert) === Collection created with first record:
"kYGKyVT1tlFm5qCQ1KaX3jR0naYv36JSY7QtNL6w6TizJ4Z_g32OH-RA6N6jGiIvZeC8pFwU4qqEGmQczkVoog"

=== List Collections === Total collections: 4 Sample collections:
["websocket_test", "test_collection", "demo_collection", "batch_users"]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully Compiling
ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s Running
`target/debug/examples/document_ttl` ✓ Authentication successful

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
"Dv96mpwDChSS1KnTEaw3BG3FWQ_ZFiWibvKYg4xafcS1utIXIgcxVz-Gyrqeo-ggN2YEMvqOTRmddw0byyntpA"

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: null

=== Query Documents === ✓ Found 1 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

✓ All document TTL operations completed successfully Compiling ekodb-examples
v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev`
profile [unoptimized + debuginfo] target(s) in 0.36s Running
`target/debug/examples/websocket_ttl` ✓ Authentication successful

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
String("PrMmPymel5Pdm23oK-XwMBshuRfq115MeVbZDEgAk-HVS547vETJle5KAiZkSRH-X5LDx1Py_TZaSfDkqVQgSg")

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.33s Running
`target/debug/examples/http_functions` 🚀 ekoDB Scripts Example (Rust/HTTP)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved:
hKiG5xXycYrBF2USzogOC6YyqEB6HRIskR-ciN8_wWxjBfCVsVrfo8iH0sZc2cigqO-3WMxfFayMyL_9Jexf9g
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved:
"H-iPh0bJ_NnQFJ_S6weBTXd4-KZ4_XYcxmpY9fLBejB03ETUBDDkhx7Z9wgrfxzXO8T8VhaiJy3ynM2GXEQG3A"
📊 Page 1: Found 3 users (limit=3, skip=0) 📊 Page 2: Found 2 users (limit=3,
skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Script saved:
F2aJMX8dvm6m6CBXX9cq0VdyI8o4HvR9mU6ftBhmlobe-7mNN-A4kTa_LkfbFKhZ2tTR3arzwL_VZ2I9GjkT0w
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved:
6nDW9JVbJ6BKRAV0vX7E7thv8dDHkQ9-hT6DX_lmH3HY1islESvL4Wlz_xuOIZgvK8YUYKnKzua2XzGbvBsYjw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
{"avg_score":60.0,"count":5,"max_score":100,"status":"active"}
{"avg_score":50.0,"count":5,"max_score":90,"status":"inactive"}

📝 Example 4: Function Management

📋 Total scripts: 4 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

✅ All examples completed! Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.31s Running
`target/debug/examples/transactions` ✓ Authentication successful

=== Setup: Creating Test Accounts === Created Alice: $1000 - ID:
2fj1MhupSI2nwuLJ71QZBZjkq9wC3dFmlCX72uSTmjWZK5oGQJcuhY74O6DkwpiUF6rsC8dkxeQ5eF68y9VRPw
Created Bob: $500 - ID:
96PWB-B-H9LD5sBsa0q7-IS_wFK4uFGxZI_IE7R3KSeYsU61TXJXB1S66tyc-9z-8iL1QIqLfQMemonQmeView

=== Example 1: Begin Transaction === Transaction ID:
d4e82eed-c8d9-41e5-8f8e-cd9290958313

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: "Active" Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: 800 Bob: 700

=== Example 5: Rollback === New transaction:
1fef2dad-38cc-4526-80aa-3f0f023e8851 Updated Bob: $700 → $600 (in transaction) ✓
Transaction rolled back Bob after rollback: 700

=== Cleanup === ✓ Deleted test accounts

✓ All transaction examples completed ✅ [32mRust direct examples complete![0m 🛠️
[36mBuilding client library...[0m cargo build -p ekodb_client Compiling
ekodb_client v0.4.0 (/Users/tek/Development/ekoDB/ekodb-client/ekodb_client)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.83s ✅
[32mClient build complete![0m 🧪 [36mRunning Rust client library examples...[0m
Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.98s Running
`target/debug/examples/client_simple_crud` ✓ Client created (token exchange
happens automatically)

=== Insert Document === Inserted: Record { fields: {"id":
String("2QPMJCC-l_fWzMf2KXt4nbpDfSzKtCdm43VM9AN2G_KmaRwRvXPhCrwFFJ46mL0krDLJ3toLI_6duVkF9tmI1Q")}
}

=== Find by ID === Found: Record { fields: {"active": Object({"type":
String("Boolean"), "value": Boolean(true)}), "value": Object({"value":
Integer(42), "type": String("Integer")}), "id":
String("2QPMJCC-l_fWzMf2KXt4nbpDfSzKtCdm43VM9AN2G_KmaRwRvXPhCrwFFJ46mL0krDLJ3toLI_6duVkF9tmI1Q"),
"name": Object({"type": String("String"), "value": String("Test Record")})} }

=== Find with Query === Found documents: [Record { fields: {"value":
Object({"value": Integer(42), "type": String("Integer")}), "name":
Object({"type": String("String"), "value": String("Test Record")}), "id":
String("2QPMJCC-l_fWzMf2KXt4nbpDfSzKtCdm43VM9AN2G_KmaRwRvXPhCrwFFJ46mL0krDLJ3toLI_6duVkF9tmI1Q"),
"active": Object({"value": Boolean(true), "type": String("Boolean")})} }]

=== Update Document === Updated: Record { fields: {"id":
String("2QPMJCC-l_fWzMf2KXt4nbpDfSzKtCdm43VM9AN2G_KmaRwRvXPhCrwFFJ46mL0krDLJ3toLI_6duVkF9tmI1Q"),
"name": Object({"type": String("String"), "value": String("Updated Record")}),
"active": Object({"value": Boolean(true), "type": String("Boolean")}), "value":
Object({"value": Integer(100), "type": String("Integer")})} }

=== Delete Document === Deleted document

=== Cleanup === ✓ Deleted collection

✓ All CRUD operations completed successfully Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.80s Running
`target/debug/examples/client_batch_operations` ✓ Client created

=== Batch Insert (via multiple inserts) === ✓ Inserted 5 records ✓ Verified:
Found 5 total records in collection

=== Update Records === ✓ Updated 3 records

=== Delete Records === ✓ Deleted 3 records

=== Cleanup === ✓ Deleted collection

✓ All batch operations completed successfully Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.60s Running
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

✓ All KV operations completed successfully Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.77s Running
`target/debug/examples/client_collection_management` ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
"Twcvr7Hx71yrRNyOvFcLbyZzzQk8eJUlEpB6niyzyhQdldvbBYxdftYuGFxX_ABMRaq3dDFbKnVk7yYfCE0jMg"

=== List Collections === Total collections: 9 Sample collections:
["scripts__ek0_bench", "users", "websocket_test", "test_collection",
"test_accounts"]

=== Count Documents === Document count: 1

=== Check Collection Exists === Collection exists: true

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully Compiling
ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.80s Running
`target/debug/examples/client_document_ttl` ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
"GBSTp7JeRVIYHFVIZnLtqdzSIMNT4S-84lMummIORE2LHnLavZMayBB-rLSfHPhyQGSeAhwxP1RHwE1UTnPzEw"

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
Some(String("W38KN1CDBWStSgPUqnyBVpQEW5kkAfXMVRs-ehSUQfjoqW0cfD_sNaOHWrvTscusmWsogECk-h1wdXtcA523Rg"))

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.96s Running
`target/debug/examples/client_simple_websocket` ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
IX3nI7PTcrnDNsCZk6wjbmTBfMH6ltoe8ZFAwWRUR7PKkv4Pdel8MK_1rHZMWWnZUU4Se8JT0crrlaazXqeAJA

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.93s Running
`target/debug/examples/client_websocket_ttl` ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
Some(String("oXAH71dym8i-fLck3wA5XCtwN-MW3HZfXv2b0biFOw1Ua3ROCixnHsp5M8NE_EwKX4scVu_bUm1ZSDO3gQl_LQ"))

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.84s Running
`target/debug/examples/client_query_builder` === ekoDB Query Builder Example ===

=== Inserting Sample Data === ✓ Inserted 5 users

=== Simple Equality Query === ✓ Found 3 active users

- Some(Object({"type": String("String"), "value": String("Bob")}))
- Some(Object({"type": String("String"), "value": String("Alice")}))
- Some(Object({"value": String("David"), "type": String("String")}))

=== Range Query (age >= 28 AND age < 35) === ✓ Found 3 users in age range

- Some(Object({"value": String("Eve"), "type": String("String")}))
- Some(Object({"type": String("String"), "value": String("Bob")}))
- Some(Object({"value": String("David"), "type": String("String")}))

=== IN Operator === ✓ Found 4 users with status active or pending

- Some(Object({"value": String("Bob"), "type": String("String")}))
- Some(Object({"value": String("Alice"), "type": String("String")}))
- Some(Object({"value": String("David"), "type": String("String")}))
- Some(Object({"value": String("Eve"), "type": String("String")}))

=== NOT IN Operator === ✓ Found 4 users not inactive

=== String Pattern Matching === ✓ Found 5 users with @example.com email

=== Regex Query === ✓ Found 0 users with names starting with A-C

=== Complex Query (active AND age >= 28 AND score > 1500) === ✓ Found 2 users
matching all conditions

- Some(Object({"type": String("String"), "value": String("David")}))
- Some(Object({"value": String("Bob"), "type": String("String")}))

=== OR Query === ✓ Found 2 users with age < 28 OR age > 32

- Some(Object({"value": String("Alice"), "type": String("String")}))
- Some(Object({"value": String("Charlie"), "type": String("String")}))

=== Sorted Query (by score descending) === ✓ Top 3 users by score:

1. Some(Object({"value": String("Bob"), "type": String("String")}))
2. Some(Object({"type": String("String"), "value": String("David")}))
3. Some(Object({"value": String("Alice"), "type": String("String")}))

=== Pagination (page 2, size 2) === ✓ Page 2 results:

- Some(Object({"value": String("Charlie"), "type": String("String")}))
- Some(Object({"value": String("David"), "type": String("String")}))

=== NOT Operator === ✓ Found 4 users NOT inactive

=== Complex Chained Query === ✓ Found 3 users with all conditions

=== Cleanup === ✓ Deleted collection

✓ All query builder operations completed successfully Compiling ekodb-examples
v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev`
profile [unoptimized + debuginfo] target(s) in 0.80s Running
`target/debug/examples/client_search` === ekoDB Search Example ===

=== Inserting Sample Documents === ✓ Inserted 5 sample documents

=== Basic Text Search === ✓ Found 3 results for 'programming'

1. Score: 19.8000 - Some(String("Rust Programming"))
2. Score: 6.6000 - Some(String("JavaScript Web Development"))
3. Score: 6.6000 - Some(String("Python for Data Science"))

=== Fuzzy Search === ✓ Found 3 results for 'progamming' (typo)

1. Score: 1.6500 - Some(String("Rust Programming"))
2. Score: 0.5500 - Some(String("JavaScript Web Development"))
3. Score: 0.5500 - Some(String("Python for Data Science"))

=== Field-Specific Search === ✓ Found 4 results in title/description

1. Score: 4.0000 Title: Some(String("Machine Learning Basics")) Matched:
   ["title", "title.value", "description", "description.value"]
2. Score: 2.0000 Title: Some(String("Python for Data Science")) Matched:
   ["description", "description.value"]
3. Score: 1.0000 Title: Some(String("Database Design")) Matched:
   ["description.value", "description"]
4. Score: 1.0000 Title: Some(String("Rust Programming")) Matched:
   ["description", "description.value"]

=== Weighted Search === ✓ Found 2 results with field weights

1. Score: 23.1000 - Some(String("Python for Data Science"))
2. Score: 3.3000 - Some(String("Machine Learning Basics"))

=== Advanced Search Options === ✓ Found 1 results with stemming

1. Score: 19.8000 - Some(String("Database Design"))

=== Search with Limit === ✓ Limited to 2 results (requested 2)

1. Score: 19.8000 - Some(String("Rust Programming"))
2. Score: 6.6000 - Some(String("JavaScript Web Development"))

=== Cleanup === ✓ Deleted collection

✓ All search operations completed successfully Execution time: 1ms Compiling
ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.86s Running
`target/debug/examples/client_schema_management` === ekoDB Schema Management
Example ===

=== Creating Collection with Schema === ✓ Created collection
'schema_client_rust' with schema

=== Inserting Valid Documents === ✓ Inserted user 1:
Some(String("I8cRyjFJEeFwl4iEfrgWG8Hh0S2SCHqfa8YZhaj7UvBWWgm0zkIAvB3wNwgbQ_RXOS8Z8GcGRQWsoneRykVUOw"))
✓ Inserted user 2:
Some(String("f5PWjETd6vaC5dYYVXXzeZrou9NpWfBLzcoLtTE6q4ByH4qrPXWZIWawVlDEsZHUVLpG_y1I3TDlGySa2A_Jng"))

=== Getting Schema === ✓ Schema for schema_client_rust:

- email: String (required)
- title: String (required)
- status: String
- age: Integer

=== Listing Collections === ✓ Total collections: 9 Sample:
["scripts__ek0_bench", "users", "websocket_test", "test_collection",
"test_accounts"]

=== Cleanup === ✓ Deleted collection

✓ All schema management operations completed successfully Compiling
ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.85s Running
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

- Laptop ($1200) by Alice Johnson
- Mouse ($25) by Alice Johnson

=== Example 5: Complex join with multiple conditions === ✓ Found 2 users with
example.com emails

- Alice Johnson (alice@example.com): Building A
- Bob Smith (bob@example.com): Building B

=== Cleanup === ✓ Deleted test collections

✓ Join operations example completed successfully Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.78s Running
`target/debug/examples/client_chat_basic` === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
tPrJggglSl_8sgRKtUhO7O74BjWJNwkAuJV493jQ-\_07K21Qrsf_Wtnqy-NM69uuKl475EErbtDdaRjEwITSqA

=== Sending Chat Message === Message ID:
\_fLlrrQCE1NZPGgHjsR9isXPVDWdRfwYN61cCVXfSoeYsQZpJ3jMM7FgbwEXxUTlVRmse_LzyDGMWQaKdSYTog

=== AI Response === Response 1: I'm sorry, but the provided context does not
contain any information about what ekoDB is or the features it has. Please
provide relevant details so I can assist you better.

=== Context Used (3 snippets) ===

Snippet 1: Collection: client_chat_basic_rust Score: 0.6333 Matched Fields:
["category", "content", "title"] Record: Object {"category": Object {"type":
String("String"), "value": String("features")}, "content": Object {"type":
String("String"), "value": String("The chat feature allows you to query your
database using natural language and get AI-powered responses with relevant
context.")}, "id":
String("6A_VVuQvVrW3Bg3kat3gmib3A6DSQCT638iTBcKsMM4p-p9vU9RfiwupNp2gP0hVmXVHpSwaUeb-LqRRbcEbmw"),
"title": Object {"type": String("String"), "value": String("AI Chat
Integration")}}

Snippet 2: Collection: client_chat_basic_rust Score: 0.5222 Matched Fields:
["title", "content"] Record: Object {"category": Object {"type":
String("String"), "value": String("documentation")}, "content": Object {"type":
String("String"), "value": String("ekoDB is a high-performance database with
intelligent caching, real-time capabilities, and AI integration.")}, "id":
String("Xz67Ms8m5el6bkGU3VgxhRuW_FzqUiQrOGu8XeDGoIXV8xk26ftlB3yc1xxNeV4ryW8yuiz5kn_NKIccUCtg9A"),
"title": Object {"type": String("String"), "value": String("Introduction to
ekoDB")}}

Snippet 3: Collection: client_chat_basic_rust Score: 0.5222 Matched Fields:
["title", "category", "content"] Record: Object {"category": Object {"type":
String("String"), "value": String("features")}, "content": Object {"type":
String("String"), "value": String("ekoDB supports full-text search, vector
search, and hybrid search with automatic context retrieval.")}, "id":
String("DkLGJyI8AYqsOztfD2qjO03vMiCl6QEamEp4jQoWeR4LJKbGk_pVzKY2knZskulGSbi8UdBdmFBHj-YryNxNBw"),
"title": Object {"type": String("String"), "value": String("Search Features")}}

Execution Time: 1843ms

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully Compiling ekodb-examples v0.1.0
(/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev` profile
[unoptimized + debuginfo] target(s) in 0.84s Running
`target/debug/examples/client_chat_sessions` === ekoDB Chat Session Management
Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
O-C14UTsuD3FnQGVrtwgnk_yWQyQeK4v8dWxNrZVg2b8khWx1wOfk0BL6Wb_SIvHmEVaPQrg9QF7lQ2N9bD6Sw
=== Sending Messages === ✓ Message 1 sent Response: Apologies, but the provided
context does not contain specific information about available products. Could
you please provide more details or a different context?

✓ Message 2 sent Response: Apologies, but the context provided does not include
information about a specific product or its price. Could you please provide more
details or specify the product you're interested in?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
5FbD3yFozPz4KlpTG3GYR_JcEiZwYeqGMEbUhU5h4HBS-AJdiXGJmM5uwTIZ9eZG7MSvajbhrsX0aDALlwWMGw
Parent:
O-C14UTsuD3FnQGVrtwgnk_yWQyQeK4v8dWxNrZVg2b8khWx1wOfk0BL6Wb_SIvHmEVaPQrg9QF7lQ2N9bD6Sw

=== Listing Sessions === ✓ Found 3 sessions Session 1:
5FbD3yFozPz4KlpTG3GYR_JcEiZwYeqGMEbUhU5h4HBS-AJdiXGJmM5uwTIZ9eZG7MSvajbhrsX0aDALlwWMGw
(Untitled) Session 2:
O-C14UTsuD3FnQGVrtwgnk_yWQyQeK4v8dWxNrZVg2b8khWx1wOfk0BL6Wb_SIvHmEVaPQrg9QF7lQ2N9bD6Sw
(Untitled) Session 3:
tPrJggglSl_8sgRKtUhO7O74BjWJNwkAuJV493jQ-\_07K21Qrsf_Wtnqy-NM69uuKl475EErbtDdaRjEwITSqA
(Untitled)

=== Getting Session Details === ✓ Session details retrieved Messages: 4

=== Deleting Branch Session === ✓ Deleted branch session:
5FbD3yFozPz4KlpTG3GYR_JcEiZwYeqGMEbUhU5h4HBS-AJdiXGJmM5uwTIZ9eZG7MSvajbhrsX0aDALlwWMGw

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully Compiling
ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.82s Running
`target/debug/examples/client_chat_advanced` === ekoDB Advanced Chat Features
Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
\_T3_8lH7TohnoVnnw5eFvq2jgGIuSZGVgxk1JxDbFUIM-Wlrk4SeWcANcPHi2zztTsbAsCBszCUZ18yhr-GLyg

=== Sending Initial Message === ✓ Message sent Response: I'm sorry, but the
context provided does not contain any specific product information. Could you
please provide more details or specify the type of products you are interested
in?

✓ Second message sent === Feature 1: Regenerate AI Response === ✓ Message
regenerated New response: Apologies, but the context provided does not include
information on the price of ekoDB. Please provide more details or consider
checking the official website or contacting the sales department of the company
for this information.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
fcQkL9x-SJ9VPI8YEYzJjcZh5CrpXeQAC8VL_ax38Gh3b2yMZsDgCkIYqONB9cTxXJiRFa5iA6AsALNAhrtwzw
✓ Sent message in second session ✓ Sessions merged successfully Total messages
in merged session: 4

=== Feature 5: Delete Message === ✓ Message deleted

✓ Messages remaining: 3

=== Cleanup === ✓ Deleted session ✓ Deleted collection

✓ All advanced chat features demonstrated successfully! Compiling ekodb-examples
v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust) Finished `dev`
profile [unoptimized + debuginfo] target(s) in 1.07s Running
`target/debug/examples/client_functions` === ekoDB Rust Client - Scripts Example
===

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved:
SnlWgtqfcNSXWsCNb78n95abVFQ5ajA8mCQv_73Qiv52GU1WLIMKX-Rgx1i-g3yZkc5AEHqbx97LoqA58sUMnA
📊 Found 20 active users

📝 Example 2: Parameterized Script

✅ Script saved:
AGmPL40U638iLD-uuVWZ24Uxo0Limu2QA7zhQ4dq2fFq9zMS3bpeIep9zmctgDdUgdqVZzzbRIeu7f5gfonDGg
📊 Found 20 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved:
WIKfAnmJDS8lSbrld42rpKEcjNT77VBC36Latarh_dEpupKcZniG90KKxaOaBIfGfklqlP-UC7P_mdnLOlSHUg
📊 Statistics: 2 groups

📝 Example 4: Script Management

📋 Total scripts: 6 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed! ✅ [32mRust client examples complete![0m
