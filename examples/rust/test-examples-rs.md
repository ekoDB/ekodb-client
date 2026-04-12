make test-examples-rust
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
