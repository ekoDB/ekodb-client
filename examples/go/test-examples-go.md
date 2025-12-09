make test-examples-go 🧪 [36mRunning Go examples (direct HTTP/WebSocket)...[0m
[34m ╔════════════════════════════════════════╗[0m [34m║ ekoDB Go Examples Test
Suite ║[0m [34m╚════════════════════════════════════════╝[0m [34m === Checking
Server Connection ===[0m [32m✓ Server is ready[0m [33m === Running 9 Examples
===[0m [34m === Running simple_crud.go ===[0m ✓ Authentication successful

=== Insert Document === Inserted:
map[id:E2DT9eZu4l_LVoKeii5DFgdo1VGA1p-1Srb7J4Xv-gxQWnJv9UVtYUU9-iKZKZze_lup4rUl8WGqlgNoxKGXiQ]

=== Find by ID === Found: map[active:map[type:Boolean value:true]
id:E2DT9eZu4l_LVoKeii5DFgdo1VGA1p-1Srb7J4Xv-gxQWnJv9UVtYUU9-iKZKZze_lup4rUl8WGqlgNoxKGXiQ
name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query === Found 1 documents

=== Update Document === Updated: map[active:map[type:Boolean value:true]
id:E2DT9eZu4l_LVoKeii5DFgdo1VGA1p-1Srb7J4Xv-gxQWnJv9UVtYUU9-iKZKZze_lup4rUl8WGqlgNoxKGXiQ
name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully [32m✓ simple_crud.go completed
successfully[0m [34m === Running simple_websocket.go ===[0m ✓ Authentication
successful

=== Inserting Test Data === ✓ Inserted test record:
tHnRpXS9iyrx1UuHjVIcjmAukoOpDqcb8Vibum5lJBrzq4lg2KxYmcaaf0nF2VPwRhNNDkl_gkP3b4Tr_E6DKw

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"xHB7dOzhn-TURJ68KMxUhViza0AENh17KfL1vjmYJsMyjVX49kQaqRZqLD_AbCoe-BrfQx-JZokRamE6b0Vm3Q",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"gxyJOI5aO4nhazTz8ifgYCc10ASLp66ugU-6T1lqQ8eZiBqUWG9WA3Sc0Dkd8Pjpv60sdsWUbp8WImMAo_lVzw",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"tHnRpXS9iyrx1UuHjVIcjmAukoOpDqcb8Vibum5lJBrzq4lg2KxYmcaaf0nF2VPwRhNNDkl_gkP3b4Tr_E6DKw",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } } ] }, "type": "Success" } ✓ Retrieved 3
record(s) via WebSocket

✓ WebSocket example completed successfully [32m✓ simple_websocket.go completed
successfully[0m [34m === Running batch_operations.go ===[0m ✓ Authentication
successful

=== Batch Insert === Batch insert response: map[failed:[]
successful:[l76PCfw-oG35i69Nh_byuVpzWXXoSvzt5UPUWuRIjCzaK3G8oo7Rfi22aHr5vM3wZWOY5zNugzNQy4if66CrfQ
XIb5xKGxOcMPi0gg6VXgJapYYGshAXlxaStE9rVz0HJKmK4w6mZwaIjhj0VdxAV21jqJh3I4tQ6QzDH9LU5AmQ
NSSfZ9EVVX44v4Rycj6G8utNfoyJsoG94eTlDlzqBOWYqFhTxvGoB6uCzguPRZZPqti_f83jRffq3YoqsS09ig
mvfzPrdd0ZVuLrLSXoBrLYkQLCmZ7LWkeUWWEe8jNIPszQs2YgNVnGnDBoxJWeTxneNwQxSYdyWryDoRHuMeCQ
l1XALdQunZuwHqDLYNUP-UbsX8r0oa-7MASqD1Phzwy4vTnSS7b5eBKmORzuN5mQKM7kbeU2Y1UuMN-k49wU8A]]
✓ Batch insert completed

=== Creating test records for update/delete === Created 3 test records

=== Batch Update === Batch update response: map[failed:[]
successful:[RBBepMDYKkgaKZTNFR6k8YrR4VmmArtxuzSKrxcCf4VBDvqEefpu9Nq_pONgDZbMYZRdc-5pnl5SC0PomKtgxg
Il0HhrPKKFZWpwMTC0GUva5cC0Jy57ilkWvvy1Mbqp4Ur31F8fc9MLwenDgOxS9L-qf6Po0yuD7FQtp2Pz0d4Q
-7w_mqSgAlCNo8QGwPWXjij4oM8b-dG9KB8nlo7thlprWb9ZFBcICwihHSLVyqNN5DoIpWhkyPpAH5Vk_ZOo6Q]]
✓ Batch update completed

=== Batch Delete === Batch delete response: map[failed:[]
successful:[RBBepMDYKkgaKZTNFR6k8YrR4VmmArtxuzSKrxcCf4VBDvqEefpu9Nq_pONgDZbMYZRdc-5pnl5SC0PomKtgxg
Il0HhrPKKFZWpwMTC0GUva5cC0Jy57ilkWvvy1Mbqp4Ur31F8fc9MLwenDgOxS9L-qf6Po0yuD7FQtp2Pz0d4Q
-7w_mqSgAlCNo8QGwPWXjij4oM8b-dG9KB8nlo7thlprWb9ZFBcICwihHSLVyqNN5DoIpWhkyPpAH5Vk_ZOo6Q]]
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
EeK8Avr_heKVqsFly_3LqpX-7j2iDlTd3rrDnPLop9VSwaNlJ44mQV8Rh-pAnQ5DKCE3kS0sYtsOH4u_VFeOlQ

=== List Collections === Total collections: 15 Sample collections:
[schema_employees_client_py demo_collection schema_users_client_py
websocket_test test_collection schema_documents_client_py ttl_cache
chat_configurations__ek0_bench batch_users test_accounts
schema_products_client_py users scripts__ek0_bench chat_messages__ek0_bench
ws_ttl_test]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully [32m✓
collection_management.go completed successfully[0m [34m === Running
document_ttl.go ===[0m ✓ Authentication successful

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
HBuof95rPU-RZFUD22hV9pTiBNhFKJVmVTp1tJkoJ\_\_MgaAw9g9k5y2czeY_dGok4W4O3TjZKXqB53kcBYzNvw

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 3 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully [32m✓ document_ttl.go
completed successfully[0m [34m === Running websocket_ttl.go ===[0m ✓
Authentication successful

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
xPXDMPwhplbRIiIEawZoom8x9ZFQoS7Wp09XQCz_Z1MabQ_ha0mhx7pcHHqxTkHjo8KIHAVjsZPVLJSU0QjsLA

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 3 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully [32m✓ websocket_ttl.go completed
successfully[0m [34m === Running http_functions.go ===[0m 🚀 ekoDB Scripts
Example (Go/HTTP)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved:
tAw_KhuiiZFnma75AiqXoEdQ52lr7OZNwacq3Sku9cZvPvmRHfVwem1ZxPe0qapP1j42E1Xf4whcMe0YD91BUw
📊 Found 50 active users

📝 Example 2: Complex Filter with Multiple Conditions

✅ Script saved:
0NwCAqNnrjNo31Q-JJ1DYFZ97sGHxNTaiMPWxfaBu2rnVp71-3z6mv19_7NWRoY3rVpmldYdcGIcmot7sAQT8w
📊 Found 10 users (status=active, score>50, sorted by score)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved:
8HBZodO0GakL3GhLmyW0-qyYMh0GWEWK2K4vC_OFMO1H8PrVNRr_ZUxdzlBXQojvjrgISZEqqASDSlTuXzYjNw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
{"avg_score":50,"count":25,"max_score":90,"status":"inactive"}
{"avg_score":60,"count":25,"max_score":100,"status":"active"}

📝 Example 4: Function Management

📋 Total scripts: 11 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

✅ All examples completed! [32m✓ http_functions.go completed successfully[0m
[34m === Running transactions.go ===[0m ✓ Authentication successful

=== Setup: Creating Test Accounts === Created Alice: $1000 - ID:
lxV4qH6c8xTCOXzrKuAdJnFXpBDfR6VwZE4XWG05OK-PPhgAKoaGcpkHbUJXMbTdCL1y8I2uXyuP4qD8DHDlrw
Created Bob: $500 - ID:
2cUUWwA2Rkh9m_Uy5MaZBI67QSMe7MahtAjxdm2a5JZSK5Fp3Axr2bXGyiVLcFgLTK3_wB5wlN4DB1eIWVo8jA

=== Example 1: Begin Transaction === Transaction ID:
0c4e1f7b-a2bc-4055-a14e-0b0e61d85c84

=== Example 2: Operations with transaction_id === Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status === Status: Active Operations: 2

=== Example 4: Commit Transaction === ✓ Transaction committed

=== Verification === Alice: map[type:Integer value:800] Bob: map[type:Integer
value:700]

=== Example 5: Rollback === New transaction:
10a6e703-e4de-4bcc-a595-7455fa98c5b0 Updated Bob: $700 → $600 (in transaction) ✓
Transaction rolled back Bob after rollback: map[type:Integer value:700]

=== Cleanup === ✓ Deleted test accounts

✓ All transaction examples completed [32m✓ transactions.go completed
successfully[0m [34m ╔════════════════════════════════════════╗[0m [34m║ Test
Summary ║[0m [34m╚════════════════════════════════════════╝[0m [34mTotal: 9[0m
[32mPassed: 9[0m [32mFailed: 0[0m ✅ [32mGo direct examples complete![0m 🧪
[36mRunning Go client library examples...[0m ✓ Client created (token exchange
happens automatically)

=== Insert Document === Inserted:
map[id:1atedczb3u31f46pk59IRljU_e7FQwU6ch6ERy8z83b6zGImru8AWJ7r0J6mDEFEwfqejf6EUrmFHRJaX8-ABA]

=== Find by ID === Found: map[active:map[type:Boolean value:true]
id:1atedczb3u31f46pk59IRljU_e7FQwU6ch6ERy8z83b6zGImru8AWJ7r0J6mDEFEwfqejf6EUrmFHRJaX8-ABA
name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query === Found documents: 1

=== Update Document === Updated: map[active:map[type:Boolean value:true]
id:1atedczb3u31f46pk59IRljU_e7FQwU6ch6ERy8z83b6zGImru8AWJ7r0J6mDEFEwfqejf6EUrmFHRJaX8-ABA
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
exuRt59Y2dAfU7csMOy58KNKIDfx0GPxNTV-i7Hx05vGXUC6UVJZnILTwLvfO7yMflw_D1yYBu7orwIk3m7WKw

=== List Collections === Total collections: 15 Sample collections:
[schema_employees_client_py schema_users_client_py websocket_test
test_collection schema_documents_client_py]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
mcCNX0jkk8zWBL7bl5cCL-BJ5-GtlyFZ1TVif2df2iqQrsxXVF3Vui3y_1BcpYXWGaiSgL2Oid37mMgjAyGJgA

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
nGxi9YqagjSJHZXAhQwlQvI0B2kGWC2t-IppSIfUUwqQ9it8E_DQseh6su18keT6SLhuRfNgyhK7Cem8rfiVig

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
0yPjCtOPNJ9WZ12wPBEMPaoN--\_6Lv_dh28cLbbiSh2v_W1MCz8xuUZS2qqbREbBZ4vi_T9OVKpESHL8p6dzBg

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
UrcLm3AsyZAWVrQ4gXyPj3DzU95NFdJvxyWR6tRKo_K4d_wobUo2q88fIp7Fq_Miqxke3dykDNwLw2P6XJPYNg

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration === ekoDB Chat Basic Example ===

=== Inserting Sample Data === ✓ Inserted 3 sample documents

=== Creating Chat Session === ✓ Created session:
oWpN5T2vfqfqTqW59MzTvosMCC_kV2m9afaABNUB4Fq93Hk_V1wOdqrQsV8wgRuWrKjyDl0A19tWZ_kFg0r6XQ

=== Sending Chat Message === Message ID:
ltCbOFBxhAI1QwZEW_Ko752geKsWUYU6drukrqkRFoPN3wAWjPoe7Ww_JkLfztKCVE7lCcgMzxFZOF8vJObqcA

=== AI Response === I'm sorry, but the context provided does not include
specific product names or their prices. Could you provide more information or
specify what type of products you're interested in?

=== Context Used (3 snippets) === Snippet 1: map[collection:client_chat_basic_go
matched_fields:[description] record:map[description:map[type:String value:A
high-performance database product with AI capabilities]
id:GCHzh04nLYJaGt25kdQdgIHr-T7ISKJJgpXy_6dL8Celogrhhje4Zdj1wI4SpbM5LCE81UPsdaYmI_ypsdwzLQ
name:map[type:String value:ekoDB] price:map[type:Integer value:99]]
score:0.1111111111111111] Snippet 2: map[collection:client_chat_basic_go
matched_fields:[description] record:map[description:map[type:String
value:Enterprise edition product with advanced features]
id:UoqP-8bhFBz5GhaVPwzBnGktAYB07AbR97NivkjJOo3WcgqZVn4UsLZdCe97LH3PwWMqv2IlhviM7CtGGJ6mpQ
name:map[type:String value:ekoDB Pro] price:map[type:Integer value:299]]
score:0.1111111111111111] Snippet 3: map[collection:client_chat_basic_go
matched_fields:[description] record:map[description:map[type:String value:Fully
managed cloud database service product]
id:K5uIXkmmKeLjfgC_hwUb1OtIXuBDnBU7sLMPtWLj8OlFTZnjvUbvAtB\_\_2fV6aI4eWmQr-moUOWVqETseN6t7A
name:map[type:String value:ekoDB Cloud] price:map[type:Integer value:499]]
score:0.1111111111111111]

Execution Time: 2125ms

=== Token Usage === Prompt tokens: 321 Completion tokens: 34 Total tokens: 355

=== Cleanup === ✓ Deleted collection

✓ Chat completed successfully === ekoDB Chat Session Management Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
GRKGm3YelUoMrdfx0o_hpFj4N7PxezKTNrdcMsXTvFu1kSlg7otVecgeEkdSB-onud-TB671Yc6WuBsUS28daQ

=== Sending Messages === ✓ Message 1 sent Response: I'm sorry, but the context
provided does not give information about any specific products.

✓ Message 2 sent Response: Apologies, but there's no information provided in the
context about any specific product or its price. Could you please specify the
product you're interested in?

=== Retrieving Session Messages === ✓ Retrieved 4 messages

=== Updating Session === ✓ Session updated

=== Branching Session === ✓ Created branch:
W7FdHB52TWHr1YbgZdRpm-DyLX5loe7TP2G3hN3NFineUR8NhNqb0-wt4fMbuRjuJCOMgNXfw--Vv_cKdljcig
Parent:
GRKGm3YelUoMrdfx0o_hpFj4N7PxezKTNrdcMsXTvFu1kSlg7otVecgeEkdSB-onud-TB671Yc6WuBsUS28daQ

=== Listing Sessions === ✓ Found 9 sessions Session 1:
W7FdHB52TWHr1YbgZdRpm-DyLX5loe7TP2G3hN3NFineUR8NhNqb0-wt4fMbuRjuJCOMgNXfw--Vv_cKdljcig
(Untitled) Session 2:
GRKGm3YelUoMrdfx0o_hpFj4N7PxezKTNrdcMsXTvFu1kSlg7otVecgeEkdSB-onud-TB671Yc6WuBsUS28daQ
(Untitled) Session 3:
oWpN5T2vfqfqTqW59MzTvosMCC_kV2m9afaABNUB4Fq93Hk_V1wOdqrQsV8wgRuWrKjyDl0A19tWZ_kFg0r6XQ
(Untitled) Session 4:
3dKLTjbKKdWiKWJMiZSl27k-QOuXrlUsrkYfN3u6MHsPZpxafROQqHN7VqKXYIA_cDw_ok8lLtRs1RHmTZtdpg
(Untitled) Session 5:
jl_qC01XuY7UlWB31pIZDlb5ZT28fbBT40JNtj-beGEb5IMZTAfAVmzBAP5t6TIvQfE-17axFs_WESUzv_q1lA
(Untitled) Session 6:
GcJoGq8p4I9IzQJSpC3mOI9-Knp0ck9RKI25PN7C5bKsw0x4b-iHoBJ16NxTCck-JY9EZKnSBXcCVt5Uqoo2Qw
(Untitled) Session 7:
DZrN-AQV9Nn4wOTelR2KA6397oce29THpcBtxbI_iT2QOQo1yaK92LZIYuZGB_sN1pUrtH1Cg8FTgApuh7BRoA
(Untitled) Session 8:
ibO8oA4VdNKJx8MlQKtRtIuVB5WanPjnMLOrJSIIietcdTJk5n1aMoIgHOY00veFNThz80mq1tJMflxUHEmqEg
(Untitled) Session 9:
wFcIqUdFKFba7_NRR2SD1v1j68swGE6y6e1ZCGgA8_XfIMj_dJ_fXiQhjVuSR3TrnICo0OW2clU5xYAVpYChpw
(Untitled)

=== Getting Session Details === ✓ Session details retrieved Messages: 4

=== Deleting Branch Session === ✓ Deleted branch session:
W7FdHB52TWHr1YbgZdRpm-DyLX5loe7TP2G3hN3NFineUR8NhNqb0-wt4fMbuRjuJCOMgNXfw--Vv_cKdljcig

=== Cleanup === ✓ Deleted collection

✓ All session management operations completed successfully === ekoDB Advanced
Chat Features Example ===

=== Inserting Sample Data === ✓ Inserted sample product

=== Creating Chat Session === ✓ Created session:
PUQ4mLmnC_HL-at_Noi3NmO0d3uh9xLt6evxS7oSIy2Hmfyqy5dnWuuk2ci8J-FFh4zdgAfKPE8kXLssIYqsEQ

=== Sending Initial Message === ✓ Message sent Response: I'm sorry, but the
context provided does not give specific information about any available
products. Could you please provide more details or a specific product category
you are interested in?

✓ Second message sent

=== Feature 1: Regenerate AI Response === ✓ Message regenerated New response:
Answer: I'm sorry, but the context provided doesn't contain information about
the price of ekoDB. Please provide more details or you may need to check the
official website or contact the supplier for the correct pricing.

=== Feature 2: Edit Message === ✓ Message content updated

=== Feature 3: Mark Message as Forgotten === ✓ Message marked as forgotten
(excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions === ✓ Created second session:
-FJ4CVqdG5Gip24DhL4UpNnFgCiZxYnXepzawKad4RjbD6jL_BV5WwyqS4NVrBLHohr1LJ5o7OlQj4JlhDcCbQ
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
1. Score: 0.772
1. Score: 0.753
1. Score: 0.750

1. Hybrid search (text + vector): Found 3 results using hybrid search (text +
   vector)
1. Score: 2.709
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

- Alice Johnson: Engineering
- Bob Smith: Sales

2. Join with filtering: Found 1 users in Engineering

- Alice Johnson: Building A

3. Join with user profiles: Found 2 users with profile data

- Alice Johnson: Senior Software Engineer
- Bob Smith: Sales Manager

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
iTgzvugR5nqShziGkGfAkQjwHF8esGPNMXRmfE7ljoxd-tEkajYDxj38NiFB-okJLAxd0x_zlvhMcghaTbCdpw
📊 Found 60 records ⏱️ Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved 📊 Found 60 users (limited) ⏱️ Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved 📊 Statistics: 2 groups map[avg_score:60 count:30 status:active]
map[avg_score:50 count:30 status:inactive] ⏱️ Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 13 🔍 Retrieved script: Get Active Users ✏️ Script updated 🗑️
Script deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

📝 Example 5: Multi-Stage Pipeline

✅ Multi-stage script saved 📊 Pipeline executed 2 stages ⏱️ Total execution
time: 0ms 📈 Stage breakdown:

📝 Example 6: Count Users

✅ Count script saved 📊 Total user count: 60 ⏱️ Execution time: 0ms

🧹 Cleaning up... ✅ Deleted collection ✅ Deleted test scripts

✅ All examples completed successfully!

💡 Key Advantages of Using the Client: • Automatic token management • Type-safe
Stage builders • Built-in error handling ✅ [32mGo client examples complete![0m
