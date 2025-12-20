make test-examples-go 🧪 [36mRunning Go examples (direct HTTP/WebSocket)...[0m
[34m ╔════════════════════════════════════════╗[0m [34m║ ekoDB Go Examples Test
Suite ║[0m [34m╚════════════════════════════════════════╝[0m [34m === Checking
Server Connection ===[0m [32m✓ Server is ready[0m [33m === Running 10 Examples
===[0m [34m === Running simple_crud.go ===[0m ✓ Authentication successful

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
