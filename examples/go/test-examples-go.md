make test-examples-go
🧪 Running Go examples (direct HTTP/WebSocket)...

╔════════════════════════════════════════╗
║     ekoDB Go Examples Test Suite      ║
╚════════════════════════════════════════╝

=== Checking Server Connection ===
✓ Server is ready

=== Running 10 Examples ===

=== Running simple_crud.go ===
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: map[id:3cDtgJ8YImcqpoNzRSaL1jXzcwjFwdhJ44mVVUN_cW0sLSD1yEP7re6yct1pFy5o42n3aR6nyFcDP-b2gjbrPw]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:3cDtgJ8YImcqpoNzRSaL1jXzcwjFwdhJ44mVVUN_cW0sLSD1yEP7re6yct1pFy5o42n3aR6nyFcDP-b2gjbrPw name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found documents: [map[active:map[type:Boolean value:true] id:3cDtgJ8YImcqpoNzRSaL1jXzcwjFwdhJ44mVVUN_cW0sLSD1yEP7re6yct1pFy5o42n3aR6nyFcDP-b2gjbrPw name:map[type:String value:Test Record] value:map[type:Integer value:42]]]

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:3cDtgJ8YImcqpoNzRSaL1jXzcwjFwdhJ44mVVUN_cW0sLSD1yEP7re6yct1pFy5o42n3aR6nyFcDP-b2gjbrPw name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ simple_crud.go completed successfully

=== Running simple_websocket.go ===
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: afrAOAVaIEvOpvRrw6pyCxkAyU5wKGa-XFEQoI0IY3KuDbP4nWD5GVPFbV_fpfIObaSWUuC5Dkdie_pjXlscpw

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1789566411971623000",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "afrAOAVaIEvOpvRrw6pyCxkAyU5wKGa-XFEQoI0IY3KuDbP4nWD5GVPFbV_fpfIObaSWUuC5Dkdie_pjXlscpw",
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "value": {
          "type": "Integer",
          "value": 42
        }
      }
    ]
  },
  "type": "Success"
}
✓ Retrieved 1 record via WebSocket

✓ WebSocket example completed successfully
✓ simple_websocket.go completed successfully

