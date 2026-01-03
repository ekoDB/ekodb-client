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
✓ Authentication successful

=== Insert Document ===
Inserted: map[id:_3E84SdN6wrFCepCBIiUjUEZM14KI0JX0_dFOjSGdqMilN6M7S2vqKacvUwhTFaKmJUlAd1KCArQc0MXDxTXbA]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:_3E84SdN6wrFCepCBIiUjUEZM14KI0JX0_dFOjSGdqMilN6M7S2vqKacvUwhTFaKmJUlAd1KCArQc0MXDxTXbA name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found 1 documents

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:_3E84SdN6wrFCepCBIiUjUEZM14KI0JX0_dFOjSGdqMilN6M7S2vqKacvUwhTFaKmJUlAd1KCArQc0MXDxTXbA name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: Cn2lyhbRJkjOssYzqMmgazEZDqNYrdDLGCdkSiFAEhrZM1lbYhPfrKNF6uvbJ4fyOY3App0LKwiS1nKK_zbUJA

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
        "id": "Cn2lyhbRJkjOssYzqMmgazEZDqNYrdDLGCdkSiFAEhrZM1lbYhPfrKNF6uvbJ4fyOY3App0LKwiS1nKK_zbUJA",
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
        "id": "NqBUHcYBhQsxpBJ6Wr445GhB5erdJyUXaXh6g2Af8sV1K5EPeEu9kVbQ_O36z37jxYySk8R6UZeDUiP1R0tk7w",
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
        "id": "3REH3FFOJS88jlLDXdXST_xRlKUK1peqhumVvWGk4SnA-eoyIpwb9hCtPMEGaOqrhuTiQ8c5hactyKNl6qQWfg",
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
=== Running batch_operations.go ===[0m
✓ Authentication successful

=== Batch Insert ===
Batch insert response: map[failed:[] successful:[R77uI2UIX0foc68ubm7DBN71Dbg5Kd3oX6g25DlkNYscix4nfBWdW82yuwcnBNNNn9xGgRNzoTXuOXcQl513rQ dggrTuWIQ-n5KflrA5maCi9eQGE1fE_cCxJ3Pc8C_zZEv-jDNXm7J8c64tYl8FZ8KfzdckktkEzwseZKQ2VAcQ rGH0JCX8Hjae3tS12BsR99UcaBy3tgi299AW8uCZhx4q9q-pq9xmiilvgBq3jvK1QN6dALMpQX1Lw4X26JCjOw o6bRubb4t76gFi9nYQedp4tz4taC3-vIbaHqEbO3m8B4HLRmAddpzLByWrOL3jj9XHot2cEFMysFrXu7Tn1vJA AOH3Un8wcyMNJ6oM8Q05jser2RPI2DdtTn6TtRP3had3C7xg63yhyBwwtJAZ9_2f7StIGPWB3p0k9YW76JO2qw]]
✓ Batch insert completed

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
Batch update response: map[failed:[] successful:[Tk9CxWnNYso5JWtqVAo2PqV6300ZunfOQ-L2w8YyzCb1uyM8wxlX-ocSTaLzZSmioHNYBbEKQmE8q2WWXCd5RQ Kz4QJJD-jIIoMOYZk8jYzjashgBevjyzTuYyPAObvE40vLmgwpTjaN_C3oyi21BSkBl-vpu6gE5nqiSnBCFHhw gABxSIEg--i87bqJgkVBIjMbVll8ulyjyh7KZAyjrIqAJ5EKUXd8mI06rESkyGk8CHpH1bVI0wcG1CgME8wLmw]]
✓ Batch update completed

=== Batch Delete ===
Batch delete response: map[failed:[] successful:[Tk9CxWnNYso5JWtqVAo2PqV6300ZunfOQ-L2w8YyzCb1uyM8wxlX-ocSTaLzZSmioHNYBbEKQmE8q2WWXCd5RQ Kz4QJJD-jIIoMOYZk8jYzjashgBevjyzTuYyPAObvE40vLmgwpTjaN_C3oyi21BSkBl-vpu6gE5nqiSnBCFHhw gABxSIEg--i87bqJgkVBIjMbVll8ulyjyh7KZAyjrIqAJ5EKUXd8mI06rESkyGk8CHpH1bVI0wcG1CgME8wLmw]]
✓ Batch delete completed

✓ All batch operations completed successfully
[32m✓ batch_operations.go completed successfully[0m
[34m
=== Running kv_operations.go ===[0m
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
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: N_702jlS4pEEGp2WCaYDHkhjk2wy18TiL1_cwer1-BvpJDVNd5XV4yvrhK7I8agkG58W8nqYxBCtfylm7eE7kQ

=== List Collections ===
Total collections: 15
Sample collections: [schema_employees_client_py batch_users test_collection users demo_collection websocket_test scripts__ek0_testing schema_users_client_py ws_ttl_test schema_products_client_py chat_configurations__ek0_testing test_accounts ttl_cache chat_messages__ek0_testing schema_documents_client_py]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
[32m✓ collection_management.go completed successfully[0m
[34m
=== Running document_ttl.go ===[0m
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: qmBOJJl6r8qcJB5UQM1XkFzD7prsd9ZkyuCB2sABpdJWwJsJWKtwSmXLrpLirklhjgST0d3V2n0Q_QHTcyz3Tg

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: <nil>

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: 5XPRdBeHLVB37_zpsVhNcicnxLuPaGRDXfgjarGfqN4CClhLD0OSoClm83JatSLdDTrzV3tZi0GFAQMwILCVnQ

=== Query Documents ===
✓ Found 6 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
[32m✓ document_ttl.go completed successfully[0m
[34m
=== Running websocket_ttl.go ===[0m
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: 2VIFSpAEyGwJJpjrv2n5XS4AQTJZaQMLm-GmEs8xeoH7Xsq2aXYgWPDmpOjzVwwSFzqyP9gaqFNGEgEDKBSMuQ

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 3 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
[32m✓ websocket_ttl.go completed successfully[0m
[34m
=== Running http_functions.go ===[0m
🚀 ekoDB Scripts Example (Go/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: hLzO3_LhuTFXuVFhVIL12NZnVtK6XQRkIYBPHbV_uBzUEWPRRZiVrneVvRU5zsLRZ3daXQZJ4EOtzEwc2_DdLw
📊 Found 20 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: swI4CDIu3ljhsYYLMGYUDmkTJAcvGSpDRgVFPdpoDkRuZ9H_ZDaWuRflhh9If0CwLrkuJd0QC7upfjiaTMgLbw
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: 3OILdqGPn1cEeWA-_qOaKi2lz49mbXUDMk3fus5wvU0e-QfXRMOWsNo_VlHrbpqBj-oCQyFV41ztkTYfxHLYaw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":60,"count":10,"max_score":100,"status":"active"}
   {"avg_score":50,"count":10,"max_score":90,"status":"inactive"}

📝 Example 4: Function Management

📋 Total scripts: 12
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
[32m✓ http_functions.go completed successfully[0m
[34m
=== Running transactions.go ===[0m
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: VYLdPJAFUjc67O1JpAlAsmu-VAJn5nPmbtzzAW2Y3brUn4R4nCj48zn3tKUlz-x3JcdJpQRwuZQ6nY9j6PRU_A
Created Bob: $500 - ID: jVCIWEhptlTvoqHbW0eLXxB0c-ryyKL0nh5Gx8c_NtmH8fLXz_upJdMvCwGTrgG726_OMvgTmnD3JYug2awRhA

=== Example 1: Begin Transaction ===
Transaction ID: cc905101-2a94-4c65-b89d-24b46aaa6d90

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
New transaction: 1bdc13d7-678b-4d59-99e5-57655690064d
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
   ✅ Script saved: iXBYF3qTo-Gxbxz5w2b2QqXm7gYhHx3d77Yj_0kD8BCyXJXDJ_gbraiXeykz0jmWnFI2BcbfiR41lCSjhawW0A

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: dAqj9iLLLxs6Rm5iKXISyPvdV6TyqG2Y9MkFWooNt4i5vHXbBtUyC2tIcrRJRGnmBED1Cgr8f2kvPWryu1Cljg
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: 6VQxhlpbQMpu60jekNHGpwJxvXVQtq-hxLTE0bp5MXK2aqml1pXWb3tiLSckqILSSq28q0Jn9H8I4xMHLunVVA

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
   ✅ Script saved: iMAYJT2EfOUy6K9yRXC-f08m-LVRBkZ1Gq5h1f2i6UGin97oADEumW973nsPumD-iPzu8RvjV4fSmNl9VdAVXg

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
   ✅ Script saved: 8TMJ3yePdytXKQqAeQD31tittAENyy7Hgim2_Jdtfo0WzaDgTkKWiIDOvHd-bFIVHkhwGglv_tih40A5GXZ9tQ

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: iXBYF3qTo-Gxbxz5w2b2...
   ✅ Deleted script: 6VQxhlpbQMpu60jekNHG...
   ✅ Deleted script: iMAYJT2EfOUy6K9yRXC-...
   ✅ Deleted script: 8TMJ3yePdytXKQqAeQD3...
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
Inserted: map[id:tp_KfIHrm4oThU4I7YKsJOSYDea8izwRDSgG1FwWcxoLEf3dvz9UuGxNiXWaRj_7yghzAcKV3LDZdCDwPSykTQ]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-03T21:35:09Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:tp_KfIHrm4oThU4I7YKsJOSYDea8izwRDSgG1FwWcxoLEf3dvz9UuGxNiXWaRj_7yghzAcKV3LDZdCDwPSykTQ metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Test Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:42]]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 0
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-01-03 21:35:09 +0000 UTC
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-01-03T21:35:09Z data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:tp_KfIHrm4oThU4I7YKsJOSYDea8izwRDSgG1FwWcxoLEf3dvz9UuGxNiXWaRj_7yghzAcKV3LDZdCDwPSykTQ metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-03T21:35:09Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:tp_KfIHrm4oThU4I7YKsJOSYDea8izwRDSgG1FwWcxoLEf3dvz9UuGxNiXWaRj_7yghzAcKV3LDZdCDwPSykTQ metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
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

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 4

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: vlOvjPTDXhomKvovNhv9J-7ClLzzkIs4YoqB_FIJdZOs__eWzzrRqHXtXiWSZRqmnIWOabfh5paC2RKPKF1kcQ

=== List Collections ===
Total collections: 14
Sample collections: [schema_employees_client_py batch_users test_collection client_collection_management_go websocket_test]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: PUAD1AG3yZeFZf9cHy4hdvzvYaniGmmWoV_bQP6uhmEaYvKE7s67zsSwgOZ5CSPUTNqpRnR53mTv8Ax1cVZ9TQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 3HFAM_ng1pJAR32_tm9lNiJLZ8VMZDD5UgNvad6xiSWrxaUazmaJirSXFOZfXS_aTuNsF2IRlkIs8X6rZTxqYg

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

=== Inserting Test Data ===
✓ Inserted test record: QFwp85JmxppGLheTG5THc-KE3sQ8nKbZr9D1059TMJ9tLpebwNlQbCNBWhTHqRlEj2BSb_ikj4b0dR4sK7UB0w

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: h52ETPl7E2tVSTJW9RGMx1iKufEQlfPBODRu06DVtnbf3ulxSOLHJRLPGCbF1ZBgcqu1yupMtdZsNw2BJh_PJw

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: 2UHvU8UWM77TNkFkjMaTU754Z7MRFajZpf8CCNnHtWyx3ilR76h8zZSuGv-KlxJJSLPd6y2Tbh2TOhhNVOB7Ew

=== Sending Chat Message ===
Message ID: frlRFmot0bCTH0kjzRKmKXBoTAtl54fw909REMdRZCjESuKRw51BSrFxrLGmmDIypaPG69yXCFg6iPGdL8n0qQ

=== AI Response ===
The available products are:

1. ekoDB: This is a high-performance database product with AI capabilities. The price of this product is $99.

2. ekoDB Pro: This is an Enterprise edition product with advanced features. The price of this product is $299.

3. ekoDB Cloud: This is a fully managed cloud database service product. The price of this product is $499.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:fefnrW-HRD8B4T6MK1vu82LrHBRcN2kmOVOhO1Uj6e6JS0gSCR1NjNivzhc4sBYghfkcKhcSs8mFBfDxK5EsxQ name:ekoDB price:99] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:ezTD091olpsqTbjFQj_6CJcVTrr8v50UApnt1vV3sg2__Xwqnycafh_9klIpbro8kG2IAhnQwzOuZjUbZY7FPw name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:j2kVxGFlTVzO_Af6GTliZ8CgLHrQsqtZEPjPRv46pqYg9_BntPdxV49e4p9lXIGk3fvdXGED3CniP_mAI-SHKg name:ekoDB Cloud price:499] score:0.1111111111111111]

Execution Time: 2561ms

=== Token Usage ===
Prompt tokens: 609
Completion tokens: 83
Total tokens: 692

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: DTY5mWctSn5Krw3sgTQNMfEazH3V04OiZEt9zM6kSi8E4kav4Tg_jfUI8SxWcpcFxSKfrG0F1KY-_7sYjoQSGw

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information available, one product that is available is ekoDB. It is a high-performance database product and it costs $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: cYD76vy6mg7d_gjkX_XT056TjZP38CRouEtEJhqF45N689Ob2q_5i2SEnwxD9GA21iRXVa4toIkMk8ne_EPi6A
  Parent: DTY5mWctSn5Krw3sgTQNMfEazH3V04OiZEt9zM6kSi8E4kav4Tg_jfUI8SxWcpcFxSKfrG0F1KY-_7sYjoQSGw

=== Listing Sessions ===
✓ Found 9 sessions
  Session 1: cYD76vy6mg7d_gjkX_XT056TjZP38CRouEtEJhqF45N689Ob2q_5i2SEnwxD9GA21iRXVa4toIkMk8ne_EPi6A (Untitled)
  Session 2: DTY5mWctSn5Krw3sgTQNMfEazH3V04OiZEt9zM6kSi8E4kav4Tg_jfUI8SxWcpcFxSKfrG0F1KY-_7sYjoQSGw (Untitled)
  Session 3: 2UHvU8UWM77TNkFkjMaTU754Z7MRFajZpf8CCNnHtWyx3ilR76h8zZSuGv-KlxJJSLPd6y2Tbh2TOhhNVOB7Ew (Untitled)
  Session 4: HvdlKRKII1VWid5cNQ0-HK17Qbk6_WXWU-tlxQ7Fjx7gnJwvMUCldZ21ikUg_fz2dfOn5hv90pnIG3uLW3Z3Rw (Untitled)
  Session 5: wg3xdpQyALUaf3raEAK_aaMirLmzVKNiRvDmUlFEchZL29zEYSS5OQTeoRJ_SUgMMVXZ0tYKAanLUknaT9L6UA (Untitled)
  Session 6: suW32bDnrdg3dzucV75MS8O41JMRsxVPZR4KXhybpNPRgnT6cVCZdvuc9_7tH0H_Nvr4g0bW5094uRafHh7o-Q (Untitled)
  Session 7: 0dBcE6CuGPe38twSLr6KwkhV4MWkOcyUILsxWuw1XuDS6lcHD9gKIUGow5S05ZNUmWnef4DZqYv4Z5Wg4QZaeg (Untitled)
  Session 8: 4BJU8CUhE7wK9eBg_tXVfWzd-xoRSNQaJvE9j4aZcul7zYysl8DFpVIexM7wACzOVLGIjvWoSqD_gOxgxQu5mA (Untitled)
  Session 9: DcLiiFvuMZQ0jNmXFlxO-028DyjsLecXVNmcPmQuWrR-3Dwbx99wRtK3uS-9YGYwDiFqoD-Wa7sA1oEN4Sq7Hw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: cYD76vy6mg7d_gjkX_XT056TjZP38CRouEtEJhqF45N689Ob2q_5i2SEnwxD9GA21iRXVa4toIkMk8ne_EPi6A

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: e945LmXeIaQE9gwjx_JXTWAihL8SWHorffSmnFuURl7tAyLch_Kk3vhwMlsp5AyYroM-Uph7To9TLA3JL0Z58A

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the provided context, one available product is "ekoDB." It is a high-performance database product. The price for this product is $99.

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
✓ Created second session: bJbj73clvxGp4MYjUexkDsFYJZ78e9upaU8opyzkREOc5kTXrWSo5hoLF3ikXUAIGgYil_ZVf0xY0U0x0pVNIg
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
  1. Score: 0.759
  2. Score: 0.745
  3. Score: 0.732

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.693
  2. Score: 1.504
  3. Score: 0.298

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
  - Mouse ($0) by Alice Johnson
  - Laptop ($0) by Alice Johnson

5. Complex join with multiple conditions:
Found 2 users with example.com emails
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✅ Deleted test collections

✅ Join operations examples completed!
🚀 ekoDB Scripts Example (Go Client)

✅ Client initialized

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: bnF8V2nADKTmQ9pNySh3iotjxEz0qgauMvybuxpIIzkGE8VCZNfI3iUEThCvbYBG9iQFtAbJywIf15AdxS4HUg
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

📋 Total scripts: 14
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
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: Imjydi5Gqak3tUJrh3_dPVvzlZPPFpglmmPyi0FbPn2udQvsmq_CSuMVHH8m-rmxPRid7MDYCC22IyUuGjhuaQ
Created Bob: $500 - ID: 4ZHEPsMXA2CxsJAmG1B0j_gk45SliiaG0frTjI2_2GfxCjNZ_KiGOhWva-h9kNMCG68zgviZwgM0_8Q8JtK7WQ

=== Example 1: Begin Transaction ===
2026/01/03 16:35:24 invalid isolation level: ReadCommitted (must be one of: READ_UNCOMMITTED, READ_COMMITTED, REPEATABLE_READ, SERIALIZABLE)
exit status 1
make[2]: *** [test-examples-go-client] Error 1
