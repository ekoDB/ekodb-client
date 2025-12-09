make test-examples-go 🧪 [36mRunning Go examples (direct HTTP/WebSocket)...[0m
[34m ╔════════════════════════════════════════╗[0m [34m║ ekoDB Go Examples Test
Suite ║[0m [34m╚════════════════════════════════════════╝[0m [34m === Checking
Server Connection ===[0m [32m✓ Server is ready[0m [33m === Running 10 Examples
===[0m [34m === Running simple_crud.go ===[0m ✓ Authentication successful

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