=== Running http_functions.go ===
🚀 ekoDB Functions Example (Go/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: YsG6KMZ-hTWinlpD4a0elRYnIcReSpigV_zuV2hmfPZC8hvHBLvnzR2Be1hPDnBDCxR1EMX1Idhs08JBImGCaA
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: erbMBGMKAYZDW7GRbuzHGDWwwABomK5QzMQzfCrczew8Q4N907mT-QJ8Z-rtcmRWDUcDYm7wD4ZY_tBbjKiP4A
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: -2gey2scpc8ohr_-N1HRpQqpK4_0khQHl4ABhwCXieSItRaPbQvCuQmn3ag3FnYrMnhjG5OALJFa8gKJweayaw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":{"type":"Float","value":60},"count":{"type":"Integer","value":5},"max_score":{"type":"Integer","value":100},"status":{"type":"String","value":"active"}}
   {"avg_score":{"type":"Float","value":50},"count":{"type":"Integer","value":5},"max_score":{"type":"Integer","value":90},"status":{"type":"String","value":"inactive"}}

📝 Example 4: Function Management

📋 Total functions: 3
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
✓ http_functions.go completed successfully

=== Running batch_operations.go ===
=== Batch Operations (Direct HTTP) ===

✓ Authentication successful

=== Batch Insert ===
✓ Batch inserted 5 records

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
✓ Batch updated 3 records

=== Batch Delete ===
✓ Batch deleted 3 records
✓ Verified: Records successfully deleted (not found)

✓ All batch operations completed successfully
✓ batch_operations.go completed successfully

=== Running kv_operations.go ===
=== Key-Value Operations (Direct HTTP) ===

✓ Authentication successful

=== KV Set ===
✓ Set key: session:user123:go

=== KV Get ===
Retrieved value: map[type:Object value:map[userId:123 username:john_doe]]

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1:go: map[type:Object value:map[name:Product 1 price:29.99]]
cache:product:2:go: map[type:Object value:map[name:Product 2 price:39.989999999999995]]
cache:product:3:go: map[type:Object value:map[name:Product 3 price:49.989999999999995]]

=== KV Delete ===
✓ Deleted key: session:user123:go
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ kv_operations.go completed successfully

=== Running collection_management.go ===
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: xdawGWVX7U75PRcn2v5xVvayVu2XzZco5qWZ9h0Z8dOmosJsF5VRhKHOIGWNvgy6DKToDovOf9DUbzcaRO5wEg

=== List Collections ===
Total collections: 13
Sample collections: [chat_agent_configs__ek0_testing agent_function_versions__ek0_testing schedules__ek0_testing functions__ek0_testing chat_configurations__ek0_testing]

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ collection_management.go completed successfully

=== Running transactions.go ===
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: kDqCp7255ht4xT6yl5MyA6UqMOyng1E7bhBGei1VT8BjX0LOC27xGclB-3doPjw_u0z2aLAD5tZo-pkfSNGhSg
Created Bob: $500 - ID: uSzfWL_ISVDKl268p120F8d4zaFEJvTT18jb3DBN_4ZNw4sEhqSbJxU7O7jdUM1G43Y7ZaN9VpNe-2EL55oAiw

=== Example 1: Begin Transaction ===
Transaction ID: 625bf002-a208-45fe-856d-86bc3928ff4f

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: map[type:Integer value:800]
Bob: map[type:Integer value:700]

=== Example 5: Rollback ===
New transaction: 700b95c2-05de-45e1-aebd-bb263d905b8d
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: map[type:Integer value:700]

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✓ transactions.go completed successfully

=== Running crud_functions.go ===
🚀 ekoDB Complete CRUD Functions Example
============================================================
Demonstrates:
  • Insert + Verify (using Query)
  • Query + Update Status + Verify
  • Query + Update Credits + Verify
  • Query Before Delete + Delete + Verify Gone

Each function shows Functions chaining with proper verification
============================================================

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted collection: crud_functions_users_go

============================================================
📝 function 1: Insert + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: WQ7QGzNt8Bi15nqhRwCJoEW7X8ANXQD6pT40arjLmSfxsLJg-yrlC8JMe2qJrGpqapfjtM2VZNQn9t815NAQDA

2️⃣ Calling function (Insert + Verify)...
   ✅ function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: ZjgY4Q8XTpUtNLukT5-8q2f-rUDs-Jx7e0eEkioJpa8Y2nwqTg_5sx4fwpxHbWHRvsIek3fqxIEWtVFyWjymcw
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: 6kH5sL2JPPHor0HtxY8gHRMyUpO_MEco6XPf2N6hdPJL65P3kp7HfjCtnaHph9XcnElre6Mcl0w1Ns5bzqDUQg

2️⃣ Calling function (Query + Update + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: map[type:String value:active]
   📋 Name: map[type:String value:Alice Smith]

============================================================
📝 function 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: 2Tkp_K83vTpVN_vrIVYsgsOqYm8KW9nr520vmRITPHzBoNpqGTvZOLF27kAn8jj0N2FhdTbAPenMtLL9QSLh6A

2️⃣ Calling function (Query + Update Credits + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: map[type:Integer value:100]
   📋 Status: map[type:String value:active]
   📋 Name: map[type:String value:Alice Smith]

============================================================
📝 function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: qaw4Ey-cS-AU6kw0q23uY5e-4K30XbiU6vr3DmnSSa-RYqEA9kx4OQK2Ev7oLekAfX25smbMM3hdKS7U0qurGQ

2️⃣ Calling function (Query + Delete + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: WQ7QGzNt8Bi15nqhRwCJ...
   ✅ Deleted script: 6kH5sL2JPPHor0HtxY8g...
   ✅ Deleted script: 2Tkp_K83vTpVN_vrIVYs...
   ✅ Deleted script: qaw4Ey-cS-AU6kw0q23u...
   ✅ Deleted collection: crud_functions_users_go

============================================================
✅ Complete CRUD Functions Example Finished!
============================================================

💡 Key Takeaways:
   ✅ Functions chain steps together
   ✅ Each function demonstrates operation + verification
   ✅ Parameters make functions reusable
   ✅ Verification is built into the function itself
   ✅ Complete CRUD lifecycle in 4 focused functions
✓ crud_functions.go completed successfully

=== Running document_ttl.go ===
╔════════════════════════════════════════════════════════╗
║     TTL EXPIRATION VERIFICATION TEST                   ║
╚════════════════════════════════════════════════════════╝

This test verifies that document TTL expiration works correctly.
We will insert documents with short TTL and verify they expire.

✓ Client connected

═══════════════════════════════════════════════════════════
TEST 1: Document TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = UMiDiPCaKwweKMB_xN2ODrvOVt4upxVr6AggqfAVEBqJZL8tvcuya2-_R9s8gDFJlkIUjaHxMEmsrnbH1ewIoQ
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: FindByID(UMiDiPCaKwweKMB_xN2ODrvOVt4upxVr6AggqfAVEBqJZL8tvcuya2-_R9s8gDFJlkIUjaHxMEmsrnbH1ewIoQ)
  Output: Found document with name = map[type:String value:TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: FindByID(UMiDiPCaKwweKMB_xN2ODrvOVt4upxVr6AggqfAVEBqJZL8tvcuya2-_R9s8gDFJlkIUjaHxMEmsrnbH1ewIoQ)
  Output: Error (expected) - request failed with status 404: ��error�Record not found (expired)
  ✓ PASS: Document expired (not found error)

═══════════════════════════════════════════════════════════
CLEANUP
═══════════════════════════════════════════════════════════

╔════════════════════════════════════════════════════════╗
║              ALL TTL TESTS PASSED ✓                    ║
╚════════════════════════════════════════════════════════╝

TTL expiration is working correctly:
  • Documents with TTL expire after the specified time
  • Documents without TTL persist indefinitely
  • Different TTL durations are handled correctly
✓ Deleted test collection
✓ document_ttl.go completed successfully

=== Running websocket_ttl.go ===
╔════════════════════════════════════════════════════════╗
║   WEBSOCKET TTL EXPIRATION VERIFICATION TEST           ║
╚════════════════════════════════════════════════════════╝

This test verifies TTL expiration works via WebSocket connections.
We will use WebSocket to insert, query, and verify TTL expiration.

✓ Client connected

═══════════════════════════════════════════════════════════
TEST: WebSocket TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'WS TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = wsm9w_8rgFq8WiUiSHGkTw6xRi0Z1iYNAcNZ04pH9XuDGgiGwqxCzEgujKiLX0PhPiaajPOJ04NBOgHVX8kmJw
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: FindByID(wsm9w_8rgFq8WiUiSHGkTw6xRi0Z1iYNAcNZ04pH9XuDGgiGwqxCzEgujKiLX0PhPiaajPOJ04NBOgHVX8kmJw)
  Output: Found document with name = map[type:String value:WS TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: FindByID(wsm9w_8rgFq8WiUiSHGkTw6xRi0Z1iYNAcNZ04pH9XuDGgiGwqxCzEgujKiLX0PhPiaajPOJ04NBOgHVX8kmJw)
  Output: Error (expected) - request failed with status 404: ��error�Record has been deleted
  ✓ PASS: Document expired (not found error)

═══════════════════════════════════════════════════════════
CLEANUP
═══════════════════════════════════════════════════════════

╔════════════════════════════════════════════════════════╗
║          WEBSOCKET TTL TEST PASSED ✓                   ║
╚════════════════════════════════════════════════════════╝

WebSocket TTL expiration is working correctly:
  • Documents with TTL inserted via client expire correctly
  • Queries correctly return nil for expired documents
✓ Deleted test collection
✓ websocket_ttl.go completed successfully

╔════════════════════════════════════════╗
║           Test Summary                 ║
╚════════════════════════════════════════╝
Total: 10
Passed: 10
Failed: 0
✅ Go direct examples complete!
bash: warning: setlocale: LC_ALL: cannot change locale (C.UTF-8): No such file or directory
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Advanced CRUD Integration Example (Go) ===

--- Inserting base record ---
Inserted record: xOwqTqSHkEiyLyyIpX1Gf9F_KuIphdvSOKdYE72YMsGTL9GutA2aIakCbIJjFvy8yUQSRHR5IsrJfERuyiUoOg

--- UpdateWithAction: increment ---
After increment by 5: count = map[type:Integer value:5]
After increment by 3: count = map[type:Integer value:8]

--- UpdateWithAction: push ---
After push 'new-tag': tags = map[type:Array value:[initial new-tag]]
After push 30.0: scores = map[type:Array value:[10 20 30]]

--- UpdateWithAction: decrement ---
After decrement by 2: count = map[type:Integer value:6]

--- UpdateWithActionSequence ---
After sequence: count=map[type:Integer value:16], tags=map[type:Array value:[initial new-tag batch-added]], scores=map[type:Array value:[10 20 30 99.9]]

--- Cleanup ---
Cleaned up all resources

=== All advanced CRUD operations completed successfully ===
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

=== Batch Insert ===
✓ Batch inserted 5 records
✓ Verified: Found 5 total records in collection

=== Batch Update ===
✓ Batch updated 3 records

=== Batch Delete ===
✓ Batch deleted 3 records

=== Cleanup ===
✓ Deleted collection

✓ All batch operations completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: Orz6u4lxCXBs0kRKIYjIGJHWiFRlecFy95GcnXCJI2cefsz6gEkeAksb1VAD_8oecMjrQKgrBvl36lddHQNDhQ

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you have any further questions or need more information, feel free to ask!

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: SKCgd5SHIA8LS45JEViLXuwdzsK4Akm6duSgSNKHgab7ipLee0ai-rTBphn6adJA4qaEnG6l1n5u6Ft9VBTuOw
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: SKCgd5SHIA8LS45JEViLXuwdzsK4Akm6duSgSNKHgab7ipLee0ai-rTBphn6adJA4qaEnG6l1n5u6Ft9VBTuOw
✓ Deleted chat session: Orz6u4lxCXBs0kRKIYjIGJHWiFRlecFy95GcnXCJI2cefsz6gEkeAksb1VAD_8oecMjrQKgrBvl36lddHQNDhQ
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: Ck1wc4wBqz7Nbp2CFll00gDXl6yusrAxyv7Kmb94ZCJM4MhTdIMbg0KPkvokmT-OXfKZAWZ1_H1HLZMrINtnyg

=== Sending Chat Message ===
Message ID: ez99c2APV1vR78cSJUK4_y_miaKAmo8oSJAdAbOmLvdoySiFRVo1KoYSC1Tij3IQpyRb9OgwC_uDQhSHFpHIQA

=== AI Response ===
The available products and their prices are as follows:

1. **ekoDB Cloud**: $499
   Description: Fully managed cloud database service product

2. **ekoDB**: $99
   Description: A high-performance database product with AI capabilities

3. **ekoDB Pro**: $299
   Description: Enterprise edition product with advanced features

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:pI8UKPTEZI2lkVPUmV7ADppA8_Ewb6HzPUQf0eCVpNj4pAKCcAMjYXnbB-cLA5Di8Z19PSPsXTDDN6BzzyZMEQ name:ekoDB Cloud price:499] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:AhdcahG8y964lJ33ZrYxyxBiNqmZ_z1Zv8_WmzZugcYPBLA8PErb4qKLQxDJcB9pff67ZaVcbwstu6mR8RnsnA name:ekoDB price:99] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:DoDFzC9Mq6d9T0-eic1xQstpyNiG-6lmUltCAIfERR5Mww_Bhgb2NGT-gDO8CuQe5JT-ak426Iznglc5UhvimQ name:ekoDB Pro price:299] score:0.1111111111111111]

Execution Time: 2347ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 82
Total tokens: 3495

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Chat Message Stream (SSE) Example (Go) ===

Created session: UkjY6eQn4Q9zX_3U2qQul6gXGRfMncWmzBJIMQP5JDmAFBbXQE3fhMt3KQ3IaWQixobjLndxGCJVeOsZtqOwGA

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}EkoDB is a high-performance, distributed database designed for applications that require low-latency and high throughput. It is particularly suited for use cases such as time-series data, event streaming, and large-scale real-time analytics.

Key features of ekoDB may include:

1. **Scalability**: EkoDB is designed to scale horizontally, allowing it to handle increasing amounts of data and requests by adding more servers to the cluster.

2. **Performance**: The database engine is optimized for fast read and write operations, making it ideal for high-load environments.

3. **Distributed Architecture**: EkoDB typically employs a distributed architecture, ensuring data redundancy and fault tolerance, which enhances reliability.

4. **Support for Various Data Models**: EkoDB might support different data models such as key-value, document, or column-family stores, catering to a range of application needs.

5. **Efficient Query Mechanisms**: It may offer indexing and querying features that improve the efficiency of data retrieval.

6. **Real-time Analytics**: Suitable for applications requiring real-time data processing and insights.

While EkoDB may not be as widely known as some other databases, it serves specific niches and use cases where performance and scalability are critical. If you have specific questions about EkoDB or need information on how to implement or use it, feel free to ask!

--- Stream complete ---
Message ID: dXm6qVlohpVc3kmwuen-Sgvj5FTxgcOaSUSsrpWZlFp9gPf8WmE_kBdaNMk2r-0dwI8lyJzdNYqSvckHIIY1jg
Execution time: 2369ms
Context window: 128000 tokens

✓ Chat message stream example completed
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

=== Get All Chat Models ===
OpenAI models: [text-embedding-ada-002 whisper-1 gpt-3.5-turbo tts-1 gpt-3.5-turbo-16k gpt-4-0613 gpt-4 davinci-002 babbage-002 gpt-3.5-turbo-instruct gpt-3.5-turbo-instruct-0914 gpt-3.5-turbo-1106 tts-1-hd tts-1-1106 tts-1-hd-1106 text-embedding-3-small text-embedding-3-large gpt-3.5-turbo-0125 gpt-4-turbo gpt-4-turbo-2024-04-09 gpt-4o gpt-4o-2024-05-13 gpt-4o-mini-2024-07-18 gpt-4o-mini gpt-4o-2024-08-06 omni-moderation-latest omni-moderation-2024-09-26 o1-2024-12-17 o1 o3-mini o3-mini-2025-01-31 gpt-4o-2024-11-20 gpt-4o-mini-search-preview-2025-03-11 gpt-4o-mini-search-preview gpt-4o-transcribe gpt-4o-mini-transcribe o1-pro-2025-03-19 o1-pro gpt-4o-mini-tts o3-2025-04-16 o4-mini-2025-04-16 o3 o4-mini gpt-4.1-2025-04-14 gpt-4.1 gpt-4.1-mini-2025-04-14 gpt-4.1-mini gpt-4.1-nano-2025-04-14 gpt-4.1-nano gpt-image-1 o4-mini-deep-research gpt-4o-transcribe-diarize o4-mini-deep-research-2025-06-26 gpt-5-chat-latest gpt-5-2025-08-07 gpt-5 gpt-5-mini-2025-08-07 gpt-5-mini gpt-5-nano-2025-08-07 gpt-5-nano gpt-audio-2025-08-28 gpt-realtime gpt-realtime-2025-08-28 gpt-audio gpt-5-codex gpt-image-1-mini gpt-5-pro-2025-10-06 gpt-5-pro gpt-audio-mini gpt-audio-mini-2025-10-06 gpt-5-search-api gpt-realtime-mini sora-2 sora-2-pro gpt-5-search-api-2025-10-14 gpt-5.1-chat-latest gpt-5.1-2025-11-13 gpt-5.1 gpt-5.1-codex gpt-5.1-codex-mini gpt-5.1-codex-max gpt-image-1.5 gpt-5.2-2025-12-11 gpt-5.2 gpt-5.2-pro-2025-12-11 gpt-5.2-pro gpt-5.2-chat-latest gpt-4o-mini-transcribe-2025-12-15 gpt-4o-mini-transcribe-2025-03-20 gpt-4o-mini-tts-2025-03-20 gpt-4o-mini-tts-2025-12-15 gpt-realtime-mini-2025-12-15 gpt-audio-mini-2025-12-15 chatgpt-image-latest gpt-5.2-codex gpt-5.3-codex gpt-realtime-1.5 gpt-audio-1.5 gpt-4o-search-preview gpt-4o-search-preview-2025-03-11 gpt-5.3-chat-latest gpt-5.4-2026-03-05 gpt-5.4-pro gpt-5.4-pro-2026-03-05 gpt-5.4 gpt-5.4-nano-2026-03-17 gpt-5.4-nano gpt-5.4-mini-2026-03-17 gpt-5.4-mini gpt-image-2 gpt-image-2-2026-04-21 gpt-5.5 gpt-5.5-2026-04-23 gpt-5.5-pro gpt-5.5-pro-2026-04-23 chat-latest gpt-realtime-translate gpt-realtime-2 gpt-realtime-whisper gpt-5.6-sol gpt-5.6-terra gpt-5.6-luna gpt-realtime-2.1 gpt-realtime-2.1-mini gpt-transcribe gpt-live-transcribe gpt-6-astra gpt-image-2.5-flare gpt-image-2.5-sunburst gpt-image-2.5-flare-2026-09-08 gpt-image-2.5-sunburst-2026-09-08 gpt-live-1]
Anthropic models: [claude-fable-5-1 claude-opus-5 claude-sonnet-5 claude-fable-5 claude-opus-4-8 claude-opus-4-7 claude-sonnet-4-6 claude-opus-4-6 claude-opus-4-5-20251101 claude-haiku-4-5-20251001 claude-sonnet-4-5-20250929]
Perplexity models: []
Gemini models: []
  anthropic: ok 11 models
  gemini: not_configured (unverified) No Gemini API Key
  openai: ok 132 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get OpenAI Models ===
OpenAI models: [text-embedding-ada-002 whisper-1 gpt-3.5-turbo tts-1 gpt-3.5-turbo-16k gpt-4-0613 gpt-4 davinci-002 babbage-002 gpt-3.5-turbo-instruct gpt-3.5-turbo-instruct-0914 gpt-3.5-turbo-1106 tts-1-hd tts-1-1106 tts-1-hd-1106 text-embedding-3-small text-embedding-3-large gpt-3.5-turbo-0125 gpt-4-turbo gpt-4-turbo-2024-04-09 gpt-4o gpt-4o-2024-05-13 gpt-4o-mini-2024-07-18 gpt-4o-mini gpt-4o-2024-08-06 omni-moderation-latest omni-moderation-2024-09-26 o1-2024-12-17 o1 o3-mini o3-mini-2025-01-31 gpt-4o-2024-11-20 gpt-4o-mini-search-preview-2025-03-11 gpt-4o-mini-search-preview gpt-4o-transcribe gpt-4o-mini-transcribe o1-pro-2025-03-19 o1-pro gpt-4o-mini-tts o3-2025-04-16 o4-mini-2025-04-16 o3 o4-mini gpt-4.1-2025-04-14 gpt-4.1 gpt-4.1-mini-2025-04-14 gpt-4.1-mini gpt-4.1-nano-2025-04-14 gpt-4.1-nano gpt-image-1 o4-mini-deep-research gpt-4o-transcribe-diarize o4-mini-deep-research-2025-06-26 gpt-5-chat-latest gpt-5-2025-08-07 gpt-5 gpt-5-mini-2025-08-07 gpt-5-mini gpt-5-nano-2025-08-07 gpt-5-nano gpt-audio-2025-08-28 gpt-realtime gpt-realtime-2025-08-28 gpt-audio gpt-5-codex gpt-image-1-mini gpt-5-pro-2025-10-06 gpt-5-pro gpt-audio-mini gpt-audio-mini-2025-10-06 gpt-5-search-api gpt-realtime-mini sora-2 sora-2-pro gpt-5-search-api-2025-10-14 gpt-5.1-chat-latest gpt-5.1-2025-11-13 gpt-5.1 gpt-5.1-codex gpt-5.1-codex-mini gpt-5.1-codex-max gpt-image-1.5 gpt-5.2-2025-12-11 gpt-5.2 gpt-5.2-pro-2025-12-11 gpt-5.2-pro gpt-5.2-chat-latest gpt-4o-mini-transcribe-2025-12-15 gpt-4o-mini-transcribe-2025-03-20 gpt-4o-mini-tts-2025-03-20 gpt-4o-mini-tts-2025-12-15 gpt-realtime-mini-2025-12-15 gpt-audio-mini-2025-12-15 chatgpt-image-latest gpt-5.2-codex gpt-5.3-codex gpt-realtime-1.5 gpt-audio-1.5 gpt-4o-search-preview gpt-4o-search-preview-2025-03-11 gpt-5.3-chat-latest gpt-5.4-2026-03-05 gpt-5.4-pro gpt-5.4-pro-2026-03-05 gpt-5.4 gpt-5.4-nano-2026-03-17 gpt-5.4-nano gpt-5.4-mini-2026-03-17 gpt-5.4-mini gpt-image-2 gpt-image-2-2026-04-21 gpt-5.5 gpt-5.5-2026-04-23 gpt-5.5-pro gpt-5.5-pro-2026-04-23 chat-latest gpt-realtime-translate gpt-realtime-2 gpt-realtime-whisper gpt-5.6-sol gpt-5.6-terra gpt-5.6-luna gpt-realtime-2.1 gpt-realtime-2.1-mini gpt-transcribe gpt-live-transcribe gpt-6-astra gpt-image-2.5-flare gpt-image-2.5-sunburst gpt-image-2.5-flare-2026-09-08 gpt-image-2.5-sunburst-2026-09-08 gpt-live-1]

=== Get Anthropic Models ===
Anthropic models: [claude-fable-5-1 claude-opus-5 claude-sonnet-5 claude-fable-5 claude-opus-4-8 claude-opus-4-7 claude-sonnet-4-6 claude-opus-4-6 claude-opus-4-5-20251101 claude-haiku-4-5-20251001 claude-sonnet-4-5-20250929]

✓ Chat Models API example complete
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: v1GI3TjyeOkvS8w70JCCznf05Lidxen9dmR7upuJjJ3GXVOCtDMhEIYqlT1yElBbx5QGzszCIr9meVX3px5GFA

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** A high-performance database product
- **Price:** $99

✓ Message 2 sent
  Response: The price of the product ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 8oHfwZxZ7sR1qhyCo9TUVqnbw1ivkFPnqJZxNGa1f6YzuRnXX26t1z3Im9S1_iD64YA3xiaXX04_L4-QW3qYyw
  Parent: v1GI3TjyeOkvS8w70JCCznf05Lidxen9dmR7upuJjJ3GXVOCtDMhEIYqlT1yElBbx5QGzszCIr9meVX3px5GFA

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: 8oHfwZxZ7sR1qhyCo9TUVqnbw1ivkFPnqJZxNGa1f6YzuRnXX26t1z3Im9S1_iD64YA3xiaXX04_L4-QW3qYyw (Untitled)
  Session 2: v1GI3TjyeOkvS8w70JCCznf05Lidxen9dmR7upuJjJ3GXVOCtDMhEIYqlT1yElBbx5QGzszCIr9meVX3px5GFA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 8oHfwZxZ7sR1qhyCo9TUVqnbw1ivkFPnqJZxNGa1f6YzuRnXX26t1z3Im9S1_iD64YA3xiaXX04_L4-QW3qYyw

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: WmhrsLRQB0JDP_TIkWVD6HoXZS1nvGsGc2DjQ67CN9kyJ3x8RW8EPET6rRc2UZj1eIP9wJeMBawCdCsIo2heCQ

=== List Collections ===
Total collections: 13
Sample collections: [chat_agent_configs__ek0_testing agent_function_versions__ek0_testing schedules__ek0_testing functions__ek0_testing chat_configurations__ek0_testing]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

=== Check Collection Exists (Before Creation) ===
Collection 'collection_utils_test_go' exists: false

=== Creating Test Documents ===
Created 5 test documents

=== Check Collection Exists (After Creation) ===
Collection 'collection_utils_test_go' exists: true

=== Count Documents ===
Document count in 'collection_utils_test_go': 5

=== Check Non-Existent Collection ===
Collection 'nonexistent_collection_xyz' exists: false

=== Cleanup ===
Deleted collection 'collection_utils_test_go'

✓ Collection Utilities example complete
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created
✓ conc_demo_pay_go saved
✓ conc_demo_rl_fail_go saved
✓ conc_demo_rl_skip_go saved
✓ conc_demo_lock_go saved

Invoke them like:
  POST /api/functions/conc_demo_pay_go        { "idempotency_key": "...", "amount": 100 }
  POST /api/functions/conc_demo_rl_fail_go    { "user_id": 42 }
  POST /api/functions/conc_demo_rl_skip_go    { "user_id": 42 }
  POST /api/functions/conc_demo_lock_go       { "resource": "queue:drain" }

✓ Cleaned up demo functions
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Convenience Methods Example ===

=== Native Map Creation ===
✓ Created record with native map: map[id:da9PiNm4qCV6BU450Jo2pltC6sPijFDvlxN-V-AG1W7XwMMdqbjB8jdQVsjR02yAPhnJcOflwGfeh4Jn5GKdkw]

=== Upsert Operation ===
✓ First upsert (update): map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:da9PiNm4qCV6BU450Jo2pltC6sPijFDvlxN-V-AG1W7XwMMdqbjB8jdQVsjR02yAPhnJcOflwGfeh4Jn5GKdkw name:map[type:String value:Alice Johnson]]
✓ Second upsert (insert): map[id:i6Kad4urTBRiuvV6jH_6-1ZLgyRUyikfv_O-wRSOg3LX72_yW7y4dvazLUFe_3s_NhDuKJ6S7fCYAFbiFChJfA]

