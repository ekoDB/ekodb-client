make test-examples
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("B_EPAEc9dv8qobHZwQLML7Clzu202Ep2Fk8H-_U91IiYy5wXoS1gpYN_UtWWnBvnV58vW8D2JL-6rQdYZ5AUVw")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("B_EPAEc9dv8qobHZwQLML7Clzu202Ep2Fk8H-_U91IiYy5wXoS1gpYN_UtWWnBvnV58vW8D2JL-6rQdYZ5AUVw"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("B_EPAEc9dv8qobHZwQLML7Clzu202Ep2Fk8H-_U91IiYy5wXoS1gpYN_UtWWnBvnV58vW8D2JL-6rQdYZ5AUVw"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("B_EPAEc9dv8qobHZwQLML7Clzu202Ep2Fk8H-_U91IiYy5wXoS1gpYN_UtWWnBvnV58vW8D2JL-6rQdYZ5AUVw"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "FeqA3vaSCqzaM09W3ES2j5OE8ThY6bgfDXBhYB9paIu6aM8tKHwgA8kvLgSA_dvnIjkt2zrbSb_ciW8KsFJ-KA"

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1776011965370548000",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "FeqA3vaSCqzaM09W3ES2j5OE8ThY6bgfDXBhYB9paIu6aM8tKHwgA8kvLgSA_dvnIjkt2zrbSb_ciW8KsFJ-KA",
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
✓ Retrieved 1 record(s) via WebSocket

✓ WebSocket example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/batch_operations`
✓ Authentication successful

=== Batch Insert ===
✓ Batch inserted 5 records
✓ Verified: Found 5 total records in collection

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
✓ Batch updated 3 records
✓ Verified: Record updated with status="active"

=== Batch Delete ===
✓ Batch deleted 3 records
✓ Verified: Records successfully deleted (not found)

✓ All batch operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/kv_operations`
✓ Authentication successful

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: Object {"userId": Number(123), "username": String("john_doe")}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: Object {"name": String("Product 1"), "price": Number(29.99)}
cache:product:2: Object {"name": String("Product 2"), "price": Number(39.989999999999995)}
cache:product:3: Object {"name": String("Product 3"), "price": Number(49.989999999999995)}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/collection_management`
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: "E17vIh66DxGTIz5PzPOkNesnZzWqgyuT0heVNoiQWkvWYk8srusiivnyWHwXm-NdgKgaVsi_hapJZsIVd40sLA"

=== List Collections ===
Total collections: 5
Sample collections: ["test_collection", "demo_collection", "batch_users", "audit__ek0_testing", "websocket_test"]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/document_ttl`
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "9WAQpiNiKEVSWEd_42B7pnB_PrRrJkkyOO21CFkt-JXs5qYXV-rX8IJw5H9A1w6YVU52CtML537z1rd31gG7yg"

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: "59PETx_BrHf6mZIJzNposW2akCXVfTZ5gnhtzP0-uhM7O9-voFiAXH6p4TdC0BXtSkoGedkUv3JtHfikEbH10Q"

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: "s5IcEmy3VAvvjvS-OU1V_YDbZ1zxFHRo5GIecHedqhEFdqCDQunjb-ZI54bmqk7hyxaMHGS1lIouLRGcO5QnvQ"

=== Query Documents ===
✓ Found 3 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/websocket_ttl`
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: String("aIBjX1e06OqvLPkvmyRzF9PDHwxZ_kkL4eLlQKSc8H96Ky8kZ2PEI-xgw-w9W8-paYBZuUN54ssaMaAyiJehiA")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  id: "aIBjX1e06OqvLPkvmyRzF9PDHwxZ_kkL4eLlQKSc8H96Ky8kZ2PEI-xgw-w9W8-paYBZuUN54ssaMaAyiJehiA"
  name: {"type":"String","value":"WebSocket TTL Test"}
  ttl: "2026-04-12T17:40:01.139819Z"
  value: {"type":"Integer","value":42}

✓ WebSocket TTL example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Functions Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: n7LxJiOxkTVMfFBVg4POyup1AngKCvwrw9eWEQVnj9ECIPxhXFBvrpNdzsOguUrqU4vaMg3Ikr6ldPSNcEc7gQ
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: "1yyjLfuf1g6tNI_Gv3AeXnFHe54uLjxbnnvlqjeucE_fDIKhTh9OE6WNowWUR4KkQTW_c4PPCtfToXgygRzhxA"
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Function saved: xBLLQqDCabpyyOfGLe6XCewZ1R9qGSQ2MELvBto7eXOybXmNwputmFDcRJ-XMXspgIy0bs22h7WkBpk1ATXt-w
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: J_6BPURHQnheB1hpnUrBg662335ai6OvybQ24fHr8b7nef3yQBGb5WRUboOu43Nb-NXFCccsUfJcz4_s8pBDnQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":50.0,"count":5,"max_score":90,"status":"inactive"}
   {"avg_score":60.0,"count":5,"max_score":100,"status":"active"}

📝 Example 4: Function Management

📋 Total scripts: 4
🔍 Retrieved script: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/transactions`
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 9PAOhHyfpU19UB47Lz-ux3DgDhDxZivAEAbkeOWjdAayZr9F1h_3Oumr6ynCBFiQ8MCf6XLTINRywaAEua5sog
Created Bob: $500 - ID: 8SalG89ru5roPHxkuioevHKhr4jJv-ZlthY8OTfDi9MTd8kDOf2cid8O0lRYPCm-R5S5HmtvuFxtqlWdo2sgmQ

=== Example 1: Begin Transaction ===
Transaction ID: 75089e43-19fd-4dac-8032-ce847d9ea485

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: {"type":"Integer","value":800}
Bob: {"type":"Integer","value":700}

=== Example 5: Rollback ===
New transaction: 578f6b7c-6554-4e60-9eff-a20705bf4a50
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"type":"Integer","value":700}

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✅ [32mRust direct examples complete![0m
🛠️  [36mBuilding client library...[0m
cargo build -p ekodb_client
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
✅ [32mClient build complete![0m
🧪 [36mRunning Rust client library examples...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record({"id": String("WBU-RnVXW9UrrlvKE7xIhkChlAaRAfrPwmsDqY4L8CZNR_Ap3fMpNszIXhrxK7w_umarWStlTGX9qjc8QB-Yag")})

=== Find by ID ===
Found: Record({"categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "metadata": Object({"value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})}), "type": String("Object")}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "id": String("WBU-RnVXW9UrrlvKE7xIhkChlAaRAfrPwmsDqY4L8CZNR_Ap3fMpNszIXhrxK7w_umarWStlTGX9qjc8QB-Yag"), "created_at": Object({"type": String("DateTime"), "value": String("2026-04-12T16:40:20.985048Z")}), "value": Object({"type": String("Integer"), "value": Integer(42)}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "name": Object({"type": String("String"), "value": String("Test Record")}), "price": Object({"value": Float(99.99), "type": String("Float")}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-04-12T16:40:20.985048Z")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 5 dims
  categories (Set): 2 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record({"active": Object({"type": String("Boolean"), "value": Boolean(true)}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "name": Object({"type": String("String"), "value": String("Test Record")}), "value": Object({"value": Integer(42), "type": String("Integer")}), "created_at": Object({"type": String("DateTime"), "value": String("2026-04-12T16:40:20.985048Z")}), "metadata": Object({"value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})}), "type": String("Object")}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "id": String("WBU-RnVXW9UrrlvKE7xIhkChlAaRAfrPwmsDqY4L8CZNR_Ap3fMpNszIXhrxK7w_umarWStlTGX9qjc8QB-Yag"), "price": Object({"type": String("Float"), "value": Float(99.99)}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])})})]

=== Update Document ===
Updated: Record({"embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "created_at": Object({"type": String("DateTime"), "value": String("2026-04-12T16:40:20.985048Z")}), "id": String("WBU-RnVXW9UrrlvKE7xIhkChlAaRAfrPwmsDqY4L8CZNR_Ap3fMpNszIXhrxK7w_umarWStlTGX9qjc8QB-Yag"), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "metadata": Object({"value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")}), "type": String("Object")}), "name": Object({"value": String("Updated Record"), "type": String("String")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "value": Object({"value": Integer(100), "type": String("Integer")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "price": Object({"value": Float(99.99), "type": String("Float")}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")})})

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: 1t8gpX_4LuTfDTTZ8YYnh5EFRoLlqe-VwjD-1TofVeqS8hRP4lViLZqS4CQVJuvRgjuYM2EVc_ntzDb0DfqI0g

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  active: true
  name: "WebSocket Test Record"
  value: 42
  id: "1t8gpX_4LuTfDTTZ8YYnh5EFRoLlqe-VwjD-1TofVeqS8hRP4lViLZqS4CQVJuvRgjuYM2EVc_ntzDb0DfqI0g"

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_batch_operations`
✓ Client created

=== Batch Insert (via multiple inserts) ===
✓ Inserted 5 records
✓ Verified: Found 5 total records in collection

=== Update Records ===
✓ Updated 3 records

=== Delete Records ===
✓ Deleted 3 records

=== Cleanup ===
✓ Deleted collection

✓ All batch operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/examples/client_collection_management`
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "wczC53q7fZ_4Fl5DPjX4LxaLhWmyVpzxqwqiwWBAAVeq-GPxAd3iUYYhYB9HoOaaXATgUAi4R17RCQ0f5QfkWA"

=== List Collections ===
Total collections: 10
Sample collections: ["test_collection", "users", "client_collection_management_rust", "test_accounts", "ws_ttl_test"]

=== Count Documents ===
Document count: 1

=== Check Collection Exists ===
Collection exists: true

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/examples/client_kv_operations`
✓ Client created

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: Object {"userId": Number(123), "username": String("john_doe")}

=== KV Batch Set ===
✓ Batch set 3 keys
  cache:product:1: success
  cache:product:2: success
  cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  cache:product:1: Record({"price": Float(29.99), "name": String("Product 1")})
  cache:product:2: Record({"price": Float(39.989999999999995), "name": String("Product 2")})
  cache:product:3: Record({"price": Float(49.989999999999995), "name": String("Product 3")})

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 4

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== KV Batch Delete ===
✓ Batch deleted 3 keys
  cache:product:1: deleted
  cache:product:2: deleted
  cache:product:3: deleted

✓ All KV operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_transactions`
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: Vm56jVMVV1U846G-hfGGfPqVIIWfMiiugNW3o1VuyNKVRX-h8PtyhjjQ53P6x-QTIvu8LvOSGMNTVAsNro-kDg
Created Bob: $500 - ID: l9eSVectBBITeWAKJVpp8h1sppiMa4qOf_nmu7pd4PmnQITEjb4KoRPpXW4vYof4scVr7jcwl76hwpAxbyRy1w

=== Example 1: Begin Transaction ===
Transaction ID: cd64d522-3950-44d5-83c5-890b29d75ec6

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: a28d39cd-6dbb-49de-b077-10dca409b65d
Status before rollback: "Active"
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_query_builder`
=== ekoDB Query Builder Example ===

=== Inserting Sample Data ===
✓ Inserted 5 users

=== Simple Equality Query ===
✓ Found 3 active users
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Alice")}))
  - Some(Object({"type": String("String"), "value": String("David")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"value": String("Eve"), "type": String("String")}))
  - Some(Object({"value": String("David"), "type": String("String")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"value": String("Eve"), "type": String("String")}))
  - Some(Object({"value": String("David"), "type": String("String")}))

=== NOT IN Operator ===
✓ Found 4 users not inactive

=== String Pattern Matching ===
✓ Found 5 users with @example.com email

=== Regex Query ===
✓ Found 0 users with names starting with A-C

=== Complex Query (active AND age >= 28 AND score > 1500) ===
✓ Found 2 users matching all conditions
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"type": String("String"), "value": String("David")}))

=== OR Query ===
✓ Found 2 users with age < 28 OR age > 32
  - Some(Object({"type": String("String"), "value": String("Charlie")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))

=== Sorted Query (by score descending) ===
✓ Top 3 users by score:
  1. Some(Object({"value": String("Bob"), "type": String("String")}))
  2. Some(Object({"value": String("David"), "type": String("String")}))
  3. Some(Object({"type": String("String"), "value": String("Alice")}))

=== Pagination (page 2, size 2) ===
✓ Page 2 results:
  - Some(Object({"type": String("String"), "value": String("Charlie")}))
  - Some(Object({"type": String("String"), "value": String("David")}))

=== NOT Operator ===
✓ Found 4 users NOT inactive

=== Complex Chained Query ===
✓ Found 3 users with all conditions

=== Cleanup ===
✓ Deleted collection

✓ All query builder operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_search`
=== ekoDB Search Example ===

=== Inserting Sample Documents ===
✓ Inserted 5 sample documents

=== Basic Text Search ===
✓ Found 3 results for 'programming'
  1. Score: 19.8000 - Some(Object {"type": String("String"), "value": String("Rust Programming")})
  2. Score: 6.6000 - Some(Object {"type": String("String"), "value": String("Python for Data Science")})
  3. Score: 6.6000 - Some(Object {"type": String("String"), "value": String("JavaScript Web Development")})

=== Fuzzy Search ===
✓ Found 3 results for 'progamming' (typo)
  1. Score: 1.6500 - Some(Object {"type": String("String"), "value": String("Rust Programming")})
  2. Score: 0.5500 - Some(Object {"type": String("String"), "value": String("Python for Data Science")})
  3. Score: 0.5500 - Some(Object {"type": String("String"), "value": String("JavaScript Web Development")})

=== Field-Specific Search ===
✓ Found 4 results in title/description
  1. Score: 4.0000
     Title: Some(Object {"type": String("String"), "value": String("Machine Learning Basics")})
     Matched: ["description", "title", "title.value", "description.value"]
  2. Score: 2.0000
     Title: Some(Object {"type": String("String"), "value": String("Python for Data Science")})
     Matched: ["description.value", "description"]
  3. Score: 1.0000
     Title: Some(Object {"type": String("String"), "value": String("Rust Programming")})
     Matched: ["description", "description.value"]
  4. Score: 1.0000
     Title: Some(Object {"type": String("String"), "value": String("Database Design")})
     Matched: ["description", "description.value"]

=== Weighted Search ===
✓ Found 2 results with field weights
  1. Score: 23.1000 - Some(Object {"type": String("String"), "value": String("Python for Data Science")})
  2. Score: 3.3000 - Some(Object {"type": String("String"), "value": String("Machine Learning Basics")})

=== Advanced Search Options ===
✓ Found 1 results with stemming
  1. Score: 19.8000 - Some(Object {"type": String("String"), "value": String("Database Design")})

=== Search with Limit ===
✓ Limited to 2 results (requested 2)
  1. Score: 19.8000 - Some(Object {"type": String("String"), "value": String("Rust Programming")})
  2. Score: 6.6000 - Some(Object {"type": String("String"), "value": String("Python for Data Science")})

=== Cleanup ===
✓ Deleted collection

✓ All search operations completed successfully
Execution time: 2ms
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_schema`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("dSpFfP3KZducVCx99GjWmSJHQLPPcJRYQoH87dP0nbjxzdmPQlrEjDXcgcRN68l4k5PGOph6YIQM4cbeu1Ju9Q"))
✓ Inserted user 2: Some(String("PVNxlTU5MJSfo0cBtso8rB1spEDQbfWK0Enxne_LoC9fIgpeigEVfpZaPTwTJ4xfLRNZaZ8Fsz_s4i39vbJehw"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - email: String
    (required)
  - title: String
    (required)
  - status: String
  - age: Integer

=== Listing Collections ===
✓ Total collections: 10
  Sample: ["test_collection", "users", "test_accounts", "ws_ttl_test", "ttl_cache"]

=== Cleanup ===
✓ Deleted collection

✓ All schema management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_joins`
=== ekoDB Rust Client - Join Operations Example ===

=== Setting up sample data ===
✓ Sample data created

=== Example 1: Single collection join (users with departments) ===
✓ Found 2 users with department data
  - Bob Smith: Sales
  - Alice Johnson: Engineering

=== Example 2: Join with filtering ===
✓ Found 1 users in Engineering
  - Alice Johnson: Building A

=== Example 3: Join with user profiles ===
✓ Found 2 users with profile data
  - Bob Smith: Sales Manager
  - Alice Johnson: Senior Software Engineer

=== Example 4: Join orders with user data ===
✓ Found 2 completed orders
  - Mouse ($25) by Alice Johnson
  - Laptop ($1200) by Alice Johnson

=== Example 5: Complex join with multiple conditions ===
✓ Found 2 users with example.com emails
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✓ Deleted test collections

✓ Join operations example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_document_ttl`
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "3MM-eKa5OoxnUzToLAPE8D-m4WAdOMNlz1HeCNRj27gr1gr9qb_n95M_EcjyG7FrMYIKK_W6yLdzhGdF2VDsVA"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("a13caH3j9NKX22xlA-bzyVeVq9TZ3zEWxrOCQDwYk8JPomjKO_LLeMr-r0ZOhlyQTxco3K43wop3uNj-8jzATg"))

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_websocket_ttl`
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Some(String("2Gy_u0sqGrHQnra0LuHzfuik2iLOpjBMWRzCncPZAuXiZdT3AKa9lYsNE1hH-4dLSjErtejeqM3D-zcOMC7vYg"))

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_edge_cache`
=== ekoDB as Edge Cache - Simple Example ===

Setting up edge cache collection...
✓ Cache entry created

Creating edge cache lookup script...
✓ Edge cache script created: 4i8vNmeri5V-WQ1eGJfk_NOuXnw1Mqq5jQmgp8IbROHSj73YYRkPOwBYLT98x_1WK6pjxLHo1MJCmNga0NnDCg

Call 1: Cache lookup
Response time: 14ms
Found 1 cached entries

Call 2: Cache lookup (connection warm)
Response time: 14ms
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_functions`
=== ekoDB Rust Client - Functions Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: 7veG8eUnJchEsatpACu1a4xb--ios-j_dtJ2Sv9Rs96WqMpSV1Gjedn8x1snJ_mzs4oX4tvkytl0lsKN13uXKg
📊 Found 20 active users

📝 Example 2: Parameterized Function

✅ Function saved: plpz-oVhbq0NmknVDLeJQV0gRojE3fAFaDSu39RIG2L35lwM1N7pPy0rHR_IZ1ZX37qcLWi2s2-yL1Vy3ao0TA
📊 Found 20 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: of8EdjA7Oj90Bqlz4MIEWkLTFAnMM7I-J2Zf-v-9V9mFqpLKODwTcpmTKYt6fZqngzzz27V0201k3fgEY3laEw
📊 Statistics: 2 groups

📝 Example 4: Function Management

📋 Total functions: 6
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.


✅ All examples completed!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_function_composition`
=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

✅ Saved reusable function: fetch_user
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: User 1
   Department: engineering

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 205.525792ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "id": 1,
    "name": "Leanne Graham",
    "email": "Sincere@april.biz",
    "phone": "1-770-736-8031 x56442",
    "website": "hildegard.org",
    "address": {
      "suite": "Apt. 5...

Second call (cache hit - from cache):
   ⏱️  Duration: 40.33475ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "website": "hildegard.org",
    "username": "Bret",
    "company": {
      "catchPhrase": "Multi-layered client-server neural-net",
      "bs": "harness real-time e-markets",
      ...
   🚀 Cache speedup: 5.1x faster!

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
   - fetch_enriched_user: Used in 50 workflows
   - get_engineering_user: Specific workflow


✅ All composition examples completed!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_functions_complete`
🚀 ekoDB Rust Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: qiqYQ3c6J01iF8czbbOrOT65JzUbFDRX4xyOv6yfCsZdIxY1_AEYO1AoUMrKNTQ-pC6PMYL-O8MjiSeU9bF1-w
📊 Found 2 product groups
   Record({"avg_price": Float(474.0), "count": Integer(2), "category": String("Furniture")})
   Record({"avg_price": Float(575.6666666666666), "count": Integer(3), "category": String("Electronics")})
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   Record({"count": Integer(3), "category": String("Electronics")})
   Record({"category": String("Furniture"), "count": Integer(2)})
⏱️  Execution time: 0ms

📝 Example 4: High Rating Products

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 5: function with Parameter Definition

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 6: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Function saved
📊 Pipeline executed 3 stages
⏱️  Total execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All complete script examples finished!

💡 This example demonstrates ekoDB's function system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Parameter definitions
   ✅ Function management (save, call, delete)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_functions_kv_wrapped`
🚀 ekoDB Rust KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: Some(String("8xGJ_kNoJzZrKGMfcoW7tfN2aRHWm98WhHdRKYbma6pF7-G1SRL7lbO68y5ErfhhSz116RBx-YH8rGozKMsvAA"))
✅ Inserted 2 products with wrapped types

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: 6mD8AB23ilNxUww-W8a0pmXzj3BaWH6uh04zVzpp65blxF8YF59WVJGKkkcTKtO5ArlgKbBifnxqNqrHreEteA
📊 function executed
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: Some(Object {"role": String("admin"), "userId": String("user_abc")})
🔍 Key exists: true
✅ Set cached data
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: 2ha1VBljuWtuf3WUhq0OzL0IjiVfeUmMPyd2OkU2_ZwXZ_x4S0Nnrts4S61DUwbYoJZVZpBk5js2VX3qZmGbqQ
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: jyBk0GqCTNLgNIWOZtSO6G-jGMaezYWBm4O3qGDjpu934zzp5ae1GDBcOdzoxgGTpOn5RHr7x3nZivx3fClrMQ
📊 Processed order with caching
⏱️  Stages executed: 1
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All KV & Wrapped Types examples completed!

💡 Key takeaways:
   ✅ Use FieldType variants for type-safe wrapped values
   ✅ FieldType::Decimal preserves precision (no floating point errors)
   ✅ KV store is great for caching and quick lookups
   ✅ KV operations work within scripts
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_rs (TQidszKU5KfFr8u39qwmwD-K0hsiNJTIfNjxqY9nvaJL2CcwuMm2wmEd0ehbcxNKOfnHx7W8mc7bwmlDN-qf5A)

Step 2: First call - Cache miss, fetches from API
Result: FunctionStats { input_count: 0, output_count: 1, execution_time_ms: 0, stages_executed: 2, stage_stats: [] }
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 42ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_swr_native`
=== ekoDB Native SWR Function ===

This example shows the new simplified SWR function that replaces
the manual FindById → If → HttpRequest → Insert pattern.

Example 1: Basic GitHub User Cache with Native SWR
─────────────────────────────────────────────────────

✓ Created native SWR script: github_user_native (Wuo3S4yrQ3z1nVheixxYllgw5L7eprTDvTplB0hpyLnAWsEtDhVavNLspH1JEyXwdxYO2IF-FuH_SUabox0zHQ)

First call (cache miss - will fetch from GitHub API):
  Response time: 129ms
  Records returned: 1
  ✓ Data fetched from API and cached with 15m TTL

Second call (cache hit - instant from KV store):
  Response time: 22ms
  Speedup: 5.9x faster
  ✓ Lightning fast cache hit


Example 2: SWR with Audit Trail Collection
─────────────────────────────────────────────────────

✓ Created SWR script with audit trail: product_swr_audit (rpW-bfKmPQMPfkz3XpsOoXLQhAqI0Vt-CLfGKMMp2aRx20ZsueefVJDK0QWSW3BTU8TFFgrVwDbZDYhDlNhuXA)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
─────────────────────────────────────────────────────

Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (B2Q_xmet2It9oQB3UqH5KDMsZLKxzIthQaOzZ_-2L4Ul9UCJpQh92O3A_OntAZHeJJ6fYJ1VpfOuC8vI8ZIWvw)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
─────────────────────────────────────────────────────

✓ Created dynamic TTL script: flexible_cache (jnU6DJrENsi3W5VzU6K6mw9NzarJPt7XS8rIV62BFLBSh2J0yt9iZNv_WC4WiC7jSKfXKC7OnxKooAhkzTrJuw)
  ✓ Cached with TTL: 5m (5 minutes)
  ✓ Cached with TTL: 1h (1 hour)
  ✓ Cached with TTL: 30s (30 seconds)

=== Key Benefits of Native SWR Function ===
✅ Simpler: One function instead of 4 (FindById → If → HttpRequest → Insert)
✅ Duration strings: Use '15m', '1h', '2h' instead of calculating seconds
✅ Built-in audit: Optional collection parameter for automatic logging
✅ Auto-enrichment: output_field populates params for downstream functions
✅ Transactional: Works correctly in both transactional and non-transactional contexts
✅ KV-optimized: Uses native KV store with proper TTL handling

=== Performance Comparison ===
Old pattern (manual):  ~4 function calls, complex script logic
New pattern (native):  ~1 function call, handled by server
Result: Simpler code, faster execution, easier maintenance

🧹 Cleaning up test data...
✓ Cleanup complete

=== Summary ===
The native SWR function provides a production-ready cache-aside pattern
with minimal code and maximum flexibility. Use it for:
  • API gateway caching
  • External API aggregation
  • Microservice response caching
  • Edge computing patterns
  • Real-time data enrichment pipelines

    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_functions_advanced`
🚀 ekoDB Rust Advanced Functions Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Function saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Function saved
📊 Category breakdown:
   Record({"category": String("Furniture"), "count": Integer(3), "avg_price": Float(365.6666666666667)})
   Record({"count": Integer(5), "avg_price": Float(367.0), "category": String("Electronics")})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_functions_ai`
🚀 ekoDB Rust AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer several benefits, including:

1. **Efficient Similarity Search**: They enable quick searches for similar items based on vector embeddings, enhancing recommendation systems and personalized experiences.

2. **Handling High-Dimensional Data**: Designed to manage and query high-dimensional data effectively, which is common in machine learning and natural language processing.

3. **Scalability**: Many vector databases are optimized for large datasets, allowing for scalable storage and retrieval as data grows.

4. **Support for Complex Queries**: They can handle complex queries that involve multi-dimensional data comparisons and gradient-based searches.

5. **Integration with ML Models**: Seamless integration with machine learning workflows, enabling direct querying of model outputs like embeddings.

6. **Real-Time Processing**: Facilitates real-time data analysis and retrieval, crucial for applications where speed is important.

7. **Rich Data Types**: Support for various data types beyond traditional structured data, accommodating unstructured data like images, texts, and audio. 

8. **Improved User Experience**: Faster response times and more relevant search results lead to better user satisfaction and engagement.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embed script saved
📊 Embedding generated
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All AI script examples finished!

💡 This example demonstrates:
   ✅ Chat completions with system/user messages
   ✅ Embedding generation for text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_functions_crud`
🚀 ekoDB Rust CRUD Functions Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Function saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Function saved
📊 User counts by status:
   Record({"count": Integer(7), "status": String("active")})
   Record({"count": Integer(3), "status": String("inactive")})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_functions_search`
🚀 ekoDB Rust Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Natural Language Processing (AI)
   2. Introduction to Machine Learning (AI)
   3. Getting Started with ekoDB (Database)
   4. Vector Databases Explained (Database)
   5. Database Design Principles (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   Record({"category": String("AI"), "count": Integer(2)})
   Record({"count": Integer(3), "category": String("Database")})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_chat_basic`
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: W_XoN7N1gKoGeMnrYxB84031ZTTwjzTNPGO90Hga4096w6VlR5nj8oHRtfCyQNWugwRiHGvLiRlRrPYKATXZoA

=== Sending Chat Message ===
Message ID: MBdsjgVb86R47A2JPm4cItygCXirHS1kc0vRmTXHusY-A4oWQnvtkMN21GDTtBPDP7k-hdyLXsj-sGVRlCc8Dg

=== AI Response ===
Response 1: ekoDB is a high-performance database that integrates intelligent caching, real-time capabilities, and AI functionalities. It is designed to enhance data management and retrieval processes.

### Key Features of ekoDB:
1. **AI Chat Integration**: The chat feature allows users to query the database using natural language, providing AI-powered responses with relevant context.
2. **Search Capabilities**:
   - **Full-text Search**: Enables users to perform extensive searches across textual content.
   - **Vector Search**: Utilizes vector embeddings to find semantically similar records, facilitating more context-aware querying.
   - **Hybrid Search**: Combines text and vector search approaches for optimal results, retrieving context automatically.

These features make ekoDB a versatile choice for applications requiring efficient data retrieval and intelligent interaction with stored information.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["content", "category", "title"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("vVm5a939xfzoG6S7cuYBS0AFAuT-v4z7rlUZPJGZ1iEHeZ-Vfgje456wI5xuKBsnAGwQtqEg50mr-eTJ2DU5Hw"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["category", "title", "content"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("-TZGba2pj5yGs4GDLfF3dRPPZlGWRl7ua3g1j7j3OSsBozUP1PkY9lNkOFin1uEzR-P5iacCGk2Hzv7mNFoBOg"), "title": String("Search Features")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "title"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("E6_ei1AhWBqdxOWBfV6y5SUaa2_VmaXLdZ_w1KgSH3dGL-ZDlulrCofpyzWud1ElI02peWYS-fh7jfoTdt1aeA"), "title": String("Introduction to ekoDB")}

Execution Time: 8620ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: owPgImkzS3ToTCg0y6eZ8jsVqlbitCINTqRxKAnoAk0jPVlCblg_6LGFB4r9S3nPp14NOA92g1_tzeqB7QZ2gg

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you need more information or have any other questions, feel free to ask!

✓ Second message sent
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you have any more questions or need further assistance, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: l5DeQ5mR60sriqk2G-P3hkq7CA5vp5JG9onoxl5Z3eTWo12j2q-ACHbJp8bD_xx8JeWcaeb6ILOzK82IBltWmA
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 0ZZzmUTPDZOFp4razygtycQFjXKop4-aPaMKK1bzpnKQ_UXhdMTBB1So4reoVYfYqSZXHnBjvWsejC2HlwDJNw
=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Product:** ekoDB
- **Description:** A high-performance database product with AI capabilities
- **Price:** $99 

If you need more information or additional products, just let me know!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: rYFHGh_C7hC5IApkDhiUPX21RC3eplI9K00KRJNlfDvaDLyo1NUr8c1lRArE2n2QSxQeyFz2iYmcx2vGfHf4kA
  Parent: 0ZZzmUTPDZOFp4razygtycQFjXKop4-aPaMKK1bzpnKQ_UXhdMTBB1So4reoVYfYqSZXHnBjvWsejC2HlwDJNw

=== Listing Sessions ===
✓ Found 4 sessions
  Session 1: rYFHGh_C7hC5IApkDhiUPX21RC3eplI9K00KRJNlfDvaDLyo1NUr8c1lRArE2n2QSxQeyFz2iYmcx2vGfHf4kA (Untitled)
  Session 2: 0ZZzmUTPDZOFp4razygtycQFjXKop4-aPaMKK1bzpnKQ_UXhdMTBB1So4reoVYfYqSZXHnBjvWsejC2HlwDJNw (Untitled)
  Session 3: l5DeQ5mR60sriqk2G-P3hkq7CA5vp5JG9onoxl5Z3eTWo12j2q-ACHbJp8bD_xx8JeWcaeb6ILOzK82IBltWmA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: rYFHGh_C7hC5IApkDhiUPX21RC3eplI9K00KRJNlfDvaDLyo1NUr8c1lRArE2n2QSxQeyFz2iYmcx2vGfHf4kA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_chat_models`
✓ Client created

=== Get All Chat Models ===
Available chat models by provider:

OpenAI models (122):
  - gpt-4-0613
  - gpt-4
  - gpt-3.5-turbo
  - gpt-5.4-mini
  - gpt-5.4
  - gpt-5.4-nano-2026-03-17
  - gpt-5.4-nano
  - gpt-5.4-mini-2026-03-17
  - davinci-002
  - babbage-002
  - gpt-3.5-turbo-instruct
  - gpt-3.5-turbo-instruct-0914
  - dall-e-3
  - dall-e-2
  - gpt-3.5-turbo-1106
  - tts-1-hd
  - tts-1-1106
  - tts-1-hd-1106
  - text-embedding-3-small
  - text-embedding-3-large
  - gpt-3.5-turbo-0125
  - gpt-4-turbo
  - gpt-4-turbo-2024-04-09
  - gpt-4o
  - gpt-4o-2024-05-13
  - gpt-4o-mini-2024-07-18
  - gpt-4o-mini
  - gpt-4o-2024-08-06
  - gpt-4o-audio-preview
  - gpt-4o-realtime-preview
  - omni-moderation-latest
  - omni-moderation-2024-09-26
  - gpt-4o-realtime-preview-2024-12-17
  - gpt-4o-audio-preview-2024-12-17
  - gpt-4o-mini-realtime-preview-2024-12-17
  - gpt-4o-mini-audio-preview-2024-12-17
  - o1-2024-12-17
  - o1
  - gpt-4o-mini-realtime-preview
  - gpt-4o-mini-audio-preview
  - o3-mini
  - o3-mini-2025-01-31
  - gpt-4o-2024-11-20
  - gpt-4o-mini-search-preview-2025-03-11
  - gpt-4o-mini-search-preview
  - gpt-4o-transcribe
  - gpt-4o-mini-transcribe
  - o1-pro-2025-03-19
  - o1-pro
  - gpt-4o-mini-tts
  - o3-2025-04-16
  - o4-mini-2025-04-16
  - o3
  - o4-mini
  - gpt-4.1-2025-04-14
  - gpt-4.1
  - gpt-4.1-mini-2025-04-14
  - gpt-4.1-mini
  - gpt-4.1-nano-2025-04-14
  - gpt-4.1-nano
  - gpt-image-1
  - gpt-4o-realtime-preview-2025-06-03
  - gpt-4o-audio-preview-2025-06-03
  - o4-mini-deep-research
  - gpt-4o-transcribe-diarize
  - o4-mini-deep-research-2025-06-26
  - gpt-5-chat-latest
  - gpt-5-2025-08-07
  - gpt-5
  - gpt-5-mini-2025-08-07
  - gpt-5-mini
  - gpt-5-nano-2025-08-07
  - gpt-5-nano
  - gpt-audio-2025-08-28
  - gpt-realtime
  - gpt-realtime-2025-08-28
  - gpt-audio
  - gpt-5-codex
  - gpt-image-1-mini
  - gpt-5-pro-2025-10-06
  - gpt-5-pro
  - gpt-audio-mini
  - gpt-audio-mini-2025-10-06
  - gpt-5-search-api
  - gpt-realtime-mini
  - gpt-realtime-mini-2025-10-06
  - sora-2
  - sora-2-pro
  - gpt-5-search-api-2025-10-14
  - gpt-5.1-chat-latest
  - gpt-5.1-2025-11-13
  - gpt-5.1
  - gpt-5.1-codex
  - gpt-5.1-codex-mini
  - gpt-5.1-codex-max
  - gpt-image-1.5
  - gpt-5.2-2025-12-11
  - gpt-5.2
  - gpt-5.2-pro-2025-12-11
  - gpt-5.2-pro
  - gpt-5.2-chat-latest
  - gpt-4o-mini-transcribe-2025-12-15
  - gpt-4o-mini-transcribe-2025-03-20
  - gpt-4o-mini-tts-2025-03-20
  - gpt-4o-mini-tts-2025-12-15
  - gpt-realtime-mini-2025-12-15
  - gpt-audio-mini-2025-12-15
  - chatgpt-image-latest
  - gpt-5.2-codex
  - gpt-5.3-codex
  - gpt-realtime-1.5
  - gpt-audio-1.5
  - gpt-4o-search-preview
  - gpt-4o-search-preview-2025-03-11
  - gpt-5.3-chat-latest
  - gpt-5.4-2026-03-05
  - gpt-5.4-pro
  - gpt-5.4-pro-2026-03-05
  - gpt-3.5-turbo-16k
  - tts-1
  - whisper-1
  - text-embedding-ada-002

Anthropic models (9):
  - claude-sonnet-4-6
  - claude-opus-4-6
  - claude-opus-4-5-20251101
  - claude-haiku-4-5-20251001
  - claude-sonnet-4-5-20250929
  - claude-opus-4-1-20250805
  - claude-opus-4-20250514
  - claude-sonnet-4-20250514
  - claude-3-haiku-20240307

Perplexity models (5):
  - sonar
  - sonar-pro
  - sonar-deep-research
  - sonar-reasoning
  - sonar-reasoning-pro

=== Get Models for Specific Provider ===

openai models (122):
  - gpt-4-0613
  - gpt-4
  - gpt-3.5-turbo
  - gpt-5.4-mini
  - gpt-5.4
  ... and 117 more

anthropic models (9):
  - claude-sonnet-4-6
  - claude-opus-4-6
  - claude-opus-4-5-20251101
  - claude-haiku-4-5-20251001
  - claude-sonnet-4-5-20250929
  ... and 4 more

perplexity models (5):
  - sonar
  - sonar-pro
  - sonar-deep-research
  - sonar-reasoning
  - sonar-reasoning-pro

✓ Chat Models API example complete
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_user_functions`
✓ Client created

=== Create User Function ===
Created user function with ID: zKnly3OJAc6pzTEH4wTWxUK6r0hh9zNERyYIB1nTZJfAnSkkKr7azujOSSJP1eO97Gu3MvFgNwTCHu8iipQBFQ

=== Get User Function ===
Retrieved: get_active_users_rs - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 13 user functions:
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_active_users_rs: Get Active Users
  - get_verified_user: Get verified and validated user
  - get_users_by_status: Get Users By Status
  - fetch_slim_user: Validate and slim down user
  - get_active_users_updated: Get Active Users (Updated)
  - get_high_scoring_active_users: Get High Scoring Active Users
  - get_active_users_paginated: Get Active Users (Paginated)
  - get_user_wrapper: Wrapper that calls fetch_user
  - swr_user: SWR pattern for user data (KV-based)
  - validate_user: Check if user exists
  - fetch_user: Fetch user by code
  - get_active_users: Get Active Users (Updated)

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_convenience_methods`
=== ekoDB Convenience Methods Example ===

=== Record Builder Pattern ===
✓ Created record with fluent builder: Record({"id": String("HEyvc2UGpI7RiYk96Qrmo0kTVPr9AIr_1eq29VCShP8uV0nW_muOP1_9MtSjM3dTchCoaOaJRO8Uh-cdP5s7vA")})

=== Upsert Operation ===
✓ First upsert (insert): Record({"id": String("20SeV5yhaeM9uG70SHXPY1DYHaJ9BqPzh6X5v3glqrswH6DhZATqDvQUTajpIR0z_xktZ5eTLGmiG3skvt1iSQ")})
✓ Second upsert (update): Record({"id": String("s5MAFjz6wD924njKBliggpJq5JhkCXx22tLUBL79RgiOiJzcfFZfY3WT6fFr7paYOngGbzArY0GDROtgQqB4sg")})

=== Find One Operation ===
✓ Found user by email: Record({"id": String("HEyvc2UGpI7RiYk96Qrmo0kTVPr9AIr_1eq29VCShP8uV0nW_muOP1_9MtSjM3dTchCoaOaJRO8Uh-cdP5s7vA"), "name": Object({"value": String("Alice Johnson"), "type": String("String")}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "age": Object({"value": Integer(28), "type": String("Integer")}), "email": Object({"value": String("alice@example.com"), "type": String("String")})})
✓ User not found (as expected)

=== Exists Check ===
✓ Record exists: true
✓ Fake record exists: false (should be false)

=== Pagination ===
✓ Inserted 25 records for pagination
✓ Page 1: 10 records (expected 10)
✓ Page 2: 10 records (expected 10)
✓ Page 3: 8 records (expected 7)

=== Cleanup ===
✓ Deleted collection

✅ All convenience methods demonstrated successfully!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/bypass_ripple_example`
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: Record({"id": String("aYDqEPI2_DXuvM11_c_EL5S4Vc_gfXgvEj8KtFi0xc7xIN5GOsc8z3dWVugrbq9xBU1c8-VmlWVKTH7ZP7Ib2w")})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: Record({"id": String("NxeFvT4fnVvD826JNutGOky9IvOJrgMklFxRfFcgKqED8NnCzlJygNHdzaWTqW8gLoAwihbSSxeZYKwBgNRyBQ")})

3. Update with bypass_ripple:
   Updated with bypass_ripple: Record({"price": Object({"value": Integer(150), "type": String("Integer")}), "id": String("aYDqEPI2_DXuvM11_c_EL5S4Vc_gfXgvEj8KtFi0xc7xIN5GOsc8z3dWVugrbq9xBU1c8-VmlWVKTH7ZP7Ib2w"), "name": Object({"value": String("Product 1"), "type": String("String")})})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: Record({"id": String("DrcoEQYdCEn69FUd777XEDfWMe2KsKMbp4iJBo5OQXLOoWn-_cWiICDCTKZrmX5IJAR9b_Ph6LM3g60XCzazAQ")})

✅ All bypass_ripple operations completed successfully!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/projection_example`
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ["email", "name", "id"]

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["user_role", "status", "age", "email", "created_at", "name", "avatar_url", "bio", "id"]

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)

Example 4: Query inactive users with profile fields
  Found 1 inactive users

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ["bio", "password", "status", "api_key", "id", "user_role", "age", "secret_token", "name", "created_at", "avatar_url", "email"]
  Projected query:
    - 3 fields per record
    - Fields: ["id", "email", "name"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
✅ [32mRust client examples complete![0m
✅ [32mAll Rust integration tests complete![0m
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'WdJ8uJ9wsbKEI8e6qVpy57wrEQEhSP2lacYKecdlVIwwWwBfpOpg1ZlKgGLyPVxaNEbpwWqCGJYN8Xki47vavA'}

=== Find by ID ===
Found: {'name': {'value': 'Test Record', 'type': 'String'}, 'id': 'WdJ8uJ9wsbKEI8e6qVpy57wrEQEhSP2lacYKecdlVIwwWwBfpOpg1ZlKgGLyPVxaNEbpwWqCGJYN8Xki47vavA', 'active': {'type': 'Boolean', 'value': True}, 'value': {'type': 'Integer', 'value': 42}}

=== Find with Query ===
Found documents: [{'value': {'type': 'Integer', 'value': 42}, 'id': 'WdJ8uJ9wsbKEI8e6qVpy57wrEQEhSP2lacYKecdlVIwwWwBfpOpg1ZlKgGLyPVxaNEbpwWqCGJYN8Xki47vavA', 'active': {'value': True, 'type': 'Boolean'}, 'name': {'type': 'String', 'value': 'Test Record'}}]

=== Update Document ===
Updated: {'id': 'WdJ8uJ9wsbKEI8e6qVpy57wrEQEhSP2lacYKecdlVIwwWwBfpOpg1ZlKgGLyPVxaNEbpwWqCGJYN8Xki47vavA', 'name': {'value': 'Updated Record', 'type': 'String'}, 'active': {'value': True, 'type': 'Boolean'}, 'value': {'value': 100, 'type': 'Integer'}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: MtXpLar30tru4Y354yBpOXUjpyCzRaHSnY8__zDq3z-SdZu4fg-jkmuyQAxukeHRhr6q1tC5bKDJeWdu3oDqHQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1298024898",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "FeqA3vaSCqzaM09W3ES2j5OE8ThY6bgfDXBhYB9paIu6aM8tKHwgA8kvLgSA_dvnIjkt2zrbSb_ciW8KsFJ-KA",
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
        "id": "MtXpLar30tru4Y354yBpOXUjpyCzRaHSnY8__zDq3z-SdZu4fg-jkmuyQAxukeHRhr6q1tC5bKDJeWdu3oDqHQ",
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
✓ Retrieved 2 record(s) via WebSocket

✓ WebSocket example completed successfully
🚀 ekoDB Functions Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: 30Rcfh0S1_tDA0REL_8TLAVAt7oSfmeZVVdJ1nfohg40imC_jkYgJhT8QWMYQIOOniiDI4zPsBUxArLpdK9qIA
📊 Found 15 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: XWbR9w_jYppWcRvXU6946Myj_78qIQVTxR-B9T-h7pRR8lVJuaPxc8GfX1nAcOIQmcXL0o3ePwDaW5riyNSUJg
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: 7IDPEuRq9zsjmbcxHq8cWH8S5s_fSYlB1d9Zh9xgcmW0tb4AJVWtpgrklJupWnq729DGJ6mdI7Y7Bfpz1Dq9hA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'max_score': 90, 'count': 15, 'status': 'inactive', 'avg_score': 50.0}
   {'count': 15, 'avg_score': 60.0, 'max_score': 100, 'status': 'active'}

📝 Example 4: Function Management

📋 Total scripts: 15
🔍 Retrieved script: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
=== Batch Operations (Direct HTTP) ===

✓ Authentication successful

=== Batch Insert ===
✓ Batch inserted 5 records
✓ Verified: Found 10 total records in collection

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
✓ Batch updated 3 records
✓ Verified: Record updated with status="active"

=== Batch Delete ===
✓ Batch deleted 3 records
✓ Verified: Records successfully deleted (not found)

✓ All batch operations completed successfully
=== Key-Value Operations (Direct HTTP) ===

✓ Authentication successful

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: {'userId': 123, 'username': 'john_doe'}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: {'name': 'Product 1', 'price': 29.99}
cache:product:2: {'price': 39.989999999999995, 'name': 'Product 2'}
cache:product:3: {'name': 'Product 3', 'price': 49.989999999999995}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: RNBRz3unpC8JLl6bZ9iPFN5RfyBEKH0RSlDnGu9itEs1tj0Dq_Y-JypIVj_G3fHtNoGrUorBHnlKxqvtLP1frg

=== List Collections ===
Total collections: 13
Sample collections: ['test_collection', 'users', 'demo_collection', 'chat_messages__ek0_testing', 'test_accounts']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Authentication successful

📋 Getting original configuration...
   Original durable_operations: True


============================================================
🔥 TEST 1: Original Config (durable=True)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: RikEQ1EPWT-w_ci_koQ9NgOg6PvyA0H0U3_pqm24s3rBCv1AXb2I-QV84Rs-j92u0IYrDtcG2tRp7yyhuYzVOg
Created Bob: $500 - ID: pULGJIiTdSocS4mtO8ieqofOVAclZs7qMJj3riTBGQy1y5StiPWz8N0IkviXE4UfddyzrBgeNAUWUOcheEOg6g

=== Example 1: Begin Transaction ===
Transaction ID: 573ce337-76d3-4b81-9909-eb0710ae248d

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'type': 'Integer', 'value': 800}
Bob: ${'type': 'Integer', 'value': 700}

=== Example 5: Rollback ===
New transaction: bfd48f1d-3d2d-4ae8-b5b1-a9f44573e28f
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: io6bd4_UtScng2ZDToJ0j8LatcA9IwztCBZrj6Xz9q9NTMGmxM9UKB5VPBNppvKXIMX7vosNx_VHWEmIGz0XTw
Created Bob: $500 - ID: Ag8SajFYgBLlKy1eyj6wxy34pTu10Z5O7MAY2_SeI_IBGAnnVak_pM55Fi4pBHc6G_KtbM5KoqLskp8MGMeKpw

=== Example 1: Begin Transaction ===
Transaction ID: 6baafb9e-0659-40e8-87d2-13e086e2e1d5

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'type': 'Integer', 'value': 800}
Bob: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ Deleted test accounts


🔄 Switching to NON-DURABLE mode...
   ✓ Config updated: durable_operations=false


============================================================
🔥 TEST 2: Non-Durable Mode (durable=false)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: gqfq4CrqlEjK1AmCxbqdAFAKbw3PV7Iagx1ZtyKxqBf8g8L7ey-oPLLLKWiLSRfby4xT-kgAdXZwAl_oOUwNMg
Created Bob: $500 - ID: ODri0-rXExlsv_dPhDU-gb1QAT8seh072nnaBGLdxzteN8_yd_u2I9aP79Rzqq5ilbjP2yF-kVXUvtiBJafRRg

=== Example 1: Begin Transaction ===
Transaction ID: 9d1875a7-1554-4f20-8b2b-cc1b62e77bd8

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'value': 800, 'type': 'Integer'}
Bob: ${'value': 700, 'type': 'Integer'}

=== Example 5: Rollback ===
New transaction: bd2e684d-b05b-44a1-8ac3-c7b15d457205
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: oawEfYiNoO5-8xGSnBkj5IT4FUQTeTMEXP7AlRYSIALmNTItMKS5FpOTbxGiWCp5Hwc6yxTgvNtI6eIOenN5wg
Created Bob: $500 - ID: R4Ita1trYuP6HVMLIV2jsiXJXZVjb0fe5GrnC1Hch0RWgMuL0PGgyQr5b39XZOApzi4CWrwRl6-M0kLmogy6fg

=== Example 1: Begin Transaction ===
Transaction ID: cfba7079-cc81-4795-883a-58d2ace46b72

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'type': 'Integer', 'value': 800}
Bob: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ Deleted test accounts


🔄 Switching to DURABLE mode...
   ✓ Config updated: durable_operations=true


============================================================
🔥 TEST 3: Durable Mode (durable=true)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: TRIDZuUYexPqCevz68nATwBd9LpSYQ_7SjxnLuywubks2IehG8BhZotN0EjUT98QyVizMPRX90bHupMzezvECQ
Created Bob: $500 - ID: rg0JAeaD0LAjSlxQ-WEPJ-F29DYCpOrZNsSNr_LHcsQw3JBF0RmUtwCFQrmBC_XiM0x_ElAuoXpbAvz1lw8IhA

=== Example 1: Begin Transaction ===
Transaction ID: bf6ba4a5-d194-4a07-b8e9-9b730ed061fb

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'value': 800, 'type': 'Integer'}
Bob: ${'type': 'Integer', 'value': 700}

=== Example 5: Rollback ===
New transaction: 98231c42-b840-4644-973b-8355d5f47d06
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 24bvdZX94ItbiY-ommS-jQvIj9oC6h2IlU-aulfSaggBCBnpcqmDbjQyk3RZ9ZkBNYG_R2QITlUhlA_HMi_R_A
Created Bob: $500 - ID: pOJY-u0pGasNGIFWFcWYpKePaAYfUwsKyf78a2j6Y5WiDxSPbOXYYfl5I-fteqVIs-GvUc_tB_NGTdpNNR6lNA

=== Example 1: Begin Transaction ===
Transaction ID: bdbf7d12-b0f3-467c-a0d8-e3c40f5d6203

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'value': 800, 'type': 'Integer'}
Bob: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ Deleted test accounts


🔄 Restoring original configuration...
   ✓ Config restored: durable_operations=True


============================================================
✅ ALL TESTS PASSED - Transactions successful
============================================================

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
📝 function 1: Insert + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: I9TMs7eKhQHBDO-Wwzti7ms7EF1zZfEpHOMnIq5mBPy4XRr73gUzXb1Xf2-lMp300t_jU-xijs_K3HOp6IBUxA

2️⃣ Calling function (Insert + Verify)...
   ✅ function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: 44plh4LIt-WV_bc5uOw-l52qdAp4me4ZP9kPREdQ7GeBIt4WeMfM7aojc95C5ShE8QHHAhZpSuUPAakvPyl2WA
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'value': 'alice@example.com', 'type': 'String'}
   📋 Status: {'type': 'String', 'value': 'pending'}
   📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================
📝 function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: LmjF0RUfyhWd2xf3sNHt8rwtnCvXdvTMkiWL171oUEngoA9-is51ZGTml6jDIaFHN4NFu3K8Ccg42tzbVbl0rg

2️⃣ Calling function (Query + Update + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {'type': 'String', 'value': 'active'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 function 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: 17N2lnC2BGd3J3pIPdDqOC-LbdFLxhbKi9BNuR202ZgElNrcvCWB4DNBfXNSWeY2AYLIYJU_q86z2Iob6Eka_w

2️⃣ Calling function (Query + Update Credits + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'type': 'Integer', 'value': 0}
   📋 Status: {'type': 'String', 'value': 'active'}
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}

============================================================
📝 function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: QBwk1YCBFk5rfSpOxxIxemkW3okxpeo77ZczbrMBa_TVjWDnBarDr1OwEmUQoZecTjlHai0tNWwBP6gqBVkQOQ

2️⃣ Calling function (Query + Delete + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: I9TMs7eKhQHBDO-Wwzti...
   ✅ Deleted script: LmjF0RUfyhWd2xf3sNHt...
   ✅ Deleted script: 17N2lnC2BGd3J3pIPdDq...
   ✅ Deleted script: QBwk1YCBFk5rfSpOxxIx...
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
  Output: Document ID = NZZgZgQZvGHBWtjWF4dVZ7fD82iu00czC8oHCOW_f-aTG_CVDXcGHV7oG6ZjCZfOFGI1AAVCRuw4pNh9y4gadA
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(NZZgZgQZvGHBWtjWF4dVZ7fD82iu00czC8oHCOW_f-aTG_CVDXcGHV7oG6ZjCZfOFGI1AAVCRuw4pNh9y4gadA)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(NZZgZgQZvGHBWtjWF4dVZ7fD82iu00czC8oHCOW_f-aTG_CVDXcGHV7oG6ZjCZfOFGI1AAVCRuw4pNh9y4gadA)
  Output: Error (expected) - Find failed: Record not found
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
  Output: Document ID = erTs4ibLtrEo8Fg-Gblr_yjOZCzVHVPcMXefX-DX9YPrchQHdNgXoTUa8RQwGapV6JUNoeXGogFSpzYhJbeTWg
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(erTs4ibLtrEo8Fg-Gblr_yjOZCzVHVPcMXefX-DX9YPrchQHdNgXoTUa8RQwGapV6JUNoeXGogFSpzYhJbeTWg)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(erTs4ibLtrEo8Fg-Gblr_yjOZCzVHVPcMXefX-DX9YPrchQHdNgXoTUa8RQwGapV6JUNoeXGogFSpzYhJbeTWg)
  Output: Error (expected) - Find failed: Record not found
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
  • Queries correctly return None for expired documents
[34m
╔════════════════════════════════════════╗[0m
[34m║   ekoDB Python Examples Test Suite    ║[0m
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
=== Running simple_crud.py ===[0m
[32m✓ simple_crud.py completed successfully[0m
[34m
=== Running simple_websocket.py ===[0m
[32m✓ simple_websocket.py completed successfully[0m
[34m
=== Running http_functions.py ===[0m
[32m✓ http_functions.py completed successfully[0m
[34m
=== Running batch_operations.py ===[0m
[32m✓ batch_operations.py completed successfully[0m
[34m
=== Running kv_operations.py ===[0m
[32m✓ kv_operations.py completed successfully[0m
[34m
=== Running collection_management.py ===[0m
[32m✓ collection_management.py completed successfully[0m
[34m
=== Running transactions.py ===[0m
[32m✓ transactions.py completed successfully[0m
[34m
=== Running crud_functions.py ===[0m
[32m✓ crud_functions.py completed successfully[0m
[34m
=== Running document_ttl.py ===[0m
[32m✓ document_ttl.py completed successfully[0m
[34m
=== Running websocket_ttl.py ===[0m
[32m✓ websocket_ttl.py completed successfully[0m
[34m
╔════════════════════════════════════════╗[0m
[34m║           Test Summary                 ║[0m
[34m╚════════════════════════════════════════╝[0m
[34mTotal: 10[0m
[32mPassed: 10[0m
[32mFailed: 0[0m
✅ [32mPython direct examples complete![0m
🐍 [36mBuilding Python client package...[0m
🍹 Building a mixed python/rust project
🔗 Found pyo3 bindings with abi3 support
🐍 Found CPython 3.11 at /Library/Frameworks/Python.framework/Versions/3.11/bin/python3
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.14s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.16.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.16.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.16.0
    Uninstalling ekodb_client-0.16.0:
      Successfully uninstalled ekodb_client-0.16.0
Successfully installed ekodb-client-0.16.0
✅ [32mPython client package built and installed![0m
🧪 [36mRunning Python client library examples...[0m
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': 'UA_NbBobOS5BHsfC2sbG-ipxoqesxiB60dLE6iPdxONRdOZ9tpjJn8Ga1Z7hVVwQUC2M1PVUHRnLWJYutw5e9g'}

=== Find by ID ===
Found: {'price': {'type': 'Float', 'value': 99.99}, 'created_at': {'value': '2026-04-12T12:42:04.163113', 'type': 'String'}, 'value': {'type': 'Integer', 'value': 42}, 'metadata': {'type': 'Object', 'value': {'key': 'value', 'nested': {'deep': True}}}, 'id': 'UA_NbBobOS5BHsfC2sbG-ipxoqesxiB60dLE6iPdxONRdOZ9tpjJn8Ga1Z7hVVwQUC2M1PVUHRnLWJYutw5e9g', 'active': {'type': 'Boolean', 'value': True}, 'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}, 'name': {'type': 'String', 'value': 'Test Record'}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-04-12 12:42:04.163113
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'key': 'value', 'nested': {'deep': True}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'price': 99.99, 'created_at': '2026-04-12T12:42:04.163113', 'value': 42, 'metadata': {'key': 'value', 'nested': {'deep': True}}, 'id': 'UA_NbBobOS5BHsfC2sbG-ipxoqesxiB60dLE6iPdxONRdOZ9tpjJn8Ga1Z7hVVwQUC2M1PVUHRnLWJYutw5e9g', 'active': True, 'data': 'aGVsbG8gd29ybGQ=', 'name': 'Test Record', 'categories': ['electronics', 'computers'], 'tags': ['tag1', 'tag2', 'tag3'], 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'user_id': '550e8400-e29b-41d4-a716-446655440000'}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'active': {'type': 'Boolean', 'value': True}, 'metadata': {'type': 'Object', 'value': {'nested': {'deep': True}, 'key': 'value'}}, 'price': {'type': 'Float', 'value': 99.99}, 'id': 'UA_NbBobOS5BHsfC2sbG-ipxoqesxiB60dLE6iPdxONRdOZ9tpjJn8Ga1Z7hVVwQUC2M1PVUHRnLWJYutw5e9g', 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'created_at': {'value': '2026-04-12T12:42:04.163113', 'type': 'String'}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'name': {'value': 'Updated Record', 'type': 'String'}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}, 'value': {'type': 'Integer', 'value': 100}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: lpQbH6HueYa2A-Mu6jXgj9_h71jPDsq69AlGT3LIyCk3_m7-pknUf86zr60NmetENtxMddPyWzqDF8X6fbcavA

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
Collection created with first record: "H8MQ2ad7oR-tq0gyIC_l0hLmL2YlwvAUlcTycO-lljjULbhTJdKpKaPF3pMvrAoJKOJ0VH501njlMomNBsRNZg"

=== List Collections ===
Total collections: 12
Sample collections: ['test_collection', 'chat_messages__ek0_testing', 'test_accounts', 'ws_ttl_test', 'ttl_cache']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Client created

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: {'value': '{"userId":123,"username":"john_doe"}'}

=== KV Batch Set ===
✓ Batch set 3 keys
  cache:product:1: success
  cache:product:2: success
  cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  cache:product:1: {'name': 'Product 1', 'price': 29.99}
  cache:product:2: {'name': 'Product 2', 'price': 39.99}
  cache:product:3: {'name': 'Product 3', 'price': 49.99}

=== KV Exists ===
Key exists: True

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 9

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: False

=== KV Batch Delete ===
✓ Batch deleted 3 keys
  cache:product:1: deleted
  cache:product:2: deleted
  cache:product:3: deleted

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: J1WVBnMGF8AZKfh88xYTjoIWdZxml0_6mloMQsOAOss7MwlHdAjP7ZIJ85NNDkThBLA-GXEzGDDHaKN5TXRz7Q
Created Bob: $500 - ID: IOESA3wrBngUFyVFNjkq5eCKXXGHc5ind2okvx5rc_mvsqTwUg_6Zq2Rq108BK1VvhgdSE6aOhrXq4ViIJ6j7g

=== Example 1: Begin Transaction ===
Transaction ID: cdd27df7-9b82-49b9-8757-4fe1e84b4f0e

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: f2a84bf5-92c5-409b-becc-812e38a7de4b
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
Found 2 valid users

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
  1. Score: 25.740, Matched: name, email, name.value, email.value
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio.value, title, bio, title.value
  2. Score: 26.400, Matched: title, title.value, bio, bio.value
  3. Score: 26.400, Matched: title.value, title, bio.value, bio
  4. Score: 26.400, Matched: title, title.value, bio, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio.value, title.value, title, bio
  2. Score: 39.600, Matched: title, title.value, bio, bio.value
  3. Score: 39.600, Matched: bio.value, title, title.value, bio
  4. Score: 39.600, Matched: title.value, bio.value, title, bio

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.755
  2. Score: 0.743
  3. Score: 0.740

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.378, Matched: title, title.value, content, content.value
  2. Score: 1.370, Matched: content.value, content, title.value, title
  3. Score: 0.372, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: title, title.value

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
Schema fields: ['age', 'email', 'name', 'status']
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
  - Bob Smith: Sales
  - Alice Johnson: Engineering

2. Join with filtering:
Found 1 users in Engineering:
  - Alice Johnson: Building A

3. Join with user profiles:
Found 2 users with profile data:
  - Bob Smith: Sales Manager
  - Alice Johnson: Senior Software Engineer

4. Join orders with user data:
Found 2 completed orders:
  - Mouse ($25) by Alice Johnson
  - Laptop ($1200) by Alice Johnson

5. Complex join with multiple conditions:
Found 2 users with example.com emails:
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✅ Deleted test collections

✅ Join operations examples completed!
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: CtiMtUuArxr7MB2je5QcbmcuIl1OnSNYJbiYFuwExJp0fHKPMvrBuLNZO0CQAah-lsW3XxX-IyoK2hQoKrHPoA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 1nkuxhbE0Vih9w2kpHSTDIJWaXzjeSy5SpRpK7ufofp2mguP9a4-kie2g8OI21T2Co_XydQjFZVsriaTN7u8RQ

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
✓ Inserted document with TTL: 0MIvzroraxeiqB42D27y3ecEExA7UmfNY6H4AFsgQTgkjH4qul9Ji49I_u5rZZCb0vxqF3xaM_0VMfOWHU7OXA

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
=== ekoDB as Edge Cache - Simple Example ===

Creating edge cache function...
✓ Edge cache script created: 8IPVpceoHAT1dHqwvn7wA4zu02h63opz4R1HPSGOT5xNG4oiDtk_ww5RvfeXOgpC5OxqJ-Gc3bunV4cCwOeB-g

Call 1: Cache miss (fetches from API)
Response time: 560ms
Result: {
  "records": [
    {
      "value": {
        "current": {
          "interval": 900,
          "temperature_2m": 12.3,
          "time": "2026-04-12T16:30"
        },
        "current_units": {
          "interval": "seconds",
          "temperature_2m": "\u00b0C",
          "time": "iso8601"
        },
        "elevation": 32.0,
        "generationtime_ms": 0.025153160095214844,
        "latitude": 40.710335,
        "longitude": -73.99308,
        "timezone": "GMT",
        "timezone_abbreviation": "GMT",
        "utc_offset_seconds": 0
      }
    }
  ],
  "stats": {
    "execution_time_ms": 0,
    "input_count": 0,
    "output_count": 1,
    "stage_stats": [],
    "stages_executed": 2
  }
}

Call 2: Cache hit (served from ekoDB)
Response time: 40ms (14.0x faster!)
Result: {
  "records": [
    {
      "value": {
        "current": {
          "interval": 900,
          "temperature_2m": 12.3,
          "time": "2026-04-12T16:30"
        },
        "current_units": {
          "interval": "seconds",
          "temperature_2m": "\u00b0C",
          "time": "iso8601"
        },
        "elevation": 32.0,
        "generationtime_ms": 0.025153160095214844,
        "latitude": 40.710335,
        "longitude": -73.99308,
        "timezone": "GMT",
        "timezone_abbreviation": "GMT",
        "utc_offset_seconds": 0
      }
    }
  ],
  "stats": {
    "execution_time_ms": 0,
    "input_count": 0,
    "output_count": 1,
    "stage_stats": [],
    "stages_executed": 2
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

🚀 ekoDB Functions Example (Python)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: T1TcFzTOyzlWdvuZv30urBQfWeSrlpvz8fmzwVgL5Q5XTsig1qbCS9ug_-jds_AF8PJRo6KdD2qIomSEj0wYbA
📊 Found 10 active users

📝 Example 2: Parameterized Function

✅ Function saved: tAdW0sFeDwMIdF84pfRNBgqFcRWdvfUGWsNVvk8SsbYfosP1YELS8IbIInSNX02XmwpqGWOfUkZCDOvs4hcvsw
📊 Found 10 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: eVMMs_KOY_clv8BlCDpXnYsTMidI9v44w8CwnNm57Z74imIwuII4KyqlzOjz2sH83whcmjwVYgjmn92CkbhiHg
📊 Statistics: 2 groups
   {'avg_score': 50.0, 'count': 5, 'status': 'inactive'}

   {'avg_score': 60.0, 'count': 5, 'status': 'active'}

📝 Example 4: Function Management

📋 Total functions: 18
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed!
=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

✅ Saved reusable function: fetch_user
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"type": "String", "value": "User 1"}
   Department: {"type": "String", "value": "engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 38.7ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "address": {
                  "city": "Gwenborough",
                  "geo": {
                        "lat": "-37.3159",
                        "lng": "81.1496"
    ...

Second call (cache hit - from cache):
   ⏱️  Duration: 38.6ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "address": {
                  "city": "Gwenborough",
                  "geo": {
                        "lat": "-37.3159",
                        "lng": "81.1496"
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
🚀 ekoDB Python Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: 5I92eiN5y_1sRKHl0Qk_cZQlNuZbXbz45kSNoUFUQpqlcjobtFRqvqpu-Ise0KgUJxE5Rt-XUTDoLm9ol6EVGg
📊 Found 2 product groups
   {'avg_price': 474.0, 'category': 'Furniture', 'count': 2}
   {'avg_price': 575.6666666666666, 'category': 'Electronics', 'count': 3}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {'category': 'Electronics', 'count': 3}
   {'category': 'Furniture', 'count': 2}
⏱️  Execution time: 0ms

📝 Example 4: High Rating Products

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 5: function with Parameter Definition

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 6: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Function saved
📊 Pipeline executed 3 stages
⏱️  Total execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All complete script examples finished!

💡 This example demonstrates ekoDB's function system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Parameter definitions
   ✅ Function management (save, call, delete)
🚀 ekoDB Python KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: chJcpoVxHIzGAkaBXIitO1RLU_EjCD1XXL3uNTMmbV7k2D21LHGQvwg_Zyk3VTTVpL3x7J964EQF82l_2X-vaQ
✅ Inserted 2 products with wrapped types

📝 Example 2: Querying and Extracting Wrapped Types

📊 Found 2 products
   • Laptop Pro
   • Wireless Mouse

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {'value': '{"role":"admin","userId":"user_abc"}'}
🔍 Key exists: True
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 5: Combined Wrapped Types + KV Usage

✅ Inserted order: paFyhS-TF4wFfzm8Bzv0TX79UtzR3lR1oAJwAg_gt-jTMh10pzm9knOdaedE7FxAtqXyXI6s_zxLh5b0Z4bj_Q
✅ Cached order status
📊 Quick status lookup: {'value': '{"status":"processing","updated_at":"2026-04-12T16:42:06.935069+00:00"}'}

🧹 Cleaning up...
✅ Cleanup complete

✅ All KV & Wrapped Types examples completed!

💡 Key takeaways:
   ✅ Use field_* helpers for type-safe wrapped values
   ✅ field_decimal() preserves precision (no floating point errors)
   ✅ KV store is great for caching and quick lookups
   ✅ Combine KV caching with collection inserts for real workflows
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_py (eTQdgAbFbazgJxhaYfjHzpUzsubfqXeD8IRjr4mj-dK4dDZORnKjFQX7ytk1_oBSo7_f0BDlsmk9iAKhGB-7_A)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
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
    }
  ],
  "stats": {
    "execution_time_ms": 0,
    "input_count": 0,
    "output_count": 1,
    "stage_stats": [],
    "stages_executed": 2
  }
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 39ms (served from cache)
Result: {
  "records": [
    {
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
    }
  ],
  "stats": {
    "execution_time_ms": 0,
    "input_count": 0,
    "output_count": 1,
    "stage_stats": [],
    "stages_executed": 2
  }
}
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
🚀 ekoDB Python Client - Native SWR Function Examples

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
✓ Created native SWR script: github_user_native (bdgx8hGjrX5XJdLf3IPZKq980fg9aqOZjUrYzMaqUf5We_4LrZj1t7rYEk-M0r12dkBwKR3TShGAuW0GQzRP2w)

First call (cache miss - will fetch from GitHub API):
  Response time: 128ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 23ms
  Speedup: 5.6x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (PRNTcy75K0XbcLZqjRRA-isE68E8_PkpIvz1gH1uWW_tlvgeXwOCqteQQ3Et5D1WHYNAqvbsxJch6-Y7WGJ_Kg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (_p_Rw41NzK3XE1Dbl_9eC4QlUe6_f2BwY6jCI9XJZUvPr83PgD7OA-2qX8waCQeLV9h9xflI5djZ_bgN0ReDdA)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (OdapOh2Z0nWXxF5WaY5g9Z7NvJrIkHD5al5GreylrAGd58hWb3a-ZflUU5t7eZUtsSHR7oBw9sp4CsIdXs138g)
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
🚀 ekoDB Python Advanced Functions Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Function saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Function saved
📊 Category breakdown:
   {'avg_price': 367.0, 'category': 'Electronics', 'count': 5}
   {'avg_price': 365.6666666666667, 'category': 'Furniture', 'count': 3}
⏱️  Execution time: 0ms

📝 Example 3: Count Total Products

✅ Function saved
📊 Total products: [{'total': 8}]
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
🚀 ekoDB Python AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   The benefits of using vector databases include:

1. **High-dimensional data handling**: They efficiently store and manage high-dimensional vectors, ideal for complex data types like images, text, and audio.

2. **Fast similarity search**: Vector databases support rapid nearest-neighbor search capabilities, enabling quick retrieval of similar items based on vector representations.

3. **Scalability**: They are designed to scale with large datasets, making them suitable for applications with extensive data growth.

4. **Support for machine learning**: They facilitate the integration of machine learning models, allowing for real-time inference and analysis.

5. **Multi-modal data support**: Vector databases can handle diverse data types, enabling applications in various domains such as natural language processing, computer vision, and recommendation systems.

6. **Enhanced performance**: Optimized algorithms improve performance in search and retrieval tasks, reducing latency and bandwidth usage.

7. **Similarity-based recommendations**: They enable personalized content recommendations based on user behavior and preferences measured through vector representations. 

8. **Flexibility**: Users can easily implement advanced analytics and personalized applications using the stored vector data.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embed script saved
📊 Embedding generated
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All AI script examples finished!

💡 This example demonstrates:
   ✅ Chat completions with system/user messages
   ✅ Embedding generation for text
🚀 ekoDB Python CRUD Functions Example

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
📊 Average score by role:
   {'avg_score': 70.0, 'count': 7, 'role': 'user'}
   {'avg_score': 20.0, 'count': 3, 'role': 'admin'}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
🚀 ekoDB Python Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Natural Language Processing (AI)
   2. Introduction to Machine Learning (AI)
   3. Getting Started with ekoDB (Database)
   4. Vector Databases Explained (Database)
   5. Database Design Principles (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {'category': 'AI', 'count': 2}
   {'category': 'Database', 'count': 3}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: hqgZ7u1n4z1PjYSwN3mGKkYDjbJgw-JhqI7U6wQhwAaSmQMFswL4loRxhLaW_4sKaj0SpDoJPx-rNtOdQ8yMWw

=== Sending Chat Message ===
Message ID: GXpyUv0anDvvXrmvJJxOeiv7Ctkm2el2AFSvSKZxKbH2oBb2HvUvaSmHW-YeO5hhC8Q3yWOiaYC5TUZI7Z6LOA

=== AI Response ===
Here are the available products and their prices:

1. **ekoDB**
   - Price: $99
   - Description: A high-performance database product with AI capabilities.

2. **ekoDB Cloud**
   - Price: $499
   - Description: Fully managed cloud database service product.

3. **ekoDB Pro**
   - Price: $299
   - Description: Enterprise edition product with advanced features.

Execution Time: 3368ms

=== Token Usage ===
Prompt tokens: 1298
Completion tokens: 90
Total tokens: 1388

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 0njM6U28ucl9FaZZebz_LXUVIxYIyRjXIwOyo-O31JcDu8LHpawYqBQJWY4ae5PRGPglZqsaTOP5tMDJd0qxvQ

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you need more information or want to see additional products, please let me know!

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['llm_provider', 'created_at', 'role', 'content', 'chat_id', 'id', 'token_usage', 'context_snippets', 'updated_at', 'llm_model'])
Debug: First message role: {'value': 'assistant', 'type': 'String'}
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you have any more questions or need further assistance, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: 5qaltAlAg_c0xW2xSTM4Las-hGRQUIRoLBisgrFpRUkeQd8WdOmlaAueAj-04BaV2RHQ5oroV5f16673NJTdjQ
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
✓ Created session: yjDtt3Myn4J9VCsJl1q64BwCSIek6DgKpLdOUgwWzwGL8ht9XgjOAZSpGsomKCMrt42gOaiH59G-lArj1Zj4Mg

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99

If you need more information or details on additional products, just let me know!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: zOxm83y9Z8HSQLgfcOuKLxLm7gNlfzIxpK26xdJZVA2rd8Up6nBp68tJ47tZXQXbxwp2rsuzzik5dK3MtW1NZg
  Parent: yjDtt3Myn4J9VCsJl1q64BwCSIek6DgKpLdOUgwWzwGL8ht9XgjOAZSpGsomKCMrt42gOaiH59G-lArj1Zj4Mg

=== Listing Sessions ===
✓ Found 7 sessions
  Session 1: zOxm83y9Z8HSQLgfcOuKLxLm7gNlfzIxpK26xdJZVA2rd8Up6nBp68tJ47tZXQXbxwp2rsuzzik5dK3MtW1NZg (Untitled)
  Session 2: yjDtt3Myn4J9VCsJl1q64BwCSIek6DgKpLdOUgwWzwGL8ht9XgjOAZSpGsomKCMrt42gOaiH59G-lArj1Zj4Mg (Untitled)
  Session 3: 5qaltAlAg_c0xW2xSTM4Las-hGRQUIRoLBisgrFpRUkeQd8WdOmlaAueAj-04BaV2RHQ5oroV5f16673NJTdjQ (Untitled)
  Session 4: hqgZ7u1n4z1PjYSwN3mGKkYDjbJgw-JhqI7U6wQhwAaSmQMFswL4loRxhLaW_4sKaj0SpDoJPx-rNtOdQ8yMWw (Untitled)
  Session 5: 0ZZzmUTPDZOFp4razygtycQFjXKop4-aPaMKK1bzpnKQ_UXhdMTBB1So4reoVYfYqSZXHnBjvWsejC2HlwDJNw (Untitled)
  Session 6: l5DeQ5mR60sriqk2G-P3hkq7CA5vp5JG9onoxl5Z3eTWo12j2q-ACHbJp8bD_xx8JeWcaeb6ILOzK82IBltWmA (Untitled)
  Session 7: W_XoN7N1gKoGeMnrYxB84031ZTTwjzTNPGO90Hga4096w6VlR5nj8oHRtfCyQNWugwRiHGvLiRlRrPYKATXZoA (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: zOxm83y9Z8HSQLgfcOuKLxLm7gNlfzIxpK26xdJZVA2rd8Up6nBp68tJ47tZXQXbxwp2rsuzzik5dK3MtW1NZg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': '0jye6ovJKPeRJjD98BzB_OwDzUS3zlg0rJoHPKXXMQCje9UcBcxKYHsxebdrjAIwJWktmTTH24EFTMImOD_cMQ'}

=== Upsert Operation ===
✓ Upsert (update existing record): 0jye6ovJKPeRJjD98BzB_OwDzUS3zlg0rJoHPKXXMQCje9UcBcxKYHsxebdrjAIwJWktmTTH24EFTMImOD_cMQ
✓ Inserted second record: tMSszyKbDxp0X2_gX3qALdU3dWfCAipOa25jG8n097tYYByirbSoY5uufsqCrldR8fBkh9qVU_Lp1-C48aLEPQ
✓ Upsert (update second record): tMSszyKbDxp0X2_gX3qALdU3dWfCAipOa25jG8n097tYYByirbSoY5uufsqCrldR8fBkh9qVU_Lp1-C48aLEPQ

=== Find One Operation ===
✓ Found user by email: {'active': {'value': True, 'type': 'Boolean'}, 'email': {'type': 'String', 'value': 'alice.j@newdomain.com'}, 'age': {'value': 29, 'type': 'Integer'}, 'name': {'type': 'String', 'value': 'Alice Johnson'}, 'id': '0jye6ovJKPeRJjD98BzB_OwDzUS3zlg0rJoHPKXXMQCje9UcBcxKYHsxebdrjAIwJWktmTTH24EFTMImOD_cMQ'}
✓ User not found (as expected)

=== Exists Check ===
✓ Record exists: True
✓ Fake record exists: False (should be False)

=== Pagination ===
✓ Inserted 25 records for pagination
✓ Page 1: 10 records (expected 10)
✓ Page 2: 10 records (expected 10)
✓ Page 3: 7 records (expected ~7)

=== Cleanup ===
✓ Deleted collection

✅ All convenience methods demonstrated successfully!
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: {'id': 'rYJGFmQu3HVs_bI_44AElfp4L9Fbx5aEnQVnJlXKL5v4b9xxjy-yBDOzwzRwME9SHBxys278Y7XIXaQc2SR-Ig'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'jHdG183ZBzRsPyw019oM6STyq21yLGvOIm7XSvjJKNx4Tifmo5rSRR2Q6VrqIAHUYS6r7ZlgMdL4mFOpv-a4Sg'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'price': {'type': 'Integer', 'value': 150}, 'id': 'rYJGFmQu3HVs_bI_44AElfp4L9Fbx5aEnQVnJlXKL5v4b9xxjy-yBDOzwzRwME9SHBxys278Y7XIXaQc2SR-Ig', 'name': {'type': 'String', 'value': 'Product 1'}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'name': {'value': 'Upsert Product', 'type': 'String'}, 'price': {'type': 'Integer', 'value': 500}, 'id': 'rYJGFmQu3HVs_bI_44AElfp4L9Fbx5aEnQVnJlXKL5v4b9xxjy-yBDOzwzRwME9SHBxys278Y7XIXaQc2SR-Ig'}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ['id', 'email', 'name']
  First user: {'type': 'String', 'value': 'Alice Johnson'} <{'type': 'String', 'value': 'alice@example.com'}>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ['name', 'email', 'bio', 'avatar_url', 'status', 'id', 'created_at', 'user_role', 'age']

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - {'type': 'String', 'value': 'Dave Brown'} (age {'value': 45, 'type': 'Integer'})
    - {'value': 'Alice Johnson', 'type': 'String'} (age {'type': 'Integer', 'value': 30})
    - {'value': 'Bob Smith', 'type': 'String'} (age {'value': 25, 'type': 'Integer'})

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - {'value': 'Carol White', 'type': 'String'}: {'value': 'Manager', 'type': 'String'}

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ['user_role', 'name', 'secret_token', 'created_at', 'email', 'api_key', 'id', 'status', 'age', 'bio', 'avatar_url', 'password']
  Projected query:
    - 3 fields per record
    - Fields: ['name', 'id', 'email']
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
=== KV Precision: Float vs Decimal ===

=== Test 1: Using Python Floats (LOSES PRECISION) ===
✓ Stored products with float prices

Retrieved float prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 2: Using field_decimal() (PRESERVES PRECISION) ===
✓ Stored products with decimal prices

Retrieved decimal prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 3: Sum Calculation Comparison ===
  Float sum: $119.97 (expected $119.97)
  Decimal sum: $119.97 (expected $119.97)

=== Test 4: Extreme Precision Example ===
  Float 0.1 + 0.2 = 0.30000000000000004 (should be 0.3)
  Decimal "0.30" = 0.30 (exact!)

=== Cleanup ===
✓ Cleaned up test keys

=== Summary ===
✅ Use field_decimal() for monetary values, percentages, and
   any case where floating-point errors are unacceptable.
✅ field_decimal() stores values as strings internally,
   preserving exact precision across all operations.
✓ Client created

=== Get All Chat Models ===
OpenAI models: ['gpt-4-0613', 'gpt-4', 'gpt-3.5-turbo', 'gpt-5.4-mini', 'gpt-5.4', 'gpt-5.4-nano-2026-03-17', 'gpt-5.4-nano', 'gpt-5.4-mini-2026-03-17', 'davinci-002', 'babbage-002', 'gpt-3.5-turbo-instruct', 'gpt-3.5-turbo-instruct-0914', 'dall-e-3', 'dall-e-2', 'gpt-3.5-turbo-1106', 'tts-1-hd', 'tts-1-1106', 'tts-1-hd-1106', 'text-embedding-3-small', 'text-embedding-3-large', 'gpt-3.5-turbo-0125', 'gpt-4-turbo', 'gpt-4-turbo-2024-04-09', 'gpt-4o', 'gpt-4o-2024-05-13', 'gpt-4o-mini-2024-07-18', 'gpt-4o-mini', 'gpt-4o-2024-08-06', 'gpt-4o-audio-preview', 'gpt-4o-realtime-preview', 'omni-moderation-latest', 'omni-moderation-2024-09-26', 'gpt-4o-realtime-preview-2024-12-17', 'gpt-4o-audio-preview-2024-12-17', 'gpt-4o-mini-realtime-preview-2024-12-17', 'gpt-4o-mini-audio-preview-2024-12-17', 'o1-2024-12-17', 'o1', 'gpt-4o-mini-realtime-preview', 'gpt-4o-mini-audio-preview', 'o3-mini', 'o3-mini-2025-01-31', 'gpt-4o-2024-11-20', 'gpt-4o-mini-search-preview-2025-03-11', 'gpt-4o-mini-search-preview', 'gpt-4o-transcribe', 'gpt-4o-mini-transcribe', 'o1-pro-2025-03-19', 'o1-pro', 'gpt-4o-mini-tts', 'o3-2025-04-16', 'o4-mini-2025-04-16', 'o3', 'o4-mini', 'gpt-4.1-2025-04-14', 'gpt-4.1', 'gpt-4.1-mini-2025-04-14', 'gpt-4.1-mini', 'gpt-4.1-nano-2025-04-14', 'gpt-4.1-nano', 'gpt-image-1', 'gpt-4o-realtime-preview-2025-06-03', 'gpt-4o-audio-preview-2025-06-03', 'o4-mini-deep-research', 'gpt-4o-transcribe-diarize', 'o4-mini-deep-research-2025-06-26', 'gpt-5-chat-latest', 'gpt-5-2025-08-07', 'gpt-5', 'gpt-5-mini-2025-08-07', 'gpt-5-mini', 'gpt-5-nano-2025-08-07', 'gpt-5-nano', 'gpt-audio-2025-08-28', 'gpt-realtime', 'gpt-realtime-2025-08-28', 'gpt-audio', 'gpt-5-codex', 'gpt-image-1-mini', 'gpt-5-pro-2025-10-06', 'gpt-5-pro', 'gpt-audio-mini', 'gpt-audio-mini-2025-10-06', 'gpt-5-search-api', 'gpt-realtime-mini', 'gpt-realtime-mini-2025-10-06', 'sora-2', 'sora-2-pro', 'gpt-5-search-api-2025-10-14', 'gpt-5.1-chat-latest', 'gpt-5.1-2025-11-13', 'gpt-5.1', 'gpt-5.1-codex', 'gpt-5.1-codex-mini', 'gpt-5.1-codex-max', 'gpt-image-1.5', 'gpt-5.2-2025-12-11', 'gpt-5.2', 'gpt-5.2-pro-2025-12-11', 'gpt-5.2-pro', 'gpt-5.2-chat-latest', 'gpt-4o-mini-transcribe-2025-12-15', 'gpt-4o-mini-transcribe-2025-03-20', 'gpt-4o-mini-tts-2025-03-20', 'gpt-4o-mini-tts-2025-12-15', 'gpt-realtime-mini-2025-12-15', 'gpt-audio-mini-2025-12-15', 'chatgpt-image-latest', 'gpt-5.2-codex', 'gpt-5.3-codex', 'gpt-realtime-1.5', 'gpt-audio-1.5', 'gpt-4o-search-preview', 'gpt-4o-search-preview-2025-03-11', 'gpt-5.3-chat-latest', 'gpt-5.4-2026-03-05', 'gpt-5.4-pro', 'gpt-5.4-pro-2026-03-05', 'gpt-3.5-turbo-16k', 'tts-1', 'whisper-1', 'text-embedding-ada-002']
Anthropic models: ['claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929', 'claude-opus-4-1-20250805', 'claude-opus-4-20250514', 'claude-sonnet-4-20250514', 'claude-3-haiku-20240307']
Perplexity models: ['sonar', 'sonar-pro', 'sonar-deep-research', 'sonar-reasoning', 'sonar-reasoning-pro']

=== Get OpenAI Models ===
OpenAI models: ['gpt-4-0613', 'gpt-4', 'gpt-3.5-turbo', 'gpt-5.4-mini', 'gpt-5.4', 'gpt-5.4-nano-2026-03-17', 'gpt-5.4-nano', 'gpt-5.4-mini-2026-03-17', 'davinci-002', 'babbage-002', 'gpt-3.5-turbo-instruct', 'gpt-3.5-turbo-instruct-0914', 'dall-e-3', 'dall-e-2', 'gpt-3.5-turbo-1106', 'tts-1-hd', 'tts-1-1106', 'tts-1-hd-1106', 'text-embedding-3-small', 'text-embedding-3-large', 'gpt-3.5-turbo-0125', 'gpt-4-turbo', 'gpt-4-turbo-2024-04-09', 'gpt-4o', 'gpt-4o-2024-05-13', 'gpt-4o-mini-2024-07-18', 'gpt-4o-mini', 'gpt-4o-2024-08-06', 'gpt-4o-audio-preview', 'gpt-4o-realtime-preview', 'omni-moderation-latest', 'omni-moderation-2024-09-26', 'gpt-4o-realtime-preview-2024-12-17', 'gpt-4o-audio-preview-2024-12-17', 'gpt-4o-mini-realtime-preview-2024-12-17', 'gpt-4o-mini-audio-preview-2024-12-17', 'o1-2024-12-17', 'o1', 'gpt-4o-mini-realtime-preview', 'gpt-4o-mini-audio-preview', 'o3-mini', 'o3-mini-2025-01-31', 'gpt-4o-2024-11-20', 'gpt-4o-mini-search-preview-2025-03-11', 'gpt-4o-mini-search-preview', 'gpt-4o-transcribe', 'gpt-4o-mini-transcribe', 'o1-pro-2025-03-19', 'o1-pro', 'gpt-4o-mini-tts', 'o3-2025-04-16', 'o4-mini-2025-04-16', 'o3', 'o4-mini', 'gpt-4.1-2025-04-14', 'gpt-4.1', 'gpt-4.1-mini-2025-04-14', 'gpt-4.1-mini', 'gpt-4.1-nano-2025-04-14', 'gpt-4.1-nano', 'gpt-image-1', 'gpt-4o-realtime-preview-2025-06-03', 'gpt-4o-audio-preview-2025-06-03', 'o4-mini-deep-research', 'gpt-4o-transcribe-diarize', 'o4-mini-deep-research-2025-06-26', 'gpt-5-chat-latest', 'gpt-5-2025-08-07', 'gpt-5', 'gpt-5-mini-2025-08-07', 'gpt-5-mini', 'gpt-5-nano-2025-08-07', 'gpt-5-nano', 'gpt-audio-2025-08-28', 'gpt-realtime', 'gpt-realtime-2025-08-28', 'gpt-audio', 'gpt-5-codex', 'gpt-image-1-mini', 'gpt-5-pro-2025-10-06', 'gpt-5-pro', 'gpt-audio-mini', 'gpt-audio-mini-2025-10-06', 'gpt-5-search-api', 'gpt-realtime-mini', 'gpt-realtime-mini-2025-10-06', 'sora-2', 'sora-2-pro', 'gpt-5-search-api-2025-10-14', 'gpt-5.1-chat-latest', 'gpt-5.1-2025-11-13', 'gpt-5.1', 'gpt-5.1-codex', 'gpt-5.1-codex-mini', 'gpt-5.1-codex-max', 'gpt-image-1.5', 'gpt-5.2-2025-12-11', 'gpt-5.2', 'gpt-5.2-pro-2025-12-11', 'gpt-5.2-pro', 'gpt-5.2-chat-latest', 'gpt-4o-mini-transcribe-2025-12-15', 'gpt-4o-mini-transcribe-2025-03-20', 'gpt-4o-mini-tts-2025-03-20', 'gpt-4o-mini-tts-2025-12-15', 'gpt-realtime-mini-2025-12-15', 'gpt-audio-mini-2025-12-15', 'chatgpt-image-latest', 'gpt-5.2-codex', 'gpt-5.3-codex', 'gpt-realtime-1.5', 'gpt-audio-1.5', 'gpt-4o-search-preview', 'gpt-4o-search-preview-2025-03-11', 'gpt-5.3-chat-latest', 'gpt-5.4-2026-03-05', 'gpt-5.4-pro', 'gpt-5.4-pro-2026-03-05', 'gpt-3.5-turbo-16k', 'tts-1', 'whisper-1', 'text-embedding-ada-002']

=== Get Anthropic Models ===
Anthropic models: ['claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929', 'claude-opus-4-1-20250805', 'claude-opus-4-20250514', 'claude-sonnet-4-20250514', 'claude-3-haiku-20240307']

✓ Chat Models API example complete
✓ Client created

=== Create User Function ===
Created user function with ID: rUkdJP97IT9jACNcXv7KeZrC0OqkgROGWUO5oMQ7uezjz-nZH0Tb9QaQb8D3HCzGz6Xum0qAmix5wg3-0dxadA

=== Get User Function ===
Retrieved: get_active_users_py - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 25 user functions:
  - get_active_users_updated: Get Active Users (Updated)
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_users_by_status: Get Users By Status
  - get_verified_user: Get verified and validated user
  - get_verified_user: Get verified and validated user
  - get_users_by_status: Get Users By Status
  - get_active_users_py: Get Active Users
  - fetch_slim_user: Validate and slim down user
  - get_user_wrapper: Wrapper that calls fetch_user
  - cache_api_call_py: Cache External API Call
  - get_active_users_paginated: Get Active Users (Paginated)
  - get_active_users_updated: Get Active Users (Updated)
  - get_high_scoring_active_users: Get High Scoring Active Users
  - fetch_slim_user: Validate and slim down user
  - validate_user: Check if user exists
  - get_active_users_paginated: Get Active Users (Paginated)
  - get_user_wrapper: Wrapper that calls fetch_user
  - swr_user: SWR pattern for user data (KV-based)
  - validate_user: Check if user exists
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_user: Fetch user by code
  - swr_user: SWR pattern for user data (KV-based)
  - get_active_users: Get Active Users (Updated)
  - fetch_user: Fetch user by code
  - get_active_users: Get Active Users (Updated)

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
✓ Client created

=== Check Collection Exists (Before Creation) ===
Collection 'collection_utils_test_py' exists: False

=== Creating Test Documents ===
Created 5 test documents

=== Check Collection Exists (After Creation) ===
Collection 'collection_utils_test_py' exists: True

=== Count Documents ===
Document count in 'collection_utils_test_py': 5

=== Check Non-Existent Collection ===
Collection 'nonexistent_collection_xyz' exists: False

=== Cleanup ===
Deleted collection 'collection_utils_test_py'

✓ Collection Utilities example complete
✅ [32mPython client examples complete![0m
✅ [32mAll Python integration tests complete![0m
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
Inserted: map[id:lcjMQ5DYPu0YgyLnumw17X3aEvqCbi2gz2SXCef8wlAQ7fUgDequcfkQqe1ccGSdjDT9xFJVc1VrjhjOXt37xw]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:lcjMQ5DYPu0YgyLnumw17X3aEvqCbi2gz2SXCef8wlAQ7fUgDequcfkQqe1ccGSdjDT9xFJVc1VrjhjOXt37xw name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found documents: [map[active:map[type:Boolean value:true] id:lcjMQ5DYPu0YgyLnumw17X3aEvqCbi2gz2SXCef8wlAQ7fUgDequcfkQqe1ccGSdjDT9xFJVc1VrjhjOXt37xw name:map[type:String value:Test Record] value:map[type:Integer value:42]]]

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:lcjMQ5DYPu0YgyLnumw17X3aEvqCbi2gz2SXCef8wlAQ7fUgDequcfkQqe1ccGSdjDT9xFJVc1VrjhjOXt37xw name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: UYjUzebVUq9jyfdiOrA690sKXzuuYoRp075pKBdtpqydEtcSDCZXMcvbBXk0YR70qiviclhcV9Mx-7bxfyFGTw

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1776012184992102000",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "FeqA3vaSCqzaM09W3ES2j5OE8ThY6bgfDXBhYB9paIu6aM8tKHwgA8kvLgSA_dvnIjkt2zrbSb_ciW8KsFJ-KA",
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
        "id": "MtXpLar30tru4Y354yBpOXUjpyCzRaHSnY8__zDq3z-SdZu4fg-jkmuyQAxukeHRhr6q1tC5bKDJeWdu3oDqHQ",
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
        "id": "UYjUzebVUq9jyfdiOrA690sKXzuuYoRp075pKBdtpqydEtcSDCZXMcvbBXk0YR70qiviclhcV9Mx-7bxfyFGTw",
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
🚀 ekoDB Functions Example (Go/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: k9zIqDy6Rq3VqTZFxf1phWLmGxxFX3RQPWxoRmc8LTyKdApAiL7_QVGmfYCNJ3DVg3NCHH3b37991WtALrRCDA
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: MLHBfDU6AuEOT230UYEyLvSP1va88D2C1gHcs2NGacyY485U6-Kd_k8vSyyZ_0_Trny_fuCAcHoMfKPwP__FLA
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: T1OMnK6o_l1_bOc_qNFge5lgcRuYU4eRNPeWEn9FUGrd_9ebQZEJ6zZA-t9rm3pH4U_oiWbprbMaMBy9dmVtrg
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":60,"count":10,"max_score":100,"status":"active"}
   {"avg_score":50,"count":10,"max_score":90,"status":"inactive"}

📝 Example 4: Function Management

📋 Total functions: 27
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

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
Collection created with first record: GDl5JO5JzxPeAJ5XUpHA8GEIidLnViVSk4S3Aaf6Y0JVAbUAuz9IQxZZ1WLPuPozAmIDuMqwnaverxXNYwaihA

=== List Collections ===
Total collections: 17
Sample collections: [schema_users_client_py test_collection users demo_collection chat_messages__ek0_testing]

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
Created Alice: $1000 - ID: 3e3ue26h6NSunNGEsQocxENr_9WD__e3D3HotTtXjhK61CnmgLlNF6gVqHJdQirjy0vUM-iVmLKbXYGRh3qJ0g
Created Bob: $500 - ID: rTpgkYGLDmL4n13jzfdNYkjLL5hzCqR4Ei4td-Nleu5v9B0Ww6GvRZafHH-FHTwqKl9k-fOCCdgG_Fbtyalz7A

=== Example 1: Begin Transaction ===
Transaction ID: 71e0dcd1-7f34-4130-b92e-544ec6666af0

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
New transaction: 0a140176-0bcd-4004-ac1d-7457848bb3d7
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: map[type:Integer value:700]

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
[32m✓ transactions.go completed successfully[0m
[34m
=== Running crud_functions.go ===[0m
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
📝 function 1: Insert + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: ApPKISk5cTdlW7gu-aWVIKl-8QpwgE3UbGn0meJBD3sdvj4ViOPjnzwuNUo2jj2X3tyv_4MrmgGMjpgV81KZmw

2️⃣ Calling function (Insert + Verify)...
   ✅ function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: 8jH1hPw1zFA6Lg8LKw0BdVRNeGk3oGuHf57o8ZC1QG93StCuQW1uce-v1GeEPqfMSTDXFl4fe_WycxAyTuxvMQ
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: gZIOmclI2YZoeTdEsGUEjCnPbO_DyDLQkzTimC2o8YzAkpbGNS6ypzIXKBEzEVk8ZSb8tRqDqtty0IDycIGk3A

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
   ✅ Function saved: P2KevOT5g9FzJotYWvFiOzeFxvTNO1QMnOAYw55-dPtyyDtA1__LxRenX4vTMenOuh8J00xpEWOl394uZQpsTg

2️⃣ Calling function (Query + Update Credits + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: map[type:Integer value:0]
   📋 Status: map[type:String value:active]
   📋 Name: map[type:String value:Alice Smith]

============================================================
📝 function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: GkOjLiOqFtRb43yXoQrBYBfM4hhQFpqtINB2BDafQe4md0ozmskCdNZLpr_-1_rZoPWFWn0czkH57bHbZ4pmjQ

2️⃣ Calling function (Query + Delete + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: ApPKISk5cTdlW7gu-aWV...
   ✅ Deleted script: gZIOmclI2YZoeTdEsGUE...
   ✅ Deleted script: P2KevOT5g9FzJotYWvFi...
   ✅ Deleted script: GkOjLiOqFtRb43yXoQrB...
   ✅ Deleted collection: users

============================================================
✅ Complete CRUD Functions Example Finished!
============================================================

💡 Key Takeaways:
   ✅ Functions chain steps together
   ✅ Each function demonstrates operation + verification
   ✅ Parameters make functions reusable
   ✅ Verification is built into the function itself
   ✅ Complete CRUD lifecycle in 4 focused functions
[32m✓ crud_functions.go completed successfully[0m
[34m
=== Running document_ttl.go ===[0m
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
  Output: Document ID = 3LK7AWkDKYayTuz73prX2NGOH6EKAa6EFSLBf1qJTGayQZBBO2UgOGYsOE9mPWycCxwExFUnkpXQGi-bVJhHpg
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: FindByID(3LK7AWkDKYayTuz73prX2NGOH6EKAa6EFSLBf1qJTGayQZBBO2UgOGYsOE9mPWycCxwExFUnkpXQGi-bVJhHpg)
  Output: Found document with name = map[type:String value:TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: FindByID(3LK7AWkDKYayTuz73prX2NGOH6EKAa6EFSLBf1qJTGayQZBBO2UgOGYsOE9mPWycCxwExFUnkpXQGi-bVJhHpg)
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

✓ Client connected

═══════════════════════════════════════════════════════════
TEST: WebSocket TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'WS TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = CE_1Y7LjblDZVuZ3KJuoOKDsfbI0UMDX20WxX7UVGh048GZnJuSFAVJjxHfPcFX8kLnwSdR0wbvDFyI6RqYyyg
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: FindByID(CE_1Y7LjblDZVuZ3KJuoOKDsfbI0UMDX20WxX7UVGh048GZnJuSFAVJjxHfPcFX8kLnwSdR0wbvDFyI6RqYyyg)
  Output: Found document with name = map[type:String value:WS TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: FindByID(CE_1Y7LjblDZVuZ3KJuoOKDsfbI0UMDX20WxX7UVGh048GZnJuSFAVJjxHfPcFX8kLnwSdR0wbvDFyI6RqYyyg)
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
Inserted: map[id:YpmpBZOGWTm52BltV6H6ZQ5NS7o_Vkp25XcED6aV-GUxNC_tdQSzag-KPF5xV1ddF4S0CN8h_L80qVnLGFcTtg]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-04-12T16:43:35Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:YpmpBZOGWTm52BltV6H6ZQ5NS7o_Vkp25XcED6aV-GUxNC_tdQSzag-KPF5xV1ddF4S0CN8h_L80qVnLGFcTtg metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Test Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:42]]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-04-12 16:43:35 +0000 UTC
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-04-12T16:43:35Z data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:YpmpBZOGWTm52BltV6H6ZQ5NS7o_Vkp25XcED6aV-GUxNC_tdQSzag-KPF5xV1ddF4S0CN8h_L80qVnLGFcTtg metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-04-12T16:43:35Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:YpmpBZOGWTm52BltV6H6ZQ5NS7o_Vkp25XcED6aV-GUxNC_tdQSzag-KPF5xV1ddF4S0CN8h_L80qVnLGFcTtg metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: i83m7R7XMn7Zev02y606OpCwoSaWeDZi0u2lAz-xOrYK-Hn6zZ94RTw5AkkhRLW7y0q0rpzZT-e_U98KR8Ydug

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
Collection created with first record: yvAIF55JeUOaWqoHa25cVpuK0Iu9xCL6TUitGexQdVU269JgzYqBg8fuJ3wz9Y9HHDuc8ELtbyKCNHJTBdtGUg

=== List Collections ===
Total collections: 16
Sample collections: [schema_users_client_py test_collection chat_messages__ek0_testing test_accounts enriched_users]

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
  cache:product:1: map[name:Product 1 price:29.99]
  cache:product:2: map[name:Product 2 price:39.99]
  cache:product:3: map[name:Product 3 price:49.99]

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 12

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
Created Alice: $1000 - ID: urNQbeYV8G6XhJTd-h3fiKVkH4Ys_OlHrFxJ3zjh6ZBaeUHSd1i8zn0XjBnd6n1GONDsHMVSYFCabgazPeR1Tw
Created Bob: $500 - ID: oJqslT30C-rUd-zeTVS-mBG83-BsdOXP5DUwqy_hB4O_T5BRl0lUzIOG7QaZiau9Sov0rj9Tx3G-24grh2iAGQ

=== Example 1: Begin Transaction ===
Transaction ID: ef7e2e82-38c2-4ef3-a1df-b097721e2c45

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: b0dfc32c-7681-4b54-9ed5-9a8218b0c359
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
  1. Score: 0.772
  2. Score: 0.758
  3. Score: 0.757

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.703
  2. Score: 1.503
  3. Score: 0.309

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
✓ Inserted document: kv1ICvqjCFL3LDs26Nw6cMUc44RzvZCeQPXIqZjOUKiWQRWh3LjWLxBhsWgUqbpj1g7G4sj-NuJidXZFdn6sAA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: eAHWp2ootvS-bjekosTkOR8s-CXaIfQf6cjN10fdQs7jgKUIPC91rfA3684tCmsuykSuGxKmLzd4lBd1W66CKA

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
✓ Inserted document with TTL: Ve7LT1KTyJRN4-zs8r3MkVCw2NOMMHSk2GnMvVpRBU_-yxrPyJn4a3IXYgup5Ug62ssBgGJ75o4pMqGyuI5y9Q

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
✓ Edge cache script created: eCYki-xJksdNAS35soq0aTE9jXLkaA_HhZbLstKXIU9sHNSrAC__bPuZ-Hj04o2Md616-QQ7OTCr-UAAj1xl0Q

Call 1: Cache lookup
Response time: 14ms
Found 1 cached entries

Call 2: Cache lookup (connection warm)
Response time: 14ms
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
🚀 ekoDB Functions Example (Go Client)

✅ Client initialized

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: bK066-IROjvda_gmbBdy4e7J_PbuhLrrBfzYZ9LE1yH_6YipcqEbPEnugPCV_ZSQ4Vvg-e5tM1SOz-ru4tU96Q
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

✅ Function saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 2 groups
   map[avg_score:50 count:5 status:inactive]
   map[avg_score:60 count:5 status:active]
⏱️  Execution time: 0ms

📝 Example 4: Function Management

📋 Total functions: 29
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
   ⏱️  Duration: 37.532292ms
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
   ⏱️  Duration: 37.913917ms
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
🚀 ekoDB Go Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: tZHFvo13Bpdb1twtwATCu_UJ_7COAp3AvtuCzCdh7GEWF2VC7R01d2Am8ZoeeqhP7kPbOFSdvayFqg1ry4Zktw
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

🧹 Cleaning up...
✅ Cleanup complete

✅ All complete script examples finished!

💡 This example demonstrates ekoDB's function system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Function management (save, call, delete)
🚀 ekoDB Go KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: llh6goweS6S1PWsHVFB7XtLpOyrQMddyGj5_QEFSYnhqKsQUySoFacitBRp7ZJAp0Qq1jIpiNmUTGm9Xob97YQ
✅ Inserted 2 products with wrapped types

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: i76kqp6rB6fwjMSc3t5cNUlZG9MwNbu_hkxgoIn3Ohdh3hWw_19GxaWZP14uPPYoqjECpooMmOgTDkNLlQU7FQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: map[role:admin userId:user_abc]
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session
📝 Example 4: KV Operations in Functions

✅ Function saved: eZKnnT0xYmlMmOxYVtcj8J9xSzWKMqqeIbLdUVI3lcqvnkFnsaeqSicp-ZaOt7UscubY1BhuhzTcmYhdMAgjSA
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: l9CmZfubc0TO7C28MzRpiki8LE6Innecdnq1nhz3GFZcdu7H_nFTunWRx8WMP4mVnBnLpg2ICdIhiIkHs9vRCg
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
✓ Created SWR script: fetch_api_user_go (K1do1qK3V8rJ11I3lLxLfPgNJ15DNDVqEcN9bASiCVjoJRCweau06qZs3TorSWErfhQJZZkp-y7oO6g4_H8CzQ)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "cached_at": "2026-04-12T12:43:41-04:00",
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
Response time: 40ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
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
✓ Created native SWR script: github_user_native (UTb934j6JkQrjmxsm1pq8YQAEzuiLSt4FV1MusEvcQ0k9YebEl6QCoUed-D9fUqkJUPLNeCilgDHNiStvjXxoA)

First call (cache miss - will fetch from GitHub API):
  Response time: 13ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 13ms
  Speedup: 1.0x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (QeMWvwv8DgZfIlGelt5VFl6OIlCQ0mZmQo6fTW6k5BwWnW33Pnp0rYJ4THHQPAxGTXReQ6BMuj3EFg312N8-cQ)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (4xF9Ele9gj2G3Isnv00zHklJBhp_ETFy6KGjlm8yMmUC3c88ApZYaDBL45yB9GOfrXup_ZzZ0c90yyfISqcr9w)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (ZQo0mWOPRDziSQQzAPjJDulOOjsB3ObWn8gLPb10vhOq0xz21BSxLjddENp0GvutEsNNf-h1M86kYTHShYEUZA)
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
   map[avg_price:367 category:Electronics count:5]
   map[avg_price:365.6666666666667 category:Furniture count:3]
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
🚀 ekoDB Go AI Functions Example

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
   map[count:3 status:inactive]
   map[count:7 status:active]
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
🚀 ekoDB Go Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. map[type:String value:Natural Language Processing] (map[type:String value:AI])
   2. map[type:String value:Introduction to Machine Learning] (map[type:String value:AI])
   3. map[type:String value:Database Design Principles] (map[type:String value:Database])
   4. map[type:String value:Getting Started with ekoDB] (map[type:String value:Database])
   5. map[type:String value:Vector Databases Explained] (map[type:String value:Database])
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   map[category:AI count:2]
   map[category:Database count:3]
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: 32PFek8QtgY3SxhW6Q7KVWSbd7uQimZbD3OOUBIeM8mjW6NtmYSx4TqnDEaMk5EP30UDXloORbnCr2umJk9wrw

=== Sending Chat Message ===
Message ID: Cf-wMmRoJnsMZKwmNBrVjRitgrhQKvuhQYmM0K1nOkm0ae4G94CqjgUoYiiYKR7vzMA06LRKhN57WNWS4OQiDA

=== AI Response ===
The available products and their prices are as follows:

1. **ekoDB**
   - Price: $99
   - Description: A high-performance database product with AI capabilities.

2. **ekoDB Pro**
   - Price: $299
   - Description: Enterprise edition product with advanced features.

3. **ekoDB Cloud**
   - Price: $499
   - Description: Fully managed cloud database service product.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:1Qsp2MplFA3pNMbL5GsQQg3JkMSdSAQp0mlyOS3UofXBFbkMc_aJ6CuxEpE88s3Gvq7dR0ufoZe2-OdLQtE6lw name:ekoDB price:99] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:lfg4hJw96nK1lMyJgKmjAk6I7YFWOGvPK75-6eY_Y8UVqFEXgN_LuGYJma7AluST0yI-4n_Jbyt-zIKKvLL8yw name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:aI7zHkSFhRZFB2gtBf7aQ9Z7idxIP13i2GkEi3xpDejaYskJB2j3104OUSCHJ2arDrWazK9cpSuqHVPnX9fsmA name:ekoDB Cloud price:499] score:0.1111111111111111]

Execution Time: 5411ms

=== Token Usage ===
Prompt tokens: 1298
Completion tokens: 91
Total tokens: 1389

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: Ymjb2aCqqnCJASJ6wuHK4oUWrrxYvqlQMAsZwJK4Q3rnN6jLlS57fUBx_agZUQTHxEMRcDkRGR1M40lvii0K5g

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you need more information or details about additional products, just let me know!

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you have any further questions or need more information, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: Be84Ao5KLBzjK4zjM0uyi7WHiFGkgnGiBz913zjhkQrGrZn6A3j7uDi1wNEsih7W6bTQ2aTuD5OcL0gfaQQrvQ
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
✓ Created session: Znp6mUA8YF5IZMlJ_Gi5RO5IK4Csu0uLWqmdO2yqmd_ANoxYa250sKiTx-bmvvu2SIgeuzVldlKiXpUYweOW_g

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** A high-performance database product
- **Price:** $99

If you need more information or additional products, please let me know!

✓ Message 2 sent
  Response: The price of the product ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: gRi2x7k1129peQk0dkW2G7pW7GTrtewBoi2dX_GhaXl3QLqfHe0uLegFfrOrecYOAZzdAD4Opdzay3p_6IuEpQ
  Parent: Znp6mUA8YF5IZMlJ_Gi5RO5IK4Csu0uLWqmdO2yqmd_ANoxYa250sKiTx-bmvvu2SIgeuzVldlKiXpUYweOW_g

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: gRi2x7k1129peQk0dkW2G7pW7GTrtewBoi2dX_GhaXl3QLqfHe0uLegFfrOrecYOAZzdAD4Opdzay3p_6IuEpQ (Untitled)
  Session 2: Znp6mUA8YF5IZMlJ_Gi5RO5IK4Csu0uLWqmdO2yqmd_ANoxYa250sKiTx-bmvvu2SIgeuzVldlKiXpUYweOW_g (Untitled)
  Session 3: Be84Ao5KLBzjK4zjM0uyi7WHiFGkgnGiBz913zjhkQrGrZn6A3j7uDi1wNEsih7W6bTQ2aTuD5OcL0gfaQQrvQ (Untitled)
  Session 4: 32PFek8QtgY3SxhW6Q7KVWSbd7uQimZbD3OOUBIeM8mjW6NtmYSx4TqnDEaMk5EP30UDXloORbnCr2umJk9wrw (Untitled)
  Session 5: yjDtt3Myn4J9VCsJl1q64BwCSIek6DgKpLdOUgwWzwGL8ht9XgjOAZSpGsomKCMrt42gOaiH59G-lArj1Zj4Mg (Untitled)
  Session 6: 5qaltAlAg_c0xW2xSTM4Las-hGRQUIRoLBisgrFpRUkeQd8WdOmlaAueAj-04BaV2RHQ5oroV5f16673NJTdjQ (Untitled)
  Session 7: hqgZ7u1n4z1PjYSwN3mGKkYDjbJgw-JhqI7U6wQhwAaSmQMFswL4loRxhLaW_4sKaj0SpDoJPx-rNtOdQ8yMWw (Untitled)
  Session 8: 0ZZzmUTPDZOFp4razygtycQFjXKop4-aPaMKK1bzpnKQ_UXhdMTBB1So4reoVYfYqSZXHnBjvWsejC2HlwDJNw (Untitled)
  Session 9: l5DeQ5mR60sriqk2G-P3hkq7CA5vp5JG9onoxl5Z3eTWo12j2q-ACHbJp8bD_xx8JeWcaeb6ILOzK82IBltWmA (Untitled)
  Session 10: W_XoN7N1gKoGeMnrYxB84031ZTTwjzTNPGO90Hga4096w6VlR5nj8oHRtfCyQNWugwRiHGvLiRlRrPYKATXZoA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: gRi2x7k1129peQk0dkW2G7pW7GTrtewBoi2dX_GhaXl3QLqfHe0uLegFfrOrecYOAZzdAD4Opdzay3p_6IuEpQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Map Creation ===
✓ Created record with native map: map[id:RwmyYxE2ulMAU6m8lDypHdehVmK_Cyxtv3A9TVEcJ16rsJJM9gGWYT71wjdrmLF_KzgMABqxrJp04lNoJ-vZ2A]

=== Upsert Operation ===
✓ First upsert (update): map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:RwmyYxE2ulMAU6m8lDypHdehVmK_Cyxtv3A9TVEcJ16rsJJM9gGWYT71wjdrmLF_KzgMABqxrJp04lNoJ-vZ2A name:map[type:String value:Alice Johnson]]
✓ Second upsert (insert): map[id:new-user-id]

=== Find One Operation ===
✓ Found user by email: map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:RwmyYxE2ulMAU6m8lDypHdehVmK_Cyxtv3A9TVEcJ16rsJJM9gGWYT71wjdrmLF_KzgMABqxrJp04lNoJ-vZ2A name:map[type:String value:Alice Johnson]]
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
Inserted with ripple: map[id:Stt9f7EdjY-LMXtzT89XNojIPtDAEcNrPKtkqP2N2E-StVdJGL3fWy4TBTnU7jotBSxK0Vydn132cMezA8vMFA]
Inserted with bypass_ripple: map[id:SpyK7UMhns7oZXmlR9q_wnn2b_aXNZ19BCC8c9dAY2kOMIqA1uZZVeClh0VTXrBknYOecB__EqtFucQWdWLxdg]
Inserted with TTL and bypass_ripple: map[id:nJupzkIMVYKWJkaTi6GLsAej4HjR0xA39tvCx3brQelBegsFAzEkWgVAttbQjEGo9mWW2HK7QjdoEutm3tLFRg]
Updated with bypass_ripple: map[id:Stt9f7EdjY-LMXtzT89XNojIPtDAEcNrPKtkqP2N2E-StVdJGL3fWy4TBTnU7jotBSxK0Vydn132cMezA8vMFA name:map[type:String value:Product 1] price:map[type:Integer value:150]]
Deleted with bypass_ripple
Batch inserted with bypass_ripple: 2 records
Upserted with bypass_ripple: map[id:custom-id]
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

=== Cleanup ===
✓ Cleaned up test keys

=== Summary ===
✅ Use FieldDecimal() for monetary values, percentages, and
   any case where floating-point errors are unacceptable.
✅ FieldDecimal() stores values as strings internally,
   preserving exact precision across all operations.
✓ Client created

=== Get All Chat Models ===
OpenAI models: [gpt-4-0613 gpt-4 gpt-3.5-turbo gpt-5.4-mini gpt-5.4 gpt-5.4-nano-2026-03-17 gpt-5.4-nano gpt-5.4-mini-2026-03-17 davinci-002 babbage-002 gpt-3.5-turbo-instruct gpt-3.5-turbo-instruct-0914 dall-e-3 dall-e-2 gpt-3.5-turbo-1106 tts-1-hd tts-1-1106 tts-1-hd-1106 text-embedding-3-small text-embedding-3-large gpt-3.5-turbo-0125 gpt-4-turbo gpt-4-turbo-2024-04-09 gpt-4o gpt-4o-2024-05-13 gpt-4o-mini-2024-07-18 gpt-4o-mini gpt-4o-2024-08-06 gpt-4o-audio-preview gpt-4o-realtime-preview omni-moderation-latest omni-moderation-2024-09-26 gpt-4o-realtime-preview-2024-12-17 gpt-4o-audio-preview-2024-12-17 gpt-4o-mini-realtime-preview-2024-12-17 gpt-4o-mini-audio-preview-2024-12-17 o1-2024-12-17 o1 gpt-4o-mini-realtime-preview gpt-4o-mini-audio-preview o3-mini o3-mini-2025-01-31 gpt-4o-2024-11-20 gpt-4o-mini-search-preview-2025-03-11 gpt-4o-mini-search-preview gpt-4o-transcribe gpt-4o-mini-transcribe o1-pro-2025-03-19 o1-pro gpt-4o-mini-tts o3-2025-04-16 o4-mini-2025-04-16 o3 o4-mini gpt-4.1-2025-04-14 gpt-4.1 gpt-4.1-mini-2025-04-14 gpt-4.1-mini gpt-4.1-nano-2025-04-14 gpt-4.1-nano gpt-image-1 gpt-4o-realtime-preview-2025-06-03 gpt-4o-audio-preview-2025-06-03 o4-mini-deep-research gpt-4o-transcribe-diarize o4-mini-deep-research-2025-06-26 gpt-5-chat-latest gpt-5-2025-08-07 gpt-5 gpt-5-mini-2025-08-07 gpt-5-mini gpt-5-nano-2025-08-07 gpt-5-nano gpt-audio-2025-08-28 gpt-realtime gpt-realtime-2025-08-28 gpt-audio gpt-5-codex gpt-image-1-mini gpt-5-pro-2025-10-06 gpt-5-pro gpt-audio-mini gpt-audio-mini-2025-10-06 gpt-5-search-api gpt-realtime-mini gpt-realtime-mini-2025-10-06 sora-2 sora-2-pro gpt-5-search-api-2025-10-14 gpt-5.1-chat-latest gpt-5.1-2025-11-13 gpt-5.1 gpt-5.1-codex gpt-5.1-codex-mini gpt-5.1-codex-max gpt-image-1.5 gpt-5.2-2025-12-11 gpt-5.2 gpt-5.2-pro-2025-12-11 gpt-5.2-pro gpt-5.2-chat-latest gpt-4o-mini-transcribe-2025-12-15 gpt-4o-mini-transcribe-2025-03-20 gpt-4o-mini-tts-2025-03-20 gpt-4o-mini-tts-2025-12-15 gpt-realtime-mini-2025-12-15 gpt-audio-mini-2025-12-15 chatgpt-image-latest gpt-5.2-codex gpt-5.3-codex gpt-realtime-1.5 gpt-audio-1.5 gpt-4o-search-preview gpt-4o-search-preview-2025-03-11 gpt-5.3-chat-latest gpt-5.4-2026-03-05 gpt-5.4-pro gpt-5.4-pro-2026-03-05 gpt-3.5-turbo-16k tts-1 whisper-1 text-embedding-ada-002]
Anthropic models: [claude-sonnet-4-6 claude-opus-4-6 claude-opus-4-5-20251101 claude-haiku-4-5-20251001 claude-sonnet-4-5-20250929 claude-opus-4-1-20250805 claude-opus-4-20250514 claude-sonnet-4-20250514 claude-3-haiku-20240307]
Perplexity models: [sonar sonar-pro sonar-deep-research sonar-reasoning sonar-reasoning-pro]

=== Get OpenAI Models ===
OpenAI models: [gpt-4-0613 gpt-4 gpt-3.5-turbo gpt-5.4-mini gpt-5.4 gpt-5.4-nano-2026-03-17 gpt-5.4-nano gpt-5.4-mini-2026-03-17 davinci-002 babbage-002 gpt-3.5-turbo-instruct gpt-3.5-turbo-instruct-0914 dall-e-3 dall-e-2 gpt-3.5-turbo-1106 tts-1-hd tts-1-1106 tts-1-hd-1106 text-embedding-3-small text-embedding-3-large gpt-3.5-turbo-0125 gpt-4-turbo gpt-4-turbo-2024-04-09 gpt-4o gpt-4o-2024-05-13 gpt-4o-mini-2024-07-18 gpt-4o-mini gpt-4o-2024-08-06 gpt-4o-audio-preview gpt-4o-realtime-preview omni-moderation-latest omni-moderation-2024-09-26 gpt-4o-realtime-preview-2024-12-17 gpt-4o-audio-preview-2024-12-17 gpt-4o-mini-realtime-preview-2024-12-17 gpt-4o-mini-audio-preview-2024-12-17 o1-2024-12-17 o1 gpt-4o-mini-realtime-preview gpt-4o-mini-audio-preview o3-mini o3-mini-2025-01-31 gpt-4o-2024-11-20 gpt-4o-mini-search-preview-2025-03-11 gpt-4o-mini-search-preview gpt-4o-transcribe gpt-4o-mini-transcribe o1-pro-2025-03-19 o1-pro gpt-4o-mini-tts o3-2025-04-16 o4-mini-2025-04-16 o3 o4-mini gpt-4.1-2025-04-14 gpt-4.1 gpt-4.1-mini-2025-04-14 gpt-4.1-mini gpt-4.1-nano-2025-04-14 gpt-4.1-nano gpt-image-1 gpt-4o-realtime-preview-2025-06-03 gpt-4o-audio-preview-2025-06-03 o4-mini-deep-research gpt-4o-transcribe-diarize o4-mini-deep-research-2025-06-26 gpt-5-chat-latest gpt-5-2025-08-07 gpt-5 gpt-5-mini-2025-08-07 gpt-5-mini gpt-5-nano-2025-08-07 gpt-5-nano gpt-audio-2025-08-28 gpt-realtime gpt-realtime-2025-08-28 gpt-audio gpt-5-codex gpt-image-1-mini gpt-5-pro-2025-10-06 gpt-5-pro gpt-audio-mini gpt-audio-mini-2025-10-06 gpt-5-search-api gpt-realtime-mini gpt-realtime-mini-2025-10-06 sora-2 sora-2-pro gpt-5-search-api-2025-10-14 gpt-5.1-chat-latest gpt-5.1-2025-11-13 gpt-5.1 gpt-5.1-codex gpt-5.1-codex-mini gpt-5.1-codex-max gpt-image-1.5 gpt-5.2-2025-12-11 gpt-5.2 gpt-5.2-pro-2025-12-11 gpt-5.2-pro gpt-5.2-chat-latest gpt-4o-mini-transcribe-2025-12-15 gpt-4o-mini-transcribe-2025-03-20 gpt-4o-mini-tts-2025-03-20 gpt-4o-mini-tts-2025-12-15 gpt-realtime-mini-2025-12-15 gpt-audio-mini-2025-12-15 chatgpt-image-latest gpt-5.2-codex gpt-5.3-codex gpt-realtime-1.5 gpt-audio-1.5 gpt-4o-search-preview gpt-4o-search-preview-2025-03-11 gpt-5.3-chat-latest gpt-5.4-2026-03-05 gpt-5.4-pro gpt-5.4-pro-2026-03-05 gpt-3.5-turbo-16k tts-1 whisper-1 text-embedding-ada-002]

=== Get Anthropic Models ===
Anthropic models: [claude-sonnet-4-6 claude-opus-4-6 claude-opus-4-5-20251101 claude-haiku-4-5-20251001 claude-sonnet-4-5-20250929 claude-opus-4-1-20250805 claude-opus-4-20250514 claude-sonnet-4-20250514 claude-3-haiku-20240307]

✓ Chat Models API example complete
✓ Client created

=== Create User Function ===
Created user function with ID: Ufok1iSCrAngL_kbcqZYI0UkDhlnb-8hyhMUkfT8BD6GtvtPOZVxSh-tCNmT5W-HMejcnXRromwHP7Irs1vX6Q

=== Get User Function ===
Retrieved: get_active_users - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 19 user functions:
  - fetch_and_store_user: Fetch user from API and cache in KV
  - swr_user: SWR pattern for user data (KV-based)
  - fetch_slim_user: Validate and slim down user
  - get_verified_user: Get verified and validated user
  - cache_api_call_py: Cache External API Call
  - fetch_slim_user: Validate and slim down user
  - validate_user: Check if user exists
  - get_user_wrapper: Wrapper that calls fetch_user
  - validate_user: Check if user exists
  - swr_user: SWR pattern for user data (KV-based)
  - fetch_user: Fetch user by code
  - validate_user: Check if user exists
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_slim_user: Validate and slim down user
  - fetch_user: Fetch user by code
  - swr_user: SWR pattern for user data (KV-based)
  - fetch_user: Fetch user by code
  - get_active_users: Get Active Users
  - fetch_and_store_user: Fetch user from API and cache in KV

=== List User Functions by Tag ===
Found 19 user functions with 'users' tag:
  - fetch_and_store_user
  - swr_user
  - fetch_slim_user
  - get_verified_user
  - cache_api_call_py
  - fetch_slim_user
  - validate_user
  - get_user_wrapper
  - validate_user
  - swr_user
  - fetch_user
  - validate_user
  - fetch_and_store_user
  - fetch_slim_user
  - fetch_user
  - swr_user
  - fetch_user
  - get_active_users
  - fetch_and_store_user

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
✓ Client created

=== Check Collection Exists (Before Creation) ===
Collection 'collection_utils_test' exists: false

=== Creating Test Documents ===
Created 5 test documents

=== Check Collection Exists (After Creation) ===
Collection 'collection_utils_test' exists: true

=== Count Documents ===
Document count in 'collection_utils_test': 5

=== Check Non-Existent Collection ===
Collection 'nonexistent_collection_xyz' exists: false

=== Cleanup ===
Deleted collection 'collection_utils_test'

✓ Collection Utilities example complete
✅ [32mGo client examples complete![0m
✅ [32mAll Go integration tests complete![0m
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.16.0 prepare
> npm run build


> @ekodb/ekodb-client@0.16.0 build
> tsc


up to date, audited 52 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.16.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning TypeScript client library examples...[0m

added 1 package, removed 1 package, and audited 15 packages in 974ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'hcXPpSrz8zAYC0TACmcnuJsmtmOzBpQgq-qCKkewP1SpYUljKPa_8JrpQAj33VBrAtOO1UACSp3fj5sxk6zI5A'
}

=== Find by ID ===
Found: {
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  name: { type: 'String', value: 'Test Record' },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  active: { value: true, type: 'Boolean' },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  price: { type: 'Float', value: 99.99 },
  value: { type: 'Integer', value: 42 },
  id: 'hcXPpSrz8zAYC0TACmcnuJsmtmOzBpQgq-qCKkewP1SpYUljKPa_8JrpQAj33VBrAtOO1UACSp3fj5sxk6zI5A',
  created_at: { type: 'DateTime', value: '2026-04-12T16:44:40.587Z' },
  metadata: { type: 'Object', value: { nested: [Object], key: 'value' } },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-04-12T16:44:40.587Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { nested: { deep: true }, key: 'value' }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  tags: [ 'tag1', 'tag2', 'tag3' ],
  name: 'Test Record',
  data: 'aGVsbG8gd29ybGQ=',
  categories: [ 'electronics', 'computers' ],
  active: true,
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  price: 99.99,
  value: 42,
  id: 'hcXPpSrz8zAYC0TACmcnuJsmtmOzBpQgq-qCKkewP1SpYUljKPa_8JrpQAj33VBrAtOO1UACSp3fj5sxk6zI5A',
  created_at: '2026-04-12T16:44:40.587Z',
  metadata: { nested: { deep: true }, key: 'value' },
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  id: 'hcXPpSrz8zAYC0TACmcnuJsmtmOzBpQgq-qCKkewP1SpYUljKPa_8JrpQAj33VBrAtOO1UACSp3fj5sxk6zI5A',
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  created_at: { type: 'DateTime', value: '2026-04-12T16:44:40.587Z' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  value: { type: 'Integer', value: 100 },
  active: { value: true, type: 'Boolean' },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  price: { value: 99.99, type: 'Float' },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  metadata: { value: { nested: [Object], key: 'value' }, type: 'Object' },
  name: { type: 'String', value: 'Updated Record' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: yJyFQ7qbk9hTGNG-w94oys-XK1qzCv4-EZpRCZrDELnCL-CHzWl0mAFgJW2lMRAoqvft2EFSRsPMzE07SL3pyQ

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
Collection created with first record: o05m13fwXNTvXX33bSFLJY0bAhCW-9JHkyxq7qQEjhSlbU9PdJgEi3QG05MjApU39w5a9NQmKfrApWE5YPeWJQ

=== List Collections ===
Total collections: 22
Sample collections: schema_users_client_py,test_collection,users,client_collection_management_ts,schema_users_client_go

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

=== KV Batch Set ===
✓ Batch set 3 keys
  cache:product:1: success
  cache:product:2: success
  cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  cache:product:1: { price: 29.99, name: 'Product 1' }
  cache:product:2: { name: 'Product 2', price: 39.99 }
  cache:product:3: { name: 'Product 3', price: 49.99 }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 12

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
Created Alice: $1000 - ID: HRAgqO6tuWSsiUIKkgYLs1VuhqejXN9fQYqtCcEb2jng2kUzehCJusvAjaNqQzKZ6ReafbhXvdyDPRa2O_iiRw
Created Bob: $500 - ID: MQEwBig-qkgTPj9NYANlkY8jP42yMPoFc5lgYxOQ5wEy4myQiTH-NWch8S1lOk4cva3FapI_CeZUxsDlPPEKjQ

=== Example 1: Begin Transaction ===
Transaction ID: 120f201e-92cb-4283-b60b-bde442b54081

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: b420b001-179c-4a91-8a5b-ab50cca0bfe0
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
  1. Score: 25.740, Matched: name.value, email, email.value, name
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title, title.value, bio.value, bio
  2. Score: 26.400, Matched: title, title.value, bio.value, bio
  3. Score: 26.400, Matched: title, bio.value, title.value, bio
  4. Score: 26.400, Matched: title.value, bio.value, bio, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio.value, title, title.value, bio
  2. Score: 39.600, Matched: bio, bio.value, title.value, title
  3. Score: 39.600, Matched: bio.value, bio, title, title.value
  4. Score: 39.600, Matched: title.value, bio, bio.value, title

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.770, Matched: 
  2. Score: 0.740, Matched: 
  3. Score: 0.733, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.696, Matched: title.value, content.value, title, content
  2. Score: 1.493, Matched: content.value, title, title.value, content
  3. Score: 0.308, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: bio, skills, bio.value, skills.value

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
  - Bob Smith: Sales
  - Alice Johnson: Engineering

2. Join with filtering:
Found 1 users in Engineering:
  - Alice Johnson: Building A

3. Join with user profiles:
Found 2 users with profile data:
  - Bob Smith: Sales Manager
  - Alice Johnson: Senior Software Engineer

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
✓ Inserted document: f0wGGRsrbKwpdrfNnlnfEIrbqGWyadlMMH0nXTiep9iG_j5k9nOUgw35y6028X4c9u2ZZApepv-0torADPf9ig

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: CWvhsfA-YcflG8O7Y3osz_nqihPH8ULarwTtPHXJh_sTOtet1GiPDNVE3gT3UMh0Q-IrA-dPF_IwSCMSqM_j3A

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
✓ Inserted document with TTL: RLc1sDOWevb0CKTprg8mItvMA1OYrpUHM-vN6rnX1NinlnVMrSPL0S4JtorLnafpQqmoQEEn3QZkF6v_k7YZ5A

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
=== ekoDB as Edge Cache - Simple Example ===

Creating edge cache function...
✓ Edge cache script created: 9puGBOOBFhJk9D9xxSr0LL76ongM7dVWnZrr0wxid912iUDxGQRV_5dvXW8VcrfWv2-AIUFslLOo2uBrgzbW0Q

Call 1: Cache miss (fetches from API)
Response time: 144ms
Result: {
  "records": [
    {
      "value": {
        "website": "hildegard.org",
        "username": "Bret",
        "company": {
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets",
          "catchPhrase": "Multi-layered client-server neural-net"
        },
        "id": 1,
        "phone": "1-770-736-8031 x56442",
        "address": {
          "geo": {
            "lng": "81.1496",
            "lat": "-37.3159"
          },
          "street": "Kulas Light",
          "suite": "Apt. 556",
          "zipcode": "92998-3874",
          "city": "Gwenborough"
        },
        "name": "Leanne Graham",
        "email": "Sincere@april.biz"
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
Response time: 41ms (3.5x faster!)
Result: {
  "records": [
    {
      "value": {
        "website": "hildegard.org",
        "username": "Bret",
        "company": {
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets",
          "catchPhrase": "Multi-layered client-server neural-net"
        },
        "id": 1,
        "phone": "1-770-736-8031 x56442",
        "address": {
          "geo": {
            "lng": "81.1496",
            "lat": "-37.3159"
          },
          "street": "Kulas Light",
          "suite": "Apt. 556",
          "zipcode": "92998-3874",
          "city": "Gwenborough"
        },
        "name": "Leanne Graham",
        "email": "Sincere@april.biz"
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

🚀 ekoDB Functions Example (TypeScript)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: 9J5cZixeHep7dkFdou0xXsoR1xgicqbD3ukrEaiGhdjqVAp7CSa6LdZt3go89sflAtePS0Qnmx4Dn47pQxMrhQ
📊 Found 15 active users

📝 Example 2: Parameterized Function

✅ Function saved: 7fBZUyOjI3Grex4rst1PtJTaicqCTS6Wr6cGL_vBUXixTW2wqPdr4VYOKXg5DpBFfFIyypGyyT4v00bimEj36g
📊 Found 15 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: dbeRJxIZyznFQEiezXQ-ygRy0NsA_dRthdRB6J9tNIE3FZnLuU2Aydyeao7MLSHXd1pzDNCZO8fNS3u_G83Ifg
📊 Statistics: 3 groups
   {"count":5,"avg_score":60,"status":"active"}
   {"status":"null","avg_score":60,"count":5}
   {"status":"inactive","count":5,"avg_score":50}

📝 Example 4: UserFunction Management

📋 Total scripts: 22
🔍 Retrieved script: Get Active Users
✏️  function updated
🗑️  function deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed!
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
   ⏱️  Duration: 38ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "company": {
      "bs": "harness real-time e-markets",
      "catchPhrase": "Multi-layered client-server neural-net",
      "name": "Romaguera-Crona"
    },
    "email": "Sincere@a...

Second call (cache hit - from cache):
   ⏱️  Duration: 38ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "company": {
      "bs": "harness real-time e-markets",
      "catchPhrase": "Multi-layered client-server neural-net",
      "name": "Romaguera-Crona"
    },
    "email": "Sincere@a...
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
🚀 ekoDB TypeScript Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: LykUNrgfwFKss-TQ1Nz2K7-xShYSwjlu951HckfLYUHJCjdvaOCjzcEHFP5ZAmSILYOig83BT7cipC_pzfGDYw
📊 Found 2 product groups
   {"category":"Furniture","count":2,"avg_price":474}
   {"avg_price":575.6666666666666,"category":"Electronics","count":3}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"count":2,"category":"Furniture"}
   {"category":"Electronics","count":3}
⏱️  Execution time: 0ms

📝 Example 4: High Rating Products

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 5: UserFunction with Parameter Definition

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 6: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Function saved
📊 Pipeline executed 3 stages
⏱️  Total execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All complete script examples finished!

💡 This example demonstrates ekoDB's function system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Parameter definitions
   ✅ Function management (save, call, delete)
🚀 ekoDB TypeScript KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: hZXYlkgZmeluv0_TqKJEbwcoZ8ty94f_8vyv1nqYkHjw3oOz9vWI1s9DtFX967PXhSlwuTJzZ4guXdyvRl06bg
✅ Inserted 2 products with wrapped types

📝 Example 2: UserFunction with Wrapped Type Parameters

✅ Function saved: 6sR6AkgkRRmayrYMH0jT7QKsnf-GuUuMpE75EvLMdTsYzEsE87_iqRTDKRsTnVrOFMMVjkRUY-juVXT9s7g68w
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: CIEi4OytfrDKRsimoyyEpCsLouEOPaNUVqFvIQAxxArdQv_NFQX8WvvPrCsO12EPmLXr7kcPdkna2vaV5WAI-g
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: hZQqBtmdYAf0wZX5AAHmZw6KheC2kKf5z2j5YJdAyxynoiU1Zsxg8hvSpdyAdtCfDbB4xHgX7kRyAjaGFOMYpQ
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
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user (MqI874Dyx-d2rGRcMojFERR4k83OFH90HWPVO1ObyFmlGSljAS3kSuH2onei99bt1QkSwmGX8wQokFoMZMZ4gg)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "id": 1,
        "phone": "1-770-736-8031 x56442",
        "email": "Sincere@april.biz",
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "bs": "harness real-time e-markets",
          "name": "Romaguera-Crona"
        },
        "name": "Leanne Graham",
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
        "username": "Bret",
        "website": "hildegard.org"
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
Result (cached): {
  "records": [
    {
      "value": {
        "id": 1,
        "phone": "1-770-736-8031 x56442",
        "email": "Sincere@april.biz",
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "bs": "harness real-time e-markets",
          "name": "Romaguera-Crona"
        },
        "name": "Leanne Graham",
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
        "username": "Bret",
        "website": "hildegard.org"
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
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_with_reviews (4LvuR4H773aHt_R31BUbhliN41SUvzKnRFRjG1_K6Jv4fnSqyWzOTEJmuIBbrWxxiNnXT39p5F2Us26oayTa_Q)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "discountPercentage": 10.48,
        "returnPolicy": "No return policy",
        "price": 9.99,
        "reviews": [
          {
            "reviewerName": "Eleanor Collins",
            "comment": "Would not recommend!",
            "rating": 3,
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "date": "2025-04-30T09:41:02.053Z"
          },
          {
            "reviewerEmail": "lucas.gordon@x.dummyjson.com",
            "comment": "Very satisfied!",
            "date": "2025-04-30T09:41:02.053Z",
            "reviewerName": "Lucas Gordon",
            "rating": 4
          },
          {
            "comment": "Highly impressed!",
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "reviewerName": "Eleanor Collins",
            "rating": 5,
            "date": "2025-04-30T09:41:02.053Z"
          }
        ],
        "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
        "dimensions": {
          "depth": 22.99,
          "height": 13.08,
          "width": 15.14
        },
        "brand": "Essence",
        "images": [
          "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
        ],
        "meta": {
          "createdAt": "2025-04-30T09:41:02.053Z",
          "barcode": "5784719087687",
          "updatedAt": "2025-04-30T09:41:02.053Z",
          "qrCode": "https://cdn.dummyjson.com/public/qr-code.png"
        },
        "stock": 99,
        "availabilityStatus": "In Stock",
        "rating": 2.56,
        "weight": 4,
        "minimumOrderQuantity": 48,
        "title": "Essence Mascara Lash Princess",
        "tags": [
          "beauty",
          "mascara"
        ],
        "id": 1,
        "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
        "warrantyInformation": "1 week warranty",
        "sku": "BEA-ESS-ESS-001",
        "shippingInformation": "Ships in 3-5 business days",
        "category": "beauty"
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
✓ Multi-API data fetched, merged, and cached atomically

=== Why This Is Powerful ===

✓ No separate cache layer (Redis, Memcached) needed
✓ No manual cache invalidation (TTL handles it)
✓ No separate edge infrastructure (ekoDB IS the edge)
✓ Atomic operations (function executes as transaction)
✓ With multi-node + ripples: Auto-sync across all nodes
✓ Sub-millisecond cache hits from internal storage
✓ One service instead of many (cache + API gateway + database)

=== Real-World Use Cases ===

1. API Gateway Pattern:
   - Client → ekoDB Function → Check cache → Call microservices → Merge → Cache

2. Database Federation:
   - Query multiple DBs (Postgres, MongoDB) + external APIs
   - Merge results in one function call

3. IoT Data Enrichment:
   - Sensor data + weather API + location API
   - Enrich and cache in one atomic operation

4. E-commerce Product Pages:
   - Product info + reviews + inventory + pricing
   - All from different sources, cached together

✓ Example complete - Your database IS your edge!

🚀 ekoDB TypeScript Client - Native SWR Function Examples

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
✓ Created native SWR script: github_user_native (AlxOpgbnopFdHtFUZONi8uxwCglcfk49VSN6LxTvjDceTFWVVK2pjgoDoqRuhrUP2Q1DEq3LzOFmR5Q0h4fMSw)

First call (cache miss - will fetch from GitHub API):
  Response time: 16ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 16ms
  Speedup: 1.0x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (j_J0e49UYABM1xme2CB5AuSGRQrUg-9gLSftPQ7BjY7ZgC9YdUcxFIF9ITuK1EDYOdN7PhzkBjalx1qJKiLaFg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (R2mz7UFJQVGIZY8wG6WCbbD5QZ3k8FEV-k01_2J5p5Pt3afYwzrlwq1VpzNd9Lj9TLUq2DNH4LMaVk7wbChjOw)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (YTbRragPSDBAkE2xFE828wcrasZQbBp6TYTmRmpQqxnI9FcDQP99vXtYAYY0qsXpniaqZusDFa4XASPncfsN5w)
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
🚀 ekoDB TypeScript Advanced Functions Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Function saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Function saved
📊 Category breakdown:
   {"avg_price":365.6666666666667,"count":3,"category":"Furniture"}
   {"avg_price":367,"category":"Electronics","count":5}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
🚀 ekoDB TypeScript AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer several benefits:

1. **Efficient Similarity Search**: They excel in searching high-dimensional data for similar items, useful for applications like recommendation systems and image retrieval.

2. **Handling Unstructured Data**: Vector databases effectively manage unstructured data such as text, images, and videos by converting them into vector representations.

3. **Scalability**: Many vector databases are designed to scale horizontally, accommodating large datasets and high query loads.

4. **Performance Optimization**: They use specialized indexing techniques (like HNSW or Annoy) for faster retrieval times, even in high-dimensional spaces.

5. **Integration with Machine Learning**: Vector databases support seamless integration with ML models, enabling real-time analytics and inference.

6. **Flexibility**: They can store and query multiple data types, making them suitable for various applications across industries.

7. **Advanced Query Capabilities**: Besides nearest neighbor searches, they allow complex queries involving multiple vector operations. 

These benefits make vector databases particularly well-suited for use cases in AI and data-driven applications.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embed script saved
📊 Embedding generated
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All AI script examples finished!

💡 This example demonstrates:
   ✅ Chat completions with system/user messages
   ✅ Embedding generation for text
🚀 ekoDB TypeScript CRUD Functions Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Function saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Function saved
📊 User counts by status:
   inactive: 3 users
   active: 7 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Function saved
📊 Average score by role:
   {"role":"admin","count":3,"avg_score":20}
   {"role":"user","avg_score":70,"count":7}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
🚀 ekoDB TypeScript Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Vector Databases Explained (Database)
   2. Database Design Principles (Database)
   3. Natural Language Processing (AI)
   4. Getting Started with ekoDB (Database)
   5. Introduction to Machine Learning (AI)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"category":"Database","count":3}
   {"category":"AI","count":2}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: Wq0FBjdjyZFL5dvEiXXh9oSzxml2MaulqyBAArknQ-YjhUBicp2wm_7t2j_4Jo6SfUpQdqr9EH0zvA7Vi4vvWA

=== Sending Chat Message ===
Message ID: IpwOQ-f3VKX_tVBRDq9R2yW6zXLkLdaWt8fJonDeWPHxfrDaf_KSkAIKltWh76rHqHziGogaLB6FAlC8c6-V1A

=== AI Response ===
Here are the available products along with their prices:

1. **ekoDB Cloud**
   - Price: $499
   - Description: Fully managed cloud database service product

2. **ekoDB Pro**
   - Price: $299
   - Description: Enterprise edition product with advanced features

3. **ekoDB**
   - Price: $99
   - Description: A high-performance database product with AI capabilities

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'PuRK4oJGdIo7Odq-mSRoFwMeZ-eE5XClfylVPOV3wM74zlh2JBphzc5eU_Z3NE2Z1WwTQlF33eJuE3Uum2ZIiw',
    description: 'Fully managed cloud database service product',
    name: 'ekoDB Cloud',
    price: 499
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'NADVBOkx3SUKMM3sizICfZRntt2bA2Wt1l5ilHNHyh-uDNDh6dpez8rynaTGh4L_DGzUS5Lp_x7UknJladcOZw',
    price: 299,
    description: 'Enterprise edition product with advanced features',
    name: 'ekoDB Pro'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB',
    price: 99,
    id: 'S49oIm8diksFMoXfLwsLVJG2RoqIxcYtBma_fC3eVV_8MhwOyassKz6mBVnsi0IC2JkBebqcYc_4UMfdOOutKQ',
    description: 'A high-performance database product with AI capabilities'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 8618ms

=== Token Usage ===
Prompt tokens: 1298
Completion tokens: 91
Total tokens: 1389

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 4H6V51Ufkzt0771BZSMO_VMEETFCLHsOAk4D55ccRESOra05RGofboOpw-0xMKL0ZEz_q3kN-VQcY3Y3Hxq_hg

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you'd like to know more or have any other questions, feel free to ask!

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you have any more questions or need further assistance, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: sB-siCOBQMoeDbbZGWMpxBRbjV56-5IaFPUO_KWVohEzM2ZhO_fe7kEzGbHZSSkKCob-yIJYdQBhnRMJkRT54w
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
✓ Created session: oXyNlcmlgi1EJFwRbqJFi4_amcde50qpGVi13cHaHKVRLpr3h-YuqugcpR8JUdfSnb_FhzTvnWquljdcqIlFRA

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name:** ekoDB  
  **Description:** A high-performance database product  
  **Price:** $99  

If you have any more questions or need additional information, feel free to ask!

✓ Message 2 sent
  Response: The price of the ekoDB product is **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: _C8xuhNOfyRDQ7Blx0FCOMUz_PMSO2wpLrCfgWtyu-jZNPo3Ne0g4FqO97tGQPCtvrkYpHTBRoabH8iEZMJNTw
  Parent: oXyNlcmlgi1EJFwRbqJFi4_amcde50qpGVi13cHaHKVRLpr3h-YuqugcpR8JUdfSnb_FhzTvnWquljdcqIlFRA

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: _C8xuhNOfyRDQ7Blx0FCOMUz_PMSO2wpLrCfgWtyu-jZNPo3Ne0g4FqO97tGQPCtvrkYpHTBRoabH8iEZMJNTw (Untitled)
  Session 2: oXyNlcmlgi1EJFwRbqJFi4_amcde50qpGVi13cHaHKVRLpr3h-YuqugcpR8JUdfSnb_FhzTvnWquljdcqIlFRA (Untitled)
  Session 3: sB-siCOBQMoeDbbZGWMpxBRbjV56-5IaFPUO_KWVohEzM2ZhO_fe7kEzGbHZSSkKCob-yIJYdQBhnRMJkRT54w (Untitled)
  Session 4: Wq0FBjdjyZFL5dvEiXXh9oSzxml2MaulqyBAArknQ-YjhUBicp2wm_7t2j_4Jo6SfUpQdqr9EH0zvA7Vi4vvWA (Untitled)
  Session 5: Znp6mUA8YF5IZMlJ_Gi5RO5IK4Csu0uLWqmdO2yqmd_ANoxYa250sKiTx-bmvvu2SIgeuzVldlKiXpUYweOW_g (Untitled)
  Session 6: Be84Ao5KLBzjK4zjM0uyi7WHiFGkgnGiBz913zjhkQrGrZn6A3j7uDi1wNEsih7W6bTQ2aTuD5OcL0gfaQQrvQ (Untitled)
  Session 7: 32PFek8QtgY3SxhW6Q7KVWSbd7uQimZbD3OOUBIeM8mjW6NtmYSx4TqnDEaMk5EP30UDXloORbnCr2umJk9wrw (Untitled)
  Session 8: yjDtt3Myn4J9VCsJl1q64BwCSIek6DgKpLdOUgwWzwGL8ht9XgjOAZSpGsomKCMrt42gOaiH59G-lArj1Zj4Mg (Untitled)
  Session 9: 5qaltAlAg_c0xW2xSTM4Las-hGRQUIRoLBisgrFpRUkeQd8WdOmlaAueAj-04BaV2RHQ5oroV5f16673NJTdjQ (Untitled)
  Session 10: hqgZ7u1n4z1PjYSwN3mGKkYDjbJgw-JhqI7U6wQhwAaSmQMFswL4loRxhLaW_4sKaj0SpDoJPx-rNtOdQ8yMWw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: _C8xuhNOfyRDQ7Blx0FCOMUz_PMSO2wpLrCfgWtyu-jZNPo3Ne0g4FqO97tGQPCtvrkYpHTBRoabH8iEZMJNTw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'wJmcMI1raCyUNzf1MaYjswusSEAD0wuT1yaft2a9e1evvI4BrESvMXU19EWQIwjPvwuZo1728WkjspV35K0C5w'
}

=== Upsert Operation ===
✓ First upsert (update): {
  id: 'wJmcMI1raCyUNzf1MaYjswusSEAD0wuT1yaft2a9e1evvI4BrESvMXU19EWQIwjPvwuZo1728WkjspV35K0C5w',
  active: { type: 'Boolean', value: true },
  age: { type: 'Integer', value: 29 },
  name: { value: 'Alice Johnson', type: 'String' },
  email: { type: 'String', value: 'alice.j@newdomain.com' }
}
✓ Second upsert (insert): {
  id: 'jQRp8Xy6FFc0gT5QCCYFKfUi3XSJtR0AJgnhcGCAJi64AUpq4CC9edM-1TEylXpVDxYsQ7IO6YTXlRE-9uOWag'
}

=== Find One Operation ===
✓ Found user by email: {
  active: { value: true, type: 'Boolean' },
  email: { value: 'alice.j@newdomain.com', type: 'String' },
  name: { type: 'String', value: 'Alice Johnson' },
  id: 'wJmcMI1raCyUNzf1MaYjswusSEAD0wuT1yaft2a9e1evvI4BrESvMXU19EWQIwjPvwuZo1728WkjspV35K0C5w',
  age: { type: 'Integer', value: 29 }
}
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
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: {"id":"Oo0_UgKl-rlf4kpD233u-oV5d9nrnytOyzPSCU471mkPz0QfNOmYYAKNUrMVxOE1DkRl1mkfAStcRNgXtAHU2Q"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"--EKiUw91yvsN-rqlMEKgQqZ4VeVQj7r9Hvb5yZPN6QUdtgvEouAQspU3bgoajL8b3fnKGuq5nBqnoYmwaieyg"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"name":{"type":"String","value":"Product 1"},"price":{"type":"Integer","value":150},"id":"Oo0_UgKl-rlf4kpD233u-oV5d9nrnytOyzPSCU471mkPz0QfNOmYYAKNUrMVxOE1DkRl1mkfAStcRNgXtAHU2Q"}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"name":{"type":"String","value":"Upsert Product"},"id":"custom-id","price":{"type":"Integer","value":500}}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ["id","name","email"]
  First user: [object Object] <[object Object]>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["name","bio","id","user_role","created_at","status","age","email","avatar_url"]

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - [object Object] (age [object Object])
    - [object Object] (age [object Object])
    - [object Object] (age [object Object])

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - [object Object]: [object Object]

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ["email","api_key","secret_token","id","age","status","password","created_at","user_role","bio","name","avatar_url"]
  Projected query:
    - 3 fields per record
    - Fields: ["id","name","email"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
=== KV Precision: Float vs Decimal ===

=== Test 1: Using JavaScript Floats (LOSES PRECISION) ===
✓ Stored products with float prices

Retrieved float prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 2: Using Field.decimal() (PRESERVES PRECISION) ===
✓ Stored products with decimal prices

Retrieved decimal prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 3: Sum Calculation Comparison ===
  Float sum: $119.97 (expected $119.97)
  Decimal sum: $119.97 (expected $119.97)

=== Test 4: Extreme Precision Example ===
  Float 0.1 + 0.2 = 0.30000000000000004 (should be 0.3)
  Decimal "0.30" = 0.30 (exact!)

=== Cleanup ===
✓ Cleaned up test keys

=== Summary ===
✅ Use Field.decimal() for monetary values, percentages, and
   any case where floating-point errors are unacceptable.
✅ Field.decimal() stores values as strings internally,
   preserving exact precision across all operations.
✓ Client created

=== List Chat Models ===
Available chat models by provider:
  openai:
    - gpt-4-0613
    - gpt-4
    - gpt-3.5-turbo
    - gpt-5.4-mini
    - gpt-5.4
    - gpt-5.4-nano-2026-03-17
    - gpt-5.4-nano
    - gpt-5.4-mini-2026-03-17
    - davinci-002
    - babbage-002
    - gpt-3.5-turbo-instruct
    - gpt-3.5-turbo-instruct-0914
    - dall-e-3
    - dall-e-2
    - gpt-3.5-turbo-1106
    - tts-1-hd
    - tts-1-1106
    - tts-1-hd-1106
    - text-embedding-3-small
    - text-embedding-3-large
    - gpt-3.5-turbo-0125
    - gpt-4-turbo
    - gpt-4-turbo-2024-04-09
    - gpt-4o
    - gpt-4o-2024-05-13
    - gpt-4o-mini-2024-07-18
    - gpt-4o-mini
    - gpt-4o-2024-08-06
    - gpt-4o-audio-preview
    - gpt-4o-realtime-preview
    - omni-moderation-latest
    - omni-moderation-2024-09-26
    - gpt-4o-realtime-preview-2024-12-17
    - gpt-4o-audio-preview-2024-12-17
    - gpt-4o-mini-realtime-preview-2024-12-17
    - gpt-4o-mini-audio-preview-2024-12-17
    - o1-2024-12-17
    - o1
    - gpt-4o-mini-realtime-preview
    - gpt-4o-mini-audio-preview
    - o3-mini
    - o3-mini-2025-01-31
    - gpt-4o-2024-11-20
    - gpt-4o-mini-search-preview-2025-03-11
    - gpt-4o-mini-search-preview
    - gpt-4o-transcribe
    - gpt-4o-mini-transcribe
    - o1-pro-2025-03-19
    - o1-pro
    - gpt-4o-mini-tts
    - o3-2025-04-16
    - o4-mini-2025-04-16
    - o3
    - o4-mini
    - gpt-4.1-2025-04-14
    - gpt-4.1
    - gpt-4.1-mini-2025-04-14
    - gpt-4.1-mini
    - gpt-4.1-nano-2025-04-14
    - gpt-4.1-nano
    - gpt-image-1
    - gpt-4o-realtime-preview-2025-06-03
    - gpt-4o-audio-preview-2025-06-03
    - o4-mini-deep-research
    - gpt-4o-transcribe-diarize
    - o4-mini-deep-research-2025-06-26
    - gpt-5-chat-latest
    - gpt-5-2025-08-07
    - gpt-5
    - gpt-5-mini-2025-08-07
    - gpt-5-mini
    - gpt-5-nano-2025-08-07
    - gpt-5-nano
    - gpt-audio-2025-08-28
    - gpt-realtime
    - gpt-realtime-2025-08-28
    - gpt-audio
    - gpt-5-codex
    - gpt-image-1-mini
    - gpt-5-pro-2025-10-06
    - gpt-5-pro
    - gpt-audio-mini
    - gpt-audio-mini-2025-10-06
    - gpt-5-search-api
    - gpt-realtime-mini
    - gpt-realtime-mini-2025-10-06
    - sora-2
    - sora-2-pro
    - gpt-5-search-api-2025-10-14
    - gpt-5.1-chat-latest
    - gpt-5.1-2025-11-13
    - gpt-5.1
    - gpt-5.1-codex
    - gpt-5.1-codex-mini
    - gpt-5.1-codex-max
    - gpt-image-1.5
    - gpt-5.2-2025-12-11
    - gpt-5.2
    - gpt-5.2-pro-2025-12-11
    - gpt-5.2-pro
    - gpt-5.2-chat-latest
    - gpt-4o-mini-transcribe-2025-12-15
    - gpt-4o-mini-transcribe-2025-03-20
    - gpt-4o-mini-tts-2025-03-20
    - gpt-4o-mini-tts-2025-12-15
    - gpt-realtime-mini-2025-12-15
    - gpt-audio-mini-2025-12-15
    - chatgpt-image-latest
    - gpt-5.2-codex
    - gpt-5.3-codex
    - gpt-realtime-1.5
    - gpt-audio-1.5
    - gpt-4o-search-preview
    - gpt-4o-search-preview-2025-03-11
    - gpt-5.3-chat-latest
    - gpt-5.4-2026-03-05
    - gpt-5.4-pro
    - gpt-5.4-pro-2026-03-05
    - gpt-3.5-turbo-16k
    - tts-1
    - whisper-1
    - text-embedding-ada-002
  anthropic:
    - claude-sonnet-4-6
    - claude-opus-4-6
    - claude-opus-4-5-20251101
    - claude-haiku-4-5-20251001
    - claude-sonnet-4-5-20250929
    - claude-opus-4-1-20250805
    - claude-opus-4-20250514
    - claude-sonnet-4-20250514
    - claude-3-haiku-20240307
  perplexity:
    - sonar
    - sonar-pro
    - sonar-deep-research
    - sonar-reasoning
    - sonar-reasoning-pro

=== Get Specific Provider Models ===
OpenAI models: gpt-4-0613, gpt-4, gpt-3.5-turbo, gpt-5.4-mini, gpt-5.4, gpt-5.4-nano-2026-03-17, gpt-5.4-nano, gpt-5.4-mini-2026-03-17, davinci-002, babbage-002, gpt-3.5-turbo-instruct, gpt-3.5-turbo-instruct-0914, dall-e-3, dall-e-2, gpt-3.5-turbo-1106, tts-1-hd, tts-1-1106, tts-1-hd-1106, text-embedding-3-small, text-embedding-3-large, gpt-3.5-turbo-0125, gpt-4-turbo, gpt-4-turbo-2024-04-09, gpt-4o, gpt-4o-2024-05-13, gpt-4o-mini-2024-07-18, gpt-4o-mini, gpt-4o-2024-08-06, gpt-4o-audio-preview, gpt-4o-realtime-preview, omni-moderation-latest, omni-moderation-2024-09-26, gpt-4o-realtime-preview-2024-12-17, gpt-4o-audio-preview-2024-12-17, gpt-4o-mini-realtime-preview-2024-12-17, gpt-4o-mini-audio-preview-2024-12-17, o1-2024-12-17, o1, gpt-4o-mini-realtime-preview, gpt-4o-mini-audio-preview, o3-mini, o3-mini-2025-01-31, gpt-4o-2024-11-20, gpt-4o-mini-search-preview-2025-03-11, gpt-4o-mini-search-preview, gpt-4o-transcribe, gpt-4o-mini-transcribe, o1-pro-2025-03-19, o1-pro, gpt-4o-mini-tts, o3-2025-04-16, o4-mini-2025-04-16, o3, o4-mini, gpt-4.1-2025-04-14, gpt-4.1, gpt-4.1-mini-2025-04-14, gpt-4.1-mini, gpt-4.1-nano-2025-04-14, gpt-4.1-nano, gpt-image-1, gpt-4o-realtime-preview-2025-06-03, gpt-4o-audio-preview-2025-06-03, o4-mini-deep-research, gpt-4o-transcribe-diarize, o4-mini-deep-research-2025-06-26, gpt-5-chat-latest, gpt-5-2025-08-07, gpt-5, gpt-5-mini-2025-08-07, gpt-5-mini, gpt-5-nano-2025-08-07, gpt-5-nano, gpt-audio-2025-08-28, gpt-realtime, gpt-realtime-2025-08-28, gpt-audio, gpt-5-codex, gpt-image-1-mini, gpt-5-pro-2025-10-06, gpt-5-pro, gpt-audio-mini, gpt-audio-mini-2025-10-06, gpt-5-search-api, gpt-realtime-mini, gpt-realtime-mini-2025-10-06, sora-2, sora-2-pro, gpt-5-search-api-2025-10-14, gpt-5.1-chat-latest, gpt-5.1-2025-11-13, gpt-5.1, gpt-5.1-codex, gpt-5.1-codex-mini, gpt-5.1-codex-max, gpt-image-1.5, gpt-5.2-2025-12-11, gpt-5.2, gpt-5.2-pro-2025-12-11, gpt-5.2-pro, gpt-5.2-chat-latest, gpt-4o-mini-transcribe-2025-12-15, gpt-4o-mini-transcribe-2025-03-20, gpt-4o-mini-tts-2025-03-20, gpt-4o-mini-tts-2025-12-15, gpt-realtime-mini-2025-12-15, gpt-audio-mini-2025-12-15, chatgpt-image-latest, gpt-5.2-codex, gpt-5.3-codex, gpt-realtime-1.5, gpt-audio-1.5, gpt-4o-search-preview, gpt-4o-search-preview-2025-03-11, gpt-5.3-chat-latest, gpt-5.4-2026-03-05, gpt-5.4-pro, gpt-5.4-pro-2026-03-05, gpt-3.5-turbo-16k, tts-1, whisper-1, text-embedding-ada-002

=== Get Anthropic Models ===
Anthropic models: claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929, claude-opus-4-1-20250805, claude-opus-4-20250514, claude-sonnet-4-20250514, claude-3-haiku-20240307

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 405: {"code":405,"message":"Method Not Allowed"}

✓ Chat Models example complete
✓ Client created

=== Create User Function ===
Created user function with ID: kHNsjlCO9VmzZupqjIiRV3OvOYXFOyNpY8BE74q2kRKpibCUvXlD5VUkAqYMcev6Um7BBPruX1g2KUsKKHwWRg

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 31 user functions:
  - get_verified_user: Get verified and validated user
  - validate_user: Check if user exists
  - fetch_and_store_user: Fetch user from API and cache in KV
  - cache_api_call: Cache External API Call
  - swr_user: SWR pattern for user data (KV-based)
  - get_active_users_ts: Get Active Users
  - fetch_user: Fetch user by code
  - get_user_wrapper: Wrapper that calls fetch_user
  - get_users_by_status: Get Users By Status
  - fetch_slim_user: Validate and slim down user
  - get_verified_user: Get verified and validated user
  - cache_api_call_py: Cache External API Call
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_api_user: Fetch User with Cache
  - fetch_slim_user: Validate and slim down user
  - fetch_slim_user: Validate and slim down user
  - validate_user: Check if user exists
  - get_user_wrapper: Wrapper that calls fetch_user
  - validate_user: Check if user exists
  - swr_user: SWR pattern for user data (KV-based)
  - fetch_user: Fetch user by code
  - swr_user: SWR pattern for user data (KV-based)
  - validate_user: Check if user exists
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_slim_user: Validate and slim down user
  - fetch_user: Fetch user by code
  - swr_user: SWR pattern for user data (KV-based)
  - get_active_users_updated: Get Active Users (Updated)
  - fetch_user: Fetch user by code
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_product_with_reviews: Fetch Product with Reviews (Multi-API)

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
✓ Client created

=== Check Collection Exists (Before Creation) ===
Collection 'collection_utils_test_ts' exists: false

=== Creating Test Documents ===
Created 5 test documents

=== Check Collection Exists (After Creation) ===
Collection 'collection_utils_test_ts' exists: true

=== Count Documents ===
Document count in 'collection_utils_test_ts': 5

=== Check Non-Existent Collection ===
Collection 'nonexistent_collection_xyz' exists: false

=== Cleanup ===
Deleted collection 'collection_utils_test_ts'

✓ Collection Utilities example complete
✅ [32mTypeScript client examples complete![0m
✅ [32mAll TypeScript integration tests complete![0m
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 1s

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
  id: 'lUhfG_LkOWy6qyOQXBKuwR10Svi51e6NHEC2ay-wBPYxwvq8XxqQBL0ZAqi4fapGbCq2Smn_g5TjkoYYUf_sUQ'
}

=== Find by ID ===
Found: {
  value: { value: 42, type: 'Integer' },
  name: { type: 'String', value: 'Test Record' },
  active: { type: 'Boolean', value: true },
  id: 'lUhfG_LkOWy6qyOQXBKuwR10Svi51e6NHEC2ay-wBPYxwvq8XxqQBL0ZAqi4fapGbCq2Smn_g5TjkoYYUf_sUQ'
}

=== Find with Query ===
Found documents: [
  {
    value: { value: 42, type: 'Integer' },
    name: { type: 'String', value: 'Test Record' },
    active: { value: true, type: 'Boolean' },
    id: 'lUhfG_LkOWy6qyOQXBKuwR10Svi51e6NHEC2ay-wBPYxwvq8XxqQBL0ZAqi4fapGbCq2Smn_g5TjkoYYUf_sUQ'
  }
]

=== Update Document ===
Updated: {
  name: { type: 'String', value: 'Updated Record' },
  id: 'lUhfG_LkOWy6qyOQXBKuwR10Svi51e6NHEC2ay-wBPYxwvq8XxqQBL0ZAqi4fapGbCq2Smn_g5TjkoYYUf_sUQ',
  value: { value: 100, type: 'Integer' },
  active: { value: true, type: 'Boolean' }
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
✓ Inserted test record: sK8ODMrIeauL8-kB9cqwBBWBFaAyjyb5O_Fmn0YmU4iZBXpibDBkpH2lI_Vk1LWp6rFx0pmR2G1jiygxzPQ-qw

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1776012329817",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "FeqA3vaSCqzaM09W3ES2j5OE8ThY6bgfDXBhYB9paIu6aM8tKHwgA8kvLgSA_dvnIjkt2zrbSb_ciW8KsFJ-KA",
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
        "id": "sK8ODMrIeauL8-kB9cqwBBWBFaAyjyb5O_Fmn0YmU4iZBXpibDBkpH2lI_Vk1LWp6rFx0pmR2G1jiygxzPQ-qw",
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
        "id": "MtXpLar30tru4Y354yBpOXUjpyCzRaHSnY8__zDq3z-SdZu4fg-jkmuyQAxukeHRhr6q1tC5bKDJeWdu3oDqHQ",
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
        "id": "UYjUzebVUq9jyfdiOrA690sKXzuuYoRp075pKBdtpqydEtcSDCZXMcvbBXk0YR70qiviclhcV9Mx-7bxfyFGTw",
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

✅ Function saved: VSJxwM2jA96yBfma-2M1BkauRMB3rpNzTWQtmAL0xsVDwe7foStK1dFz9jEbkCnFt4_5iX0CI-M586NryyLoLg
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: TQ2ua7-086ARb9n_CCWOOEisfBXIGyOTMOh3OozZMAH78ORtivK0rWcWihfwvC1NOqdohW1ADsocpMHXlvw6KQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: mzfwKq9noUJhRi88RCq-8dXH9klgAlzP_K9PA1rwgfyUW67Dt7tjTvHZJE-KQweaKO4ybQKuTTLfI1bua3fnag
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"max_score":100,"count":10,"status":"active","avg_score":60}
   {"max_score":90,"status":"inactive","avg_score":50,"count":10}

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
Collection created with first record: xBhXFnCG0mKTdhr2t-g4WfOngvVRNYYnI2g6Y8qQAI9bESvtWb0luWMmCJpmjo7QqkOXZg7NCZJxHrLHjRm0fA

=== List Collections ===
Total collections: 25
Sample collections: [
  'schema_users_client_py',
  'test_collection',
  'users',
  'demo_collection',
  'schema_users_client_go'
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
Created Alice: $1000 - ID: 3B_8y0xTkAEESvSwJ4dVezz3r0bQR5T3fa1Ny5a9HfFyz_apO083Opfqdlwjd72Z8okkABB6c0akr8hgV6YGEA
Created Bob: $500 - ID: T1sUMsEehxuvXoyif4ulfSGb1gLcAF_t-jyDkquT9GTKrPS8FKAOzavfj8OBMXBRyUOZRDAIRgIuXvRwCA3vvw

=== Example 1: Begin Transaction ===
Transaction ID: a493cbab-a0c2-47bc-bc45-b4cdc4fe04d8

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
New transaction: bc5f86be-7255-4b0c-b4ac-2a9465f7990c
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
   ✅ Function saved: 3BwnWlDVFVRwAl7MZQydnJ5cCxERmj-B9_Vq7hzL0a_fzrMSX8o6P09QLXe8fdrEwwpnX_au9Y1jl7XQPoBH4w

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
   ✅ Function saved: BbCeb7c9CNtrvkv32qdOw7bH5h7EAj3tjWx0HEG2-j4VLmeXt7diqJwNEUA-P1W0ckI4Lf3GgX9YUdv6DKTsTQ

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
   ✅ Function saved: ZduogjzYmM-uiny_I_67aVVbTJKA57ht4xU6kDySm5iR1QVZecvchCCzMpOYK7nCHyJ9iQkqXgiHOHA9UqE_Ww

2️⃣ Calling Function (Query + Update Credits + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"value":0,"type":"Integer"}
   📋 Status: {"value":"active","type":"String"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: r9DqM9BbLzJ8n4xjMH-pez-U8puEBAG0yO5kQBqWlYcZraRHfQ8vhIeU3irypivtUYseNO2ew0s9hcGbcrYFrg

2️⃣ Calling Function (Query + Delete + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted function: 3BwnWlDVFVRwAl7MZQyd...
   ✅ Deleted function: BbCeb7c9CNtrvkv32qdO...
   ✅ Deleted function: ZduogjzYmM-uiny_I_67...
   ✅ Deleted function: r9DqM9BbLzJ8n4xjMH-p...
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
  Output: Document ID = uX_ZMrQGdsmia33xfvaz0zS0z1a3WlWXL0Qsc3zI2y3nLIlrtnG6IsH1-fHskSNWezGTPyImqEH9wfePCBrzlw
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(uX_ZMrQGdsmia33xfvaz0zS0z1a3WlWXL0Qsc3zI2y3nLIlrtnG6IsH1-fHskSNWezGTPyImqEH9wfePCBrzlw)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(uX_ZMrQGdsmia33xfvaz0zS0z1a3WlWXL0Qsc3zI2y3nLIlrtnG6IsH1-fHskSNWezGTPyImqEH9wfePCBrzlw)
  Output: Error (expected) - Request failed with status 404: {"error":"Record not found (expired)"}
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
  Output: Document ID = POca6dehQZRFZmrYSaVK_OybUrrys8vOvLzLegZHiFU0gwXdAi199CTSRvzL0jOqARTKC3qwidTdU-hWktoJnA
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(POca6dehQZRFZmrYSaVK_OybUrrys8vOvLzLegZHiFU0gwXdAi199CTSRvzL0jOqARTKC3qwidTdU-hWktoJnA)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(POca6dehQZRFZmrYSaVK_OybUrrys8vOvLzLegZHiFU0gwXdAi199CTSRvzL0jOqARTKC3qwidTdU-hWktoJnA)
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

> @ekodb/ekodb-client@0.16.0 prepare
> npm run build


> @ekodb/ekodb-client@0.16.0 build
> tsc


up to date, audited 52 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.16.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning JavaScript client library examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 999ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'LEWYyY41q_YyyOrr5KBslR4tAHOc_5t4NN30E8Aj8F8anpKDKyzclfDrAx-d3lR5SaeX7H-8yY4q5ZL-aBZEEA'
}

=== Find by ID ===
Found: {
  value: { value: 42, type: 'Integer' },
  active: { value: true, type: 'Boolean' },
  id: 'LEWYyY41q_YyyOrr5KBslR4tAHOc_5t4NN30E8Aj8F8anpKDKyzclfDrAx-d3lR5SaeX7H-8yY4q5ZL-aBZEEA',
  name: { value: 'Test Record', type: 'String' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  value: { type: 'Integer', value: 100 },
  id: 'LEWYyY41q_YyyOrr5KBslR4tAHOc_5t4NN30E8Aj8F8anpKDKyzclfDrAx-d3lR5SaeX7H-8yY4q5ZL-aBZEEA',
  name: { type: 'String', value: 'Updated Record' },
  active: { type: 'Boolean', value: true }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: UU6GiLM41CfOI0PfncLtyWpRfnGebH3xBRHt4FMt9B5pBna9nvyQK4TGoVzHgOeaGhkwzT8T0oeKXiyzkokTBA

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
Collection created with first record: cHR-yAQYQIyvT_GtAn5ymNoLTYyN4lYeE37uLfxVsbknDrRFNGDmwpDaGDa0WGcPPvGiwG1uKxffu_y64Ic0bg

=== List Collections ===
Total collections: 24
Sample collections: schema_users_client_py,test_collection,schema_users_client_go,chat_messages__ek0_testing,schema_documents_client_go

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
Retrieved value: { userId: 123, username: 'john_doe' }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { price: 29.99, name: 'Product 1' }
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 0 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 15

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: ur7hLmVoCHoXGzC3P7bpWmPN6XmEC2iuIJSlBRUCIfz2NsNNc12ujmLiM0RVXbPE2IBVMgSugIqWl7qTWKu-pw
Created Bob: $500 - ID: n6RhPM738GpxwjvA7pIv2pdzBgQlsXEX6lzvbmeHvE_e2ly1-CWWO9IHTH2N5zLhgaw_To9Zulpoin6uU1Rx8w

=== Example 1: Begin Transaction ===
Transaction ID: aa284c96-d603-43fc-9710-28cf2b0bf543

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 684900fd-a3f5-43b7-9f7c-84283d6059e1
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
  1. Score: 25.740, Matched: name.value, email.value, name, email
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio.value, title, bio, title.value
  2. Score: 26.400, Matched: bio.value, bio, title, title.value
  3. Score: 26.400, Matched: title.value, bio.value, bio, title
  4. Score: 26.400, Matched: title.value, bio, bio.value, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio, title.value, bio.value, title
  2. Score: 39.600, Matched: bio, title.value, bio.value, title
  3. Score: 39.600, Matched: bio, title.value, title, bio.value
  4. Score: 39.600, Matched: title.value, title, bio, bio.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.757, Matched: 
  2. Score: 0.739, Matched: 
  3. Score: 0.729, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.695, Matched: content, content.value, title.value, title
  2. Score: 1.503, Matched: title, content.value, title.value, content
  3. Score: 0.291, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: bio.value, skills, skills.value, bio

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
✓ Inserted document: 4zq43g_B_n3_HNwar9ors2_nIUhfG58a4AJnFW6cnh0QOCKN_bRN9nKWG9w218SUF_LgxcLbHWvjmCyhume7wA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: RVCZhPoOlBU_xfV4DqxhJyfrQJTQKHpoO9TQqWCkOIh4Jf_nDRuvjngpMJr17IcsffrkXldQs6ujpKUZs5nVZg

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
✓ Inserted document with TTL: jacgYk-exFNrs_mARIp2PUBOsZPC53ZFRIKv0Obw5qtZAesqiT-DhKYRbEWeNQZqtq0cmZAgMKcO0SlKk4cVYA

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
✓ Edge cache script created: gnRxF91tEr1YbdpQG1sXAMzOqmteX07Jh59HQKPR0U0qInfWWXhBb5v2XkEcW-Ir9C7t4-kuiY8SG9I_Eq0tLQ

Call 1: Cache miss (fetches from API)
Response time: 41ms
Result: {
  "records": [
    {
      "value": {
        "timezone": "GMT",
        "timezone_abbreviation": "GMT",
        "current_units": {
          "interval": "seconds",
          "time": "iso8601",
          "temperature_2m": "°C"
        },
        "latitude": 40.710335,
        "elevation": 32,
        "current": {
          "temperature_2m": 12.3,
          "time": "2026-04-12T16:30",
          "interval": 900
        },
        "longitude": -73.99308,
        "generationtime_ms": 0.025153160095214844,
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
Response time: 41ms (1x faster!)
Result: {
  "records": [
    {
      "value": {
        "timezone": "GMT",
        "timezone_abbreviation": "GMT",
        "current_units": {
          "interval": "seconds",
          "time": "iso8601",
          "temperature_2m": "°C"
        },
        "latitude": 40.710335,
        "elevation": 32,
        "current": {
          "temperature_2m": 12.3,
          "time": "2026-04-12T16:30",
          "interval": 900
        },
        "longitude": -73.99308,
        "generationtime_ms": 0.025153160095214844,
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

✅ Function saved: npRINyX32BsX_-CJoe0mjtjGDmBUvqOgzmvpaJBUa_oTJb0DZAwDrxe1bnXPZYftiO19RFQRg3oZlmjJhlhO8Q
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

✅ Function saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 2 groups
   {"count":5,"avg_score":60,"status":"active"}
   {"status":"inactive","avg_score":50,"count":5}
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
   Name: {"value":"User 1","type":"String"}
   Department: {"value":"engineering","type":"String"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 163ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "website": "hildegard.org",
    "username": "Bret",
    "email": "Sincere@april.biz",
    "phone": "1-770-736-8031 x56442",
    "company": {
      "name": "Romaguera-Crona",
      "...

Second call (cache hit - from cache):
   ⏱️  Duration: 38ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "website": "hildegard.org",
    "username": "Bret",
    "email": "Sincere@april.biz",
    "phone": "1-770-736-8031 x56442",
    "company": {
      "name": "Romaguera-Crona",
      "...
   🚀 Cache speedup: 4.3x faster!

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

✅ Function saved: wDpv_aD4lzENm_uImg3USX28EvvQCkTTMy_HPMgc-q8s43UyepH9Q9udH9Kvs6lgc95mdFS1gCphlAAg7dIc5g
📊 Found 2 product groups
   {"count":2,"avg_price":474,"category":"Furniture"}
   {"count":3,"avg_price":575.6666666666666,"category":"Electronics"}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"category":"Furniture","count":2}
   {"category":"Electronics","count":3}
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

✅ Inserted order: xA8iETbyhlJuc89l2f-l4e7cOoZCm8hlHd1WYkP9sjjTIG9t95typQADPs999Df6UwkEGzefmxOXBUZ_rez3hw
✅ Inserted 2 products with wrapped types

📝 Example 2: Function with Wrapped Type Parameters

✅ Function saved: ff4zxcKPo763ew1Rn7zNOz_-4u4Jl4PKTV_pZ1d3Bf4zV9lCTuNfzOaVSZyaR1PI5UTHLs1fcqr-HYE5crKB2g
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"userId":"user_abc","role":"admin"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: LjmShSwT438gGqKZHlQY_f0nLvq86tnD6txKzvGIpH5HrfHFukI2Pif6qWAezl0JrLiuD5pQ6rljrXud5YiGLw
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: Pu4JDXunaJ1z14ImkVNKe7WtkvJZcVH6BLuOeG8XcmWRWMT6buzHNrXjDOdiLIGewWq74ckYR_livvnqLZNTcg
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
   inactive: 3 users
   active: 7 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Function saved
📊 Average scores by role:
   admin: 20.0 (3 users)
   user: 70.0 (7 users)
⏱️  Execution time: 0ms

📝 Example 4: Top Users by Score

✅ Function saved
📊 Users (showing first 5 of 10):
   1. User 10 - Score: 100
   2. User 9 - Score: 90
   3. User 5 - Score: 50
   4. User 7 - Score: 70
   5. User 8 - Score: 80
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
   1. Getting Started with ekoDB (Database)
   2. Introduction to Machine Learning (AI)
   3. Vector Databases Explained (Database)
   4. Natural Language Processing (AI)
   5. Database Design Principles (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   Database: 3 documents
   AI: 2 documents
⏱️  Execution time: 0ms

📝 Example 3: Select Specific Fields

✅ Function saved
📊 Document titles (5 docs):
   1. Getting Started with ekoDB
   2. Introduction to Machine Learning
   3. Vector Databases Explained
   4. Natural Language Processing
   5. Database Design Principles
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Function saved
📊 Projected documents (showing first 3):
   1. Getting Started with ekoDB
   2. Introduction to Machine Learning
   3. Vector Databases Explained
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Function saved
📊 All documents (5 total, showing first 2):
   1. Getting Started with ekoDB (Database)
   2. Introduction to Machine Learning (AI)
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
   Electronics: 6 items (avg $325.67)
   Furniture: 4 items (avg $294.00)
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
   1. Laptop Pro - $1299 (⭐4.8)
   2. Monitor 27" - $399 (⭐4.6)
   3. USB-C Cable - $19 (⭐4.3)
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
   1. Precision: Vector data is very accurate and precise as it allows the representation of data at the original resolution and form without generalization.

2. More Control: Users have more control over the data as the specifics can be manipulated and rendered based on specific needs.

3. Topological Information: Vector databases can contain topological info that allows understanding of the geographic context of data – like connectedness and adjacency.

4. Scalability: Data remains accurate and precise even when zoom in or out.

5. Lesser space: A clean vector data set usually requires less space for storage compared to raster data.

6. Supports complex analysis: Vector databases have better capabilities to support complex spatial analysis such as network analysis or topology. 

7. High-quality graphics: They maintain a high-quality visualization for graphic features like points, lines, and polygons.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding function saved
📊 Generated embeddings for 3 articles
   1. "Advanced Query Patterns" - 1536D vector
   2. "Draft Article" - 1536D vector
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
✓ Created SWR script: fetch_api_user_js (UE-M8uOi0T5Q-ddjO1MGvnXD_sbR-8lrn5F8ao5sCAOuUeaJElcqK2OCET7TuzahrHlE-qjt3wxuG66YYoCtdg)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "id": 1,
        "phone": "1-770-736-8031 x56442",
        "email": "Sincere@april.biz",
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "bs": "harness real-time e-markets",
          "name": "Romaguera-Crona"
        },
        "name": "Leanne Graham",
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
        "username": "Bret",
        "website": "hildegard.org"
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
Response time: 47ms (served from cache)
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
✓ Created native SWR script: github_user_native (mcGTqb1W3XpQWBBXHaEOMzzgJEebsxrL1ALLLEdsjHiR9ygcr1PXorgSqRxeYMSGjJ0hHXHESEI_7AwrCvyk7g)

First call (cache miss - will fetch from GitHub API):
  Response time: 16ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 21ms
  Speedup: 0.8x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (XjqKG155O6TlkRtxM0JDM5lpPEu-0jHGDcl24U_CdLnqbhVl7co5m7O0rs3g5FLfbrpteqlctqF9pxpOR83F_A)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (49F780SP8-a05_RZWzQJiM_9CsiJSbERblbYT9Ah_Gr59KWsCZikl_PpaYgfkP730rCpJEruf1XaXzMBcSQEEA)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (xtor-4qa5nbppvauBKySOwAuk4aoVdK-2cv-MyWucb83DB88ppJag7Z5r7aVsQZNu39N_eEXXN8_sfXC32Pr5Q)
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
✓ Created session: 3u8q01KGFjTppiBW42YtHmAL2L6XCBGEazuy5vkKZ_7CAFSzmWd7xV6cJDzEqpq5pDaZBtIOd6xl4yj-c-XMkg

=== Sending Chat Message ===
Message ID: Vs3tfl2qIRUeZdVX5npOpk38XzbxFSpeR2CRwX5zys3uw7Y7umOwxLHsShXTdvVJTiPTrsodq8YCV5nRlcyAtw

=== AI Response ===
Here are the available products along with their prices:

1. **ekoDB**  
   - **Description:** A high-performance database product with AI capabilities  
   - **Price:** $99

2. **ekoDB Cloud**  
   - **Description:** Fully managed cloud database service product  
   - **Price:** $499

3. **ekoDB Pro**  
   - **Description:** Enterprise edition product with advanced features  
   - **Price:** $299

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    price: 99,
    description: 'A high-performance database product with AI capabilities',
    name: 'ekoDB',
    id: 'sOBsEqIUCprJD_3DzV8nmk7i2nrdF1obSiizhdC_nnX7Wpo91OCT2KdKZEoqI1BG2m4VcOoRXvFEnv0V4O7cmQ'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    price: 499,
    description: 'Fully managed cloud database service product',
    id: '0nuQCf1U87muYXicDph9p-WbLT4_hCFdvwbaT5KYqz27pCabAWMyyKi7K978jYkmDdXWrv-9VrwziF5yzX1ZKw',
    name: 'ekoDB Cloud'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB Pro',
    id: 'MdRygPEQn3PuAwPcrTdEUUt1A2Oqoct7tR8hr3icQFcU1wvzJMcE8a5GopA7F3nAJsnaYr0rkDXwEMfa8_pDfA',
    price: 299,
    description: 'Enterprise edition product with advanced features'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 3105ms

=== Token Usage ===
Prompt tokens: 1298
Completion tokens: 100
Total tokens: 1398

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 2R6O0XEt_avDZYXuyuJdjUrR2-4qo4hOcGj4oZDD00E92AisWb8WMz6fDTWybUuL4Z7tFNMEExRySxPFSsahSQ

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you would like to know more details or have other questions, feel free to ask!

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you have any further questions or need more information, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: g1vTMsKcRFdECWVlFg0YGCtnzoupn8YnGihUTNPVqNKPkDCvTY13rh4FjtFwH3O6Svm3okIwkqkRybqU3WwZeQ
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
✓ Created session: AsU1Oil6uViYNupwxqcnnyBvwTZvJh1HzLipSWj0d_90ZzryQ07GkYbAY-_kVFkvyUSRy1F5HviUV2iE5Prlbw

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99

If you need more information or additional products, let me know!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: goFpTA90HsvlHCweGwxb_ja_qBb6YW-rtlWYfyOZAA3dokSiCZFQvfYfdF7yHKkWDs6vuBx4W9wmos_NPzBddQ
  Parent: AsU1Oil6uViYNupwxqcnnyBvwTZvJh1HzLipSWj0d_90ZzryQ07GkYbAY-_kVFkvyUSRy1F5HviUV2iE5Prlbw

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: goFpTA90HsvlHCweGwxb_ja_qBb6YW-rtlWYfyOZAA3dokSiCZFQvfYfdF7yHKkWDs6vuBx4W9wmos_NPzBddQ (Untitled)
  Session 2: AsU1Oil6uViYNupwxqcnnyBvwTZvJh1HzLipSWj0d_90ZzryQ07GkYbAY-_kVFkvyUSRy1F5HviUV2iE5Prlbw (Untitled)
  Session 3: g1vTMsKcRFdECWVlFg0YGCtnzoupn8YnGihUTNPVqNKPkDCvTY13rh4FjtFwH3O6Svm3okIwkqkRybqU3WwZeQ (Untitled)
  Session 4: 3u8q01KGFjTppiBW42YtHmAL2L6XCBGEazuy5vkKZ_7CAFSzmWd7xV6cJDzEqpq5pDaZBtIOd6xl4yj-c-XMkg (Untitled)
  Session 5: oXyNlcmlgi1EJFwRbqJFi4_amcde50qpGVi13cHaHKVRLpr3h-YuqugcpR8JUdfSnb_FhzTvnWquljdcqIlFRA (Untitled)
  Session 6: sB-siCOBQMoeDbbZGWMpxBRbjV56-5IaFPUO_KWVohEzM2ZhO_fe7kEzGbHZSSkKCob-yIJYdQBhnRMJkRT54w (Untitled)
  Session 7: Wq0FBjdjyZFL5dvEiXXh9oSzxml2MaulqyBAArknQ-YjhUBicp2wm_7t2j_4Jo6SfUpQdqr9EH0zvA7Vi4vvWA (Untitled)
  Session 8: Znp6mUA8YF5IZMlJ_Gi5RO5IK4Csu0uLWqmdO2yqmd_ANoxYa250sKiTx-bmvvu2SIgeuzVldlKiXpUYweOW_g (Untitled)
  Session 9: Be84Ao5KLBzjK4zjM0uyi7WHiFGkgnGiBz913zjhkQrGrZn6A3j7uDi1wNEsih7W6bTQ2aTuD5OcL0gfaQQrvQ (Untitled)
  Session 10: 32PFek8QtgY3SxhW6Q7KVWSbd7uQimZbD3OOUBIeM8mjW6NtmYSx4TqnDEaMk5EP30UDXloORbnCr2umJk9wrw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: goFpTA90HsvlHCweGwxb_ja_qBb6YW-rtlWYfyOZAA3dokSiCZFQvfYfdF7yHKkWDs6vuBx4W9wmos_NPzBddQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
✅ [32mJavaScript client examples complete![0m
✅ [32mAll JavaScript integration tests complete![0m
🟣 [36mBuilding Kotlin client library...[0m
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE
> Task :jar UP-TO-DATE
> Task :assemble UP-TO-DATE
> Task :compileTestKotlin UP-TO-DATE
> Task :compileTestJava NO-SOURCE
> Task :processTestResources NO-SOURCE
> Task :testClasses UP-TO-DATE
> Task :test UP-TO-DATE
> Task :check UP-TO-DATE
> Task :build UP-TO-DATE

BUILD SUCCESSFUL in 3s
4 actionable tasks: 4 up-to-date
✅ [32mKotlin client built![0m
🧪 [36mRunning Kotlin client library examples...[0m
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Simple CRUD Example ===

=== Create ===
✓ Inserted user: EkoRecord(fields={id=StringValue(value=BeE8D5ve0eqdewAul7S0lgI5ez9ZdOlijNzeixeOXMk_tecNGTdv_vEnKP_V8bPPbAXs1X6HWsYhBRKaifseEg)})
  User ID: BeE8D5ve0eqdewAul7S0lgI5ez9ZdOlijNzeixeOXMk_tecNGTdv_vEnKP_V8bPPbAXs1X6HWsYhBRKaifseEg

=== Read ===
✓ Found user by ID: EkoRecord(fields={id=StringValue(value=BeE8D5ve0eqdewAul7S0lgI5ez9ZdOlijNzeixeOXMk_tecNGTdv_vEnKP_V8bPPbAXs1X6HWsYhBRKaifseEg), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)}), embedding=ObjectValue(value={value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)]), type=StringValue(value=Array)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=28)}), created_at=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Sun Apr 12 12:46:57 EDT 2026)}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Alice Johnson
  email (String): alice@example.com
  age (Integer): 28
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): Sun Apr 12 12:46:57 EDT 2026
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1, tag2, tag3]
  metadata (Object): {key=value, nested={deep=true}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): [electronics, computers]
  data (Bytes): 11 bytes
Record fields: id, name, tags, user_id, embedding, active, email, age, created_at, data, metadata, price, categories

=== Update ===
✓ Updated user: EkoRecord(fields={embedding=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)])}), metadata=ObjectValue(value={value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})}), type=StringValue(value=Object)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), id=StringValue(value=BeE8D5ve0eqdewAul7S0lgI5ez9ZdOlijNzeixeOXMk_tecNGTdv_vEnKP_V8bPPbAXs1X6HWsYhBRKaifseEg), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), created_at=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Sun Apr 12 12:46:57 EDT 2026)}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])})})

=== Query ===
✓ Found 1 users matching query
  - EkoRecord(fields={price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), id=StringValue(value=BeE8D5ve0eqdewAul7S0lgI5ez9ZdOlijNzeixeOXMk_tecNGTdv_vEnKP_V8bPPbAXs1X6HWsYhBRKaifseEg), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), embedding=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)])}), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), created_at=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Sun Apr 12 12:46:57 EDT 2026)})})

=== Delete ===
✓ Deleted user with ID: BeE8D5ve0eqdewAul7S0lgI5ez9ZdOlijNzeixeOXMk_tecNGTdv_vEnKP_V8bPPbAXs1X6HWsYhBRKaifseEg

✓ Confirmed user was deleted

=== Cleanup ===
✓ Deleted collection: kotlin_users_example

=== Example Complete ===

BUILD SUCCESSFUL in 6s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Simple WebSocket Example ===

=== Create WebSocket Client ===
✓ WebSocket client created

=== Connect to WebSocket ===
✓ Connected to WebSocket

=== Insert Test Record ===
✓ Inserted test record

=== Find All via WebSocket ===
✓ WebSocket findAll result:
  {"data":[{"id":"aUc7XMG5QcY6a9MMPZOBGqbMeIAK8mdtIngyIJIunQBy7fdI7eqh_SzWReKCOwDHUitBlJXmwXvYqiwDclVNig","name":{"type":"String","value":"Test User"},"status":{"type":"String","value":"active"}}]}

=== Close WebSocket ===
✓ WebSocket closed

=== Cleanup ===
✓ Deleted collection: kotlin_websocket_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Batch Operations Example ===

=== Batch Insert ===
✓ Inserted 5 records
  IDs: U0PnVE5gA0HLwyy3rH1O-nfgLtTiJ-qqS76CcRc6Q_cR8O4IqE_RIDzgn76DI69HKGehDkFcNvxG6ksYMBU2eg, shjm_QEvVOyrO17L0-f-rlA_WXPlfzIsNXpLIiq7pEDef--TmQ7cotRY18Tm6oNpZCcSdibK0X31G_yQxzyG2A, NLVxqYecvC64xH2keisuGjvgTGVdOpZ0RBitSI-XBB8g8Ag-UxPgad57BQGO-fgSDNlzoOXZYOTyz8eJtGlKtQ...

=== Batch Update ===
✓ Updated 3 records

=== Batch Delete ===
✓ Deleted 2 records

=== Cleanup ===
✓ Deleted collection: kotlin_batch_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Collection Management Example ===

=== List Collections ===
✓ Found 28 collections
  - schema_users_client_py
  - test_collection
  - users
  - schema_users_client_go
  - chat_messages__ek0_testing

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
✓ Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
✓ Schema: {"fields":{"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2026-04-12T16:47:14.175131Z","last_modified":"2026-04-12T16:47:14.175131Z","bypass_ripple":false,"primary_key_alias":"id"}

=== Cleanup ===
✓ Deleted collection: kotlin_collection_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - KV Operations Example ===

=== KV Set ===
✓ Set key: user:123

=== KV Get ===
✓ Retrieved value: {"email":"alice@example.com","name":"Alice","role":"admin"}

=== KV Set with TTL ===
✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key ===
✓ Session value: {"user_id":"123","created_at":1776012438496}
  (Will expire in 10 seconds)

=== KV Batch Set ===
✓ Batch set 3 keys
  config:db: success
  config:cache: success
  config:api: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  config:db: {"value":{"host":"localhost","port":5432}}
  config:cache: {"value":{"ttl":3600,"enabled":true}}
  config:api: {"value":{"retries":3,"timeout":30}}

=== KV Exists ===
✓ Key exists: true

=== KV Find (Pattern Query) ===
✓ Found 3 keys matching 'config:.*'

=== KV Query (Alias for Find) ===
✓ Total keys in store: 16

=== KV Delete ===
✓ Deleted key: user:123

=== Verify Deletion ===
✓ Key exists after delete: false

=== KV Batch Delete ===
✓ Batch deleted 3 keys
  config:db: deleted
  config:cache: deleted
  config:api: deleted

=== Example Complete ===

BUILD SUCCESSFUL in 6s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: cy53LJXWlewzGN3Ue33yL7nh_vRM04PnS7kIEoqfTTuCtAz6mDwQyn_9HQM7H_eXDn2F-tcJ-W93pQ2nZh5p3A
Created Bob: $500 - ID: gs2A7LYelv2CdRVKSkJx1Z3pB2a6ejqVupc5H_QSFrbfbTXRe0VetIDQtW6MCoTMA48IWz7aZzlO7olF23RBOg

=== Example 1: Begin Transaction ===
Transaction ID: 2b8bae66-75c3-40e6-8d96-67360dc28c53

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 7a63639b-2262-463c-ba08-8a27d2c13ec7
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Query Builder Example ===

=== Inserting Sample Data ===
✓ Inserted 5 records

=== Query 1: Equality (city = 'NYC') ===
✓ Found 2 records in NYC

=== Query 2: Range (age >= 25 AND age < 32) ===
✓ Found 3 records with age 25-31

=== Query 3: Sort by score (descending) ===
✓ Top 3 scores:
  - Score: ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=95)})
  - Score: ObjectValue(value={value=IntegerValue(value=92), type=StringValue(value=Integer)})
  - Score: ObjectValue(value={value=IntegerValue(value=88), type=StringValue(value=Integer)})

=== Query 4: Complex (score > 80 AND age >= 25) ===
✓ Found 4 high-scoring adults

=== Query 5: IN (city IN ['NYC', 'LA']) ===
✓ Found 4 records in NYC or LA

=== Query 6: Pagination (skip 2, limit 2) ===
✓ Page 2 (2 records):
  - ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Charlie)})
  - ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Diana)})

=== Query 7: Contains (name contains 'a') ===
✓ Found 2 names containing 'a'

=== Cleanup ===
✓ Deleted collection: kotlin_query_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Search Example ===

=== Inserting Sample Documents ===
✓ Inserted 5 sample documents

=== Basic Text Search ===
✓ Search results for 'programming':
  {"results":[{"record":{"views":{"type":"Integer","value":463},"title":{"type":"String","value":"Rust Programming"},"tags":{"value":"programming,rust,tutorial","type":"String"},"id":"lZJtEX2Z9hyYEzxmgVp5xXK0eOJYAlI84rba_kv5tc-Y-SBTfBNO0Bp0sh6E1YZBfbF93kJwduYMVdeshSkz-g","description":{"value":"Learn Rust programming language with hands-on examples and best practices.","type":"String"}},"score":19.8,"matched_fields":["title","description","tags"]},{"record":{"id":"k7pHXy5UFy0P8Oeye4NTa3HMojiFbajcM-gs-C_S_GOv1tHl0yPtS1Um_EF8Kjlv-gd4BhZ9KjW1eYgFOMpcfw","description":{"value":"Master Python for data analysis, machine learning, and visualization.","type":"String"},"views":{"value":42,"type":"Integer"},"title":{"type":"String","value":"Python for Data Science"},"tags":{"type":"String","value":"programming,python,data-science"}},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"title":{"type":"String","value":"JavaScript Web Development"},"id":"rMabiyVfQKLe1LfbeU8uoIKg-a6X_ojEU_zn-5F3v8OtXG9UiYTiQoIR6JxYZdsbDSs8w0YTDmRqWorPeY7Xnw","tags":{"type":"String","value":"programming,javascript,web"},"views":{"value":56,"type":"Integer"},"description":{"type":"String","value":"Build modern web applications using JavaScript, React, and Node.js."}},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' ===
✓ Found results for 'machine learning'
  {"results":[{"record":{"tags":{"type":"String","value":"ai,machine-learning,python"},"views":{"value":469,"type":"Integer"},"id":"7tUQgdJTJYgi2-6ExZSz1ek6-RjQvIJNqgzgR1G2peV7W4jQEtV74tEVPowcJgDg9qGRi95yZ3q4NAAMJw_5uQ","title":{"type":"String","value":"Machine Learning Basics"},"description":{"value":"Introduction to machine learning algorithms and neural networks.","type":"String"}},"score":2.7,"matched_fields":["description","title","tags"]},{"record":{"description":{"value":"Master Python for data analysis, machine learning, and visualization.","type":"String"},"views":{"value":42,"type":"Integer"},"id":"k7pHXy5UFy0P8Oeye4NTa3HMojiFbajcM-gs-C_S_GOv1tHl0yPtS1Um_EF8Kjlv-gd4BhZ9KjW1eYgFOMpcfw","title":{"type":"String","value":"Python for Data Science"},"tags":{"type":"String","value":"programming,python,data-science"}},"score":1.0,"matched_fields":["description"]},{"record":{"title":{"type":"String","value":"Database Design"},"id":"XUuhkd8xIlBaY1NFD1pq3zPqk1pQjj1vXZaOGpzpCk6Gwl2h7El94hAhb8XunbiclnrImNyj63yL8-Azzu7uKw","views":{"value":935,"type":"Integer"},"tags":{"type":"String","value":"database,design,sql"},"description":{"value":"Learn database design principles, normalization, and query optimization.","type":"String"}},"score":0.5,"matched_fields":["description"]},{"record":{"views":{"type":"Integer","value":463},"tags":{"value":"programming,rust,tutorial","type":"String"},"id":"lZJtEX2Z9hyYEzxmgVp5xXK0eOJYAlI84rba_kv5tc-Y-SBTfBNO0Bp0sh6E1YZBfbF93kJwduYMVdeshSkz-g","title":{"type":"String","value":"Rust Programming"},"description":{"value":"Learn Rust programming language with hands-on examples and best practices.","type":"String"}},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

=== Cleanup ===
✓ Deleted collection: kotlin_search_example

✓ All search operations completed successfully

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'kotlin_schema_example' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: StringValue(value=hZRu42poH-3BDtKA-AI71UfADB-JeF4K17q_lCXGED44hpqc8zWdWoUkTUsK3zLnc7AKPaFEtfs1mXCTT5J8rA)
✓ Inserted user 2: StringValue(value=1gPt9M0Hy3r6QqdAstzD3e-dDGAYi-dMYeD9lZRgW9eGzEUg4m12UTtsddNqOTnUCAwig-mwugW3YcrQwJUdcg)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections ===
✓ Total collections: 29
  Sample: [schema_users_client_py, test_collection, users, schema_users_client_go, chat_messages__ek0_testing]

=== Cleanup ===
✓ Deleted collection: kotlin_schema_example

✓ All schema management operations completed successfully

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Join Operations Example ===

=== Setting up sample data ===
✓ Sample data created

=== Example 1: Single collection join (users with departments) ===
✓ Found 2 users with department data
  - Bob Smith: Sales
  - Alice Johnson: Engineering

=== Example 2: Join with filtering ===
✓ Found 1 users in Engineering
  - Alice Johnson: Building A

=== Example 3: Join with user profiles ===
✓ Found 2 users with profile data
  - Bob Smith: Sales Manager
  - Alice Johnson: Senior Software Engineer

=== Example 4: Join orders with user data ===
✓ Found 2 completed orders
  - Laptop ($1200) by Alice Johnson
  - Mouse ($25) by Alice Johnson

=== Example 5: Complex join with multiple conditions ===
✓ Found 2 users with example.com emails
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✓ Deleted test collections

✓ Join operations example completed successfully

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Document TTL Example ===

=== Insert with TTL ===
✓ Inserted document with 10s TTL
  Document ID: 8pv_CpJSHRAX4muwI8odzbtNtV0rrz09DT0e09lF9KWlrAPdajmutygf_be0Rxkbm-sODWErj8_t6eaTwzx5Nw

=== Verify Document Exists ===
✓ Document found: user_id, id, session_id, created_at, ttl

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: 1qvT8YBMX-TVLQ0DX_-j2NkM1AJl2fR0KoxhBZSQb3wvgNkzIsuSSiZrje6soO7unxzreOYaurzFy81sk25GmA

=== TTL Expiration ===
✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document ===
✓ Long TTL document still exists: value, id, cache_key, ttl

=== Delete Document ===
✓ Deleted document

=== Cleanup ===
✓ Deleted collection: kotlin_ttl_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - WebSocket TTL Example ===

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: StringValue(value=d1Vo6X79as6TUWynI3bf-Q8ioecGxhD3ZzvURWmHkHmL_VujqzepUtLqn71EURZYXz5K801LJ-7cpO3uwuBsZg)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"data":[{"created_at":{"type":"Integer","value":1776012472006},"id":"d1Vo6X79as6TUWynI3bf-Q8ioecGxhD3ZzvURWmHkHmL_VujqzepUtLqn71EURZYXz5K801LJ-7cpO3uwuBsZg","name":{"type":"String","value":"WebSocket TTL Test"},"ttl":"2026-04-12T17:47:52.102618Z","value":{"type":"Integer","value":42}}]}

✓ WebSocket closed

=== Cleanup ===
✓ Deleted collection: kotlin_websocket_ttl_example

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB as Edge Cache - Simple Example ===

Setting up edge cache collection...
✓ Cache entry created

Creating edge cache lookup function...
✓ Edge cache function created: gUFYZN_-VbWuKSYX60Fild6RLWxjmnafx52WLoScVK_QKi_8gPvylyvjt6DEUNkoKHVNSQLkOOdtVnrgCICnOw

Call 1: Cache lookup
Found 1 cached entries
Response time: 28ms

Call 2: Cache lookup (connection warm)
Found 1 cached entries
Response time: 20ms

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

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
🚀 ekoDB Functions Example (Kotlin Client)

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✅ Client initialized

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: GOy1uaD3hCDlKT5lqt4O98et2umMR3G3wnyjqqYt3SPnNpLTIvDML3WnXmxIf5E1Wuvum3HcptArgWm9XJK6vw
📊 Found 15 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

✅ Function saved
📊 Found 15 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 3 groups
   {"count":5,"status":"active","avg_score":60.0}
   {"count":5,"avg_score":50.0,"status":"inactive"}
   {"status":"null","count":5,"avg_score":60.0}
⏱️  Execution time: 0ms

📝 Example 4: function Management

📋 Total functions: 36
🔍 Retrieved function: Get Active Users
✏️  function updated
🗑️  function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

📝 Example 5: Multi-Stage Pipeline

✅ Multi-stage function saved
📊 Pipeline executed 2 stages
⏱️  Total execution time: 0ms
📈 Stage breakdown:

📝 Example 6: Count Users

✅ Count function saved
📊 Total user count: 15
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Deleted collection
✅ Deleted test functions

✅ All examples completed successfully!

💡 Key Advantages of Using the Client:
   • Automatic token management
   • Type-safe Stage builders
   • Built-in error handling

BUILD SUCCESSFUL in 4s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
🚀 ekoDB Kotlin Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: qzCAixokjWBBKri8FG4fHNNOYxykkg_nMVf4F7miojxZzjqTV9gTC8OWut7CLU3HIQB0hiyZTfqGXsB3dBsxNA
📊 Found 2 category groups

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories

📝 Example 4: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Function saved
📊 Pipeline executed with 1 results

🧹 Cleaning up...
✅ Cleanup complete

✅ All complete function examples finished!

💡 This example demonstrates ekoDB's function system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Function management (save, call, delete)

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
🚀 ekoDB Kotlin KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within functions
   • Combined wrapped types + KV workflows

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✅ Client initialized

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: StringValue(value=xzMfmDojyk5g6KCUYsn11pLHyqMFfF4ObofpvwpGyC5ngO_OBcXz5T_atiIJ7HWgz7Bw2Mg_O9ZhpZrOMoy4aw)
✅ Inserted 2 products

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: 2hKpfo_dKIdjFus1jRvsz9xWWqN_YgkeD8qgBN83E_6jNkkVXZ5oMX3gkMMHgPuyQfsiiLkdz3vdKidjW84NMw
📊 Created order via function
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"userId":"user_abc","role":"admin"}
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: 6HeWKRYu8MT64Lfp8G0x2DrQhivYcJQBQB41P5zfGTFt3r955adyXPMxSsNJzV--hyYqzdVMX4QGwX7Xk1QJRg
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: gsrsmGP4Bi1j5EC1cezShDppsDu30chudkoHNdHJXC9zEisecFKCo9f9PgiSmntpAth13zoIKNSFnKwjhzFk4g
📊 Processed order with caching
⏱️  Stages executed: 3
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All KV & Wrapped Types examples completed!

💡 Key takeaways:
   ✅ Use field* helpers for type-safe wrapped values
   ✅ fieldDecimal() preserves precision (no floating point errors)
   ✅ KV store is great for caching and quick lookups
   ✅ FunctionStageConfig.Kv* classes work within functions

BUILD SUCCESSFUL in 4s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Setting up cache collection...
✓ Cache entry created

Step 2: Create SWR cache lookup function
✓ Created SWR function: swr_cache_lookup_kt (zCGVnYttEE4sTNzvKrZuFglb_ebQIurxyT8ZeTSseVGJLV_gAKjCAtJUBW12Mj9-L3I1-d8XFsuWYl8R8BEuMQ)

Step 3: First call - Cache lookup
Found 1 cached entries
✓ Cache lookup complete

Step 4: Second call - Fast cache hit
Response time: 18ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
🚀 ekoDB Kotlin Client - Native SWR Function Examples

📋 Demonstrates:
   • Single-function SWR pattern (replaces 4-step pipeline)
   • Automatic cache checking, HTTP fetching, and cache setting
   • Built-in audit trail support
   • Duration string TTLs ('15m', '1h', '30s')
   • Multi-function pipeline integration
   • Dynamic TTL configuration

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.

Example 1: Basic Native SWR
────────────────────────────────────────────────────────────────────────────────
Single function replaces KvGet → If → HttpRequest → KvSet pipeline
✓ Created native SWR function: github_user_native (pspmKohifYO2CPolfZy8fpHbaApveHT8_-t6Sk2FJJrt4-y16f4Rd2kg8gfGpLDxH2M9PTZ2jd-RpconaqgLLw)

First call (cache miss - will fetch from GitHub API):
  Response time: 27ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 17ms
  Speedup: 1.6x faster 🚀


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR function with audit trail: product_swr_audit (bJyJPXvwP0TnVMBJBi6b8Ngv5Gb4CKuPkDD6arBLXTy6ahq6K9dHbHGD-0OXJ0ZCHMenEoFZDvzvxYlAzlko3Q)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (diqSFpr_mmDamrFvRrxZN19iublouEYRnijgV-sh6AIMzVpF61UPafmfMGh0gscACio0H_m0aAqYLCEYXQgsYQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL function: flexible_cache (Tb55cgOaW-PKBV2Z8dYc_GkjemwrIjoGITm2NguTzzm98kbSv28avpuDxoJOUXOX4yUfLdMomyYrRIa89VW_gQ)
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

BUILD SUCCESSFUL in 4s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
🚀 ekoDB Kotlin Advanced Functions Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Function saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Function saved
📊 Category breakdown:
   {"category":"Furniture","avg_price":365.6666666666667,"count":3}
   {"count":5,"category":"Electronics","avg_price":367.0}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced function examples finished!

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
🚀 ekoDB Kotlin AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat function saved
🤖 AI Response:
   {"tokens_used":232,"response":"Vector databases offer several benefits:\n\n1. **Efficient Similarity Search**: They are optimized for high-dimensional vector data, enabling fast retrieval of similar items using techniques like nearest neighbor search.\n\n2. **Scalability**: Vector databases can handle large amounts of data efficiently, making them suitable for applications with big datasets.\n\n3. **Performance**: They provide high performance for complex queries involving vector embeddings, such as those generated by machine learning models.\n\n4. **Flexibility**: Support for various data types such as text, images, and audio, allowing for versatile applications across different domains.\n\n5. **Real-time Processing**: Enable real-time analytics and insights, which are essential for dynamic applications like recommendation systems.\n\n6. **Integration with AI/ML**: Seamlessly integrate with machine learning frameworks, facilitating model deployment and inference directly on the vector space.\n\n7. **Enhanced Data Retrieval**: Improve user experience by providing more relevant and context-aware search results through semantic similarity rather than exact matches."}
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embed function saved
📊 Embedding generated
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All AI function examples finished!

💡 This example demonstrates:
   ✅ Chat completions with system/user messages
   ✅ Embedding generation for text

BUILD SUCCESSFUL in 8s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
🚀 ekoDB Kotlin CRUD Functions Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Function saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Function saved
📊 User counts by status:
   {"status":"active","count":7}
   {"status":"inactive","count":3}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD function examples finished!

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
🚀 ekoDB Kotlin Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. {"type":"String","value":"Natural Language Processing"} ({"type":"String","value":"AI"})
   2. {"type":"String","value":"Vector Databases Explained"} ({"type":"String","value":"Database"})
   3. {"value":"Introduction to Machine Learning","type":"String"} ({"value":"AI","type":"String"})
   4. {"type":"String","value":"Getting Started with ekoDB"} ({"value":"Database","type":"String"})
   5. {"type":"String","value":"Database Design Principles"} ({"type":"String","value":"Database"})
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"count":2,"category":"AI"}
   {"category":"Database","count":3}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search function examples finished!

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Basic Chat Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: AdXwjH4oYtGrTLzIBPGRtUY6tnerUJbDWMISK1x5tC0qBWRSr-ZbvDZDQFHFYbNJwDajpEgTyx3UmuAg85LI8A

=== Sending Chat Message ===
✓ Chat response:
  Message ID: "uD_htPW4fPNJWUV466j6hxynHmSZ0qNCcLiFK3XBXOJ0tElsHAYjZ00JdoO468W_5hY1MkbO9hM4yIJidh3-Rg"
  Responses: ["ekoDB is a high-performance database that supports various advanced functionalities:\n\n1. **AI Chat Integration**: The chat feature allows you to query your database using natural language and receive AI-powered responses with relevant context.\n\n2. **Search Features**: ekoDB offers full-text search, vector search, and hybrid search capabilities, enabling users to retrieve content effectively while maintaining context.\n\n3. **Real-Time Capabilities**: It is designed for real-time data operations, making it suitable for applications that require immediate data access and processing.\n\n4. **Intelligent Caching**: ekoDB employs intelligent caching mechanisms to enhance performance and reduce latency during data retrieval.\n\nOverall, ekoDB combines advanced search capabilities with AI integration and performance optimization features, making it a powerful option for various database needs."]

=== Cleanup ===
✓ Deleted chat session
✓ Deleted collection: kotlin_chat_basic_example

✓ Basic chat example completed successfully

BUILD SUCCESSFUL in 8s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: IuwX1-pBcG-yaXHGbhE5h_yXJjMTRQL9duAdsLOLtbZ5FJpj1XbFknM6fZYTt_nFf6xse3rNc8T_KDHfamu9sQ

=== Sending Initial Message ===
✓ Message sent
  Responses: ["The available product is:\n\n- **Name:** ekoDB\n- **Description:** High-performance database product\n- **Price:** $99\n\nIf you need more information or have other questions, feel free to ask!"]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["Currently, there is one product available:\n\n- **Name:** ekoDB\n- **Description:** High-performance database product\n- **Price:** $99\n\nIf you have any more questions or need additional information, feel free to ask!"]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: aTYjig4aAqJ3DzuxvYCVgk9GT86asFp0LWcFKteLnduCCcr9b8DPyI0e380RhdyRtQFMuGTqoYlddiImZmaCig

=== Merging Sessions ===
✓ Merged sessions
  Total messages in merged session: 2

=== Deleting Message ===
✓ Deleted message

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_advanced_example

✓ Advanced chat features example completed successfully

BUILD SUCCESSFUL in 16s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: A4nw7EUcs9TOFK9QRdJmQs_cKAZ95kcCit6xprp805BvxuvuqkMdwowiZ7abfTGAKSkwTKTnUC4gK1hYLrwypA

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["The available product is:\n\n- **Product Name:** ekoDB\n- **Price:** $99\n- **ID:** VXMQ85TEwjU6OCGsqzeRpxc8SWbXRp0OB4liQND9ksw69IpbYUkLL-3H2TSHPnPkVlGAxA2gQfNPOQjqv0JB8Q\n\nIf you need more information or additional products, let me know!"]

✓ Message 2 sent
  Responses: ["The price of the ekoDB product is **$99**."]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"chat_id":{"type":"String","value":"A4nw7EUcs9TOFK9QRdJmQs_cKAZ95kcCit6xprp805BvxuvuqkMdwowiZ7abfTGAKSkwTKTnUC4gK1hYLrwypA"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"VXMQ85TEwjU6OCGsqzeRpxc8SWbXRp0OB4liQND9ksw69IpbYUkLL-3H2TSHPnPkVlGAxA2gQfNPOQjqv0JB8Q","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-04-12T16:49:15.279327Z"},"id":"AIy-7iuo_koxpq4p6aGQ3MV4DUXns1OqCfPBtKWhfVEOfTwnyYPPqHIWAWV4h2kRp65dtdWR6Um7JYihIoYjdA","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":82,"prompt_tokens":2504,"total_tokens":2586}},"updated_at":{"type":"DateTime","value":"2026-04-12T16:49:15.279327Z"}},{"chat_id":{"type":"String","value":"A4nw7EUcs9TOFK9QRdJmQs_cKAZ95kcCit6xprp805BvxuvuqkMdwowiZ7abfTGAKSkwTKTnUC4gK1hYLrwypA"},"content":{"type":"String","value":"The available product is:\n\n- **Product Name:** ekoDB\n- **Price:** $99\n- **ID:** VXMQ85TEwjU6OCGsqzeRpxc8SWbXRp0OB4liQND9ksw69IpbYUkLL-3H2TSHPnPkVlGAxA2gQfNPOQjqv0JB8Q\n\nIf you need more information or additional products, let me know!"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"VXMQ85TEwjU6OCGsqzeRpxc8SWbXRp0OB4liQND9ksw69IpbYUkLL-3H2TSHPnPkVlGAxA2gQfNPOQjqv0JB8Q","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-04-12T16:49:15.284450Z"},"id":"f6k2Ry4vY-6C8MRlw6djznQcEiDgxHkkPpS7t9enk7sOye9p9zHeU2wI_Y694TXgGohGSChV9VU9b7UqiHpNxQ","llm_model":{"type":"String","value":"gpt-4o-mini"},"llm_provider":{"type":"String","value":"openai"},"role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":82,"prompt_tokens":2504,"total_tokens":2586}},"tool_call_count":{"type":"Number","value":2},"tool_call_history":{"type":"Object","value":{"iterations":2,"tool_calls":[{"arguments":{"collection":"kotlin_chat_sessions_example","limit":10,"select_fields":["product","id","price"]},"id":"call_N0sV0Cy15yntcDBabAWCOTwx","name":"query_collection"}],"tool_results":[{"error":null,"result":{"count":1,"records":[{"id":"VXMQ85TEwjU6OCGsqzeRpxc8SWbXRp0OB4liQND9ksw69IpbYUkLL-3H2TSHPnPkVlGAxA2gQfNPOQjqv0JB8Q","price":99,"product":"ekoDB"}]},"success":true,"tool_call_id":"call_N0sV0Cy15yntcDBabAWCOTwx","tool_name":"query_collection"}]}},"updated_at":{"type":"DateTime","value":"2026-04-12T16:49:15.284450Z"}},{"chat_id":{"type":"String","value":"A4nw7EUcs9TOFK9QRdJmQs_cKAZ95kcCit6xprp805BvxuvuqkMdwowiZ7abfTGAKSkwTKTnUC4gK1hYLrwypA"},"content":{"type":"String","value":"What is the price?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description","product","price","id"],"record":{"description":"A high-performance database product with AI capabilities","id":"VXMQ85TEwjU6OCGsqzeRpxc8SWbXRp0OB4liQND9ksw69IpbYUkLL-3H2TSHPnPkVlGAxA2gQfNPOQjqv0JB8Q","price":99,"product":"ekoDB"},"score":1.6757575757575758}]},"created_at":{"type":"DateTime","value":"2026-04-12T16:49:18.847706Z"},"id":"uXbgZIFqnCY1q4j4g8cNhN0d_lB1QLpImMQLPih3KdtuiP-jsfopw35Ph3ER7vdpVmmz5NSQx18UsgSolLBnEw","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":10,"prompt_tokens":1287,"total_tokens":1297}},"updated_at":{"type":"DateTime","value":"2026-04-12T16:49:18.847706Z"}},{"chat_id":{"type":"String","value":"A4nw7EUcs9TOFK9QRdJmQs_cKAZ95kcCit6xprp805BvxuvuqkMdwowiZ7abfTGAKSkwTKTnUC4gK1hYLrwypA"},"content":{"type":"String","value":"The price of the ekoDB product is **$99**."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description","product","price","id"],"record":{"description":"A high-performance database product with AI capabilities","id":"VXMQ85TEwjU6OCGsqzeRpxc8SWbXRp0OB4liQND9ksw69IpbYUkLL-3H2TSHPnPkVlGAxA2gQfNPOQjqv0JB8Q","price":99,"product":"ekoDB"},"score":1.6757575757575758}]},"created_at":{"type":"DateTime","value":"2026-04-12T16:49:18.852600Z"},"id":"ka1hAMQ_tLKEMr-hFA_KzElF_Al168w_LiFUabAN-cVaQNJzVFKOP_0WpRlNfk6y90A0LRBCh6v-1d7AE2dEKQ","llm_model":{"type":"String","value":"gpt-4o-mini"},"llm_provider":{"type":"String","value":"openai"},"role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":10,"prompt_tokens":1287,"total_tokens":1297}},"updated_at":{"type":"DateTime","value":"2026-04-12T16:49:18.852600Z"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 16

=== Branching Session ===
✓ Created branched session: 0ki1FD33CHLHMTamGC_T3p-ZbXds6xG__STJkunoLvr6aP-JzAsaWe3PwYmAYfanBsVwNg_zH-qrdj-waVwLIw

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_sessions_example

✓ Chat session management example completed successfully

BUILD SUCCESSFUL in 11s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Chat Models Example ===

=== List Chat Models ===
Available chat models by provider:
  openai:
    - gpt-4-0613
    - gpt-4
    - gpt-3.5-turbo
    - gpt-5.4-mini
    - gpt-5.4
    - gpt-5.4-nano-2026-03-17
    - gpt-5.4-nano
    - gpt-5.4-mini-2026-03-17
    - davinci-002
    - babbage-002
    - gpt-3.5-turbo-instruct
    - gpt-3.5-turbo-instruct-0914
    - dall-e-3
    - dall-e-2
    - gpt-3.5-turbo-1106
    - tts-1-hd
    - tts-1-1106
    - tts-1-hd-1106
    - text-embedding-3-small
    - text-embedding-3-large
    - gpt-3.5-turbo-0125
    - gpt-4-turbo
    - gpt-4-turbo-2024-04-09
    - gpt-4o
    - gpt-4o-2024-05-13
    - gpt-4o-mini-2024-07-18
    - gpt-4o-mini
    - gpt-4o-2024-08-06
    - gpt-4o-audio-preview
    - gpt-4o-realtime-preview
    - omni-moderation-latest
    - omni-moderation-2024-09-26
    - gpt-4o-realtime-preview-2024-12-17
    - gpt-4o-audio-preview-2024-12-17
    - gpt-4o-mini-realtime-preview-2024-12-17
    - gpt-4o-mini-audio-preview-2024-12-17
    - o1-2024-12-17
    - o1
    - gpt-4o-mini-realtime-preview
    - gpt-4o-mini-audio-preview
    - o3-mini
    - o3-mini-2025-01-31
    - gpt-4o-2024-11-20
    - gpt-4o-mini-search-preview-2025-03-11
    - gpt-4o-mini-search-preview
    - gpt-4o-transcribe
    - gpt-4o-mini-transcribe
    - o1-pro-2025-03-19
    - o1-pro
    - gpt-4o-mini-tts
    - o3-2025-04-16
    - o4-mini-2025-04-16
    - o3
    - o4-mini
    - gpt-4.1-2025-04-14
    - gpt-4.1
    - gpt-4.1-mini-2025-04-14
    - gpt-4.1-mini
    - gpt-4.1-nano-2025-04-14
    - gpt-4.1-nano
    - gpt-image-1
    - gpt-4o-realtime-preview-2025-06-03
    - gpt-4o-audio-preview-2025-06-03
    - o4-mini-deep-research
    - gpt-4o-transcribe-diarize
    - o4-mini-deep-research-2025-06-26
    - gpt-5-chat-latest
    - gpt-5-2025-08-07
    - gpt-5
    - gpt-5-mini-2025-08-07
    - gpt-5-mini
    - gpt-5-nano-2025-08-07
    - gpt-5-nano
    - gpt-audio-2025-08-28
    - gpt-realtime
    - gpt-realtime-2025-08-28
    - gpt-audio
    - gpt-5-codex
    - gpt-image-1-mini
    - gpt-5-pro-2025-10-06
    - gpt-5-pro
    - gpt-audio-mini
    - gpt-audio-mini-2025-10-06
    - gpt-5-search-api
    - gpt-realtime-mini
    - gpt-realtime-mini-2025-10-06
    - sora-2
    - sora-2-pro
    - gpt-5-search-api-2025-10-14
    - gpt-5.1-chat-latest
    - gpt-5.1-2025-11-13
    - gpt-5.1
    - gpt-5.1-codex
    - gpt-5.1-codex-mini
    - gpt-5.1-codex-max
    - gpt-image-1.5
    - gpt-5.2-2025-12-11
    - gpt-5.2
    - gpt-5.2-pro-2025-12-11
    - gpt-5.2-pro
    - gpt-5.2-chat-latest
    - gpt-4o-mini-transcribe-2025-12-15
    - gpt-4o-mini-transcribe-2025-03-20
    - gpt-4o-mini-tts-2025-03-20
    - gpt-4o-mini-tts-2025-12-15
    - gpt-realtime-mini-2025-12-15
    - gpt-audio-mini-2025-12-15
    - chatgpt-image-latest
    - gpt-5.2-codex
    - gpt-5.3-codex
    - gpt-realtime-1.5
    - gpt-audio-1.5
    - gpt-4o-search-preview
    - gpt-4o-search-preview-2025-03-11
    - gpt-5.3-chat-latest
    - gpt-5.4-2026-03-05
    - gpt-5.4-pro
    - gpt-5.4-pro-2026-03-05
    - gpt-3.5-turbo-16k
    - tts-1
    - whisper-1
    - text-embedding-ada-002
  anthropic:
    - claude-sonnet-4-6
    - claude-opus-4-6
    - claude-opus-4-5-20251101
    - claude-haiku-4-5-20251001
    - claude-sonnet-4-5-20250929
    - claude-opus-4-1-20250805
    - claude-opus-4-20250514
    - claude-sonnet-4-20250514
    - claude-3-haiku-20240307
  perplexity:
    - sonar
    - sonar-pro
    - sonar-deep-research
    - sonar-reasoning
    - sonar-reasoning-pro

=== Get OpenAI Models ===
OpenAI models: gpt-4-0613, gpt-4, gpt-3.5-turbo, gpt-5.4-mini, gpt-5.4, gpt-5.4-nano-2026-03-17, gpt-5.4-nano, gpt-5.4-mini-2026-03-17, davinci-002, babbage-002, gpt-3.5-turbo-instruct, gpt-3.5-turbo-instruct-0914, dall-e-3, dall-e-2, gpt-3.5-turbo-1106, tts-1-hd, tts-1-1106, tts-1-hd-1106, text-embedding-3-small, text-embedding-3-large, gpt-3.5-turbo-0125, gpt-4-turbo, gpt-4-turbo-2024-04-09, gpt-4o, gpt-4o-2024-05-13, gpt-4o-mini-2024-07-18, gpt-4o-mini, gpt-4o-2024-08-06, gpt-4o-audio-preview, gpt-4o-realtime-preview, omni-moderation-latest, omni-moderation-2024-09-26, gpt-4o-realtime-preview-2024-12-17, gpt-4o-audio-preview-2024-12-17, gpt-4o-mini-realtime-preview-2024-12-17, gpt-4o-mini-audio-preview-2024-12-17, o1-2024-12-17, o1, gpt-4o-mini-realtime-preview, gpt-4o-mini-audio-preview, o3-mini, o3-mini-2025-01-31, gpt-4o-2024-11-20, gpt-4o-mini-search-preview-2025-03-11, gpt-4o-mini-search-preview, gpt-4o-transcribe, gpt-4o-mini-transcribe, o1-pro-2025-03-19, o1-pro, gpt-4o-mini-tts, o3-2025-04-16, o4-mini-2025-04-16, o3, o4-mini, gpt-4.1-2025-04-14, gpt-4.1, gpt-4.1-mini-2025-04-14, gpt-4.1-mini, gpt-4.1-nano-2025-04-14, gpt-4.1-nano, gpt-image-1, gpt-4o-realtime-preview-2025-06-03, gpt-4o-audio-preview-2025-06-03, o4-mini-deep-research, gpt-4o-transcribe-diarize, o4-mini-deep-research-2025-06-26, gpt-5-chat-latest, gpt-5-2025-08-07, gpt-5, gpt-5-mini-2025-08-07, gpt-5-mini, gpt-5-nano-2025-08-07, gpt-5-nano, gpt-audio-2025-08-28, gpt-realtime, gpt-realtime-2025-08-28, gpt-audio, gpt-5-codex, gpt-image-1-mini, gpt-5-pro-2025-10-06, gpt-5-pro, gpt-audio-mini, gpt-audio-mini-2025-10-06, gpt-5-search-api, gpt-realtime-mini, gpt-realtime-mini-2025-10-06, sora-2, sora-2-pro, gpt-5-search-api-2025-10-14, gpt-5.1-chat-latest, gpt-5.1-2025-11-13, gpt-5.1, gpt-5.1-codex, gpt-5.1-codex-mini, gpt-5.1-codex-max, gpt-image-1.5, gpt-5.2-2025-12-11, gpt-5.2, gpt-5.2-pro-2025-12-11, gpt-5.2-pro, gpt-5.2-chat-latest, gpt-4o-mini-transcribe-2025-12-15, gpt-4o-mini-transcribe-2025-03-20, gpt-4o-mini-tts-2025-03-20, gpt-4o-mini-tts-2025-12-15, gpt-realtime-mini-2025-12-15, gpt-audio-mini-2025-12-15, chatgpt-image-latest, gpt-5.2-codex, gpt-5.3-codex, gpt-realtime-1.5, gpt-audio-1.5, gpt-4o-search-preview, gpt-4o-search-preview-2025-03-11, gpt-5.3-chat-latest, gpt-5.4-2026-03-05, gpt-5.4-pro, gpt-5.4-pro-2026-03-05, gpt-3.5-turbo-16k, tts-1, whisper-1, text-embedding-ada-002

=== Get Anthropic Models ===
Anthropic models: claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929, claude-opus-4-1-20250805, claude-opus-4-20250514, claude-sonnet-4-20250514, claude-3-haiku-20240307

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Request failed with status 405: {"code":405,"message":"Method Not Allowed"}

=== Chat Models Example Complete ===

BUILD SUCCESSFUL in 6s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - User Functions Example ===

=== Create User Function ===
Created user function with ID: e4VK9tHFWXrpDtN9semjM4lkD0uOA9wSNLrY8pkBg_6LW4UX9um4JBr87bAzR7J4NnNewPyQ19Z3Qta9Toj8Dw

=== Get User Function ===
Retrieved: "get_active_users_kt" - "Get Active Users (Kotlin)"
Description: "Fetches all users and filters by active status"

=== List All User Functions ===
Found 31 user functions:
  - "validate_user": "Check if user exists"
  - "fetch_and_store_user": "Fetch user from API and cache in KV"
  - "cache_api_call": "Cache External API Call"
  - "swr_user": "SWR pattern for user data (KV-based)"
  - "get_active_users_kt": "Get Active Users (Kotlin)"

=== List User Functions by Tag ===
Found 31 user functions with 'kotlin' tag:
  - "validate_user"
  - "fetch_and_store_user"
  - "cache_api_call"
  - "swr_user"
  - "get_active_users_kt"
  - "fetch_user"
  - "fetch_and_store_user"
  - "fetch_slim_user"
  - "cache_api_call_py"
  - "fetch_and_store_user"
  - "validate_user"
  - "fetch_user"
  - "fetch_slim_user"
  - "fetch_api_user"
  - "cache_api_call_js"
  - "fetch_slim_user"
  - "fetch_slim_user"
  - "validate_user"
  - "swr_user"
  - "validate_user"
  - "swr_user"
  - "fetch_user"
  - "swr_user"
  - "validate_user"
  - "fetch_and_store_user"
  - "fetch_slim_user"
  - "fetch_user"
  - "swr_user"
  - "fetch_user"
  - "fetch_and_store_user"
  - "fetch_product_with_reviews"

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

=== User Functions Example Complete ===

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
=== ekoDB Convenience Methods Example ===

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== Native Object Creation ===
✓ Created record: EkoRecord(fields={id=StringValue(value=UwNb00UxtJl6CLF6r3gguSktKD5PAR66RMdx7fcVGb-EWqMfZgj_Xvaj_8ViDMOVJKeV9eXjFU6TnKJFvVhGRA)})

=== Upsert Operation ===
✓ First upsert (update): EkoRecord(fields={email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice.j@newdomain.com)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), id=StringValue(value=UwNb00UxtJl6CLF6r3gguSktKD5PAR66RMdx7fcVGb-EWqMfZgj_Xvaj_8ViDMOVJKeV9eXjFU6TnKJFvVhGRA)})
✓ Second upsert (insert): EkoRecord(fields={id=StringValue(value=vqdKFzIZCUOfZbd1SLLHlCvTXWdls7n_arEk2PWUKn25vcC5Q6iuk-4sPISoF_2cWLIuGwSHlOwkUlEYkcKONg)})

=== Find One Operation ===
✓ Found user by email: EkoRecord(fields={age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), id=StringValue(value=UwNb00UxtJl6CLF6r3gguSktKD5PAR66RMdx7fcVGb-EWqMfZgj_Xvaj_8ViDMOVJKeV9eXjFU6TnKJFvVhGRA), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), email=ObjectValue(value={value=StringValue(value=alice.j@newdomain.com), type=StringValue(value=String)})})
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

BUILD SUCCESSFUL in 10s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
=== Bypass Ripple Example ===

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
1. Basic insert (ripple enabled):
   Inserted with ripple: EkoRecord(fields={id=StringValue(value=GXs98A9LQ3jOEkiMeuYvWQJrk4RPBNaPDxcZPbtXgSYHM1FeuFzYo2mUA5jIc9gOykguMadOcnnbxDrYcz0mBA)})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: EkoRecord(fields={id=StringValue(value=WFeK2atjHiAiysmmES6ibWSZvAUn5BmWZslpzkFDTNwTJx6X3MtNwrUjvNHBdJzip1Xs25sgTTGtBhTcIcXD_g)})

3. Update with bypass_ripple:
   Updated with bypass_ripple: EkoRecord(fields={name=ObjectValue(value={value=StringValue(value=Product 1), type=StringValue(value=String)}), price=ObjectValue(value={value=IntegerValue(value=150), type=StringValue(value=Integer)}), id=StringValue(value=GXs98A9LQ3jOEkiMeuYvWQJrk4RPBNaPDxcZPbtXgSYHM1FeuFzYo2mUA5jIc9gOykguMadOcnnbxDrYcz0mBA)})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: EkoRecord(fields={id=StringValue(value=GXs98A9LQ3jOEkiMeuYvWQJrk4RPBNaPDxcZPbtXgSYHM1FeuFzYo2mUA5jIc9gOykguMadOcnnbxDrYcz0mBA), price=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=500)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Upsert Product)})})

✅ All bypass_ripple operations completed successfully!

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields
Fetched 3 users with only 3 fields each

Example 2: Exclude sensitive fields
Fetched 2 admins without sensitive data
  Password field excluded: true

Example 3: Complex query with projection
Fetched 3 active users with profile fields

Example 4: Find by ID with projection
Fetched user profile: Alice Johnson

Example 5: Compare full vs projected data
Full query returned 12 fields per user
Projected query returned 3 fields per user

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
✅ [32mKotlin client examples complete![0m
🟣 [33mKotlin Transactions...[0m
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: Y9Zn2_SLQADmvSamGpEHo5YGmA5rajxeG1j-Qq4WFAHpK6nUSnAzVRE3Z7mp8t08rTQZoWWBmbBcDq-d4Jps6w
Created Bob: $500 - ID: XWOeNJKEOOt-rg6KSAVT7Wntb9YES5J-lJgzUwVhKfyhML71EX-qzP9aXeMRJFb_HwluljdCibp7BnGY3PrlRA

=== Example 1: Begin Transaction ===
Transaction ID: 30b43d23-9ac1-43ad-93c7-d8c89b8b75a5

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: d82f67ce-98ab-4fde-9a33-8668885daca2
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mAll Kotlin integration tests complete![0m

🤖 [36mRAG Conversation System Examples[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m

[33mPrerequisites:[0m
  1. ekoDB server running (make run in ekodb/)
  2. OPENAI_API_KEY set in server environment
  3. API_BASE_URL and API_BASE_KEY exported in your shell

[36mBuilding Rust client library...[0m
✓ Rust client built

[36mBuilding Python client bindings...[0m
🍹 Building a mixed python/rust project
🔗 Found pyo3 bindings with abi3 support
🐍 Found CPython 3.11 at /Library/Frameworks/Python.framework/Versions/3.11/bin/python3
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.16.0-cp38-abi3-macosx_11_0_arm64.whl
✓ Python client built and installed

[36mBuilding TypeScript client library...[0m
✓ TypeScript client built

[36mInstalling TypeScript client in examples...[0m
✓ TypeScript client installed

[36mBuilding TypeScript example...[0m
✓ TypeScript example built

[36mBuilding Go client library...[0m
✓ Go client built

[36mBuilding Go RAG example...[0m
✓ Go example built

[36mBuilding Kotlin client library...[0m
✓ Kotlin client built

[36mBuilding Kotlin RAG example...[0m
✓ Kotlin example built

[36mRunning Rust RAG Example...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB RAG Conversation System ===

This example shows how ekoDB can power a self-improving AI system
that learns from its own conversation history.

=== Step 1: Building Conversation History ===
Storing previous conversations with embeddings...

  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 34 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 690.296375ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 505.082458ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 761.65075ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 613.165125ms
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 437.894875ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 475.155958ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 383.82125ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 418.453625ms
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 503.203083ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 464.073958ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 500.792042ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 988.3775ms
    • Function auto-cleaned up by client
✓ Stored performance optimization conversation (4 messages)

=== Step 2: New User Question with Context Retrieval ===
User asks: "How do I write memory-safe high-performance database code?"

=== Step 3: Searching Related Context ===
Using hybrid search to find relevant messages from all conversations...


→ Generating embedding for user question...
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 381.54375ms
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 102.684667ms

✓ Found 5 related messages across all conversations:
  1. From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  2. From conv_database_design
     What is database normalization?

  3. From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

  4. From conv_database_design
     When should I use NoSQL over SQL?

  5. From conv_performance
     How can I optimize database queries?

=== Step 4: Generating Context-Aware Response ===
✓ AI Response (with context from 3 conversations):

Writing memory-safe, high-performance database code involves several best practices that consider both application performance and data integrity. Here are some key strategies to achieve this:

### 1. **Use Prepared Statements**
   - **Safety**: Prepared statements help prevent SQL injection attacks by separating SQL logic from data.
   - **Performance**: They can improve performance since the database can cache the execution plan.

### 2. **Connection Pooling**
   - Use connection pools to manage database connections efficiently. This reduces the overhead of establishing connections repeatedly and improves throughput.

### 3. **Transactions**
   - Use transactions to ensure ACID compliance. This will maintain data integrity across multiple operations.
   - In high-concurrency environments, consider using optimistic concurrency control to reduce locking contention.

### 4. **Proper Indexing**
   - Create indexes on frequently queried columns to optimize read performance. 
   - Analyze your queries using tools like `EXPLAIN` to identify where additional indexing may be beneficial.
   - Keep in mind that while indexes improve read performance, they can slow down write operations.

### 5. **Batch Processing**
   - For bulk inserts, updates, or deletes, use batch operations to reduce the number of round-trips to the database, which can improve performance.

### 6. **Efficient Query Design**
   - Write efficient and concise SQL queries to minimize execution time. Avoid SELECT * and use only the columns you need.
   - Utilize derived tables, common table expressions (CTEs), or views to simplify complex queries, but be cautious of performance impacts.

### 7. **Handle Memory Management Explicitly**
   - In languages that require manual memory management (like C or C++), ensure you properly allocate and deallocate memory.
   - Use smart pointers in C++ to automatically manage the lifecycle of objects, reducing the likelihood of memory leaks.

### 8. **Optimize Data Types**
   - Use appropriate data types for your database columns. For example, use INTEGER instead of BIGINT if a smaller range suffices. This can save both space and improve performance.

### 9. **Asynchronous Operations**
   - For I/O-bound applications, consider using asynchronous database operations to avoid blocking, which can improve overall throughput.

### 10. **Monitoring and Profiling**
   - Continuously monitor your database performance and query execution times.
   - Use profiling tools to identify bottlenecks in your code or database queries.

### 11. **Data Structure Choices**
   - Choose appropriate data structures based on the nature of operations being performed.
   - Consider using NoSQL databases if your data is highly unstructured or requires flexible schema evolution, as they can offer better performance for certain workloads.

### 12. **Profiling and Testing**
   - Regularly profile your application to identify memory and performance hotspots.
   - Implement unit and integration tests to ensure that your database interactions behave as expected while maintaining performance.

By carefully incorporating these practices, you can write database code that is not only memory-safe but also high-performance, leading to more robust and scalable applications.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 796.812ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 3285 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 689.116042ms
    • Function auto-cleaned up by client
✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...


→ Executing text_search()...
  • Collection: rag_messages
  • Query: "ownership system"
  • Limit: 3 results
  • Search method: Full-text with fuzzy matching & stemming
  • No vector embeddings needed - pure keyword search
  ✓ Text search completed in 68.443375ms

✓ Found 3 messages mentioning ownership:
  1. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  2. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  3. From conv_rust_programming: The borrow checker enforces Rust's ownership rules at compile time. It ensures that references don't outlive the data they point to and prevents data races by allowing either multiple immutable references or one mutable reference.

=== System Statistics ===

→ Querying database statistics...
  • Using find_all() helper - simplified query API

📊 Database Statistics:
  • Total conversations: 4
  • Total messages stored: 14
  • All messages indexed for vector search ✓
  • All messages indexed for text search ✓
  • All messages queryable by metadata ✓

=== Step 8: Dynamic Search Configuration ===
Each conversation can have its own search config...

💡 Conversations can store custom search configurations:
  • Search type: hybrid, text, or vector
  • Relevance thresholds
  • Filter by tags or metadata
  • Collection-specific settings
  • Per-conversation AI behavior

This enables context-aware search tuned to each conversation's needs!


=== Cleanup ===
Deleting example collections...

✅ All done! RAG system demonstrated successfully.

✓ Using search results to enhance AI responses (RAG)
✓ Building a self-improving knowledge base
✓ Dynamic search configurations per conversation

ekoDB provides everything needed for AI-powered applications:
  • Vector search (semantic similarity)
  • Text search (keyword matching)
  • Hybrid search (best of both)
  • AI functions (Chat, Embed)
  • Flexible querying and filtering
  • All in one database - no external dependencies!


[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Python RAG Example...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB RAG Conversation System ===

This example shows how ekoDB can power a self-improving AI system
that learns from its own conversation history.

=== Step 1: Building Conversation History ===
Storing previous conversations with embeddings...

  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 34 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.469s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.471s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.384s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.591s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.616s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.553s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.355s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.419s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.363s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.488s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.491s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.491s
    • Function auto-cleaned up by client
✓ Stored performance optimization conversation (4 messages)

=== Step 2: New User Question with Context Retrieval ===
User asks: "How do I write memory-safe high-performance database code?"

=== Step 3: Searching Related Context ===
Using hybrid search to find relevant messages from all conversations...


→ Generating embedding for user question...
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.470s
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.093s

✓ Found 5 related messages across all conversations:
  1. [Score: 0.331] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

  2. [Score: 0.300] From conv_database_design
     What is database normalization?

  3. [Score: 0.300] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  4. [Score: 0.256] From conv_database_design
     When should I use NoSQL over SQL?

  5. [Score: 0.188] From conv_performance
     How can I optimize database queries?

=== Step 4: Generating Context-Aware Response ===
✓ AI Response (with context from 3 conversations):

Writing memory-safe, high-performance database code involves a combination of best practices and techniques to ensure that your application handles data efficiently and without memory leaks or unsafe operations. Here’s a structured approach:

### 1. Choose the Right Database Technology
- **Relational vs. NoSQL**: Select a database that aligns with your data model. Use relational databases (SQL) for well-defined schemas and complex queries, and NoSQL for flexibility and high write throughput with unstructured data.
- **Connection Pooling**: Use a connection pool to manage database connections efficiently, reducing the overhead of frequently opening and closing connections.

### 2. Use Language Features Wisely
- **Memory Management**: Utilize memory-safe programming languages (like Rust or Swift) or frameworks that support automatic memory management (like Java with garbage collection).
- **Strong Typing**: Leverage strong typing to minimize type-related errors and undefined behaviors.

### 3. Optimize Queries
- **Query Optimization**: Analyze and optimize SQL queries to reduce execution time. Use indexing judiciously to enhance query performance, especially for columns frequently used in WHERE clauses and JOIN statements.
- **Batch Operations**: Use batch inserts and updates to minimize the number of calls to the database and reduce overhead.

### 4. Effective Use of Transactions
- **ACID Compliance**: Ensure transactions are atomic, consistent, isolated, and durable. Use transactions to group multiple operations into a single unit of work.
- **Avoid Long-Running Transactions**: Keep transactions short to minimize locking and maintain performance.

### 5. Monitor and Analyze Performance
- **Profiling**: Regularly profile your database interactions to identify bottlenecks. Tools like query execution plans can help diagnose slow queries.
- **Logging**: Implement logging for error handling and troubleshooting, but ensure it doesn’t become a performance bottleneck.

### 6. Implement Caching
- **Use In-Memory Caching**: Implement caching strategies (like Redis or Memcached) to store frequently accessed data, reducing the need for repeated database queries.

### 7. Use Frameworks and Libraries
- **ORMs**: Use Object-Relational Mappers (ORMs) when appropriate; they can help with memory management and improve code readability, but be cautious about performance overhead.
- **Database Abstraction**: Using a well-defined interface for database interactions can help isolate changes and improve maintainability.

### 8. Ensure Data Integrity
- **Validation**: Validate inputs rigorously to prevent SQL injection attacks. Utilize prepared statements and parameterized queries.
- **Normalization**: Normalize your database schema to minimize redundancy and maintain data integrity, but balance this with the performance needs of your application.

### 9. Keep Resources Managed
- **Close Connections Properly**: Always ensure that database connections are closed after use. Use constructs that automatically release resources (like "using" statements in C# or try-with-resources in Java).
- **Error Handling**: Implement robust error handling to gracefully manage exceptions and avoid resource leaks.

### Example in Rust
If you're using Rust, you could leverage its ownership model for memory safety:

```rust
use rusqlite::{params, Connection, Result};

fn insert_user(conn: &Connection, name: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO user (name) VALUES (?1)",
        params![name],
    )?;
    Ok(())
}

fn main() -> Result<()> {
    let conn = Connection::open("my_database.db")?;
    insert_user(&conn, "Alice")?;
    Ok(())
}
```

### Conclusion
By following these best practices, you can build database code that is both memory safe and high performance. Continuous monitoring and iterative improvements are key to maintaining optimal performance as system demands evolve.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.360s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 3929 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.526s
    • Function auto-cleaned up by client
✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...

✓ Found 3 messages mentioning ownership:
  1. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  2. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  3. From conv_new_question: Writing memory-safe, high-performance database code involves a combination of best practices and techniques to ensure that your application handles data efficiently and without memory leaks or unsafe operations. Here’s a structured approach:

### 1. Choose the Right Database Technology
- **Relational vs. NoSQL**: Select a database that aligns with your data model. Use relational databases (SQL) for well-defined schemas and complex queries, and NoSQL for flexibility and high write throughput with unstructured data.
- **Connection Pooling**: Use a connection pool to manage database connections efficiently, reducing the overhead of frequently opening and closing connections.

### 2. Use Language Features Wisely
- **Memory Management**: Utilize memory-safe programming languages (like Rust or Swift) or frameworks that support automatic memory management (like Java with garbage collection).
- **Strong Typing**: Leverage strong typing to minimize type-related errors and undefined behaviors.

### 3. Optimize Queries
- **Query Optimization**: Analyze and optimize SQL queries to reduce execution time. Use indexing judiciously to enhance query performance, especially for columns frequently used in WHERE clauses and JOIN statements.
- **Batch Operations**: Use batch inserts and updates to minimize the number of calls to the database and reduce overhead.

### 4. Effective Use of Transactions
- **ACID Compliance**: Ensure transactions are atomic, consistent, isolated, and durable. Use transactions to group multiple operations into a single unit of work.
- **Avoid Long-Running Transactions**: Keep transactions short to minimize locking and maintain performance.

### 5. Monitor and Analyze Performance
- **Profiling**: Regularly profile your database interactions to identify bottlenecks. Tools like query execution plans can help diagnose slow queries.
- **Logging**: Implement logging for error handling and troubleshooting, but ensure it doesn’t become a performance bottleneck.

### 6. Implement Caching
- **Use In-Memory Caching**: Implement caching strategies (like Redis or Memcached) to store frequently accessed data, reducing the need for repeated database queries.

### 7. Use Frameworks and Libraries
- **ORMs**: Use Object-Relational Mappers (ORMs) when appropriate; they can help with memory management and improve code readability, but be cautious about performance overhead.
- **Database Abstraction**: Using a well-defined interface for database interactions can help isolate changes and improve maintainability.

### 8. Ensure Data Integrity
- **Validation**: Validate inputs rigorously to prevent SQL injection attacks. Utilize prepared statements and parameterized queries.
- **Normalization**: Normalize your database schema to minimize redundancy and maintain data integrity, but balance this with the performance needs of your application.

### 9. Keep Resources Managed
- **Close Connections Properly**: Always ensure that database connections are closed after use. Use constructs that automatically release resources (like "using" statements in C# or try-with-resources in Java).
- **Error Handling**: Implement robust error handling to gracefully manage exceptions and avoid resource leaks.

### Example in Rust
If you're using Rust, you could leverage its ownership model for memory safety:

```rust
use rusqlite::{params, Connection, Result};

fn insert_user(conn: &Connection, name: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO user (name) VALUES (?1)",
        params![name],
    )?;
    Ok(())
}

fn main() -> Result<()> {
    let conn = Connection::open("my_database.db")?;
    insert_user(&conn, "Alice")?;
    Ok(())
}
```

### Conclusion
By following these best practices, you can build database code that is both memory safe and high performance. Continuous monitoring and iterative improvements are key to maintaining optimal performance as system demands evolve.

=== System Statistics ===
Total conversations: 4
Total messages stored: 14
All messages are indexed for vector search ✓
All messages are indexed for text search ✓
All messages are queryable by metadata ✓

=== Step 8: Dynamic Search Configuration ===
Each conversation can have its own search config...

💡 Conversations can store custom search configurations:
  • Search type: hybrid, text, or vector
  • Relevance thresholds
  • Filter by tags or metadata
  • Collection-specific settings
  • Per-conversation AI behavior

This enables context-aware search tuned to each conversation's needs!

=== Cleanup ===
✓ Cleanup complete


=== 📚 Summary: What This Example Showed ===

🔧 ekoDB Native Capabilities Used:
  ✓ Functions with Embed operation (AI integration)
  ✓ Hybrid Search (text + vector combined)
  ✓ Text Search (full-text with stemming)
  ✓ Automatic embedding generation
  ✓ Cross-collection queries

🚀 New Client Helper Methods:
  • client.embed(text, model) - Generate embeddings
  • client.hybrid_search() - Semantic + keyword search
  • client.text_search() - Full-text search
  • client.find_all() - Query all documents

💡 Key Takeaways:
  1. ekoDB handles AI Functions natively - no external services needed
  2. One-line embedding generation with auto-cleanup
  3. Hybrid search combines semantic understanding + keyword matching
  4. Perfect for RAG: store, search, and retrieve context
  5. All AI capabilities accessible through simple client methods

🎯 Build production RAG systems with ekoDB!
   → Set OPENAI_API_KEY in your ekoDB server environment
   → Use these client helpers to make AI integration simple
   → Scale to millions of documents with native indexing


[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning TypeScript RAG Example...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB RAG Conversation System ===

This example shows how ekoDB can power a self-improving AI system
that learns from its own conversation history.

=== Step 1: Building Conversation History ===
Storing previous conversations with embeddings...

  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 34 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.615s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.534s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.677s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.587s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.472s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.380s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.448s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.540s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.549s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.616s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.582s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.514s
    • Function auto-cleaned up by client
✓ Stored performance optimization conversation (4 messages)

=== Step 2: New User Question with Context Retrieval ===
User asks: "How do I write memory-safe high-performance database code?"

=== Step 3: Searching Related Context ===
Using hybrid search to find relevant messages from all conversations...


→ Generating embedding for user question...
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.657s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.081s
✓ Found 5 related messages across all conversations:
  1. [Score: 0.199] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

  2. [Score: 0.180] From conv_database_design
     What is database normalization?

  3. [Score: 0.180] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  4. [Score: 0.154] From conv_database_design
     When should I use NoSQL over SQL?

  5. [Score: 0.112] From conv_performance
     How can I optimize database queries?

=== Step 4: Generating Context-Aware Response ===
✓ AI Response (with context from 3 conversations):

Writing memory-safe, high-performance database code involves adhering to best practices in both memory management and database interaction. Here’s a comprehensive approach:

### 1. Use Safe Programming Practices
- **Language Choice**: Opt for languages that enforce safety (like Rust) or provide memory management features (like Java, C#).
- **Automatic Memory Management**: Use languages or frameworks with garbage collection or automatic memory management to avoid memory leaks.
- **Prevent Buffer Overflows**: Always validate input sizes and use safe functions that handle buffers properly.

### 2. Implement Strong Typing
- **Type Safety**: Use strong typing to catch errors at compile time, reducing runtime errors. Strongly-typed languages (like Rust or Scala) offer better guarantees against many memory issues.
- **Using ORM**: If applicable, consider using an Object-Relational Mapping (ORM) tool that provides type-safe database abstractions, which can help prevent SQL injection vulnerabilities and manage schema changes easily.

### 3. Optimize Database Interaction
- **Use Prepared Statements**: Always use prepared statements to prevent SQL injection and optimize execution plans.
- **Batch Operations**: For inserting or updating multiple records, use batch operations to reduce the number of round trips to the database.
- **Connection Pooling**: Use connection pools to manage database connections efficiently, reducing overhead from repeatedly opening and closing connections.

### 4. Query Optimization
- **Indexing**: Properly index columns that are frequently queried to enhance search performance.
- **Analyze Queries**: Use query analysis tools to understand execution plans and optimize slow queries. Ensure you review and refactor queries that have unnecessary complexity.
- **Limit Data Retrieval**: Always limit the number of records retrieved (e.g., using pagination) to enhance performance.

### 5. Memory Management
- **Avoid Memory Bloat**: Minimize the amount of data loaded into memory at once. Stream results where possible instead of loading entire datasets into memory.
- **Free Resources**: Ensure that database connections, result sets, and cursors are properly closed to free resources.
- **Garbage Collection Tuning**: If using a garbage-collected language, tune the garbage collection settings according to your application's needs.

### 6. Use Concurrency Wisely
- **Connection Handling**: If your application is concurrent, ensure proper handling of database connections in a multi-threaded environment to avoid contention.
- **Transaction Management**: Use appropriate transaction isolation levels based on your needs. Sometimes, less strict isolation levels can improve performance without compromising data integrity.

### 7. Error Handling
- **Graceful Degradation**: Implement error handling strategies that can deal with failures gracefully without crashing.
- **Logging and Monitoring**: Use logging and monitoring tools to keep track of database performance and errors.

### Example: Using Rust with Diesel (ORM)
Here’s a simple pseudo-code example in Rust using Diesel for safe database interactions:

```rust
#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::pg::PgConnection;

fn establish_connection() -> PgConnection {
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn insert_user(conn: &PgConnection, new_user: &NewUser) -> QueryResult<User> {
    use schema::users;

    diesel::insert_into(users::table)
        .values(new_user)
        .get_result(conn)
}

fn main() {
    let connection = establish_connection();

    let new_user = NewUser {
        name: "John Doe".to_string(),
        email: "john.doe@example.com".to_string(),
    };

    match insert_user(&connection, &new_user) {
        Ok(user) => println!("User created: {:?}", user),
        Err(e) => eprintln!("Error inserting user: {:?}", e),
    }
}
```

### Final Thoughts
Writing efficient and memory-safe database code requires careful planning and execution across various facets such as language, ORM usage, query optimization, and memory management. By following best practices and leveraging available tools and libraries, you can significantly enhance your application's performance and safety.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.560s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 4394 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.583s
    • Function auto-cleaned up by client
✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...


→ Executing textSearch()...
  • Collection: rag_messages
  • Query: "ownership system"
  • Limit: 3 results
  • Search method: Full-text with fuzzy matching & stemming
  • No vector embeddings needed - pure keyword search
  ✓ Text search completed in 0.050s
✓ Found 3 messages mentioning ownership:
  1. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  2. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  3. From conv_rust_programming: The borrow checker enforces Rust's ownership rules at compile time. It ensures that references don't outlive the data they point to and prevents data races by allowing either multiple immutable references or one mutable reference.

=== System Statistics ===

→ Querying database statistics...
  • Using findAllWithLimit() helper - simplified query API

📊 Database Statistics:
  • Total conversations: 4
  • Total messages stored: 14
  • All messages indexed for vector search ✓
  • All messages indexed for text search ✓
  • All messages queryable by metadata ✓

=== Step 8: Dynamic Search Configuration ===
Each conversation can have its own search config...

💡 Conversations can store custom search configurations:
  • Search type: hybrid, text, or vector
  • Relevance thresholds
  • Filter by tags or metadata
  • Collection-specific settings
  • Per-conversation AI behavior

This enables context-aware search tuned to each conversation's needs!

=== Cleanup ===
✓ Cleanup complete


=== 📚 Summary: What This Example Showed ===

🔧 ekoDB Native Capabilities Used:
  ✓ Functions with Embed operation (AI integration)
  ✓ Hybrid Search (text + vector combined)
  ✓ Text Search (full-text with stemming)
  ✓ Automatic embedding generation
  ✓ Cross-collection queries

🚀 New Client Helper Methods:
  • client.embed(text, model) - Generate embeddings
  • client.hybridSearch() - Semantic + keyword search
  • client.textSearch() - Full-text search
  • client.findAllWithLimit() - Query all documents

💡 Key Takeaways:
  1. ekoDB handles AI Functions natively - no external services needed
  2. One-line embedding generation with auto-cleanup
  3. Hybrid search combines semantic understanding + keyword matching
  4. Perfect for RAG: store, search, and retrieve context
  5. All AI capabilities accessible through simple client methods

🎯 Build production RAG systems with ekoDB!
   → Set OPENAI_API_KEY in your ekoDB server environment
   → Use these client helpers to make AI integration simple
   → Scale to millions of documents with native indexing


[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Go RAG Example...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB RAG Conversation System ===

This example shows how ekoDB can power a self-improving AI system
that learns from its own conversation history.

=== Step 1: Building Conversation History ===
Storing previous conversations with embeddings...

  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 34 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.499s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.674s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.302s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.440s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.530s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.476s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.395s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.431s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.623s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.356s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.358s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.419s
    • Function auto-cleaned up by client
✓ Stored performance optimization conversation (4 messages)

=== Step 2: New User Question with Context Retrieval ===
User asks: "How do I write memory-safe high-performance database code?"

=== Step 3: Searching Related Context ===
Using hybrid search to find relevant messages from all conversations...


→ Generating embedding for user question...
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.518s
    • Function auto-cleaned up by client

→ Executing HybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.082s

✓ Found 5 related messages across all conversations:
  1. [Score: 0.474] From conv_performance
     How can I optimize database queries?

  2. [Score: 0.466] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  3. [Score: 0.449] From conv_database_design
     When should I use NoSQL over SQL?

  4. [Score: 0.434] From conv_database_design
     What is database normalization?

  5. [Score: 0.405] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

=== Step 4: Generating Context-Aware Response ===
✓ AI Response (with context from 3 conversations):

Writing memory-safe, high-performance database code is critical for ensuring that applications run efficiently while avoiding memory leaks and other issues that can arise due to improper resource management. Here are several strategies to achieve this:

1. **Use Prepared Statements**:
   - Prepared statements help protect against SQL injection attacks and can improve performance by allowing the database to cache execution plans.

2. **Implement Connection Pooling**:
   - Use connection pools to manage database connections efficiently. This reduces the overhead of repeatedly opening and closing connections.

3. **Employ Transaction Management**:
   - Group database operations into transactions to ensure consistency and efficiency. Utilize transaction support in your database when performing multiple related operations.

4. **Batch Processing**:
   - For large datasets, consider batch operations. Instead of inserting or updating records individually, group them into a single query to reduce database round trips.

5. **Efficient Query Design**:
   - Write queries that minimize resource usage. Analyze your SQL queries to ensure they utilize indexes properly, joining only necessary tables and selecting only required fields.

6. **Normalize Your Database**:
   - Follow database normalization principles to reduce redundancy and improve data integrity. This leads to smaller, more efficient tables that require less memory and processing power.

7. **Database Indexing**:
   - Use appropriate indexes to speed up data retrieval operations. However, while indexing improves read performance, it can slow down write operations. Therefore, choose indexes wisely.

8. **Memory Management Techniques**:
   - If you're using languages like C or C++, allocate and deallocate memory carefully. Utilize smart pointers for automatic memory management where applicable.
   - In managed environments (like Java or .NET), ensure that you’re understanding and utilizing garbage collection optimally.

9. **Use of ORM (Object-Relational Mapping) Carefully**:
   - While ORMs can simplify database interactions, they can also introduce inefficiencies. Understand their workings and use them judiciously to avoid performance overhead.

10. **Database Connection Handling**:
    - Always close database connections and clean up resources in a finally block or use try-with-resources (Java) or constructs like “using” in C# to ensure connections are closed.

11. **Profiling and Monitoring**:
    - Continuously profile your database interactions and monitor performance. Tools such as SQL Profiler or database-specific logging can help identify slow queries and help you optimize them.

12. **Asynchronous Operations**:
    - When appropriate, implement asynchronous database calls to improve application responsiveness and throughput.

By combining these strategies, you can create high-performance database code that is also memory-safe, improving the overall reliability and efficiency of your applications.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.463s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 3012 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.538s
    • Function auto-cleaned up by client
✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...


→ Executing TextSearch()...
  • Collection: rag_messages
  • Query: "ownership system"
  • Limit: 3 results
  • Search method: Full-text with fuzzy matching & stemming
  • No vector embeddings needed - pure keyword search
  ✓ Text search completed in 0.047s

✓ Found 3 messages mentioning ownership:
  1. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  2. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  3. From conv_rust_programming: The borrow checker enforces Rust's ownership rules at compile time. It ensures that references don't outlive the data they point to and prevents data races by allowing either multiple immutable references or one mutable reference.

=== System Statistics ===

→ Querying database statistics...
  • Using FindAll() helper - simplified query API

📊 Database Statistics:
  • Total conversations: 4
  • Total messages stored: 14
  • All messages indexed for vector search ✓
  • All messages indexed for text search ✓
  • All messages queryable by metadata ✓

=== Step 8: Dynamic Search Configuration ===
Each conversation can have its own search config...

💡 Conversations can store custom search configurations:
  • Search type: hybrid, text, or vector
  • Relevance thresholds
  • Filter by tags or metadata
  • Collection-specific settings
  • Per-conversation AI behavior

This enables context-aware search tuned to each conversation's needs!

=== Cleanup ===
✓ Cleanup complete


=== 📚 Summary: What This Example Showed ===

🔧 ekoDB Native Capabilities Used:
  ✓ Functions with Embed operation (AI integration)
  ✓ Hybrid Search (text + vector combined)
  ✓ Text Search (full-text with stemming)
  ✓ Automatic embedding generation
  ✓ Cross-collection queries

🚀 New Client Helper Methods:
  • client.Embed(text, model) - Generate embeddings
  • client.HybridSearch() - Semantic + keyword search
  • client.TextSearch() - Full-text search
  • client.FindAll() - Query all documents

💡 Key Takeaways:
  1. ekoDB handles AI Functions natively - no external services needed
  2. One-line embedding generation with auto-cleanup
  3. Hybrid search combines semantic understanding + keyword matching
  4. Perfect for RAG: store, search, and retrieve context
  5. All AI capabilities accessible through simple client methods

🎯 Build production RAG systems with ekoDB!
   → Set OPENAI_API_KEY in your ekoDB server environment
   → Use these client helpers to make AI integration simple
   → Scale to millions of documents with native indexing


[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Kotlin RAG Example...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB RAG Conversation System ===

This example shows how ekoDB can power a self-improving AI system
that learns from its own conversation history.

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== Step 1: Building Conversation History ===
Storing previous conversations with embeddings...

  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 34 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.48s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.548s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.536s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.589s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.743s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.383s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.061s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.61s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.592s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.559s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.38s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.36s
    • Function auto-cleaned up by client
✓ Stored performance optimization conversation (4 messages)

=== Step 2: New User Question with Context Retrieval ===
User asks: "How do I write memory-safe high-performance database code?"

=== Step 3: Searching Related Context ===
Using hybrid search to find relevant messages from all conversations...


→ Generating embedding for user question...
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.549s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.086s

✓ Found 5 related messages across all conversations:
  1. [Score: 0.199] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

  2. [Score: 0.180] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  3. [Score: 0.180] From conv_database_design
     What is database normalization?

  4. [Score: 0.154] From conv_database_design
     When should I use NoSQL over SQL?

  5. [Score: 0.112] From conv_performance
     How can I optimize database queries?

=== Step 4: Generating Context-Aware Response ===
✓ Context prepared from search results
✓ AI would use this context to generate comprehensive response

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.436s
    • Function auto-cleaned up by client
✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...


→ Executing textSearch()...
  • Collection: rag_messages
  • Query: "ownership system"
  • Limit: 3 results
  • Search method: Full-text with fuzzy matching & stemming
  • No vector embeddings needed - pure keyword search
  ✓ Text search completed in 0.045s

✓ Found 3 messages mentioning ownership:
  1. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  2. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  3. From conv_rust_programming: The borrow checker enforces Rust's ownership rules at compile time. It ensures that references don't outlive the data they point to and prevents data races by allowing either multiple immutable references or one mutable reference.

=== System Statistics ===

→ Querying database statistics...
  • Using findAllWithLimit() helper - simplified query API

📊 Database Statistics:
  • Total conversations: 4
  • Total messages stored: 13
  • All messages indexed for vector search ✓
  • All messages indexed for text search ✓
  • All messages queryable by metadata ✓

=== Step 8: Dynamic Search Configuration ===
Each conversation can have its own search config...

💡 Conversations can store custom search configurations:
  • Search type: hybrid, text, or vector
  • Relevance thresholds
  • Filter by tags or metadata
  • Collection-specific settings
  • Per-conversation AI behavior

This enables context-aware search tuned to each conversation's needs!

=== Cleanup ===
✓ Cleanup complete


=== 📚 Summary: What This Example Showed ===

🔧 ekoDB Native Capabilities Used:
  ✓ Functions with Embed operation (AI integration)
  ✓ Hybrid Search (text + vector combined)
  ✓ Text Search (full-text with stemming)
  ✓ Automatic embedding generation
  ✓ Cross-collection queries

🚀 New Client Helper Methods:
  • client.embed(text, model) - Generate embeddings
  • client.hybridSearch() - Semantic + keyword search
  • client.textSearch() - Full-text search
  • client.findAllWithLimit() - Query all documents

💡 Key Takeaways:
  1. ekoDB handles AI Functions natively - no external services needed
  2. One-line embedding generation with auto-cleanup
  3. Hybrid search combines semantic understanding + keyword matching
  4. Perfect for RAG: store, search, and retrieve context
  5. All AI capabilities accessible through simple client methods

🎯 Build production RAG systems with ekoDB!
   → Set OPENAI_API_KEY in your ekoDB server environment
   → Use these client helpers to make AI integration simple
   → Scale to millions of documents with native indexing


[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
✅ [32mRAG Examples Complete![0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m

[32mWhat you just saw across 5 languages:[0m
  ✓ Embeddings generated via ekoDB Functions
  ✓ Hybrid search (semantic + keyword)
  ✓ Text search with stemming
  ✓ Cross-conversation context retrieval
  ✓ Simple client helpers wrapping powerful AI

[36mMission: AI for All 🚀[0m - Making RAG accessible to everyone!

✅ [32mAll RAG examples complete! Output saved to test-examples-rag.md[0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
🌐 [36mTesting SWR (Stale-While-Revalidate) Pattern Examples[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m

📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.16.0 prepare
> npm run build


> @ekodb/ekodb-client@0.16.0 build
> tsc


up to date, audited 52 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.16.0 build
> tsc

✅ [32mTypeScript client built![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning TypeScript SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user (vsbWTebpJThcHxb1iPnoOiB5hJ6Fb9KxXeqDQRCrONNBzWuS8QMZ1afgZLrb_cIfPfyP9EufRMArQW5na8VZdw)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "website": "hildegard.org",
        "address": {
          "suite": "Apt. 556",
          "city": "Gwenborough",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "street": "Kulas Light",
          "zipcode": "92998-3874"
        },
        "name": "Leanne Graham",
        "username": "Bret",
        "email": "Sincere@april.biz",
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "bs": "harness real-time e-markets",
          "name": "Romaguera-Crona"
        },
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
Response time: 40ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "website": "hildegard.org",
        "address": {
          "suite": "Apt. 556",
          "city": "Gwenborough",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "street": "Kulas Light",
          "zipcode": "92998-3874"
        },
        "name": "Leanne Graham",
        "username": "Bret",
        "email": "Sincere@april.biz",
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "bs": "harness real-time e-markets",
          "name": "Romaguera-Crona"
        },
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
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_with_reviews (gVU8ZQG173WemZkUdaJOU7w5Fj65wlOMqQgXk6E4G9aij0t8tx0w2GN0FX8adDc4-RetGortzJXD9uzoTv2kvQ)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "discountPercentage": 10.48,
        "returnPolicy": "No return policy",
        "price": 9.99,
        "reviews": [
          {
            "reviewerName": "Eleanor Collins",
            "comment": "Would not recommend!",
            "rating": 3,
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "date": "2025-04-30T09:41:02.053Z"
          },
          {
            "reviewerEmail": "lucas.gordon@x.dummyjson.com",
            "comment": "Very satisfied!",
            "date": "2025-04-30T09:41:02.053Z",
            "reviewerName": "Lucas Gordon",
            "rating": 4
          },
          {
            "comment": "Highly impressed!",
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "reviewerName": "Eleanor Collins",
            "rating": 5,
            "date": "2025-04-30T09:41:02.053Z"
          }
        ],
        "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
        "dimensions": {
          "depth": 22.99,
          "height": 13.08,
          "width": 15.14
        },
        "brand": "Essence",
        "images": [
          "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
        ],
        "meta": {
          "createdAt": "2025-04-30T09:41:02.053Z",
          "barcode": "5784719087687",
          "updatedAt": "2025-04-30T09:41:02.053Z",
          "qrCode": "https://cdn.dummyjson.com/public/qr-code.png"
        },
        "stock": 99,
        "availabilityStatus": "In Stock",
        "rating": 2.56,
        "weight": 4,
        "minimumOrderQuantity": 48,
        "title": "Essence Mascara Lash Princess",
        "tags": [
          "beauty",
          "mascara"
        ],
        "id": 1,
        "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
        "warrantyInformation": "1 week warranty",
        "sku": "BEA-ESS-ESS-001",
        "shippingInformation": "Ships in 3-5 business days",
        "category": "beauty"
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
✓ Multi-API data fetched, merged, and cached atomically

=== Why This Is Powerful ===

✓ No separate cache layer (Redis, Memcached) needed
✓ No manual cache invalidation (TTL handles it)
✓ No separate edge infrastructure (ekoDB IS the edge)
✓ Atomic operations (function executes as transaction)
✓ With multi-node + ripples: Auto-sync across all nodes
✓ Sub-millisecond cache hits from internal storage
✓ One service instead of many (cache + API gateway + database)

=== Real-World Use Cases ===

1. API Gateway Pattern:
   - Client → ekoDB Function → Check cache → Call microservices → Merge → Cache

2. Database Federation:
   - Query multiple DBs (Postgres, MongoDB) + external APIs
   - Merge results in one function call

3. IoT Data Enrichment:
   - Sensor data + weather API + location API
   - Enrich and cache in one atomic operation

4. E-commerce Product Pages:
   - Product info + reviews + inventory + pricing
   - All from different sources, cached together

✓ Example complete - Your database IS your edge!

=== ekoDB as Edge Cache - Simple Example ===

Creating edge cache function...
✓ Edge cache script created: ZoJG1AigAlukafMy2Flps8KUV7O1N6cuFZnN2pHDTY2upvNPBt8-8UiJnyvChXIYDz29zhBTy8v5NAvGGrYvPA

Call 1: Cache miss (fetches from API)
Response time: 164ms
Result: {
  "records": [
    {
      "value": {
        "phone": "1-770-736-8031 x56442",
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "bs": "harness real-time e-markets",
          "name": "Romaguera-Crona"
        },
        "id": 1,
        "email": "Sincere@april.biz",
        "name": "Leanne Graham",
        "website": "hildegard.org",
        "address": {
          "street": "Kulas Light",
          "suite": "Apt. 556",
          "zipcode": "92998-3874",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "city": "Gwenborough"
        },
        "username": "Bret"
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
Response time: 40ms (4.1x faster!)
Result: {
  "records": [
    {
      "value": {
        "phone": "1-770-736-8031 x56442",
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "bs": "harness real-time e-markets",
          "name": "Romaguera-Crona"
        },
        "id": 1,
        "email": "Sincere@april.biz",
        "name": "Leanne Graham",
        "website": "hildegard.org",
        "address": {
          "street": "Kulas Light",
          "suite": "Apt. 556",
          "zipcode": "92998-3874",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "city": "Gwenborough"
        },
        "username": "Bret"
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

✅ [32mTypeScript SWR examples complete![0m
🐍 [36mBuilding Python client package...[0m
🍹 Building a mixed python/rust project
🔗 Found pyo3 bindings with abi3 support
🐍 Found CPython 3.11 at /Library/Frameworks/Python.framework/Versions/3.11/bin/python3
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.14s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.16.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.16.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
Successfully installed ekodb-client-0.16.0
✅ [32mPython client package built and installed![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Python SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (0GSunQ8GEoitC6ueUdr2MgWMMIvuRthaZokBVD1MY4b6Bd4X7p4j_e1ptH9uAI6zm2_Ms9Ytt1bqa8jd2smSEg)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 195ms
Result: [
  {
    "cached_at": "1776012756",
    "data": {
      "type": "Object",
      "value": {
        "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
        "bio": null,
        "blog": "",
        "company": "Linux Foundation",
        "created_at": "2011-09-03T15:26:22Z",
        "email": null,
        "events_url": "https://api.github.com/users/torvalds/events{/privacy}",
        "followers": 296118,
        "followers_url": "https://api.github.com/users/torvalds/followers",
        "following": 0,
        "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
        "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
        "gravatar_id": "",
        "hireable": null,
        "html_url": "https://github.com/torvalds",
        "id": 1024025,
        "location": "Portland, OR",
        "login": "torvalds",
        "name": "Linus Torvalds",
        "node_id": "MDQ6VXNlcjEwMjQwMjU=",
        "organizations_url": "https://api.github.com/users/torvalds/orgs",
        "public_gists": 1,
        "public_repos": 11,
        "received_events_url": "https://api.github.com/users/torvalds/received_events",
        "repos_url": "https://api.github.com/users/torvalds/repos",
        "site_admin": false,
        "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
        "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
        "twitter_username": null,
        "type": "User",
        "updated_at": "2026-01-13T07:10:05Z",
        "url": "https://api.github.com/users/torvalds",
        "user_view_type": "public"
      }
    },
    "id": "torvalds"
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 47ms (4.1x faster!)
Result: [
  {
    "cached_at": "1776012756",
    "data": {
      "type": "Object",
      "value": {
        "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
        "bio": null,
        "blog": "",
        "company": "Linux Foundation",
        "created_at": "2011-09-03T15:26:22Z",
        "email": null,
        "events_url": "https://api.github.com/users/torvalds/events{/privacy}",
        "followers": 296118,
        "followers_url": "https://api.github.com/users/torvalds/followers",
        "following": 0,
        "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
        "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
        "gravatar_id": "",
        "hireable": null,
        "html_url": "https://github.com/torvalds",
        "id": 1024025,
        "location": "Portland, OR",
        "login": "torvalds",
        "name": "Linus Torvalds",
        "node_id": "MDQ6VXNlcjEwMjQwMjU=",
        "organizations_url": "https://api.github.com/users/torvalds/orgs",
        "public_gists": 1,
        "public_repos": 11,
        "received_events_url": "https://api.github.com/users/torvalds/received_events",
        "repos_url": "https://api.github.com/users/torvalds/repos",
        "site_admin": false,
        "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
        "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
        "twitter_username": null,
        "type": "User",
        "updated_at": "2026-01-13T07:10:05Z",
        "url": "https://api.github.com/users/torvalds",
        "user_view_type": "public"
      }
    }
  }
]
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (HiapeQYJmN0X0OF71ErOTA521KStHXayb6KfimDgsvv-YOFRQGuqhojeTaHw5lYT2GucgLYpqPI7QTeDjb33sg)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "enriched_at": "1776012756",
    "enriched_data": {
      "type": "Object",
      "value": {
        "availabilityStatus": "In Stock",
        "brand": "Essence",
        "category": "beauty",
        "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
        "dimensions": {
          "depth": 22.99,
          "height": 13.08,
          "width": 15.14
        },
        "discountPercentage": 10.48,
        "id": 1,
        "images": [
          "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
        ],
        "meta": {
          "barcode": "5784719087687",
          "createdAt": "2025-04-30T09:41:02.053Z",
          "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
          "updatedAt": "2025-04-30T09:41:02.053Z"
        },
        "minimumOrderQuantity": 48,
        "price": 9.99,
        "rating": 2.56,
        "returnPolicy": "No return policy",
        "reviews": [
          {
            "comment": "Would not recommend!",
            "date": "2025-04-30T09:41:02.053Z",
            "rating": 3,
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "reviewerName": "Eleanor Collins"
          },
          {
            "comment": "Very satisfied!",
            "date": "2025-04-30T09:41:02.053Z",
            "rating": 4,
            "reviewerEmail": "lucas.gordon@x.dummyjson.com",
            "reviewerName": "Lucas Gordon"
          },
          {
            "comment": "Highly impressed!",
            "date": "2025-04-30T09:41:02.053Z",
            "rating": 5,
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "reviewerName": "Eleanor Collins"
          }
        ],
        "shippingInformation": "Ships in 3-5 business days",
        "sku": "BEA-ESS-ESS-001",
        "stock": 99,
        "tags": [
          "beauty",
          "mascara"
        ],
        "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
        "title": "Essence Mascara Lash Princess",
        "warrantyInformation": "1 week warranty",
        "weight": 4
      }
    },
    "id": "1"
  }
]
✓ Data fetched, enriched, and cached atomically

=== Why This Is Powerful ===
✓ No separate cache layer (Redis, Memcached) needed
✓ No manual cache invalidation (TTL handles it)
✓ No separate edge infrastructure (ekoDB IS the edge)
✓ Atomic operations (function executes as transaction)
✓ With multi-node + ripples: Auto-sync across all nodes
✓ Sub-millisecond cache hits from internal storage
✓ One service instead of many (cache + API gateway + database)

=== Real-World Use Cases ===
1. API Gateway Pattern:
   - Client → ekoDB Function → Check cache → Call microservices → Merge → Cache

2. Database Federation:
   - Query multiple DBs (Postgres, MongoDB) + external APIs
   - Merge results in one function call

3. IoT Data Enrichment:
   - Sensor data + weather API + location API
   - Enrich and cache in one atomic operation

4. E-commerce Product Pages:
   - Product info + reviews + inventory + pricing
   - All from different sources, cached together

✓ Example complete - Your database IS your edge!

✅ [32mPython SWR examples complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Go SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (9VFBhtXFbn3mOuSMQPwGKtSE_suNcjVQDdr2yLj7p0zxGdgcLa8YiJImnv_vMv23I44dbIUSRykkXc_pGB3Rtw)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 39.195667ms
Result: [
  {
    "cached_at": "1776012756",
    "data": {
      "type": "Object",
      "value": {
        "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
        "bio": null,
        "blog": "",
        "company": "Linux Foundation",
        "created_at": "2011-09-03T15:26:22Z",
        "email": null,
        "events_url": "https://api.github.com/users/torvalds/events{/privacy}",
        "followers": 296118,
        "followers_url": "https://api.github.com/users/torvalds/followers",
        "following": 0,
        "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
        "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
        "gravatar_id": "",
        "hireable": null,
        "html_url": "https://github.com/torvalds",
        "id": 1024025,
        "location": "Portland, OR",
        "login": "torvalds",
        "name": "Linus Torvalds",
        "node_id": "MDQ6VXNlcjEwMjQwMjU=",
        "organizations_url": "https://api.github.com/users/torvalds/orgs",
        "public_gists": 1,
        "public_repos": 11,
        "received_events_url": "https://api.github.com/users/torvalds/received_events",
        "repos_url": "https://api.github.com/users/torvalds/repos",
        "site_admin": false,
        "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
        "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
        "twitter_username": null,
        "type": "User",
        "updated_at": "2026-01-13T07:10:05Z",
        "url": "https://api.github.com/users/torvalds",
        "user_view_type": "public"
      }
    }
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 39.402375ms (1.0x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (i9R8L_2T8C8gb0mHUzZKU3Nip94JYE0frxE3uyyXAWiXADf44YCL7TpvudGUZdYZY135inSQa-t-nwP9cA3JVw)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "enriched_data": {
      "type": "Object",
      "value": {
        "availabilityStatus": "In Stock",
        "brand": "Essence",
        "category": "beauty",
        "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
        "dimensions": {
          "depth": 22.99,
          "height": 13.08,
          "width": 15.14
        },
        "discountPercentage": 10.48,
        "id": 1,
        "images": [
          "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
        ],
        "meta": {
          "barcode": "5784719087687",
          "createdAt": "2025-04-30T09:41:02.053Z",
          "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
          "updatedAt": "2025-04-30T09:41:02.053Z"
        },
        "minimumOrderQuantity": 48,
        "price": 9.99,
        "rating": 2.56,
        "returnPolicy": "No return policy",
        "reviews": [
          {
            "comment": "Would not recommend!",
            "date": "2025-04-30T09:41:02.053Z",
            "rating": 3,
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "reviewerName": "Eleanor Collins"
          },
          {
            "comment": "Very satisfied!",
            "date": "2025-04-30T09:41:02.053Z",
            "rating": 4,
            "reviewerEmail": "lucas.gordon@x.dummyjson.com",
            "reviewerName": "Lucas Gordon"
          },
          {
            "comment": "Highly impressed!",
            "date": "2025-04-30T09:41:02.053Z",
            "rating": 5,
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "reviewerName": "Eleanor Collins"
          }
        ],
        "shippingInformation": "Ships in 3-5 business days",
        "sku": "BEA-ESS-ESS-001",
        "stock": 99,
        "tags": [
          "beauty",
          "mascara"
        ],
        "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
        "title": "Essence Mascara Lash Princess",
        "warrantyInformation": "1 week warranty",
        "weight": 4
      }
    }
  }
]
✓ Data fetched, enriched, and cached atomically

=== Why This Is Powerful ===
✓ No separate cache layer (Redis, Memcached) needed
✓ No manual cache invalidation (TTL handles it)
✓ No separate edge infrastructure (ekoDB IS the edge)
✓ Atomic operations (function executes as transaction)
✓ With multi-node + ripples: Auto-sync across all nodes
✓ Sub-millisecond cache hits from internal storage
✓ One service instead of many (cache + API gateway + database)

=== Real-World Use Cases ===
1. API Gateway Pattern:
   - Client → ekoDB Function → Check cache → Call microservices → Merge → Cache

2. Database Federation:
   - Query multiple DBs (Postgres, MongoDB) + external APIs
   - Merge results in one function call

3. IoT Data Enrichment:
   - Sensor data + weather API + location API
   - Enrich and cache in one atomic operation

4. E-commerce Product Pages:
   - Product info + reviews + inventory + pricing
   - All from different sources, cached together

✓ Example complete - Your database IS your edge!

✅ [32mGo SWR examples complete![0m
🛠️  [36mBuilding client library...[0m
cargo build -p ekodb_client
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
✅ [32mClient build complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Rust SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/examples/swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (rdjb5Rv4i7q7pKiIBPNgDVsf0msaTBTyV6jI6bCdbJdL4a75Tl9gDYvQaJwctWpIGvIIWIm8rvRpiLV-XZ6B7g)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 41ms
Result: {
  "data": {
    "value": {
      "updated_at": "2026-01-13T07:10:05Z",
      "id": 1024025,
      "url": "https://api.github.com/users/torvalds",
      "created_at": "2011-09-03T15:26:22Z",
      "gravatar_id": "",
      "received_events_url": "https://api.github.com/users/torvalds/received_events",
      "node_id": "MDQ6VXNlcjEwMjQwMjU=",
      "name": "Linus Torvalds",
      "login": "torvalds",
      "hireable": null,
      "public_gists": 1,
      "public_repos": 11,
      "followers": 296118,
      "following": 0,
      "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
      "company": "Linux Foundation",
      "blog": "",
      "followers_url": "https://api.github.com/users/torvalds/followers",
      "email": null,
      "site_admin": false,
      "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
      "organizations_url": "https://api.github.com/users/torvalds/orgs",
      "bio": null,
      "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
      "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
      "events_url": "https://api.github.com/users/torvalds/events{/privacy}",
      "repos_url": "https://api.github.com/users/torvalds/repos",
      "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
      "location": "Portland, OR",
      "twitter_username": null,
      "html_url": "https://github.com/torvalds",
      "type": "User",
      "user_view_type": "public"
    },
    "type": "Object"
  },
  "cached_at": "1776012756"
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 52ms (0.8x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (NiHD5jrArAdoj8FjOuEK9-qBlOOv1hz5eNAaCVcdz6GMwvJBCmjLKTKegPNwEGjMyD2s4-1a46IbUqvfzOqoSA)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: {
  "enriched_data": {
    "type": "Object",
    "value": {
      "images": [
        "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
      ],
      "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
      "weight": 4,
      "reviews": [
        {
          "reviewerEmail": "eleanor.collins@x.dummyjson.com",
          "date": "2025-04-30T09:41:02.053Z",
          "rating": 3,
          "reviewerName": "Eleanor Collins",
          "comment": "Would not recommend!"
        },
        {
          "reviewerName": "Lucas Gordon",
          "date": "2025-04-30T09:41:02.053Z",
          "rating": 4,
          "reviewerEmail": "lucas.gordon@x.dummyjson.com",
          "comment": "Very satisfied!"
        },
        {
          "reviewerEmail": "eleanor.collins@x.dummyjson.com",
          "reviewerName": "Eleanor Collins",
          "date": "2025-04-30T09:41:02.053Z",
          "rating": 5,
          "comment": "Highly impressed!"
        }
      ],
      "warrantyInformation": "1 week warranty",
      "availabilityStatus": "In Stock",
      "price": 9.99,
      "sku": "BEA-ESS-ESS-001",
      "returnPolicy": "No return policy",
      "shippingInformation": "Ships in 3-5 business days",
      "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
      "tags": [
        "beauty",
        "mascara"
      ],
      "stock": 99,
      "rating": 2.56,
      "discountPercentage": 10.48,
      "title": "Essence Mascara Lash Princess",
      "minimumOrderQuantity": 48,
      "dimensions": {
        "width": 15.14,
        "depth": 22.99,
        "height": 13.08
      },
      "category": "beauty",
      "id": 1,
      "meta": {
        "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
        "createdAt": "2025-04-30T09:41:02.053Z",
        "barcode": "5784719087687",
        "updatedAt": "2025-04-30T09:41:02.053Z"
      },
      "brand": "Essence"
    }
  }
}
✓ Data fetched, enriched, and cached atomically

=== Why This Is Powerful ===
✓ No separate cache layer (Redis, Memcached) needed
✓ No manual cache invalidation (TTL handles it)
✓ No separate edge infrastructure (ekoDB IS the edge)
✓ Atomic operations (function executes as transaction)
✓ With multi-node + ripples: Auto-sync across all nodes
✓ Sub-millisecond cache hits from internal storage
✓ One service instead of many (cache + API gateway + database)

=== Real-World Use Cases ===
1. API Gateway Pattern:
   - Client → ekoDB Function → Check cache → Call microservices → Merge → Cache

2. Database Federation:
   - Query multiple DBs (Postgres, MongoDB) + external APIs
   - Merge results in one function call

3. IoT Data Enrichment:
   - Sensor data + weather API + location API
   - Enrich and cache in one atomic operation

4. E-commerce Product Pages:
   - Product info + reviews + inventory + pricing
   - All from different sources, cached together

✓ Example complete - Your database IS your edge!

✅ [32mRust SWR examples complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Kotlin SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR function: fetch_github_user (3bv9bxdTNgUi96HarXJNrzzPA3vb6ilAIzj_BTtsB2-rx8EizZzmkWjOj3D5nPkhhqSiZlm08d1ohbwHxfAvqA)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 52ms
Result: [{"cached_at":"1776012756","data":{"value":{"login":"torvalds","site_admin":false,"events_url":"https://api.github.com/users/torvalds/events{/privacy}","subscriptions_url":"https://api.github.com/users/torvalds/subscriptions","created_at":"2011-09-03T15:26:22Z","following_url":"https://api.github.com/users/torvalds/following{/other_user}","gists_url":"https://api.github.com/users/torvalds/gists{/gist_id}","email":null,"bio":null,"name":"Linus Torvalds","html_url":"https://github.com/torvalds","received_events_url":"https://api.github.com/users/torvalds/received_events","hireable":null,"node_id":"MDQ6VXNlcjEwMjQwMjU=","gravatar_id":"","public_gists":1,"public_repos":11,"blog":"","organizations_url":"https://api.github.com/users/torvalds/orgs","following":0,"updated_at":"2026-01-13T07:10:05Z","id":1024025,"url":"https://api.github.com/users/torvalds","followers_url":"https://api.github.com/users/torvalds/followers","repos_url":"https://api.github.com/users/torvalds/repos","starred_url":"https://api.github.com/users/torvalds/starred{/owner}{/repo}","avatar_url":"https://avatars.githubusercontent.com/u/1024025?v=4","followers":296118,"location":"Portland, OR","twitter_username":null,"type":"User","user_view_type":"public","company":"Linux Foundation"},"type":"Object"}}]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 45ms
Cache hit was 1.2x faster!

✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment function: fetch_product_enriched (8OzwaWGr67ZmWoUvJJLl-Jl25iCrAImVYddGiEwdi48ZT_PG6dzU2kFJgP4iuQuTEuhIsMO3hkD7--perT2-8g)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [{"enriched_data":{"type":"Object","value":{"id":1,"reviews":[{"rating":3,"comment":"Would not recommend!","reviewerName":"Eleanor Collins","reviewerEmail":"eleanor.collins@x.dummyjson.com","date":"2025-04-30T09:41:02.053Z"},{"date":"2025-04-30T09:41:02.053Z","rating":4,"comment":"Very satisfied!","reviewerEmail":"lucas.gordon@x.dummyjson.com","reviewerName":"Lucas Gordon"},{"reviewerName":"Eleanor Collins","comment":"Highly impressed!","date":"2025-04-30T09:41:02.053Z","rating":5,"reviewerEmail":"eleanor.collins@x.dummyjson.com"}],"dimensions":{"width":15.14,"height":13.08,"depth":22.99},"description":"The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.","tags":["beauty","mascara"],"availabilityStatus":"In Stock","category":"beauty","title":"Essence Mascara Lash Princess","price":9.99,"images":["https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"],"minimumOrderQuantity":48,"brand":"Essence","returnPolicy":"No return policy","sku":"BEA-ESS-ESS-001","shippingInformation":"Ships in 3-5 business days","thumbnail":"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp","rating":2.56,"weight":4,"meta":{"updatedAt":"2025-04-30T09:41:02.053Z","qrCode":"https://cdn.dummyjson.com/public/qr-code.png","barcode":"5784719087687","createdAt":"2025-04-30T09:41:02.053Z"},"warrantyInformation":"1 week warranty","discountPercentage":10.48,"stock":99}}}]
✓ Data fetched, enriched, and cached atomically

=== Why This Is Powerful ===
✓ No separate cache layer (Redis, Memcached) needed
✓ No manual cache invalidation (TTL handles it)
✓ No separate edge infrastructure (ekoDB IS the edge)
✓ Atomic operations (function executes as transaction)
✓ With multi-node + ripples: Auto-sync across all nodes
✓ Sub-millisecond cache hits from internal storage
✓ One service instead of many (cache + API gateway + database)

=== Real-World Use Cases ===
1. API Gateway Pattern:
   - Client → ekoDB Function → Check cache → Call microservices → Merge → Cache

2. Database Federation:
   - Query multiple DBs (Postgres, MongoDB) + external APIs
   - Merge results in one function call

3. IoT Data Enrichment:
   - Sensor data + weather API + location API
   - Enrich and cache in one atomic operation

4. E-commerce Product Pages:
   - Product info + reviews + inventory + pricing
   - All from different sources, cached together

✓ Example complete - Your database IS your edge!


BUILD SUCCESSFUL in 4s
2 actionable tasks: 1 executed, 1 up-to-date
✅ [32mKotlin SWR examples complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
✅ [32mAll SWR Examples Complete![0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m

[32mWhat you just saw - ekoDB as Edge Cache:[0m
  ✓ FindById → Check cache
  ✓ If/Else → Conditional execution
  ✓ HttpRequest → External API calls
  ✓ Insert with TTL → Auto-expiring cache
  ✓ Sub-millisecond cache hits
  ✓ No Redis, no CDN, no cache invalidation logic needed

[36mYour DATABASE is your EDGE! 🚀[0m


[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
🔗 [36mTesting Function Composition Examples[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m

🛠️  [36mBuilding client library...[0m
cargo build -p ekodb_client
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
✅ [32mClient build complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Rust Function Composition Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_function_composition`
=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

✅ Saved reusable function: fetch_user
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: User 1
   Department: engineering

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 161.457625ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "phone": "1-770-736-8031 x56442",
    "id": 1,
    "name": "Leanne Graham",
    "email": "Sincere@april.biz",
    "address": {
      "zipcode": "92998-3874",
      "geo": {
        ...

Second call (cache hit - from cache):
   ⏱️  Duration: 40.585083ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "website": "hildegard.org",
    "id": 1,
    "name": "Leanne Graham",
    "username": "Bret",
    "address": {
      "street": "Kulas Light",
      "zipcode": "92998-3874",
      "s...
   🚀 Cache speedup: 4.0x faster!

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
   - fetch_enriched_user: Used in 50 workflows
   - get_engineering_user: Specific workflow


✅ All composition examples completed!
✅ [32mRust function composition examples complete![0m
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.16.0 prepare
> npm run build


> @ekodb/ekodb-client@0.16.0 build
> tsc


up to date, audited 52 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.16.0 build
> tsc

✅ [32mTypeScript client built![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning TypeScript Function Composition Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

✅ Saved reusable function: fetch_user
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"value":"User 1","type":"String"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 41ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "company": {
      "name": "Romaguera-Crona",
      "bs": "harness real-time e-markets",
      "catchPhrase": "Multi-layered client-server neural-net"
    },
    "username": "Bret",...

Second call (cache hit - from cache):
   ⏱️  Duration: 41ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "company": {
      "name": "Romaguera-Crona",
      "bs": "harness real-time e-markets",
      "catchPhrase": "Multi-layered client-server neural-net"
    },
    "username": "Bret",...
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
✅ [32mTypeScript function composition examples complete![0m
🐍 [36mBuilding Python client package...[0m
🍹 Building a mixed python/rust project
🔗 Found pyo3 bindings with abi3 support
🐍 Found CPython 3.11 at /Library/Frameworks/Python.framework/Versions/3.11/bin/python3
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.15s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.16.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.16.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.16.0
    Uninstalling ekodb_client-0.16.0:
      Successfully uninstalled ekodb_client-0.16.0
Successfully installed ekodb-client-0.16.0
✅ [32mPython client package built and installed![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Python Function Composition Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

✅ Saved reusable function: fetch_user
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"type": "String", "value": "User 1"}
   Department: {"type": "String", "value": "engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 42.3ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "address": {
                  "city": "Gwenborough",
                  "geo": {
                        "lat": "-37.3159",
                        "lng": "81.1496"
    ...

Second call (cache hit - from cache):
   ⏱️  Duration: 41.3ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "address": {
                  "city": "Gwenborough",
                  "geo": {
                        "lat": "-37.3159",
                        "lng": "81.1496"
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
✅ [32mPython function composition examples complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Go Function Composition Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
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
   ⏱️  Duration: 40.740083ms
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
   ⏱️  Duration: 41.584667ms
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
✅ [32mGo function composition examples complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning JavaScript Function Composition Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

✅ Saved reusable function: fetch_user
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"value":"User 1","type":"String"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 44ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "company": {
      "name": "Romaguera-Crona",
      "bs": "harness real-time e-markets",
      "catchPhrase": "Multi-layered client-server neural-net"
    },
    "username": "Bret",...

Second call (cache hit - from cache):
   ⏱️  Duration: 41ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "company": {
      "name": "Romaguera-Crona",
      "bs": "harness real-time e-markets",
      "catchPhrase": "Multi-layered client-server neural-net"
    },
    "username": "Bret",...
   🚀 Cache speedup: 1.1x faster!

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
✅ [32mJavaScript function composition examples complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
✅ [32mAll Function Composition Examples Complete![0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m

[32mWhat you just saw - CallFunction composability:[0m
  ✓ Reusable Function building blocks
  ✓ Functions calling other Functions
  ✓ Clean SWR patterns via composition
  ✓ Multi-level nesting (arbitrary depth)
  ✓ No code duplication
  ✓ Single source of truth

[36mBuild complex workflows from simple pieces! 🚀[0m


[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
📡 [36mWebSocket Subscription Tests[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m

[33mThese tests verify real-time WebSocket subscriptions by:[0m
  1. Authenticating and connecting via WebSocket
  2. Subscribing to a collection
  3. Inserting records via REST to trigger notifications
  4. Verifying MutationNotification push messages arrive
  5. Unsubscribing and cleaning up

🛠️  [36mBuilding client library...[0m
cargo build -p ekodb_client
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
✅ [32mClient build complete![0m

🦀 [33mRust WebSocket Subscription Test...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_websocket_subscribe`
✓ Authentication successful

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_rs' ===

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: "fLH3gxwxWUehgpVPEh5gssaLMAWmRiq_QFMzLnLRSZFQjszRjoh78qardbfTJcBmUNMQgc09xuFhbFOBrzRAjg"

  📡 Notification received:
     Event:      "insert"
     Collection: "ws_subscribe_example_rs"
     Record IDs: ["fLH3gxwxWUehgpVPEh5gssaLMAWmRiq_QFMzLnLRSZFQjszRjoh78qardbfTJcBmUNMQgc09xuFhbFOBrzRAjg"]
     Timestamp:  "2026-04-12T16:53:01.409965+00:00"

Inserting record 2...
✓ Inserted: "nmmiw2sHNnK6xxumJ_LHX4KXMg6h5dhSYUdr0OV-5R9wRl62hzFju75Ke-Q80T38swhh3asOSPr_rp-c7XJm6A"

  📡 Notification received:
     Event:      "insert"
     Record IDs: ["nmmiw2sHNnK6xxumJ_LHX4KXMg6h5dhSYUdr0OV-5R9wRl62hzFju75Ke-Q80T38swhh3asOSPr_rp-c7XJm6A"]

=== Unsubscribing ===
✓ Unsubscribed

✓ WebSocket subscription example completed successfully
✅ [32mRust subscription test complete![0m

🔷 [33mGo WebSocket Subscription Test...[0m
=== WebSocket Subscription Example (Go) ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_go' ===
✓ Subscribed (subscription_id: sub_da146e934db944e7bdadd1c5d97fedc3)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: nRMaE4yAGu5H_m049HT5_YjVsW4TyurjHluYWJkVLaI1V9cNpi2fnxTTPfCpbkZCqKe3CFjBI-cIeryZ9mpTvQ

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_go
     Record IDs: [nRMaE4yAGu5H_m049HT5_YjVsW4TyurjHluYWJkVLaI1V9cNpi2fnxTTPfCpbkZCqKe3CFjBI-cIeryZ9mpTvQ]
     Timestamp:  2026-04-12T16:53:01.543041+00:00

Inserting record 2...
✓ Inserted: JjqAA1YJv3d4YaNSHFRtMzwMVU5LYAnPpFF7fV7hrdM6_eFNbr8cGdSOT8BnShP1IZ01Cej3EeoyystpyeHFCw

  📡 Notification received:
     Event:      insert
     Record IDs: [JjqAA1YJv3d4YaNSHFRtMzwMVU5LYAnPpFF7fV7hrdM6_eFNbr8cGdSOT8BnShP1IZ01Cej3EeoyystpyeHFCw]

=== Unsubscribing ===
✓ Unsubscribed

✓ WebSocket subscription example completed successfully
✅ [32mGo subscription test complete![0m

🐍 [33mPython WebSocket Subscription Test...[0m
=== WebSocket Subscription Example (Python) ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_py' ===
✓ Subscribed (subscription_id: sub_549b4a38184043bb989b07daca08ad0a)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: 1iUXB796QXY70VhBrqzaUjq8wv3dvBpGcR--ImD5vaFc6Mv9zK2UL3p0H_E7aP7VnqWqxFUIstOqOg8ZkCRXkg

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_py
     Record IDs: 1iUXB796QXY70VhBrqzaUjq8wv3dvBpGcR--ImD5vaFc6Mv9zK2UL3p0H_E7aP7VnqWqxFUIstOqOg8ZkCRXkg
     Timestamp:  2026-04-12T16:53:01.693771+00:00

Inserting record 2...
✓ Inserted: PkHzAgbOREL3Hp4qXc2j7Q885H08spu8OeIUfBMrZr8J4zrQQZ5PPPbCoDhtYDXyrDXNbMS-Ob2PPY1OUemvEw

  📡 Notification received:
     Event:      insert
     Record IDs: PkHzAgbOREL3Hp4qXc2j7Q885H08spu8OeIUfBMrZr8J4zrQQZ5PPPbCoDhtYDXyrDXNbMS-Ob2PPY1OUemvEw

=== Unsubscribing ===
✓ Unsubscribed: {'collection': 'ws_subscribe_example_py', 'found': True, 'unsubscribed': True}

✓ WebSocket subscription example completed successfully
✅ [32mPython subscription test complete![0m
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.16.0 prepare
> npm run build


> @ekodb/ekodb-client@0.16.0 build
> tsc


up to date, audited 52 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.16.0 build
> tsc

✅ [32mTypeScript client built![0m

📘 [33mTypeScript WebSocket Subscription Test...[0m
=== WebSocket Subscription Example ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_ts' ===
✓ Subscribed (subscription_id: sub_e848be3584594a42bbe306864c1082ec)

=== Listening for mutation notifications ===

Inserting a record...

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_ts
     Record IDs: vzOb5uLDBkaGRr39Eo6hSPCpKA4HHDfEUSDu3IjXBw6ym2dVI6vO5yex5NV5br8E9LcJCNr7jq2kR2ZyzUKq1g
     Timestamp:  2026-04-12T16:53:04.403297+00:00
     Records:    {"active":true,"id":"vzOb5uLDBkaGRr39Eo6hSPCpKA4HHDfEUSDu3IjXBw6ym2dVI6vO5yex5NV5br8E9LcJCNr7jq2kR2Z...
✓ Inserted record: vzOb5uLDBkaGRr39Eo6hSPCpKA4HHDfEUSDu3IjXBw6ym2dVI6vO5yex5NV5br8E9LcJCNr7jq2kR2ZyzUKq1g

Inserting another record...

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_ts
     Record IDs: 0uxpUt6rK8bqpSj2gZhfPWue7osXT5VeA-pZSNCpV4_jpWhYJKrJxJaEA9NggTbGrRVgiXRtHuaTS5G4JQmthw
     Timestamp:  2026-04-12T16:53:05.427166+00:00
     Records:    {"active":true,"id":"0uxpUt6rK8bqpSj2gZhfPWue7osXT5VeA-pZSNCpV4_jpWhYJKrJxJaEA9NggTbGrRVgiXRtHuaTS5G...

=== Unsubscribing ===
✓ Unsubscribed: {"collection":"ws_subscribe_example_ts","found":true,"unsubscribed":true}

✓ WebSocket subscription example completed successfully
✅ [32mTypeScript subscription test complete![0m

🟣 [33mKotlin WebSocket Subscription Test...[0m
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
=== WebSocket Subscription Example (Kotlin) ===

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✓ Authentication successful

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_kt' ===
✓ Subscribed (subscription_id: sub_118ea8836414447ba4234e92a31bda0a)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: "hlYyYUcGhgXwUtfXecXZriUOuke8pCeDCuW3i00VdhjYkW-dJZQYhHxDqW8Uta5O4-fs5WOihyphiK2b0PGUZA"

  📡 Notification received:
     Event:      "insert"
     Collection: "ws_subscribe_example_kt"
     Record IDs: ["hlYyYUcGhgXwUtfXecXZriUOuke8pCeDCuW3i00VdhjYkW-dJZQYhHxDqW8Uta5O4-fs5WOihyphiK2b0PGUZA"]
     Timestamp:  "2026-04-12T16:53:10.460633+00:00"

Inserting record 2...
✓ Inserted: "nlEMldaUtzjw2QNToG573jkr8kc1draxptt3yhlClBgkuh2nQ2CHHvXWRhv9eLkZnfdqBu-hX6k-0u7uA88C4A"

  📡 Notification received:
     Event:      "insert"
     Record IDs: ["nlEMldaUtzjw2QNToG573jkr8kc1draxptt3yhlClBgkuh2nQ2CHHvXWRhv9eLkZnfdqBu-hX6k-0u7uA88C4A"]

=== Unsubscribing ===
✓ Unsubscribed

✓ WebSocket subscription example completed successfully

BUILD SUCCESSFUL in 4s
2 actionable tasks: 1 executed, 1 up-to-date
✅ [32mKotlin subscription test complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
✅ [32mAll WebSocket Subscription Tests Passed![0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
