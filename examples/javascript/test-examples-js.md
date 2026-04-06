make test-examples-javascript
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 972ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
[34m
╔════════════════════════════════════════╗[0m
[34m║  ekoDB JavaScript Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m
[34m
=== Checking Server Connection ===[0m
[32m✓ Server is ready[0m
[34m
=== Getting Authentication Token ===[0m
[32m✓ Authentication successful[0m
[33m
=== Running 10 Examples ===[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/simple_crud.js ===[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {
  id: 'n6qDpKVwIG7CvYM_pbSkMz8jINOFBaPrdBtji9I-wJI6vAWL_JTv61fL6D1lDkIG4hijXREizCPg7mYQly8uuQ'
}

=== Find by ID ===
Found: {
  active: { type: 'Boolean', value: true },
  id: 'n6qDpKVwIG7CvYM_pbSkMz8jINOFBaPrdBtji9I-wJI6vAWL_JTv61fL6D1lDkIG4hijXREizCPg7mYQly8uuQ',
  value: { value: 42, type: 'Integer' },
  name: { type: 'String', value: 'Test Record' }
}

=== Find with Query ===
Found documents: [
  {
    value: { value: 42, type: 'Integer' },
    active: { type: 'Boolean', value: true },
    id: 'n6qDpKVwIG7CvYM_pbSkMz8jINOFBaPrdBtji9I-wJI6vAWL_JTv61fL6D1lDkIG4hijXREizCPg7mYQly8uuQ',
    name: { type: 'String', value: 'Test Record' }
  }
]

=== Update Document ===
Updated: {
  name: { value: 'Updated Record', type: 'String' },
  value: { type: 'Integer', value: 100 },
  id: 'n6qDpKVwIG7CvYM_pbSkMz8jINOFBaPrdBtji9I-wJI6vAWL_JTv61fL6D1lDkIG4hijXREizCPg7mYQly8uuQ',
  active: { type: 'Boolean', value: true }
}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/simple_websocket.js ===[0m
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: -Yj6FXVKqtIpXSDw1ZskUD_hr_mPEJd9qIbvPpuNkR72r2TVSj3CVpB0KROO-vVR3c2aEA3_R0GwJCveKYOBxQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1775366468191",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "ec3L9X5xmLser5g9GPnFon_UALisZsV0hhKjRm_VJEpL4uTz0ahQM0jhRS5z3wLMWmVJsr_BhD1jqQ-nQujzUw",
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
        "id": "-Yj6FXVKqtIpXSDw1ZskUD_hr_mPEJd9qIbvPpuNkR72r2TVSj3CVpB0KROO-vVR3c2aEA3_R0GwJCveKYOBxQ",
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
        "id": "M3mSBAp10I75QixK6vT-brdSo_q7MSHafLfCr4Q0cOmppQVBlQCXrDgmbtfsD9RdbXFqtDeOqvbEC0gjfRifNQ",
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
        "id": "c1i-VSVWIrdWR6V_fqAt9gYeFb4NEK79wRxhE2e-rS1NrAVhfIwV131gxPfckEaWJEf3ZhSJFn1SDg5j6md7Fw",
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
✓ Retrieved 4 record(s) via WebSocket

✓ WebSocket example completed successfully
WebSocket closed
[32m✓ simple_websocket.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/http_functions.js ===[0m
🚀 ekoDB Functions Example (JavaScript/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: YDk4UeEMLpX7I6TGU28ffPSXTg9PAqdiAB01c0B5QcWhes-E2Q98HAdR-UwMvS4YlEUgnAlWbDczulIlSSp0nw
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: cw-5uvoXrWUglrHPvWGOg3sS8B9h9N3urGblqR7-0UBNfHfOJPaFHWAerG58nvawJA2LnHNQcDP5OoOGOg3RwQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: K_JMxrKsOTNQHbf3rpLsPd6XwhKt40vDCPsPr15YTn2nMuS3vWwkPB4ukiJIv6l_G-ZMjCjXmaUdrUBwNv9KVw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"count":10,"status":"inactive","max_score":90,"avg_score":50}
   {"count":10,"status":"active","avg_score":60,"max_score":100}

📝 Example 4: Function Management

📋 Total functions: 33
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
[32m✓ http_functions.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/batch_operations.js ===[0m
=== Batch Operations (Direct HTTP) ===

✓ Authentication successful

=== Batch Insert ===
✓ Batch inserted 5 records
✓ Verified: Found 20 total records in collection

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
✓ Batch updated 3 records
✓ Verified: Record updated with status="active"

=== Batch Delete ===
✓ Batch deleted 3 records
✓ Verified: Records successfully deleted (not found)

✓ All batch operations completed successfully
[32m✓ batch_operations.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/kv_operations.js ===[0m
=== Key-Value Operations (Direct HTTP) ===

✓ Authentication successful

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: { username: 'john_doe', userId: 123 }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { name: 'Product 1', price: 29.99 }
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
[32m✓ kv_operations.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/collection_management.js ===[0m
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: b7iNdiQ_UOoh3OkE3iW6Bfg8wwe5V5qUiER5D8ripVrJ-iOmjUzLrsN-bmcKC9T4fPkfsgn-nzUAFFgtsqb7Jw

=== List Collections ===
Total collections: 25
Sample collections: [
  'schema_products_client_go',
  'ttl_cache',
  'schema_products_client_ts',
  'chat_messages__ek0_testing',
  'demo_collection'
]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
[32m✓ collection_management.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/transactions.js ===[0m
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: ytvuv1EudC6yDR92grep3qEelZ9f-IOWKS5pjlBbnpO5q9yOvLlijdCYais0EtzRnZAFhTj_5plgHOzffp6d7w
Created Bob: $500 - ID: PPYdXUECwWV61Aa42EnxGW3EEDRLH8cxL3It8VpzswI3J9fqXWdbwC2ovCM0pwiGRTepuLw0Nd9kJC-iLQwR4Q

=== Example 1: Begin Transaction ===
Transaction ID: fbbdd209-0606-4912-8193-131d812fdf87

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: {"type":"Integer","value":800}
Bob: {"type":"Integer","value":700}

=== Example 5: Rollback ===
New transaction: c31119d7-04dd-424f-8b78-45f21087581d
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"value":700,"type":"Integer"}

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
[32m✓ transactions.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/crud_functions.js ===[0m
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
📝 Function 1: Insert + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: clEvVBswZ2d_oO_vrh2cEnEQfvtTcP7VOSKFfSIasZbHmVyMysZYuv_XGRJe-6TrFWkIbhy9UD8Z4powJ47qfA

2️⃣ Calling Function (Insert + Verify)...
   ✅ Function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Name: [object Object]
   📋 Email: {"value":"alice@example.com","type":"String"}
   📋 Status: [object Object]
   📋 Credits: [object Object]

============================================================
📝 Function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: inwBwQs5yyF5ERuXS3_jBDNolOm0IBWPm_V2FdCo4vBeHFI_r2jSSsxE8hBgpKXwSb-1WDvsA193f-o8OiY1zg

2️⃣ Calling Function (Query + Update + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {"type":"String","value":"active"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Function 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: rfIn0oT9CXQ4i8iOcAH7gqoyxWzor3EWp1en2xjcHdTGfVXSmS1mh5PT8lb7i65e5JlDLcwpF-d5ik3YoBsMKA

2️⃣ Calling Function (Query + Update Credits + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"type":"Integer","value":0}
   📋 Status: {"value":"active","type":"String"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: BwtxcAeaJLtpLzPmXMlLFtcR_lAI1r4TfTxkVvBBuOTLtwwO-PbDvoCMcYNd2NXNieDSu0ROKZBxlq9mRyqf1g

2️⃣ Calling Function (Query + Delete + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted function: clEvVBswZ2d_oO_vrh2c...
   ✅ Deleted function: inwBwQs5yyF5ERuXS3_j...
   ✅ Deleted function: rfIn0oT9CXQ4i8iOcAH7...
   ✅ Deleted function: BwtxcAeaJLtpLzPmXMlL...
   ✅ Deleted collection: users

============================================================
✅ Complete CRUD Functions Example Finished!
============================================================

💡 Key Takeaways:
   ✅ Functions chain Functions together
   ✅ Each function demonstrates operation + verification
   ✅ Parameters make functions reusable
   ✅ Verification is built into the function itself
   ✅ Complete CRUD lifecycle in 4 focused functions
[32m✓ crud_functions.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/document_ttl.js ===[0m
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
  Output: Document ID = jvg3v0SncPV1WoM7QFMQAhlrsIY6_VDVfufLcwlm0X3r0s1VK-kbrLZl89yOiPiQHg7amJjNBnaXe7fNRQJcCw
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(jvg3v0SncPV1WoM7QFMQAhlrsIY6_VDVfufLcwlm0X3r0s1VK-kbrLZl89yOiPiQHg7amJjNBnaXe7fNRQJcCw)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(jvg3v0SncPV1WoM7QFMQAhlrsIY6_VDVfufLcwlm0X3r0s1VK-kbrLZl89yOiPiQHg7amJjNBnaXe7fNRQJcCw)
  Output: Error (expected) - Request failed with status 404: {"error":"Record has been deleted"}
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
[32m✓ document_ttl.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/websocket_ttl.js ===[0m
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
  Output: Document ID = ydBxL7LB-pL_67LsgkRK1OVd_rMqbt0FaRu93wORkUoWvL3d37lyhWfX78slnfd2ppb6SBdLdHvCqsWgjIptIA
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(ydBxL7LB-pL_67LsgkRK1OVd_rMqbt0FaRu93wORkUoWvL3d37lyhWfX78slnfd2ppb6SBdLdHvCqsWgjIptIA)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(ydBxL7LB-pL_67LsgkRK1OVd_rMqbt0FaRu93wORkUoWvL3d37lyhWfX78slnfd2ppb6SBdLdHvCqsWgjIptIA)
  Output: Error (expected) - Request failed with status 404: {"error":"Record not found (expired)"}
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
  • Queries correctly return null for expired documents
[32m✓ websocket_ttl.js completed successfully[0m
[34m
╔════════════════════════════════════════╗[0m
[34m║           Test Summary                 ║[0m
[34m╚════════════════════════════════════════╝[0m
[34mTotal: 10[0m
[32mPassed: 10[0m
[32mFailed: 0[0m
✅ [32mJavaScript direct examples complete![0m
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.15.2 prepare
> npm run build


> @ekodb/ekodb-client@0.15.2 build
> tsc


up to date, audited 52 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.15.2 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning JavaScript client library examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 987ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: '-keKo0uKpGbQ4u9w4Ub2XcmrnZBvvvRBaO2wiXAmxVrEYg-ZnaoePu_UGg4AG3ntJDyVmNIdD4yXfrahWQLynw'
}

=== Find by ID ===
Found: {
  value: { value: 42, type: 'Integer' },
  active: { type: 'Boolean', value: true },
  id: '-keKo0uKpGbQ4u9w4Ub2XcmrnZBvvvRBaO2wiXAmxVrEYg-ZnaoePu_UGg4AG3ntJDyVmNIdD4yXfrahWQLynw',
  name: { type: 'String', value: 'Test Record' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  id: '-keKo0uKpGbQ4u9w4Ub2XcmrnZBvvvRBaO2wiXAmxVrEYg-ZnaoePu_UGg4AG3ntJDyVmNIdD4yXfrahWQLynw',
  active: { type: 'Boolean', value: true },
  name: { value: 'Updated Record', type: 'String' },
  value: { value: 100, type: 'Integer' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: YkMulus5gEYM1hJX-MOP0KEHNAKPegSqxoBpHlfFz-D3RK3d2VP6fsD7QMy8O9QcZtKpzSkIcGbnJgEvh5II-Q

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
Collection created with first record: sA-TgTO2wPy5gI5zQ4bhSpvbMcn-ZwbNPUTiJqtgR52Es-IflwYmyh9FGkA6oS9BrJpWAzhHSOW1JlDSu6xw6g

=== List Collections ===
Total collections: 24
Sample collections: schema_products_client_go,ttl_cache,schema_products_client_ts,chat_messages__ek0_testing,schema_documents_client_py

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
Retrieved value: { username: 'john_doe', userId: 123 }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { name: 'Product 1', price: 29.99 }
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 0 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 17

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: JYgw8_BbIojHi68TEAtGqJZmRBeyAxTU7-WjB24kIA3G0ytYRsAazXvIkjl3uBtIe_yfN62gGpd8Cq66D0ovTg
Created Bob: $500 - ID: P3CrB5XLgaFHEIsI2P4eECmNk7fcZxtrT639MsxwSLjolRo094JthihhMAugj7Z7Ts-sssThamtUgxV9Do9Jmw

=== Example 1: Begin Transaction ===
Transaction ID: 6b94e638-e36f-441d-8d2c-14835406575c

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 3a9b0515-87ea-4a9b-847e-061ba906c7cb
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
  1. Score: 25.740, Matched: name.value, email, name, email.value
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio.value, bio, title.value, title
  2. Score: 26.400, Matched: bio.value, title, bio, title.value
  3. Score: 26.400, Matched: title, bio, bio.value, title.value
  4. Score: 26.400, Matched: title, bio, title.value, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title.value, bio, bio.value, title
  2. Score: 39.600, Matched: bio, bio.value, title, title.value
  3. Score: 39.600, Matched: bio, title, bio.value, title.value
  4. Score: 39.600, Matched: bio, title.value, bio.value, title

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.760, Matched: 
  2. Score: 0.750, Matched: 
  3. Score: 0.747, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.699, Matched: title.value, title, content.value, content
  2. Score: 1.504, Matched: title.value, title, content.value, content
  3. Score: 0.300, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills, skills.value, bio, bio.value

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
Schema fields: [ 'age', 'email', 'name', 'status' ]
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
Found 2 users with department data:
  - Alice Johnson: Engineering
  - Bob Smith: Sales

2. Join with filtering:
Found 1 users in Engineering:
  - Alice Johnson: Building A

3. Join with user profiles:
Found 2 users with profile data:
  - Alice Johnson: Senior Software Engineer
  - Bob Smith: Sales Manager

4. Join orders with user data:
Found 2 completed orders:
  - Laptop ($1200) by Alice Johnson
  - Mouse ($25) by Alice Johnson

5. Complex join with multiple conditions:
Found 2 users with example.com emails:
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✅ Deleted test collections

✅ Join operations examples completed!
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: injV9g_dN698LWWaPZPy17joeGtIknq7cD6G_zTgCqWIxgc7GleJoXUwSJfeggXgsQoB8te2CgpAhIoe9ErKuw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: VW560nW2Bxug3t5jEpU9INe2ZPGE-9nfwOwZxn9QLfjQdWUI9E6N8QSqqa3EytAAAVseW2mIGEbngKojA4uZLw

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
✓ Inserted document with TTL: MFUraBc5G-BHavBVjqHjOWA74MMMmn1dKVFDFCvUd0TbrAHS81Nq3u8CLHV0yKz4vY0QdwoD1A3nkoy1yq_5Mw

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
=== ekoDB as Edge Cache - Simple Example ===

Creating edge cache function...
✓ Edge cache script created: csm8CIXwiX8aM7aO01nE67t8h__3Gte4WbPDZEBn8GcDBvMgRUCtYnD8J0Yl6FBvXtfUF7f4AL4C2rMSWqGrng

Call 1: Cache miss (fetches from API)
Response time: 41ms
Result: {
  "records": [
    {
      "value": {
        "longitude": -73.99308,
        "current": {
          "temperature_2m": 5.6,
          "time": "2026-04-05T05:15",
          "interval": 900
        },
        "timezone_abbreviation": "GMT",
        "generationtime_ms": 0.01704692840576172,
        "timezone": "GMT",
        "latitude": 40.710335,
        "current_units": {
          "temperature_2m": "°C",
          "interval": "seconds",
          "time": "iso8601"
        },
        "elevation": 32,
        "utc_offset_seconds": 0
      }
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

Call 2: Cache hit (served from ekoDB)
Response time: 43ms (1x faster!)
Result: {
  "records": [
    {
      "value": {
        "longitude": -73.99308,
        "current": {
          "temperature_2m": 5.6,
          "time": "2026-04-05T05:15",
          "interval": 900
        },
        "timezone_abbreviation": "GMT",
        "generationtime_ms": 0.01704692840576172,
        "timezone": "GMT",
        "latitude": 40.710335,
        "current_units": {
          "temperature_2m": "°C",
          "interval": "seconds",
          "time": "iso8601"
        },
        "elevation": 32,
        "utc_offset_seconds": 0
      }
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

=== The Magic ===
- Your DATABASE is your EDGE
- No Redis needed
- No CDN needed
- No cache invalidation logic needed (TTL handles it)
- With ripples: All nodes auto-sync cache
- One service: Database + Cache + Edge Functions

✓ Example complete!

🚀 ekoDB Functions Example (JavaScript Client)

✅ Client initialized (token exchange automatic)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: udO8nuxUoon4naJdz9uvw6X3N12I38ATzvq2p8uWIvUKy3lN2nBmN5dXRoRTVEVk4J5DWNl8R7jaXvXvOjNdsA
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

✅ Function saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 2 groups
   {"count":5,"status":"inactive","avg_score":50}
   {"status":"active","avg_score":60,"count":5}
⏱️  Execution time: 0ms

📝 Example 4: Function Management

📋 Total functions: 36
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

📝 Example 5: Multi-Stage Pipeline

✅ Multi-stage function saved
📊 Pipeline executed 2 stages
⏱️  Total execution time: 0ms
📈 Stage breakdown:

📝 Example 6: Count Users

✅ Count function saved
📊 Total user count: 10
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Deleted collection
✅ Deleted test functions

✅ All examples completed successfully!

💡 Key Advantages of Using the Client:
   • Automatic token management
   • Type-safe Stage builders
   • ChatMessage helpers
   • Cleaner, more maintainable code
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
   Department: {"value":"engineering","type":"String"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 39ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "name": "Leanne Graham",
    "address": {
      "zipcode": "92998-3874",
      "city": "Gwenborough",
      "street": "Kulas Light",
      "suite": "Apt. 556",
      "geo": {
      ...

Second call (cache hit - from cache):
   ⏱️  Duration: 38ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "name": "Leanne Graham",
    "address": {
      "zipcode": "92998-3874",
      "city": "Gwenborough",
      "street": "Kulas Light",
      "suite": "Apt. 556",
      "geo": {
      ...
   🚀 Cache speedup: 1.0x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: get_verified_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: User 1
   Department: engineering

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - get_verified_user: Specific workflow


✅ All composition examples completed!
🚀 ekoDB JavaScript Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: AYOv6-ZyTpKPk_OnVnWm3DLTi0yQUgRxrfO9nuDbyjBDBepQ-532Yh2FQsQaHW7dV2cV1M5qNNKm3mwTqs46Aw
📊 Found 2 product groups
   {"count":3,"category":"Electronics","avg_price":575.6666666666666}
   {"count":2,"category":"Furniture","avg_price":474}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"category":"Electronics","count":3}
   {"category":"Furniture","count":2}
⏱️  Execution time: 0ms

📝 Example 4: High Rating Products

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 5: Function with Parameter Definition

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 6: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Function saved
📊 Pipeline executed 3 stages
⏱️  Total execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All complete function examples finished!

💡 This example demonstrates ekoDB's Function system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Parameter definitions
   ✅ Function management (save, call, delete)
🚀 ekoDB JavaScript KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: ueo_m6sewHO3J8Sbgbqm40Bc7QswkuOE8K-u61jsOXEqEGwKvDwOeoKArkJaqtBJ71YZwg7r4gmONjf17HUHPw
✅ Inserted 2 products with wrapped types

📝 Example 2: Function with Wrapped Type Parameters

✅ Function saved: i2JgCXrD6jCcSkzHgKDkYYtKNgRB_mb6YkqYA3a4s-kOo0cfpfubhzpAsL38AR7eVIk8LboXRgKpJdTlnUk_0Q
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: s_5nBMO2fkE-TJ9e5Lt8Ka-bGkoe0uNdqQm_B3SipbnDCyNcT8YCu3UqHPMAuoHia6nla03zEGctWs8sjHqM2A
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: LJMajLCx5UnZcinMa24JGxpfpwLK_C1MtkZoHnb1ywS4pUbjVrZJKeLWBvmrfkOVqqCWWeJp9537IM-g_at0JQ
📊 Processed order with caching
⏱️  Stages executed: 3
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All KV & Wrapped Types examples completed!

💡 Key takeaways:
   ✅ Use Field.* helpers for type-safe wrapped values
   ✅ Field.decimal() preserves precision (no floating point errors)
   ✅ KV store is great for caching and quick lookups
   ✅ Stage.kv*() functions work within scripts
   ✅ Combine KV caching with collection inserts for real workflows
🚀 ekoDB CRUD Functions Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Function saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Function saved
📊 User counts by status:
   active: 7 users
   inactive: 3 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Function saved
📊 Average scores by role:
   user: 70.0 (7 users)
   admin: 20.0 (3 users)
⏱️  Execution time: 0ms

📝 Example 4: Top Users by Score

✅ Function saved
📊 Users (showing first 5 of 10):
   1. User 7 - Score: 70
   2. User 9 - Score: 90
   3. User 4 - Score: 40
   4. User 3 - Score: 30
   5. User 6 - Score: 60
⏱️  Execution time: 0ms

📝 Example 5: User Summary Statistics

✅ Function saved
📊 User summary (2 groups):
   inactive users: 3 users, Total Score: 180
   active users: 7 users, Total Score: 370
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD function examples completed!
🚀 ekoDB Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Vector Databases Explained (Database)
   2. Getting Started with ekoDB (Database)
   3. Introduction to Machine Learning (AI)
   4. Natural Language Processing (AI)
   5. Database Design Principles (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   AI: 2 documents
   Database: 3 documents
⏱️  Execution time: 0ms

📝 Example 3: Select Specific Fields

✅ Function saved
📊 Document titles (5 docs):
   1. Vector Databases Explained
   2. Getting Started with ekoDB
   3. Introduction to Machine Learning
   4. Natural Language Processing
   5. Database Design Principles
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Function saved
📊 Projected documents (showing first 3):
   1. Vector Databases Explained
   2. Getting Started with ekoDB
   3. Introduction to Machine Learning
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Function saved
📊 All documents (5 total, showing first 2):
   1. Vector Databases Explained (Database)
   2. Getting Started with ekoDB (Database)
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search function examples completed!
🚀 ekoDB Advanced Functions Example

📋 Setting up test data...
✅ Created 10 products

📝 Example 1: List All Products

✅ Function saved
📊 Found 10 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Function saved
📊 Found 2 categories
   Furniture: 4 items (avg $294.00)
   Electronics: 6 items (avg $325.67)
⏱️  Execution time: 0ms

📝 Example 3: Count All Products

✅ Function saved
📊 Total products: 10
⏱️  Execution time: 0ms

📝 Example 4: Multi-Stage Aggregation

✅ Function saved
📊 Category analysis (2 categories):
   Electronics:
      Products: 6 | Stock: 232 | Avg Rating: ⭐4.52
   Furniture:
      Products: 4 | Stock: 43 | Avg Rating: ⭐4.26

⏱️  Total execution time: 0ms
📈 Pipeline stages:

📝 Example 5: Project Specific Fields

✅ Function saved
📊 Product summaries (10 items, showing first 3):
   1. Webcam HD - $119 (⭐4.5)
   2. USB-C Cable - $19 (⭐4.3)
   3. Monitor 27" - $399 (⭐4.6)
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced function examples completed!
🚀 ekoDB AI Functions Example

⚠️  Note: These examples require AI API credentials (OpenAI, etc.)

📋 Setting up test data...
✅ Created 3 articles

📝 Example 1: Simple Chat Completion

✅ Chat function saved
🤖 AI Response:
   1. High Precision: Vector data is more accurate compared to raster data, hence providing better precision.

2. Compact Data Structure: Vector databases typically involve smaller file sizes as they only contain the geographic coordinates of the data points.

3. Scalability: Vector data can be scaled up and down without any loss of quality. This is useful in detailed map viewing.

4. Advanced Analysis: Vector databases enable advanced spatial analysis and operations like network analysis, proximity analysis, topological modeling etc.

5. Clean Aesthetics: Vector data often have sharp, clear lines, and solid colors representing different areas, providing clean aesthetics.

6. Conserves Attributes: Each object can have various attributes linked with it, which can be stored and manipulated in the database easily.

Keep in mind that the suitability of vector databases can greatly depend on the specific use-cases.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding function saved
📊 Generated embeddings for 3 articles
   1. "Draft Article" - 1536D vector
   2. "Advanced Query Patterns" - 1536D vector
   3. "Getting Started with ekoDB" - 1536D vector
⏱️  Execution time: 0ms

📝 Example 3: List All Articles

✅ Function saved
📊 Found 3 articles
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All AI examples completed!
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_js (f_02M4j0OWIByScjWtthkymuIi3cSaoHYJDhRFOoqYY33QCwV4nxsXaAjpL-rKTG9X_L7UvNKOE8xWoaMmuKcw)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "address": {
          "zipcode": "92998-3874",
          "geo": {
            "lng": "81.1496",
            "lat": "-37.3159"
          },
          "city": "Gwenborough",
          "street": "Kulas Light",
          "suite": "Apt. 556"
        },
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets"
        },
        "username": "Bret",
        "website": "hildegard.org",
        "name": "Leanne Graham",
        "email": "Sincere@april.biz",
        "phone": "1-770-736-8031 x56442",
        "id": 1
      }
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
Response time: 41ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
🚀 ekoDB JavaScript Client - Native SWR Function Examples

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
✓ Created native SWR script: github_user_native (LH-oW7q9FHtL2UrWx5wm1PP7-9c_6BVUJtmHTftbBW2dtfizv-8jE2abdic7kk7PJcU78bynsVdfn4XNuZ0ekg)

First call (cache miss - will fetch from GitHub API):
  Response time: 16ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 17ms
  Speedup: 0.9x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (nS5ZNwkLw1H9FT_Dye6zYSHs9hlMiXOp1BZWQVCNrssusRw9LgiJ61Su0ZnxfNs57McJMGhW6RJyKirgSxl2SQ)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (fg8waqiubvLSCyYMxaoV-VTLSpPgEJw8NkctKBS9x-Cn5Kaq8WnUbN0JTsr32vOW06F03AwSKnScDI0N6BUowg)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (8bzrc9wO5-ykR3bg6TtizyO3irDakBOzWZJgFhZwxyTDqS25PvENkXQs8T4Vg4X27oYhtD9sx_c57j0k7obtHA)
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
✓ Deleted 4 test functions

✅ All examples completed!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: R8PFo0xSv3m8_urcvduRh6ziap9WG2NO9ymScCDBTK16C4hkMQbP6dwVAOhx3uxvDpQ2wkX6rLKSzgcwo9NAQw

=== Sending Chat Message ===
Message ID: WoODPOSgT21Qqubi2XOj-HGpkocB8o1m0HTl8I_vagZfyeAsf4XsdPuJj41Z8TjziBAoq0eCwcwbasj45NzzlA

=== AI Response ===
Here are the available products and their prices:

1. **ekoDB**: $99
2. **ekoDB Cloud**: $499
3. **ekoDB Pro**: $299

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    id: 'H0GtOZlEmuDRb-heroDJdQOqYIZZOA2ljpbYg5UqEskSPG4m0p6jcrYHdjqTaI_VbodytYPVMkMjwR9zLibodg',
    description: 'A high-performance database product with AI capabilities',
    price: 99,
    name: 'ekoDB'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB Cloud',
    description: 'Fully managed cloud database service product',
    id: 'iLnoJc6Cid3_s9G7LRb2boMtBrjWMnXKsScuCGKwDSbCY5DaLiywbeUvX1NUzb28AwtI4V_PPpHjpvo_jmYJDA',
    price: 499
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    price: 299,
    description: 'Enterprise edition product with advanced features',
    id: 'yRJUBjIeFKZ0NPzStBRjYQ445QWPvuCoCVMP88ueNteYUelm0Abtxiv5HNw9oYh4-OwMUI54D3QHMRd6DwaRJw',
    name: 'ekoDB Pro'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2890ms

=== Token Usage ===
Prompt tokens: 2772
Completion tokens: 46
Total tokens: 2818

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: -7NNuuibheeS326TGqSY3hxq2RfGZWVDZ1Uo7XFZak7iairGNF-3KIS3cd6E9evWA31gC4vvF0SwahYCWk1jzg

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you need more details or have other questions, feel free to ask!

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you have any more questions or need further information, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: nlY2J_jvrqtaqZlw3-XboGiZdo7_4DTVrN4gXDUmPHR4XBSgjH9lPBEokhnd98Kq7fZ-tF4d5BC_A2CpyB0kMg
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 6

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 5

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: xLpy2pVgsLRqniSAV3FL1rvE4zxZiB7Qxrtob3cCTxLhc498U44U6NJEPWfMopCHLOG81q2V3TUWwO6d4DSdCg

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99

If you need more information or additional products, feel free to ask!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 931xtxZmlofkEZZppdxdREePZFfBJ2cFMsw5u_HkV2bVK9luEUFLw5jYNSKsUQv7vzhhjnLtSOMAZ4SJ54Uvpg
  Parent: xLpy2pVgsLRqniSAV3FL1rvE4zxZiB7Qxrtob3cCTxLhc498U44U6NJEPWfMopCHLOG81q2V3TUWwO6d4DSdCg

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: 931xtxZmlofkEZZppdxdREePZFfBJ2cFMsw5u_HkV2bVK9luEUFLw5jYNSKsUQv7vzhhjnLtSOMAZ4SJ54Uvpg (Untitled)
  Session 2: xLpy2pVgsLRqniSAV3FL1rvE4zxZiB7Qxrtob3cCTxLhc498U44U6NJEPWfMopCHLOG81q2V3TUWwO6d4DSdCg (Untitled)
  Session 3: nlY2J_jvrqtaqZlw3-XboGiZdo7_4DTVrN4gXDUmPHR4XBSgjH9lPBEokhnd98Kq7fZ-tF4d5BC_A2CpyB0kMg (Untitled)
  Session 4: R8PFo0xSv3m8_urcvduRh6ziap9WG2NO9ymScCDBTK16C4hkMQbP6dwVAOhx3uxvDpQ2wkX6rLKSzgcwo9NAQw (Untitled)
  Session 5: p0mgDCG_plaRA80Tf7sjvYBIdjXQKQNMCIEH75Q2PA2b6hSTTX7RirdluinPQvpVl8pr3Dq9wfqntEhfBnnTeQ (Untitled)
  Session 6: YlC_6OxGmSQWIjGWb6cWzmAMweRr1aMPVgjWWn61Cz8oZ5Bnh5sEfznNjX9i2zMfsBW4DMxA1kqMemiPPQfRUw (Untitled)
  Session 7: ZzXwZ62zSy29eDryiiPTrlNcdexdA_lC-Fhi7IgYMecSrKKCC9TYiWVDZJqTCU4fw51UfAH6VwEbzSnC1cwVBg (Untitled)
  Session 8: L-i-wdaN13ZIEgsA4f8TODZpG1VETNbwlBSN6zJqZJVegkZFHF3AHSw9rB7i33x8SGrwhu7hpe_aLQIzhBkqvg (Untitled)
  Session 9: Tkc91CJkSRbOsOa29gqzOm---g9q2D8jvab9QJB1X4xAtBbuQPLwHauSVSX5RtAiwW03AI_6rkXizUzfnAlwsA (Untitled)
  Session 10: wDHGhDHQow440ENunyApPaYlggdEgVQBOK3TDrLOor-A8e6BtjXPOLXso8hn6qurUBaJWROUAZ5PGcQUdXcNCw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 931xtxZmlofkEZZppdxdREePZFfBJ2cFMsw5u_HkV2bVK9luEUFLw5jYNSKsUQv7vzhhjnLtSOMAZ4SJ54Uvpg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
✅ [32mJavaScript client examples complete![0m