=== Find One Operation ===
✓ Found user by email: map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:da9PiNm4qCV6BU450Jo2pltC6sPijFDvlxN-V-AG1W7XwMMdqbjB8jdQVsjR02yAPhnJcOflwGfeh4Jn5GKdkw name:map[type:String value:Alice Johnson]]
✓ User not found (as expected)

=== Exists Check ===
✓ Record exists: true
✓ Fake record exists: false (should be false)

=== Pagination ===
✓ Inserted 25 records for pagination
✓ Page 1: 10 records (expected 10)
✓ Page 2: 10 records (expected 10)
✓ Page 3: 7 records (expected 7)

=== Cleanup ===
✓ Deleted collection

✅ All convenience methods demonstrated successfully!
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created
✓ crypto_demo_hmac_go saved
✓ crypto_demo_aes_go saved
✓ crypto_demo_uuid_go saved
✓ crypto_demo_totp_go saved
✓ crypto_demo_encoding_go saved

Invoke them with:
  POST /api/functions/crypto_demo_hmac_go     { "payload": "hi" }
  POST /api/functions/crypto_demo_aes_go      { "plaintext": "secret" }
  POST /api/functions/crypto_demo_uuid_go
  POST /api/functions/crypto_demo_totp_go
  POST /api/functions/crypto_demo_encoding_go { "title": "Héllo World" }

