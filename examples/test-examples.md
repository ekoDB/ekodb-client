make test-examples 🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
Compiling ekodb-examples v0.1.0
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

✅ All examples completed! ✅ [32mRust client examples complete![0m ✅ [32mAll
Rust integration tests complete![0m 🧪 [36mRunning Python examples (direct
HTTP/WebSocket)...[0m ✓ Authentication successful

=== Insert Document === Inserted: {'id':
'0b2eEPAdIv7qIW0-GAmSc1mYz2D3oDCdAeKVOHxAamT-3DDDbIddV6V-g09iGSed5PQdOYRwkTeUInWh-6OkFg'}

=== Find by ID === Found: {'value': {'type': 'Integer', 'value': 42}, 'active':
{'type': 'Boolean', 'value': True}, 'name': {'type': 'String', 'value': 'Test
Record'}, 'id':
'0b2eEPAdIv7qIW0-GAmSc1mYz2D3oDCdAeKVOHxAamT-3DDDbIddV6V-g09iGSed5PQdOYRwkTeUInWh-6OkFg'}

=== Find with Query === Found documents: 1

=== Update Document === Updated: {'value': {'type': 'Integer', 'value': 100},
'active': {'type': 'Boolean', 'value': True}, 'id':
'0b2eEPAdIv7qIW0-GAmSc1mYz2D3oDCdAeKVOHxAamT-3DDDbIddV6V-g09iGSed5PQdOYRwkTeUInWh-6OkFg',
'name': {'type': 'String', 'value': 'Updated Record'}}

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully ✓ Authentication successful

=== Inserting Test Data === ✓ Inserted test record:
dTPCdp_g6cKAoqQLPCx-Y-rgKg5040O1X1Y4EBATWa3og1GDIm2dwdlm5-E8NZFc3KliH7RMsq4MNmmrvv0NMQ

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"-g3odd0twQrSsVSLVsISeu0mVnyzhoYUmFFRGNOwE63t_8qQwJM0iIFFDVgCcXfWKD0yRacu6KH6p4ml-i7KgQ",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"dTPCdp_g6cKAoqQLPCx-Y-rgKg5040O1X1Y4EBATWa3og1GDIm2dwdlm5-E8NZFc3KliH7RMsq4MNmmrvv0NMQ",
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

=== KV Get === Retrieved value: {'username': 'john_doe', 'userId': 123}

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: {'name': 'Product 1', 'price': 29.99}
cache:product:2: {'price': 39.989999999999995, 'name': 'Product 2'}
cache:product:3: {'name': 'Product 3', 'price': 49.989999999999995}

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Authentication successful

=== Create Collection (via insert) === Collection created with first record:
7ZPXWPNsWpp9BZP4CS11VddoAS7-F221UQVm0ooqsm4FE7W1pdjzxNB-3SNZ7jMqnaEH-Xz7dcAQLEPzK0aMpw

=== List Collections === Total collections: 11 Sample collections:
['scripts__ek0_bench', 'users', 'websocket_test', 'test_collection',
'test_accounts']

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: False

✓ All collection management operations completed successfully ✓ Authentication
successful

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
2fCysQdUJFn0uP-9i0XW6lYgZR4Q_ERcSVTLRd_Er4SMaehbgfzghf5MZDFI4bXmOGTcwjv83OFyaPx6aIuLUg

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully ✓ Authentication successful

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
9eq61ac2iCRMbYxMysrUM2_TBAnNRKQ2Q9_51KZQJj9hglzp1dvA4XrncYocbBvLjKXK5Yq2x1ptsOm6W6EBxw

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 2 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully 🚀 ekoDB Scripts Example
(Python/HTTP)

✓ Authentication successful

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved:
vGns7ObnUR4Inrl0NJbcIKJPR31HJ1XVIGCRP1k\_-aVvyy5oXe3OhZ5Tn5y2LZZpltO3_VHeY_Zilf4eC-AR8w
📊 Found 15 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved:
lBIGFL-WwCZOlPefhXzp1VHBEG6TjF-\_B_aVZYg2KvnSq7M_9K66AQrcHEFDWQtSPh0GqKG7zfXiix4rkQr1_g
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved:
QYyd0nJaYutOdP8eP5O3EBdXlBgmjdT4luoXHgEGSWxtXVv_gpg5XcMQYRB5i-pF7JxY_BgOtlrmbKUXS3oWJg
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups {'count':
15, 'status': 'inactive', 'avg_score': 50.0, 'max_score': 90} {'avg_score':
60.0, 'count': 15, 'max_score': 100, 'status': 'active'}

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
Cq5gvK8s6rA9DPlMJBP5G1oMAJNAIpzv5Iis8owE1-Zp-t2-cluEIdAdliv8BbJ_j7XASIv0o7VzHLpQRHZfKw
Created Bob: $500 - ID:
XU2qas4x9isSzJEB1VBcg2ZVT98m_1pu7CuU4HoTEj7ut8iLS4jvcDYXs0mK8yGgzhnVyJFE5RllbGN2jBX77g

=== Example 1: Begin Transaction === Transaction ID:
666ea42c-98b8-4fa3-9972-d7cac1e7ac84

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: Active Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: $800 Bob: $700

=== Example 5: Rollback === New transaction:
f69690d1-b9d1-47bc-bc6c-d24dd73c2987 Updated Bob: $700 → $600 (in transaction) ✓
Transaction rolled back Bob after rollback: $700

=== Cleanup === ✓ All transaction examples completed === Setup: Creating Test
Accounts === Created Alice: $1000 - ID:
B65yf_FfotM9o7R--oTjvbWWU3kXXiKFNCFWcC_Pm936PkcDmvpO0LnWLxfY42nP-U9Cdz6DkGquGYUZPJN0Zw
Created Bob: $500 - ID:
qHxoz-sycuD78DBGkT3-AcN4fhBq3wLwzvqBDeLXbGuBx5dW0rplh-Z3kjnth0vj35Z81-lnRCx0wU_jiwAIgQ

=== Example 1: Begin Transaction === Transaction ID:
961be601-6b30-40eb-9a27-a8c8f72bff00

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
lirolGvIZxIUEaU3e8QB2_nKim8RTha8RGdXD9z6o_j-yC9zC6JC5v-yHG2_2c3wJSqMG7ONzWtZQTiBWbq3RQ
Created Bob: $500 - ID:
zHvExxWEusF21XwN20W7JvkOlVtntg2R7hyMzY9mOhBbvcQoaxCvCJDzTiFntgJ4qL64AH--ldQriMRstm40ig

=== Example 1: Begin Transaction === Transaction ID:
52af1949-f4fe-48e0-a874-5ddf04ead181

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: Active Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: $800 Bob: $700

=== Example 5: Rollback === New transaction:
cbbd0e64-069a-48be-aed2-e32113d56e6d Updated Bob: $700 → $600 (in transaction) ✓
Transaction rolled back Bob after rollback: $700

=== Cleanup === ✓ All transaction examples completed === Setup: Creating Test
Accounts === Created Alice: $1000 - ID:
f9yFv-7icuSkB2nxC5eojWlf2uuNKuztgA2j80KjlaQSnQCbtcakUDijEQKtMOTEsqWwtXKhBCLpPRL79ikDdg
Created Bob: $500 - ID:
NasvWFpz_nmGcBfEBAQX_fHWYMSS99NrF0vD0PWj6x5bjG3YGiBCuXqhoRnieT0GWECFaWKNWRzu0GwSOjzoSw

=== Example 1: Begin Transaction === Transaction ID:
3b1fb635-9f67-45db-b9f4-b889e79d872e

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: $800 Bob: $700

=== Cleanup === ✓ Deleted test accounts

🔄 Switching to DURABLE mode... ✓ Config updated: durable_operations=true

============================================================ 🔥 TEST 3: Durable
Mode (durable=true) ============================================================

=== Setup: Creating Test Accounts === Created Alice: $1000 - ID:
xvX_oLMEDAK9xX07wTSvijpMOoNzZJtJOG93zTVWWIDAhEp5VzGu7ELgziTPiltcNUhbch7Oowx3kDqiluKvvQ
Created Bob: $500 - ID:
22g3XVXEdJp1K_hxCnWkbCl65Cga0lzORAF-UnmbqKcnxvfe3VPUxn3EQyzfc5q7lA8uWevm9M3cndNr7XEgKw

=== Example 1: Begin Transaction === Transaction ID:
f77d79bc-8e46-4fdf-a623-b58bd9134c9c

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: Active Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: $800 Bob: $700

=== Example 5: Rollback === New transaction:
7ca8aec7-8553-4c64-9501-83f809e06d63 Updated Bob: $700 → $600 (in transaction) ✓
Transaction rolled back Bob after rollback: $700

=== Cleanup === ✓ All transaction examples completed === Setup: Creating Test
Accounts === Created Alice: $1000 - ID:
-U4W2xdGETJe4wYV2guTkoGG9rRDgRWwsk4Wh-d-Fz0JXcR9Vyfn0OFfGIcogZwwDToei055G44X9emOetOOgg
Created Bob: $500 - ID:
OLDsoa6CcepY8iWzF3K2qx64slrMJ42nPpc2Rs6X7YAatjKoIOsuFsM7lYQLSb430Vt5mGr8PVLKpTFy0UgT4Q

=== Example 1: Begin Transaction === Transaction ID:
164a4e72-ef74-4b2c-9fe6-c0ce590beff1

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
admraV-x3hTt_IdYco_8frhrU0lE4WeJrO3kMPTojGxIrmvD7KCG8lFtVaKO99HZ8hx-Nm5bfksfmhbMT55mIw

2️⃣ Calling Script (Insert + Verify)... ✅ Script executed: 2 Functions ⏱️
Execution time: 0ms

3️⃣ Verification Results: ✅ Found 1 record(s) 📋 User ID:
U0xGwy8D_VpO9-z9AfIRWJv27T6T_xOUInPGtpinzuDpKCgqbYjcme7CH6Cw-NbWSyCFk83ckdZFAc5OdnytcQ
📋 Name: {'type': 'String', 'value': 'Alice Smith'} 📋 Email: {'type': 'String',
'value': 'alice@example.com'} 📋 Status: {'type': 'String', 'value': 'pending'}
📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================ 📝 Script 2:
Query + Update + Verify
============================================================

1️⃣ Saving Script... ✅ Script saved:
22O_tRh1bWye0Jklc-14DUgGur7xIDmZBnGG3g4V0R704gZDpBAAfP8zjOFwZ4QFm7D4MMc3rdQCe0phhZl9Ig

2️⃣ Calling Script (Query + Update + Verify)... ✅ Script executed: 3 Functions
⏱️ Execution time: 0ms

3️⃣ Verification Results: ✅ Found 1 record(s) 📋 Status updated to: {'value':
'active', 'type': 'String'} 📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================ 📝 Script 3:
Query + Update Credits + Verify
============================================================

1️⃣ Saving Script... ✅ Script saved:
U99W7dp_QibtDjX8MBnjMKNQTbco1T-eKHjCFkNi76L5MM3Lm5hh2ZFlhYOS8z29wx7GgMVBFdd3iZtBNEMZTg

2️⃣ Calling Script (Query + Update Credits + Verify)... ✅ Script executed: 3
Functions ⏱️ Execution time: 0ms

3️⃣ Verification Results: ✅ Found 1 record(s) 📋 Credits updated to: {'value':
0, 'type': 'Integer'} 📋 Status: {'type': 'String', 'value': 'active'} 📋 Name:
{'type': 'String', 'value': 'Alice Smith'}

============================================================ 📝 Script 4: Query
Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script... ✅ Script saved:
Xoi5FvjMTsD5DcqC3oi9zJyvRym4pNzdsFC65WvlYKjqAFFg2afnumVcZaempB8aW8J1DCPd_U475eCiJCgkLw

2️⃣ Calling Script (Query + Delete + Verify)... ✅ Script executed: 3 Functions
⏱️ Execution time: 0ms

3️⃣ Verification Results: 📊 Before delete: Record existed (verified by first
Query) ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================ 🧹 Cleanup
============================================================ ✅ Deleted script:
admraV-x3hTt_IdYco_8... ✅ Deleted script: 22O_tRh1bWye0Jklc-14... ✅ Deleted
script: U99W7dp_QibtDjX8MBnj... ✅ Deleted script: Xoi5FvjMTsD5DcqC3oi9... ✅
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
default Finished `release` profile [optimized] target(s) in 0.13s 📦 Built wheel
for abi3 Python ≥ 3.8 to
/Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.4.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m Processing
./ekodb-client-py/target/wheels/ekodb_client-0.4.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client Successfully installed
ekodb-client-0.4.0 ✅ [32mPython client package built and installed![0m 🧪
[36mRunning Python client library examples...[0m ✓ Client created (token
exchange happens automatically)

=== Insert Document === Inserted: {'id':
'kq9-EKebeSyav56wOwfdXcvYf4RRuHH5g3GTb33CFabH2AZiiKgTQQJGWcmGxLcPGSOmS1nkFLzHHCI8zMjBYQ'}

=== Find by ID === Found: {'id':
'kq9-EKebeSyav56wOwfdXcvYf4RRuHH5g3GTb33CFabH2AZiiKgTQQJGWcmGxLcPGSOmS1nkFLzHHCI8zMjBYQ',
'active': {'type': 'Boolean', 'value': True}, 'name': {'value': 'Test Record',
'type': 'String'}, 'value': {'value': 42, 'type': 'Integer'}}

=== Find with Query === Found documents: 1

=== Update Document === Updated: {'id':
'kq9-EKebeSyav56wOwfdXcvYf4RRuHH5g3GTb33CFabH2AZiiKgTQQJGWcmGxLcPGSOmS1nkFLzHHCI8zMjBYQ',
'value': {'type': 'Integer', 'value': 100}, 'name': {'value': 'Updated Record',
'type': 'String'}, 'active': {'type': 'Boolean', 'value': True}}

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
"DyzHSvf4H3h32qrWE5q3zHFfOfvIbByCtgPReew5YjRulc3oMDE0tDGF4tunCs3ZVQtKxMPy7KGwrqp9VcVepQ"

=== List Collections === Total collections: 10 Sample collections:
['scripts__ek0_bench', 'websocket_test', 'client_collection_management_python',
'test_collection', 'test_accounts']

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: False

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
Cp47foUmRq3xplYGPWdRlWupssVh4loKGC85orhYjchcrxqp1wnA-wzk6-6qvvIEoPCbWxkZU8Vu8JmPxOSPPg

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
eAWeemSz3153_uBCPiEQPFfCsWNzDorRdPNkIWw4Zr3DEohC4zstyzVwjte43jJv4PYdRc0JQtDosJxtHHB79A

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
khuNDoY2TIBByOsr2JmjM3xlvSFFOVVPz8p7saUkx4LYIyruUMpWu7gsjivuV9jnBEPi1JgaAMRqlDwebmaNqw

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
QRZ_Q0EhMgAKVfAbMhzB-koQE48VkdwoWUR0Fh1lZY1AQEwzPUp6FyrpoJF7uC0PbAL8es25LqrhIUNQYgxMOQ

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
1. Score: 25.740, Matched: name.value, email.value, name, email
1. Score: 12.540, Matched: name.value, name

1. Fuzzy search (typo tolerance): Found 4 results with fuzzy matching
1. Score: 26.400, Matched: title.value, bio.value, title, bio
1. Score: 26.400, Matched: title.value, bio, bio.value, title
1. Score: 26.400, Matched: title, title.value, bio.value, bio
1. Score: 26.400, Matched: bio, bio.value, title, title.value

1. Search with field weights: Found 4 results with weighted fields
1. Score: 39.600, Matched: bio.value, bio, title.value, title
1. Score: 39.600, Matched: title.value, title, bio, bio.value
1. Score: 39.600, Matched: title.value, bio, title, bio.value
1. Score: 39.600, Matched: title.value, bio, bio.value, title

1. Search with minimum score threshold: Found 2 results with score >= 0.3
1. Score: 13.200, Matched: bio, bio.value
1. Score: 13.200, Matched: bio, bio.value

1. Search with stemming and exact match boosting: Found 2 results (matches: run,
   running, runs)
1. Score: 13.200, Matched: bio.value, bio
1. Score: 13.200, Matched: bio, bio.value

1. Vector search (semantic search): Found 3 semantically similar documents
1. Score: 0.763
1. Score: 0.743
1. Score: 0.731

1. Hybrid search (text + vector): Found 3 results using hybrid search (text +
   vector)
1. Score: 1.712, Matched: content.value, title.value, title, content
1. Score: 1.120, Matched: title, content.value, content, title.value
1. Score: 0.534, Matched:

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

- Laptop ($1200) by Alice Johnson
- Mouse ($25) by Alice Johnson

5. Complex join with multiple conditions: Found 2 users with example.com emails:

- Alice Johnson (alice@example.com): Building A
- Bob Smith (bob@example.com): Building B

=== Cleanup === ✅ Deleted test collections

✅ Join operations examples completed! === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
jC9n-MLxQZb3--dNvpyK_CMNOi8WJDW2Mm_iNNCGVoKg8l-xArdqbsUhIQcVe-iVPo8jjolL5q5-41gIUXepAQ

=== Sending Chat Message === Message ID:
zH47drYScXP2CHGvijs8NhS6Txy6kopIuU2Y9c6CcnYXIk-Dcc0Z79h7lD_R-IRfTaJmvltz1hNmLKatMa6z3Q

=== AI Response === I'm sorry, but the provided context doesn't include specific
information about the names and prices of the products. Could you please provide
more details or a different context?

Execution Time: 1616ms

=== Token Usage === Prompt tokens: 316 Completion tokens: 33 Total tokens: 349

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
s2WcgbfvYLkuAG0qe_pB3ULJxvn0w8ZdFc3XIhqmOeNvr4HdqkK_r0Jj9ctHuD2YITZSbmJy5CUrv8Ky4CQFDg

=== Sending Messages === ✓ Message 1 sent Response: I'm sorry, but the context
provided doesn't include specific information about any available products. Can
you please provide more details?

✓ Message 2 sent Response: Apologies, but I can't provide the information you're
looking for because the context doesn't specify any product or price. Could you
please provide more details?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
Fh9yMPjqKSHbTSfjflNvaXZGmC5XUwFaAmBxFJ7f7akUnp31EyAcxY9d--DsdTT9wjjsEZmNSebZuKVq_nOKWw
Parent:
s2WcgbfvYLkuAG0qe_pB3ULJxvn0w8ZdFc3XIhqmOeNvr4HdqkK_r0Jj9ctHuD2YITZSbmJy5CUrv8Ky4CQFDg

=== Listing Sessions === ✓ Found 6 sessions Session 1:
Fh9yMPjqKSHbTSfjflNvaXZGmC5XUwFaAmBxFJ7f7akUnp31EyAcxY9d--DsdTT9wjjsEZmNSebZuKVq_nOKWw
(Untitled) Session 2:
s2WcgbfvYLkuAG0qe_pB3ULJxvn0w8ZdFc3XIhqmOeNvr4HdqkK_r0Jj9ctHuD2YITZSbmJy5CUrv8Ky4CQFDg
(Untitled) Session 3:
jC9n-MLxQZb3--dNvpyK_CMNOi8WJDW2Mm_iNNCGVoKg8l-xArdqbsUhIQcVe-iVPo8jjolL5q5-41gIUXepAQ
(Untitled) Session 4:
fcQkL9x-SJ9VPI8YEYzJjcZh5CrpXeQAC8VL_ax38Gh3b2yMZsDgCkIYqONB9cTxXJiRFa5iA6AsALNAhrtwzw
(Untitled) Session 5:
O-C14UTsuD3FnQGVrtwgnk_yWQyQeK4v8dWxNrZVg2b8khWx1wOfk0BL6Wb_SIvHmEVaPQrg9QF7lQ2N9bD6Sw
(Untitled) Session 6:
tPrJggglSl_8sgRKtUhO7O74BjWJNwkAuJV493jQ-\_07K21Qrsf_Wtnqy-NM69uuKl475EErbtDdaRjEwITSqA
(Untitled)

=== Deleting Branch Session === ✓ Deleted branch session:
Fh9yMPjqKSHbTSfjflNvaXZGmC5XUwFaAmBxFJ7f7akUnp31EyAcxY9d--DsdTT9wjjsEZmNSebZuKVq_nOKWw

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
npi8nCqi4VoJXNetA3-XiriOJBVGGlTWhT79lXh-V4bR-\_BO3djsVLjXlPHnMYka41WqzI7K-1rSC_YAXaTmBQ

=== Sending Initial Message === ✓ Message sent Response: I'm sorry, but the
context provided does not contain specific information about any available
products. Could you please provide more detailed information?

✓ Second message sent

Debug: Found 4 messages Debug: First message keys: dict_keys(['id', 'role',
'token_usage', 'updated_at', 'content', 'created_at', 'chat_id']) Debug: First
message role: {'value': 'assistant', 'type': 'String'} === Feature 1: Regenerate
AI Response === ✓ Message regenerated New response: Answer: I'm sorry, but the
context provided does not have information on the price of ekoDB.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
HJUs621HzLEuG-m3hdCO5FpoC-fowMhZgf8gzr4v6o8alHcz6CEqa3dEurUFur_j7nF6j77ahqyjSO2SnTo54A
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
BFXx9qCk8JLY0YYkHj_bIEHjPWfsJruHhXjb3y8xZuhbCc98LEQ6fzfgdp9T16DKp762-8NepBHVX_BzeuHXTQ
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved:
6HEgY5dOU6*1GBFg_UMqky5hKTRqR9EOgVOt1AxLaZ9jTLobIDLN3Jia8cyBXmVYS*-Kyi-1gEl0h6bldVSFQA
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved:
tGNLO_sIHTvRw7CJ2DaQ1uhhmEVs3CohSXcj20azKjdmH7gNFdgZzWIpbMMd7hS-5cz37kBzz6EHm6lb70eYNQ
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
9CFu1ShlhxCCNeUZhp6VNfbEvuT1QgyBIRhBkSE9mU8MuoRtwJkjH5_sPzyy31fKOO0jKgAMPyEQUkf62cF_JA
📊 Found 2 product groups {'avg_price': 575.6666666666666, 'category':
'Electronics', 'count': 3} {'avg_price': 474.0, 'category': 'Furniture',
'count': 2} ⏱️ Execution time: 0ms

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
map[id:9vlanQ6aMsaHABJ8RZvWMnK_L6DSbC4IhpWJouDyLaPCNT6odjajOTyAcbDmXqXdqGDwGUF8F3KHgA5Cr7XLzw]

=== Find by ID === Found: map[active:map[type:Boolean value:true]
id:9vlanQ6aMsaHABJ8RZvWMnK_L6DSbC4IhpWJouDyLaPCNT6odjajOTyAcbDmXqXdqGDwGUF8F3KHgA5Cr7XLzw
name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query === Found 1 documents

=== Update Document === Updated: map[active:map[type:Boolean value:true]
id:9vlanQ6aMsaHABJ8RZvWMnK_L6DSbC4IhpWJouDyLaPCNT6odjajOTyAcbDmXqXdqGDwGUF8F3KHgA5Cr7XLzw
name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully [32m✓ simple_crud.go completed
successfully[0m [34m === Running simple_websocket.go ===[0m ✓ Authentication
successful

=== Inserting Test Data === ✓ Inserted test record:
BqL4lJ96z7t2MSMg5wG1jQZCCV83GzgtuZAhxdm5IqpxPXen3tQ-AFbQUd2RqqJMrJvIgqdSsuxR9whS4c3EnQ

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"-g3odd0twQrSsVSLVsISeu0mVnyzhoYUmFFRGNOwE63t_8qQwJM0iIFFDVgCcXfWKD0yRacu6KH6p4ml-i7KgQ",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"dTPCdp_g6cKAoqQLPCx-Y-rgKg5040O1X1Y4EBATWa3og1GDIm2dwdlm5-E8NZFc3KliH7RMsq4MNmmrvv0NMQ",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"BqL4lJ96z7t2MSMg5wG1jQZCCV83GzgtuZAhxdm5IqpxPXen3tQ-AFbQUd2RqqJMrJvIgqdSsuxR9whS4c3EnQ",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } } ] }, "type": "Success" } ✓ Retrieved 3
record(s) via WebSocket

✓ WebSocket example completed successfully [32m✓ simple_websocket.go completed
successfully[0m [34m === Running batch_operations.go ===[0m ✓ Authentication
successful

=== Batch Insert === Batch insert response: map[failed:[]
successful:[gg2Dl8pdfNxfoqpVe4U7aR4NZeiLDgyk3--B-vJiI1N31GkqeiKj2oD8DjqQ6Fu8fU5gv2mIBKQTu4WRY-OufA
JrwbAhVLD0dBqNlBYvExwsG3DSjIXsNe4N-MI7ruZkkJQpRuBLTSjNn_XXx9QCxuVlKEnsVPlGwP2E6dCibr4w
MiR42aqQK0A-hmQJym1nARBSZcaDwJ9Zjao54s3PSIlTx3dHMwFV2d_n0V95ubt9BWdYm6yrUy43hDLHZRgGNA
fhDk9KxKz8JS1py8S6Ws_xtXUU7bc7Q_8tVVo4HXDKLqitacebQF5IffB-ZBnYunXxDSBZJXcL_SsGeZ6d1Qtg
FjAaEz6N4aNeJnvw40XavXcsDwdz1L4RSzpwNyEZb8J4kJVeNysrdcNKEpTVEMF46CwweXx0Y7pe6f_x2voQZA]]
✓ Batch insert completed

=== Creating test records for update/delete === Created 3 test records

=== Batch Update === Batch update response: map[failed:[]
successful:[Vn3_yRQuls8_LNg01V2ENFKMQgn1MfuyyzpdPHb1nktsaZipXUkh2wV-b782v3k5HT3w3DmJf9gEu57P5hyLOQ
abDtBVHhReLvylKMdbZ-1q1buXWO44mB75una7DFBeD9LfaTTc1Vge4J4TZ0zRMuoXtzQgAeQb_5J6vUQbOHkg
g9FzybQg62dOlcBHLakEyzaNWIE1DJeCYjzmjfMrV3f7oCwLgDqSiOjX9lm5Dj_ufLSQ0aMcBtqwOVr-XMG8-g]]
✓ Batch update completed

=== Batch Delete === Batch delete response: map[failed:[]
successful:[Vn3_yRQuls8_LNg01V2ENFKMQgn1MfuyyzpdPHb1nktsaZipXUkh2wV-b782v3k5HT3w3DmJf9gEu57P5hyLOQ
abDtBVHhReLvylKMdbZ-1q1buXWO44mB75una7DFBeD9LfaTTc1Vge4J4TZ0zRMuoXtzQgAeQb_5J6vUQbOHkg
g9FzybQg62dOlcBHLakEyzaNWIE1DJeCYjzmjfMrV3f7oCwLgDqSiOjX9lm5Dj_ufLSQ0aMcBtqwOVr-XMG8-g]]
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
mmZQ0nAkOA2fQljWvecW88FnHdbzp4HtIUul5xax6RShkSRjYubgWlq5sB0SFqJAcwPERTBCb_TqH7N8tORdSA

=== List Collections === Total collections: 15 Sample collections:
[scripts__ek0_bench users websocket_test schema_documents_client_py
test_collection test_accounts demo_collection chat_configurations__ek0_bench
schema_employees_client_py chat_messages__ek0_bench schema_users_client_py
schema_products_client_py batch_users ttl_cache ws_ttl_test]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully [32m✓
collection_management.go completed successfully[0m [34m === Running
document_ttl.go ===[0m ✓ Authentication successful

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
bjiNq6VTU5S_XiyZhlMF8FX-Yalie8YnM2_q-flRY2jfyVuthKYrNZ8z7CfJBLDnLVWBpc8m1NVfOT7eWc8M0A

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 3 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully [32m✓ document_ttl.go
completed successfully[0m [34m === Running websocket_ttl.go ===[0m ✓
Authentication successful

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
lkaZFaYbG6VAnr4rvP87NdQsf19oYVT-cJ_0MblO3LynBMM5hMS0wLZMQ230BeqdI8Ss-jhwhUVslMjiE16a8Q

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 3 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully [32m✓ websocket_ttl.go completed
successfully[0m [34m === Running http_functions.go ===[0m 🚀 ekoDB Scripts
Example (Go/HTTP)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved:
iwbJ0d76OiujCSSSvgxJr0kyt0piNkrWwW5J-3iHI7g_AA5HZ5uaJc4cRdSNpDYNFXv6AJsj1HCn8XKHkOdUmw
📊 Found 20 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved:
ZF4AePH9LYjxLzJSkWBUYsDsAWuBLgEgotFf2_3fNCrvQkpx6aIKHEIbDiibyiKVpUBIH_OSlTs_y3ULbkXPlg
📊 Page 1: Found 3 users (limit=3, skip=0) 📊 Page 2: Found 3 users (limit=3,
skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved:
pti5TLMVfQJibZp1TTxlcYR-mNVSKYM6n-iGkzVjT6t4X\_-oDLMdK_uSrYCigUbgOMFE9AIokuHu2kr8_Dm78A
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
WciYRyYSZxmiSiPwcjeYAQ2EWJfS--3pUkSIEdQXNIvXQf2tyg6J4p9PgpjKCuMvU99Xz6FwiY1BNJ-EDXXGKA
Created Bob: $500 - ID:
BDRCyJl0swY8CqJh4jCbD0jcFA6MI1O8JapHEjAGgJn0HRSWvMMmm8QAHSOXAMb5fM5CxYO_6rrbPjT9-9eXtQ

=== Example 1: Begin Transaction === Transaction ID:
e3babe26-44df-46fb-9be4-1581b5b356a6

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: Active Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: 800 Bob: 700

=== Example 5: Rollback === New transaction:
066af6fe-bcab-41d8-b468-35442447082e Updated Bob: $700 → $600 (in transaction) ✓
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
IY9wJGiB4VRvPRPPjbr6KtzxaAAdqWPqeF6-vTwJhLW8pqqJFEV35j9IVJMcaQY394pZyB6GJkMmHHDLhR-MoA

2️⃣ Calling Script (Insert + Verify)... ✅ Script executed: 2 Functions ⏱️
Execution time: 0ms

3️⃣ Verification Results: ✅ Found 1 record(s) 📋 User ID:
ul0oVIrrcvwrUDNPKZnA06OPKNw4j-HskRPT015sXlz6zh3QPC15pQXAQzUyeIq5dvd1EvBPZJSm4p8f8IQEzw
📋 Name: map[type:String value:Alice Smith] 📋 Email: map[type:String
value:alice@example.com] 📋 Status: map[type:String value:pending] 📋 Credits:
map[type:Integer value:0]

============================================================ 📝 Script 2:
Query + Update + Verify
============================================================

1️⃣ Saving Script... ✅ Script saved:
frUNamokrr36u4-5ckj5maAxdNBiFH9bHgMXXuKBWjC72hWNN-6DaR9VpBKcLTvHrtnyoRgmmFivl8WnV07ELw

2️⃣ Calling Script (Query + Update + Verify)... ✅ Script executed: 3 Functions
⏱️ Execution time: 0ms

3️⃣ Verification Results: ✅ Found 1 record(s) 📋 Status updated to:
map[type:String value:active] 📋 Name: map[type:String value:Alice Smith]

============================================================ 📝 Script 3:
Query + Update Credits + Verify
============================================================

1️⃣ Saving Script... ✅ Script saved:
z0TT3M2idpAxVz8Xwag7917RHMKHRhDrwDeinqGkV1qurMHrQ6l-7NZA-DGIchpVth4VmX4FukEWImghQ5tkBg

2️⃣ Calling Script (Query + Update Credits + Verify)... ✅ Script executed: 3
Functions ⏱️ Execution time: 0ms

3️⃣ Verification Results: ✅ Found 1 record(s) 📋 Credits updated to:
map[type:Integer value:0] 📋 Status: map[type:String value:active] 📋 Name:
map[type:String value:Alice Smith]

============================================================ 📝 Script 4: Query
Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script... ✅ Script saved:
IxZ7RDDFhgJ4Y_2GpjzC3ArKbOkW0EOYlYPfX9EtQ6yZLpp5kXIrz6t80t23SupW78zJ13kg2lnOXjqufGAYSw

2️⃣ Calling Script (Query + Delete + Verify)... ✅ Script executed: 3 Functions
⏱️ Execution time: 0ms

3️⃣ Verification Results: 📊 Before delete: Record existed (verified by first
Query) ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================ 🧹 Cleanup
============================================================ ✅ Deleted script:
IY9wJGiB4VRvPRPPjbr6... ✅ Deleted script: frUNamokrr36u4-5ckj5... ✅ Deleted
script: z0TT3M2idpAxVz8Xwag7... ✅ Deleted script: IxZ7RDDFhgJ4Y_2GpjzC... ✅
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
map[id:Y4FzG8uyP45XWISlFXISu5DpOX1eFX4twi-OE0MHlz4TEp-5O59q93u75SIBRlhIRDMq8F9itvaq3eoVg2D4mQ]

=== Find by ID === Found: map[active:map[type:Boolean value:true]
id:Y4FzG8uyP45XWISlFXISu5DpOX1eFX4twi-OE0MHlz4TEp-5O59q93u75SIBRlhIRDMq8F9itvaq3eoVg2D4mQ
name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query === Found documents: 1

=== Update Document === Updated: map[active:map[type:Boolean value:true]
id:Y4FzG8uyP45XWISlFXISu5DpOX1eFX4twi-OE0MHlz4TEp-5O59q93u75SIBRlhIRDMq8F9itvaq3eoVg2D4mQ
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
MOLANVyaDmHYemwccSQXbliSbRlrBj-0M9fSKTOakJzne2pzbglDwHgWmR4ygewhcp-VAOLie3nYNW7KV8b6TA

=== List Collections === Total collections: 14 Sample collections:
[scripts__ek0_bench websocket_test schema_documents_client_py test_collection
test_accounts]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
NwcM4Bqs7PXayfY5ZWpRC8kL9jt4KO3ph32rieXLIEOrEOT-JKRk5et7I-haVHVZ7nyxrr6DmWQuqTRIJBDxAQ

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
nUZ6BM7W78kpkG_8OTKNmVwReUsMRCm4GMd5tEyGzKFVDIY5V7yE6ePhR3EkPA5y7tEY4UUlscFIpL6U6TIkyg

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
70Swl71W1URTRWt7PccyWmiEDqP6pElRM\_\_QFW16lvJeL3hVO1aSe5fzt1tiTSeFC2UbAXTVo8ZrS_xLbsq2JQ

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
qW5bTHahLp2GiZQ7XL7J7tzD6E8J4j8F6MOvZ4h16-bC1jhtBpY4b-b5tt16i88bLGrpEyV_7QHN5FNWC2HZYg

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
l4ZyFBZ3lJRgkSZ4Yf_N1b9Rfvz-q_wNsegBtgwaPpi0efGToI0ktyU3xujoPSikQhQid-3f9P65Q_6az3ddpA

=== Sending Chat Message === Message ID:
uYXeU7zTN_dGo-Bxb_YGAhoM2x-2hh-82A_3kW4b6xovA4bkXNPazqV-i_3g4uEiqjNtQZLgeqDxp7qU5gz6aQ

=== AI Response === I'm sorry, but the context provided doesn't contain specific
information about the names and prices of the products. Could you please provide
more details or a different context?

=== Context Used (3 snippets) === Snippet 1: map[collection:client*chat_basic_go
matched_fields:[description] record:map[description:map[type:String value:A
high-performance database product with AI capabilities]
id:4Kft-1K6hAO2NHv8rfOIFSVOTGISxUZESNDjq40Hcqa5iYQ1D9UMCLnNwWC9HMylJLIzbSbVkd4CHFD0w2dVqA
name:map[type:String value:ekoDB] price:map[type:Integer value:99]]
score:0.1111111111111111] Snippet 2: map[collection:client_chat_basic_go
matched_fields:[description] record:map[description:map[type:String value:Fully
managed cloud database service product]
id:nKnyWoDbki0rp3erBt7UO_30y_t0J6HwAGNWIWIwOqZGmSOnSMLtxrWnrNTU2tzqdLLyKe-EYTyQrmL4URQ8lw
name:map[type:String value:ekoDB Cloud] price:map[type:Integer value:499]]
score:0.1111111111111111] Snippet 3: map[collection:client_chat_basic_go
matched_fields:[description] record:map[description:map[type:String
value:Enterprise edition product with advanced features]
id:eTwyqyf4qNrhhC812ji9qYkYKncsvq2CYWyPopNTSMMRiIj*-UgvX7nLhVKI9TvhEf65gvnu55vsgdF5NB_ekA
name:map[type:String value:ekoDB Pro] price:map[type:Integer value:299]]
score:0.1111111111111111]

Execution Time: 1163ms

=== Token Usage === Prompt tokens: 318 Completion tokens: 33 Total tokens: 351

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
k4j8sFpfurOcPhiKbDk0sPYzER1PhQ5lKRQOqxTQUqdIR8jGnrA8E6AXhZQQ4dvea02zgRAZYEf7cvpbnGslHg

=== Sending Messages === ✓ Message 1 sent Response: I'm sorry, but the context
provided doesn't contain specific information about any available products.
Could you please provide more details or specify what type of products you're
looking for?

✓ Message 2 sent Response: Apologies, but the context provided doesn't contain
information about the price of any product. Could you please specify the product
you are interested in?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
7u7g-QCuonF2p15QW3ST3umpoFurqkjx2rncah_hsR94xEGgnHmFvp0sBNqRW-buzKubHNBdTkyxN1TkrFeg1Q
Parent:
k4j8sFpfurOcPhiKbDk0sPYzER1PhQ5lKRQOqxTQUqdIR8jGnrA8E6AXhZQQ4dvea02zgRAZYEf7cvpbnGslHg

=== Listing Sessions === ✓ Found 9 sessions Session 1:
7u7g-QCuonF2p15QW3ST3umpoFurqkjx2rncah_hsR94xEGgnHmFvp0sBNqRW-buzKubHNBdTkyxN1TkrFeg1Q
(Untitled) Session 2:
k4j8sFpfurOcPhiKbDk0sPYzER1PhQ5lKRQOqxTQUqdIR8jGnrA8E6AXhZQQ4dvea02zgRAZYEf7cvpbnGslHg
(Untitled) Session 3:
l4ZyFBZ3lJRgkSZ4Yf_N1b9Rfvz-q_wNsegBtgwaPpi0efGToI0ktyU3xujoPSikQhQid-3f9P65Q_6az3ddpA
(Untitled) Session 4:
HJUs621HzLEuG-m3hdCO5FpoC-fowMhZgf8gzr4v6o8alHcz6CEqa3dEurUFur_j7nF6j77ahqyjSO2SnTo54A
(Untitled) Session 5:
s2WcgbfvYLkuAG0qe_pB3ULJxvn0w8ZdFc3XIhqmOeNvr4HdqkK_r0Jj9ctHuD2YITZSbmJy5CUrv8Ky4CQFDg
(Untitled) Session 6:
jC9n-MLxQZb3--dNvpyK_CMNOi8WJDW2Mm_iNNCGVoKg8l-xArdqbsUhIQcVe-iVPo8jjolL5q5-41gIUXepAQ
(Untitled) Session 7:
fcQkL9x-SJ9VPI8YEYzJjcZh5CrpXeQAC8VL_ax38Gh3b2yMZsDgCkIYqONB9cTxXJiRFa5iA6AsALNAhrtwzw
(Untitled) Session 8:
O-C14UTsuD3FnQGVrtwgnk_yWQyQeK4v8dWxNrZVg2b8khWx1wOfk0BL6Wb_SIvHmEVaPQrg9QF7lQ2N9bD6Sw
(Untitled) Session 9:
tPrJggglSl_8sgRKtUhO7O74BjWJNwkAuJV493jQ-\_07K21Qrsf_Wtnqy-NM69uuKl475EErbtDdaRjEwITSqA
(Untitled)

=== Getting Session Details === ✓ Session details retrieved Messages: 4

=== Deleting Branch Session === ✓ Deleted branch session:
7u7g-QCuonF2p15QW3ST3umpoFurqkjx2rncah_hsR94xEGgnHmFvp0sBNqRW-buzKubHNBdTkyxN1TkrFeg1Q

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
UX1imcSRdqm2Hpaw9DlUmdygstj2TR-sTLm-wEiV_xmrEWWuy5oOZNrnLKfyvCw4Y7UjqvEwvsdtHhQE42UZvQ

=== Sending Initial Message === ✓ Message sent Response: I'm sorry, but the
context provided does not contain information about any specific products. Could
you please provide more details or try again?

✓ Second message sent

=== Feature 1: Regenerate AI Response === ✓ Message regenerated New response:
Answer: I'm sorry, but the context provided doesn't include information on the
price of ekoDB.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
mwE_UXqsFXQSRhOFX3Nt4hIcexq-iFNBRqP5H0ZO8L9T_DRrXwPd7ew4cQqi6J3KXSiL_LHAuUz3c936tjUDPQ
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
1. Score: 0.753
1. Score: 0.749
1. Score: 0.717

1. Hybrid search (text + vector): Found 3 results using hybrid search (text +
   vector)
1. Score: 2.687
1. Score: 1.501
1. Score: 0.300

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
eri0xIZvKalNs7um6fL6JnFwD2A_MR46mlgFF-CMM9VfzsShiZBcdRqSvan4kKVpCPZf4SoT1PlXUejACJsqGw
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

added 1 package, removed 1 package, and audited 13 packages in 641ms

3 packages are looking for funding run `npm fund` for details

found 0 vulnerabilities ✓ Client created (token exchange happens automatically)

=== Insert Document === Inserted: { id:
'mNXDtq_O56G5JgxDURUKp_THNkyxEhoHZ6ZiX5svtDrULzaDK1mziJNMMR1siAp0uWO3Y5auf3OkSFE3ZdJS4g'
}

=== Find by ID === Found: { active: { value: true, type: 'Boolean' }, id:
'mNXDtq_O56G5JgxDURUKp_THNkyxEhoHZ6ZiX5svtDrULzaDK1mziJNMMR1siAp0uWO3Y5auf3OkSFE3ZdJS4g',
value: { value: 42, type: 'Integer' }, name: { type: 'String', value: 'Test
Record' } }

=== Find with Query === Found documents: 1

=== Update Document === Updated: { value: { type: 'Integer', value: 100 },
active: { value: true, type: 'Boolean' }, id:
'mNXDtq_O56G5JgxDURUKp_THNkyxEhoHZ6ZiX5svtDrULzaDK1mziJNMMR1siAp0uWO3Y5auf3OkSFE3ZdJS4g',
name: { type: 'String', value: 'Updated Record' } }

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
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
FCfORvZic6GzUM2chbEgYKZoHETZchIiyOPvto2v0pFD07aJhPd-7DP57tlDtNrn_sWixABELEX_M1LUyJdc3A

=== List Collections === Total collections: 18 Sample collections:
scripts\_\_ek0_bench,websocket_test,schema_documents_client_py,schema_users_client_go,schema_employees_client_go

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
NMPRK-KpjcR1RipIx0-MKlqNAJAiW99WFB7MJ1gCYJxOAWcquYp7_EeVmB6\_\_WOI5TiP1XGNd5RNVWx5C8N3NA

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
zxJ91zOWW76kXW7JjA9poUil-O_KbjvJFPcBOPt7YaPIZ0HozDnpwP38QR8_Od6V8ZHce4RMLQ927odziIABog

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
iYJgu7ge66cqydk_c8Hdiypi_iWSEy50Tob14RUl8685TnYKOcyIShjHVyRWlHfNOfiRf_8fpzhMnmEAeBleGA

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
5Zk04fBZ7bIzjRlRdcNgxxA-as7obxyo8nWIe7zzFu0jq9K01A9FV5BTVYpORj1frcUcJBsMoZe_T0_e406Vzw

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
SngZo3l4_KRCF_AHJMU2c6QrBn_b2A61Btx4CnhiPTKepSlxp95VdfMtrOQYBo7JAlO7NjqXbDeHkoHLKBIY-Q

=== Sending Chat Message === Message ID:
AlEmyvlJelEIBHQRPh2AarXZ0NIWayjcRcxmd60i2zdNrVTQuilAzLPQNFHwoSftwW_LYzgtMRH471XmhJpHXw

=== AI Response === I'm sorry, but the context provided does not include
specific information about the names or prices of any products. Could you please
provide more detailed information?

=== Context Used (3 snippets) === Snippet 1: { collection:
'client_chat_basic_ts', record: { name: { value: 'ekoDB Pro', type: 'String' },
id:
'ROJ7nld8cpEgVPjgwLNdW3LslcT7Rk5A7-L372j_zceb0JvgOtP6yVEtmr2Z-sxdTRUlGE8GDKGMvuY9-PYkKA',
price: { value: 299, type: 'Integer' }, description: { value: 'Enterprise
edition product with advanced features', type: 'String' } }, score:
0.1111111111111111, matched_fields: [ 'description' ] } Snippet 2: { collection:
'client_chat_basic_ts', record: { price: { type: 'Integer', value: 99 }, id:
'FLrLO6Ckk_b0uzOsdtDAD31AOArTSO3TQ3jBeWQBHFLCeL6Cr0dPxTNFx3lmo8ogMhbZu8ufQk_41NbBEA45Kw',
description: { type: 'String', value: 'A high-performance database product with
AI capabilities' }, name: { type: 'String', value: 'ekoDB' } }, score:
0.1111111111111111, matched_fields: [ 'description' ] } Snippet 3: { collection:
'client_chat_basic_ts', record: { description: { type: 'String', value: 'Fully
managed cloud database service product' }, id:
'\_enem1mL3KL1jmvF4tvFUyyThGG0X2oci4vsmvUiOFB3d_hi9fuZQY2Xxq2RguvcSd9KDRCwRLQNNHq34ahn2w',
price: { type: 'Integer', value: 499 }, name: { value: 'ekoDB Cloud', type:
'String' } }, score: 0.1111111111111111, matched_fields: [ 'description' ] }

Execution Time: 1125ms

=== Token Usage === Prompt tokens: 312 Completion tokens: 30 Total tokens: 342

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
sgngjXb6_bp3zDAGpPWCq7z6WzaoKH9CC9eysgNqtmhdqnm1UOr7FVJAPkajpQpc4EldHKAoN-ZV1aKTy50mlg

=== Sending Messages === ✓ Message 1 sent Response: I'm sorry, but the context
provided does not include specific information about any products. Could you
please provide more details?

✓ Message 2 sent Response: As an assistant, I'm sorry but I cannot provide the
price as the context does not specify any product. Could you please provide more
details or specify the product you're referring to?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
TTGkBG2oCVvHaIu1FV9mR2s_dPjniPYvWF8_W_nQr4-LiilCNV5CZ9gD6ztByPueHnfxjB9oWxhKlYUjZ2iWeg
Parent:
sgngjXb6_bp3zDAGpPWCq7z6WzaoKH9CC9eysgNqtmhdqnm1UOr7FVJAPkajpQpc4EldHKAoN-ZV1aKTy50mlg

=== Listing Sessions === ✓ Found 10 sessions Session 1:
TTGkBG2oCVvHaIu1FV9mR2s_dPjniPYvWF8_W_nQr4-LiilCNV5CZ9gD6ztByPueHnfxjB9oWxhKlYUjZ2iWeg
(Untitled) Session 2:
sgngjXb6_bp3zDAGpPWCq7z6WzaoKH9CC9eysgNqtmhdqnm1UOr7FVJAPkajpQpc4EldHKAoN-ZV1aKTy50mlg
(Untitled) Session 3:
SngZo3l4_KRCF_AHJMU2c6QrBn_b2A61Btx4CnhiPTKepSlxp95VdfMtrOQYBo7JAlO7NjqXbDeHkoHLKBIY-Q
(Untitled) Session 4:
mwE_UXqsFXQSRhOFX3Nt4hIcexq-iFNBRqP5H0ZO8L9T_DRrXwPd7ew4cQqi6J3KXSiL_LHAuUz3c936tjUDPQ
(Untitled) Session 5:
k4j8sFpfurOcPhiKbDk0sPYzER1PhQ5lKRQOqxTQUqdIR8jGnrA8E6AXhZQQ4dvea02zgRAZYEf7cvpbnGslHg
(Untitled) Session 6:
l4ZyFBZ3lJRgkSZ4Yf_N1b9Rfvz-q_wNsegBtgwaPpi0efGToI0ktyU3xujoPSikQhQid-3f9P65Q_6az3ddpA
(Untitled) Session 7:
HJUs621HzLEuG-m3hdCO5FpoC-fowMhZgf8gzr4v6o8alHcz6CEqa3dEurUFur_j7nF6j77ahqyjSO2SnTo54A
(Untitled) Session 8:
s2WcgbfvYLkuAG0qe_pB3ULJxvn0w8ZdFc3XIhqmOeNvr4HdqkK_r0Jj9ctHuD2YITZSbmJy5CUrv8Ky4CQFDg
(Untitled) Session 9:
jC9n-MLxQZb3--dNvpyK_CMNOi8WJDW2Mm_iNNCGVoKg8l-xArdqbsUhIQcVe-iVPo8jjolL5q5-41gIUXepAQ
(Untitled) Session 10:
fcQkL9x-SJ9VPI8YEYzJjcZh5CrpXeQAC8VL_ax38Gh3b2yMZsDgCkIYqONB9cTxXJiRFa5iA6AsALNAhrtwzw
(Untitled)

=== Getting Session Details === ✓ Session details retrieved Messages: 4

=== Deleting Branch Session === ✓ Deleted branch session:
TTGkBG2oCVvHaIu1FV9mR2s_dPjniPYvWF8_W_nQr4-LiilCNV5CZ9gD6ztByPueHnfxjB9oWxhKlYUjZ2iWeg

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
5nAF76hbhd4_GP47_LsQ5OFFMV_5oxUVNAM8ZkQVhOJj95jVCxvvNEFGBwfRa9ydXjGkLL4mjUGbu_TXBJt0eg

=== Sending Initial Message === ✓ Message sent Response: I'm sorry, but the
context provided does not contain information about any specific products. Could
you please provide more details?

✓ Second message sent

=== Feature 1: Regenerate AI Response === ✓ Message regenerated New response:
Apologies, but the context provided does not include information about the price
of ekoDB. I recommend checking the product's official website or contacting
their customer service for accurate pricing details.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
4tm8p8PHqserwqg0kX_R7SbM_FrLwlMB3wkaw4zz-h2VoLtY65QidsxhWYR_FHbGtONOjUZS2i0V-1F_c53How
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
1. Score: 25.740, Matched: name.value, name, email, email.value
1. Score: 12.540, Matched: name, name.value

1. Fuzzy search (typo tolerance): Found 4 results with fuzzy matching
1. Score: 26.400, Matched: bio, title, title.value, bio.value
1. Score: 26.400, Matched: title.value, title, bio, bio.value
1. Score: 26.400, Matched: title.value, bio.value, title, bio
1. Score: 26.400, Matched: title.value, title, bio, bio.value

1. Search with field weights: Found 4 results with weighted fields
1. Score: 39.600, Matched: title.value, bio, bio.value, title
1. Score: 39.600, Matched: title.value, bio.value, bio, title
1. Score: 39.600, Matched: bio.value, bio, title.value, title
1. Score: 39.600, Matched: title.value, title, bio, bio.value

1. Search with minimum score threshold: Found 2 results with score >= 0.3
1. Score: 13.200, Matched: bio, bio.value
1. Score: 13.200, Matched: bio.value, bio

1. Search with stemming and exact match boosting: Found 1 results (matches:
   work, working, worked)
1. Score: 13.200, Matched: bio, bio.value

1. Vector search (semantic search): Found 3 semantically similar documents
1. Score: 0.759, Matched:
1. Score: 0.755, Matched:
1. Score: 0.730, Matched:

1. Hybrid search (text + vector): Found 3 results using hybrid search (text +
   vector)
1. Score: 2.704, Matched: title, content, content.value, title.value
1. Score: 1.502, Matched: content.value, title.value, content, title
1. Score: 0.292, Matched:

1. Case-sensitive search: Found 1 results (case-sensitive)
1. Score: 26.400, Matched: skills.value, bio, skills, bio.value

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

- Bob Smith: Sales
- Alice Johnson: Engineering

2. Join with filtering: Found 1 users in Engineering:

- Alice Johnson: Building A

3. Join with user profiles: Found 2 users with profile data:

- Bob Smith: Sales Manager
- Alice Johnson: Senior Software Engineer

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
lWAL6q3xJ01hpSZE0-EpdQxccTWR7uZ3fcgu1xQgp4fZJvmTVzl8vH4CwSGgX5tRVgrC-N0ZqHPMw6Et1E4r6A
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved:
yO6aO86yLtjKxit7Gv9NlhKXbarIF4ir9X4GuPZMnhVDpkaEsgQdezu6Hn1QYSUvSPWhpNxfqSYFN9lnadlL1A
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved:
lA9VOZiBP9eQct43aD3l1Y8PhYNYi4i4E1sfDiXigISAhN1Lyc4k8YpXBaNUvDB3EyQPkV7XcEz5TBQvnZefaw
📊 Statistics: 2 groups {"status":"active","count":5,"avg_score":60}
{"status":"inactive","avg_score":50,"count":5}

📝 Example 4: Script Management

📋 Total scripts: 18 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed! 🚀 ekoDB TypeScript Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data... ✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved:
MWdyl_vfgk0x8uS78OixHacMQtypIMScmGUAL6Io-prBqVItQ1IssV5G78Z54KVFFDkg4w0KxKU5eGUErMAPGQ
📊 Found 2 product groups {"count":2,"category":"Furniture","avg_price":474}
{"category":"Electronics","avg_price":575.6666666666666,"count":3} ⏱️ Execution
time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved 📊 Found 5 products ⏱️ Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved 📊 Found 2 categories {"count":3,"category":"Electronics"}
{"category":"Furniture","count":2} ⏱️ Execution time: 0ms

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

added 1 package, removed 1 package, and audited 9 packages in 644ms

1 package is looking for funding run `npm fund` for details

found 0 vulnerabilities [34m ╔════════════════════════════════════════╗[0m [34m║
ekoDB JavaScript Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m [34m === Checking Server
Connection ===[0m (node:64616) [DEP0040] DeprecationWarning: The `punycode`
module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) [32m✓
Server is ready[0m [34m === Getting Authentication Token ===[0m [32m✓
Authentication successful[0m [33m === Running 10 Examples ===[0m [34m ===
Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_crud.js
===[0m ✓ Authentication successful

=== Insert Document === (node:64619) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) Inserted:
{ id:
'hNPNIzSkA3dosq1VfDcFwQDk3abC1-8N9gR00fxls9JcjMUIKM-53-URbFo0X_4ZLemiMemQG2a6HO40W9MYBQ'
}

=== Find by ID === Found: { active: { type: 'Boolean', value: true }, name: {
value: 'Test Record', type: 'String' }, value: { type: 'Integer', value: 42 },
id:
'hNPNIzSkA3dosq1VfDcFwQDk3abC1-8N9gR00fxls9JcjMUIKM-53-URbFo0X_4ZLemiMemQG2a6HO40W9MYBQ'
}

=== Find with Query === Found documents: 1

=== Update Document === Updated: { value: { type: 'Integer', value: 100 }, id:
'hNPNIzSkA3dosq1VfDcFwQDk3abC1-8N9gR00fxls9JcjMUIKM-53-URbFo0X_4ZLemiMemQG2a6HO40W9MYBQ',
active: { value: true, type: 'Boolean' }, name: { value: 'Updated Record', type:
'String' } }

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully [32m✓ simple_crud.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_websocket.js
===[0m ✓ Authentication successful

=== Inserting Test Data === (node:64631) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Inserted
test record:
h55LGL5gfIitB9FnEuNni-5AAMeQnY9oEbJRjPp16SM1o4GjNGmxOcG4osdtIoM-AUC5A05DlZgFw_aCRDHnLw

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"h55LGL5gfIitB9FnEuNni-5AAMeQnY9oEbJRjPp16SM1o4GjNGmxOcG4osdtIoM-AUC5A05DlZgFw_aCRDHnLw",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"-g3odd0twQrSsVSLVsISeu0mVnyzhoYUmFFRGNOwE63t_8qQwJM0iIFFDVgCcXfWKD0yRacu6KH6p4ml-i7KgQ",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"dTPCdp_g6cKAoqQLPCx-Y-rgKg5040O1X1Y4EBATWa3og1GDIm2dwdlm5-E8NZFc3KliH7RMsq4MNmmrvv0NMQ",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"BqL4lJ96z7t2MSMg5wG1jQZCCV83GzgtuZAhxdm5IqpxPXen3tQ-AFbQUd2RqqJMrJvIgqdSsuxR9whS4c3EnQ",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } } ] }, "type": "Success" } ✓ Retrieved 4
record(s) via WebSocket

✓ WebSocket example completed successfully [32m✓ simple_websocket.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/batch_operations.js
===[0m ✓ Authentication successful

=== Batch Insert === (node:64644) [DEP0040] DeprecationWarning: The `punycode`
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

=== KV Set === (node:64647) [DEP0040] DeprecationWarning: The `punycode` module
is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Set key:
session:user123

=== KV Get === Retrieved value: { loginTime: '2025-12-20T08:37:59.904Z', userId:
123, username: 'john_doe' }

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: { name: 'Product 1', price: 29.99 }
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully [32m✓ kv_operations.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/collection_management.js
===[0m ✓ Authentication successful

=== Create Collection (via insert) === (node:64650) [DEP0040]
DeprecationWarning: The `punycode` module is deprecated. Please use a userland
alternative instead. (Use `node --trace-deprecation ...` to show where the
warning was created) Collection created with first record:
8pl1Zl3kcC3XKUFVHaHD5PNCo0x76P9VupUQE8DbrmXG2ZhijxTCie3pu2MwZpRQ28a6_B7uS87qBduUt8QPPg

=== List Collections === Total collections: 23 Sample collections: [
'schema_employees_client_ts', 'scripts__ek0_bench', 'users', 'websocket_test',
'schema_documents_client_py' ]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully [32m✓
collection_management.js completed successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/document_ttl.js
===[0m ✓ Authentication successful

=== Insert Document with TTL (1 hour) === (node:64666) [DEP0040]
DeprecationWarning: The `punycode` module is deprecated. Please use a userland
alternative instead. (Use `node --trace-deprecation ...` to show where the
warning was created) ✓ Inserted document:
KRXAl0sTEFgTJBEVwb4kazPDphi1TKfhOlrZEoZomk8_GPMH5Fmsb525jw73attfVuBuYkmmVy1JQtW6tNJDLA

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 4 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully [32m✓ document_ttl.js
completed successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/websocket_ttl.js
===[0m ✓ Authentication successful

=== Insert Test Data with TTL === (node:64669) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Inserted
document with TTL:
ykYYNcVFf0AtwDhTEEkyW2B7XTTIMMBuVnKuJQqzWFAHYZt0-H4Bde7tT8tmuqAYdOYEpb-mqsv_7oQ_YCpkbg

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 4 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully [32m✓ websocket_ttl.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/http_functions.js
===[0m 🚀 ekoDB Scripts Example (JavaScript/HTTP)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved:
4YW-\_V7-glSzc3snaNlrDmuLUEnooVPqicFJeHhX_8g2QyrG3-APH5MlFvhEbX75pME-S0MKpIStC2dVN3kbAw
📊 Found 20 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved:
U7onT-GFtLxXRL_dnZETli-qrEmwCMx21696xrzOLC2FE1yjTPDT_Md3lDZQARKW7amWvL0i3MYviSJyITMGdQ
📊 Page 1: Found 3 users (limit=3, skip=0) 📊 Page 2: Found 3 users (limit=3,
skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved:
iDzmB1WHCpEZjRPzuhT2IKZKEmcNYIhAB7tlbthAWHT7yFadFnLdatufKl3Sv8gG43LuSMjPY8Y4YkXySzLoMQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
{"max_score":100,"avg_score":60,"count":10,"status":"active"}
{"status":"inactive","max_score":90,"avg_score":50,"count":10}

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
qyhKTuLeB8JysOzjwQmVJVtfovqQzgnrvq96Zl*c-jxYN0hy44tSzrSBDflcDKtF3jM4rUMdvVaJzkhAFB6i9A
Created Bob: $500 - ID:
sISkOk0szdD8SlGs9yvDfx-yo0SeczB1gflSubvSgAPVj8BcFR9*-rM0_UuBK-z5-jdchGfKv0F0-uVXNxXCGw

=== Example 1: Begin Transaction === Transaction ID:
b098620c-e8f2-4d41-bb5d-26c1fbac5818

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: Active Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: 800 Bob: 700

=== Example 5: Rollback === New transaction:
8f6aac08-cbaa-40d0-af5d-6080c1ed9ee2 Updated Bob: $700 → $600 (in transaction) ✓
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
inSM-Xo2tnvT0NUKyF20CzT4m24LWKUBz5RafmJw8UA-Uytj9HUraYMf1qed_wyesNvSa636uRA1K3Lni_T71w

2️⃣ Calling Script (Insert + Verify)... ✅ Script executed: 2 Functions ⏱️
Execution time: 0ms

3️⃣ Verification Results: ✅ Found 1 record(s) 📋 Name: [object Object] 📋 Email:
{"value":"alice@example.com","type":"String"} 📋 Status: [object Object] 📋
Credits: [object Object]

============================================================ 📝 Script 2:
Query + Update + Verify
============================================================

1️⃣ Saving Script... ✅ Script saved:
b3YFcCHr9IGnU_8CvoRCyiN8csIgOuCVUxhtXL5J9JXuPKLam4eN-xPlML0P-tqX4oduYW-uCcA1u8xUjEZR_Q

2️⃣ Calling Script (Query + Update + Verify)... ✅ Script executed: 3 Functions
⏱️ Execution time: 0ms

3️⃣ Verification Results: ✅ Found 1 record(s) 📋 Status updated to:
{"type":"String","value":"active"} 📋 Name: {"type":"String","value":"Alice
Smith"}

============================================================ 📝 Script 3:
Query + Update Credits + Verify
============================================================

1️⃣ Saving Script... ✅ Script saved:
urDR2k-beKAVbj1qC3Wh7-w4h1_ZWyxkZcADsAQXZi8beGRZR7yaKk8-8u9AZRz5dZJaJEDauvZUAVpRXAyGPQ

2️⃣ Calling Script (Query + Update Credits + Verify)... ✅ Script executed: 3
Functions ⏱️ Execution time: 0ms

3️⃣ Verification Results: ✅ Found 1 record(s) 📋 Credits updated to:
{"value":0,"type":"Integer"} 📋 Status: {"type":"String","value":"active"} 📋
Name: {"type":"String","value":"Alice Smith"}

============================================================ 📝 Script 4: Query
Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script... ✅ Script saved:
mX86OE3OZlC1My5OpMktjp31MXrCVhfJkuty-1ZVli\_\_vS5L4DdI_4JpFEQKWsRvEM7tAL9RruqgCBlSuKWyiQ

2️⃣ Calling Script (Query + Delete + Verify)... ✅ Script executed: 3 Functions
⏱️ Execution time: 0ms

3️⃣ Verification Results: 📊 Before delete: Record existed (verified by first
Query) ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================ 🧹 Cleanup
============================================================ ✅ Deleted script:
inSM-Xo2tnvT0NUKyF20... ✅ Deleted script: b3YFcCHr9IGnU_8CvoRC... ✅ Deleted
script: urDR2k-beKAVbj1qC3Wh... ✅ Deleted script: mX86OE3OZlC1My5OpMkt... ✅
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

up to date, audited 7 packages in 790ms

found 0 vulnerabilities

> @ekodb/ekodb-client@0.4.0 build tsc

✅ [32mTypeScript client built![0m 🧪 [36mRunning JavaScript client library
examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 632ms

1 package is looking for funding run `npm fund` for details

found 0 vulnerabilities ✓ Client created (token exchange happens automatically)

=== Insert Document === Inserted: { id:
'LGoFNZrgWiS_a1NsKLgP-7VpjDWlnn0IqWpik_3oLBpgWy2qWoKXmK1fVkmVw5OE0KuJs5DJew0nAvDSMhWOTQ'
}

=== Find by ID === Found: { active: { type: 'Boolean', value: true }, value: {
type: 'Integer', value: 42 }, name: { type: 'String', value: 'Test Record' },
id:
'LGoFNZrgWiS_a1NsKLgP-7VpjDWlnn0IqWpik_3oLBpgWy2qWoKXmK1fVkmVw5OE0KuJs5DJew0nAvDSMhWOTQ'
}

=== Find with Query === Found documents: 1

=== Update Document === Updated: { active: { type: 'Boolean', value: true }, id:
'LGoFNZrgWiS_a1NsKLgP-7VpjDWlnn0IqWpik_3oLBpgWy2qWoKXmK1fVkmVw5OE0KuJs5DJew0nAvDSMhWOTQ',
value: { value: 100, type: 'Integer' }, name: { type: 'String', value: 'Updated
Record' } }

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
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
Fp47CCeJ_dRIB1F2epTs6zOxaccHXisLBsYgZKcWceKqy31t3X5dJSJFEWDO0n9UI_pwGBoAwMPEQLNGZZ-Nzw

=== List Collections === Total collections: 22 Sample collections:
schema_employees_client_ts,scripts\_\_ek0_bench,websocket_test,schema_documents_client_py,schema_users_client_go

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
FTCz-L5hIUnmerkP8Yln8yf7Y5EuX7vmL9pmBc92YQwCBrrSEUmaTDyrUcc7HgKiEXEG-AhoUczIEJwdgiJxNw

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
Mke5N5SEfYvj9kavBTlgu0zvZmJKbGVRfRnnM1r3DtrARgQprdfstYTp5FsbwHzV_8g8Mcojqqw9xfg0IHQDbg

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
2IErvAw4WfDmWVKT_FkQIOI6xEce9fn3gC1E0qdHHq-JC_uw0o3RHAThwXW6v_SGjCCBXWh-\_Ap_3r4quKE0LA

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
Poq1QSvPETIqTYC_vNW1IixNHHvAyH28YpruybbDRHQDam21hjfEbAbHwrVUNxpQafPXU3Mth6_YCiMeWttq2w

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration 🚀 ekoDB Scripts Example (JavaScript/HTTP)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved:
j1fQOXk3WHwpA2vOmuCWY2QsBjbDwca0wvaFq9ainR9kjyBNaXOovPxgKEBDtmD5hoQP9XHiX2yK6EKw2-Jb-A
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved:
9uiRsalCyekSSHYMJAwQEwlTZyXkBKPelqcdIY_o1bGUWns5PDj7WT8pA20rJl32eT2yQSe27vsnWazoLE9VMw
📊 Page 1: Found 3 users (limit=3, skip=0) 📊 Page 2: Found 2 users (limit=3,
skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved:
cp_d--rZWzOtyKPxobl6HVE3nCjMqiCNJL-cpx2CPUa8IIrZc0fQJRudxyB8R9PjyA8KaQmRQYHVw7mNjzb_Qg
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
{"count":5,"max_score":90,"avg_score":50,"status":"inactive"}
{"avg_score":60,"max_score":100,"count":5,"status":"active"}

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
3sz1K9ABi3KB2qDlJbXBIhiyT0u2vQTjUK1f3epYosODL9Ntt4dvcSouvFg_jFSELaO4H4sIpE5QBD2ZGUbkFQ
📊 Found 20 records ⏱️ Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved 📊 Found 20 users (limited) ⏱️ Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved 📊 Statistics: 2 groups
{"status":"active","count":10,"avg_score":60}
{"status":"inactive","avg_score":50,"count":10} ⏱️ Execution time: 0ms

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

✅ Script saved 📊 Category analysis (2 categories): Electronics: Products: 6 |
Stock: 232 | Avg Rating: ⭐4.52 Furniture: Products: 4 | Stock: 43 | Avg Rating:
⭐4.26

⏱️ Total execution time: 0ms 📈 Pipeline stages:

📝 Example 5: Project Specific Fields

✅ Script saved 📊 Product summaries (10 items, showing first 3):

1.  Webcam HD - $119 (⭐4.5)
2.  USB-C Cable - $19 (⭐4.3)
3.  Laptop Pro - $1299 (⭐4.8) ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Cleanup complete

✅ All advanced script examples completed! 🚀 ekoDB CRUD Scripts Example

📋 Setting up test data... ✅ Created 10 test users

📝 Example 1: List All Users

✅ Script saved 📊 Found 10 users ⏱️ Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Script saved 📊 User counts by status: active: 7 users inactive: 3 users ⏱️
Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Script saved 📊 Average scores by role: user: 70.0 (7 users) admin: 20.0 (3
users) ⏱️ Execution time: 0ms

📝 Example 4: Top Users by Score

✅ Script saved 📊 Users (showing first 5 of 0): ⏱️ Execution time: 0ms

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
3.  Natural Language Processing (AI)
4.  Database Design Principles (Database)
5.  Getting Started with ekoDB (Database) ⏱️ Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved 📊 Documents by category: AI: 2 documents Database: 3 documents
⏱️ Execution time: 0ms

📝 Example 3: Select Specific Fields

✅ Script saved 📊 Document titles (5 docs):

1.  Vector Databases Explained
2.  Introduction to Machine Learning
3.  Natural Language Processing
4.  Database Design Principles
5.  Getting Started with ekoDB ⏱️ Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved 📊 Projected documents (showing first 3):

1.  Vector Databases Explained
2.  Introduction to Machine Learning
3.  Natural Language Processing ⏱️ Execution time: 0ms

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

1. Efficient Querying: Vector databases allow spatial indexing, which makes
   querying for certain data faster.
2. High Precision: They offer superior accuracy and precision as they store data
   as points, lines, and polygons.
3. Scalability: Vector databases can easily handle large datasets.
4. Flexibility: They allow various types of queries and spatial operations.
5. Better Visualization: Vector data offer better visualization of data as they
   can represent complex features accurately.
6. Supports Topological Analysis: Vector databases can identify spatial
   relationships between different entities.
7. Compact Data Storage: Vector models allow compact storage of complex data. ⏱️
   Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding script saved 📊 Generated embeddings for 3 articles

1.  "Advanced Query Patterns" - 1536D vector
2.  "Draft Article" - 1536D vector
3.  "Getting Started with ekoDB" - 1536D vector ⏱️ Execution time: 0ms

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
Record(fields={id=StringValue(value=3yJijN7r8ntdO5SnWpb_s9h9HUwYcUjFbonfVjScNSEYPDE-wWi1aCLydVYM1vqYY4T0So_nDLYDvd88aBqwqA)})
User ID:
3yJijN7r8ntdO5SnWpb_s9h9HUwYcUjFbonfVjScNSEYPDE-wWi1aCLydVYM1vqYY4T0So_nDLYDvd88aBqwqA

=== Read === ✓ Found user by ID:
Record(fields={age=ObjectValue(value={type=StringValue(value=Integer),
value=IntegerValue(value=28)}),
id=StringValue(value=3yJijN7r8ntdO5SnWpb_s9h9HUwYcUjFbonfVjScNSEYPDE-wWi1aCLydVYM1vqYY4T0So_nDLYDvd88aBqwqA),
active=ObjectValue(value={type=StringValue(value=Boolean),
value=BooleanValue(value=true)}),
name=ObjectValue(value={value=StringValue(value=Alice Johnson),
type=StringValue(value=String)}),
email=ObjectValue(value={type=StringValue(value=String),
value=StringValue(value=alice@example.com)})})

=== Update === ✓ Updated user:
Record(fields={name=ObjectValue(value={value=StringValue(value=Alice Johnson),
type=StringValue(value=String)}),
age=ObjectValue(value={value=IntegerValue(value=29),
type=StringValue(value=Integer)}),
active=ObjectValue(value={value=BooleanValue(value=true),
type=StringValue(value=Boolean)}),
city=ObjectValue(value={value=StringValue(value=San Francisco),
type=StringValue(value=String)}),
id=StringValue(value=3yJijN7r8ntdO5SnWpb_s9h9HUwYcUjFbonfVjScNSEYPDE-wWi1aCLydVYM1vqYY4T0So_nDLYDvd88aBqwqA),
email=ObjectValue(value={value=StringValue(value=alice@example.com),
type=StringValue(value=String)})})

=== Query === ✓ Found 1 users matching query

- Record(fields={email=ObjectValue(value={value=StringValue(value=alice@example.com),
  type=StringValue(value=String)}),
  active=ObjectValue(value={type=StringValue(value=Boolean),
  value=BooleanValue(value=true)}),
  city=ObjectValue(value={value=StringValue(value=San Francisco),
  type=StringValue(value=String)}),
  age=ObjectValue(value={type=StringValue(value=Integer),
  value=IntegerValue(value=29)}),
  name=ObjectValue(value={type=StringValue(value=String),
  value=StringValue(value=Alice Johnson)}),
  id=StringValue(value=3yJijN7r8ntdO5SnWpb_s9h9HUwYcUjFbonfVjScNSEYPDE-wWi1aCLydVYM1vqYY4T0So_nDLYDvd88aBqwqA)})

=== Delete === ✓ Deleted user with ID:
3yJijN7r8ntdO5SnWpb_s9h9HUwYcUjFbonfVjScNSEYPDE-wWi1aCLydVYM1vqYY4T0So_nDLYDvd88aBqwqA

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
9Fw1jsIDlyuDsT7cEW1SBUgJX8OPjG4mszFLSAiRHsacwv4Aj1UPyaKe_kpVvp-O75PuSuGDgGT-k91pAdkmSA,
d5fSbMcDNBH0o_Fu2x_oJ-TeRfv8w-OPbP3mQhbePGWStdp6cl88A7YWjf-2C94J3KVH9RAVitZjZSxSUj5hag,
3SXNaVSi2FAlEVxFJ9Ko1U4--60MVZh8rhWXB8nmrc8W4WpezsM97Umvs118df3mug4hXa6uni9L42TFK2Q4Ig...

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
- Score: ObjectValue(value={value=IntegerValue(value=92),
  type=StringValue(value=Integer)})
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
{"user_id":"123","created_at":1766219924365} (Will expire in 10 seconds)

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

- schema_employees_client_ts
- scripts\_\_ek0_bench
- websocket_test
- schema_documents_client_py
- schema_users_client_go

=== Check Collection Existence === Collection 'kotlin_collection_example'
exists: false

=== Create Collection with Schema === ✓ Created collection with schema:
kotlin_collection_example

=== Get Collection Schema === ✓ Schema:
{"fields":{"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2025-12-20T08:38:48.539030Z","last_modified":"2025-12-20T08:38:48.539030Z","bypass_ripple":false,"primary_key_alias":"id"}

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
NO6QcRM-pDDQ-7fRhv7zDOfP9EEnZKMBMhfK0Nd78N7awTkcLAOB1lglaPozaJ1eB4vyi95LzN7LeWeE_FDTmw

=== Verify Document Exists === ✓ Document found: [ttl_duration, id, session_id,
user_id, created_at]

=== Insert with Longer TTL === ✓ Inserted document with 1h TTL Document ID:
tEZ1jJ7KyB7bKd78aHaFV-fw7xbxqdUKNo18SXWl3vNX7zBcqycoAhutvoZgVNfLUaQTRQeX0kXDaZNSywbxDQ

=== TTL Expiration === ✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document === ✓ Long TTL document still exists: [cache_key,
ttl_duration, value, id]

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
{"payload":{"data":[{"id":"WCNvRgyX-YWGEly-xRexQ6pP-jzNX8VvXaPt7apAUljr4X_9b_qaxEZ7_oHB37bJklpp5EYrboNlHVmVKXFhcw","name":{"type":"String","value":"Test
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
StringValue(value=mzlxXcLypdyjaRRRyXsZJM6KSImCR6wFmBoOUQLGr0fTsUQJGeXzb4JL3SGSXmUMuk76_GPp6rOTDi_BM_4BHw)
✓ Inserted user 2:
StringValue(value=CCdP2ca1SjvmWPj08AOxtOc4aXD5lSbUVoIUd2fcTEGSUgqdwWdK2U-sm0DPLCO7n414Cu1QhcEUQWUZmBgWbw)

=== Getting Schema === ✓ Schema for kotlin_schema_example: Fields:
{"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections === ✓ Total collections: 22 Sample:
[schema_employees_client_ts, scripts__ek0_bench, websocket_test,
schema_documents_client_py, schema_users_client_go]

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
hands-on examples and best practices.","title":"Rust
Programming","views":60,"tags":"programming,rust,tutorial","id":"oOTI_G-gP52QhCpfdTnwMPCJiJXeoIiBWPPccliEOxNoR9p7IinfS5w8UxtHOBo158Jb5WJ8_QOoIWvcXJ3IMQ"},"score":19.8,"matched_fields":["title","tags","description"]},{"record":{"id":"Q_0kqWQdtjm8kQp2gizcbqEGPDxNUwKZ7vP6rKfi-GJ9mbjvXIlFtoQIECpj_HJZG5g47tvEziRndD-AvjNvaA","tags":"programming,javascript,web","views":13,"description":"Build
modern web applications using JavaScript, React, and
Node.js.","title":"JavaScript Web
Development"},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"id":"AOnvVcrq-xp4rH8H-P4-MmtCwPBTc4NV4qFWwEuJ0izYwuF4WDmjJiceuH1wfhcliMZUA9DLAcnoJ9-btrgx6A","title":"Python
for Data Science","tags":"programming,python,data-science","description":"Master
Python for data analysis, machine learning, and
visualization.","views":356},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' === ✓ Found results for 'machine learning'
{"results":[{"record":{"views":856,"tags":"ai,machine-learning,python","id":"X2IsAYmYqXpgvQeZxrTKHUpw5XQkNFdLiGD3u56c7Jtv10QgtcpXgqYkRRwenPAkgQ71H04AsICSzYtxMyWZ4Q","description":"Introduction
to machine learning algorithms and neural networks.","title":"Machine Learning
Basics"},"score":2.7,"matched_fields":["title","description","tags"]},{"record":{"description":"Master
Python for data analysis, machine learning, and
visualization.","id":"AOnvVcrq-xp4rH8H-P4-MmtCwPBTc4NV4qFWwEuJ0izYwuF4WDmjJiceuH1wfhcliMZUA9DLAcnoJ9-btrgx6A","views":356,"tags":"programming,python,data-science","title":"Python
for Data
Science"},"score":1.0,"matched_fields":["description"]},{"record":{"tags":"programming,rust,tutorial","id":"oOTI_G-gP52QhCpfdTnwMPCJiJXeoIiBWPPccliEOxNoR9p7IinfS5w8UxtHOBo158Jb5WJ8_QOoIWvcXJ3IMQ","views":60,"title":"Rust
Programming","description":"Learn Rust programming language with hands-on
examples and best
practices."},"score":0.5,"matched_fields":["description"]},{"record":{"title":"Database
Design","id":"N8RCiDAEY3uKN5sxqynAEV2WHZID--qbSKFwn8v5GL7Zxu4DDrTgFJJ2rDeB1LFnRqZ52wKijs7n4oXbIh82sg","views":521,"tags":"database,design,sql","description":"Learn
database design principles, normalization, and query
optimization."},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

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
StringValue(value=Xgg4bI7IOQwQKyK1Ekg5pNwjnaSQq3HUfVY0Gaw1LkgZcOP60JP78nu5enJXJ1tUM7LoHJGs_lcMek3H9RRs2A)

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved data via
WebSocket:
{"payload":{"data":[{"created_at":{"type":"Integer","value":1766219949292},"id":"Xgg4bI7IOQwQKyK1Ekg5pNwjnaSQq3HUfVY0Gaw1LkgZcOP60JP78nu5enJXJ1tUM7LoHJGs_lcMek3H9RRs2A","name":{"type":"String","value":"WebSocket
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

- Laptop ($1200) by Alice Johnson
- Mouse ($25) by Alice Johnson

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
BlhhwhZ4OjJxdcasv7_QNbUMnlGMud-QHJyTv-PoQkqu1I-q3IpQ6AjJwOuDcDb83zWWZJ_jKHE2-lYswiAW5w

=== Sending Chat Message === ✓ Chat response: Message ID:
"ZhpUUsd6-7TsfFwuFsMMmV-S5Iwxpa_Jg1ZXqCSAMWs-7KcuIYjEDtawLmlNHfRhE0H14p2PTDzPXgHv-kU4jw"
Responses: ["I'm sorry, but the provided context does not contain any
information about ekoDB and its features."]

=== Cleanup === ✓ Deleted chat session ✓ Deleted collection:
kotlin_chat_basic_example

✓ Basic chat example completed successfully

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
> Kotlin Client - Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
uD53qN9L6PgL0rmu5vhxRF5CZeR5PbsuDZGDFkRoyg8OxjnZAGcjfd2lafWCcrcl7dNd4l6DgpRkj8SkKbJmdA

=== Sending Messages === ✓ Message 1 sent Responses: ["I'm sorry, but the
context provided does not contain specific information about the available
products. Could you please provide more details?"]

✓ Message 2 sent Responses: ["Apologies, but there's no information given about
a specific product's price in the provided context. Could you please specify the
product you're inquiring about?"]

=== Getting Message History === ✓ Retrieved message history Total messages:
[{"chat_id":{"type":"String","value":"uD53qN9L6PgL0rmu5vhxRF5CZeR5PbsuDZGDFkRoyg8OxjnZAGcjfd2lafWCcrcl7dNd4l6DgpRkj8SkKbJmdA"},"content":{"type":"String","value":"What
products are
available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":{"type":"String","value":"A
high-performance database product with AI
capabilities"},"id":"xzt28UHDHES0WBTORhAHRT5ozSxXtSasV4bjjFw2ZCCy9Qq7EIMUXkLlVVHvwutQPe0AGVq9pLi_IEG2BPPKsQ","price":{"type":"Integer","value":99},"product":{"type":"String","value":"ekoDB"}},"score":0.25}]},"created_at":{"type":"DateTime","value":"2025-12-20T08:39:24.705502Z"},"id":"EHBqu3da_kYid-ulRXz7gvr8V9nhx2927v36D4lpCsWDXHBFBWitK0O3gw8pW6ilVHp8S1uDxSXQZDMC_TufqQ","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":25,"prompt_tokens":156,"total_tokens":181}},"updated_at":{"type":"DateTime","value":"2025-12-20T08:39:24.705502Z"}},{"chat_id":{"type":"String","value":"uD53qN9L6PgL0rmu5vhxRF5CZeR5PbsuDZGDFkRoyg8OxjnZAGcjfd2lafWCcrcl7dNd4l6DgpRkj8SkKbJmdA"},"content":{"type":"String","value":"I'm
sorry, but the context provided does not contain specific information about the
available products. Could you please provide more
details?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":{"type":"String","value":"A
high-performance database product with AI
capabilities"},"id":"xzt28UHDHES0WBTORhAHRT5ozSxXtSasV4bjjFw2ZCCy9Qq7EIMUXkLlVVHvwutQPe0AGVq9pLi_IEG2BPPKsQ","price":{"type":"Integer","value":99},"product":{"type":"String","value":"ekoDB"}},"score":0.25}]},"created_at":{"type":"DateTime","value":"2025-12-20T08:39:24.708081Z"},"id":"cUMcLVqrYK1pgyhRMnjzcld2r40RZoOhWWJNWLi4ritU6pMripJ6gxjlPWMFWJfoCyw3LSsRru66zskthT_Cpg","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":25,"prompt_tokens":156,"total_tokens":181}},"updated_at":{"type":"DateTime","value":"2025-12-20T08:39:24.708081Z"}},{"chat_id":{"type":"String","value":"uD53qN9L6PgL0rmu5vhxRF5CZeR5PbsuDZGDFkRoyg8OxjnZAGcjfd2lafWCcrcl7dNd4l6DgpRkj8SkKbJmdA"},"content":{"type":"String","value":"What
is the
price?"},"created_at":{"type":"DateTime","value":"2025-12-20T08:39:26.240909Z"},"id":"75aoc0ecNczFTwmPtOq0B5UcVK5YTjaRQw3lZ-gZ706XipktieA3jobHepdMhdAu2BTarhHoa2j70AHky9YrSQ","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":32,"prompt_tokens":70,"total_tokens":102}},"updated_at":{"type":"DateTime","value":"2025-12-20T08:39:26.240909Z"}},{"chat_id":{"type":"String","value":"uD53qN9L6PgL0rmu5vhxRF5CZeR5PbsuDZGDFkRoyg8OxjnZAGcjfd2lafWCcrcl7dNd4l6DgpRkj8SkKbJmdA"},"content":{"type":"String","value":"Apologies,
but there's no information given about a specific product's price in the
provided context. Could you please specify the product you're inquiring
about?"},"created_at":{"type":"DateTime","value":"2025-12-20T08:39:26.243977Z"},"id":"SC0SOJZW_IZ-rtDC7HGVPJvPoqAO2MjekRaYfRwYmK51ax8-0Ijv3BJWHJ9-RPDQzQ6CORJrrtZ0uuniK8DIxw","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":32,"prompt_tokens":70,"total_tokens":102}},"updated_at":{"type":"DateTime","value":"2025-12-20T08:39:26.243977Z"}}]

=== Updating Session === ✓ Updated session system prompt

=== Listing Sessions === ✓ Total sessions: 13

=== Branching Session === ✓ Created branched session:
bj6dwb_0xMfDKbdcoxsFDdjYmAQTrbRtJZcef6K0F_8u1RKvXM6-dDSvg4xEaMajoqX6-2a0q7t1h1EK5cK9hA

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
qHAiwp1Trmj4VtgP7LN9RMTaeG32qgthzPFbDg0eVbgLPw1Vs4apZeNhJ5-J4Gt3cKZ2vlMYeODrQUPcuIHYfQ

=== Sending Initial Message === ✓ Message sent Responses: ["I'm sorry, but the
context provided does not contain information about any specific products. Could
you please provide more details?"]

✓ Second message sent

=== Regenerating AI Response === ✓ AI response regenerated New responses: ["I'm
sorry, but the context provided doesn't give information about any specific
products. Could you please provide more details or specify the type of products
you're interested in?"]

=== Updating Message === ✓ Updated message content

=== Toggling Forgotten Status === ✓ Marked message as forgotten (excluded from
context)

=== Creating Second Session for Merge === ✓ Created second session:
Lx2iJs96Ye7M13exjGZ7GQ\_\_hMFvNiJUKXfVjIv043I94AX6kRH0f2iCq3btv0pJ6MzmgWgiJCWsQre1dJAOYw

=== Merging Sessions === ✓ Merged sessions Merged session ID: null

=== Deleting Message === ✓ Deleted message

=== Cleanup === ✓ Deleted chat sessions ✓ Deleted collection:
kotlin_chat_advanced_example

✓ Advanced chat features example completed successfully

BUILD SUCCESSFUL in 6s 2 actionable tasks: 1 executed, 1 up-to-date To honour
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
ahUrOx8-9GcQ8RF9Xt67MWQswstC5M3VCywD5bCrmhljwS1DpgYdhYJshZqdr8I1Pozu84fvZXtDzpAROHaSHQ
📊 Found 10 records ⏱️ Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved 📊 Found 10 users (limited) ⏱️ Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved 📊 Statistics: 2 groups
{"count":5,"status":"active","avg_score":60.0}
{"count":5,"status":"inactive","avg_score":50.0} ⏱️ Execution time: 0ms

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
operation ✓ Generated embedding: 1536 dimensions in 322.310666ms • Function
auto-cleaned up by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 169 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 271.825834ms • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 33
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 251.234458ms • Function auto-cleaned up
by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 230 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 289.611209ms • Function auto-cleaned up by client ✓ Stored Rust
programming conversation (4 messages) → Calling ekoDB embed() helper... • Using
model: text-embedding-3-small • Text length: 31 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 257.280167ms • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 217
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 350.550208ms • Function auto-cleaned up
by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 33 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 311.860542ms • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 232
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 538.007833ms • Function auto-cleaned up
by client ✓ Stored database design conversation (4 messages) → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 36
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 298.990625ms • Function auto-cleaned up
by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 178 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 291.418375ms • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 37
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 405.823958ms • Function auto-cleaned up
by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 213 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 273.1395ms • Function auto-cleaned up by client ✓ Stored
performance optimization conversation (4 messages)

=== Step 2: New User Question with Context Retrieval === User asks: "How do I
write memory-safe high-performance database code?"

=== Step 3: Searching Related Context === Using hybrid search to find relevant
messages from all conversations...

→ Generating embedding for user question... → Calling ekoDB embed() helper... •
Using model: text-embedding-3-small • Text length: 58 characters • Behind the
scenes: Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 309.944709ms • Function auto-cleaned up by client

→ Executing hybrid_search()... • Collection: rag_messages • Query text: "How do
I write memory-safe high-performance database code?" • Vector dimensions: 1536 •
Limit: 5 results • Search type: Semantic (vector) + Keyword (text) • Server
combines both scores for relevance ranking ✓ Search completed in 103.481625ms

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

Answer: Writing memory-safe, high-performance database code involves several
steps:

1. **Understanding your data**: As mentioned in the context, it's important to
   choose the right database type (SQL or NoSQL) depending on your data's
   structure, relationships, and the type of queries you'll be performing.

2. **Normalize your data**: Normalize your data to reduce redundancy and improve
   data integrity. As stated in Context 3, normalization involves dividing large
   tables into smaller ones and defining relationships between them with foreign
   keys.

3. **Optimize your queries**: Reduce the amount of data that needs to be read
   from the database. This can be done by limiting the number of records
   returned, using WHERE clauses, indexing, and avoiding using wildcards at the
   beginning of a LIKE clause.

4. **Use prepared statements**: Prepared statements not only help prevent SQL
   injection attacks but can also enhance performance, especially when executing
   a single statement repeatedly.

5. **Connection pooling**: Create a pool of database connections and reuse them,
   rather than creating a new connection every time a client makes a request.
   This significantly reduces the overhead of establishing a new connection.

6. **Use efficient data types**: Choose the most efficient data type for your
   columns to reduce the amount of space your data consumes.

7. **Caching**: Cache the results of queries that are requested frequently and
   don't change often.

8. **Memory management**: Regularly check your application for memory leaks and
   ensure all database connections are closed when they are no longer needed.

9. **Concurrency control**: Use appropriate locking mechanisms to control the
   simultaneous access of data by multiple threads.

Remember, the performance of your database code depends largely on the specific
requirements and constraints of your project. It's essential to continuously
monitor and tune your database as those requirements evolve.

=== Step 5: Storing New Conversation === → Calling ekoDB embed() helper... •
Using model: text-embedding-3-small • Text length: 58 characters • Behind the
scenes: Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 284.626167ms • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 1965
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 281.515041ms • Function auto-cleaned up
by client ✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search === Searching for messages about
'ownership' across ALL conversations...

→ Executing text_search()... • Collection: rag_messages • Query: "ownership
system" • Limit: 3 results • Search method: Full-text with fuzzy matching &
stemming • No vector embeddings needed - pure keyword search ✓ Text search
completed in 62.01475ms

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
operation ✓ Generated embedding: 1536 dimensions in 0.409s • Function
auto-cleaned up by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 169 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.316s • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 33
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.223s • Function auto-cleaned up by
client → Calling ekoDB embed() helper... • Using model: text-embedding-3-small •
Text length: 230 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.267s • Function
auto-cleaned up by client ✓ Stored Rust programming conversation (4 messages) →
Calling ekoDB embed() helper... • Using model: text-embedding-3-small • Text
length: 31 characters • Behind the scenes: Creating temp Function with Embed
operation ✓ Generated embedding: 1536 dimensions in 0.204s • Function
auto-cleaned up by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 217 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.233s • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 33
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.231s • Function auto-cleaned up by
client → Calling ekoDB embed() helper... • Using model: text-embedding-3-small •
Text length: 232 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.311s • Function
auto-cleaned up by client ✓ Stored database design conversation (4 messages) →
Calling ekoDB embed() helper... • Using model: text-embedding-3-small • Text
length: 36 characters • Behind the scenes: Creating temp Function with Embed
operation ✓ Generated embedding: 1536 dimensions in 0.300s • Function
auto-cleaned up by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 178 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.586s • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 37
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.195s • Function auto-cleaned up by
client → Calling ekoDB embed() helper... • Using model: text-embedding-3-small •
Text length: 213 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.307s • Function
auto-cleaned up by client ✓ Stored performance optimization conversation (4
messages)

=== Step 2: New User Question with Context Retrieval === User asks: "How do I
write memory-safe high-performance database code?"

=== Step 3: Searching Related Context === Using hybrid search to find relevant
messages from all conversations...

→ Generating embedding for user question... → Calling ekoDB embed() helper... •
Using model: text-embedding-3-small • Text length: 58 characters • Behind the
scenes: Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.295s • Function auto-cleaned up by client

→ Executing hybrid_search()... • Collection: rag_messages • Query text: "How do
I write memory-safe high-performance database code?" • Vector dimensions: 1536 •
Limit: 5 results • Search type: Semantic (vector) + Keyword (text) • Server
combines both scores for relevance ranking ✓ Search completed in 0.091s

✓ Found 5 related messages across all conversations:

1. [Score: 0.000] From conv_database_design Use NoSQL when you need: flexible
   schemas, horizontal scaling, high write throughput, or when working with
   unstructured data. SQL is better for complex queries, ACID transactions, and
   structured data with well-defined relationships.

2. [Score: 0.000] From conv_database_design What is database normalization?

3. [Score: 0.000] From conv_database_design Database normalization is the
   process of organizing data to reduce redundancy and improve data integrity.
   It involves dividing large tables into smaller ones and defining
   relationships between them using foreign keys.

4. [Score: 0.000] From conv_database_design When should I use NoSQL over SQL?

5. [Score: 0.000] From conv_performance How can I optimize database queries?

=== Step 4: Generating Context-Aware Response === ✓ AI Response (with context
from 3 conversations):

Answer: Writing memory-safe and high-performance database code involves several
factors:

1. Choose the Right Database: Depending on your use case, either SQL or NoSQL
   might suit you better. For example, if you need to handle unstructured data,
   have high write throughput, or need horizontal scaling, you might want to
   choose NoSQL (context 1). If your data is structured with well-defined
   relationships, or you need ACID transactions or complex queries, SQL might be
   a better choice (context 1).

2. Database Normalization: Normalization can help improve data integrity and
   reduce redundancy. It involves dividing larger tables into smaller ones and
   defining relationships between them using foreign keys (context 3). This can
   help improve performance by reducing the amount of data that needs to be read
   from or written to the database.

3. Optimize Your Queries: You can optimize your database queries by using
   indexes, using the correct query operators, avoiding full table scans,
   limiting the amount of returned data, and properly utilizing joins,
   subqueries, and other advanced SQL features.

4. Memory Management: Depending on the language you're using to interact with
   the database, there might be specific tools or best practices for managing
   memory. For example, in languages like C++, you need to be careful about
   freeing any memory that you allocate. In more managed languages like Java or
   Python, it's important to understand how the garbage collector works and how
   to write code that doesn't unnecessarily hold onto memory.

5. Error Handling: Proper error handling can prevent memory leaks and other
   potential issues. Always ensure that your code can gracefully handle any
   database errors and clean up any resources it was using.

6. Testing and Profiling: Regularly test your code for performance and memory
   usage. Profiling tools can help you identify any bottlenecks or memory leaks.

Remember, the specifics will depend on your particular use case, the database
you're using, and the language you're writing your code in.

=== Step 5: Storing New Conversation === → Calling ekoDB embed() helper... •
Using model: text-embedding-3-small • Text length: 58 characters • Behind the
scenes: Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.388s • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 2040
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.245s • Function auto-cleaned up by
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
operation ✓ Generated embedding: 1536 dimensions in 0.311s • Function
auto-cleaned up by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 169 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.303s • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 33
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.277s • Function auto-cleaned up by
client → Calling ekoDB embed() helper... • Using model: text-embedding-3-small •
Text length: 230 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.263s • Function
auto-cleaned up by client ✓ Stored Rust programming conversation (4 messages) →
Calling ekoDB embed() helper... • Using model: text-embedding-3-small • Text
length: 31 characters • Behind the scenes: Creating temp Function with Embed
operation ✓ Generated embedding: 1536 dimensions in 0.264s • Function
auto-cleaned up by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 217 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.388s • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 33
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.199s • Function auto-cleaned up by
client → Calling ekoDB embed() helper... • Using model: text-embedding-3-small •
Text length: 232 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.356s • Function
auto-cleaned up by client ✓ Stored database design conversation (4 messages) →
Calling ekoDB embed() helper... • Using model: text-embedding-3-small • Text
length: 36 characters • Behind the scenes: Creating temp Function with Embed
operation ✓ Generated embedding: 1536 dimensions in 0.214s • Function
auto-cleaned up by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 178 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.326s • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 37
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.309s • Function auto-cleaned up by
client → Calling ekoDB embed() helper... • Using model: text-embedding-3-small •
Text length: 213 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.410s • Function
auto-cleaned up by client ✓ Stored performance optimization conversation (4
messages)

=== Step 2: New User Question with Context Retrieval === User asks: "How do I
write memory-safe high-performance database code?"

=== Step 3: Searching Related Context === Using hybrid search to find relevant
messages from all conversations...

→ Generating embedding for user question... → Calling ekoDB embed() helper... •
Using model: text-embedding-3-small • Text length: 58 characters • Behind the
scenes: Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.286s • Function auto-cleaned up by client

→ Executing hybridSearch()... • Collection: rag_messages • Query text: "How do I
write memory-safe high-performance database code?" • Vector dimensions: 1536 •
Limit: 5 results • Search type: Semantic (vector) + Keyword (text) • Server
combines both scores for relevance ranking ✓ Search completed in 0.088s ✓ Found
5 related messages across all conversations:

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

=== Step 4: Generating Context-Aware Response === ✓ AI Response (with context
from 3 conversations):

Writing memory-safe, high-performance database code involves a combination of
efficient memory management, well-structured queries, and appropriate database
choice. Here are a few strategies:

1. Database Selection: Choose the right database system based on your needs. As
   mentioned in Context 2, NoSQL databases are suitable for flexible schemas,
   horizontal scaling, high write throughput, and unstructured data. SQL
   databases are better for complex queries, ACID transactions, and structured
   data with well-defined relationships.

2. Query Optimization: You can optimize database queries by using indexes,
   avoiding full table scans, limiting the result set, using joins wisely, and
   avoiding N+1 queries. Also, consider using stored procedures or prepared
   statements to reuse code and minimize database hits.

3. Normalization: Database normalization, as described in Context 5, helps
   improve data integrity and reduces redundancy. This process organizes data by
   dividing large tables into smaller ones and establishing relationships
   between them using foreign keys. Normalization can lead to performance
   improvements due to decreased redundancy.

4. Memory Management: Be mindful of the memory usage in your code. Avoid storing
   large datasets in memory all at once, and instead process data in chunks if
   possible. In addition, ensure that any resources are properly released after
   use to prevent memory leaks.

5. Concurrency: Implement concurrency control mechanisms to handle multiple
   simultaneous operations. This can help to maximize throughput and minimize
   contention for resources.

6. Caching: Use caching techniques to store frequently accessed data in memory,
   reducing the need for expensive database calls.

By combining these strategies, you can write code that is both memory-safe and
high-performance.

=== Step 5: Storing New Conversation === → Calling ekoDB embed() helper... •
Using model: text-embedding-3-small • Text length: 58 characters • Behind the
scenes: Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.295s • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 1816
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.270s • Function auto-cleaned up by
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
operation ✓ Generated embedding: 1536 dimensions in 0.247s • Function
auto-cleaned up by client → Calling ekoDB Embed() helper... • Using model:
text-embedding-3-small • Text length: 169 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.456s • Function auto-cleaned up by client → Calling ekoDB
Embed() helper... • Using model: text-embedding-3-small • Text length: 33
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.240s • Function auto-cleaned up by
client → Calling ekoDB Embed() helper... • Using model: text-embedding-3-small •
Text length: 230 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.266s • Function
auto-cleaned up by client ✓ Stored Rust programming conversation (4 messages) →
Calling ekoDB Embed() helper... • Using model: text-embedding-3-small • Text
length: 31 characters • Behind the scenes: Creating temp Function with Embed
operation ✓ Generated embedding: 1536 dimensions in 0.256s • Function
auto-cleaned up by client → Calling ekoDB Embed() helper... • Using model:
text-embedding-3-small • Text length: 217 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.278s • Function auto-cleaned up by client → Calling ekoDB
Embed() helper... • Using model: text-embedding-3-small • Text length: 33
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.278s • Function auto-cleaned up by
client → Calling ekoDB Embed() helper... • Using model: text-embedding-3-small •
Text length: 232 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.289s • Function
auto-cleaned up by client ✓ Stored database design conversation (4 messages) →
Calling ekoDB Embed() helper... • Using model: text-embedding-3-small • Text
length: 36 characters • Behind the scenes: Creating temp Function with Embed
operation ✓ Generated embedding: 1536 dimensions in 0.250s • Function
auto-cleaned up by client → Calling ekoDB Embed() helper... • Using model:
text-embedding-3-small • Text length: 178 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.329s • Function auto-cleaned up by client → Calling ekoDB
Embed() helper... • Using model: text-embedding-3-small • Text length: 37
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.289s • Function auto-cleaned up by
client → Calling ekoDB Embed() helper... • Using model: text-embedding-3-small •
Text length: 213 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.281s • Function
auto-cleaned up by client ✓ Stored performance optimization conversation (4
messages)

=== Step 2: New User Question with Context Retrieval === User asks: "How do I
write memory-safe high-performance database code?"

=== Step 3: Searching Related Context === Using hybrid search to find relevant
messages from all conversations...

→ Generating embedding for user question... → Calling ekoDB Embed() helper... •
Using model: text-embedding-3-small • Text length: 58 characters • Behind the
scenes: Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.482s • Function auto-cleaned up by client

→ Executing HybridSearch()... • Collection: rag_messages • Query text: "How do I
write memory-safe high-performance database code?" • Vector dimensions: 1536 •
Limit: 5 results • Search type: Semantic (vector) + Keyword (text) • Server
combines both scores for relevance ranking ✓ Search completed in 0.082s

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

Answer: Writing memory-safe, high-performance database code involves a
combination of good practices including efficient database design, effective use
of SQL or NoSQL depending on your needs, as well as optimizing your database
queries.

1. Efficient Database Design: This is where concepts like 'Database
   Normalization' come into play. Normalization helps in reducing data
   redundancy and improving data integrity by dividing large tables into smaller
   ones and defining relationships between them using foreign keys.

2. Choice of SQL or NoSQL: Depending on your needs, you may choose either SQL or
   NoSQL databases. Use NoSQL for flexible schemas, horizontal scaling, high
   write throughput, or when working with unstructured data. On the other hand,
   SQL is better for complex queries, ACID transactions, and structured data
   with well-defined relationships.

3. Optimizing Database Queries: To optimize your database queries, ensure that
   you're only retrieving the data you need (limit the use of SELECT \*), make
   good use of indexing to speed up retrieval of data, avoid using heavy
   operations like HAVING and UNION where you can use WHERE and JOIN
   respectively. Also, avoid N+1 queries where you load child entities one by
   one, instead, use join queries to load them in a single trip to your
   database.

4. Memory Management: In terms of memory safety, ensure that you close all
   database connections after operations to free up resources. Also, use
   parameterized queries or prepared statements to guard against SQL injection,
   which can lead to unauthorized use of memory.

5. Regular Monitoring and Maintenance: Regularly monitor the performance of your
   database. Use tools that can help identify slow queries, analyze database
   workload, and suggest indexes. Regular database maintenance like updating
   statistics, rebuilding indexes, and removing old data also helps in
   maintaining high performance.

Remember, what works best can depend on the specific database system you're
using, and the specific use case of your application.

=== Step 5: Storing New Conversation === → Calling ekoDB Embed() helper... •
Using model: text-embedding-3-small • Text length: 58 characters • Behind the
scenes: Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.332s • Function auto-cleaned up by client → Calling ekoDB
Embed() helper... • Using model: text-embedding-3-small • Text length: 2030
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.229s • Function auto-cleaned up by
client ✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search === Searching for messages about
'ownership' across ALL conversations...

→ Executing TextSearch()... • Collection: rag_messages • Query: "ownership
system" • Limit: 3 results • Search method: Full-text with fuzzy matching &
stemming • No vector embeddings needed - pure keyword search ✓ Text search
completed in 0.054s

✓ Found 3 messages mentioning ownership:

1. From conv_rust_programming: Rust's key features include: memory safety
   without garbage collection, zero-cost abstractions, ownership system,
   powerful type system, and excellent concurrency support.

2. From conv_performance: Rust's ownership system provides zero-cost memory
   management. Use Box for heap allocation, Rc/Arc for shared ownership, and
   avoid cloning large data structures. The compiler optimizes away unnecessary
   allocations.

3. From conv_new_question: Answer: Writing memory-safe, high-performance
   database code involves a combination of good practices including efficient
   database design, effective use of SQL or NoSQL depending on your needs, as
   well as optimizing your database queries.

4. Efficient Database Design: This is where concepts like 'Database
   Normalization' come into play. Normalization helps in reducing data
   redundancy and improving data integrity by dividing large tables into smaller
   ones and defining relationships between them using foreign keys.

5. Choice of SQL or NoSQL: Depending on your needs, you may choose either SQL or
   NoSQL databases. Use NoSQL for flexible schemas, horizontal scaling, high
   write throughput, or when working with unstructured data. On the other hand,
   SQL is better for complex queries, ACID transactions, and structured data
   with well-defined relationships.

6. Optimizing Database Queries: To optimize your database queries, ensure that
   you're only retrieving the data you need (limit the use of SELECT \*), make
   good use of indexing to speed up retrieval of data, avoid using heavy
   operations like HAVING and UNION where you can use WHERE and JOIN
   respectively. Also, avoid N+1 queries where you load child entities one by
   one, instead, use join queries to load them in a single trip to your
   database.

7. Memory Management: In terms of memory safety, ensure that you close all
   database connections after operations to free up resources. Also, use
   parameterized queries or prepared statements to guard against SQL injection,
   which can lead to unauthorized use of memory.

8. Regular Monitoring and Maintenance: Regularly monitor the performance of your
   database. Use tools that can help identify slow queries, analyze database
   workload, and suggest indexes. Regular database maintenance like updating
   statistics, rebuilding indexes, and removing old data also helps in
   maintaining high performance.

Remember, what works best can depend on the specific database system you're
using, and the specific use case of your application.

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
operation ✓ Generated embedding: 1536 dimensions in 0.346s • Function
auto-cleaned up by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 169 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.218s • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 33
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.343s • Function auto-cleaned up by
client → Calling ekoDB embed() helper... • Using model: text-embedding-3-small •
Text length: 230 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.299s • Function
auto-cleaned up by client ✓ Stored Rust programming conversation (4 messages) →
Calling ekoDB embed() helper... • Using model: text-embedding-3-small • Text
length: 31 characters • Behind the scenes: Creating temp Function with Embed
operation ✓ Generated embedding: 1536 dimensions in 0.327s • Function
auto-cleaned up by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 217 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.256s • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 33
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.264s • Function auto-cleaned up by
client → Calling ekoDB embed() helper... • Using model: text-embedding-3-small •
Text length: 232 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.217s • Function
auto-cleaned up by client ✓ Stored database design conversation (4 messages) →
Calling ekoDB embed() helper... • Using model: text-embedding-3-small • Text
length: 36 characters • Behind the scenes: Creating temp Function with Embed
operation ✓ Generated embedding: 1536 dimensions in 0.226s • Function
auto-cleaned up by client → Calling ekoDB embed() helper... • Using model:
text-embedding-3-small • Text length: 178 characters • Behind the scenes:
Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.252s • Function auto-cleaned up by client → Calling ekoDB
embed() helper... • Using model: text-embedding-3-small • Text length: 37
characters • Behind the scenes: Creating temp Function with Embed operation ✓
Generated embedding: 1536 dimensions in 0.559s • Function auto-cleaned up by
client → Calling ekoDB embed() helper... • Using model: text-embedding-3-small •
Text length: 213 characters • Behind the scenes: Creating temp Function with
Embed operation ✓ Generated embedding: 1536 dimensions in 0.255s • Function
auto-cleaned up by client ✓ Stored performance optimization conversation (4
messages)

=== Step 2: New User Question with Context Retrieval === User asks: "How do I
write memory-safe high-performance database code?"

=== Step 3: Searching Related Context === Using hybrid search to find relevant
messages from all conversations...

→ Generating embedding for user question... → Calling ekoDB embed() helper... •
Using model: text-embedding-3-small • Text length: 58 characters • Behind the
scenes: Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.221s • Function auto-cleaned up by client

→ Executing hybridSearch()... • Collection: rag_messages • Query text: "How do I
write memory-safe high-performance database code?" • Vector dimensions: 1536 •
Limit: 5 results • Search type: Semantic (vector) + Keyword (text) • Server
combines both scores for relevance ranking ✓ Search completed in 0.08s

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

=== Step 4: Generating Context-Aware Response === ✓ Context prepared from search
results ✓ AI would use this context to generate comprehensive response

=== Step 5: Storing New Conversation === → Calling ekoDB embed() helper... •
Using model: text-embedding-3-small • Text length: 58 characters • Behind the
scenes: Creating temp Function with Embed operation ✓ Generated embedding: 1536
dimensions in 0.23s • Function auto-cleaned up by client ✓ New conversation
stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search === Searching for messages about
'ownership' across ALL conversations...

→ Executing textSearch()... • Collection: rag_messages • Query: "ownership
system" • Limit: 3 results • Search method: Full-text with fuzzy matching &
stemming • No vector embeddings needed - pure keyword search ✓ Text search
completed in 0.04s

✓ Found 3 messages mentioning ownership:

1. From conv_rust_programming: Rust's key features include: memory safety
   without garbage collection, zero-cost abstractions, ownership system,
   powerful type system, and excellent concurrency support.

2. From conv_performance: Rust's ownership system provides zero-cost memory
   management. Use Box for heap allocation, Rc/Arc for shared ownership, and
   avoid cloning large data structures. The compiler optimizes away unnecessary
   allocations.

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
