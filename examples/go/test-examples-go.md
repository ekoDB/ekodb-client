make test-examples-go 🧪 [36mRunning Go examples (direct HTTP/WebSocket)...[0m
[34m ╔════════════════════════════════════════╗[0m [34m║ ekoDB Go Examples Test
Suite ║[0m [34m╚════════════════════════════════════════╝[0m [34m === Checking
Server Connection ===[0m [32m✓ Server is ready[0m [33m === Running 8 Examples
===[0m [34m === Running simple_crud.go ===[0m ✓ Authentication successful

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