✓ Cleaned up demo functions
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== Distinct Values Example ===

Inserting sample products...
Inserted 8 products

=== Distinct Categories (all products) ===
Found 3 distinct categories:
  - map[type:String value:books]
  - map[type:String value:clothing]
  - map[type:String value:electronics]

=== Distinct Statuses (all products) ===
Found 3 distinct statuses:
  - map[type:String value:active]
  - map[type:String value:archived]
  - map[type:String value:discontinued]

=== Distinct Statuses in Electronics ===
Found 2 distinct statuses for electronics:
  - map[type:String value:active]
  - map[type:String value:discontinued]

Cleanup done.
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: bsGAJ83cwsNFbEK0raV8qTr59Yvrx4tPnCPM7jDEbwd-AG2pjJP8F7VZyHeNOztNOzpTQJStOiqkE3oiqDhZZQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: dh3_9iUW8irkPt2XEz2AR9893Vf8t5GYXRIEqd7zKI06BILhLCQxWi2b9KLsUbj1lueqXUxWO-z0KEWUBN437w

=== Query Documents ===
✓ Found 2 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB as Edge Cache - Simple Example ===

Setting up edge cache collection...
✓ Cache entry created

Creating edge cache lookup script...
✓ Edge cache script created: VNlV3w48vk2MXVpaTNce--C9NLXnZ4eN64_Vpkh-pOj9RtgIRkKnV8ZzW_09CJbq7WHSHgA0h2_6rs2L-1bwAA

