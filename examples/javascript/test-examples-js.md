make test-examples-javascript 🧪 [36mRunning JavaScript examples (direct
HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 643ms

1 package is looking for funding run `npm fund` for details

found 0 vulnerabilities [34m ╔════════════════════════════════════════╗[0m [34m║
ekoDB JavaScript Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m [34m === Checking Server
Connection ===[0m (node:4040) [DEP0040] DeprecationWarning: The `punycode`
module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) [32m✓
Server is ready[0m [34m === Getting Authentication Token ===[0m [32m✓
Authentication successful[0m [33m === Running 7 Examples ===[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_crud.js
===[0m ✓ Authentication successful

=== Insert Document === (node:4042) [DEP0040] DeprecationWarning: The `punycode`
module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) Inserted:
{ id:
'MPf0hPUWhISXnFyaMEMN3hdLZ5oBnC2JVa8wFylXaEbPDbuEykz2yhKRo-tYD-aJJ82VmYPm0UI8iJciEvMLJQ'
}

=== Find by ID === Found: { name: { type: 'String', value: 'Test Record' }, id:
'MPf0hPUWhISXnFyaMEMN3hdLZ5oBnC2JVa8wFylXaEbPDbuEykz2yhKRo-tYD-aJJ82VmYPm0UI8iJciEvMLJQ',
value: { type: 'Integer', value: 42 }, active: { value: true, type: 'Boolean' }
}

=== Find with Query === Found documents: 1

=== Update Document === Updated: { id:
'MPf0hPUWhISXnFyaMEMN3hdLZ5oBnC2JVa8wFylXaEbPDbuEykz2yhKRo-tYD-aJJ82VmYPm0UI8iJciEvMLJQ',
value: { type: 'Integer', value: 100 }, name: { value: 'Updated Record', type:
'String' }, active: { value: true, type: 'Boolean' } }

=== Delete Document === Deleted document

✓ All CRUD operations completed successfully [32m✓ simple_crud.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_websocket.js
===[0m ✓ Authentication successful

=== Inserting Test Data === (node:4054) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Inserted
test record:
Se3UvJ0XI7OyDDEKDxZ7JtsiEFdNXRxU-xtiLI-koXCy0hnZvBSXDQ5Mb1TPSSJkfldUF9qBrW24AdUTvJWZ_w

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === Response: { "payload": { "data": [ {
"active": { "type": "Boolean", "value": true }, "id":
"3w36KS3micZ5Sx6JR842Xfz3tMv0TGF8mEiZuZQug0PjIq9F8NIVLPIicjsMLRFBoh5oyoGjV_-pFNM8Vo2rBw",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"Se3UvJ0XI7OyDDEKDxZ7JtsiEFdNXRxU-xtiLI-koXCy0hnZvBSXDQ5Mb1TPSSJkfldUF9qBrW24AdUTvJWZ_w",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"WrJRSnl-xSMYxP45vOAOl7aa7ol-xCmeDuXpZABUjpuYjnidLLF93I7dbbLK_eGfc2WUEPGJESmErjkTweqLRQ",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } }, { "active": { "type": "Boolean", "value":
true }, "id":
"6zgKLocRk8WYRfFz-J1dF0GPcsGdsemFXr7AyajGSwtUPG0vJlbWvqUCsZlMtjaTUx4KY3S3gw_ebP0Jy8A1kQ",
"name": { "type": "String", "value": "WebSocket Test Record" }, "value": {
"type": "Integer", "value": 42 } } ] }, "type": "Success" } ✓ Retrieved 4
record(s) via WebSocket

✓ WebSocket example completed successfully [32m✓ simple_websocket.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/batch_operations.js
===[0m ✓ Authentication successful

=== Batch Insert === (node:4057) [DEP0040] DeprecationWarning: The `punycode`
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

=== KV Set === (node:4070) [DEP0040] DeprecationWarning: The `punycode` module
is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Set key:
session:user123

=== KV Get === Retrieved value: { username: 'john_doe', loginTime:
'2025-11-08T06:24:04.122Z', userId: 123 }

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: { name: 'Product 1', price: 29.99 }
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully [32m✓ kv_operations.js completed
successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/collection_management.js
===[0m ✓ Authentication successful

=== Create Collection (via insert) === (node:4079) [DEP0040] DeprecationWarning:
The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
Collection created with first record:
fcFCBy3UuibObZImRPYAm_0k5EgvWJh3at4XzIxDAXiu2Iq77qtUS4GKyXz2L1E_JWTNOiBmgyZbA0VxXwyPVA

=== List Collections === Total collections: 22 Sample collections: [
'functions__ek0_bench', 'schema_products_client_ts', 'websocket_test',
'schema_documents_client_go', 'chat_configurations__ek0_bench' ]

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully [32m✓
collection_management.js completed successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/document_ttl.js
===[0m ✓ Authentication successful

=== Insert Document with TTL (1 hour) === (node:4081) [DEP0040]
DeprecationWarning: The `punycode` module is deprecated. Please use a userland
alternative instead. (Use `node --trace-deprecation ...` to show where the
warning was created) ✓ Inserted document:
8UKOAxku9qyZi-JxAgz1SVEstNE9QG-4y1K-erSe6ZyBRzW52PgzOYcJnNvVARxW8MSy81a\_-iB_cP2UyA_k1Q

=== Insert Document with TTL (5 minutes) === ✓ Inserted document: created

=== Query Documents === ✓ Found 4 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully [32m✓ document_ttl.js
completed successfully[0m [34m === Running
/Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/websocket_ttl.js
===[0m ✓ Authentication successful

=== Insert Test Data with TTL === (node:4101) [DEP0040] DeprecationWarning: The
`punycode` module is deprecated. Please use a userland alternative instead. (Use
`node --trace-deprecation ...` to show where the warning was created) ✓ Inserted
document with TTL:
bGcqdaDurAkfD6aFYHK9uj3OUSVtJYSQxfjA9AjbbgJJQYj_XuDkw0f9U_qYX40OFms8tVmpwmXyVoq-v828Dg

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 4 record(s) via
WebSocket

✓ WebSocket TTL example completed successfully [32m✓ websocket_ttl.js completed
successfully[0m [34m ╔════════════════════════════════════════╗[0m [34m║ Test
Summary ║[0m [34m╚════════════════════════════════════════╝[0m [34mTotal: 7[0m
[32mPassed: 7[0m [32mFailed: 0[0m ✅ [32mJavaScript direct examples complete![0m
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.3.0 prepare npm run build

> @ekodb/ekodb-client@0.3.0 build tsc

up to date, audited 7 packages in 900ms

found 0 vulnerabilities

> @ekodb/ekodb-client@0.3.0 build tsc

✅ [32mTypeScript client built![0m 🧪 [36mRunning JavaScript client library
examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 964ms

1 package is looking for funding run `npm fund` for details

found 0 vulnerabilities ✓ Client created (token exchange happens automatically)

=== Insert Document === Inserted: { id:
'XD9LloQ-pu2va6jg0Pr0ocgU8vget5Wn-VBJbshmxsaLv6RPpyQ2rj0JsENDiDjhONlsCmFe7i53NOKh85Bshg'
}

=== Find by ID === Found: { id:
'XD9LloQ-pu2va6jg0Pr0ocgU8vget5Wn-VBJbshmxsaLv6RPpyQ2rj0JsENDiDjhONlsCmFe7i53NOKh85Bshg',
name: { value: 'Test Record', type: 'String' }, active: { value: true, type:
'Boolean' }, value: { type: 'Integer', value: 42 } }

=== Find with Query === Found documents: 1

=== Update Document === Updated: { id:
'XD9LloQ-pu2va6jg0Pr0ocgU8vget5Wn-VBJbshmxsaLv6RPpyQ2rj0JsENDiDjhONlsCmFe7i53NOKh85Bshg',
active: { value: true, type: 'Boolean' }, name: { type: 'String', value:
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

=== KV Get === Retrieved value: { username: 'john_doe', userId: 123 }

=== Set Multiple Keys === ✓ Set 3 keys

=== Get Multiple Keys === cache:product:1: { name: 'Product 1', price: 29.99 }
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

=== KV Delete === ✓ Deleted key: session:user123 ✓ Verified: Key successfully
deleted (not found)

=== Delete Multiple Keys === ✓ Deleted 3 keys

✓ All KV operations completed successfully ✓ Client created

=== Create Collection (via insert) === Collection created with first record:
yR862PjnlNCsIFGFnNB8oQDKBMOEowpOj0wjMCit40b7SklyLiwPjNEUyfL8jubM9dJ-M3n9ucaoufEx5x1EJw

=== List Collections === Total collections: 22 Sample collections:
functions**ek0_bench,schema_products_client_ts,websocket_test,schema_documents_client_go,chat_configurations**ek0_bench

=== Count Documents === Document count: 1

=== Delete Collection === Collection deleted successfully

=== Verify Deletion === Collection still exists: false

✓ All collection management operations completed successfully ✓ Client created

=== Insert Document with TTL (1 hour) === ✓ Inserted document:
E-GrBapZEh7Wu4QbeAk7TY40SxbURQftVcF5PnteAO5tsBwH-y-fZwa9SfMwIi35Whtl9bdkBv-0FJ-QWgxQAg

=== Insert Document with TTL (5 minutes) === ✓ Inserted document:
D28QdDPfJ3a5H4pdFZYF5JSPOCeOm09IiCDaXhogQ6JV0TrWyb-fvmZiA3Rytle5F2UjoVCZkgDrYbp1pbSDdA

=== Query Documents === ✓ Found 2 documents with TTL

=== Update Document === ✓ Updated document

=== Delete Document === ✓ Deleted document

=== Cleanup === ✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration ✓ Client created

=== Inserting Test Data === ✓ Inserted test record:
JcXmfSppsZs4j6StB4H4RO6eEzXGt-AksJkBSz3SfcxZt3V07Yo2hPnudn6u42YKKi0HQKya91D1bMJQo5aYTw

=== Connecting to WebSocket === ✓ WebSocket connected

=== Querying Data via WebSocket === ✓ Retrieved 1 record(s) via WebSocket Record
1: 4 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket example completed successfully ✓ Client created

=== Insert Test Data with TTL === ✓ Inserted document with TTL:
IHyIyhz2KzDs7qXQ3Iu-7TxGG15l6y1EQarA1qVFM51fcXuLMUoY9oLFhWwqS0z4EaAlSLegerKwgiixzzIdxg

=== Query via WebSocket === ✓ WebSocket connected ✓ Retrieved 1 record(s) via
WebSocket Record 1: 5 fields

=== Cleanup === ✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified
duration 🚀 ekoDB Saved Functions Example (JavaScript/HTTP)

📋 Setting up test data... ✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved:
uP3DfUegqTlipQMnBmXjjQDGeV4_ms2U8Knt3mY6CUmULZSrbbtKcLWi_1PJweH2QO-Y9iZTjFIbu7rDLw8-LQ
📊 Found 50 active users

📝 Example 2: Parameterized Function

✅ Function saved:
hrTJEjQS61FbvXchoRp4NQU40e3Z2C_9Cwefx7q7LPNNVz_ikoobWNdbHDZQiGKG9rQbm7hvyjsjKPecDnFUSQ
📊 Found 50 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved:
XGm82zQ_KAdz-zGba6hejkgu7qdCOhMAafPmAmwQb8JAiIuxJBG91WDqsTsoz6lOrALJ0kyGxrcdM2JpCvKOGw
📊 Statistics: 2 groups {"avg_score":60,"count":25,"status":"active"}
{"count":25,"avg_score":50,"status":"inactive"}

📝 Example 4: Function Management

📋 Total functions: 11 🔍 Retrieved function: Get Active Users ✏️ Function
updated 🗑️ Function deleted

ℹ️ Note: GET/UPDATE/DELETE operations require the encrypted ID ℹ️ Only CALL can
use either ID or label

✅ All examples completed! ✅ [32mJavaScript client examples complete![0m
