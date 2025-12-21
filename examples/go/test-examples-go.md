make test-examples-go 🧪 [36mRunning Go examples (direct HTTP/WebSocket)...[0m
[34m ╔════════════════════════════════════════╗[0m [34m║ ekoDB Go Examples Test
Suite ║[0m [34m╚════════════════════════════════════════╝[0m [34m === Checking
Server Connection ===[0m [32m✓ Server is ready[0m [33m === Running 10 Examples
===[0m [34m === Running simple_crud.go ===[0m ✓ Authentication successful

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