Call 1: Cache lookup
Response time: 2ms
Found 1 cached entries

Call 2: Cache lookup (connection warm)
Response time: 4ms
Found 1 cached entries

=== The Magic ===
- Your DATABASE is your EDGE
- No Redis needed
- No CDN needed
- No cache invalidation logic needed (TTL handles it)
- With ripples: All nodes auto-sync cache
- One service: Database + Cache + Edge Functions

✓ Example complete!

🧹 Cleaning up...
✓ Cleanup complete
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

✅ Saved reusable function: fetch_user
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"type":"String","value":"User 1"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 94.590709ms
   📊 Records: 1
   📦 Data: {
        "value": {
          "type": "Object",
          "value": {
            "address": {
              "city": "Gwenborough",
              "geo": {
                "lat": "-37.3159",
          ...

Second call (cache hit - from cache):
   ⏱️  Duration: 4.78075ms
   📊 Records: 1
   📦 Data: {
        "value": {
          "type": "Object",
          "value": {
            "address": {
              "city": "Gwenborough",
              "geo": {
                "lat": "-37.3159",
          ...
   🚀 Cache speedup: 23.5x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: get_verified_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: {"type":"String","value":"User 1"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - get_verified_user: Specific workflow


✅ All composition examples completed!
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
client_function_contract: ok
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
🚀 ekoDB Functions Example (Go Client)

✅ Client initialized

🧹 Cleaning up...
✅ Deleted collection
✅ Deleted test scripts

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: LvxDU4er-O37PM-tnPPtYkhWpGC70j09XUVeK5NuqjzxLBxEwKGWuBQt4Db-JkT8CHQ5fxvuE9BlClpLUxrkDw
📊 Found 5 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

✅ Function saved
📊 Found 3 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 2 groups
   map[avg_score:map[type:Float value:60] count:map[type:Integer value:5] status:map[type:String value:active]]
   map[avg_score:map[type:Float value:50] count:map[type:Integer value:5] status:map[type:String value:inactive]]
⏱️  Execution time: 0ms

📝 Example 4: Function Management

📋 Total functions: 3
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

📝 Example 5: Multi-Stage Pipeline

✅ Multi-stage script saved
📊 Pipeline executed 2 stages
⏱️  Total execution time: 0ms
📈 Stage breakdown:

📝 Example 6: Count Users

✅ Count script saved
📊 Total user count: 10
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Deleted collection
✅ Deleted test scripts

✅ All examples completed successfully!

💡 Key Advantages of Using the Client:
   • Automatic token management
   • Type-safe Stage builders
   • Built-in error handling
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
🚀 ekoDB Go Advanced Functions Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Function saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Function saved
📊 Category breakdown:
   map[avg_price:map[type:Float value:365.6666666666667] category:map[type:String value:Furniture] count:map[type:Integer value:3]]
   map[avg_price:map[type:Float value:367] category:map[type:String value:Electronics] count:map[type:Integer value:5]]
⏱️  Execution time: 0ms

✅ All advanced script examples finished!
🧹 Cleaning up...
✅ Cleanup complete
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
🚀 ekoDB Go AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
📊 AI Response generated
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embed script saved
📊 Embedding generated
⏱️  Execution time: 0ms

✅ All AI script examples finished!

💡 This example demonstrates:
   ✅ Chat completions with system/user messages
   ✅ Embedding generation for text
🧹 Cleaning up...
✅ Cleanup complete
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
🚀 ekoDB Go Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: Tl9QminPFEn8gTBUxcPPNWxnJRENWuemGwd4hCdpxzX3-6kpNuvEq-yEasf30sb90_8ot_xuxrl-iFyX3iEe6A
📊 Found 2 category stats

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories

📝 Example 4: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Function saved
📊 Pipeline executed 3 stages


✅ All complete script examples finished!

💡 This example demonstrates ekoDB's function system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Function management (save, call, delete)
🧹 Cleaning up...
✅ Cleanup complete
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
🚀 ekoDB Go CRUD Functions Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Function saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Function saved
📊 User counts by status:
   map[count:map[type:Integer value:3] status:map[type:String value:inactive]]
   map[count:map[type:Integer value:7] status:map[type:String value:active]]
⏱️  Execution time: 0ms

✅ All CRUD script examples finished!
🧹 Cleaning up...
✅ Cleanup complete
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
🚀 ekoDB Go KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: IMn-iJ5WNMOtRnnKJO12_iNpTXbJN6k5EyrcSFlvgroJvOm1EXiDt1bhJNYTrzXnid5LOhZIIB5u302htDj-Ig
✅ Inserted 2 products with wrapped types

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: Awew1BkW0kSPEz8gSA2pJW8HpAXhNUv6WwtQ21mM7GLIMgX06nDCOztL6qTyOyYDa-h5GgNFfdoy_TeSFC8Vgw
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: map[type:Object value:map[role:admin userId:user_abc]]
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session
📝 Example 4: KV Operations in Functions

✅ Function saved: Cr1sE_Lmw6_bwzFlNKKaiVxqzTJAub9Y68VkWvfeVwjXSsP7NqFvoDiRCTyF_7df1Z1WFR8pD22qk8u76ZIPWA
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: ECcbcqW3FBKzBVJ2GDUvhDn00XiotiQh29IikRqxasen4LrRJqP2IQfNVf-gwj_DSWuhQ2k9PNlcjKd6zaE22w
📊 Processed order with caching
⏱️  Stages executed: 3
⏱️  Execution time: 0ms

✅ All KV & Wrapped Types examples completed!

💡 Key takeaways:
   ✅ Use Field* helpers for type-safe wrapped values
   ✅ FieldDecimal() preserves precision (no floating point errors)
   ✅ KV store is great for caching and quick lookups
   ✅ StageKv*() functions work within scripts
🧹 Cleaning up...
✅ Cleanup complete

perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
🚀 ekoDB Go Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. map[type:String value:Introduction to Machine Learning] (map[type:String value:AI])
   2. map[type:String value:Getting Started with ekoDB] (map[type:String value:Database])
   3. map[type:String value:Natural Language Processing] (map[type:String value:AI])
   4. map[type:String value:Vector Databases Explained] (map[type:String value:Database])
   5. map[type:String value:Database Design Principles] (map[type:String value:Database])
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   map[category:map[type:String value:Database] count:map[type:Integer value:3]]
   map[category:map[type:String value:AI] count:map[type:Integer value:2]]
⏱️  Execution time: 0ms

✅ All search script examples finished!
🧹 Cleaning up...
✅ Cleanup complete
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Goal Template CRUD Example (Go) ===

--- Creating goal template ---
Created template: Data Migration (id: O-cvT9FX3IFdZW3GSCwdF8H61LnF4CbQFXocgRIpghNqH3CJuBIYYiT7G81XQ68rf8tXDPcoI9NjnbCNsP7oZA)

--- Listing templates ---
Templates: map[count:1 items:[map[description:map[type:String value:Template for migrating data between schemas] id:O-cvT9FX3IFdZW3GSCwdF8H61LnF4CbQFXocgRIpghNqH3CJuBIYYiT7G81XQ68rf8tXDPcoI9NjnbCNsP7oZA steps:map[type:Array value:[map[description:Analyze source schema] map[description:Create target schema] map[description:Migrate records] map[description:Validate results]]] title:map[type:String value:Data Migration]]]]

--- Getting template ---
Fetched: Data Migration

--- Updating template ---
Updated description: Updated: comprehensive data migration workflow

--- Deleting template ---
Template deleted successfully

✓ Goal template CRUD example completed
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Goals, Tasks & Agents Integration Example (Go) ===

--- Creating goal ---
Created goal: Deploy v2.0 (id: 37fO5DgzYGGoGG434f6_ux4MjyedtMdwZ2Yi8wxDMtKKjGe5mhkg70hJ13q5NqLFNNlHUL-T9jbitwPHn55W2g)

--- Listing goals ---
Goals response: map[count:1 goals:[map[created_at:2026-09-16T13:48:05.712236+00:00 description:Ship the v2.0 release to production id:37fO5DgzYGGoGG434f6_ux4MjyedtMdwZ2Yi8wxDMtKKjGe5mhkg70hJ13q5NqLFNNlHUL-T9jbitwPHn55W2g status:pending steps:[{"description":"Run integration tests"},{"description":"Build release artifacts"},{"description":"Deploy to staging"}] title:Deploy v2.0 updated_at:2026-09-16T13:48:05.712236+00:00]]]

--- Getting goal ---
Fetched goal: Deploy v2.0

--- Updating goal ---
Updated description: Ship the v2.0 release to production (with hotfix)

--- Searching goals ---
Search result: map[count:1 items:[map[_score:12.870000000000001 created_at:map[type:DateTime value:2026-09-16T13:48:05.712236+00:00] description:map[type:String value:Ship the v2.0 release to production (with hotfix)] id:37fO5DgzYGGoGG434f6_ux4MjyedtMdwZ2Yi8wxDMtKKjGe5mhkg70hJ13q5NqLFNNlHUL-T9jbitwPHn55W2g status:map[type:String value:pending] steps:map[type:String value:[{"description":"Run integration tests"},{"description":"Build release artifacts"},{"description":"Deploy to staging"}]] title:map[type:String value:Deploy v2.0] updated_at:map[type:DateTime value:2026-09-16T13:48:05.730398+00:00]]]]

--- Goal step lifecycle ---
Step 0 started
Step 0 completed
Step 1 started
Step 1 failed

--- Completing and approving goal ---
Goal status after complete: pending_review
Goal status after approve: in_progress

--- Testing GoalReject ---
Goal status after reject: failed

--- Creating task ---
Created task: Run benchmarks (id: V8H3q9l2f4LpzQdSsAMEyXDI8KqMMPXYW6C5XXZVs8GTsKNfzl-GSwr0X_JGeCYGTUp53owlr52By0IuU8iVBw)

--- Listing tasks ---
Tasks: map[count:1 items:[map[description:map[type:String value:Execute YCSB benchmarks against staging] due_at:map[type:DateTime value:2026-09-17T13:48:05+00:00] id:V8H3q9l2f4LpzQdSsAMEyXDI8KqMMPXYW6C5XXZVs8GTsKNfzl-GSwr0X_JGeCYGTUp53owlr52By0IuU8iVBw name:map[type:String value:Run benchmarks]]]]

--- Getting task ---
Fetched task: Run benchmarks

--- Task lifecycle: start -> succeed ---
Task status: running
Task status: active

--- Task lifecycle: pause -> resume -> fail ---
Task status after pause: paused
Task status after resume: active
Task failed successfully

--- Due tasks ---
Due tasks: map[count:0 items:[]]

--- Deleting tasks ---
Tasks deleted

--- Creating agent ---
Created agent: benchmark-runner (id: Tzn_HTMPQ8o3A3P7ITfgC3qefh3qD46AgbxGSIAatWdsm9BH3Z0Chcrga5KrTwqOKm5PEBAQFW2zFyYpNXyKbA)

--- Listing agents ---
Agents: map[count:1 items:[map[deployment_id:map[type:String value:deploy_prod_1] description:map[type:String value:Runs periodic benchmarks] id:Tzn_HTMPQ8o3A3P7ITfgC3qefh3qD46AgbxGSIAatWdsm9BH3Z0Chcrga5KrTwqOKm5PEBAQFW2zFyYpNXyKbA llm_model:map[type:String value:gpt-4.1] name:map[type:String value:benchmark-runner]]]]

--- Getting agent by ID ---
Fetched agent: benchmark-runner

--- Getting agent by name ---
Agent by name: benchmark-runner

--- Updating agent ---
Updated agent description: Runs hourly YCSB benchmarks

--- Agents by deployment ---
Agents in deploy_prod_1: map[count:0 items:[]]

--- Deleting agent ---
Agent deleted

--- Deleting goal ---
Goal deleted

=== All goals, tasks & agents operations completed successfully ===
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== Join Operations Examples ===

Setting up sample data...
✅ Sample data created

1. Single collection join (users with departments):
Found 2 users with department data
  - Alice Johnson: Engineering
  - Bob Smith: Sales

2. Join with filtering:
Found 1 users in Engineering
  - Alice Johnson: Building A

3. Join with user profiles:
Found 2 users with profile data
  - Alice Johnson: Senior Software Engineer
  - Bob Smith: Sales Manager

4. Join orders with user data:
Found 2 completed orders
  - Laptop ($1200) by Alice Johnson
  - Mouse ($25) by Alice Johnson

5. Complex join with multiple conditions:
Found 2 users with example.com emails
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✅ Deleted test collections

✅ Join operations examples completed!
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created
✓ go_users_register saved
✓ go_users_login saved
✓ go_users_verify_token saved

=== Auth flow defined as pure stored functions ===
Call them like:
  POST /api/functions/go_users_register { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/go_users_login    { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/go_users_verify_token { "token": "<jwt>" }

✓ Cleaned up demo functions
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB KV Links Integration Example (Go) ===

--- Inserting documents ---
Inserted doc1: UzoRgHjEkSjYEH-AI3v7jCBTsJj4jKpP5HJxEAnWmYPHk19uO5K9dTlp9LCKyPnJbLLPA6CU7fR4s4mXwr6nGg
Inserted doc2: HW0V0XdbI_Do72FPsLifFpLDl_aq1wUdLvHF0ed8-_pzq9uiU7V-auzA7GEH7RzTXm3Tk3ULfRAhUnUjItLkcQ

--- Setting KV entry ---
KV entry set: team:engineering:go

--- Linking KV key to documents ---
Linked doc1: map[]
Linked doc2: map[]

--- Getting links ---
Links for team:engineering:go: [map[collection:kv_links_example_go created_at:2026-09-16T13:48:07.859124Z document_id:HW0V0XdbI_Do72FPsLifFpLDl_aq1wUdLvHF0ed8-_pzq9uiU7V-auzA7GEH7RzTXm3Tk3ULfRAhUnUjItLkcQ field_path:<nil> last_accessed:2026-09-16T13:48:07.860351Z metadata:map[]] map[collection:kv_links_example_go created_at:2026-09-16T13:48:07.857642Z document_id:UzoRgHjEkSjYEH-AI3v7jCBTsJj4jKpP5HJxEAnWmYPHk19uO5K9dTlp9LCKyPnJbLLPA6CU7fR4s4mXwr6nGg field_path:<nil> last_accessed:2026-09-16T13:48:07.860351Z metadata:map[]]]

--- Unlinking doc2 ---
Unlink result: map[]

--- Verifying links after unlink ---
Links after unlink: [map[collection:kv_links_example_go created_at:2026-09-16T13:48:07.857642Z document_id:UzoRgHjEkSjYEH-AI3v7jCBTsJj4jKpP5HJxEAnWmYPHk19uO5K9dTlp9LCKyPnJbLLPA6CU7fR4s4mXwr6nGg field_path:<nil> last_accessed:2026-09-16T13:48:07.863334Z metadata:map[]]]

--- Cleanup ---
Cleaned up all resources

=== All KV link operations completed successfully ===
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

=== KV Set ===
✓ Set key: session:user123:go

=== KV Get ===
Retrieved value: map[type:Object value:map[userId:123 username:john_doe]]

=== KV Batch Set ===
✓ Batch set 3 keys
  cache:product:1:go: success
  cache:product:2:go: success
  cache:product:3:go: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  cache:product:1:go: map[name:Product 1 price:29.99]
  cache:product:2:go: map[name:Product 2 price:39.99]
  cache:product:3:go: map[name:Product 3 price:49.99]

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*:go'

=== KV Query (Alias for Find) ===
Total keys in store: 4

=== KV Delete ===
✓ Deleted key: session:user123:go
✓ Verified: Key exists after delete: false

=== KV Batch Delete ===
✓ Batch deleted 3 keys
  cache:product:1:go: deleted
  cache:product:2:go: deleted
  cache:product:3:go: deleted

✓ All KV operations completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== KV Precision: Float vs Decimal ===

=== Test 1: Using Go Floats (LOSES PRECISION) ===
✓ Stored products with float prices

Retrieved float prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 2: Using FieldDecimal() (PRESERVES PRECISION) ===
✓ Stored products with decimal prices

Retrieved decimal prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 3: Sum Calculation Comparison ===
  Float sum: $119.97 (expected $119.97)
  Decimal sum: $119.97 (expected $119.97)

=== Test 4: Extreme Precision Example ===
  Float 0.1 + 0.2 = 0.29999999999999999 (should be 0.3)
  Decimal "0.30" = 0.30 (exact!)

=== Summary ===
✅ Use FieldDecimal() for monetary values, percentages, and
   any case where floating-point errors are unacceptable.
✅ FieldDecimal() stores values as strings internally,
   preserving exact precision across all operations.

=== Cleanup ===
✓ Cleaned up test keys
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created
✓ go_route_admin saved
✓ go_route_user_by_id saved
✓ go_route_user_posts saved
✓ go_route_org_create_member saved

Try them with curl:
  curl http://localhost:8080/api/route/users/admin
  curl http://localhost:8080/api/route/users/42
  curl http://localhost:8080/api/route/users/42/posts/7
  curl -X POST http://localhost:8080/api/route/orgs/acme/members \
       -H 'Content-Type: application/json' -d '{"name":"alice"}'

✓ Cleaned up demo functions
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== Query Builder Examples ===

Setting up test data...
✅ Test data created

1. Simple equality query:
Found 2 active users

2. Range query with sorting:
Found 3 users aged 18-65

3. String operations:
Found 2 users with @example.com emails

4. IN operator:
Found 2 privileged users

5. Complex query with multiple conditions:
Found 1 active US users over 21

6. Pagination:
Page 1: 2 users

7. NOT IN operator:
Found 3 valid users

8. Using bypass flags:
Found 2 users (bypassed cache)

=== Cleanup ===
✅ Deleted test collection

✅ Query Builder examples completed!
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Raw Completion Stream (SSE) Example ===

--- Basic SSE Raw Completion ---
Response: The three primary colors are red, blue, and yellow.

--- Structured Output via SSE ---
JSON response: [
  {
    "name": "Earth",
    "diameter_km": 12742
  },
  {
    "name": "Mars",
    "diameter_km": 6779
  },
  {
    "name": "Jupiter",
    "diameter_km": 139820
  }
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello! I hope you're having a wonderful day.

=== Done ===
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Schedules Integration Example (Go) ===

--- Creating schedule ---
Created schedule: nightly-backup (id: e9fb0210-7e2d-4f82-85bc-b65ea61d94ac)

--- Listing schedules ---
Schedules: map[count:1 schedules:[map[created_at:2026-09-16T13:48:14.297211Z cron_expression:0 0 0 * * * description:Runs a database backup every night at midnight enabled:true function_label:schedule_noop_go_17740 id:e9fb0210-7e2d-4f82-85bc-b65ea61d94ac last_execution:<nil> name:nightly-backup next_execution:2026-09-17T00:00:00Z parameters:map[] stats:map[avg_execution_time_ms:0 failed_executions:0 last_error:<nil> successful_executions:0 total_executions:0] timezone:UTC updated_at:2026-09-16T13:48:14.297211Z]]]

--- Getting schedule ---
Fetched schedule: nightly-backup (cron: 0 0 0 * * *)

--- Updating schedule ---
Updated cron: 0 0 2 * * *

--- Triggering schedule ---
Trigger response: map[schedule_id:e9fb0210-7e2d-4f82-85bc-b65ea61d94ac status:triggered]

--- Pausing schedule ---
Schedule enabled: false

--- Resuming schedule ---
Schedule enabled: true

--- Deleting schedule ---
Schedule deleted

=== All schedule operations completed successfully ===
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== Schema Management Examples ===

1. Creating user schema with basic fields:
✅ User schema created

2. Creating product schema with text index:
✅ Product schema with indexes created

3. Creating document schema with vector index:
✅ Document schema with vector index created

4. Retrieving collection schema:
Schema fields: 4 fields
Schema version: 1

5. Retrieving collection metadata:
Collection has 4 fields

6. Creating employee schema with all constraint types:
✅ Employee schema with all constraints created

✅ Schema management examples completed!
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== Search Examples ===

Setting up test data...
✅ Test data created

1. Basic full-text search:
Found 2 results
  1. Score: 12.870
  2. Score: 6.270

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 13.200
  2. Score: 13.200
  3. Score: 13.200
  4. Score: 13.200

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400
  2. Score: 26.400
  3. Score: 26.400
  4. Score: 26.400

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 6.600
  2. Score: 6.600

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 6.600

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.763
  2. Score: 0.754
  3. Score: 0.725

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.490
  2. Score: 0.901
  3. Score: 0.305

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200

9. Vector search with a metadata pre-filter (category = ml):
Found 2 documents in category "ml" (NLP excluded)
  1. Deep Learning Fundamentals (category: ml)
  2. Introduction to Machine Learning (category: ml)


✅ Search examples completed!
=== Cleanup ===
✅ Deleted test collections
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: map[id:3Cnc85FwZxs4bHBvdTJfrphlFFZRS8WrgVqyepp9ojwZktlSruuQ_8QhtYp2l3vUrq8GaedZUVUmrx2-Hk2rPA]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-09-16T13:48:16+00:00] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:3Cnc85FwZxs4bHBvdTJfrphlFFZRS8WrgVqyepp9ojwZktlSruuQ_8QhtYp2l3vUrq8GaedZUVUmrx2-Hk2rPA metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Test Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:42]]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-09-16 13:48:16 +0000 +0000
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-09-16T13:48:16+00:00 data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:3Cnc85FwZxs4bHBvdTJfrphlFFZRS8WrgVqyepp9ojwZktlSruuQ_8QhtYp2l3vUrq8GaedZUVUmrx2-Hk2rPA metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-09-16T13:48:16+00:00] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:3Cnc85FwZxs4bHBvdTJfrphlFFZRS8WrgVqyepp9ojwZktlSruuQ_8QhtYp2l3vUrq8GaedZUVUmrx2-Hk2rPA metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: 8sFz8JMhr2jjeXGkVLiFRH7uCgPwO-NxyH3dUJ08H1H1yDuRfpLLev4d2GrcVGyZ8oXvOIfY5T0KNyn9lwSojg

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
🚀 ekoDB Go Client - Native SWR Function Examples

📋 Demonstrates:
   • Single-function SWR pattern (replaces 4-step pipeline)
   • Automatic cache checking, HTTP fetching, and cache setting
   • Built-in audit trail support
   • Duration string TTLs ('15m', '1h', '30s')
   • Multi-function pipeline integration
   • Dynamic TTL configuration


Example 1: Basic Native SWR
────────────────────────────────────────────────────────────────────────────────
Single function replaces KvGet → If → HttpRequest → KvSet pipeline
✓ Created native SWR script: github_user_native_go (ZGSeOq7CasfCxG--WXoIiciwkFE6Nl_h2Z5HMCrg3THqQB-OnzXvhsZya3D-cSvOxbdLQzk4cH4yGqa2ygXkrA)

First call (cache miss - will fetch from GitHub API):
  Response time: 152ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 6ms
  Speedup: 25.3x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_go (ZDujMSaPljFsHYD85eMCypuZQClT78VC8Kk1dG5xb9Siyp4vBUCm_SAuZ-CS37oiR7DGWnGefHJ-WwOlqU7NcA)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_go' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_go (e33qmG9teCuzqQu96ooISHxc9BCDvucFbqA__IfddCT9ij5eevy1UyQ_kM_qWd6cLZjFwQXxj1DicC49sNYX-A)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_go' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_go (6mNRgEGoBDullLfbQNfnAt8FMsqh7Fyxrv99Lb60528nMAosj-OK7tL6p1DDBjVDJ_CcE5x0h3XKpFbLj2UJug)
  ✓ Cached with TTL: 5m (5 minutes)
  ✓ Cached with TTL: 1h (1 hour)
  ✓ Cached with TTL: 30s (30 seconds)

================================================================================
✅ Key Benefits of Native SWR:
✅ Single function: Replaces 4-function cache-aside pattern
✅ Duration strings: Use '15m', '1h', '2h' instead of calculating seconds
✅ Built-in audit: Optional collection parameter for automatic logging
✅ Auto-enrichment: output_field populates params for downstream functions
✅ Transactional: Works correctly in both transactional and non-transactional contexts
✅ KV-optimized: Uses native KV store with proper TTL handling

=== Performance Comparison ===
Legacy Pattern: KvGet → If → HttpRequest → KvSet → Insert (5 functions)
Native SWR:     SWR → Insert (2 functions)
Result:         60% fewer functions, cleaner code, same behavior 🎯

🧹 Cleaning up...
✓ Deleted 4 test scripts

✅ All examples completed!
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_go (4V02qL5UJCRSPg8Qy2UwmUTWPusKjEGPlZIKlvIgKeD9yORLkHMGWc9yBHmePIo6cjY72xkFjVEJROK_gk4uqw)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "cached_at": {
        "type": "DateTime",
        "value": "2026-09-16T13:48:18+00:00"
      },
      "data": {
        "type": "Object",
        "value": {
          "address": {
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "street": "Kulas Light",
            "suite": "Apt. 556",
            "zipcode": "92998-3874"
          },
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "email": "Sincere@april.biz",
          "id": 1,
          "name": "Leanne Graham",
          "phone": "1-770-736-8031 x56442",
          "username": "Bret",
          "website": "hildegard.org"
        }
      },
      "id": "1"
    }
  ],
  "stats": {
    "input_count": 0,
    "output_count": 1,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 11ms (served from cache)
✓ Lightning fast cache hit

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
🧹 Cleaning up...
✓ Cleanup complete

perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 6mggmyV-oZ-BVccvy6WU3S30W2JHrc4he5ZV-Evjnc2yY4k-Bm4q1WwucmGbO9BNZHX5odCTU3RqOmWZ2wByBw
Created Bob: $500 - ID: nlucW5_NZ5J09HtTVjveeiSkKnassnl9Jlz52M078b1ceGCbV6Dd5eBvIwqwlP95e_8G7rI7p2W1CVD781qfOw

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): e1b12090-6804-4e5b-8374-f41d68e6c6f6

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

✓ Verified committed balances: Alice=$800, Bob=$700

=== Example 5: Rollback Demo ===
New transaction: 37eb9f64-99d2-4ae1-af22-969f35fd10b3
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

=== Create User Function ===
Created user function with ID: c_34Wf2QFxDhdj7Z4iQ722YTn9Rvj1unU_HT-aP1eNsT2P44uU3OCW0IDJsOkGlZn5TAJvYtdzIZ_G6d0-FH3A

=== Get User Function ===
Retrieved: get_active_users_go - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 1 user functions:
  - get_active_users_go: Get Active Users

=== List User Functions by Tag ===
Found 1 user functions with 'users' tag:
  - get_active_users_go

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== WebSocket Chat Streaming Example (Go) ===

Created chat session: lvcuTU5UBxu38FIoCpOmdVRDzP403ZdoDPeZ9laHWCBLPVLm5Wl_saEIcVAjlmojXLo_DvIPZ2_CWdNFwePgPg

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: wX_-3uEe4PfROazYY4PrYTUDo0vjtiCeJI2jtw7Ik99tI9BskmWMXDYws5MlDV0j3i2FHCI604WHjx-g19t1IQ
Execution time: 903ms
Token usage: {"completion_tokens":8,"prompt_tokens":15,"total_tokens":23}

Full response: {"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}The capital of France is Paris....
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== WebSocket Subscription Example (Go) ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_go' ===
✓ Subscribed (subscription_id: sub_b2472f68d7c0421cb874561766c088e2)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: 4XZq4jfDFJzhnR7y1bWXihSUqWARu_dYI6CHrvhK8yTSSPd8USCgJKTbmTiYVEou_d7juEGIthO7g-Iv_A8qbA

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_go
     Record IDs: [4XZq4jfDFJzhnR7y1bWXihSUqWARu_dYI6CHrvhK8yTSSPd8USCgJKTbmTiYVEou_d7juEGIthO7g-Iv_A8qbA]
     Timestamp:  2026-09-16T13:48:21.959103+00:00

Inserting record 2...
✓ Inserted: _SmKrZVPjDoULNEBvafzR0avAVJXfco0sVp2O7HPoXa73XTJqN6q6G_HLWsbicTV0iffvY6JZ-ZNG5bnlUPJvQ

  📡 Notification received:
     Event:      insert
     Record IDs: [_SmKrZVPjDoULNEBvafzR0avAVJXfco0sVp2O7HPoXa73XTJqN6q6G_HLWsbicTV0iffvY6JZ-ZNG5bnlUPJvQ]

=== Unsubscribing ===
✓ Unsubscribed

✓ WebSocket subscription example completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Rif2Nxfnh5ffyKF294tYL6IIbYqR8MV7NFQjK45XU_vb65QlBGlJYgaHzYQCNQS-hDBpcEVmDJV8y0gxZmJ8Eg

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
Inserted with ripple: map[id:hWncVRBtB2er72vYulF0q1E1gQdxzvLZ89_iGE_lV_Nq45Q_-SBne78KuoeWwP-yvcCkjOEBAiegpyI85FFC3A]
Inserted with bypass_ripple: map[id:M9u4w1dfFKlouqlC2WnR9Nh1FEFTsZ6H-WKV0gxX2YqlcvJbdzzynRaJ-cR6gwGwJtfxI3j5kWFSKplJXy-lXg]
Inserted with TTL and bypass_ripple: map[id:CEz6TDnfxA1z6S6rjFPwquA3nP3a2vk8kwAG2r8hIpB9CdWYU-jxjxGUEhKf1i7_6u9tt6GzfI8TpcQOQ6j7xg]
Updated with bypass_ripple: map[id:hWncVRBtB2er72vYulF0q1E1gQdxzvLZ89_iGE_lV_Nq45Q_-SBne78KuoeWwP-yvcCkjOEBAiegpyI85FFC3A name:map[type:String value:Product 1] price:map[type:Integer value:150]]
Deleted with bypass_ripple
Batch inserted with bypass_ripple: 2 records
Upserted with bypass_ripple: map[id:yjSK26YSF_uDAHtIOfF-ibzm2Zxz0cutJQt3UtQvkoZw-hgOfhGHHx58yKlkLCTDyUkjqBsAxeW4q8m_Og9u0A]
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: [email id name]
  First user: Alice Johnson <alice@example.com>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: [age avatar_url bio created_at email id name status user_role]

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - Dave Brown (age 45)
    - Alice Johnson (age 30)
    - Bob Smith (age 25)

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - Carol White: Manager

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: [age api_key avatar_url bio created_at email id name password secret_token status user_role]
  Projected query:
    - 3 fields per record
    - Fields: [email id name]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
