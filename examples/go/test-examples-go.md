make test-examples-go
🧪 [36mRunning Go examples (direct HTTP/WebSocket)...[0m
[34m
╔════════════════════════════════════════╗[0m
[34m║     ekoDB Go Examples Test Suite      ║[0m
[34m╚════════════════════════════════════════╝[0m
[34m
=== Checking Server Connection ===[0m
[32m✓ Server is ready[0m
[33m
=== Running 10 Examples ===[0m
[34m
=== Running simple_crud.go ===[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: map[id:Nfl1ZEDeFIJK58BM2Au-tjVCR0DjIJlbHrYx8UiR0kJwJrW-ARX4rJ3TU-80nadW0r7LJafNfnLMnL5SIJmNww]

=== Find by ID ===
Found: map[active:true id:Nfl1ZEDeFIJK58BM2Au-tjVCR0DjIJlbHrYx8UiR0kJwJrW-ARX4rJ3TU-80nadW0r7LJafNfnLMnL5SIJmNww name:Test Record value:42]

=== Find with Query ===
Found documents: map[]

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:Nfl1ZEDeFIJK58BM2Au-tjVCR0DjIJlbHrYx8UiR0kJwJrW-ARX4rJ3TU-80nadW0r7LJafNfnLMnL5SIJmNww name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: i5vbX_z97nJ1E56GlHzp2wC_FmlBv1Wm-Ipn3OO4Ra22X_MSsZWRGj03UKV_xtN8YVAD3vqj8KPRCFrFF9WyLA

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "i5vbX_z97nJ1E56GlHzp2wC_FmlBv1Wm-Ipn3OO4Ra22X_MSsZWRGj03UKV_xtN8YVAD3vqj8KPRCFrFF9WyLA",
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "value": {
          "type": "Integer",
          "value": 42
        }
      },
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "SNyEc7R-CZkxYBn4G05Ra_eW5cYKuJXomLyBtWiFJVcI8VEUsZ4LRygII3U0d05JOVnjULyCL6HURn3kXafJ2Q",
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "value": {
          "type": "Integer",
          "value": 42
        }
      },
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "UTmrXQDM_xk24AvjXafv75Ex54tPoRRcsv09e6t_5bW_U4BC3Vkjl5mrH9x0OXidPmHqBPVf2zcWdZvSCusrYw",
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
✓ Retrieved 3 record(s) via WebSocket

✓ WebSocket example completed successfully
[32m✓ simple_websocket.go completed successfully[0m
[34m
=== Running http_functions.go ===[0m
🚀 ekoDB Scripts Example (Go/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: YWBGRKFkEYFWiVTRoaEgpEQH6jstnwFaV72XgcQ_3EtTc0Ww5gcvTebFIW-SOQ8x-sgYCfctYVbKJgz20AOA4w
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: QmDh8jzllhYUWoVF_Nd5rGCe9nUdLUXMBSOyzaHCYD5yhKH-qXR5sxicnosbUghwGQnA3JuqE5AtIO1GpFmqxw
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: ubImi78dvQ1JHF5Uun1AyaJaJqEmiEE74W_cFKbx6f7ArtttR8W5TtGN2Eppc36MiCXJcLvI7bDtDziZpnHqUw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":60,"count":10,"max_score":100,"status":"active"}
   {"avg_score":50,"count":10,"max_score":90,"status":"inactive"}

📝 Example 4: Function Management

📋 Total scripts: 27
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
[32m✓ http_functions.go completed successfully[0m
[34m
=== Running batch_operations.go ===[0m
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
[32m✓ batch_operations.go completed successfully[0m
[34m
=== Running kv_operations.go ===[0m
=== Key-Value Operations (Direct HTTP) ===

✓ Authentication successful

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: map[userId:123 username:john_doe]

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: map[name:Product 1 price:29.99]
cache:product:2: map[name:Product 2 price:39.989999999999995]
cache:product:3: map[name:Product 3 price:49.989999999999995]

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
[32m✓ kv_operations.go completed successfully[0m
[34m
=== Running collection_management.go ===[0m
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: GjLXUlwtXG4km7E6knPdzKiNjKqGv2H_hdSeNKJjEmLLEX568Z2GhHhSyo4a87Y9cROi_IJafHjoZAulhWjm5w

=== List Collections ===
Total collections: 15
Sample collections: [schema_employees_client_py scripts__ek0_testing ws_ttl_test test_accounts schema_products_client_py]

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
[32m✓ collection_management.go completed successfully[0m
[34m
=== Running transactions.go ===[0m
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: QVDAIycoOK_2RZMyy-g4kv3YAGCPPytFjVAPguPTF0UwI99Jaaj4OvsS2VMWr39kpDHJDhLATtF7xwiZ9oLw-w
Created Bob: $500 - ID: UwQRMNn3Df8HqKTUH8MB6mm1sV-tuFLVsZWTdeVAM8aN1sbyEEKJYYOCq78JWtCnC_SGZqov_Kbab2QOVtFFsQ

=== Example 1: Begin Transaction ===
Transaction ID: 58ee918b-0e59-48ee-8ba5-110395a36ce0

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: 800
Bob: 700

=== Example 5: Rollback ===
New transaction: 7e012dd7-a173-46ec-af3c-a5a09dd4494d
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: 700

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
[32m✓ transactions.go completed successfully[0m
[34m
=== Running crud_scripts.go ===[0m
🚀 ekoDB Complete CRUD Scripts Example
============================================================
Demonstrates:
  • Insert + Verify (using Query)
  • Query + Update Status + Verify
  • Query + Update Credits + Verify
  • Query Before Delete + Delete + Verify Gone

Each Script shows Functions chaining with proper verification
============================================================

============================================================
📝 Script 1: Insert + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: 1yunHy568zshE3ds4AW4C7IxCcSol4qpJfF5iAfCpwKMQRJAgJvxymxnbhvuBLlIcwyaEM5MvVLBP6onXBZvFQ

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: ikcVM0U_S2v6lMsK1rWP_uI1pEMtF_MhyKjeuj1L0DqoihF5sPbDiPewmJJYUSKMVVEcKTjfvUVAtQyEhVvyiw
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: 8m_j3LCtj9MIlWbk5Kj0KB9EAIhAr5gmA0IkVC9ZLPw1xzfQaEB_xIL-jjaUVwp_H6oY4nnmUvFKkCWtEyTh6g

2️⃣ Calling Script (Query + Update + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: map[type:String value:active]
   📋 Name: map[type:String value:Alice Smith]

============================================================
📝 Script 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: eCzvyAvcjU4Bl8cLTvNiBEno12MH1QDpetRp2Goa3R9VKs6cFA60RiQPYWO1bfgmlG73OlSgvrVoQpT9wez2qQ

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: map[type:Integer value:0]
   📋 Status: map[type:String value:active]
   📋 Name: map[type:String value:Alice Smith]

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: ckBAiaJKeUmZm3EcYGB_N-hDlHl1Hdtkszs_VlMXpniiunJX95nOnCOE_6sfpJlSgxhXhgzrjFpUBauoc2A4Vg

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: 1yunHy568zshE3ds4AW4...
   ✅ Deleted script: 8m_j3LCtj9MIlWbk5Kj0...
   ✅ Deleted script: eCzvyAvcjU4Bl8cLTvNi...
   ✅ Deleted script: ckBAiaJKeUmZm3EcYGB_...
   ✅ Deleted collection: users

============================================================
✅ Complete CRUD Scripts Example Finished!
============================================================

💡 Key Takeaways:
   ✅ Scripts chain Functions together
   ✅ Each Script demonstrates operation + verification
   ✅ Parameters make Scripts reusable
   ✅ Verification is built into the Script itself
   ✅ Complete CRUD lifecycle in 4 focused Scripts
[32m✓ crud_scripts.go completed successfully[0m
[34m
=== Running document_ttl.go ===[0m
╔════════════════════════════════════════════════════════╗
║     TTL EXPIRATION VERIFICATION TEST                   ║
╚════════════════════════════════════════════════════════╝

This test verifies that document TTL expiration works correctly.
We will insert documents with short TTL and verify they expire.

2026/01/26 19:55:59 No .env file found, using defaults
✓ Client connected

═══════════════════════════════════════════════════════════
TEST 1: Document TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = NWlstgYdbu-U7YYN61dZ3jMcK3-HElMfHb3ASLDBuJlhogNj4mi3q5hCXJxqSDHYb8h6L2fPDPqDY2NTZkbMmw
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: FindByID(NWlstgYdbu-U7YYN61dZ3jMcK3-HElMfHb3ASLDBuJlhogNj4mi3q5hCXJxqSDHYb8h6L2fPDPqDY2NTZkbMmw)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: FindByID(NWlstgYdbu-U7YYN61dZ3jMcK3-HElMfHb3ASLDBuJlhogNj4mi3q5hCXJxqSDHYb8h6L2fPDPqDY2NTZkbMmw)
  Output: Error (expected) - request failed with status 404: ��Record not found (expired)
  ✓ PASS: Document expired (not found error)

═══════════════════════════════════════════════════════════
CLEANUP
═══════════════════════════════════════════════════════════
✓ Deleted test collection

╔════════════════════════════════════════════════════════╗
║              ALL TTL TESTS PASSED ✓                    ║
╚════════════════════════════════════════════════════════╝

TTL expiration is working correctly:
  • Documents with TTL expire after the specified time
  • Documents without TTL persist indefinitely
  • Different TTL durations are handled correctly
[32m✓ document_ttl.go completed successfully[0m
[34m
=== Running websocket_ttl.go ===[0m
╔════════════════════════════════════════════════════════╗
║   WEBSOCKET TTL EXPIRATION VERIFICATION TEST           ║
╚════════════════════════════════════════════════════════╝

This test verifies TTL expiration works via WebSocket connections.
We will use WebSocket to insert, query, and verify TTL expiration.

2026/01/26 19:56:06 No .env file found, using defaults
✓ Client connected

═══════════════════════════════════════════════════════════
TEST: WebSocket TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'WS TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = 0XdGy1sSHFbbNnRGxjjFRAHAAmv2xlNLOA8H3Kpmg8pgl1SO21usPfgnr4nfCwXLlGMVptt2iKvDGlb0fys-vA
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: FindByID(0XdGy1sSHFbbNnRGxjjFRAHAAmv2xlNLOA8H3Kpmg8pgl1SO21usPfgnr4nfCwXLlGMVptt2iKvDGlb0fys-vA)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: FindByID(0XdGy1sSHFbbNnRGxjjFRAHAAmv2xlNLOA8H3Kpmg8pgl1SO21usPfgnr4nfCwXLlGMVptt2iKvDGlb0fys-vA)
  Output: Error (expected) - request failed with status 404: ��Record not found (expired)
  ✓ PASS: Document expired (not found error)

═══════════════════════════════════════════════════════════
CLEANUP
═══════════════════════════════════════════════════════════
✓ Deleted test collection

╔════════════════════════════════════════════════════════╗
║          WEBSOCKET TTL TEST PASSED ✓                   ║
╚════════════════════════════════════════════════════════╝

WebSocket TTL expiration is working correctly:
  • Documents with TTL inserted via client expire correctly
  • Queries correctly return nil for expired documents
[32m✓ websocket_ttl.go completed successfully[0m
[34m
╔════════════════════════════════════════╗[0m
[34m║           Test Summary                 ║[0m
[34m╚════════════════════════════════════════╝[0m
[34mTotal: 10[0m
[32mPassed: 10[0m
[32mFailed: 0[0m
✅ [32mGo direct examples complete![0m
🧪 [36mRunning Go client library examples...[0m
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: map[id:C0cdRYJGx_CDUQX9KEo-Z7xnEj1aP7eVgSpO76FJukZeDqg9RZy_D6hv4Hwk-ojTGermKzjgViVKLE7dCy7n_g]

=== Find by ID ===
Found: map[active:true categories:[electronics computers] created_at:2026-01-26T19:56:14-05:00 data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:C0cdRYJGx_CDUQX9KEo-Z7xnEj1aP7eVgSpO76FJukZeDqg9RZy_D6hv4Hwk-ojTGermKzjgViVKLE7dCy7n_g metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 0
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-01-26 19:56:14 -0500 EST
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-01-26T19:56:14-05:00 data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:C0cdRYJGx_CDUQX9KEo-Z7xnEj1aP7eVgSpO76FJukZeDqg9RZy_D6hv4Hwk-ojTGermKzjgViVKLE7dCy7n_g metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-27T00:56:14Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:C0cdRYJGx_CDUQX9KEo-Z7xnEj1aP7eVgSpO76FJukZeDqg9RZy_D6hv4Hwk-ojTGermKzjgViVKLE7dCy7n_g metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: aV2GryWmqdaSN-Nwd5aRbgU4LpkdvlmF_N-4vcorgNoXmktMDBoB25n4kIvpjTx5f92NtzVXMPpv_DICztrqGw

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
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
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: GJ8_t8U7kDNHh6-p8vjcUaXLxqFovu0_bfa3HvYs1W6nsVQIsqr1_siO_iiTjJllD5x-sMYX7-posyIrkYHz6g

=== List Collections ===
Total collections: 14
Sample collections: [schema_employees_client_py scripts__ek0_testing ws_ttl_test test_accounts schema_products_client_py]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: map[userId:123 username:john_doe]

=== KV Batch Set ===
✓ Batch set 3 keys
  cache:product:1: success
  cache:product:2: success
  cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  cache:product:1: map[value:map[name:Product 1 price:29.99]]
  cache:product:2: map[value:map[name:Product 2 price:39.99]]
  cache:product:3: map[value:map[name:Product 3 price:49.99]]

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 11

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== KV Batch Delete ===
✓ Batch deleted 3 keys
  cache:product:1: deleted
  cache:product:2: deleted
  cache:product:3: deleted

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: WMEeySDrOtoDGgu2XUHjJLtRRH-EHjgCPeutSYY7vOtWMc1gsCFYOLYuUskVyVAbtmTQiiogkw5rXr4Yjv2PkQ
Created Bob: $500 - ID: FoJXkCPxyC_MIuD1rJm0D_6tYAz1Lr8lRzDub12NLXA2Rjd1anX45L-NqwXZY8_kDZbLj1FQ6LgPW4m21V5b5Q

=== Example 1: Begin Transaction ===
Transaction ID: a45f66ad-47b1-4723-85e3-646f129cbdc9

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: adcb5e07-0e2d-4b59-b890-70ea045a90ab
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
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
=== Search Examples ===

Setting up test data...
✅ Test data created

1. Basic full-text search:
Found 2 results
  1. Score: 25.740
  2. Score: 12.540

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400
  2. Score: 26.400
  3. Score: 26.400
  4. Score: 26.400

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600
  2. Score: 39.600
  3. Score: 39.600
  4. Score: 39.600

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200
  2. Score: 13.200

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.749
  2. Score: 0.745
  3. Score: 0.742

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.698
  2. Score: 1.500
  3. Score: 0.297

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400

=== Cleanup ===
✅ Deleted test collections

✅ Search examples completed!
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
=== Join Operations Examples ===

Setting up sample data...
✅ Sample data created

1. Single collection join (users with departments):
Found 2 users with department data
  - Bob Smith: Sales
  - Alice Johnson: Engineering

2. Join with filtering:
Found 1 users in Engineering
  - Alice Johnson: Building A

3. Join with user profiles:
Found 2 users with profile data
  - Bob Smith: Sales Manager
  - Alice Johnson: Senior Software Engineer

4. Join orders with user data:
Found 2 completed orders
  - Laptop ($0) by Alice Johnson
  - Mouse ($0) by Alice Johnson

5. Complex join with multiple conditions:
Found 2 users with example.com emails
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✅ Deleted test collections

✅ Join operations examples completed!
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: ppIzNAbBmGBbKqiBf3ou5pCpzYcWcgBJD_tv0XEu9TBFhR_mE5PQv1IUCHgsVsD5Bf7DESTz6HfFacwJ0fSORQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: ueuIJcdtyW1a-Ad9TXbmjnbc6_lYM67ut0_VDFTKa64cgT5F0gIId9UbKJnwvqQQSvqt9m84BsAPvLaqRhiUwA

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
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: t9EAsimAKZqZaMp8JeszGyqYJWbOj-8zZyQE4n7iYxwH5qE654h9HBIECgZ3hSRmQM4HJxvkDzF2k588VaJkvg

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
=== ekoDB as Edge Cache - Simple Example ===

Setting up edge cache collection...
✓ Cache entry created

Creating edge cache lookup script...
✓ Edge cache script created: 84oBzXCYXnFfXuareyyy6F50bpyfkPtkJ7e2RboIXMfmCC5bgF6BdQxOGUCNIYrO7_Q3n95WR4jnRwEl6LwKiw

Call 1: Cache lookup
Response time: 2ms
Found 1 cached entries

Call 2: Cache lookup (connection warm)
Response time: 2ms
Found 1 cached entries

🧹 Cleaning up...
✓ Cleanup complete

=== The Magic ===
- Your DATABASE is your EDGE
- No Redis needed
- No CDN needed
- No cache invalidation logic needed (TTL handles it)
- With ripples: All nodes auto-sync cache
- One service: Database + Cache + Edge Functions

✓ Example complete!
🚀 ekoDB Scripts Example (Go Client)

✅ Client initialized

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: pz1vNUOLoY_-o77JMyjznjvjcXolQWDnlpMXtf3PqSQSYSp-i0JX5puxukO0RAS6zBeB9NaqGMK5PxgL-FQNkA
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   map[avg_score:50 count:5 status:inactive]
   map[avg_score:60 count:5 status:active]
⏱️  Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 29
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

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
   ⏱️  Duration: 2.837959ms
   📊 Records: 1
   📦 Data: {
        "value": {
          "address": {
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "street":...

Second call (cache hit - from cache):
   ⏱️  Duration: 2.733ms
   📊 Records: 1
   📦 Data: {
        "value": {
          "address": {
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "street":...
   🚀 Cache speedup: 1.0x faster!

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
🚀 ekoDB Go Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: swdx_P6sM_9Bk5er3gCABH-7OG81co2xtIeODgPKpJtaAYeQR0z4jIZ-i4GfPU6e4wDgXMPbO77SiM164t---Q
📊 Found 2 category stats

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories

📝 Example 4: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Script saved
📊 Pipeline executed 3 stages

🧹 Cleaning up...
✅ Cleanup complete

✅ All complete script examples finished!

💡 This example demonstrates ekoDB's Script system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Script management (save, call, delete)
🚀 ekoDB Go KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: CpWFAMHzjxRWv1EpwclYRZeSEMvnLbFtlI3WMc198VEuUwZhT5oE9eYS-MK39XiCcJm50KtuYIzGNEaOE4FSbQ
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: UC6hnhryghq8BPCl60Oe8VwsmHCq391jKwXKBr1nf3Z2EG4sv9YWkxPBLj-ethXydreVPEf8-MdRLqvRdtk6zg
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: map[role:admin userId:user_abc]
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session
📝 Example 4: KV Operations in Scripts

✅ Script saved: c4pyFpn6ixmCYUeHVE-jxTdKktx_Y53j9CMz8YBfC61eeaXn-tyg8zKlpHvwD4USW3Kj-DVmvCfRa7EQpJoasA
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: u4S3PRSsfyeMgPfIb37wrBExkPMco-nOCcj3ZQ79WmdU-WZuCBW0B6Q59HpYc8ybZOdc2mZbcES2UuGFaXY1_Q
📊 Processed order with caching
⏱️  Stages executed: 3
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All KV & Wrapped Types examples completed!

💡 Key takeaways:
   ✅ Use Field* helpers for type-safe wrapped values
   ✅ FieldDecimal() preserves precision (no floating point errors)
   ✅ KV store is great for caching and quick lookups
   ✅ StageKv*() functions work within scripts
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_go (0_dw0ZtvZWV4u5sVTxhHkfnfnANQZrGUtagpjKmi6606ARIvSQ9ylQ3g5ylCaNVG681pclPMLLIYqSkINFzJFQ)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "cached_at": "2026-01-26T19:56:17-05:00",
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
      "id": "9VuieQdWS2cudaStvBBhGpxIGD90kKaX86td7OsOP1Z7_2a5OtZC-WhZV3wPFnTR_S_sMIjt5n_vCYgTVi62Qw"
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
Response time: 61ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
🚀 ekoDB Go Advanced Scripts Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Script saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Script saved
📊 Category breakdown:
   map[avg_price:365.6666666666667 category:Furniture count:3]
   map[avg_price:367 category:Electronics count:5]
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
🚀 ekoDB Go AI Scripts Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
📊 AI Response generated
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embed script saved
🧹 Cleaning up...
✅ Cleanup complete

✅ All AI script examples finished!

💡 This example demonstrates:
   ✅ Chat completions with system/user messages
   ✅ Embedding generation for text
🚀 ekoDB Go CRUD Scripts Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Script saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Script saved
📊 User counts by status:
   map[count:3 status:inactive]
   map[count:7 status:active]
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
🚀 ekoDB Go Search Scripts Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Script saved
📊 Found 5 documents
   1. map[type:String value:Introduction to Machine Learning] (map[type:String value:AI])
   2. map[type:String value:Getting Started with ekoDB] (map[type:String value:Database])
   3. map[type:String value:Natural Language Processing] (map[type:String value:AI])
   4. map[type:String value:Vector Databases Explained] (map[type:String value:Database])
   5. map[type:String value:Database Design Principles] (map[type:String value:Database])
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   map[category:Database count:3]
   map[category:AI count:2]
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: hOirRCXEPeKhjl3cqR-FjGWJYz-CC5616S4qL_3_e5F9XF2l-2x_HP-5_OkQmN-iOwXBNvT7oE538Kvzwf8R4Q

=== Sending Chat Message ===
Message ID: o0NiHCF2Bp92x0riDsFTzlytz5Z6o3gT9ec0Z3Z3lAf5hyqSea0vcU_lcFHKPdrWMR1CLOqY8EobvxYHk-zrdQ

=== AI Response ===
The available products and their respective prices are as follows:

1. **ekoDB**: A high-performance database product with AI capabilities priced at $99.
2. **ekoDB Pro**: This is the enterprise edition product with advanced features and it costs $299.
3. **ekoDB Cloud**: A fully managed cloud database service product available for $499.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:EI6eISrH5MM80icw0loDBge9grEajJJG9GM1gfqtrqmOSnZjydn4OQFu9tgnQcm2Hxw5U3eAOjyHJLsL0n6jRA name:ekoDB price:99] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:p0QURUVgXCG3myQG5VszgNwU6bOGbDS1Bhg23vDMZw5zHWor0kBoWRY1S3nsv4_Mm2Qynm80s1gFrdkT65rx9w name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:p4RiR2_cuoahWlEJUxDheDxEf4mPeZ4PBmv4nK8Et4YPFKDigpUpjE3hGt6HPp3wbprtDXXvdwo8iV_DHgcaLw name:ekoDB Cloud price:499] score:0.1111111111111111]

Execution Time: 2098ms

=== Token Usage ===
Prompt tokens: 457
Completion tokens: 71
Total tokens: 528

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: U7zPuembNAzeHG7EPTYc7srnEaxrbw3DVPtF_dYfnxoTIEpmKVpitzwid_F6PZEuhP7BXxvHdhcT644ZEjVjxQ

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the available context, one product that is available is the "ekoDB". It's a high-performance database product and it's priced at $99.

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
✓ Created second session: woXubsbUeD5ZkrfMPFhh4KzorIMwlqY44nzU-yATGRo-tEmr6MvBd3MketHD7BynjDK0wDfBu7EvT86fiOCBHg
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 4

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 3

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: iNZEpgS6r8Uqw0x8SEIEnhHg4JXAtjjC-KqEzM7n8XtjCj_svrdOBRIH8JMbHviLioiCVq76SLfW74PxEZma7Q

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, the available product is ekoDB. It is a high-performance database product and is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: kuS_CIv5krQVGtEtC-0RnE_08Jnaq9XKV9zyb6NF974kkfN536h7AZ2SEtX0aj8i0jtKTTNg34ejiP8eez2CSg
  Parent: iNZEpgS6r8Uqw0x8SEIEnhHg4JXAtjjC-KqEzM7n8XtjCj_svrdOBRIH8JMbHviLioiCVq76SLfW74PxEZma7Q

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: kuS_CIv5krQVGtEtC-0RnE_08Jnaq9XKV9zyb6NF974kkfN536h7AZ2SEtX0aj8i0jtKTTNg34ejiP8eez2CSg (Untitled)
  Session 2: iNZEpgS6r8Uqw0x8SEIEnhHg4JXAtjjC-KqEzM7n8XtjCj_svrdOBRIH8JMbHviLioiCVq76SLfW74PxEZma7Q (Untitled)
  Session 3: woXubsbUeD5ZkrfMPFhh4KzorIMwlqY44nzU-yATGRo-tEmr6MvBd3MketHD7BynjDK0wDfBu7EvT86fiOCBHg (Untitled)
  Session 4: hOirRCXEPeKhjl3cqR-FjGWJYz-CC5616S4qL_3_e5F9XF2l-2x_HP-5_OkQmN-iOwXBNvT7oE538Kvzwf8R4Q (Untitled)
  Session 5: SMtDTtUsPPGtex-Ojt2Tj_Ve8q93_4P8VE49G97Cqp1WuMODqN66buqX9WkHOJlHD56oTCxFfDjbDht_dAd4xw (Untitled)
  Session 6: vEMkVSZyHWnylKAfzuZhIfSJHFddMGWFbGS5usQHj1mOghiTFrJwKrtlOnm4q7P9cQjYzNv780LUL5XkrObmOg (Untitled)
  Session 7: DIyovkFIAzMyJ-y-TF-R3WXH-CnHmRyC3kFLhWSqFybg7QqLg6KzmNnjWLg4V4xbYSnZ3PbuGBPFJToeMYMbug (Untitled)
  Session 8: rHeX6XiKBfV6mnPQJ8Q7XI4mrrFMtZxMsSV-HOnSvtoXcSLVkZo32kkj-TuzGpbf3C0XFZD2600RDT1NCFYDnw (Untitled)
  Session 9: r6b9Xtu3fZXerjXX8LMYCAZoqSIvdapmKeLiGUgzMeS_A8z_ox_9W3kWlPiqv9s45hsnouA1hcxjVuZHpEUlPA (Untitled)
  Session 10: iYdwekxmt8ZM7XtMQgOGtgN534O1WtHojnY5RMQQHqz_wUtx3ZoW-2e2CK0xu0VSfw7TaAREHx1sDl_i1nhvcg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: kuS_CIv5krQVGtEtC-0RnE_08Jnaq9XKV9zyb6NF974kkfN536h7AZ2SEtX0aj8i0jtKTTNg34ejiP8eez2CSg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Map Creation ===
✓ Created record with native map: map[id:y8P59tp3ojXuaBHh8sdhHe7lBd6kJK-ymPrsdxc7nJZ80WV7OHELkzGtPfxi6mnw_enCh4oaq_O4NFFxOYTqFA]

=== Upsert Operation ===
✓ First upsert (update): map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:y8P59tp3ojXuaBHh8sdhHe7lBd6kJK-ymPrsdxc7nJZ80WV7OHELkzGtPfxi6mnw_enCh4oaq_O4NFFxOYTqFA name:map[type:String value:Alice Johnson]]
✓ Second upsert (insert): map[id:469gH7c3iFfvQeeJc9w12X3En6aVm7b3D8plKlHGDq5saHZQzYVAUFOKkf4fsyVdssJlSK7i2wIGRTPazWzsdQ]

=== Find One Operation ===
✓ Found user by email: map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:y8P59tp3ojXuaBHh8sdhHe7lBd6kJK-ymPrsdxc7nJZ80WV7OHELkzGtPfxi6mnw_enCh4oaq_O4NFFxOYTqFA name:map[type:String value:Alice Johnson]]
✓ User not found (as expected)

=== Exists Check ===
✓ Record exists: true
✓ Fake record exists: false (should be false)

=== Pagination ===
✓ Inserted 25 records for pagination
✓ Page 1: 10 records (expected 10)
✓ Page 2: 10 records (expected 10)
✓ Page 3: 7 records (expected ~7)

=== Cleanup ===
✓ Deleted collection

✅ All convenience methods demonstrated successfully!
Inserted with ripple: map[id:BCVwwGA1RIGuctPzhC62Ng-3TVeDKHvaSDmFfMdcunfkko6pKeKpD-pbIv4adVJDQE5bCuNPLlPjXovmfpWwEw]
Inserted with bypass_ripple: map[id:2bA8CJn_5rTTw_mJggaJC1CD5_ms4DZEHopDdP6drU1EEc6JB9L_-RbCONR36YRpNHvNEgWXwnkVrSsSD4rpxw]
Inserted with TTL and bypass_ripple: map[id:YMlQjdMiVe_nlDWst1c07OvesD8GWPL3_ml2t18n2RDHPROOooMojHSfa3EFbVe_VylSIpSKSE0eK2QuExELHg]
Updated with bypass_ripple: map[id:BCVwwGA1RIGuctPzhC62Ng-3TVeDKHvaSDmFfMdcunfkko6pKeKpD-pbIv4adVJDQE5bCuNPLlPjXovmfpWwEw name:map[type:String value:Product 1] price:map[type:Integer value:150]]
Deleted with bypass_ripple
Batch inserted with bypass_ripple: 2 records
Upserted with bypass_ripple: map[id:J9uomRIoNBgqbop5dHaGQUKk_HVlHx21LyPzUP8MtaOUNGXu7kBgOTfuyj4z1VuC66TEjujjVVY0oZ8M_cFGfA]
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields
Fetched 3 users with only 3 fields each

Example 2: Exclude sensitive fields
Fetched 2 admins without sensitive data
  Password field excluded: false

Example 3: Complex query with projection
Fetched 3 active users with profile fields

Example 4: Find by ID with projection
Fetched user profile: Alice Johnson

Example 5: Compare full vs projected data
Full query returned 12 fields per user
Projected query returned 12 fields per user

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
✅ [32mGo client examples complete![0m
