make test-examples
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
    Updating crates.io index
     Locking 2 packages to latest compatible versions
    Updating async-compression v0.4.44 -> v0.4.45
    Updating compression-codecs v0.4.39 -> v0.4.40
   Compiling compression-codecs v0.4.40
   Compiling async-compression v0.4.45
   Compiling tower-http v0.6.11
   Compiling reqwest v0.12.28
   Compiling ekodb_client v0.26.1 (ekoDB/ekodb-client/ekodb_client)
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 8.54s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("Xfj_joxKsdV_-Zm4nm28W7R_V_RNgp4uN9AYqxI1MrulObOLCo2ShXOmo5-KRQXKLRNuFUZeLA_ehtjUtdb6Lg")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("Xfj_joxKsdV_-Zm4nm28W7R_V_RNgp4uN9AYqxI1MrulObOLCo2ShXOmo5-KRQXKLRNuFUZeLA_ehtjUtdb6Lg"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("Xfj_joxKsdV_-Zm4nm28W7R_V_RNgp4uN9AYqxI1MrulObOLCo2ShXOmo5-KRQXKLRNuFUZeLA_ehtjUtdb6Lg"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("Xfj_joxKsdV_-Zm4nm28W7R_V_RNgp4uN9AYqxI1MrulObOLCo2ShXOmo5-KRQXKLRNuFUZeLA_ehtjUtdb6Lg"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.01s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "WsbRCK94m4hP3hqb2DP9BPADED5Qb3sY8pyagkVQFOgdbEuIFn84mi1fBQkRrv_aJlCNvJp3vQhqcdHXQRFFCQ"

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1788814926396347000",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "WsbRCK94m4hP3hqb2DP9BPADED5Qb3sY8pyagkVQFOgdbEuIFn84mi1fBQkRrv_aJlCNvJp3vQhqcdHXQRFFCQ",
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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.85s
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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.86s
     Running `target/debug/examples/kv_operations`
✓ Authentication successful

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: Object {"type": String("Object"), "value": Object {"userId": Number(123), "username": String("john_doe")}}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: Object {"type": String("Object"), "value": Object {"name": String("Product 1"), "price": Number(29.99)}}
cache:product:2: Object {"type": String("Object"), "value": Object {"name": String("Product 2"), "price": Number(39.989999999999995)}}
cache:product:3: Object {"type": String("Object"), "value": Object {"name": String("Product 3"), "price": Number(49.989999999999995)}}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.85s
     Running `target/debug/examples/collection_management`
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: "XQ1z9U6UpcgERuWHvqqmxGFH74h8a5ChiRtyHqC5lvXtmRwxJPaR7RQX8O5U76pkIRxbcCCLqoaIldsGpO9M2w"

=== List Collections ===
Total collections: 5
Sample collections: ["test_collection", "websocket_test", "demo_collection", "audit__ek0_testing", "batch_users"]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.80s
     Running `target/debug/examples/document_ttl`
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "l1Om7bCTMMrgQkkXHgIh7e0-hUyINNwxDGPrttUpQFB2EW7aFwIs4TYh8DZI0C7RjKov5ikSOFp0rcEixtRUVQ"

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: "ucOMbuSo8UdoPgt8qPEQsH1IMJRDPJlCWzESqQd2WxnZWEBRBxlJOiP76aIShcvpK5uThQPCP7g0FHaBNPvOFg"

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: "DTtdpqHJKADQTX6dBZH5nwNPFHFCaQU4Tr2vIXEi3MIr9GTZlBV0_Yy4nFH7lrrLfN0DaCtQv9GmUB08abpPtQ"

=== Query Documents ===
✓ Found 3 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.11s
     Running `target/debug/examples/websocket_ttl`
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: String("k-wlRXeFJh0W17-R-6AEl-11FJw_KUj8Mu-zQScHMLa0oZSlSF8va2zlEqJEvJvFRAt_0pUiJtAAEGP6kAuPFw")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  id: "k-wlRXeFJh0W17-R-6AEl-11FJw_KUj8Mu-zQScHMLa0oZSlSF8va2zlEqJEvJvFRAt_0pUiJtAAEGP6kAuPFw"
  name: {"type":"String","value":"WebSocket TTL Test"}
  ttl: "2026-09-07T22:08:35.525060Z"
  value: {"type":"Integer","value":42}

✓ WebSocket TTL example completed successfully
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.86s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Functions Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: 2ifjrlP1syYyZBmCHTvXTaHjKGfFCrdBXuunN_h0jQzLhW_RVETFMx1wHLZOK65-5MLoaFx7W5_5xNHO-2fCOg
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: 1rU8QMqezgTnI5LUaimbi-SQzn2xCjMEW-lnO69OqhEBwruAldH8ixgvAwwlC_HK7HloBh9xY_lmsw-KVzL1ZQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Function saved: VEwj1umXns1q52WZzjLeNQyUYJ3dJlItfn5aHQATG8yPLBKhB1W5djxVUwncadGdeJjJ4DmSxM8U4JGeL1i_PQ
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: M-ovIF0LnpBDUeAoGvSsnAAZi9O3D_9gIB6wso-77QLGPmupzoGeDRt7e6c7RgWj0panA6JvnFXVTXwpZtSVTw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":{"type":"Float","value":60.0},"count":{"type":"Integer","value":5},"max_score":{"type":"Integer","value":100},"status":{"type":"String","value":"active"}}
   {"avg_score":{"type":"Float","value":50.0},"count":{"type":"Integer","value":5},"max_score":{"type":"Integer","value":90},"status":{"type":"String","value":"inactive"}}

📝 Example 4: Function Management

📋 Total scripts: 4
🔍 Retrieved script: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.84s
     Running `target/debug/examples/transactions`
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: JQFP02jyZoLTIdaPi8wVlSTjDeQzY7LTHFF0_8oONaV47ZEl0wbgnVO6myGXaoHn1hVJi9xaHUcSxa4VEVMqRg
Created Bob: $500 - ID: x-Zb_baqh4_1nKmz1LlcuYSPs4vzOicoT0FgMATJCOpCob-zvQ-89hMvMlw8nk0VxREA2PEbSjzn480if_T0TA

=== Example 1: Begin Transaction ===
Transaction ID: 274c1a7e-5d60-4ab6-8ea3-e9b20a7dc5e0

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
New transaction: 48777642-c500-449b-b343-3d8e90ede4fe
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"type":"Integer","value":700}

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✅ [32mRust direct examples complete![0m
🛠️  [36mBuilding client library...[0m
   Compiling tokio v1.53.1
   Compiling ring v0.17.14
   Compiling zerovec v0.11.8
   Compiling zerotrie v0.2.5
   Compiling futures-util v0.3.34
   Compiling serde v1.0.229
   Compiling compression-codecs v0.4.40
   Compiling ipnet v2.12.2
   Compiling thiserror v2.0.20
   Compiling serde_urlencoded v0.7.1
   Compiling chrono v0.4.45
   Compiling rmp-serde v1.3.1
   Compiling rust_decimal v1.43.0
   Compiling tinystr v0.8.4
   Compiling potential_utf v0.1.6
   Compiling rustls-webpki v0.103.15
   Compiling icu_locale_core v2.1.1
   Compiling icu_collections v2.1.1
   Compiling rustls v0.23.44
   Compiling icu_provider v2.1.1
   Compiling icu_properties v2.1.2
   Compiling icu_normalizer v2.1.1
   Compiling hyper v1.11.1
   Compiling tower v0.5.3
   Compiling async-compression v0.4.45
   Compiling tokio-util v0.7.19
   Compiling idna_adapter v1.2.1
   Compiling hyper-util v0.1.20
   Compiling tokio-rustls v0.26.5
   Compiling tungstenite v0.29.0
   Compiling idna v1.1.0
   Compiling url v2.5.8
   Compiling tokio-tungstenite v0.29.0
   Compiling hyper-rustls v0.27.9
   Compiling tower-http v0.6.11
   Compiling reqwest v0.12.28
   Compiling ekodb_client v0.26.1 (ekoDB/ekodb-client/ekodb_client)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 13.74s
✅ [32mClient build complete![0m
🧪 [36mRunning Rust client library examples...[0m
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.24s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record({"id": String("YOFezpbtCVI4E1e2SZBg1MwKBRCzmhiadShO3ApgeuNAIVxUoCujiC25CJCTRV0d-1k_-e5qVB-7Me5w28_KZQ")})

=== Find by ID ===
Found: Record({"categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "value": Object({"value": Integer(42), "type": String("Integer")}), "metadata": Object({"value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})}), "type": String("Object")}), "created_at": Object({"value": String("2026-09-07T21:11:34.448054+00:00"), "type": String("DateTime")}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "name": Object({"type": String("String"), "value": String("Test Record")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "price": Object({"value": Float(99.99), "type": String("Float")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "id": String("YOFezpbtCVI4E1e2SZBg1MwKBRCzmhiadShO3ApgeuNAIVxUoCujiC25CJCTRV0d-1k_-e5qVB-7Me5w28_KZQ"), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-09-07T21:11:34.448054+00:00")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 5 dims
  categories (Set): 2 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record({"active": Object({"value": Boolean(true), "type": String("Boolean")}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "id": String("YOFezpbtCVI4E1e2SZBg1MwKBRCzmhiadShO3ApgeuNAIVxUoCujiC25CJCTRV0d-1k_-e5qVB-7Me5w28_KZQ"), "name": Object({"value": String("Test Record"), "type": String("String")}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "metadata": Object({"value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")}), "type": String("Object")}), "value": Object({"value": Integer(42), "type": String("Integer")}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "created_at": Object({"value": String("2026-09-07T21:11:34.448054+00:00"), "type": String("DateTime")}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")})})]

=== Update Document ===
Updated: Record({"categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "metadata": Object({"type": String("Object"), "value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")})}), "value": Object({"value": Integer(100), "type": String("Integer")}), "name": Object({"type": String("String"), "value": String("Updated Record")}), "created_at": Object({"value": String("2026-09-07T21:11:34.448054+00:00"), "type": String("DateTime")}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "id": String("YOFezpbtCVI4E1e2SZBg1MwKBRCzmhiadShO3ApgeuNAIVxUoCujiC25CJCTRV0d-1k_-e5qVB-7Me5w28_KZQ"), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "price": Object({"value": Float(99.99), "type": String("Float")})})

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.46s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: 3H9Uang1n-a9I1Ed_hnfrSoaLDXKwEx9_69yYOfN0rc5P0wxgaxbz80pZNTqk_48nxt4RjS3Eh68tRNalzwfNA

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  active: true
  value: 42
  id: "3H9Uang1n-a9I1Ed_hnfrSoaLDXKwEx9_69yYOfN0rc5P0wxgaxbz80pZNTqk_48nxt4RjS3Eh68tRNalzwfNA"
  name: "WebSocket Test Record"

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.23s
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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.10s
     Running `target/debug/examples/client_collection_management`
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "sWkwSua2F8kl3IEpLxFV3lRYYMQPM2HJc3BOu7R2_naCaKtd1vjhZb-sb6luoifHoS8lZ0e_fqugz5GmyIHjag"

=== List Collections ===
Total collections: 11
Sample collections: ["ttl_cache", "test_collection", "websocket_test", "audit__ek0_testing", "functions__ek0_testing"]

=== Count Documents ===
Document count: 1

=== Check Collection Exists ===
Collection exists: true

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.02s
     Running `target/debug/examples/client_kv_operations`
✓ Client created

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: Object {"type": String("Object"), "value": Object {"userId": Number(123), "username": String("john_doe")}}

=== KV Batch Set ===
✓ Batch set 3 keys
  cache:product:1: success
  cache:product:2: success
  cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  cache:product:1: Record({"price": Float(29.99), "name": String("Product 1")})
  cache:product:2: Record({"name": String("Product 2"), "price": Float(39.989999999999995)})
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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.16s
     Running `target/debug/examples/client_transactions`
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: uLh35Q-1p9gZ20Z_0kbkIber-sS-VDHyERlEfJ9Ee65tfusOoCwpSP60K8bN773ltky2Wn1TDnOxx_f48-ahLQ
Created Bob: $500 - ID: eBxsv_DsRbVk-gltwih3ekBKf33dmq5xM4VoQ4nUcwNC1OfSv_-sWQXGUZnLsbsgwLDIyeYNgxdPU8Ytsua87w

=== Example 1: Begin Transaction ===
Transaction ID: 2d2691c6-6f17-45b0-bb82-5ff429fbe896

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: cd9a1915-5f78-4f25-9541-cd6085d941e0
Status before rollback: "Active"
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.19s
     Running `target/debug/examples/client_query_builder`
=== ekoDB Query Builder Example ===

=== Inserting Sample Data ===
✓ Inserted 5 users

=== Simple Equality Query ===
✓ Found 3 active users
  - Some(Object({"type": String("String"), "value": String("Alice")}))
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"type": String("String"), "value": String("Eve")}))
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Eve")}))
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))

=== NOT IN Operator ===
✓ Found 4 users not inactive

=== String Pattern Matching ===
✓ Found 5 users with @example.com email

=== Prefix Query (StartsWith) ===
✓ Found 1 users with names starting with A
  - Some(Object({"value": String("Alice"), "type": String("String")}))

=== Complex Query (active AND age >= 28 AND score > 1500) ===
✓ Found 2 users matching all conditions
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))

=== OR Query ===
✓ Found 2 users with age < 28 OR age > 32
  - Some(Object({"type": String("String"), "value": String("Charlie")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))

=== Sorted Query (by score descending) ===
✓ Top 3 users by score:
  1. Some(Object({"value": String("Bob"), "type": String("String")}))
  2. Some(Object({"type": String("String"), "value": String("David")}))
  3. Some(Object({"value": String("Alice"), "type": String("String")}))

=== Pagination (page 2, size 2) ===
✓ Page 2 results:
  - Some(Object({"type": String("String"), "value": String("Charlie")}))
  - Some(Object({"value": String("David"), "type": String("String")}))

=== NOT Operator ===
✓ Found 4 users NOT inactive

=== Complex Chained Query ===
✓ Found 3 users with all conditions

=== Cleanup ===
✓ Deleted collection

✓ All query builder operations completed successfully
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.14s
     Running `target/debug/examples/client_search`
=== ekoDB Search Example ===

=== Inserting Sample Documents ===
✓ Inserted 5 sample documents

=== Basic Text Search ===
✓ Found 3 results for 'programming'
  1. Score: 26.4000 - Some(Object {"type": String("String"), "value": String("Rust Programming")})
  2. Score: 13.2000 - Some(Object {"type": String("String"), "value": String("Python for Data Science")})
  3. Score: 13.2000 - Some(Object {"type": String("String"), "value": String("JavaScript Web Development")})

=== Fuzzy Search ===
✓ Found 3 results for 'progamming' (typo)
  1. Score: 2.2000 - Some(Object {"type": String("String"), "value": String("Rust Programming")})
  2. Score: 1.1000 - Some(Object {"type": String("String"), "value": String("Python for Data Science")})
  3. Score: 1.1000 - Some(Object {"type": String("String"), "value": String("JavaScript Web Development")})

=== Field-Specific Search ===
✓ Found 4 results in title/description
  1. Score: 2.0000
     Title: Some(Object {"type": String("String"), "value": String("Machine Learning Basics")})
     Matched: ["description", "title"]
  2. Score: 1.0000
     Title: Some(Object {"type": String("String"), "value": String("Python for Data Science")})
     Matched: ["description"]
  3. Score: 0.5000
     Title: Some(Object {"type": String("String"), "value": String("Database Design")})
     Matched: ["description"]
  4. Score: 0.5000
     Title: Some(Object {"type": String("String"), "value": String("Rust Programming")})
     Matched: ["description"]

=== Weighted Search ===
✓ Found 2 results with field weights
  1. Score: 23.1000 - Some(Object {"type": String("String"), "value": String("Python for Data Science")})
  2. Score: 3.3000 - Some(Object {"type": String("String"), "value": String("Machine Learning Basics")})

=== Advanced Search Options ===
✓ Found 1 results with stemming
  1. Score: 26.4000 - Some(Object {"type": String("String"), "value": String("Database Design")})

=== Search with Limit ===
✓ Limited to 2 results (requested 2)
  1. Score: 26.4000 - Some(Object {"type": String("String"), "value": String("Rust Programming")})
  2. Score: 13.2000 - Some(Object {"type": String("String"), "value": String("Python for Data Science")})

=== Search with a metadata pre-filter (category = programming) ===
✓ Found 2 results in category 'programming' (database/ai excluded)
  1. Some(Object {"type": String("String"), "value": String("Python for Data Science")}) (category: Some(Object {"type": String("String"), "value": String("programming")}))
  2. Some(Object {"type": String("String"), "value": String("Rust Programming")}) (category: Some(Object {"type": String("String"), "value": String("programming")}))

=== Cleanup ===
✓ Deleted collection

✓ All search operations completed successfully
Execution time: 2ms
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.18s
     Running `target/debug/examples/client_schema`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("34_Vk0LInxgOC02fuYrEs-NM0uPQRyhhPhSLmTLFQMNq3VBzm7e-xdIepxloz3tcLWYxZS21i5kwovb8C42BLw"))
✓ Inserted user 2: Some(String("lWwyH6yfmbsEIPY8l4MYhadIYfjTjT9jU9TEpYpQ9K9aB4yTzX-RUJUNzD6qTgPzGA3ZNzepEvkOt7nI3zOV9g"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - email: String
    (required)
  - age: Integer
  - status: String
  - title: String
    (required)

=== Listing Collections ===
✓ Total collections: 11
  Sample: ["ttl_cache", "test_collection", "websocket_test", "audit__ek0_testing", "functions__ek0_testing"]

=== Cleanup ===
✓ Deleted collection

✓ All schema management operations completed successfully
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.18s
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
  - Laptop ($1200) by Alice Johnson
  - Mouse ($25) by Alice Johnson

=== Example 5: Complex join with multiple conditions ===
✓ Found 2 users with example.com emails
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✓ Deleted test collections

✓ Join operations example completed successfully
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.13s
     Running `target/debug/examples/client_document_ttl`
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "IJwzYi9nvWMVctDO6a1fW00qYMZcU-JxiQJbYX_I3uFg1PHALbUpaWxBXr4dZ50Z7U1rIXf8ihid5ra9P9PF_A"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("4Q8h63l_eMM8yby2twpWR_cCNVWMZ9vXPq5wbmsKJMlB_SlvxZy9VIjT_haOkdv8O8WtZM44b7vWlebcnXQfGA"))

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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.44s
     Running `target/debug/examples/client_websocket_ttl`
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Some(String("Oz9ZxxZAsG-Q0ZZHPuAz1cVQzEFiSVy6FLHZM5WmG2EyrktliVdoWGhaIb82sXFdLRZqjSxLBF1F-YDr4_94LA"))

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.57s
     Running `target/debug/examples/client_edge_cache`
=== ekoDB as Edge Cache - Simple Example ===

Setting up edge cache collection...
✓ Cache entry created

Creating edge cache lookup script...
✓ Edge cache script created: wDe8Taxv5T-t75NfdHYD6hkLrkPFXap7-tQCicbTwstOw146o2kJbKE-ULu9ndhV21hBONBvEuwFEd6Gi4aRfQ

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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.60s
     Running `target/debug/examples/client_functions`
=== ekoDB Rust Client - Functions Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

ℹ️  Function 'get_active_users' already existed — updated instead
✅ Function saved: sGylymQg03HvEk5zxGblwwZdpCdEFu0e5j3_Wk5mle3K8_myTEnsX_uwwAuWxY67rVFTr98rRE0i7RD7KP4h-Q
📊 Found 20 active users

📝 Example 2: Parameterized Function

✅ Function saved: qHTPlxnOq56l_g4BTta9_9sNbgbaAuyO-EUEg74ushDzAufMb7_mW0nUAV_LABCY9o8KiCIHIRb55R0FS8YAfA
📊 Found 20 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: NBWtexlYhl5xknwtyPFxKcuITOO0lqoi0JZizbsphHo-jllX4AW8AmxxQ0-PAANdavIBbeEUyR-N-VN6UwZn8w
📊 Statistics: 2 groups

📝 Example 4: Function Management

📋 Total functions: 5
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.


✅ All examples completed!
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.63s
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
   ⏱️  Duration: 115.800042ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "username": "Bret",
      "website": "hildegard.org",
      "name": "Leanne Graham",
      "company": {
        "name": "Romaguera-Crona",
   ...

Second call (cache hit - from cache):
   ⏱️  Duration: 2.12525ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "phone": "1-770-736-8031 x56442",
      "name": "Leanne Graham",
      "email": "Sincere@april.biz",
      "id": 1,
      "company": {
       ...
   🚀 Cache speedup: 57.5x faster!

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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.68s
     Running `target/debug/examples/client_functions_complete`
🚀 ekoDB Rust Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: NYU431Gk4LnAbzRsGW2oVfdRQUut7jyjF4DDH8bdybGwgd5siDl7-95wOqaijvwSbjS7BfbYMbvroNnb6nA-jA
📊 Found 2 product groups
   Record({"avg_price": Object({"type": String("Float"), "value": Float(575.6666666666666)}), "category": Object({"value": String("Electronics"), "type": String("String")}), "count": Object({"value": Integer(3), "type": String("Integer")})})
   Record({"count": Object({"value": Integer(2), "type": String("Integer")}), "avg_price": Object({"value": Float(474.0), "type": String("Float")}), "category": Object({"value": String("Furniture"), "type": String("String")})})
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   Record({"count": Object({"type": String("Integer"), "value": Integer(2)}), "category": Object({"type": String("String"), "value": String("Furniture")})})
   Record({"count": Object({"value": Integer(3), "type": String("Integer")}), "category": Object({"value": String("Electronics"), "type": String("String")})})
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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.78s
     Running `target/debug/examples/client_functions_kv_wrapped`
🚀 ekoDB Rust KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: Some(String("Dy6bxRhqG4AZhm2UzQ2OoxpxMmDHy1lu40EL-q6iXHAUtr2b5_0mVVsUUkrdp3OrlT6uoCqxzpLPwWvfMDQtHQ"))
✅ Inserted 2 products with wrapped types

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: LM1q90Dize_rhOBXgYpkjJTBLbsjv35z-4fHkol6S50OI4z8gckA2IkVrIvUp9mCKQN-dKpro8aOJkU1DIB3fg
📊 function executed
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: Some(Object {"type": String("Object"), "value": Object {"role": String("admin"), "userId": String("user_abc")}})
🔍 Key exists: true
✅ Set cached data
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: adzT_PcRcvgBRn24hNKCHvQBU-bBy3UJRqydXVNZo4OpUACTjcJmkSxeUTWByxgiL-4tf1Q7JiaFX7wSuYzHuQ
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: MNiCMYgS0Yhx4c1iPHogl-ZNZlX9rmSHcsUMm5osmtWGYrYBZj9OERrFDZdy2XiK9drpJuYqK1MMc5w8PH9rkw
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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.52s
     Running `target/debug/examples/client_swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_rs (3fDQ9U1eGpd6z_UH7PLL6EtM3n0J0ds-thbeC8-iZzByYhg0gmLiVPfkCqkeek7G1HVsKsbaP5Yk7wXDJeb75g)

Step 2: First call - Cache miss, fetches from API
Result: FunctionStats { input_count: 0, output_count: 1, execution_time_ms: 0, stages_executed: 2, stage_stats: [] }
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 7ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.59s
     Running `target/debug/examples/client_swr_native`
=== ekoDB Native SWR Function ===

This example shows the new simplified SWR function that replaces
the manual FindById → If → HttpRequest → Insert pattern.

Example 1: Basic GitHub User Cache with Native SWR
─────────────────────────────────────────────────────

✓ Created native SWR script: github_user_native (o9h8OizHwOyIpGvjjg8mj4XX0g4yZ8RbsOhGt7IF-6-d5L2_affeRDY87IWmU4Yp-62yOQBhjYcgQHwUGSXxnA)

First call (cache miss - will fetch from GitHub API):
  Response time: 120ms
  Records returned: 1
  ✓ Data fetched from API and cached with 15m TTL

Second call (cache hit - instant from KV store):
  Response time: 3ms
  Speedup: 40.0x faster
  ✓ Lightning fast cache hit


Example 2: SWR with Audit Trail Collection
─────────────────────────────────────────────────────

✓ Created SWR script with audit trail: product_swr_audit (CGwbG2ra4hn3FGBxKZTwFWdiln8woMlTGI0X0CTS3QqWScAEoal_KJ2c5pyvzqsWuzK_3QkFynY9aqvCf6IhSw)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
─────────────────────────────────────────────────────

Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (4VV3DdXQEI9gfjcSSqLZwOIWTOJ77-YTH5C_8ogrriOzf9IA0xOYfNybgIblXaQhc_rhhvEZ-qJoArwTlMlFAQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
─────────────────────────────────────────────────────

✓ Created dynamic TTL script: flexible_cache (2i_fN-L8gcttQ9vBpF4F1qMnhXvYdnlIh5byp7x-zEcbRBK0pmQB5uBfO8tCsMnYy2ma5kKfBQUc8yUyPzo0lA)
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

   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.62s
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
   Record({"category": Object({"value": String("Electronics"), "type": String("String")}), "avg_price": Object({"value": Float(367.0), "type": String("Float")}), "count": Object({"type": String("Integer"), "value": Integer(5)})})
   Record({"count": Object({"value": Integer(3), "type": String("Integer")}), "avg_price": Object({"value": Float(365.6666666666667), "type": String("Float")}), "category": Object({"value": String("Furniture"), "type": String("String")})})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.56s
     Running `target/debug/examples/client_functions_ai`
🚀 ekoDB Rust AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.72s
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
   Record({"status": Object({"value": String("inactive"), "type": String("String")}), "count": Object({"value": Integer(3), "type": String("Integer")})})
   Record({"status": Object({"type": String("String"), "value": String("active")}), "count": Object({"type": String("Integer"), "value": Integer(7)})})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.65s
     Running `target/debug/examples/client_functions_search`
🚀 ekoDB Rust Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Natural Language Processing (AI)
   2. Database Design Principles (Database)
   3. Introduction to Machine Learning (AI)
   4. Getting Started with ekoDB (Database)
   5. Vector Databases Explained (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   Record({"count": Object({"value": Integer(3), "type": String("Integer")}), "category": Object({"type": String("String"), "value": String("Database")})})
   Record({"category": Object({"type": String("String"), "value": String("AI")}), "count": Object({"value": Integer(2), "type": String("Integer")})})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.25s
     Running `target/debug/examples/client_chat_basic`
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: EpncQpgraSLlTwxuY9OQk1S-6BHw17FPHrGe1hQSC1k3Y-mKu82KFBbgejEmRy0C6de3NyaqW20Z830tZ6GwRA

=== Sending Chat Message ===
Message ID: _bCyt54ZQRLmjWubbNHjdDtngJkzD8nAfRSRHEskSCh0wT-ZnAW1HO0FW7fIMlslTn-TEXsPKKX65xMxY-d3KA

=== AI Response ===
Response 1: ekoDB is a high-performance database that offers intelligent caching, real-time capabilities, and AI integration. It supports various advanced features, such as:

1. **AI Chat Integration**: With the chat feature, you can query your database using natural language, receiving AI-powered responses that include relevant context.

2. **Search Capabilities**: ekoDB provides multi-faceted search options, including:
   - **Full-text search**: For keyword-based queries.
   - **Vector search**: For semantic searches that understand the meaning rather than just the keywords.
   - **Hybrid search**: Combining both full-text and vector search capabilities, with automatic context retrieval.

These features make ekoDB a versatile option for developers looking to leverage AI and real-time data access in their applications.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["title", "category", "content"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("EDPxorvAUVQuKl6nHLQYiOFnsBZdYPoUHkUcWsyi9KI-L5rqMiEwhKBQN4b39OWqSGKckBypPio0Wbm7scRykA"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "content", "category"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("fdbKv4lcJHPR8o3cediWdpCEQgsFtRzoyyPxVDjr5lLlifI-klpxbOHsEeh6JN6jW_GoFN3XJBTpCyVI-hgOkw"), "title": String("Search Features")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "content"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("PfdCbwoZgZZDp-q7lIO9gLB_PAaEHc9aB7Rb0J1vGi05pylhuwS-txVk1JbCPAhO7NLbD9koEMQef_rSlQoQkw"), "title": String("Introduction to ekoDB")}

Execution Time: 2724ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.28s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: l6PazNM4BlvtXDWLhmCO0n9a7nLJ-P1BjPSgcoX7Jk0f0CQhl5Vag4OEJWCwx2vPhmMHwxV3WJ3K773W0neg6A

=== Sending Initial Message ===
✓ Message sent
  Response: It seems there are currently no active products available in the database. If you have specific criteria or categories you would like to explore, please let me know!

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
✓ Created second session: L1UQzFyFR4ccuvwyDYzO0gicZTwcGTyE2eObq41Pxc0Af_hs8haT87q3nMhpH4rHu1SgAXeMe2rlCNCgMNRpWA
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.32s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 1XLUyFhbuCQozWHtbWvx5Jjk1LEyfQxrk2ovJjO7bwll-2gtz50K88C5Zqr1REVwWLTNjHiDTqQmG2I9hmEMLQ
=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Product Name:** ekoDB
- **Description:** A high-performance database product with AI capabilities
- **Price:** $99 

Would you like to know anything else about this product or explore more options?

✓ Message 2 sent
  Response: The price of the product ekoDB is **$99**. Would you like to know more about it?

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: LR0ksPOrUjNXkn9MUh_pzsW-0A-yTc5h_afCXuz6jPxZC0l2BD7MWC22HCi_ls4XPNOGwHc1NpEE6bnb4fVd9g
  Parent: 1XLUyFhbuCQozWHtbWvx5Jjk1LEyfQxrk2ovJjO7bwll-2gtz50K88C5Zqr1REVwWLTNjHiDTqQmG2I9hmEMLQ

=== Listing Sessions ===
✓ Found 4 sessions
  Session 1: LR0ksPOrUjNXkn9MUh_pzsW-0A-yTc5h_afCXuz6jPxZC0l2BD7MWC22HCi_ls4XPNOGwHc1NpEE6bnb4fVd9g (Untitled)
  Session 2: 1XLUyFhbuCQozWHtbWvx5Jjk1LEyfQxrk2ovJjO7bwll-2gtz50K88C5Zqr1REVwWLTNjHiDTqQmG2I9hmEMLQ (Untitled)
  Session 3: L1UQzFyFR4ccuvwyDYzO0gicZTwcGTyE2eObq41Pxc0Af_hs8haT87q3nMhpH4rHu1SgAXeMe2rlCNCgMNRpWA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: LR0ksPOrUjNXkn9MUh_pzsW-0A-yTc5h_afCXuz6jPxZC0l2BD7MWC22HCi_ls4XPNOGwHc1NpEE6bnb4fVd9g

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.97s
     Running `target/debug/examples/client_chat_models`
✓ Client created

=== Get All Chat Models ===
Available chat models by provider:

OpenAI models (127):
  - text-embedding-ada-002
  - whisper-1
  - gpt-3.5-turbo
  - tts-1
  - gpt-3.5-turbo-16k
  - gpt-4-0613
  - gpt-4
  - davinci-002
  - babbage-002
  - gpt-3.5-turbo-instruct
  - gpt-3.5-turbo-instruct-0914
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
  - omni-moderation-latest
  - omni-moderation-2024-09-26
  - o1-2024-12-17
  - o1
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
  - gpt-5.4
  - gpt-5.4-nano-2026-03-17
  - gpt-5.4-nano
  - gpt-5.4-mini-2026-03-17
  - gpt-5.4-mini
  - gpt-image-2
  - gpt-image-2-2026-04-21
  - gpt-5.5
  - gpt-5.5-2026-04-23
  - gpt-5.5-pro
  - gpt-5.5-pro-2026-04-23
  - chat-latest
  - gpt-realtime-translate
  - gpt-realtime-2
  - gpt-realtime-whisper
  - gpt-5.6-sol
  - gpt-5.6-terra
  - gpt-5.6-luna
  - gpt-realtime-2.1
  - gpt-realtime-2.1-mini
  - gpt-transcribe
  - gpt-live-transcribe
  - gpt-6-astra

Anthropic models (11):
  - claude-fable-5-1
  - claude-opus-5
  - claude-sonnet-5
  - claude-fable-5
  - claude-opus-4-8
  - claude-opus-4-7
  - claude-sonnet-4-6
  - claude-opus-4-6
  - claude-opus-4-5-20251101
  - claude-haiku-4-5-20251001
  - claude-sonnet-4-5-20250929

Perplexity models (0):

Gemini models (0):

Provider status:
  anthropic: ok 11 models
  gemini: not_configured (unverified) No Gemini API Key
  openai: ok 127 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get Models for Specific Provider ===

openai models (127):
  - text-embedding-ada-002
  - whisper-1
  - gpt-3.5-turbo
  - tts-1
  - gpt-3.5-turbo-16k
  ... and 122 more

anthropic models (11):
  - claude-fable-5-1
  - claude-opus-5
  - claude-sonnet-5
  - claude-fable-5
  - claude-opus-4-8
  ... and 6 more
GetChatModel(perplexity) error: Record not found

✓ Chat Models API example complete
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.46s
     Running `target/debug/examples/client_user_functions`
✓ Client created

=== Create User Function ===
Created user function with ID: 7bYbzU8VG8GC9vms_tCr5eG0jfLUR0_k0C0WFGdpbCEXvDp4YUT6WbgLHHxnVAhVZoPXHsMs-4_Rl2WmEt1Qfw

=== Get User Function ===
Retrieved: get_active_users_rs - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 12 user functions:
  - validate_user: Check if user exists
  - get_user_wrapper: Wrapper that calls fetch_user
  - swr_user: SWR pattern for user data (KV-based)
  - get_active_users_paginated: Get Active Users (Paginated)
  - get_active_users_rs: Get Active Users
  - get_users_by_status: Get Users By Status
  - get_high_scoring_active_users: Get High Scoring Active Users
  - fetch_user: Fetch user by code
  - get_verified_user: Get verified and validated user
  - fetch_slim_user: Validate and slim down user
  - get_active_users_updated: Get Active Users (Updated)
  - fetch_and_store_user: Fetch user from API and cache in KV

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.24s
     Running `target/debug/examples/client_convenience_methods`
=== ekoDB Convenience Methods Example ===

=== Record Builder Pattern ===
✓ Created record with fluent builder: Record({"id": String("Xq7kuhDN9xAA3deSehfJDqWwu3yXnEnXP7cVrtVBLmf8TeJuNuZp7l7CU8miSN8q9SN70IsOEnBv8TSW30sKEw")})

=== Upsert Operation ===
✓ First upsert (insert): Record({"id": String("pX-hm57WybYW9D5-KUiUp5kgwfLX4HZfdAUJxAfz3Azsi4H9cufURtbvQTjZBom6B9FJ1Yt7XvV96x9vBavTTA")})
✓ Second upsert (update): Record({"id": String("3tMZgh2C1zjMqFbdlttcVgKdEZ4XfOL9PHQbzQoLF9pmi4rd-8R-leiq_JKX7grZvSqiKZ_s69mtRLmba-OeVQ")})

=== Find One Operation ===
✓ Found user by email: Record({"id": String("Xq7kuhDN9xAA3deSehfJDqWwu3yXnEnXP7cVrtVBLmf8TeJuNuZp7l7CU8miSN8q9SN70IsOEnBv8TSW30sKEw"), "name": Object({"type": String("String"), "value": String("Alice Johnson")}), "email": Object({"type": String("String"), "value": String("alice@example.com")}), "age": Object({"value": Integer(28), "type": String("Integer")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)})})
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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.17s
     Running `target/debug/examples/bypass_ripple_example`
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: Record({"id": String("TX7P_YS66RS49z2hh0q6Ex4g84Usv7YvQ9V0dud3_xkBGqAYokAB4PqyyBxZSjB1IA06GDpAnIHyruYSXKKl-A")})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: Record({"id": String("Rwqs2_0gIxZi-a2Od4JWbywm_Nxh5HMK5mwpkCf4dyj8fqvZOL1nszTEVdVDfxSFCdhCB2PcYV2NkyHPL-UIag")})

3. Update with bypass_ripple:
   Updated with bypass_ripple: Record({"name": Object({"value": String("Product 1"), "type": String("String")}), "price": Object({"type": String("Integer"), "value": Integer(150)}), "id": String("TX7P_YS66RS49z2hh0q6Ex4g84Usv7YvQ9V0dud3_xkBGqAYokAB4PqyyBxZSjB1IA06GDpAnIHyruYSXKKl-A")})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: Record({"id": String("eVjU0YqgI204MSb2GVmsX4h0Q6W4t8M7lPlfYDwYMEmTFeOMYYMWFA6HtVK_uFV5_p_IJ-3SGZ1ijOu-lyf4KA")})

✅ All bypass_ripple operations completed successfully!
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.18s
     Running `target/debug/examples/projection_example`
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ["id", "name", "email"]

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["bio", "user_role", "age", "avatar_url", "status", "email", "name", "created_at", "id"]

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)

Example 4: Query inactive users with profile fields
  Found 1 inactive users

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ["status", "password", "email", "user_role", "name", "id", "avatar_url", "bio", "created_at", "api_key", "secret_token", "age"]
  Projected query:
    - 3 fields per record
    - Fields: ["id", "name", "email"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.03s
     Running `target/debug/examples/client_jwt_auth_flow`
✓ Client created
✓ rs_users_register saved
✓ rs_users_login saved
✓ rs_users_verify_token saved

=== Auth flow defined as pure stored functions ===
Call them like:
  POST /api/functions/rs_users_register { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/rs_users_login    { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/rs_users_verify_token { "token": "<jwt>" }

Set JWT_SECRET in ekoDB's environment_vars whitelist before invoking — the {{env.JWT_SECRET}} placeholder reads from that whitelist, NEVER from the function definition.

✓ Cleaned up demo functions
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.05s
     Running `target/debug/examples/client_crypto_stages`
✓ Client created
✓ crypto_demo_hmac saved
✓ crypto_demo_aes saved
✓ crypto_demo_uuid saved
✓ crypto_demo_totp saved
✓ crypto_demo_encoding saved

All crypto-stage demos defined. Invoke any of them with:
  POST /api/functions/crypto_demo_hmac     { "payload": "hi" }
  POST /api/functions/crypto_demo_aes      { "plaintext": "secret" }
  POST /api/functions/crypto_demo_uuid
  POST /api/functions/crypto_demo_totp
  POST /api/functions/crypto_demo_encoding { "title": "Héllo World" }

✓ Cleaned up demo functions
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.06s
     Running `target/debug/examples/client_concurrency_stages`
✓ Client created
✓ conc_demo_pay saved
✓ conc_demo_rl_fail saved
✓ conc_demo_rl_skip saved
✓ conc_demo_lock saved

Invoke them like:
  POST /api/functions/conc_demo_pay        { "idempotency_key": "...", "amount": 100 }
  POST /api/functions/conc_demo_rl_fail    { "user_id": 42 }
  POST /api/functions/conc_demo_rl_skip    { "user_id": 42 }
  POST /api/functions/conc_demo_lock       { "resource": "queue:drain" }

✓ Cleaned up demo functions
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.16s
     Running `target/debug/examples/client_path_routed_function`
✓ Client created
✓ rs_route_admin → GET /api/route/users/admin
✓ rs_route_user_by_id → GET /api/route/users/:id
✓ rs_route_user_posts → GET /api/route/users/:id/posts/:post_id
✓ rs_route_org_create_member → POST /api/route/orgs/:org/members

Try them with curl:
  curl http://localhost:8080/api/route/users/admin
  curl http://localhost:8080/api/route/users/42
  curl http://localhost:8080/api/route/users/42/posts/7
  curl -X POST http://localhost:8080/api/route/orgs/acme/members \
       -H 'Content-Type: application/json' -d '{"name":"alice"}'

✓ Cleaned up demo functions
✅ [32mRust client examples complete![0m
✅ [32mAll Rust integration tests complete![0m
📦 [36mEnsuring Python example dependencies in .venv...[0m
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'a1Q5wlEhvqySoP3kgNk40K5Ijz_ty6228uY7R3jdt1PWY_9AwRQWmzaOuEYcUY357OSO7cUHcKCPURGECnCL_g'}

=== Find by ID ===
Found: {'name': {'value': 'Test Record', 'type': 'String'}, 'value': {'type': 'Integer', 'value': 42}, 'id': 'a1Q5wlEhvqySoP3kgNk40K5Ijz_ty6228uY7R3jdt1PWY_9AwRQWmzaOuEYcUY357OSO7cUHcKCPURGECnCL_g', 'active': {'type': 'Boolean', 'value': True}}

=== Find with Query ===
Found documents: [{'id': 'a1Q5wlEhvqySoP3kgNk40K5Ijz_ty6228uY7R3jdt1PWY_9AwRQWmzaOuEYcUY357OSO7cUHcKCPURGECnCL_g', 'value': {'type': 'Integer', 'value': 42}, 'active': {'type': 'Boolean', 'value': True}, 'name': {'value': 'Test Record', 'type': 'String'}}]

=== Update Document ===
Updated: {'active': {'type': 'Boolean', 'value': True}, 'value': {'value': 100, 'type': 'Integer'}, 'id': 'a1Q5wlEhvqySoP3kgNk40K5Ijz_ty6228uY7R3jdt1PWY_9AwRQWmzaOuEYcUY357OSO7cUHcKCPURGECnCL_g', 'name': {'value': 'Updated Record', 'type': 'String'}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: eo4k2XKeeP4Jw-NIruzkp5BZ37laWem-xan_dukdvdrmZzoejTtyeo5p4ZQXOFLyMu6bG4acNriPRHzHTq0EoQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "type": "Success",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "id": "eo4k2XKeeP4Jw-NIruzkp5BZ37laWem-xan_dukdvdrmZzoejTtyeo5p4ZQXOFLyMu6bG4acNriPRHzHTq0EoQ",
        "value": {
          "type": "Integer",
          "value": 42
        }
      },
      {
        "value": {
          "value": 42,
          "type": "Integer"
        },
        "id": "WsbRCK94m4hP3hqb2DP9BPADED5Qb3sY8pyagkVQFOgdbEuIFn84mi1fBQkRrv_aJlCNvJp3vQhqcdHXQRFFCQ",
        "name": {
          "value": "WebSocket Test Record",
          "type": "String"
        },
        "active": {
          "value": true,
          "type": "Boolean"
        }
      }
    ]
  },
  "messageId": "1203295986"
}
✓ Retrieved 2 record(s) via WebSocket

✓ WebSocket example completed successfully
🚀 ekoDB Functions Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: aQOhxtl8FqlCWAfLt2kwxkBBWENcuPzH6NvLErvi6igzH0oBBanpH3WNqr33v8qjesn0v97Aap20UK57dLbiXg
📊 Found 15 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

ℹ️  Function 'get_active_users_paginated' already existed — updated instead
✅ Function saved: get_active_users_paginated
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: Vdeac4D5AE1Yu0e01zhmaHhYZ9Nm9THgZSo8j-SRN9MPn0a91xHPNoeRkmeySi10cTclh4mUgBgwDOuI9FfrNA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'max_score': {'type': 'Integer', 'value': 100}, 'count': {'type': 'Integer', 'value': 15}, 'avg_score': {'value': 60.0, 'type': 'Float'}, 'status': {'value': 'active', 'type': 'String'}}
   {'count': {'type': 'Integer', 'value': 15}, 'max_score': {'type': 'Integer', 'value': 90}, 'avg_score': {'type': 'Float', 'value': 50.0}, 'status': {'value': 'inactive', 'type': 'String'}}

📝 Example 4: Function Management

📋 Total scripts: 13
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
Retrieved value: {'value': {'username': 'john_doe', 'userId': 123}, 'type': 'Object'}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: {'value': {'name': 'Product 1', 'price': 29.99}, 'type': 'Object'}
cache:product:2: {'type': 'Object', 'value': {'price': 39.989999999999995, 'name': 'Product 2'}}
cache:product:3: {'value': {'name': 'Product 3', 'price': 49.989999999999995}, 'type': 'Object'}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: IYUSaUbLfQ_PKDpAZKhyblsLz6GnsPXC-yLxOt0yHSTMEmRyHuWZJDTUonE0oZsI2oQYCDVTA56Lr-ZWsBoPwQ

=== List Collections ===
Total collections: 15
Sample collections: ['products', 'ttl_cache', 'test_collection', 'websocket_test', 'demo_collection']

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
Created Alice: $1000 - ID: 4zSyg9uMWXBIZp5pdG09vVVtv-9JKQZDx9Jy3_2nItvRFPlgsUU77I4c_z19B4GKtpXm168T0opDja99SlzFNQ
Created Bob: $500 - ID: zI9zMrj6Kzlk4V0jbY1gmKyJxx8D4xxhnAXuefWbGfxMt6E_-_eF35NFdo15OWOQ25541Td_NmEr8vfz9y65Dg

=== Example 1: Begin Transaction ===
Transaction ID: 04a42851-e07f-4666-873d-2d5cb7c9b68b

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
New transaction: 9f001080-8c8b-4a0f-9242-84e9804fc2f7
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: j6zd2pmeUKTOIWURRGc0TMgyKxlrn2hh1PHEwwf4KCOELgZetVE4kj5M13d-rNuO1cqvhEJWFd2m7fnr81E9qg
Created Bob: $500 - ID: XDzRzFlO-5foII8yZ6E3M_8g7u89ytQhJp_jUmnNBIXME0URjOy-2Mj05Epi7jEXkBuoxugqc8NxQx2FXYvGGA

=== Example 1: Begin Transaction ===
Transaction ID: 2c764a89-78a4-4913-9aff-0847db854e8b

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'value': 800, 'type': 'Integer'}
Bob: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ Deleted test accounts


🔄 Switching to NON-DURABLE mode...
   ✓ Config updated: durable_operations=false


============================================================
🔥 TEST 2: Non-Durable Mode (durable=false)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: O2e0z3Z7haFSsDNinnxuZI0l79hjKcsIChWQt0NHBJ_-YDk6aiNQHRx_G2oLgFo5hF0e5CnWegh8rhVcDe0Scw
Created Bob: $500 - ID: RQqc9MkpQtQqh0uHellOafhMUu0ch5X2OpY9iTVl8WmjMddflQPaE2bCVpNCi-tSvt7Zq8IJS_WWQok5JZQ8aQ

=== Example 1: Begin Transaction ===
Transaction ID: 8d2a2e82-fcad-48f8-b21a-100b3b07f7c3

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
New transaction: 5095ef61-af82-4604-b77f-9e0738bb15ac
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 2Meq2Oq-bqtXEyOgzll3gkZb-q6_Q1fdYb6wb5vNf4KuMtAY_1uzWdNRuHyvRu9fU6spY2tmv5mIAZiOr5k20g
Created Bob: $500 - ID: D_IT-TIbTRv_DwyUjYrlz_-RUwJZ7pwC3QLuafUl1UuY0Q5VYb9JrMAJOaAro0Ox7iIMcRHi105wQ9U9ObwljA

=== Example 1: Begin Transaction ===
Transaction ID: a13f161a-634f-43f4-a6cd-18e608e32199

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
Created Alice: $1000 - ID: RbFJVZCdpm7b4YpE4q70AIWNQke1LXS5ijSm0lBOCiUnNwG5F9xeSis-aVlo_ACyaimnX46zchlM4GwaVySTmQ
Created Bob: $500 - ID: Bsef3UwZoq_OfybqwAFmMSW_IsZtAaT--GSFuxHE06pWj6GQPm5OW70B5b06bk8yTnp2anUMU5J_QCdvsKK28Q

=== Example 1: Begin Transaction ===
Transaction ID: f71f1ab5-04f9-4cfa-b415-a05bdd418300

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
New transaction: bde2f986-03b4-4b1e-9523-4c8e56b91c7e
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: ZbNsO3K7Oya9NXlx-TqEKwRLpKB6rY1FVnb14SslwthsQT6ScpmY_ldxKdkCUjmKlyLxL5eI_sIrbJ8j5-uakw
Created Bob: $500 - ID: gp1DZNgs4Lzb0rfvDBND95uZzQgFV5feI-hT5WwXI3uIuE56U8njvy5n-FeHnECrsB-U7A2M7OL5ULC1NrNmOw

=== Example 1: Begin Transaction ===
Transaction ID: cdffb018-57be-4344-acca-ad0b472cf2eb

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'value': 800, 'type': 'Integer'}
Bob: ${'value': 700, 'type': 'Integer'}

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
   ✅ Function saved: 7awGHMBG3oKYZRgo1KQ3iJmzODKVXIRWTOk4NaMzz94Z5D8JhvQWsdo6Eo9b4LmYi9gvbnzAQgpMmMEwCSOG6A

2️⃣ Calling function (Insert + Verify)...
   ✅ function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: 3MzLRJwJJ6yNGJ_0olLOju--BUsYajj5V0dNR_NnFLJkQhTwdvFCskt1qDfvuuFvVDHvMuW6MkIV-zbil4DRJA
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'value': 'pending', 'type': 'String'}
   📋 Credits: {'type': 'Integer', 'value': 0}

============================================================
📝 function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: 16rRnN2tEGU81ceTkwgfLp8GKbkECl47-2OfFs8gd98ZMj5pw5zhVAGG4TxNSYLmAMBiLG2FjPq7Ba9s7fylAg

2️⃣ Calling function (Query + Update + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {'value': 'active', 'type': 'String'}
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}

============================================================
📝 function 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: mKgNzr8opctVS7c3bNIp2tHfa2GONJHak1BuroblMaA2K-CiWr81FZhL36-EpTVvjlhSgkoHz5hE3-8skAgbFQ

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
   ✅ Function saved: B10xKdCtBK0r4c8P1doG-hDuh52tOyTmZdCz9UVcZiz5Eqb65ttmMvOP-w2RtVG3ZvkchxAzMOxabWX_ztNJLg

2️⃣ Calling function (Query + Delete + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: 7awGHMBG3oKYZRgo1KQ3...
   ✅ Deleted script: 16rRnN2tEGU81ceTkwgf...
   ✅ Deleted script: mKgNzr8opctVS7c3bNIp...
   ✅ Deleted script: B10xKdCtBK0r4c8P1doG...
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
  Output: Document ID = EXYBokqZpVvuW-34sPo0YXAmw4reCUI8ZaIZ9Az0DqUc6cwRHjhnAKVCBUYnjYpkS-EWaF-1fu9kxW_iIEY6Ew
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(EXYBokqZpVvuW-34sPo0YXAmw4reCUI8ZaIZ9Az0DqUc6cwRHjhnAKVCBUYnjYpkS-EWaF-1fu9kxW_iIEY6Ew)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(EXYBokqZpVvuW-34sPo0YXAmw4reCUI8ZaIZ9Az0DqUc6cwRHjhnAKVCBUYnjYpkS-EWaF-1fu9kxW_iIEY6Ew)
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
  Output: Document ID = OQ5OXwbGJmpAEsftgWLJgP3NMpuYwf2jARmim1WGlMJEjVclruKyTkpjJeTiYfMiy3ybFYqUUMoZe3yW2rAHyw
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(OQ5OXwbGJmpAEsftgWLJgP3NMpuYwf2jARmim1WGlMJEjVclruKyTkpjJeTiYfMiy3ybFYqUUMoZe3yW2rAHyw)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(OQ5OXwbGJmpAEsftgWLJgP3NMpuYwf2jARmim1WGlMJEjVclruKyTkpjJeTiYfMiy3ybFYqUUMoZe3yW2rAHyw)
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
🔧 [36mEnsuring maturin is available in .venv...[0m
🔨 [36mBuilding wheel...[0m
🍹 Building a mixed python/rust project
🐍 Found CPython 3.11 at /Library/Frameworks/Python.framework/Versions/3.11/bin/python3
🔗 Found pyo3 bindings with abi3-py3.8 support
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
   Compiling smallvec v1.16.0
   Compiling find-msvc-tools v0.1.12
   Compiling syn v3.0.5
   Compiling mio v1.2.3
   Compiling crc32fast v1.5.1
   Compiling rustls v0.23.44
   Compiling miniz_oxide v0.9.1
   Compiling ipnet v2.12.2
   Compiling cc v1.4.5
   Compiling parking_lot_core v0.9.12
   Compiling parking_lot v0.12.5
   Compiling flate2 v1.1.10
   Compiling ring v0.17.14
   Compiling compression-codecs v0.4.40
   Compiling zerovec-derive v0.11.6
   Compiling tokio-macros v2.7.2
   Compiling displaydoc v0.2.7
   Compiling futures-macro v0.3.34
   Compiling serde_derive v1.0.229
   Compiling thiserror-impl v2.0.20
   Compiling tokio v1.53.1
   Compiling zerotrie v0.2.5
   Compiling futures-util v0.3.34
   Compiling zerovec v0.11.8
   Compiling thiserror v2.0.20
   Compiling serde v1.0.229
   Compiling tinystr v0.8.4
   Compiling potential_utf v0.1.6
   Compiling icu_collections v2.3.0
   Compiling icu_locale_core v2.3.0
   Compiling serde_urlencoded v0.7.1
   Compiling chrono v0.4.45
   Compiling rust_decimal v1.43.0
   Compiling rmp-serde v1.3.1
   Compiling icu_provider v2.3.1
   Compiling icu_normalizer v2.3.0
   Compiling icu_properties v2.3.0
   Compiling rustls-webpki v0.103.15
   Compiling idna_adapter v1.2.2
   Compiling hyper v1.11.1
   Compiling idna v1.1.0
   Compiling tokio-util v0.7.19
   Compiling async-compression v0.4.45
   Compiling url v2.5.8
   Compiling tower v0.5.3
   Compiling pyo3-async-runtimes v0.29.0
   Compiling hyper-util v0.1.20
   Compiling tower-http v0.6.11
   Compiling tokio-rustls v0.26.5
   Compiling tungstenite v0.29.0
   Compiling hyper-rustls v0.27.9
   Compiling reqwest v0.12.28
   Compiling tokio-tungstenite v0.29.0
   Compiling ekodb_client v0.26.1 (ekoDB/ekodb-client/ekodb_client)
   Compiling ekodb-client-py v0.26.1 (ekoDB/ekodb-client/ekodb-client-py)
    Finished `release` profile [optimized] target(s) in 1m 05s
📦 Built wheel for abi3 Python ≥ 3.8 to ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.26.1-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel into .venv...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.26.1-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.26.1
    Uninstalling ekodb_client-0.26.1:
      Successfully uninstalled ekodb_client-0.26.1
Successfully installed ekodb-client-0.26.1
🧪 [36mEnsuring test dependencies (pytest) in .venv...[0m
✅ [32mPython client package built and installed![0m
🧪 [36mRunning Python client library examples...[0m
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': 'EvF-JhBbxnRr5xUGeSBQABQ4fyFXmJ9QIoMUG_xDZmd1GRZe-d4bDWi95EG0keEHVPK44nDaVp3_Rym3D-1AUw'}

=== Find by ID ===
Found: {'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'metadata': {'type': 'Object', 'value': {'nested': {'deep': True}, 'key': 'value'}}, 'id': 'EvF-JhBbxnRr5xUGeSBQABQ4fyFXmJ9QIoMUG_xDZmd1GRZe-d4bDWi95EG0keEHVPK44nDaVp3_Rym3D-1AUw', 'value': {'type': 'Integer', 'value': 42}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'active': {'value': True, 'type': 'Boolean'}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'price': {'type': 'Float', 'value': 99.99}, 'name': {'value': 'Test Record', 'type': 'String'}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'created_at': {'type': 'String', 'value': '2026-09-07T17:45:51.582285'}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-09-07 17:45:51.582285
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'nested': {'deep': True}, 'key': 'value'}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'tags': ['tag1', 'tag2', 'tag3'], 'categories': ['electronics', 'computers'], 'metadata': {'nested': {'deep': True}, 'key': 'value'}, 'id': 'EvF-JhBbxnRr5xUGeSBQABQ4fyFXmJ9QIoMUG_xDZmd1GRZe-d4bDWi95EG0keEHVPK44nDaVp3_Rym3D-1AUw', 'value': 42, 'data': 'aGVsbG8gd29ybGQ=', 'active': True, 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'price': 99.99, 'name': 'Test Record', 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'created_at': '2026-09-07T17:45:51.582285'}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'metadata': {'type': 'Object', 'value': {'key': 'value', 'nested': {'deep': True}}}, 'price': {'value': 99.99, 'type': 'Float'}, 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}, 'value': {'type': 'Integer', 'value': 100}, 'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}, 'id': 'EvF-JhBbxnRr5xUGeSBQABQ4fyFXmJ9QIoMUG_xDZmd1GRZe-d4bDWi95EG0keEHVPK44nDaVp3_Rym3D-1AUw', 'created_at': {'value': '2026-09-07T17:45:51.582285', 'type': 'String'}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'active': {'type': 'Boolean', 'value': True}, 'name': {'value': 'Updated Record', 'type': 'String'}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: YiKAR-m432k_Dbi7mQlK7DK8rQMEcoswMXGqWyhbiLZaVRJ9rVlQ3AuYpS2NW1BSlowl7J6LKSvuvfjxJkGK7w

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
Collection created with first record: "ZQ-lYIMJXpju2ZIVubirsLkSx_mqCR-W32kJHqxF6ZSXPoCIAWbX3OgRjUzSdp1DoG0bNmDvdMXQMVbUM6SqOg"

=== List Collections ===
Total collections: 14
Sample collections: ['products', 'ttl_cache', 'test_collection', 'websocket_test', 'chat_configurations__ek0_testing']

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
Retrieved value: {'value': '{"type":"Object","value":{"userId":123,"username":"john_doe"}}'}

=== KV Batch Set ===
✓ Batch set 3 keys
  cache:product:1: success
  cache:product:2: success
  cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  cache:product:1: {'price': 29.99, 'name': 'Product 1'}
  cache:product:2: {'name': 'Product 2', 'price': 39.99}
  cache:product:3: {'price': 49.99, 'name': 'Product 3'}

=== KV Exists ===
Key exists: True

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 6

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
Created Alice: $1000 - ID: yhTEl0mjHcP2H6FwcMMYeoU3eW8cEDdgv25XNOCxsU2gbjR53ZQDKWQ1bsjCDAKcHe7adwDpUpSGiWDTHbCzAQ
Created Bob: $500 - ID: _3s4vR0IQGYe8d2ACxN379RK5YrHHIgHf6XzdAIW3u8qyOGEZoCqXcXaGuwGSDuEz8LqRenoWuJqxWrmT7aMIA

=== Example 1: Begin Transaction ===
Transaction ID: a9f53d07-b148-4254-839b-3c2ad91f4194

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 0502005d-e8d0-4a33-8284-26ddf2c501d7
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
  1. Score: 12.870, Matched: email, name
  2. Score: 6.270, Matched: name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 13.200, Matched: bio, title
  2. Score: 13.200, Matched: bio, title
  3. Score: 13.200, Matched: bio, title
  4. Score: 13.200, Matched: title, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400, Matched: title, bio
  2. Score: 26.400, Matched: bio, title
  3. Score: 26.400, Matched: title, bio
  4. Score: 26.400, Matched: bio, title

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 6.600, Matched: bio
  2. Score: 6.600, Matched: bio

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 6.600, Matched: bio
  2. Score: 6.600, Matched: bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.751
  2. Score: 0.732
  3. Score: 0.720

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.366, Matched: title, content
  2. Score: 0.860, Matched: content, title
  3. Score: 0.375, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 6.600, Matched: title

9. Vector search with a metadata pre-filter (category = ml):
Found 2 documents in category "ml" (NLP excluded)
  1. Introduction to Machine Learning (category: ml)
  2. Deep Learning Fundamentals (category: ml)

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
✓ Inserted document: IhIyxXzg8h0v-2lDMxyW9LKyybpO0CnW9Jg8ll0XkVZ32mKfQkMjm_qWumr__jglVRrKqafRpJGH7-BChcJdgQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: X0-6UWcWwTRAX2GNzyLOwRxloOQsMxa-rsHCMDIUFkLFtIpCen1Xd_K7RJwzpfaEjTMe4oUMP2zDnkbuKJLuvQ

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
✓ Inserted document with TTL: I52ollVSHC7QKSNotyzSJdi7rZclamQASY9cQ2QhAOouqEKRKbOAfNjKny21H_rDOC8ZODPiwTJdR4LPMviQ5A

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
✓ Edge cache script created: SabLIGLiEs9jEZhgP33XWXfHibOVcf260UhOo0gsTlpv2pV1XXZBnQM_COSVfoqjk4hOtcU2Yj1_fDfAgSn1NA

Call 1: Cache miss (fetches from API)
Response time: 463ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "current": {
            "interval": 900,
            "temperature_2m": 26.4,
            "time": "2026-09-07T21:45"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "\u00b0C",
            "time": "iso8601"
          },
          "elevation": 32.0,
          "generationtime_ms": 0.020742416381835938,
          "latitude": 40.710335,
          "longitude": -73.99308,
          "timezone": "GMT",
          "timezone_abbreviation": "GMT",
          "utc_offset_seconds": 0
        }
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
Response time: 14ms (34.2x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "current": {
            "interval": 900,
            "temperature_2m": 26.4,
            "time": "2026-09-07T21:45"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "\u00b0C",
            "time": "iso8601"
          },
          "elevation": 32.0,
          "generationtime_ms": 0.020742416381835938,
          "latitude": 40.710335,
          "longitude": -73.99308,
          "timezone": "GMT",
          "timezone_abbreviation": "GMT",
          "utc_offset_seconds": 0
        }
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

ℹ️  Function 'get_active_users' already existed — updated instead
✅ Function saved: get_active_users
📊 Found 10 active users

📝 Example 2: Parameterized Function

ℹ️  Function 'get_users_by_status' already existed — updated instead
✅ Function saved: get_users_by_status
📊 Found 10 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: Bej__DMx1TsPCHeDWAqjwQrK4yO280cLF349X8Qe6QWmaH11P6KCmowRlDCfcUUiBo_WGA9A62jgkrwu4W5O2Q
📊 Statistics: 2 groups
   {'avg_score': {'type': 'Float', 'value': 60.0}, 'count': {'type': 'Integer', 'value': 5}, 'status': {'type': 'String', 'value': 'active'}}

   {'avg_score': {'type': 'Float', 'value': 50.0}, 'count': {'type': 'Integer', 'value': 5}, 'status': {'type': 'String', 'value': 'inactive'}}

📝 Example 4: Function Management

📋 Total functions: 14
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

ℹ️  Function 'fetch_user' already existed — updated instead
✅ Saved reusable function: fetch_user
ℹ️  Function 'get_user_wrapper' already existed — updated instead
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"type": "String", "value": "User 1"}
   Department: {"type": "String", "value": "engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

ℹ️  Function 'fetch_and_store_user' already existed — updated instead
✅ Saved reusable function: fetch_and_store_user (uses KV)
ℹ️  Function 'swr_user' already existed — updated instead
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 87.5ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "type": "Object",
            "value": {
                  "address": {
                        "city": "Gwenborough",
                        "geo": {
                 ...

Second call (cache hit - from cache):
   ⏱️  Duration: 2.5ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "type": "Object",
            "value": {
                  "address": {
                        "city": "Gwenborough",
                        "geo": {
                 ...
   🚀 Cache speedup: 35.3x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

ℹ️  Function 'validate_user' already existed — updated instead
✅ Level 1 function: validate_user
ℹ️  Function 'fetch_slim_user' already existed — updated instead
✅ Level 2 function: fetch_slim_user (calls validate_user)
ℹ️  Function 'get_verified_user' already existed — updated instead
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

✅ Function saved: j-FfwMe1hD5AEe8kef_rXOrbbxR7M3AT5sl3U2ebQYeLegwzCnQxXdCb1Ifr-X6AlLioQndyUiDyRC56_hPXOA
📊 Found 2 product groups
   {'avg_price': {'type': 'Float', 'value': 474.0}, 'category': {'type': 'String', 'value': 'Furniture'}, 'count': {'type': 'Integer', 'value': 2}}
   {'avg_price': {'type': 'Float', 'value': 575.6666666666666}, 'category': {'type': 'String', 'value': 'Electronics'}, 'count': {'type': 'Integer', 'value': 3}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {'category': {'type': 'String', 'value': 'Furniture'}, 'count': {'type': 'Integer', 'value': 2}}
   {'category': {'type': 'String', 'value': 'Electronics'}, 'count': {'type': 'Integer', 'value': 3}}
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

✅ Inserted order: zHL8H7SIXdHrqfKsHUkMUTVkOsRpogO3ba7f10hXBGERNXKXDafEtzbiHPdx6xLsxQ69UehQeBC_q1oCVtb5_A
✅ Inserted 2 products with wrapped types

📝 Example 2: Querying and Extracting Wrapped Types

📊 Found 2 products
   • Wireless Mouse
   • Laptop Pro

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {'value': '{"type":"Object","value":{"role":"admin","userId":"user_abc"}}'}
🔍 Key exists: True
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 5: Combined Wrapped Types + KV Usage

✅ Inserted order: 5jLe2y4E2tQzz6qBGahbdXfd7Tf_3jje44r473TF58gyZ82W9NSUBvIWalDeT5MvuUyRaEFH3rN1K4pmh9zkiA
✅ Cached order status
📊 Quick status lookup: {'value': '{"type":"Object","value":{"status":"processing","updated_at":"2026-09-07T21:45:54.527404+00:00"}}'}

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
✓ Created SWR script: fetch_api_user_py (fxUFE9JIn1JbZJs0ic8LgAEi0yDJKckws4qwSUVUfvVC9zmFZRFYq6MTtUhF2mAJ2mt4AMFWtJKeRVZDsG7h5g)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
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
Response time: 2ms (served from cache)
Result: {
  "records": [
    {
      "value": {
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
✓ Created native SWR script: github_user_native (1z1z7w7vKOw2kY3ehSrm5WPK-sdgTEJMu_8Jor8kBMYDuWuYNO-jkDNxhBMZBPkROCIin7rTML77Q2--pXdQoA)

First call (cache miss - will fetch from GitHub API):
  Response time: 11201ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 8ms
  Speedup: 1482.4x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (xZnkPwtbTr5eJDOXe3c6u36rQ_oJKEDQPOXa_vH9fED2-XXII71CSt_958e3ik5u_QEs6ZyIgGZ-7lUGWyRsbQ)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (GNvfJ_xijkUBMoOlSp2l7sJZ0l0WEmMpZCFUosywpIYEFTZX5gs6MLAGbonvvQ8a9nI3sHxjdfc5CJ5o-WQ2Qg)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (SSCjxkh4Rj-0soIhyXJckd6xAvC3subZ95cQQbnyfPwa2LeZ1C7IGQG1ZE05HZLDB9hbtKHyNycxkeOMvfPPNw)
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
   {'avg_price': {'type': 'Float', 'value': 367.0}, 'category': {'type': 'String', 'value': 'Electronics'}, 'count': {'type': 'Integer', 'value': 5}}
   {'avg_price': {'type': 'Float', 'value': 365.6666666666667}, 'category': {'type': 'String', 'value': 'Furniture'}, 'count': {'type': 'Integer', 'value': 3}}
⏱️  Execution time: 0ms

📝 Example 3: Count Total Products

✅ Function saved
📊 Total products: [{'total': {'type': 'Integer', 'value': 8}}]
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
   Vector databases offer several benefits, including:

1. **Efficient Similarity Search**: They enable fast similarity searches using vector representations, making them ideal for applications like recommendation systems and image retrieval.

2. **Handling Unstructured Data**: They efficiently manage and query unstructured data, such as text and images, by converting them into vector embeddings.

3. **Scalability**: Vector databases can scale to handle large datasets and high-dimensional vectors, supporting growing data needs.

4. **Real-time Processing**: Many vector databases are optimized for real-time querying, allowing for quick response times in dynamic environments.

5. **Advanced Machine Learning Integration**: They support machine learning workloads, enabling seamless integration of AI models for tasks like clustering and classification.

6. **Dimensionality Reduction Support**: Vector databases often incorporate techniques for dimensionality reduction, which can improve search efficiency and reduce storage costs.

7. **Multi-modal Data Support**: They can work with different types of data (text, images, audio) by representing them in a unified vector space.

Overall, vector databases enhance performance and capabilities in managing and querying complex data types.
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
   inactive: 3 users
   active: 7 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Function saved
📊 Average score by role:
   {'avg_score': {'type': 'Float', 'value': 70.0}, 'count': {'type': 'Integer', 'value': 7}, 'role': {'type': 'String', 'value': 'user'}}
   {'avg_score': {'type': 'Float', 'value': 20.0}, 'count': {'type': 'Integer', 'value': 3}, 'role': {'type': 'String', 'value': 'admin'}}
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
   1. Database Design Principles (Database)
   2. Natural Language Processing (AI)
   3. Introduction to Machine Learning (AI)
   4. Getting Started with ekoDB (Database)
   5. Vector Databases Explained (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {'category': {'type': 'String', 'value': 'Database'}, 'count': {'type': 'Integer', 'value': 3}}
   {'category': {'type': 'String', 'value': 'AI'}, 'count': {'type': 'Integer', 'value': 2}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: daiZC2QZTBDSnnyM0mBh543J3Ee0r_G5DhBylJLWym6POyg7M3-dT4tow1FkEhyU8qc6hIfOLBx-FI_9u16NIA

=== Sending Chat Message ===
Message ID: Z1ovPwJ2kPbYsyqjsy1VExkeLPHg0k9Cln16ohUx0lVHw03Ab0K6ddqtKWLfaPdLkw9kh-HIsJJuQ4HllzZsKw

=== AI Response ===
The available products and their prices are as follows:

1. **ekoDB Pro**
   - Price: $299
   - Description: Enterprise edition product with advanced features

2. **ekoDB**
   - Price: $99
   - Description: A high-performance database product with AI capabilities

3. **ekoDB Cloud**
   - Price: $499
   - Description: Fully managed cloud database service product

Execution Time: 1761ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 90
Total tokens: 3503

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: F0HuY0ip7tQDKrvJ6MATDEXoovwHKTK1DWMZMCCrguajTcs9RqP-okILyItnUmQ7uVdyDtZzWfdv45vAVcPTfQ

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you need more details or other products, just let me know!

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['content', 'created_at', 'chat_id', 'llm_model', 'id', 'role', 'llm_provider', 'token_usage', 'updated_at', 'context_snippets'])
Debug: First message role: {'type': 'String', 'value': 'assistant'}
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: MsFgaGHuEMqowhdNh55laOo1tJMk2CoF8bjwtylNl90Ei5BOTrKz0kqajiENJs7al67mxOoZizgXgU1_a19jWg
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: nkBJXWlWv5CQEGIuO_iZt14BLDnyv3qqWNQSWIMEdEG76GnvcBXraqm_M40BZLRGqU3AaCGxIVVZv7wwyClb2g

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** A high-performance database product
- **Price:** $99

If you need more information or have any other questions, feel free to ask!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: JZyJ6uwvW4GaYyX1ux4VkCPcIR5NTaqX5wMo1jJepkZvZb6DDC2AuyrZ7fSG-wlPDxpVHRjHYAmkw1Jf_HlEVA
  Parent: nkBJXWlWv5CQEGIuO_iZt14BLDnyv3qqWNQSWIMEdEG76GnvcBXraqm_M40BZLRGqU3AaCGxIVVZv7wwyClb2g

=== Listing Sessions ===
✓ Found 7 sessions
  Session 1: JZyJ6uwvW4GaYyX1ux4VkCPcIR5NTaqX5wMo1jJepkZvZb6DDC2AuyrZ7fSG-wlPDxpVHRjHYAmkw1Jf_HlEVA (Untitled)
  Session 2: nkBJXWlWv5CQEGIuO_iZt14BLDnyv3qqWNQSWIMEdEG76GnvcBXraqm_M40BZLRGqU3AaCGxIVVZv7wwyClb2g (Untitled)
  Session 3: MsFgaGHuEMqowhdNh55laOo1tJMk2CoF8bjwtylNl90Ei5BOTrKz0kqajiENJs7al67mxOoZizgXgU1_a19jWg (Untitled)
  Session 4: daiZC2QZTBDSnnyM0mBh543J3Ee0r_G5DhBylJLWym6POyg7M3-dT4tow1FkEhyU8qc6hIfOLBx-FI_9u16NIA (Untitled)
  Session 5: 1XLUyFhbuCQozWHtbWvx5Jjk1LEyfQxrk2ovJjO7bwll-2gtz50K88C5Zqr1REVwWLTNjHiDTqQmG2I9hmEMLQ (Untitled)
  Session 6: L1UQzFyFR4ccuvwyDYzO0gicZTwcGTyE2eObq41Pxc0Af_hs8haT87q3nMhpH4rHu1SgAXeMe2rlCNCgMNRpWA (Untitled)
  Session 7: EpncQpgraSLlTwxuY9OQk1S-6BHw17FPHrGe1hQSC1k3Y-mKu82KFBbgejEmRy0C6de3NyaqW20Z830tZ6GwRA (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: JZyJ6uwvW4GaYyX1ux4VkCPcIR5NTaqX5wMo1jJepkZvZb6DDC2AuyrZ7fSG-wlPDxpVHRjHYAmkw1Jf_HlEVA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': '-Hf0Shv71kf9kwGjhSzD0YYz-MXTAy-uDnErUXXFqBhOwWhMo74NcZqY89OAvZtevNj0neuV1QYUoD7hZEmXtA'}

=== Upsert Operation ===
✓ Upsert (update existing record): -Hf0Shv71kf9kwGjhSzD0YYz-MXTAy-uDnErUXXFqBhOwWhMo74NcZqY89OAvZtevNj0neuV1QYUoD7hZEmXtA
✓ Inserted second record: L4pf8IKDJi445rF3t_eGy3U8edgfOnQOR58VEkTkhrYEYwSZuPKaGQbMm8kMb40skKkCv32o5NGujYBfXWqFXA
✓ Upsert (update second record): L4pf8IKDJi445rF3t_eGy3U8edgfOnQOR58VEkTkhrYEYwSZuPKaGQbMm8kMb40skKkCv32o5NGujYBfXWqFXA

=== Find One Operation ===
✓ Found user by email: {'email': {'value': 'alice.j@newdomain.com', 'type': 'String'}, 'id': '-Hf0Shv71kf9kwGjhSzD0YYz-MXTAy-uDnErUXXFqBhOwWhMo74NcZqY89OAvZtevNj0neuV1QYUoD7hZEmXtA', 'name': {'type': 'String', 'value': 'Alice Johnson'}, 'age': {'type': 'Integer', 'value': 29}, 'active': {'value': True, 'type': 'Boolean'}}
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
   Inserted with ripple: {'id': 'rz0ZfcMp1ez7DnM4a0le60qXzN4tOw7dQnU909ZGElfgVFHXB10bE21ONzfGqYbEeLvX6u9zfOYLUHiytt8M8g'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'pCXreJ0K1b61UbXxxsYcuZwrwRuQgxjOHw0xEtEd9D4fuu_vAY6r41WJBJVTjp31Y-jfvw_IzaTDTedZkIb4QQ'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'id': 'rz0ZfcMp1ez7DnM4a0le60qXzN4tOw7dQnU909ZGElfgVFHXB10bE21ONzfGqYbEeLvX6u9zfOYLUHiytt8M8g', 'price': {'type': 'Integer', 'value': 150}, 'name': {'type': 'String', 'value': 'Product 1'}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'name': {'type': 'String', 'value': 'Upsert Product'}, 'price': {'value': 500, 'type': 'Integer'}, 'id': 'rz0ZfcMp1ez7DnM4a0le60qXzN4tOw7dQnU909ZGElfgVFHXB10bE21ONzfGqYbEeLvX6u9zfOYLUHiytt8M8g'}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ['email', 'name', 'id']
  First user: {'value': 'Alice Johnson', 'type': 'String'} <{'value': 'alice@example.com', 'type': 'String'}>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ['user_role', 'age', 'created_at', 'avatar_url', 'id', 'bio', 'email', 'name', 'status']

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - {'value': 'Dave Brown', 'type': 'String'} (age {'type': 'Integer', 'value': 45})
    - {'type': 'String', 'value': 'Alice Johnson'} (age {'type': 'Integer', 'value': 30})
    - {'value': 'Bob Smith', 'type': 'String'} (age {'value': 25, 'type': 'Integer'})

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - {'type': 'String', 'value': 'Carol White'}: {'type': 'String', 'value': 'Manager'}

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ['created_at', 'secret_token', 'age', 'status', 'user_role', 'email', 'bio', 'id', 'name', 'avatar_url', 'password', 'api_key']
  Projected query:
    - 3 fields per record
    - Fields: ['email', 'name', 'id']
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
  Float 0.1 + 0.2 = None (should be 0.3)
  Decimal "0.30" = None (exact!)

=== Cleanup ===
✓ Cleaned up test keys

=== Summary ===
✅ Use field_decimal() for monetary values, percentages, and
   any case where floating-point errors are unacceptable.
✅ field_decimal() stores values as strings internally,
   preserving exact precision across all operations.
✓ Client created

=== Get All Chat Models ===
OpenAI models: ['text-embedding-ada-002', 'whisper-1', 'gpt-3.5-turbo', 'tts-1', 'gpt-3.5-turbo-16k', 'gpt-4-0613', 'gpt-4', 'davinci-002', 'babbage-002', 'gpt-3.5-turbo-instruct', 'gpt-3.5-turbo-instruct-0914', 'gpt-3.5-turbo-1106', 'tts-1-hd', 'tts-1-1106', 'tts-1-hd-1106', 'text-embedding-3-small', 'text-embedding-3-large', 'gpt-3.5-turbo-0125', 'gpt-4-turbo', 'gpt-4-turbo-2024-04-09', 'gpt-4o', 'gpt-4o-2024-05-13', 'gpt-4o-mini-2024-07-18', 'gpt-4o-mini', 'gpt-4o-2024-08-06', 'omni-moderation-latest', 'omni-moderation-2024-09-26', 'o1-2024-12-17', 'o1', 'o3-mini', 'o3-mini-2025-01-31', 'gpt-4o-2024-11-20', 'gpt-4o-mini-search-preview-2025-03-11', 'gpt-4o-mini-search-preview', 'gpt-4o-transcribe', 'gpt-4o-mini-transcribe', 'o1-pro-2025-03-19', 'o1-pro', 'gpt-4o-mini-tts', 'o3-2025-04-16', 'o4-mini-2025-04-16', 'o3', 'o4-mini', 'gpt-4.1-2025-04-14', 'gpt-4.1', 'gpt-4.1-mini-2025-04-14', 'gpt-4.1-mini', 'gpt-4.1-nano-2025-04-14', 'gpt-4.1-nano', 'gpt-image-1', 'o4-mini-deep-research', 'gpt-4o-transcribe-diarize', 'o4-mini-deep-research-2025-06-26', 'gpt-5-chat-latest', 'gpt-5-2025-08-07', 'gpt-5', 'gpt-5-mini-2025-08-07', 'gpt-5-mini', 'gpt-5-nano-2025-08-07', 'gpt-5-nano', 'gpt-audio-2025-08-28', 'gpt-realtime', 'gpt-realtime-2025-08-28', 'gpt-audio', 'gpt-5-codex', 'gpt-image-1-mini', 'gpt-5-pro-2025-10-06', 'gpt-5-pro', 'gpt-audio-mini', 'gpt-audio-mini-2025-10-06', 'gpt-5-search-api', 'gpt-realtime-mini', 'sora-2', 'sora-2-pro', 'gpt-5-search-api-2025-10-14', 'gpt-5.1-chat-latest', 'gpt-5.1-2025-11-13', 'gpt-5.1', 'gpt-5.1-codex', 'gpt-5.1-codex-mini', 'gpt-5.1-codex-max', 'gpt-image-1.5', 'gpt-5.2-2025-12-11', 'gpt-5.2', 'gpt-5.2-pro-2025-12-11', 'gpt-5.2-pro', 'gpt-5.2-chat-latest', 'gpt-4o-mini-transcribe-2025-12-15', 'gpt-4o-mini-transcribe-2025-03-20', 'gpt-4o-mini-tts-2025-03-20', 'gpt-4o-mini-tts-2025-12-15', 'gpt-realtime-mini-2025-12-15', 'gpt-audio-mini-2025-12-15', 'chatgpt-image-latest', 'gpt-5.2-codex', 'gpt-5.3-codex', 'gpt-realtime-1.5', 'gpt-audio-1.5', 'gpt-4o-search-preview', 'gpt-4o-search-preview-2025-03-11', 'gpt-5.3-chat-latest', 'gpt-5.4-2026-03-05', 'gpt-5.4-pro', 'gpt-5.4-pro-2026-03-05', 'gpt-5.4', 'gpt-5.4-nano-2026-03-17', 'gpt-5.4-nano', 'gpt-5.4-mini-2026-03-17', 'gpt-5.4-mini', 'gpt-image-2', 'gpt-image-2-2026-04-21', 'gpt-5.5', 'gpt-5.5-2026-04-23', 'gpt-5.5-pro', 'gpt-5.5-pro-2026-04-23', 'chat-latest', 'gpt-realtime-translate', 'gpt-realtime-2', 'gpt-realtime-whisper', 'gpt-5.6-sol', 'gpt-5.6-terra', 'gpt-5.6-luna', 'gpt-realtime-2.1', 'gpt-realtime-2.1-mini', 'gpt-transcribe', 'gpt-live-transcribe', 'gpt-6-astra']
Anthropic models: ['claude-fable-5-1', 'claude-opus-5', 'claude-sonnet-5', 'claude-fable-5', 'claude-opus-4-8', 'claude-opus-4-7', 'claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929']
Perplexity models: []
Gemini models: []

=== Get OpenAI Models ===
OpenAI models: ['text-embedding-ada-002', 'whisper-1', 'gpt-3.5-turbo', 'tts-1', 'gpt-3.5-turbo-16k', 'gpt-4-0613', 'gpt-4', 'davinci-002', 'babbage-002', 'gpt-3.5-turbo-instruct', 'gpt-3.5-turbo-instruct-0914', 'gpt-3.5-turbo-1106', 'tts-1-hd', 'tts-1-1106', 'tts-1-hd-1106', 'text-embedding-3-small', 'text-embedding-3-large', 'gpt-3.5-turbo-0125', 'gpt-4-turbo', 'gpt-4-turbo-2024-04-09', 'gpt-4o', 'gpt-4o-2024-05-13', 'gpt-4o-mini-2024-07-18', 'gpt-4o-mini', 'gpt-4o-2024-08-06', 'omni-moderation-latest', 'omni-moderation-2024-09-26', 'o1-2024-12-17', 'o1', 'o3-mini', 'o3-mini-2025-01-31', 'gpt-4o-2024-11-20', 'gpt-4o-mini-search-preview-2025-03-11', 'gpt-4o-mini-search-preview', 'gpt-4o-transcribe', 'gpt-4o-mini-transcribe', 'o1-pro-2025-03-19', 'o1-pro', 'gpt-4o-mini-tts', 'o3-2025-04-16', 'o4-mini-2025-04-16', 'o3', 'o4-mini', 'gpt-4.1-2025-04-14', 'gpt-4.1', 'gpt-4.1-mini-2025-04-14', 'gpt-4.1-mini', 'gpt-4.1-nano-2025-04-14', 'gpt-4.1-nano', 'gpt-image-1', 'o4-mini-deep-research', 'gpt-4o-transcribe-diarize', 'o4-mini-deep-research-2025-06-26', 'gpt-5-chat-latest', 'gpt-5-2025-08-07', 'gpt-5', 'gpt-5-mini-2025-08-07', 'gpt-5-mini', 'gpt-5-nano-2025-08-07', 'gpt-5-nano', 'gpt-audio-2025-08-28', 'gpt-realtime', 'gpt-realtime-2025-08-28', 'gpt-audio', 'gpt-5-codex', 'gpt-image-1-mini', 'gpt-5-pro-2025-10-06', 'gpt-5-pro', 'gpt-audio-mini', 'gpt-audio-mini-2025-10-06', 'gpt-5-search-api', 'gpt-realtime-mini', 'sora-2', 'sora-2-pro', 'gpt-5-search-api-2025-10-14', 'gpt-5.1-chat-latest', 'gpt-5.1-2025-11-13', 'gpt-5.1', 'gpt-5.1-codex', 'gpt-5.1-codex-mini', 'gpt-5.1-codex-max', 'gpt-image-1.5', 'gpt-5.2-2025-12-11', 'gpt-5.2', 'gpt-5.2-pro-2025-12-11', 'gpt-5.2-pro', 'gpt-5.2-chat-latest', 'gpt-4o-mini-transcribe-2025-12-15', 'gpt-4o-mini-transcribe-2025-03-20', 'gpt-4o-mini-tts-2025-03-20', 'gpt-4o-mini-tts-2025-12-15', 'gpt-realtime-mini-2025-12-15', 'gpt-audio-mini-2025-12-15', 'chatgpt-image-latest', 'gpt-5.2-codex', 'gpt-5.3-codex', 'gpt-realtime-1.5', 'gpt-audio-1.5', 'gpt-4o-search-preview', 'gpt-4o-search-preview-2025-03-11', 'gpt-5.3-chat-latest', 'gpt-5.4-2026-03-05', 'gpt-5.4-pro', 'gpt-5.4-pro-2026-03-05', 'gpt-5.4', 'gpt-5.4-nano-2026-03-17', 'gpt-5.4-nano', 'gpt-5.4-mini-2026-03-17', 'gpt-5.4-mini', 'gpt-image-2', 'gpt-image-2-2026-04-21', 'gpt-5.5', 'gpt-5.5-2026-04-23', 'gpt-5.5-pro', 'gpt-5.5-pro-2026-04-23', 'chat-latest', 'gpt-realtime-translate', 'gpt-realtime-2', 'gpt-realtime-whisper', 'gpt-5.6-sol', 'gpt-5.6-terra', 'gpt-5.6-luna', 'gpt-realtime-2.1', 'gpt-realtime-2.1-mini', 'gpt-transcribe', 'gpt-live-transcribe', 'gpt-6-astra']

=== Get Anthropic Models ===
Anthropic models: ['claude-fable-5-1', 'claude-opus-5', 'claude-sonnet-5', 'claude-fable-5', 'claude-opus-4-8', 'claude-opus-4-7', 'claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929']

✓ Chat Models API example complete
✓ Client created

=== Create User Function ===
Created user function with ID: re4FI3SolAfgZ3bliXAqK-R8mc-t2oJ7FsJN1kpz2iIeCrdH5P0R8tc90RDx0_5buUDH1A_dRTaO-6hyjjTbdA

=== Get User Function ===
Retrieved: get_active_users_py - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 14 user functions:
  - get_active_users_updated: Get Active Users (Updated)
  - validate_user: Check if user exists
  - get_users_by_status: Get Users By Status
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_slim_user: Validate and slim down user
  - cache_api_call_py: Cache External API Call
  - fetch_user: Fetch user by code
  - get_user_wrapper: Wrapper that calls fetch_user
  - get_high_scoring_active_users: Get High Scoring Active Users
  - get_verified_user: Get verified and validated user
  - get_active_users_py: Get Active Users
  - swr_user: SWR pattern for user data (KV-based)
  - get_active_users_updated: Get Active Users (Updated)
  - get_active_users_paginated: Get Active Users (Paginated)

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
✓ Client created
✓ py_users_register saved
✓ py_users_login saved
✓ py_users_verify_token saved

=== Auth flow defined as pure stored functions ===
Call them like:
  POST /api/functions/py_users_register { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/py_users_login    { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/py_users_verify_token { "token": "<jwt>" }

✓ Cleaned up demo functions
✓ Client created
✓ crypto_demo_hmac saved
✓ crypto_demo_aes saved
✓ crypto_demo_uuid saved
✓ crypto_demo_totp saved
✓ crypto_demo_encoding saved

Invoke them with:
  POST /api/functions/crypto_demo_hmac     { "payload": "hi" }
  POST /api/functions/crypto_demo_aes      { "plaintext": "secret" }
  POST /api/functions/crypto_demo_uuid
  POST /api/functions/crypto_demo_totp
  POST /api/functions/crypto_demo_encoding { "title": "Héllo World" }

✓ Cleaned up demo functions
✓ Client created
✓ conc_demo_pay saved
✓ conc_demo_rl_fail saved
✓ conc_demo_rl_skip saved
✓ conc_demo_lock saved

Invoke them like:
  POST /api/functions/conc_demo_pay        { "idempotency_key": "...", "amount": 100 }
  POST /api/functions/conc_demo_rl_fail    { "user_id": 42 }
  POST /api/functions/conc_demo_rl_skip    { "user_id": 42 }
  POST /api/functions/conc_demo_lock       { "resource": "queue:drain" }

✓ Cleaned up demo functions
✓ Client created
✓ py_route_admin saved
✓ py_route_user_by_id saved
✓ py_route_user_posts saved
✓ py_route_org_create_member saved

Try them with curl:
  curl http://localhost:8080/api/route/users/admin
  curl http://localhost:8080/api/route/users/42
  curl http://localhost:8080/api/route/users/42/posts/7
  curl -X POST http://localhost:8080/api/route/orgs/acme/members \
       -H 'Content-Type: application/json' -d '{"name":"alice"}'

✓ Cleaned up demo functions
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
Inserted: map[id:0GvgMc3lj9I55AgcZEghmUSNC3X1PnQLAOCXOwqDx2ybz4z6DQNRbMW2-A2q6DkliJVS7GQIeBO6zdS1IGR2aw]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:0GvgMc3lj9I55AgcZEghmUSNC3X1PnQLAOCXOwqDx2ybz4z6DQNRbMW2-A2q6DkliJVS7GQIeBO6zdS1IGR2aw name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found documents: [map[active:map[type:Boolean value:true] id:0GvgMc3lj9I55AgcZEghmUSNC3X1PnQLAOCXOwqDx2ybz4z6DQNRbMW2-A2q6DkliJVS7GQIeBO6zdS1IGR2aw name:map[type:String value:Test Record] value:map[type:Integer value:42]]]

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:0GvgMc3lj9I55AgcZEghmUSNC3X1PnQLAOCXOwqDx2ybz4z6DQNRbMW2-A2q6DkliJVS7GQIeBO6zdS1IGR2aw name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: myzh8ozWbo5uFwQiqnAkokhilfiGHiMexcrtOt5iGq6OK6Nf_x7KPR6M7rjnYzTUbCq1xZf2P8lSNWf4D7FC5Q

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1788817591210923000",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "myzh8ozWbo5uFwQiqnAkokhilfiGHiMexcrtOt5iGq6OK6Nf_x7KPR6M7rjnYzTUbCq1xZf2P8lSNWf4D7FC5Q",
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
        "id": "eo4k2XKeeP4Jw-NIruzkp5BZ37laWem-xan_dukdvdrmZzoejTtyeo5p4ZQXOFLyMu6bG4acNriPRHzHTq0EoQ",
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
        "id": "WsbRCK94m4hP3hqb2DP9BPADED5Qb3sY8pyagkVQFOgdbEuIFn84mi1fBQkRrv_aJlCNvJp3vQhqcdHXQRFFCQ",
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

✅ Function saved: Ce-EnLPtzYY9Xiz72eI4a0AUMmjcvTeYjpiStghkKVmqqwuPIqkMJhBwjgq7ra9dtQYW3zXjDLefj57z33holQ
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

ℹ️  Function 'get_active_users_paginated' already existed — updated instead
✅ Function saved: MeQV3_i3yzwXweQIO_tVmxFSYhdfxQrE4co4qvoEmX73e3sZaQ_N8t00rdbORwOu1zfEPKgkJsDbQLOzusQhOg
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: s9rJfRt58WlwK7ytbJc9TNYLhsikOkLDlIVFMMiPEE3NPIsb2zA33Evxrm3ddee34AbZ09dzTo1GWYvzkRff1A
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":{"type":"Float","value":50},"count":{"type":"Integer","value":10},"max_score":{"type":"Integer","value":90},"status":{"type":"String","value":"inactive"}}
   {"avg_score":{"type":"Float","value":60},"count":{"type":"Integer","value":10},"max_score":{"type":"Integer","value":100},"status":{"type":"String","value":"active"}}

📝 Example 4: Function Management

📋 Total functions: 15
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
Retrieved value: map[type:Object value:map[userId:123 username:john_doe]]

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: map[type:Object value:map[name:Product 1 price:29.99]]
cache:product:2: map[type:Object value:map[name:Product 2 price:39.989999999999995]]
cache:product:3: map[type:Object value:map[name:Product 3 price:49.989999999999995]]

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
Collection created with first record: BzVLve3AuyusluMmd9hgzAIFMTaUdFAaclJ7k9iW4JybHhDfrXjqkLRWpyD9ooAPv-9KRy6gpYxK9hF0zlOIHQ

=== List Collections ===
Total collections: 19
Sample collections: [ttl_cache test_collection websocket_test schema_products_client_py demo_collection]

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
Created Alice: $1000 - ID: IO0j5iLJfCSyM5DKyGbp-p_1zj5FjV8d6nl1kg8b7qHZBU2oAkrUXshqEJAJvxyJwDr7clPiNbbUGhdHIdkC2Q
Created Bob: $500 - ID: aQ1XvCLpx4UKPAR3pOpgjTU03GKn1f-n_629caibO5C_012zv3UXqTBjyafjLrEiXDluUld_7p7RzVaz-YRTtg

=== Example 1: Begin Transaction ===
Transaction ID: d4c3755e-783a-4393-9d2e-faa0c52eff2e

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
New transaction: 30c64ebc-09e5-4cc4-b993-e7deef958046
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
   ✅ Function saved: _EPvKgtlTNW_LPmuyytzB7P39Gh1oqBF_k3_R5T6UELEHly-fwPqIIB_A8DGknDCaQtqyiG0du2ru9PcB5eF2g

2️⃣ Calling function (Insert + Verify)...
   ✅ function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: j9q-_YjDtbJtUXPGBKKMzE1oD5bmyV4OBD_7lw3qDFZZ0rBZOZ4u9PhDX2U2Si3C_f2VtpcbBSV1d13Gz7x_Ng
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: BqElK0WxN1ujp4ZTfGmL1OQjqjfOW3orjXFU-2BClQDVCQsfjDoaBX6LVJbIlP2GVa6VpyFK7xu5N01xYJi80w

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
   ✅ Function saved: jenXRA1TQURHDZSkE8JRvO9bw2Yd4hwnez7vzjebrmik77XGvhyjQBtoTF1LDHvfdLcplpBYXNn7dathSVK4TA

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
   ✅ Function saved: 1nVXthIXRtG_IV_6gdDRd4jJNKLgylXAUcklsuUnh8kuQMGSI_Nlk-7-ZRNcc_JGjF96PNvi0iIanBeHPvYtAQ

2️⃣ Calling function (Query + Delete + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: _EPvKgtlTNW_LPmuyytz...
   ✅ Deleted script: BqElK0WxN1ujp4ZTfGmL...
   ✅ Deleted script: jenXRA1TQURHDZSkE8JR...
   ✅ Deleted script: 1nVXthIXRtG_IV_6gdDR...
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
  Output: Document ID = nCPlmcJmY2Hm3czFt-BEIhN7nHjthx5XG4QkSj3_llQ9f18jLMMwXSWb9sdpGF8G-j0cfRICRO_ixO8PvBLMBw
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: FindByID(nCPlmcJmY2Hm3czFt-BEIhN7nHjthx5XG4QkSj3_llQ9f18jLMMwXSWb9sdpGF8G-j0cfRICRO_ixO8PvBLMBw)
  Output: Found document with name = map[type:String value:TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: FindByID(nCPlmcJmY2Hm3czFt-BEIhN7nHjthx5XG4QkSj3_llQ9f18jLMMwXSWb9sdpGF8G-j0cfRICRO_ixO8PvBLMBw)
  Output: Error (expected) - request failed with status 404: ��error�Record not found (expired)
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
  Output: Document ID = 4rp7CAj3xCg4OJyGcKZkLBEco6j2_zj56X4RNmSt6Vff_vkcua4aiQNvxyt8FY0pEZAYyGpuxn96hdKNlBOrgQ
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: FindByID(4rp7CAj3xCg4OJyGcKZkLBEco6j2_zj56X4RNmSt6Vff_vkcua4aiQNvxyt8FY0pEZAYyGpuxn96hdKNlBOrgQ)
  Output: Found document with name = map[type:String value:WS TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: FindByID(4rp7CAj3xCg4OJyGcKZkLBEco6j2_zj56X4RNmSt6Vff_vkcua4aiQNvxyt8FY0pEZAYyGpuxn96hdKNlBOrgQ)
  Output: Error (expected) - request failed with status 404: ��error�Record not found (expired)
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
Inserted: map[id:h-RJ0MUGYOxXC-5YXke8MZ-vEfzJl8nSXI0C1X0B8d1Rl3l1BC3BLb0RxCiZULvCcocfNGxctxsXS8xFPZGmSg]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-09-07T21:47:04+00:00] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:h-RJ0MUGYOxXC-5YXke8MZ-vEfzJl8nSXI0C1X0B8d1Rl3l1BC3BLb0RxCiZULvCcocfNGxctxsXS8xFPZGmSg metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Test Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:42]]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-09-07 21:47:04 +0000 +0000
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-09-07T21:47:04+00:00 data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:h-RJ0MUGYOxXC-5YXke8MZ-vEfzJl8nSXI0C1X0B8d1Rl3l1BC3BLb0RxCiZULvCcocfNGxctxsXS8xFPZGmSg metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-09-07T21:47:04+00:00] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:h-RJ0MUGYOxXC-5YXke8MZ-vEfzJl8nSXI0C1X0B8d1Rl3l1BC3BLb0RxCiZULvCcocfNGxctxsXS8xFPZGmSg metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: 2HXV4IUZziUnOmr4hcNsFTRS1FppAA4N7-wSkY-DNpj8hcPF-QlCJV72Z0zMcbtYdnIeN68iIpeTrClcfc0rtA

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
Collection created with first record: TQKHHx0Pl9fysXDk0wCF4L4TeDnRcZ2k4b5Bx25LUsviUPkJN2eLApyvyWA8ZMKIHtCJRvFFrjhsh0hpg6MUaw

=== List Collections ===
Total collections: 18
Sample collections: [ttl_cache test_collection websocket_test schema_products_client_py chat_configurations__ek0_testing]

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
Retrieved value: map[type:Object value:map[userId:123 username:john_doe]]

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
Created Alice: $1000 - ID: lIuxYI6SOH6KsRIUi5ll30-qVonVHr9jIxPPpHrmfA4cBPqKVfwTHFvTr9HdZGsugvx6o2WSB86hmVuZTrWQ4w
Created Bob: $500 - ID: bV_WbgytEPFP3Sv_WU1FinrgESeLG0W4PJaGXWlW6vvHWFbULHMbo3L4h5b6XmZYtocMHH-PnGMs_8x--RqIVA

=== Example 1: Begin Transaction ===
Transaction ID: cc44823e-8ff4-412d-8955-e56f6115f9d5

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: afc35f98-f12b-4869-881e-696b7b37a96e
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
  1. Score: 0.754
  2. Score: 0.739
  3. Score: 0.719

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.501
  2. Score: 0.895
  3. Score: 0.288

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200

9. Vector search with a metadata pre-filter (category = ml):
Found 2 documents in category "ml" (NLP excluded)
  1. Introduction to Machine Learning (category: ml)
  2. Deep Learning Fundamentals (category: ml)

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
✓ Inserted document: r5ZFhYbaeHmWEkNKk98fQoZuod0U90jlIDuo4B3BRQKc1MMMvJAPQbLlOTcnvgP8LIsjD6A1HN3BXSJwf8dH9Q

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: fE07bdVlPlZMr4Rl6W08_y_53OgpKm9otNRqNJpR-jfcmb3duqHYjdvQt8uGep8wcOBfnoJrZEvhH9TqbUTQcg

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
✓ Inserted document with TTL: 7Gt2xWuQqzkRpC7h6LRzY41aPQUmBWDc1hnGZGJoNu1Su0vZPva2EM-my9ysXP55yP0iI_pUfVj73ZtolhsrRA

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
✓ Edge cache script created: S0FAXVkJ3vJsJxKA75znUfxxpvMZ4BJ9doAiRCYnXV7Zr-WEbnk4wRsCPxAmvkbtYPkBiW65C4ZshtFL6xk-fw

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
🚀 ekoDB Functions Example (Go Client)

✅ Client initialized

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

ℹ️  Function 'get_active_users' already existed — updated instead
✅ Function saved: ozLy4wDDOaGm7D_Rkmp-JE_BigNaZbLbtHE_ibFqSCj_FJ6IbpJWCnmJwwIVM5HocFVE_6uAaj99xwfA35boDg
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

ℹ️  Function 'get_users_by_status' already existed — updated instead
✅ Function saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 2 groups
   map[avg_score:map[type:Float value:50] count:map[type:Integer value:5] status:map[type:String value:inactive]]
   map[avg_score:map[type:Float value:60] count:map[type:Integer value:5] status:map[type:String value:active]]
⏱️  Execution time: 0ms

📝 Example 4: Function Management

📋 Total functions: 15
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
📊 Total user count: 0
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

Function 'fetch_user' already existed — updated instead
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

Function 'fetch_and_store_user' already existed — updated instead
✅ Saved reusable function: fetch_and_store_user (uses KV)
Function 'swr_user' already existed — updated instead
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 2.072583ms
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
   ⏱️  Duration: 1.984208ms
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
   🚀 Cache speedup: 2.0x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

Function 'validate_user' already existed — updated instead
✅ Level 1 function: validate_user
Function 'fetch_slim_user' already existed — updated instead
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

✅ Function saved: OKBZ_BA6SvytuLqaRDrd9IwUNVuAyVn-w5aiZnDr4L0Yo7yuG26KdvHWdzY6isSBf-IpmLmwS1ijb2UIPgklmQ
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

✅ Inserted order: Z8AHZiYQTU-LE_6gEjEXEp17l2l5sKR1N4bFU9K7iMtX5APeA0_stpP8EkV5217i8pusJVnYvQ46YATFAmFKRg
✅ Inserted 2 products with wrapped types

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: _0hsfJhH3zhzujr-IKdifwBMveuISKCL31BM0ujYVMpWXM_PCSdrvC8EwENRg1yB3Ou4tTmdXty12Se_fWYO4A
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: map[type:Object value:map[role:admin userId:user_abc]]
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session
📝 Example 4: KV Operations in Functions

✅ Function saved: uN0_NYaRSTxy89ycWQrF1sED3M8e36M4NN4opwdzyILevwwZjubCaRZd_F1qzbT7hy_zxQ-rA3YrCXsIn83UsQ
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: G--GNISCc9juJd95XMp5xSA9DHhsiz5tFSVPwnoYZOUFjfZBvsdONgQJvFFErKtsNn2j5SwX-idLkXR63nDakA
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
✓ Created SWR script: fetch_api_user_go (_wdSfTdSIvqK3RKW8I53XIDo_fdhI6kE5GxI8A-bo3uzX6ZO0sH15l9nFVzpatLstJHEfBm4X1PuzqegAS8M1Q)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "cached_at": {
        "type": "DateTime",
        "value": "2026-09-07T21:47:14+00:00"
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
Response time: 2ms (served from cache)
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
✓ Created native SWR script: github_user_native (g4vzbZi-Z1jh7Wf85sv9nrBXBZdrL4W3_1XCMf0t3vNVpkNoO2jB-26O_V_J3cokBP-gD5uBmhG0HGxdne6iWA)

First call (cache miss - will fetch from GitHub API):
  Response time: 5ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 5ms
  Speedup: 1.0x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (Xm58tGu9YLlIayjDhdhotU7SuO_v0MsfrLFQorNRxMMsSu8_fRdEHnHk9ONOtRcpUGiCz79uliRHohLH6s9VLA)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (C2r8cxfWnSRTNVQummLBO4znL-G2uSi_Po2qa9uByPnWksQwJ75oO-_AdXAjKbNV0A3tRJp2RQ0UAFvxCSNyxg)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (1j3rgJIqoKGW47QZogk_hPC0X3YgZ4Dn769Px482eIJjB8mP2gsdPmQ9-3aRJZS2u3dIAS1aOJSkaUqM8zKrLA)
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
   map[avg_price:map[type:Float value:365.6666666666667] category:map[type:String value:Furniture] count:map[type:Integer value:3]]
   map[avg_price:map[type:Float value:367] category:map[type:String value:Electronics] count:map[type:Integer value:5]]
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
   map[count:map[type:Integer value:7] status:map[type:String value:active]]
   map[count:map[type:Integer value:3] status:map[type:String value:inactive]]
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
   1. map[type:String value:Vector Databases Explained] (map[type:String value:Database])
   2. map[type:String value:Introduction to Machine Learning] (map[type:String value:AI])
   3. map[type:String value:Getting Started with ekoDB] (map[type:String value:Database])
   4. map[type:String value:Natural Language Processing] (map[type:String value:AI])
   5. map[type:String value:Database Design Principles] (map[type:String value:Database])
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   map[category:map[type:String value:AI] count:map[type:Integer value:2]]
   map[category:map[type:String value:Database] count:map[type:Integer value:3]]
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: EGq4BpwScMdsQDiBfksclFl8625dagxm3gVyxIyXJKhHxHZyinKJiOelY97gqIKri4o4BnJj2GVPWG8XuixU0Q

=== Sending Chat Message ===
Message ID: mbZtWJSaQkAZtYIXT_cYMeGBT4fLpXu2z5YS_tXrRGvcp9DxMjybJWf-vCGtkbGbNdxIMhIINjjmK1vBfPiNwg

=== AI Response ===
Here are the available products and their prices:

1. **ekoDB Pro**
   - Description: Enterprise edition product with advanced features
   - Price: $299

2. **ekoDB Cloud**
   - Description: Fully managed cloud database service product
   - Price: $499

3. **ekoDB**
   - Description: A high-performance database product with AI capabilities
   - Price: $99

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:AXrK8NWL4g_oSpQjpiOqy1yF1NChT1W9jKmHuP4HRq4DqTmCvzxgRbpBiyuW1-KQP73BQJ92W0vXSolzIpFH5w name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:QVp5UFRjgksMU81w0-_VgOz-SD0vlQ7dG95uBC6E5ALXZa96TuyY-YS-HHtErZRBLtgqKqUnDlgm1cgc5YsPww name:ekoDB Cloud price:499] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:96bHVMXS7wMfahxtbbUSOFQtRtnHfl7WrEX9ugrbPUYc_qqShYF6n19Cetl1TuS7EgQSd0EMJ__biu4RdB0uxA name:ekoDB price:99] score:0.1111111111111111]

Execution Time: 2083ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 89
Total tokens: 3502

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: _2H2yeGPCd7-YtnDyN6f0V20uICvJdTCwMuwNeYap0nJRTa5cLgNcAcdYaCldl1N50nVf-289J258QMDs7Y8wg

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you need more information or have further questions, feel free to ask!

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you need any more information, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: Kh_2y1eqgZka3uqUF1ti21UPDhrWr12DBbgPg3rsoS8AX9-4AVxYHe8VZ1pIwPbRRRaEA6LYfPvIteEXjW8_yQ
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 5hi3bOxBbTb2WcHkXfr7bCB4mS9f44BEJ-DAXVsqrF96OST0diI2NxRenbSDb4MEKRC9FbQwtn6_hP_p7xTt4w

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** A high-performance database product
- **Price:** $99

If you need more information or have any other questions, feel free to ask!

✓ Message 2 sent
  Response: The price of the ekoDB product is **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 7zdpWsP6f4aUa10hNP87v9yWu5_opG6xiaCFryc82tWO-V2d3SYDZ-AVAwm6eMG42TeyqpRng19IRtAQe_OUmw
  Parent: 5hi3bOxBbTb2WcHkXfr7bCB4mS9f44BEJ-DAXVsqrF96OST0diI2NxRenbSDb4MEKRC9FbQwtn6_hP_p7xTt4w

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: 7zdpWsP6f4aUa10hNP87v9yWu5_opG6xiaCFryc82tWO-V2d3SYDZ-AVAwm6eMG42TeyqpRng19IRtAQe_OUmw (Untitled)
  Session 2: 5hi3bOxBbTb2WcHkXfr7bCB4mS9f44BEJ-DAXVsqrF96OST0diI2NxRenbSDb4MEKRC9FbQwtn6_hP_p7xTt4w (Untitled)
  Session 3: Kh_2y1eqgZka3uqUF1ti21UPDhrWr12DBbgPg3rsoS8AX9-4AVxYHe8VZ1pIwPbRRRaEA6LYfPvIteEXjW8_yQ (Untitled)
  Session 4: EGq4BpwScMdsQDiBfksclFl8625dagxm3gVyxIyXJKhHxHZyinKJiOelY97gqIKri4o4BnJj2GVPWG8XuixU0Q (Untitled)
  Session 5: nkBJXWlWv5CQEGIuO_iZt14BLDnyv3qqWNQSWIMEdEG76GnvcBXraqm_M40BZLRGqU3AaCGxIVVZv7wwyClb2g (Untitled)
  Session 6: MsFgaGHuEMqowhdNh55laOo1tJMk2CoF8bjwtylNl90Ei5BOTrKz0kqajiENJs7al67mxOoZizgXgU1_a19jWg (Untitled)
  Session 7: daiZC2QZTBDSnnyM0mBh543J3Ee0r_G5DhBylJLWym6POyg7M3-dT4tow1FkEhyU8qc6hIfOLBx-FI_9u16NIA (Untitled)
  Session 8: 1XLUyFhbuCQozWHtbWvx5Jjk1LEyfQxrk2ovJjO7bwll-2gtz50K88C5Zqr1REVwWLTNjHiDTqQmG2I9hmEMLQ (Untitled)
  Session 9: L1UQzFyFR4ccuvwyDYzO0gicZTwcGTyE2eObq41Pxc0Af_hs8haT87q3nMhpH4rHu1SgAXeMe2rlCNCgMNRpWA (Untitled)
  Session 10: EpncQpgraSLlTwxuY9OQk1S-6BHw17FPHrGe1hQSC1k3Y-mKu82KFBbgejEmRy0C6de3NyaqW20Z830tZ6GwRA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 7zdpWsP6f4aUa10hNP87v9yWu5_opG6xiaCFryc82tWO-V2d3SYDZ-AVAwm6eMG42TeyqpRng19IRtAQe_OUmw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Map Creation ===
✓ Created record with native map: map[id:g2dYWtzchw0RezQWNayrytyMoNGAOj4TXSzD6U_FpSedmf7cuIBAp72DjhWopb2n-MoNhZyWDYhJrhvLd6vr8Q]

=== Upsert Operation ===
✓ First upsert (update): map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:g2dYWtzchw0RezQWNayrytyMoNGAOj4TXSzD6U_FpSedmf7cuIBAp72DjhWopb2n-MoNhZyWDYhJrhvLd6vr8Q name:map[type:String value:Alice Johnson]]
✓ Second upsert (insert): map[id:new-user-id]

=== Find One Operation ===
✓ Found user by email: map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:g2dYWtzchw0RezQWNayrytyMoNGAOj4TXSzD6U_FpSedmf7cuIBAp72DjhWopb2n-MoNhZyWDYhJrhvLd6vr8Q name:map[type:String value:Alice Johnson]]
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
Inserted with ripple: map[id:lmWdR3sjMfPyUaV-zYBN9FaBipFGC3-w2kbDNiZxS-Oht0e3Stx9__jkRBkuvcSUX0uhA11iB2q77pI0Onwoxw]
Inserted with bypass_ripple: map[id:fYohOiopLUQL75aoOZw-6UTpAE56TZLGq4z9Hm32N5APr8b1whVlmkWlT_y39OLcvgF80-Q4jbvrTShwE6Bp6A]
Inserted with TTL and bypass_ripple: map[id:k2qqMcsszy4AgA2eERwY-V-72cIsaDp-dU-M9LBxuwRHVrMD-kTxcxszWN0OWqVryrZUBIkfb6nyzt7DFPI4dg]
Updated with bypass_ripple: map[id:lmWdR3sjMfPyUaV-zYBN9FaBipFGC3-w2kbDNiZxS-Oht0e3Stx9__jkRBkuvcSUX0uhA11iB2q77pI0Onwoxw name:map[type:String value:Product 1] price:map[type:Integer value:150]]
Deleted with bypass_ripple
Batch inserted with bypass_ripple: 2 records
Upserted with bypass_ripple: map[id:custom-id]
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: [email id name]
  First user: Bob Smith <bob@example.com>

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
  Float 0.1 + 0.2 = 0.00000000000000000 (should be 0.3)
  Decimal "0.30" =  (exact!)

=== Cleanup ===
✓ Cleaned up test keys

=== Summary ===
✅ Use FieldDecimal() for monetary values, percentages, and
   any case where floating-point errors are unacceptable.
✅ FieldDecimal() stores values as strings internally,
   preserving exact precision across all operations.
✓ Client created

=== Get All Chat Models ===
OpenAI models: [text-embedding-ada-002 whisper-1 gpt-3.5-turbo tts-1 gpt-3.5-turbo-16k gpt-4-0613 gpt-4 davinci-002 babbage-002 gpt-3.5-turbo-instruct gpt-3.5-turbo-instruct-0914 gpt-3.5-turbo-1106 tts-1-hd tts-1-1106 tts-1-hd-1106 text-embedding-3-small text-embedding-3-large gpt-3.5-turbo-0125 gpt-4-turbo gpt-4-turbo-2024-04-09 gpt-4o gpt-4o-2024-05-13 gpt-4o-mini-2024-07-18 gpt-4o-mini gpt-4o-2024-08-06 omni-moderation-latest omni-moderation-2024-09-26 o1-2024-12-17 o1 o3-mini o3-mini-2025-01-31 gpt-4o-2024-11-20 gpt-4o-mini-search-preview-2025-03-11 gpt-4o-mini-search-preview gpt-4o-transcribe gpt-4o-mini-transcribe o1-pro-2025-03-19 o1-pro gpt-4o-mini-tts o3-2025-04-16 o4-mini-2025-04-16 o3 o4-mini gpt-4.1-2025-04-14 gpt-4.1 gpt-4.1-mini-2025-04-14 gpt-4.1-mini gpt-4.1-nano-2025-04-14 gpt-4.1-nano gpt-image-1 o4-mini-deep-research gpt-4o-transcribe-diarize o4-mini-deep-research-2025-06-26 gpt-5-chat-latest gpt-5-2025-08-07 gpt-5 gpt-5-mini-2025-08-07 gpt-5-mini gpt-5-nano-2025-08-07 gpt-5-nano gpt-audio-2025-08-28 gpt-realtime gpt-realtime-2025-08-28 gpt-audio gpt-5-codex gpt-image-1-mini gpt-5-pro-2025-10-06 gpt-5-pro gpt-audio-mini gpt-audio-mini-2025-10-06 gpt-5-search-api gpt-realtime-mini sora-2 sora-2-pro gpt-5-search-api-2025-10-14 gpt-5.1-chat-latest gpt-5.1-2025-11-13 gpt-5.1 gpt-5.1-codex gpt-5.1-codex-mini gpt-5.1-codex-max gpt-image-1.5 gpt-5.2-2025-12-11 gpt-5.2 gpt-5.2-pro-2025-12-11 gpt-5.2-pro gpt-5.2-chat-latest gpt-4o-mini-transcribe-2025-12-15 gpt-4o-mini-transcribe-2025-03-20 gpt-4o-mini-tts-2025-03-20 gpt-4o-mini-tts-2025-12-15 gpt-realtime-mini-2025-12-15 gpt-audio-mini-2025-12-15 chatgpt-image-latest gpt-5.2-codex gpt-5.3-codex gpt-realtime-1.5 gpt-audio-1.5 gpt-4o-search-preview gpt-4o-search-preview-2025-03-11 gpt-5.3-chat-latest gpt-5.4-2026-03-05 gpt-5.4-pro gpt-5.4-pro-2026-03-05 gpt-5.4 gpt-5.4-nano-2026-03-17 gpt-5.4-nano gpt-5.4-mini-2026-03-17 gpt-5.4-mini gpt-image-2 gpt-image-2-2026-04-21 gpt-5.5 gpt-5.5-2026-04-23 gpt-5.5-pro gpt-5.5-pro-2026-04-23 chat-latest gpt-realtime-translate gpt-realtime-2 gpt-realtime-whisper gpt-5.6-sol gpt-5.6-terra gpt-5.6-luna gpt-realtime-2.1 gpt-realtime-2.1-mini gpt-transcribe gpt-live-transcribe gpt-6-astra]
Anthropic models: [claude-fable-5-1 claude-opus-5 claude-sonnet-5 claude-fable-5 claude-opus-4-8 claude-opus-4-7 claude-sonnet-4-6 claude-opus-4-6 claude-opus-4-5-20251101 claude-haiku-4-5-20251001 claude-sonnet-4-5-20250929]
Perplexity models: []
Gemini models: []
  anthropic: ok 11 models
  gemini: not_configured (unverified) No Gemini API Key
  openai: ok 127 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get OpenAI Models ===
OpenAI models: [text-embedding-ada-002 whisper-1 gpt-3.5-turbo tts-1 gpt-3.5-turbo-16k gpt-4-0613 gpt-4 davinci-002 babbage-002 gpt-3.5-turbo-instruct gpt-3.5-turbo-instruct-0914 gpt-3.5-turbo-1106 tts-1-hd tts-1-1106 tts-1-hd-1106 text-embedding-3-small text-embedding-3-large gpt-3.5-turbo-0125 gpt-4-turbo gpt-4-turbo-2024-04-09 gpt-4o gpt-4o-2024-05-13 gpt-4o-mini-2024-07-18 gpt-4o-mini gpt-4o-2024-08-06 omni-moderation-latest omni-moderation-2024-09-26 o1-2024-12-17 o1 o3-mini o3-mini-2025-01-31 gpt-4o-2024-11-20 gpt-4o-mini-search-preview-2025-03-11 gpt-4o-mini-search-preview gpt-4o-transcribe gpt-4o-mini-transcribe o1-pro-2025-03-19 o1-pro gpt-4o-mini-tts o3-2025-04-16 o4-mini-2025-04-16 o3 o4-mini gpt-4.1-2025-04-14 gpt-4.1 gpt-4.1-mini-2025-04-14 gpt-4.1-mini gpt-4.1-nano-2025-04-14 gpt-4.1-nano gpt-image-1 o4-mini-deep-research gpt-4o-transcribe-diarize o4-mini-deep-research-2025-06-26 gpt-5-chat-latest gpt-5-2025-08-07 gpt-5 gpt-5-mini-2025-08-07 gpt-5-mini gpt-5-nano-2025-08-07 gpt-5-nano gpt-audio-2025-08-28 gpt-realtime gpt-realtime-2025-08-28 gpt-audio gpt-5-codex gpt-image-1-mini gpt-5-pro-2025-10-06 gpt-5-pro gpt-audio-mini gpt-audio-mini-2025-10-06 gpt-5-search-api gpt-realtime-mini sora-2 sora-2-pro gpt-5-search-api-2025-10-14 gpt-5.1-chat-latest gpt-5.1-2025-11-13 gpt-5.1 gpt-5.1-codex gpt-5.1-codex-mini gpt-5.1-codex-max gpt-image-1.5 gpt-5.2-2025-12-11 gpt-5.2 gpt-5.2-pro-2025-12-11 gpt-5.2-pro gpt-5.2-chat-latest gpt-4o-mini-transcribe-2025-12-15 gpt-4o-mini-transcribe-2025-03-20 gpt-4o-mini-tts-2025-03-20 gpt-4o-mini-tts-2025-12-15 gpt-realtime-mini-2025-12-15 gpt-audio-mini-2025-12-15 chatgpt-image-latest gpt-5.2-codex gpt-5.3-codex gpt-realtime-1.5 gpt-audio-1.5 gpt-4o-search-preview gpt-4o-search-preview-2025-03-11 gpt-5.3-chat-latest gpt-5.4-2026-03-05 gpt-5.4-pro gpt-5.4-pro-2026-03-05 gpt-5.4 gpt-5.4-nano-2026-03-17 gpt-5.4-nano gpt-5.4-mini-2026-03-17 gpt-5.4-mini gpt-image-2 gpt-image-2-2026-04-21 gpt-5.5 gpt-5.5-2026-04-23 gpt-5.5-pro gpt-5.5-pro-2026-04-23 chat-latest gpt-realtime-translate gpt-realtime-2 gpt-realtime-whisper gpt-5.6-sol gpt-5.6-terra gpt-5.6-luna gpt-realtime-2.1 gpt-realtime-2.1-mini gpt-transcribe gpt-live-transcribe gpt-6-astra]

=== Get Anthropic Models ===
Anthropic models: [claude-fable-5-1 claude-opus-5 claude-sonnet-5 claude-fable-5 claude-opus-4-8 claude-opus-4-7 claude-sonnet-4-6 claude-opus-4-6 claude-opus-4-5-20251101 claude-haiku-4-5-20251001 claude-sonnet-4-5-20250929]

✓ Chat Models API example complete
✓ Client created

=== Create User Function ===
Created user function with ID: Res8YXGt04bVZM4VTqWmuDhGV1kUXWorgizDO0MnIkPaROwzupN4yUsFvZbxB9H-gyg7cxmuCZtuGQsPC7lPdA

=== Get User Function ===
Retrieved: get_active_users - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 9 user functions:
  - fetch_slim_user: Validate and slim down user
  - fetch_user: Fetch user by code
  - validate_user: Check if user exists
  - swr_user: SWR pattern for user data (KV-based)
  - cache_api_call_py: Cache External API Call
  - get_verified_user: Get verified and validated user
  - get_user_wrapper: Wrapper that calls fetch_user
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_active_users: Get Active Users

=== List User Functions by Tag ===
Found 9 user functions with 'users' tag:
  - fetch_slim_user
  - fetch_user
  - validate_user
  - swr_user
  - cache_api_call_py
  - get_verified_user
  - get_user_wrapper
  - fetch_and_store_user
  - get_active_users

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
✓ Client created
✓ crypto_demo_hmac saved
✓ crypto_demo_aes saved
✓ crypto_demo_uuid saved
✓ crypto_demo_totp saved
✓ crypto_demo_encoding saved

Invoke them with:
  POST /api/functions/crypto_demo_hmac     { "payload": "hi" }
  POST /api/functions/crypto_demo_aes      { "plaintext": "secret" }
  POST /api/functions/crypto_demo_uuid
  POST /api/functions/crypto_demo_totp
  POST /api/functions/crypto_demo_encoding { "title": "Héllo World" }

✓ Cleaned up demo functions
✓ Client created
✓ conc_demo_pay saved
✓ conc_demo_rl_fail saved
✓ conc_demo_rl_skip saved
✓ conc_demo_lock saved

Invoke them like:
  POST /api/functions/conc_demo_pay        { "idempotency_key": "...", "amount": 100 }
  POST /api/functions/conc_demo_rl_fail    { "user_id": 42 }
  POST /api/functions/conc_demo_rl_skip    { "user_id": 42 }
  POST /api/functions/conc_demo_lock       { "resource": "queue:drain" }

✓ Cleaned up demo functions
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
✅ [32mGo client examples complete![0m
✅ [32mAll Go integration tests complete![0m
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.26.1 prepare
> npm run build


> @ekodb/ekodb-client@0.26.1 build
> tsc


up to date, audited 52 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.26.1 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning TypeScript client library examples...[0m

added 1 package, removed 1 package, and audited 13 packages in 1s

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'TUtR-YvXv_OJXbs1cq48I6DIGZCP33tzruJYBctOsF-oh5TithfO2QOW8rcBaw9iAS1udNqkRmiHIoyDSDSHng'
}

=== Find by ID ===
Found: {
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  price: { type: 'Float', value: 99.99 },
  active: { value: true, type: 'Boolean' },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  created_at: { type: 'DateTime', value: '2026-09-07T21:47:45.074+00:00' },
  id: 'TUtR-YvXv_OJXbs1cq48I6DIGZCP33tzruJYBctOsF-oh5TithfO2QOW8rcBaw9iAS1udNqkRmiHIoyDSDSHng',
  name: { type: 'String', value: 'Test Record' },
  metadata: { value: { key: 'value', nested: [Object] }, type: 'Object' },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  value: { type: 'Integer', value: 42 }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-09-07T21:47:45.074Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { key: 'value', nested: { deep: true } }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  tags: [ 'tag1', 'tag2', 'tag3' ],
  price: 99.99,
  active: true,
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  data: 'aGVsbG8gd29ybGQ=',
  created_at: '2026-09-07T21:47:45.074+00:00',
  id: 'TUtR-YvXv_OJXbs1cq48I6DIGZCP33tzruJYBctOsF-oh5TithfO2QOW8rcBaw9iAS1udNqkRmiHIoyDSDSHng',
  name: 'Test Record',
  metadata: { key: 'value', nested: { deep: true } },
  categories: [ 'electronics', 'computers' ],
  value: 42
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  metadata: { value: { key: 'value', nested: [Object] }, type: 'Object' },
  name: { type: 'String', value: 'Updated Record' },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  value: { value: 100, type: 'Integer' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  id: 'TUtR-YvXv_OJXbs1cq48I6DIGZCP33tzruJYBctOsF-oh5TithfO2QOW8rcBaw9iAS1udNqkRmiHIoyDSDSHng',
  created_at: { type: 'DateTime', value: '2026-09-07T21:47:45.074+00:00' },
  active: { type: 'Boolean', value: true },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  price: { value: 99.99, type: 'Float' },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: Qhn0qLw0XR8ZxjoLgHlohgcr6bUjgXGBYUyksSR39N9DVdYLkWxHrHf5TbrU6Jg4ezhIo6EGDYtRoAaQ2vklQA

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
Collection created with first record: UOIvnFWBl93XgWpY14g0HH4bR8HfAVrZZnrfT6HaBv-_l7wo3B59MUjIWDEoeyrnDtLoSRcGoM5kYR7FsrpGrA

=== List Collections ===
Total collections: 24
Sample collections: products,ttl_cache,schema_employees_client_go,test_collection,websocket_test

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
Retrieved value: { value: { username: 'john_doe', userId: 123 }, type: 'Object' }

=== KV Batch Set ===
✓ Batch set 3 keys
  cache:product:1: success
  cache:product:2: success
  cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  cache:product:1: { name: 'Product 1', price: 29.99 }
  cache:product:2: { name: 'Product 2', price: 39.99 }
  cache:product:3: { name: 'Product 3', price: 49.99 }

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
Created Alice: $1000 - ID: JOHTOJnoTOUyKFCeoM_1hDVnXgszGon71qvJbMTaiUMR6nD4eB8CGGE228zEKXLmZROEJxwlIr8KbTcztMt5ZA
Created Bob: $500 - ID: K2Vpr6c-zlRFUwZR_qWpQybah_E5b8w3NkpJjcqpMRk0T3myvb-ZIt4nXTHqbdBkTP2iuqDlDNQ2CuqqVzic7A

=== Example 1: Begin Transaction ===
Transaction ID: bf5cc9f1-0270-42e2-95eb-535d21db6ac3

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: f402bc79-2c7f-4d68-9720-b2b6d307bbeb
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
  1. Score: 12.870, Matched: email, name
  2. Score: 6.270, Matched: name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 13.200, Matched: bio, title
  2. Score: 13.200, Matched: title, bio
  3. Score: 13.200, Matched: title, bio
  4. Score: 13.200, Matched: bio, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400, Matched: bio, title
  2. Score: 26.400, Matched: bio, title
  3. Score: 26.400, Matched: bio, title
  4. Score: 26.400, Matched: bio, title

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 6.600, Matched: bio
  2. Score: 6.600, Matched: bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 6.600, Matched: bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.744, Matched: 
  2. Score: 0.736, Matched: 
  3. Score: 0.729, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.498, Matched: title, content
  2. Score: 0.894, Matched: content, title
  3. Score: 0.291, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: skills, bio

9. Vector search with a metadata pre-filter (category = ml):
Found 2 documents in category "ml" (NLP excluded)
  1. Introduction to Machine Learning (category: ml)
  2. Deep Learning Fundamentals (category: ml)

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
✓ Inserted document: nYk5JepRDC0u8oRbPIJmoh05xu5tpgFOHaDTfdjxbatQv9x0v6vMjkzIV6KPvtvc3IYKKAiX02CktQSEWUFYpg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: qbGFgZTR52MwbRtJA1YFbDNEquI8oCKdZ7gP8E02NniD0ATtj5Hymfx7MC_EmnMw2tY9QFu4yD2cbpJk0aUs0g

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
✓ Inserted document with TTL: vAATbxvuuukBZsuk3Lg3U6Z3aJ33lnNcEEpUP9eP_avP2rkEihg05IpeV7n2ta34CArybg_uxQVZOzMzHR6foA

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
✓ Edge cache script created: ElD7AU5ZpWMoljVH1FgmY4Or1U4oCQP5lRWZUqyXBnLM6EgzjbCIIcVIJdiN-IukQkYmob8xVxgCfC95mMcg2w

Call 1: Cache miss (fetches from API)
Response time: 66ms
Result: {
  "records": [
    {
      "value": {
        "value": {
          "address": {
            "street": "Kulas Light",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "suite": "Apt. 556",
            "zipcode": "92998-3874",
            "city": "Gwenborough"
          },
          "phone": "1-770-736-8031 x56442",
          "name": "Leanne Graham",
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "email": "Sincere@april.biz",
          "id": 1,
          "website": "hildegard.org",
          "username": "Bret"
        },
        "type": "Object"
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
Response time: 2ms (33x faster!)
Result: {
  "records": [
    {
      "value": {
        "value": {
          "address": {
            "street": "Kulas Light",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "suite": "Apt. 556",
            "zipcode": "92998-3874",
            "city": "Gwenborough"
          },
          "phone": "1-770-736-8031 x56442",
          "name": "Leanne Graham",
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "email": "Sincere@april.biz",
          "id": 1,
          "website": "hildegard.org",
          "username": "Bret"
        },
        "type": "Object"
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

✅ Function saved: dJqB2rLMtP4USlK7vf0-KIzzzCt5VLhZRvdkCoZFPXJW67WAWk2Kcl2FhRoHHoITbGEI-a0rUOcKACeu7rcw8w
📊 Found 15 active users

📝 Example 2: Parameterized Function

✅ Function saved: DL0U2hMOTwLLQyBgjMvl-l1d4Sk9zFZdZsUHD2pYE0SsBH3f-lk4Esk5prwSGryyLsdo0435kryOsnTgIGvNog
📊 Found 15 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: s-RirrUzfNlQUGEjwczB7oj3yTYeCgsl9_TU0-j6Wi1f3FOO5rlpTbLwJfyTERJfQ5Qy7c9f4J29G45S3XK3vA
📊 Statistics: 3 groups
   {"avg_score":{"value":60,"type":"Float"},"status":{"type":"String","value":"null"},"count":{"value":5,"type":"Integer"}}
   {"status":{"type":"String","value":"inactive"},"avg_score":{"type":"Float","value":50},"count":{"value":5,"type":"Integer"}}
   {"avg_score":{"type":"Float","value":60},"count":{"type":"Integer","value":5},"status":{"value":"active","type":"String"}}

📝 Example 4: UserFunction Management

📋 Total scripts: 12
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

Function 'fetch_user' already existed — updated instead
✅ Saved reusable function: fetch_user
Function 'get_user_wrapper' already existed — updated instead
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"value":"User 1","type":"String"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

Function 'fetch_and_store_user' already existed — updated instead
✅ Saved reusable function: fetch_and_store_user (uses KV)
Function 'swr_user' already existed — updated instead
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 1ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "address": {
        "street": "Kulas Light",
        "city": "Gwenborough",
        "geo": {
          "lat": "-37.3159",
          "lng": "8...

Second call (cache hit - from cache):
   ⏱️  Duration: 2ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "address": {
        "street": "Kulas Light",
        "city": "Gwenborough",
        "geo": {
          "lat": "-37.3159",
          "lng": "8...
   🚀 Cache speedup: 0.5x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

Function 'validate_user' already existed — updated instead
✅ Level 1 function: validate_user
Function 'fetch_slim_user' already existed — updated instead
✅ Level 2 function: fetch_slim_user (calls validate_user)
Function 'get_verified_user' already existed — updated instead
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

✅ Function saved: eN25y5naouwyakSEVqOfQ1yRASkJJZUPTYTeSMECxMzyNKZm7srz9pgJXzhIjUgMoubMvR_HL80DwtMHgtVF_A
📊 Found 2 product groups
   {"count":{"value":2,"type":"Integer"},"avg_price":{"value":474,"type":"Float"},"category":{"type":"String","value":"Furniture"}}
   {"count":{"value":3,"type":"Integer"},"category":{"type":"String","value":"Electronics"},"avg_price":{"type":"Float","value":575.6666666666666}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"category":{"type":"String","value":"Furniture"},"count":{"type":"Integer","value":2}}
   {"count":{"type":"Integer","value":3},"category":{"type":"String","value":"Electronics"}}
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

✅ Inserted order: NiyIMb45ocKxZrRTZKi8BqHkcNiWdBdigMyzVurDWPZz8Y96ALNYYpDO-N-zCkGjbWS9cTloRAAiekf4v8XM2Q
✅ Inserted 2 products with wrapped types

📝 Example 2: UserFunction with Wrapped Type Parameters

✅ Function saved: 5xSo4JfsfjR3pdlF5U5cZAxCHEc-pOwb8cicHPZ1q7njkeogFBOhy54lMQCrF7dsra4q1izb8zSEwrZoKIShCg
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"type":"Object","value":{"userId":"user_abc","role":"admin"}}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: Y10QbIxDcvAUaXM3ccGRWYKwzApoNe3KOpG0txIM1VOvOVxMM4H-_iBbdN4wO_uMyOsvGdgFMeGBj-Bj3bQU6w
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: lWdk8UcrcPJcc2PCsFckBfy0c2Q8pHGE2x-c9EzgoPEpY8ymXsOX7U59Lsgc47Td_BfjQLVCwc-DasM4Pyy5AQ
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
✓ Created SWR script: fetch_api_user (PgXYP7cVkuyqfaKKSXV2rSw8UDtaX6K-olROgY6tGAD4yOk2J2y1o_s4rnZxvI0bWpR8s0gvjijyVO0a-PNfJA)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "value": {
          "website": "hildegard.org",
          "address": {
            "street": "Kulas Light",
            "zipcode": "92998-3874",
            "city": "Gwenborough",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "suite": "Apt. 556"
          },
          "phone": "1-770-736-8031 x56442",
          "name": "Leanne Graham",
          "username": "Bret",
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets"
          },
          "email": "Sincere@april.biz",
          "id": 1
        },
        "type": "Object"
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
Response time: 2ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "website": "hildegard.org",
          "address": {
            "street": "Kulas Light",
            "zipcode": "92998-3874",
            "city": "Gwenborough",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "suite": "Apt. 556"
          },
          "phone": "1-770-736-8031 x56442",
          "name": "Leanne Graham",
          "username": "Bret",
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets"
          },
          "email": "Sincere@april.biz",
          "id": 1
        }
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
✓ Created enrichment script: fetch_product_with_reviews (yKquqfQsDHSBIJgIHKCeE5qeABM2sjimKMRRKfoPT2VxvIFoSslka62RqE1PmFWhM6fDFU9wLpdMdIN3LfUiJQ)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "value": {
          "shippingInformation": "Ships in 3-5 business days",
          "title": "Essence Mascara Lash Princess",
          "stock": 99,
          "warrantyInformation": "1 week warranty",
          "weight": 4,
          "sku": "BEA-ESS-ESS-001",
          "rating": 2.56,
          "id": 1,
          "category": "beauty",
          "availabilityStatus": "In Stock",
          "tags": [
            "beauty",
            "mascara"
          ],
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "price": 9.99,
          "returnPolicy": "No return policy",
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "reviews": [
            {
              "reviewerName": "Eleanor Collins",
              "date": "2025-04-30T09:41:02.053Z",
              "rating": 3,
              "comment": "Would not recommend!",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com"
            },
            {
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerName": "Lucas Gordon",
              "reviewerEmail": "lucas.gordon@x.dummyjson.com",
              "comment": "Very satisfied!",
              "rating": 4
            },
            {
              "date": "2025-04-30T09:41:02.053Z",
              "comment": "Highly impressed!",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "rating": 5,
              "reviewerName": "Eleanor Collins"
            }
          ],
          "meta": {
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
            "updatedAt": "2025-04-30T09:41:02.053Z",
            "barcode": "5784719087687",
            "createdAt": "2025-04-30T09:41:02.053Z"
          },
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "brand": "Essence",
          "discountPercentage": 10.48,
          "minimumOrderQuantity": 48,
          "dimensions": {
            "depth": 22.99,
            "width": 15.14,
            "height": 13.08
          }
        },
        "type": "Object"
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
✓ Created native SWR script: github_user_native (xgoGw1j5VMHQUetrdHuaEXIUAN0RVDeNHfY2tesFF-2mwXV7HkjPGp2BMdfcNAJqCgXCKx9e_gi4HJVWzS4ROA)

First call (cache miss - will fetch from GitHub API):
  Response time: 4ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 3ms
  Speedup: 1.3x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (xiG0BtdJ8X9qiEchdGESL-4KXfwETgqUq_eAUkilR8Q_jHuVP9tW_zXAqxB1v5oOiWCayT8ZpHYDunAJVptbZA)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (6gz9vStJkUdKTaB60o1kx-pjtL5g4nUI2MiJcQSNnJ1SAGJRf0hp8WxUytSek8YF5o3RvYlJCXCN9x-AudZcsw)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (JLqSQZs4qr9pWazrx5irN8OBEN2rtYN6r4oJqii4qD97DotkMEW16p0Y11HMDwEwPLChyTLWIwpjuHH1dDfrwQ)
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
   {"avg_price":{"type":"Float","value":365.6666666666667},"count":{"type":"Integer","value":3},"category":{"value":"Furniture","type":"String"}}
   {"avg_price":{"type":"Float","value":367},"category":{"type":"String","value":"Electronics"},"count":{"type":"Integer","value":5}}
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
   Vector databases offer several benefits, including:

1. **Similarity Search**: Efficiently find similar items using high-dimensional vector representations, ideal for tasks like image and text similarity.

2. **Scalability**: Designed to handle large datasets with high-dimensional vectors, making them suitable for modern applications.

3. **Fast Retrieval**: Optimized algorithms for nearest neighbor search allow for rapid data retrieval.

4. **Flexibility**: Support for various data types such as images, text, and audio, enabling diverse use cases.

5. **Improved Performance**: Specialized indexing structures (e.g., HNSW, Annoy) enhance performance over traditional databases for vector data.

6. **Integration with AI/ML**: Naturally integrates with machine learning workflows, particularly for applications like recommendation systems and natural language processing.

7. **Real-time Processing**: Enables real-time applications by supporting immediate updates and querying of vectors.

8. **Dimensionality Reduction**: Many frameworks include tools for reducing dimensions while preserving similarity, optimizing performance. 

These strengths make vector databases particularly advantageous in AI, search engines, recommendation systems, and any domain that relies on similarity comparisons.
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
   active: 7 users
   inactive: 3 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Function saved
📊 Average score by role:
   {"avg_score":{"type":"Float","value":70},"count":{"value":7,"type":"Integer"},"role":{"value":"user","type":"String"}}
   {"role":{"type":"String","value":"admin"},"count":{"type":"Integer","value":3},"avg_score":{"type":"Float","value":20}}
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
   1. Database Design Principles (Database)
   2. Natural Language Processing (AI)
   3. Introduction to Machine Learning (AI)
   4. Vector Databases Explained (Database)
   5. Getting Started with ekoDB (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"category":{"type":"String","value":"Database"},"count":{"value":3,"type":"Integer"}}
   {"category":{"type":"String","value":"AI"},"count":{"type":"Integer","value":2}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: sjclm-aZIqRQEaVKRC2CXdhXbzklEgeGdfCdlSgrAS5TbqDCfzNpGiRXszCUHMQnO2zG2AmHf1FG_UwX9fqSyg

=== Sending Chat Message ===
Message ID: O41mKWZjn2uerH6fYAt8tJlgPVpDLAS6bkoqkYmLVZU0BbCal48yhulyUnO7dRU4SGGDHFX5E9az9LJshlC_-A

=== AI Response ===
The available products and their prices are as follows:

1. **ekoDB**: $99  
   Description: A high-performance database product with AI capabilities.

2. **ekoDB Cloud**: $499  
   Description: Fully managed cloud database service product.

3. **ekoDB Pro**: $299  
   Description: Enterprise edition product with advanced features.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'A high-performance database product with AI capabilities',
    id: '8JdYi5JWqzhE5HFOq3iVYRBVn0c4lMf41Ijaq67kDjGuGXsqiXIQzuLcJH7YxlwlKWTGpv_z3Axver0_SWQuLg',
    name: 'ekoDB',
    price: 99
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 499,
    id: 'spkZ7tpnaqD8lp4k9kfVBnAyi8dmU4X3tD4mbrdFmKlxrNDMgqWxJlRESUD-yr_zNJ5w2fCr-IxOp9jl595kPw',
    name: 'ekoDB Cloud',
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Enterprise edition product with advanced features',
    price: 299,
    name: 'ekoDB Pro',
    id: 'L7l8xUgYuOHEgeYD2rInHggnl1F0ZJ0LdhZtcMcRw5QcR3juoYLapB7K97ZwCiF2F0OkNY7YCWLqgxCnM2Ow_Q'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 1721ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 83
Total tokens: 3496

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: ZLwovwfZAels3YSVt7gJ135LFJOSNT5QuikuchYT7fGAV8bdeZWjPDXO-F7TkLqpWkY-j83fobyYgwzDlZr1KA

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

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
✓ Created second session: B6IIPrlCRAJncGdiIVYwrU8vPsM-QTsUq6VGbpioL3_NKWXM4N_d9zRb_kUQMQlstoTx_q60VDE_1C-Mr15vhQ
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 6RYvcxkccIYvET9JFSOmMNzfvbnZm6kzWHLQdDBVw3gjLQJNDXABj0lTKF6J4fUup43qcFsICDKLK1PWnBP_ow

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99

If you have any specific questions or if you need more information, feel free to ask!

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: KuGEluBPl9Z26uQqSMc0BtjQC7ThcUs_JIdU5wVrbn38fEvetMR4Rq5JxRQ_0XNeAFHkP9tzAreToQptX5TvGg
  Parent: 6RYvcxkccIYvET9JFSOmMNzfvbnZm6kzWHLQdDBVw3gjLQJNDXABj0lTKF6J4fUup43qcFsICDKLK1PWnBP_ow

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: KuGEluBPl9Z26uQqSMc0BtjQC7ThcUs_JIdU5wVrbn38fEvetMR4Rq5JxRQ_0XNeAFHkP9tzAreToQptX5TvGg (Untitled)
  Session 2: 6RYvcxkccIYvET9JFSOmMNzfvbnZm6kzWHLQdDBVw3gjLQJNDXABj0lTKF6J4fUup43qcFsICDKLK1PWnBP_ow (Untitled)
  Session 3: B6IIPrlCRAJncGdiIVYwrU8vPsM-QTsUq6VGbpioL3_NKWXM4N_d9zRb_kUQMQlstoTx_q60VDE_1C-Mr15vhQ (Untitled)
  Session 4: sjclm-aZIqRQEaVKRC2CXdhXbzklEgeGdfCdlSgrAS5TbqDCfzNpGiRXszCUHMQnO2zG2AmHf1FG_UwX9fqSyg (Untitled)
  Session 5: 5hi3bOxBbTb2WcHkXfr7bCB4mS9f44BEJ-DAXVsqrF96OST0diI2NxRenbSDb4MEKRC9FbQwtn6_hP_p7xTt4w (Untitled)
  Session 6: Kh_2y1eqgZka3uqUF1ti21UPDhrWr12DBbgPg3rsoS8AX9-4AVxYHe8VZ1pIwPbRRRaEA6LYfPvIteEXjW8_yQ (Untitled)
  Session 7: EGq4BpwScMdsQDiBfksclFl8625dagxm3gVyxIyXJKhHxHZyinKJiOelY97gqIKri4o4BnJj2GVPWG8XuixU0Q (Untitled)
  Session 8: nkBJXWlWv5CQEGIuO_iZt14BLDnyv3qqWNQSWIMEdEG76GnvcBXraqm_M40BZLRGqU3AaCGxIVVZv7wwyClb2g (Untitled)
  Session 9: MsFgaGHuEMqowhdNh55laOo1tJMk2CoF8bjwtylNl90Ei5BOTrKz0kqajiENJs7al67mxOoZizgXgU1_a19jWg (Untitled)
  Session 10: daiZC2QZTBDSnnyM0mBh543J3Ee0r_G5DhBylJLWym6POyg7M3-dT4tow1FkEhyU8qc6hIfOLBx-FI_9u16NIA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: KuGEluBPl9Z26uQqSMc0BtjQC7ThcUs_JIdU5wVrbn38fEvetMR4Rq5JxRQ_0XNeAFHkP9tzAreToQptX5TvGg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'QYiZbsTW9VHdc-yf30ElLMNyyDbu7uvOlqgh5AdURpsvdFnH6FYpv4gNpSIovxUN_bzTVuMRgOjpVAOd1knOtQ'
}

=== Upsert Operation ===
✓ First upsert (update): {
  age: { type: 'Integer', value: 29 },
  name: { value: 'Alice Johnson', type: 'String' },
  email: { value: 'alice.j@newdomain.com', type: 'String' },
  id: 'QYiZbsTW9VHdc-yf30ElLMNyyDbu7uvOlqgh5AdURpsvdFnH6FYpv4gNpSIovxUN_bzTVuMRgOjpVAOd1knOtQ',
  active: { value: true, type: 'Boolean' }
}
✓ Second upsert (insert): {
  id: 'OQBxdeJNYNfKhLeEJNZjUCHPu9iGd9ZvvANjeg9lZIxcoky-pLIXL3NS_kCkOePJDkuAgLekvP2laKxnPvDy4w'
}

=== Find One Operation ===
✓ Found user by email: {
  email: { value: 'alice.j@newdomain.com', type: 'String' },
  id: 'QYiZbsTW9VHdc-yf30ElLMNyyDbu7uvOlqgh5AdURpsvdFnH6FYpv4gNpSIovxUN_bzTVuMRgOjpVAOd1knOtQ',
  active: { type: 'Boolean', value: true },
  name: { type: 'String', value: 'Alice Johnson' },
  age: { value: 29, type: 'Integer' }
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
   Inserted with ripple: {"id":"Snv6NpB7NLMwaAQ0_VUtvHMKKURne6pJpmlLH93TYLyG73UCzvFMxnnoKx4FQQ01Ksx2ScY57KHHmhfIsXWbNw"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"KsmFakcMKYSdwCaB_tJdPmp3je2N9iNrOpzM2EfocEzirj6ZzUOQyc5FnMQi8leFi6yRQonxIj1EiXTSbKGzvQ"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"id":"Snv6NpB7NLMwaAQ0_VUtvHMKKURne6pJpmlLH93TYLyG73UCzvFMxnnoKx4FQQ01Ksx2ScY57KHHmhfIsXWbNw","price":{"value":150,"type":"Integer"},"name":{"value":"Product 1","type":"String"}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"name":{"value":"Upsert Product","type":"String"},"price":{"value":500,"type":"Integer"},"id":"custom-id"}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ["name","id","email"]
  First user: [object Object] <[object Object]>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["created_at","status","avatar_url","id","email","user_role","age","name","bio"]

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
    - Fields: ["secret_token","status","user_role","id","age","created_at","bio","email","name","password","api_key","avatar_url"]
  Projected query:
    - 3 fields per record
    - Fields: ["name","id","email"]
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
  Float 0.1 + 0.2 = undefined (should be 0.3)
  Decimal "0.30" = undefined (exact!)

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
    - text-embedding-ada-002
    - whisper-1
    - gpt-3.5-turbo
    - tts-1
    - gpt-3.5-turbo-16k
    - gpt-4-0613
    - gpt-4
    - davinci-002
    - babbage-002
    - gpt-3.5-turbo-instruct
    - gpt-3.5-turbo-instruct-0914
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
    - omni-moderation-latest
    - omni-moderation-2024-09-26
    - o1-2024-12-17
    - o1
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
    - gpt-5.4
    - gpt-5.4-nano-2026-03-17
    - gpt-5.4-nano
    - gpt-5.4-mini-2026-03-17
    - gpt-5.4-mini
    - gpt-image-2
    - gpt-image-2-2026-04-21
    - gpt-5.5
    - gpt-5.5-2026-04-23
    - gpt-5.5-pro
    - gpt-5.5-pro-2026-04-23
    - chat-latest
    - gpt-realtime-translate
    - gpt-realtime-2
    - gpt-realtime-whisper
    - gpt-5.6-sol
    - gpt-5.6-terra
    - gpt-5.6-luna
    - gpt-realtime-2.1
    - gpt-realtime-2.1-mini
    - gpt-transcribe
    - gpt-live-transcribe
    - gpt-6-astra
  anthropic:
    - claude-fable-5-1
    - claude-opus-5
    - claude-sonnet-5
    - claude-fable-5
    - claude-opus-4-8
    - claude-opus-4-7
    - claude-sonnet-4-6
    - claude-opus-4-6
    - claude-opus-4-5-20251101
    - claude-haiku-4-5-20251001
    - claude-sonnet-4-5-20250929
Provider status:
  anthropic: ok 11 models
  gemini: not_configured (unverified) No Gemini API Key
  openai: ok 127 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get Specific Provider Models ===
OpenAI models: text-embedding-ada-002, whisper-1, gpt-3.5-turbo, tts-1, gpt-3.5-turbo-16k, gpt-4-0613, gpt-4, davinci-002, babbage-002, gpt-3.5-turbo-instruct, gpt-3.5-turbo-instruct-0914, gpt-3.5-turbo-1106, tts-1-hd, tts-1-1106, tts-1-hd-1106, text-embedding-3-small, text-embedding-3-large, gpt-3.5-turbo-0125, gpt-4-turbo, gpt-4-turbo-2024-04-09, gpt-4o, gpt-4o-2024-05-13, gpt-4o-mini-2024-07-18, gpt-4o-mini, gpt-4o-2024-08-06, omni-moderation-latest, omni-moderation-2024-09-26, o1-2024-12-17, o1, o3-mini, o3-mini-2025-01-31, gpt-4o-2024-11-20, gpt-4o-mini-search-preview-2025-03-11, gpt-4o-mini-search-preview, gpt-4o-transcribe, gpt-4o-mini-transcribe, o1-pro-2025-03-19, o1-pro, gpt-4o-mini-tts, o3-2025-04-16, o4-mini-2025-04-16, o3, o4-mini, gpt-4.1-2025-04-14, gpt-4.1, gpt-4.1-mini-2025-04-14, gpt-4.1-mini, gpt-4.1-nano-2025-04-14, gpt-4.1-nano, gpt-image-1, o4-mini-deep-research, gpt-4o-transcribe-diarize, o4-mini-deep-research-2025-06-26, gpt-5-chat-latest, gpt-5-2025-08-07, gpt-5, gpt-5-mini-2025-08-07, gpt-5-mini, gpt-5-nano-2025-08-07, gpt-5-nano, gpt-audio-2025-08-28, gpt-realtime, gpt-realtime-2025-08-28, gpt-audio, gpt-5-codex, gpt-image-1-mini, gpt-5-pro-2025-10-06, gpt-5-pro, gpt-audio-mini, gpt-audio-mini-2025-10-06, gpt-5-search-api, gpt-realtime-mini, sora-2, sora-2-pro, gpt-5-search-api-2025-10-14, gpt-5.1-chat-latest, gpt-5.1-2025-11-13, gpt-5.1, gpt-5.1-codex, gpt-5.1-codex-mini, gpt-5.1-codex-max, gpt-image-1.5, gpt-5.2-2025-12-11, gpt-5.2, gpt-5.2-pro-2025-12-11, gpt-5.2-pro, gpt-5.2-chat-latest, gpt-4o-mini-transcribe-2025-12-15, gpt-4o-mini-transcribe-2025-03-20, gpt-4o-mini-tts-2025-03-20, gpt-4o-mini-tts-2025-12-15, gpt-realtime-mini-2025-12-15, gpt-audio-mini-2025-12-15, chatgpt-image-latest, gpt-5.2-codex, gpt-5.3-codex, gpt-realtime-1.5, gpt-audio-1.5, gpt-4o-search-preview, gpt-4o-search-preview-2025-03-11, gpt-5.3-chat-latest, gpt-5.4-2026-03-05, gpt-5.4-pro, gpt-5.4-pro-2026-03-05, gpt-5.4, gpt-5.4-nano-2026-03-17, gpt-5.4-nano, gpt-5.4-mini-2026-03-17, gpt-5.4-mini, gpt-image-2, gpt-image-2-2026-04-21, gpt-5.5, gpt-5.5-2026-04-23, gpt-5.5-pro, gpt-5.5-pro-2026-04-23, chat-latest, gpt-realtime-translate, gpt-realtime-2, gpt-realtime-whisper, gpt-5.6-sol, gpt-5.6-terra, gpt-5.6-luna, gpt-realtime-2.1, gpt-realtime-2.1-mini, gpt-transcribe, gpt-live-transcribe, gpt-6-astra

=== Get Anthropic Models ===
Anthropic models: claude-fable-5-1, claude-opus-5, claude-sonnet-5, claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 404: {"error":"Unknown provider 'nonexistent_provider_xyz'","error_kind":"unknown_provider"}

✓ Chat Models example complete
✓ Client created

=== Create User Function ===
Created user function with ID: SWDgrLGzDF-tD-BxxBzcKBVTKciQCS9S3XPMfZ4Tmpp-g7VczxiPiRLq02TSlcOnVuh4Z3M48oY5RUlYVCAODg

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 14 user functions:
  - get_users_by_status: Get Users By Status
  - cache_api_call: Cache External API Call
  - get_user_wrapper: Wrapper that calls fetch_user
  - fetch_user: Fetch user by code
  - swr_user: SWR pattern for user data (KV-based)
  - validate_user: Check if user exists
  - fetch_slim_user: Validate and slim down user
  - get_verified_user: Get verified and validated user
  - get_active_users_updated: Get Active Users (Updated)
  - fetch_api_user: Fetch User with Cache
  - get_active_users_ts: Get Active Users
  - cache_api_call_py: Cache External API Call
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
✓ Client created
✓ ts_users_register saved
✓ ts_users_login saved
✓ ts_users_verify_token saved

=== Auth flow defined as pure stored functions ===
Call them like:
  POST /api/functions/ts_users_register { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/ts_users_login    { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/ts_users_verify_token { "token": "<jwt>" }

Set JWT_SECRET in ekoDB's environment_vars whitelist before invoking.

✓ Cleaned up demo functions
✓ Client created
✓ crypto_demo_hmac saved
✓ crypto_demo_aes saved
✓ crypto_demo_uuid saved
✓ crypto_demo_totp saved
✓ crypto_demo_encoding saved

Invoke them with:
  POST /api/functions/crypto_demo_hmac     { "payload": "hi" }
  POST /api/functions/crypto_demo_aes      { "plaintext": "secret" }
  POST /api/functions/crypto_demo_uuid
  POST /api/functions/crypto_demo_totp
  POST /api/functions/crypto_demo_encoding { "title": "Héllo World" }

✓ Cleaned up demo functions
✓ Client created
✓ conc_demo_pay saved
✓ conc_demo_rl_fail saved
✓ conc_demo_rl_skip saved
✓ conc_demo_lock saved

Invoke them like:
  POST /api/functions/conc_demo_pay        { "idempotency_key": "...", "amount": 100 }
  POST /api/functions/conc_demo_rl_fail    { "user_id": 42 }
  POST /api/functions/conc_demo_rl_skip    { "user_id": 42 }
  POST /api/functions/conc_demo_lock       { "resource": "queue:drain" }

✓ Cleaned up demo functions
✓ Client created
✓ ts_route_admin → GET /api/route/users/admin
✓ ts_route_user_by_id → GET /api/route/users/:id
✓ ts_route_user_posts → GET /api/route/users/:id/posts/:post_id
✓ ts_route_org_create_member → POST /api/route/orgs/:org/members

Try them with curl:
  curl http://localhost:8080/api/route/users/admin
  curl http://localhost:8080/api/route/users/42
  curl http://localhost:8080/api/route/users/42/posts/7
  curl -X POST http://localhost:8080/api/route/orgs/acme/members \
       -H 'Content-Type: application/json' -d '{"name":"alice"}'

✓ Cleaned up demo functions
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
=== Running ekoDB/ekodb-client/examples/javascript/simple_crud.js ===[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {
  id: 'bOGFlvx_LZHQxqmwPdKc8h6EN8pSxYPAItUYeG5L86Lwg6LBckPQLXgLTujh89iEECKPi1lphhaajpGzV8nC_g'
}

=== Find by ID ===
Found: {
  id: 'bOGFlvx_LZHQxqmwPdKc8h6EN8pSxYPAItUYeG5L86Lwg6LBckPQLXgLTujh89iEECKPi1lphhaajpGzV8nC_g',
  active: { type: 'Boolean', value: true },
  name: { type: 'String', value: 'Test Record' },
  value: { value: 42, type: 'Integer' }
}

=== Find with Query ===
Found documents: [
  {
    active: { value: true, type: 'Boolean' },
    id: 'bOGFlvx_LZHQxqmwPdKc8h6EN8pSxYPAItUYeG5L86Lwg6LBckPQLXgLTujh89iEECKPi1lphhaajpGzV8nC_g',
    name: { value: 'Test Record', type: 'String' },
    value: { type: 'Integer', value: 42 }
  }
]

=== Update Document ===
Updated: {
  id: 'bOGFlvx_LZHQxqmwPdKc8h6EN8pSxYPAItUYeG5L86Lwg6LBckPQLXgLTujh89iEECKPi1lphhaajpGzV8nC_g',
  active: { value: true, type: 'Boolean' },
  value: { type: 'Integer', value: 100 },
  name: { value: 'Updated Record', type: 'String' }
}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.js completed successfully[0m
[34m
=== Running ekoDB/ekodb-client/examples/javascript/simple_websocket.js ===[0m
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: 30SauUmfPHlTgmROWW2BDaUNvLyJ8PnKii8WxntAuOTQK1r1jd4TkA09L6ttBMdNREHVF3HXwasWOhW5-Rwx4A

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "type": "Success",
  "payload": {
    "data": [
      {
        "id": "myzh8ozWbo5uFwQiqnAkokhilfiGHiMexcrtOt5iGq6OK6Nf_x7KPR6M7rjnYzTUbCq1xZf2P8lSNWf4D7FC5Q",
        "active": {
          "type": "Boolean",
          "value": true
        },
        "value": {
          "type": "Integer",
          "value": 42
        },
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        }
      },
      {
        "value": {
          "value": 42,
          "type": "Integer"
        },
        "id": "30SauUmfPHlTgmROWW2BDaUNvLyJ8PnKii8WxntAuOTQK1r1jd4TkA09L6ttBMdNREHVF3HXwasWOhW5-Rwx4A",
        "name": {
          "value": "WebSocket Test Record",
          "type": "String"
        },
        "active": {
          "type": "Boolean",
          "value": true
        }
      },
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "eo4k2XKeeP4Jw-NIruzkp5BZ37laWem-xan_dukdvdrmZzoejTtyeo5p4ZQXOFLyMu6bG4acNriPRHzHTq0EoQ",
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "value": {
          "value": 42,
          "type": "Integer"
        }
      },
      {
        "id": "WsbRCK94m4hP3hqb2DP9BPADED5Qb3sY8pyagkVQFOgdbEuIFn84mi1fBQkRrv_aJlCNvJp3vQhqcdHXQRFFCQ",
        "name": {
          "value": "WebSocket Test Record",
          "type": "String"
        },
        "value": {
          "type": "Integer",
          "value": 42
        },
        "active": {
          "value": true,
          "type": "Boolean"
        }
      }
    ]
  },
  "messageId": "1788817699073"
}
✓ Retrieved 4 record(s) via WebSocket

✓ WebSocket example completed successfully
WebSocket closed
[32m✓ simple_websocket.js completed successfully[0m
[34m
=== Running ekoDB/ekodb-client/examples/javascript/http_functions.js ===[0m
🚀 ekoDB Functions Example (JavaScript/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: SaaBPM8pwKJbBJfM6OKoE2M-GbJfS2R7PSh0RKZhoMYYJRIwFebyCfJCzw6gM3jqnG-4ekRcoBLs-aC0Rd0AMg
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: WL5OEqVozpKxLCsiSXgVHr5Z2qTyf7FQhzT1f7RGGEr_rrqnZpbSoptmE16PrVzh-3x51ruGXZG_nwWkRmpzHg
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: jiqLMzqN44cJwOli6ucLpzLzZ6vNllgJ3gvdG20e4rVYjteqmwyjXLvCpd3_R-x_YqnYBGmeQ6y_fq2XiqYi-g
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":{"value":50,"type":"Float"},"count":{"type":"Integer","value":10},"status":{"value":"inactive","type":"String"},"max_score":{"type":"Integer","value":90}}
   {"status":{"type":"String","value":"active"},"avg_score":{"value":60,"type":"Float"},"count":{"type":"Integer","value":10},"max_score":{"type":"Integer","value":100}}

📝 Example 4: Function Management

📋 Total functions: 16
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
[32m✓ http_functions.js completed successfully[0m
[34m
=== Running ekoDB/ekodb-client/examples/javascript/batch_operations.js ===[0m
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
=== Running ekoDB/ekodb-client/examples/javascript/kv_operations.js ===[0m
=== Key-Value Operations (Direct HTTP) ===

✓ Authentication successful

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: { type: 'Object', value: { userId: 123, username: 'john_doe' } }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { value: { name: 'Product 1', price: 29.99 }, type: 'Object' }
cache:product:2: {
  value: { price: 39.989999999999995, name: 'Product 2' },
  type: 'Object'
}
cache:product:3: {
  type: 'Object',
  value: { name: 'Product 3', price: 49.989999999999995 }
}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
[32m✓ kv_operations.js completed successfully[0m
[34m
=== Running ekoDB/ekodb-client/examples/javascript/collection_management.js ===[0m
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: WWijaCrYn5XZtAggBHZwVdq0K3_lHsJMTKD4zM90bYvfK88hhUqv5FkIBjRkXWM2QxQV39_bKXS6g68D3x8Z0w

=== List Collections ===
Total collections: 27
Sample collections: [
  'schema_documents_client_ts',
  'schema_employees_client_ts',
  'ttl_cache',
  'schema_employees_client_go',
  'test_collection'
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
=== Running ekoDB/ekodb-client/examples/javascript/transactions.js ===[0m
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: G5mVE4nypxLFIGcVWJZc9GU0rcOC03LHGttMtQFVgiS6SY8XnhoQz-ROq6NI7jZAz3BNn134Pbn-qh8BjVey0w
Created Bob: $500 - ID: Tbwoaobti5a44xtSdM-ejs2wNkzAM-a3LkTMqYFoBgWW7zDpPHtngKkgRz8WyYwxGXkXlc-fjGdgMjfelQaGGA

=== Example 1: Begin Transaction ===
Transaction ID: cce50fd3-d086-430a-b542-113df7cf178d

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: {"value":800,"type":"Integer"}
Bob: {"value":700,"type":"Integer"}

=== Example 5: Rollback ===
New transaction: f04faf3b-c019-4955-8641-4ec15133ad38
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"type":"Integer","value":700}

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
[32m✓ transactions.js completed successfully[0m
[34m
=== Running ekoDB/ekodb-client/examples/javascript/crud_functions.js ===[0m
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
   ✅ Function saved: dKuZr4Zk5Oz7OLxx0qRprLm3JSfycWn-Pxj8EPmzWjbw6aXm6Z4niZJ8K_WNyjeyZ8jEXBh77FpBK1UUbeg96Q

2️⃣ Calling Function (Insert + Verify)...
   ✅ Function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Name: [object Object]
   📋 Email: {"type":"String","value":"alice@example.com"}
   📋 Status: [object Object]
   📋 Credits: [object Object]

============================================================
📝 Function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: JZzbWlNrZKde4fecuOtHnCgjUAFkKr70Y5CMpV2ajObIkj4x42kf2P-2y9N6fkmFWPh1AjqMhjLl6F8nZAl0xg

2️⃣ Calling Function (Query + Update + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {"value":"active","type":"String"}
   📋 Name: {"type":"String","value":"Alice Smith"}

============================================================
📝 Function 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: Ttk7h2n4tkWFpowOVp3Gk1XBca3MUdD445jBYxtjGGTQNUovMPuKSD9vk4YDu93Z8sIL6m9UfnQ9xfCb8YgPXg

2️⃣ Calling Function (Query + Update Credits + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"type":"Integer","value":0}
   📋 Status: {"type":"String","value":"active"}
   📋 Name: {"type":"String","value":"Alice Smith"}

============================================================
📝 Function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: De48f8b30oUb78q5mxNzcwsJPKaO2kfqfZhs5QHHMldEmsr7hnnY6hr7pq5owdwmhhqLIVycjdG8Q3r5t58dIQ

2️⃣ Calling Function (Query + Delete + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted function: dKuZr4Zk5Oz7OLxx0qRp...
   ✅ Deleted function: JZzbWlNrZKde4fecuOtH...
   ✅ Deleted function: Ttk7h2n4tkWFpowOVp3G...
   ✅ Deleted function: De48f8b30oUb78q5mxNz...
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
=== Running ekoDB/ekodb-client/examples/javascript/document_ttl.js ===[0m
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
  Output: Document ID = ufNj738-hhpGlOGtPpFC0kBbRIceW1bmxuaCUXJysjSuhzUlym3mUZp8LsbzchQn3oIJ_81y1PGJLrkfZUTRcA
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(ufNj738-hhpGlOGtPpFC0kBbRIceW1bmxuaCUXJysjSuhzUlym3mUZp8LsbzchQn3oIJ_81y1PGJLrkfZUTRcA)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(ufNj738-hhpGlOGtPpFC0kBbRIceW1bmxuaCUXJysjSuhzUlym3mUZp8LsbzchQn3oIJ_81y1PGJLrkfZUTRcA)
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
=== Running ekoDB/ekodb-client/examples/javascript/websocket_ttl.js ===[0m
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
  Output: Document ID = XI8j-BkXaFUShmizLv5H7ydUQ4DUcdC6pOfLuJHo6pIP3sTez3fqT8cxW2zObRDH5r_RQ9AKDHnrFiGkdA52_w
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(XI8j-BkXaFUShmizLv5H7ydUQ4DUcdC6pOfLuJHo6pIP3sTez3fqT8cxW2zObRDH5r_RQ9AKDHnrFiGkdA52_w)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(XI8j-BkXaFUShmizLv5H7ydUQ4DUcdC6pOfLuJHo6pIP3sTez3fqT8cxW2zObRDH5r_RQ9AKDHnrFiGkdA52_w)
  Output: Error (expected) - Request failed with status 404: {"error":"Record has been deleted"}
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

> @ekodb/ekodb-client@0.26.1 prepare
> npm run build


> @ekodb/ekodb-client@0.26.1 build
> tsc


up to date, audited 52 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.26.1 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning JavaScript client library examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 1s

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'a-ph9PLR3yA6ROx2qUOTSRBETmWwMgGEM3-yPK2Hf9Z48we5Su49eunzPeZ-ZFNv1ssOSFnrMZqOzBVKD_BsrQ'
}

=== Find by ID ===
Found: {
  id: 'a-ph9PLR3yA6ROx2qUOTSRBETmWwMgGEM3-yPK2Hf9Z48we5Su49eunzPeZ-ZFNv1ssOSFnrMZqOzBVKD_BsrQ',
  value: { value: 42, type: 'Integer' },
  name: { value: 'Test Record', type: 'String' },
  active: { value: true, type: 'Boolean' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  id: 'a-ph9PLR3yA6ROx2qUOTSRBETmWwMgGEM3-yPK2Hf9Z48we5Su49eunzPeZ-ZFNv1ssOSFnrMZqOzBVKD_BsrQ',
  value: { type: 'Integer', value: 100 },
  name: { type: 'String', value: 'Updated Record' },
  active: { value: true, type: 'Boolean' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: Fd0uqTqPo3pRXAu-a0nrXg7y92xbeeyBIbFNtKtWYFK9NkGwnK-b1knON8REZBpHh8VU4oG9tGe1PVyRhMfByw

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
Collection created with first record: Mpk1doLgoJaKHTsy-oMF4jN0QWYHVSCFk0jJHtGDdm5GldPNmWcX3m9TWdUCJwJUhl_EppS_XmA_hwTx4ePwfg

=== List Collections ===
Total collections: 26
Sample collections: schema_documents_client_ts,schema_employees_client_ts,ttl_cache,schema_employees_client_go,test_collection

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
Retrieved value: { value: { username: 'john_doe', userId: 123 }, type: 'Object' }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { value: { name: 'Product 1', price: 29.99 }, type: 'Object' }
cache:product:2: {
  type: 'Object',
  value: { name: 'Product 2', price: 39.989999999999995 }
}
cache:product:3: {
  value: { price: 49.989999999999995, name: 'Product 3' },
  type: 'Object'
}

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 0 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 16

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 8388gaxQtK_sE3MDC0Y82OD8NnKLK305EuF_qNmtVSBbHpyuQPm2-fz0nDXQirGLcp5V8T0cH_aZEdBRq4by4w
Created Bob: $500 - ID: ZFmh5ReBlu2lE2L2xwjVCosVn0Ht6Y8U5DCgOKvtQ5jPvP-wSplLPMPkqqsw3NQrVBqN6yOWrezT3jWQ0Twh0g

=== Example 1: Begin Transaction ===
Transaction ID: 69d3b392-67b4-48a7-8f38-058899be1d98

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: b557f828-e18d-49ad-bcd8-632cab16c372
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
  1. Score: 12.870, Matched: email, name
  2. Score: 6.270, Matched: name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 13.200, Matched: title, bio
  2. Score: 13.200, Matched: title, bio
  3. Score: 13.200, Matched: bio, title
  4. Score: 13.200, Matched: bio, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400, Matched: title, bio
  2. Score: 26.400, Matched: title, bio
  3. Score: 26.400, Matched: title, bio
  4. Score: 26.400, Matched: bio, title

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 6.600, Matched: bio
  2. Score: 6.600, Matched: bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 6.600, Matched: bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.765, Matched: 
  2. Score: 0.763, Matched: 
  3. Score: 0.762, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.506, Matched: title, content
  2. Score: 0.905, Matched: content, title
  3. Score: 0.305, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: skills, bio

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
✓ Inserted document: 5iUKB3vl_8r3an3YTwpM1n_l-D-s998qAwj2ijwIviMCDKmE12jyQsrNy9d0WduHHRh0umq3rWgSCU-1eWuVHg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: KjTWGyCsy24EufCwYab8tk5Q6XcJny7WpO1utUTqPWdAvDNi_W4gyKkVM0ZqKSbBv0M4QFxSDKCANr00E7s73g

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
✓ Inserted document with TTL: esZ_-EMaD4oATYNf86HL5XgEg8qy_uJOLSId_X_sK8moGZ8eN5tLAMt_GkgQEEAxWEsd3G9Bz1GERmPHzGqO2g

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
✓ Edge cache script created: APLTP1otmMB7tvUYWXmAO2XWwz9CDd69M6BncQX3D14nxV_TcMKVUNx0dG2pne6nyzo4StclEmTl-isEjXnP0g

Call 1: Cache miss (fetches from API)
Response time: 2ms
Result: {
  "records": [
    {
      "value": {
        "value": {
          "latitude": 40.710335,
          "longitude": -73.99308,
          "timezone_abbreviation": "GMT",
          "utc_offset_seconds": 0,
          "timezone": "GMT",
          "current": {
            "interval": 900,
            "temperature_2m": 26.4,
            "time": "2026-09-07T21:45"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "°C",
            "time": "iso8601"
          },
          "elevation": 32,
          "generationtime_ms": 0.020742416381835938
        },
        "type": "Object"
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
Response time: 2ms (1x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "latitude": 40.710335,
          "longitude": -73.99308,
          "timezone_abbreviation": "GMT",
          "utc_offset_seconds": 0,
          "timezone": "GMT",
          "current": {
            "interval": 900,
            "temperature_2m": 26.4,
            "time": "2026-09-07T21:45"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "°C",
            "time": "iso8601"
          },
          "elevation": 32,
          "generationtime_ms": 0.020742416381835938
        }
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

ℹ️  Function 'get_active_users' already existed — updated instead
✅ Function saved: QeG6MJqTboXd0b7yT8sbqgZUo0e9TnLqva3MAIpsaOiIiP3ICySHL_be-JUV4dg86Qi8s9z4ZXZ_4ESi3Ed7ZA
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

ℹ️  Function 'get_users_by_status' already existed — updated instead
✅ Function saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 2 groups
   {"status":{"type":"String","value":"inactive"},"avg_score":{"value":50,"type":"Float"},"count":{"value":5,"type":"Integer"}}
   {"avg_score":{"type":"Float","value":60},"status":{"type":"String","value":"active"},"count":{"type":"Integer","value":5}}
⏱️  Execution time: 0ms

📝 Example 4: Function Management

📋 Total functions: 17
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

ℹ️  Function 'fetch_user' already existed — updated instead
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

ℹ️  Function 'fetch_and_store_user' already existed — updated instead
✅ Saved reusable function: fetch_and_store_user (uses KV)
ℹ️  Function 'swr_user' already existed — updated instead
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 5ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "address": {
        "street": "Kulas Light",
        "city": "Gwenborough",
        "geo": {
          "lat": "-37.3159",
          "lng": "8...

Second call (cache hit - from cache):
   ⏱️  Duration: 2ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "address": {
        "street": "Kulas Light",
        "city": "Gwenborough",
        "geo": {
          "lat": "-37.3159",
          "lng": "8...
   🚀 Cache speedup: 2.5x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

ℹ️  Function 'validate_user' already existed — updated instead
✅ Level 1 function: validate_user
ℹ️  Function 'fetch_slim_user' already existed — updated instead
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

✅ Function saved: JBp18BiG-VWP6tQvyr9h5gvkYaoCuS0KgCQW1UExAXaimTodLuoeaf6fa38zDS8ZSphB9fPXrZO2jCNRJX-nRA
📊 Found 2 product groups
   {"category":{"value":"Furniture","type":"String"},"count":{"type":"Integer","value":2},"avg_price":{"value":474,"type":"Float"}}
   {"count":{"type":"Integer","value":3},"avg_price":{"type":"Float","value":575.6666666666666},"category":{"type":"String","value":"Electronics"}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"count":{"type":"Integer","value":2},"category":{"type":"String","value":"Furniture"}}
   {"count":{"type":"Integer","value":3},"category":{"value":"Electronics","type":"String"}}
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

✅ Inserted order: L0elCMymegWeTYe-AYmNMMP_NqibJBnYFdKUBL6b3t6T_9mC5paClgCEzRdszUdq-J9sfgC6qIS0WDl4M10DfA
✅ Inserted 2 products with wrapped types

📝 Example 2: Function with Wrapped Type Parameters

✅ Function saved: yuFJUMlqcpYZOUAcEE_bcuBsuUq79pQt6p0vIQ2l9sNC_2kT14cBM6bkKq_NodKfkn538uXTLV3iAZ9Ca3CdcQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"value":{"role":"admin","userId":"user_abc"},"type":"Object"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: 9BTYyNWvCNn97AiQ1hTrBDKd9347up-_twDtT47zM1jTwtDrPdtAUp_AaQZe_QZojEvEFb8fwqWqSlbJ5G0Urg
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: cpzB3nWBcBmcnS514_DBDA5cLT31xUg4RZRSyan3N4LbvV_kPMCNOljjh5e_mWP_Kyg1EAosZGXF3o0pBqlpMA
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
   admin: 20.0 (3 users)
   user: 70.0 (7 users)
⏱️  Execution time: 0ms

📝 Example 4: Top Users by Score

✅ Function saved
📊 Users (showing first 5 of 10):
   1. User 2 - Score: 20
   2. User 7 - Score: 70
   3. User 10 - Score: 100
   4. User 6 - Score: 60
   5. User 3 - Score: 30
⏱️  Execution time: 0ms

📝 Example 5: User Summary Statistics

✅ Function saved
📊 User summary (2 groups):
   active users: 7 users, Total Score: 370
   inactive users: 3 users, Total Score: 180
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
   3. Natural Language Processing (AI)
   4. Database Design Principles (Database)
   5. Vector Databases Explained (Database)
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
   3. Natural Language Processing
   4. Database Design Principles
   5. Vector Databases Explained
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Function saved
📊 Projected documents (showing first 3):
   1. Getting Started with ekoDB
   2. Introduction to Machine Learning
   3. Natural Language Processing
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
   1. Bookshelf - $149 (⭐4.1)
   2. Keyboard - $89 (⭐4.4)
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
   1. Precision: Vector databases can represent data with high precision without compromising the data quality.

2. Scalability: The objects in vector databases can be easily scaled up or down without losing data integrity.

3. Compactness: Vector databases require less storage space compared to raster databases, as they only store the points, lines, or polygons that represent a feature.

4. Rich Data Representation: Vector databases can represent complex geographical features such as rivers and roads more effectively than raster equivalents. They can readily handle both discrete objects (i.e., buildings) and abstract entities (i.e., sales territories).

5. Flexibility: They allow for more advanced spatial analysis and operations, such as calculating the shortest path or nearest neighbor.

6. Interactivity: Vector databases can facilitate user interactivity such as location searching and routing due to their inherent spatial indexing capability.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding function saved
📊 Generated embeddings for 3 articles
   1. "Draft Article" - 0D vector
   2. "Getting Started with ekoDB" - 0D vector
   3. "Advanced Query Patterns" - 0D vector
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
✓ Created SWR script: fetch_api_user_js (TiB8E1myjANZshxpExwgTgdGGlnhsaTeEg_BoL9Ee_XK7FIxdEtwoPejS4ZztWMyIL0I2hUc0Q4QA7bUFkh2tg)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "value": {
          "website": "hildegard.org",
          "address": {
            "street": "Kulas Light",
            "zipcode": "92998-3874",
            "city": "Gwenborough",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "suite": "Apt. 556"
          },
          "phone": "1-770-736-8031 x56442",
          "name": "Leanne Graham",
          "username": "Bret",
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets"
          },
          "email": "Sincere@april.biz",
          "id": 1
        },
        "type": "Object"
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
Response time: 8ms (served from cache)
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
✓ Created native SWR script: github_user_native (6SGQ47exyVaGriCTedRevlcozaqNKbKuS9h7Ua6dzuqCviDLNpErrpN4lefKK5TQ5LPoaaGjN3coEbnMTT4uDA)

First call (cache miss - will fetch from GitHub API):
  Response time: 4ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 8ms
  Speedup: 0.5x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (BR04u-5WtWXqGA1NR0eEa17Qf18xOEJmn7Nz75CrHX-cpMj8McKaoqIF6qIePKklzfRW0jyI-kzB27GufM_RzA)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (mX0c_p_Rcc8VxzNYIOYkmFzE2ZhwCLlkLpRryh2s4eozb3R0JNDHxnbBc_a-Oho75pjH_qSsBmrZW6vos8mMlw)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (rXYUPTKeJHPjJq8mRG16jCaX2vs_RbuqeBfDvxhT_EzEgnsmTtMFZuVHcnmNzyjgNS8G3nBeU0nRQZQ257TepA)
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
✓ Created session: 6-dVb9zuuQ9An4rAh8d3f88LWYm3GJklkG29YY9JMmUuTSZWzdzXltvguM9QwNExUVtuImrHvQwYSJcIcLSd2A

=== Sending Chat Message ===
Message ID: GVSsHHT-LtVq1_Ctr4MsozS4GPgxf4Q7fqxxyeq_gf1FjDm-5VHZWckTGiHYmlOy0Q-Wy5dgSuFJXf6-T8Tp-Q

=== AI Response ===
Here are the available products along with their prices:

1. **ekoDB Pro**: $299
   - Description: Enterprise edition product with advanced features

2. **ekoDB Cloud**: $499
   - Description: Fully managed cloud database service product

3. **ekoDB**: $99
   - Description: A high-performance database product with AI capabilities

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    description: 'Enterprise edition product with advanced features',
    price: 299,
    id: '37909zGNsk4Qz18-jIEFlPIMsCNUXcVR5H0FytsP8nAfmNX2eJ3oNzr_jlCNsDOyNEFE17NSglxQ6duJl7qlLQ',
    name: 'ekoDB Pro'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB Cloud',
    price: 499,
    id: 'QRz3zyzYXMYszC1Zwoz2JX9NxnysSDXGq88RPovqE8OEfB8sy3ZJFEFX2rHnZuh4WaElSgwzAs2y8e91_HHjEA',
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    price: 99,
    id: 'AmXTDGLhDS9oBl0Qln6Nnq9UB2V4sT_vsFHUZO5zymutWbWltZKW22tvgc4AwhXqIe9ViEX58Z5I7JL_btWwuw',
    description: 'A high-performance database product with AI capabilities',
    name: 'ekoDB'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 1769ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 82
Total tokens: 3495

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: zM8LWo9PCwP2YyZR9ZAn97VqqxCLFoUPRH91tDBOLA2eG_7PUmLX5joXEhd1aRQpfB5pEBVnexy6QLVTHoPHDQ

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you need more information or additional products, let me know!

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
✓ Created second session: X8oeB_fKki_eLybnqkRR3M9ttpz8lgBNDBx2IC0ebUlyieP-qFO-8ONN-k8lRtU8iR6xbrUZaxNj7IWW7QIzpw
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: cAej9UWto-P6q4z_OjBPkvUGvdu2twaxnl-O1VxY2Ec5yoxcDg9DgIiOO0FLNjVYSdYtZ-J7DcntBpV_QI0JMQ

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is **ekoDB**, which is described as a high-performance database product. The price is **$99**.

✓ Message 2 sent
  Response: The price of **ekoDB** is **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 27v6n0iQgheMxUCm29oL6aXQuC0bB0RCZOF1CNu-gpj5taywmpJySUlkh0HDRp7IW_69sbpRU_gpY2RiCyyXgg
  Parent: cAej9UWto-P6q4z_OjBPkvUGvdu2twaxnl-O1VxY2Ec5yoxcDg9DgIiOO0FLNjVYSdYtZ-J7DcntBpV_QI0JMQ

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: 27v6n0iQgheMxUCm29oL6aXQuC0bB0RCZOF1CNu-gpj5taywmpJySUlkh0HDRp7IW_69sbpRU_gpY2RiCyyXgg (Untitled)
  Session 2: cAej9UWto-P6q4z_OjBPkvUGvdu2twaxnl-O1VxY2Ec5yoxcDg9DgIiOO0FLNjVYSdYtZ-J7DcntBpV_QI0JMQ (Untitled)
  Session 3: X8oeB_fKki_eLybnqkRR3M9ttpz8lgBNDBx2IC0ebUlyieP-qFO-8ONN-k8lRtU8iR6xbrUZaxNj7IWW7QIzpw (Untitled)
  Session 4: 6-dVb9zuuQ9An4rAh8d3f88LWYm3GJklkG29YY9JMmUuTSZWzdzXltvguM9QwNExUVtuImrHvQwYSJcIcLSd2A (Untitled)
  Session 5: 6RYvcxkccIYvET9JFSOmMNzfvbnZm6kzWHLQdDBVw3gjLQJNDXABj0lTKF6J4fUup43qcFsICDKLK1PWnBP_ow (Untitled)
  Session 6: B6IIPrlCRAJncGdiIVYwrU8vPsM-QTsUq6VGbpioL3_NKWXM4N_d9zRb_kUQMQlstoTx_q60VDE_1C-Mr15vhQ (Untitled)
  Session 7: sjclm-aZIqRQEaVKRC2CXdhXbzklEgeGdfCdlSgrAS5TbqDCfzNpGiRXszCUHMQnO2zG2AmHf1FG_UwX9fqSyg (Untitled)
  Session 8: 5hi3bOxBbTb2WcHkXfr7bCB4mS9f44BEJ-DAXVsqrF96OST0diI2NxRenbSDb4MEKRC9FbQwtn6_hP_p7xTt4w (Untitled)
  Session 9: Kh_2y1eqgZka3uqUF1ti21UPDhrWr12DBbgPg3rsoS8AX9-4AVxYHe8VZ1pIwPbRRRaEA6LYfPvIteEXjW8_yQ (Untitled)
  Session 10: EGq4BpwScMdsQDiBfksclFl8625dagxm3gVyxIyXJKhHxHZyinKJiOelY97gqIKri4o4BnJj2GVPWG8XuixU0Q (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 27v6n0iQgheMxUCm29oL6aXQuC0bB0RCZOF1CNu-gpj5taywmpJySUlkh0HDRp7IW_69sbpRU_gpY2RiCyyXgg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
✅ [32mJavaScript client examples complete![0m
✅ [32mAll JavaScript integration tests complete![0m
🟣 [36mBuilding Kotlin client library...[0m
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.14.4/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE
> Task :jar UP-TO-DATE
> Task :assemble UP-TO-DATE
> Task :loadKtlintReporters UP-TO-DATE
> Task :runKtlintCheckOverKotlinScripts UP-TO-DATE
> Task :ktlintKotlinScriptCheck UP-TO-DATE
> Task :runKtlintCheckOverMainSourceSet UP-TO-DATE
> Task :ktlintMainSourceSetCheck UP-TO-DATE
> Task :runKtlintCheckOverTestSourceSet UP-TO-DATE
> Task :ktlintTestSourceSetCheck UP-TO-DATE
> Task :compileTestKotlin UP-TO-DATE
> Task :compileTestJava NO-SOURCE
> Task :processTestResources NO-SOURCE
> Task :testClasses UP-TO-DATE
> Task :test UP-TO-DATE
> Task :check UP-TO-DATE
> Task :build UP-TO-DATE

BUILD SUCCESSFUL in 4s
11 actionable tasks: 11 up-to-date
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
✓ Inserted user: EkoRecord(fields={id=StringValue(value=lX_OzBVtzNCorBkHQSZEU8dAcYPj9ii-07GRv7FvMWlcZZiDxnQ_i6dhJqsXkTU6xZSE9RsD9HwhlP_3p9MBlw)})
  User ID: lX_OzBVtzNCorBkHQSZEU8dAcYPj9ii-07GRv7FvMWlcZZiDxnQ_i6dhJqsXkTU6xZSE9RsD9HwhlP_3p9MBlw

=== Read ===
✓ Found user by ID: EkoRecord(fields={email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=28)}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), created_at=ObjectValue(value={value=StringValue(value=Mon Sep 07 17:49:32 EDT 2026), type=StringValue(value=String)}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)]), type=StringValue(value=Array)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={nested=ObjectValue(value={deep=BooleanValue(value=true)}), key=StringValue(value=value)})}), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), embedding=ObjectValue(value={value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)]), type=StringValue(value=Array)}), id=StringValue(value=lX_OzBVtzNCorBkHQSZEU8dAcYPj9ii-07GRv7FvMWlcZZiDxnQ_i6dhJqsXkTU6xZSE9RsD9HwhlP_3p9MBlw), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Alice Johnson
  email (String): alice@example.com
  age (Integer): 28
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): Mon Sep 07 17:49:32 EDT 2026
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1, tag2, tag3]
  metadata (Object): {nested={deep=true}, key=value}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): [electronics, computers]
  data (Bytes): 11 bytes
Record fields: email, age, price, name, created_at, tags, metadata, user_id, embedding, id, data, categories, active

=== Update ===
✓ Updated user: EkoRecord(fields={embedding=ObjectValue(value={value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)]), type=StringValue(value=Array)}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)]), type=StringValue(value=Array)}), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), metadata=ObjectValue(value={value=ObjectValue(value={nested=ObjectValue(value={deep=BooleanValue(value=true)}), key=StringValue(value=value)}), type=StringValue(value=Object)}), price=ObjectValue(value={value=FloatValue(value=99.99), type=StringValue(value=Float)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), id=StringValue(value=lX_OzBVtzNCorBkHQSZEU8dAcYPj9ii-07GRv7FvMWlcZZiDxnQ_i6dhJqsXkTU6xZSE9RsD9HwhlP_3p9MBlw), created_at=ObjectValue(value={value=StringValue(value=Mon Sep 07 17:49:32 EDT 2026), type=StringValue(value=String)}), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), city=ObjectValue(value={value=StringValue(value=San Francisco), type=StringValue(value=String)}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])})})

=== Query ===
✓ Found 1 users matching query
  - EkoRecord(fields={city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), id=StringValue(value=lX_OzBVtzNCorBkHQSZEU8dAcYPj9ii-07GRv7FvMWlcZZiDxnQ_i6dhJqsXkTU6xZSE9RsD9HwhlP_3p9MBlw), categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), price=ObjectValue(value={value=FloatValue(value=99.99), type=StringValue(value=Float)}), data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), embedding=ObjectValue(value={value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)]), type=StringValue(value=Array)}), created_at=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Mon Sep 07 17:49:32 EDT 2026)}), metadata=ObjectValue(value={value=ObjectValue(value={nested=ObjectValue(value={deep=BooleanValue(value=true)}), key=StringValue(value=value)}), type=StringValue(value=Object)})})

=== Delete ===
✓ Deleted user with ID: lX_OzBVtzNCorBkHQSZEU8dAcYPj9ii-07GRv7FvMWlcZZiDxnQ_i6dhJqsXkTU6xZSE9RsD9HwhlP_3p9MBlw

✓ Confirmed user was deleted

=== Cleanup ===
✓ Deleted collection: kotlin_users_example

=== Example Complete ===

BUILD SUCCESSFUL in 7s
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
  {"data":[{"id":"xzBpZeSsRc4LqFd-mvhY4_-6HX1KjZEl3Jlo06AdMr4K2T-xtvStSIGnVQOlMDIWrqGj_I8nWTXmaNzKlpigCw","name":{"value":"Test User","type":"String"},"status":{"value":"active","type":"String"}}]}

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
  IDs: POVaDzpkv-tRHOmgc8lDBiC8DtqWAm-mFxev0qYHVVgDIbAyyaBO-BVYvAuvfecsDAAFHkiwEL34GPjHHriwEw, Dmzz5qiyKNZfpcy9vio69qsB5L-RtSm3U0QGwV3Vzk6p76lFV8tvt9Rck9Hu-vNePV_fnVqOgrnFNbbGkySbtQ, zfpFNQ_lPiQdoOd-OQdEWvamUvcp-efC0txh4gPnwkqoAPk3hh81LeLFSbCtds7STyVvla8VOJArvSStz0lyWA...

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
✓ Found 30 collections
  - schema_documents_client_ts
  - schema_employees_client_ts
  - ttl_cache
  - schema_employees_client_go
  - test_collection

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
✓ Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
✓ Schema: {"fields":{"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2026-09-07T21:49:49.504041Z","last_modified":"2026-09-07T21:49:49.504041Z","bypass_ripple":false,"primary_key_alias":"id"}

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
✓ Retrieved value: {"value":{"name":"Alice","email":"alice@example.com","role":"admin"},"type":"Object"}

=== KV Set with TTL ===
✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key ===
✓ Session value: {"type":"Object","value":{"user_id":"123","created_at":1788817793901}}
  (Will expire in 10 seconds)

=== KV Batch Set ===
✓ Batch set 3 keys
  config:db: success
  config:cache: success
  config:api: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  config:db: {"value":{"host":"localhost","port":5432}}
  config:cache: {"value":{"enabled":true,"ttl":3600}}
  config:api: {"value":{"timeout":30,"retries":3}}

=== KV Exists ===
✓ Key exists: true

=== KV Find (Pattern Query) ===
✓ Found 3 keys matching 'config:.*'

=== KV Query (Alias for Find) ===
✓ Total keys in store: 17

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
Created Alice: $1000 - ID: MHznQGuBO0DZ1hw-GBosHo-QxOOEvTY-5_Q90CRbI6f59JoWg_YGB6ywR1lUeh-YRDnN_dEXUxMtCfX6pEzWXA
Created Bob: $500 - ID: uoaJ6v-49GRDpAehZvJ40UlxnU9avDMzIpsRZzWE65tsXLLwcnU7JrtPwP1Se0Z3M6SoTQYW2JLj5sMUyOvlMA

=== Example 1: Begin Transaction ===
Transaction ID: f4fcdf3b-2a4f-4d67-9edd-2bcdbf9d13b6

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 289ed433-ce20-41ce-8c69-bcef042f936b
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
  - Score: ObjectValue(value={value=IntegerValue(value=95), type=StringValue(value=Integer)})
  - Score: ObjectValue(value={value=IntegerValue(value=92), type=StringValue(value=Integer)})
  - Score: ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=88)})

=== Query 4: Complex (score > 80 AND age >= 25) ===
✓ Found 4 high-scoring adults

=== Query 5: IN (city IN ['NYC', 'LA']) ===
✓ Found 4 records in NYC or LA

=== Query 6: Pagination (skip 2, limit 2) ===
✓ Page 2 (2 records):
  - ObjectValue(value={value=StringValue(value=Charlie), type=StringValue(value=String)})
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
  {"results":[{"record":{"category":{"value":"programming","type":"String"},"tags":{"value":"programming,rust,tutorial","type":"String"},"id":"-cXO9cogexVFby2zTFU0tnlYZnx_5vTrTDW2opj6rZuKXtsuk9oKhogEzWF5BTpO0gSKWl_tgGPHUtBIcxjEug","views":{"type":"Integer","value":983},"description":{"type":"String","value":"Learn Rust programming language with hands-on examples and best practices."},"title":{"value":"Rust Programming","type":"String"}},"score":26.400000000000002,"matched_fields":["description","category","title","tags"]},{"record":{"description":{"type":"String","value":"Build modern web applications using JavaScript, React, and Node.js."},"tags":{"type":"String","value":"programming,javascript,web"},"title":{"type":"String","value":"JavaScript Web Development"},"views":{"type":"Integer","value":764},"id":"M-5PQNVUryHrDBCHvsVSHz-0OD6ezcGYjEN7RTXhyXNE0g3ydOR0xX18rEsO9xw5lytPZFIBwWsoVALj157BcA","category":{"value":"programming","type":"String"}},"score":13.200000000000001,"matched_fields":["tags","category"]},{"record":{"category":{"value":"programming","type":"String"},"id":"FWwgwmUphUU35eHh5OZkM_D-yxcytzUPLS0Nu2T-ji2v-1x-tywJg-SI2Cq1l_5GrLOqsVBQBjK591kQbOzN2g","tags":{"type":"String","value":"programming,python,data-science"},"views":{"type":"Integer","value":236},"title":{"value":"Python for Data Science","type":"String"},"description":{"value":"Master Python for data analysis, machine learning, and visualization.","type":"String"}},"score":13.200000000000001,"matched_fields":["category","tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' ===
✓ Found results for 'machine learning'
  {"results":[{"record":{"views":{"value":445,"type":"Integer"},"description":{"value":"Introduction to machine learning algorithms and neural networks.","type":"String"},"title":{"type":"String","value":"Machine Learning Basics"},"category":{"type":"String","value":"ai"},"tags":{"type":"String","value":"ai,machine-learning,python"},"id":"1IYPG5xeWEc9vjCZpmi1u2eoNwScdfkFJIhmrXkV3my0qk-aCGb2tQjijXltLVxkjsGJr-eGDKzHWjNfX2Dopw"},"score":2.7,"matched_fields":["title","tags","description"]},{"record":{"tags":{"type":"String","value":"programming,python,data-science"},"title":{"value":"Python for Data Science","type":"String"},"description":{"type":"String","value":"Master Python for data analysis, machine learning, and visualization."},"category":{"type":"String","value":"programming"},"id":"FWwgwmUphUU35eHh5OZkM_D-yxcytzUPLS0Nu2T-ji2v-1x-tywJg-SI2Cq1l_5GrLOqsVBQBjK591kQbOzN2g","views":{"type":"Integer","value":236}},"score":1.0,"matched_fields":["description"]},{"record":{"description":{"value":"Learn Rust programming language with hands-on examples and best practices.","type":"String"},"category":{"type":"String","value":"programming"},"views":{"value":983,"type":"Integer"},"title":{"value":"Rust Programming","type":"String"},"tags":{"value":"programming,rust,tutorial","type":"String"},"id":"-cXO9cogexVFby2zTFU0tnlYZnx_5vTrTDW2opj6rZuKXtsuk9oKhogEzWF5BTpO0gSKWl_tgGPHUtBIcxjEug"},"score":0.5,"matched_fields":["description"]},{"record":{"description":{"value":"Learn database design principles, normalization, and query optimization.","type":"String"},"category":{"type":"String","value":"database"},"title":{"type":"String","value":"Database Design"},"id":"C-eFX1E1-RynI-PIo9sXXtpzOlQtj-kB1IJ-pHnMCX3ps1yX8lphaho--4o_lTCMF4JJZ_BOVe1gvx8jwfLpGg","views":{"type":"Integer","value":23},"tags":{"value":"database,design,sql","type":"String"}},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

=== Search with a metadata pre-filter (category = programming) ===
✓ Found results in category 'programming' (database/ai excluded)
  {"results":[{"record":{"category":{"type":"String","value":"programming"},"views":{"value":983,"type":"Integer"},"tags":{"value":"programming,rust,tutorial","type":"String"},"id":"-cXO9cogexVFby2zTFU0tnlYZnx_5vTrTDW2opj6rZuKXtsuk9oKhogEzWF5BTpO0gSKWl_tgGPHUtBIcxjEug","title":{"value":"Rust Programming","type":"String"},"description":{"value":"Learn Rust programming language with hands-on examples and best practices.","type":"String"}},"score":6.6000000000000005,"matched_fields":["description"]},{"record":{"tags":{"type":"String","value":"programming,python,data-science"},"title":{"type":"String","value":"Python for Data Science"},"category":{"type":"String","value":"programming"},"views":{"type":"Integer","value":236},"description":{"type":"String","value":"Master Python for data analysis, machine learning, and visualization."},"id":"FWwgwmUphUU35eHh5OZkM_D-yxcytzUPLS0Nu2T-ji2v-1x-tywJg-SI2Cq1l_5GrLOqsVBQBjK591kQbOzN2g"},"score":6.6000000000000005,"matched_fields":["description"]}],"total":2,"execution_time_ms":0}

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
✓ Inserted user 1: StringValue(value=DURxphptp5UyZEwE_kzF1_d6jUjwxbNdciBU0sAgqTu9atyh6mVVvI5PEWm8F_vLyjrQF5OSFgBpkuj3VySVxw)
✓ Inserted user 2: StringValue(value=mHWsTYoo53LjFfUhIdH0pea5GzxhAbJBRtOomG13KbJrR5ewbLRyXOlEoHFN7CRiW614tQlWN_n8set1NphtkQ)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections ===
✓ Total collections: 31
  Sample: [schema_documents_client_ts, schema_employees_client_ts, ttl_cache, schema_employees_client_go, test_collection]

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
=== ekoDB Kotlin Client - Document TTL Example ===

=== Insert with TTL ===
✓ Inserted document with 10s TTL
  Document ID: JvdElZ3d66tKRstyVW35LNbzCAQzxZMRggqmEDos1BDXX0mBjDALhV2Zggt3JvhaZJ-1VfCqtLvkP0Ch8ZQUlw

=== Verify Document Exists ===
✓ Document found: session_id, user_id, id, created_at, ttl

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: TBQGkgvQa4-lKR082vKRkYVQYjEL2qSRYGdz0kgLN2fS7vuDkNr7GMEKiYsUp2MLJ-TyoYlaem52FAgkHqzlDQ

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
✓ Inserted document with TTL: StringValue(value=kmELxHJdojeDnbzrz6XBfi42nlzWct3JPPlWPwwNp73NFUqnwaO5K65EBXC1pWHiGAICs1KMjqXwqV7Q-flyZA)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"data":[{"created_at":{"value":1788817828754,"type":"Integer"},"value":{"type":"Integer","value":42},"name":{"type":"String","value":"WebSocket TTL Test"},"id":"kmELxHJdojeDnbzrz6XBfi42nlzWct3JPPlWPwwNp73NFUqnwaO5K65EBXC1pWHiGAICs1KMjqXwqV7Q-flyZA","ttl":"2026-09-07T22:50:28.855020Z"}]}

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
✓ Edge cache function created: icrYFn0w01TCK5W69efxjDTHNR-vT0O2FTX9dYI3XLQIToDspf08izQ5Rjeo_3htbubhjBOrsfAj2whPlreLjw

Call 1: Cache lookup
Found 1 cached entries
Response time: 18ms

Call 2: Cache lookup (connection warm)
Found 1 cached entries
Response time: 5ms

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

✅ Function saved: Mmbut7SPb_4bOWBA-uGJJ3eWoY0nAlZseXwuTxKl-Ld3DLRiEeTg-oVvfG6Xoczip0sE7m_EDW5oMS71bnvi6g
📊 Found 15 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

✅ Function saved
📊 Found 15 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 3 groups
   {"count":{"value":5,"type":"Integer"},"avg_score":{"type":"Float","value":60.0},"status":{"type":"String","value":"active"}}
   {"count":{"value":5,"type":"Integer"},"avg_score":{"type":"Float","value":60.0},"status":{"type":"String","value":"null"}}
   {"avg_score":{"type":"Float","value":50.0},"status":{"type":"String","value":"inactive"},"count":{"value":5,"type":"Integer"}}
⏱️  Execution time: 0ms

📝 Example 4: function Management

📋 Total functions: 16
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

ℹ️  Function 'count_users' already existed — updated instead
✅ Count function saved
📊 Total user count: {"type":"Integer","value":15}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Deleted collection
✅ Deleted test functions

✅ All examples completed successfully!

💡 Key Advantages of Using the Client:
   • Automatic token management
   • Type-safe Stage builders
   • Built-in error handling

BUILD SUCCESSFUL in 7s
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

✅ Function saved: gOE0SVsvFn0KDJEqPVZTfqnIukxMutXxANGxqHGdmMFXz8VGY8Jn5kvCR17Pm3F6PXptbTj-WsnHHkC_xQ2RQA
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

✅ Inserted order: StringValue(value=OuO3Y7wrDOUJgFo4E7cq2XyRNlGxjJHYq_XWTLRKm18L7yLxs2ax_NRsIxvU_geqaKuq_aWysBWeLwpZvV9KuA)
✅ Inserted 2 products

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: IEH266791-PnOSpaKV8q1xEh0K_kJ3bXoSB0grF24CpWUJozX8gKrLLoWp2M-_QCD5KhlnfyQKfY1K9wb1BR-A
📊 Created order via function
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"type":"Object","value":{"userId":"user_abc","role":"admin"}}
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: Xy4-CU5N4bf_3qo3M5hpDmH-Jk8wRf9bpNjCnLN1J8g7340Qhf4Fgf_nTk0XYeXpKwIuxEbB4lk2OyANeIY1jQ
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: E7v2w2SsK5jpm-pNVRuAKQgWxcrqEe6AoOyOfjR49HOCcR5pLJERI4kU4iN-ZqFAYXtSgB1ec1lZR5HdD-Kleg
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
✓ Created SWR function: swr_cache_lookup_kt (ZrSskB3jeHi5IUXNfgrAXvOqPyWjE49KkifY8atDbIp2atAz4PjqmLbkTFlLJdj4kWuHeB7BI6_6aziSXRiuXg)

Step 3: First call - Cache lookup
Found 1 cached entries
✓ Cache lookup complete

Step 4: Second call - Fast cache hit
Response time: 4ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation

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
✓ Created native SWR function: github_user_native (brQkB-nBjSWaHE4a-jhDbxdByq0Q1bG0JlN9D5-qH4ej4wsvw5ODOZU2S3nNK4AqBUY9gpe07Nf5miOv6pEWSw)

First call (cache miss - will fetch from GitHub API):
  Response time: 19ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 12ms
  Speedup: 1.6x faster 🚀


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR function with audit trail: product_swr_audit (kpc6HoXqvS5WMcwoSqBa2d5sPigHeNEr5islCxYzQg98QfcMC1mcTZOsOa0OsyEaKGpc3EmrNcxj5LCniW0R4w)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (8KgxY41BnkPjThBDs3zYpSK_3nAMxOxyiLXo2hYnK8at79ZBo3EN6m_IH1-5yD2fA9c2RCOmufQ_N7pXEsEVZA)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL function: flexible_cache (6A5J-OXNTCU9u6Dh_yo7Gdy9y2cXgR0k7Kz-qrFpQ8ubDuFYXCytjzED3cQ7osJM9_ISyZOVW5qGEqFgDl3tWA)
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
   {"count":{"type":"Integer","value":3},"category":{"type":"String","value":"Furniture"},"avg_price":{"value":365.6666666666667,"type":"Float"}}
   {"category":{"value":"Electronics","type":"String"},"count":{"value":5,"type":"Integer"},"avg_price":{"type":"Float","value":367.0}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced function examples finished!

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
🚀 ekoDB Kotlin AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat function saved
🤖 AI Response:
   {"response":{"type":"String","value":"Vector databases offer several benefits:\n\n1. **Efficient Similarity Search**: They enable fast retrieval of similar items using vector representations, making them ideal for applications like recommendation systems and image retrieval.\n\n2. **High-dimensional Data Handling**: They are optimized for high-dimensional spaces, which is crucial for managing complex data types like text, images, and audio.\n\n3. **Scalability**: Vector databases can scale to handle large datasets efficiently, supporting distributed architectures.\n\n4. **Real-time Processing**: Many vector databases support real-time querying and updates, allowing for dynamic data interactions.\n\n5. **Integration with Machine Learning**: They seamlessly integrate with machine learning pipelines, as they inherently support vector embeddings commonly used in AI applications.\n\n6. **Flexibility**: Suitable for various use cases, from semantic search to anomaly detection, as they can capture nuanced relationships in data.\n\n7. **Improved Accuracy**: By using embeddings, they can improve the accuracy of searches and recommendations compared to traditional databases."},"tokens_used":{"type":"Integer","value":232}}
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
   {"count":{"type":"Integer","value":7},"status":{"type":"String","value":"active"}}
   {"status":{"value":"inactive","type":"String"},"count":{"type":"Integer","value":3}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD function examples finished!

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
🚀 ekoDB Kotlin Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. {"value":"Getting Started with ekoDB","type":"String"} ({"type":"String","value":"Database"})
   2. {"type":"String","value":"Introduction to Machine Learning"} ({"value":"AI","type":"String"})
   3. {"type":"String","value":"Database Design Principles"} ({"type":"String","value":"Database"})
   4. {"type":"String","value":"Vector Databases Explained"} ({"type":"String","value":"Database"})
   5. {"type":"String","value":"Natural Language Processing"} ({"type":"String","value":"AI"})
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"count":{"value":2,"type":"Integer"},"category":{"value":"AI","type":"String"}}
   {"count":{"type":"Integer","value":3},"category":{"type":"String","value":"Database"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search function examples finished!

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
=== ekoDB Kotlin Client - Basic Chat Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: HfTPhViM_PJ9UiTmeX8eM8f11OtA-8MN0wZ-5tXKbPa_PG9gRPBW1Y2H3HF0jpIW8FNg6RmZOPVS7VC7m4yrOw

=== Sending Chat Message ===
✓ Chat response:
  Message ID: "yJT3zkAVUshodbkDBWYkJqOmkE_mclM80Eeh085Bu5JkfmD-V1GhryXWfP0NHu8HL1ENidTn7GXcikHqaxjuQg"
  Responses: ["ekoDB is a high-performance database that integrates intelligent caching, real-time capabilities, and AI features to enhance its functionality. \n\nSome key features of ekoDB include:\n\n1. **AI Chat Integration**: This allows users to query the database using natural language and receive AI-powered responses with relevant context.\n2. **Search Capabilities**: ekoDB supports full-text search, vector search, and hybrid search, enabling automatic context retrieval during query execution.\n\nThese features combine to provide a powerful and flexible database solution that enhances user interaction and data retrieval."]

=== Cleanup ===
✓ Deleted chat session
✓ Deleted collection: kotlin_chat_basic_example

✓ Basic chat example completed successfully

BUILD SUCCESSFUL in 5s
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
✓ Created session: 33Ox51YmC1n65Q4kGkU02-lZ0Vze3nfpqCfbzC86yaEH-9ad929dtJ3Iy6x2ozB4--M-YYDbPX23n29mICaHWw

=== Sending Initial Message ===
✓ Message sent
  Responses: ["It seems there are currently no products available in the database. The query returned zero records. If you need assistance with anything else or would like to add new products, feel free to let me know!"]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["The price of ekoDB is $99."]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: tKNSqjodSQ0Mbm3Z-zVgrkraN9a7f9n-Ly9U22QFCQEQVGzP2ux-6dumy_2yt-hXidNWZ3o8ThBtF9WJRfa36g

=== Merging Sessions ===
✓ Merged sessions
  Total messages in merged session: 5

=== Deleting Message ===
✓ Deleted message

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_advanced_example

✓ Advanced chat features example completed successfully

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
=== ekoDB Kotlin Client - Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: ilU4gi-NaQRhgqxAWViWPQuvyW0B3qRTjmlHdJrPcWxYCY7Otb7N8Hgtjd3TVexYt5Doywuzh7zTTGODLhTOMA

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["The available product is:\n\n- **Product Name**: ekoDB\n- **Description**: A high-performance database product with AI capabilities\n- **Price**: $99\n\nIf you have any more questions or need further information, feel free to ask!"]

✓ Message 2 sent
  Responses: ["The price of the product ekoDB is $99."]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"chat_id":{"type":"String","value":"ilU4gi-NaQRhgqxAWViWPQuvyW0B3qRTjmlHdJrPcWxYCY7Otb7N8Hgtjd3TVexYt5Doywuzh7zTTGODLhTOMA"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"Vy6XfjsIWltLiJuZGNBsYyUcV0xzAsPBvrz62VSKhLpRZD8qRqJ0SpZze_0CU5LJNMUsghqM4IrtXhdMtO656Q","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-09-07T21:51:43.719947+00:00"},"id":"_Wvbk5emPMhNhsaEkoMkbIT8OmTxha-lKdBNR4_3gPrlsX5A_68VkTx0b4NBCec_A5VVYqVNgCVuobWCUsth9Q","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":116,"prompt_tokens":6740,"total_tokens":6856}},"updated_at":{"type":"DateTime","value":"2026-09-07T21:51:43.719947+00:00"}},{"chat_id":{"type":"String","value":"ilU4gi-NaQRhgqxAWViWPQuvyW0B3qRTjmlHdJrPcWxYCY7Otb7N8Hgtjd3TVexYt5Doywuzh7zTTGODLhTOMA"},"content":{"type":"String","value":"The available product is:\n\n- **Product Name**: ekoDB\n- **Description**: A high-performance database product with AI capabilities\n- **Price**: $99\n\nIf you have any more questions or need further information, feel free to ask!"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"Vy6XfjsIWltLiJuZGNBsYyUcV0xzAsPBvrz62VSKhLpRZD8qRqJ0SpZze_0CU5LJNMUsghqM4IrtXhdMtO656Q","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-09-07T21:51:43.738122+00:00"},"id":"DB-UNhdSa42I2oc1aEXx_nr0Dux73_zBVrHeg8oreKdEconkpklzE4xMGaLYVtv7CKx2ed8EzOl5TQNGmFt9hw","llm_model":{"type":"String","value":"gpt-4o-mini"},"llm_provider":{"type":"String","value":"openai"},"role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":116,"prompt_tokens":6740,"total_tokens":6856}},"tool_call_count":{"type":"Number","value":2},"tool_call_history":{"type":"Object","value":{"iterations":2,"tool_calls":[{"arguments":{"collection":"kotlin_chat_sessions_example","filter":{"content":{"expressions":[{"content":{"field":"price","operator":"Gte","value":0},"type":"Condition"}],"operator":"And"},"limit":50,"select_fields":["product","price"],"type":"Logical"}},"id":"call_jsH9UiTuTt4q7xiyzKlablYw","name":"query_collection"}],"tool_results":[{"error":null,"result":{"count":1,"records":[{"description":"A high-performance database product with AI capabilities","id":"Vy6XfjsIWltLiJuZGNBsYyUcV0xzAsPBvrz62VSKhLpRZD8qRqJ0SpZze_0CU5LJNMUsghqM4IrtXhdMtO656Q","price":99,"product":"ekoDB"}]},"success":true,"tool_call_id":"call_jsH9UiTuTt4q7xiyzKlablYw","tool_name":"query_collection"}]}},"updated_at":{"type":"DateTime","value":"2026-09-07T21:51:43.738122+00:00"}},{"chat_id":{"type":"String","value":"ilU4gi-NaQRhgqxAWViWPQuvyW0B3qRTjmlHdJrPcWxYCY7Otb7N8Hgtjd3TVexYt5Doywuzh7zTTGODLhTOMA"},"content":{"type":"String","value":"What is the price?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description","product","price"],"record":{"description":"A high-performance database product with AI capabilities","id":"Vy6XfjsIWltLiJuZGNBsYyUcV0xzAsPBvrz62VSKhLpRZD8qRqJ0SpZze_0CU5LJNMUsghqM4IrtXhdMtO656Q","price":99,"product":"ekoDB"},"score":2.0928571428571425}]},"created_at":{"type":"DateTime","value":"2026-09-07T21:51:44.588303+00:00"},"id":"SEQJ5_DvktCUwH8y7cQSgmfwuPc-NGu2Jhscrxv6rghmENZHZRoJI688yzTIuIdBf6FdywNt-TxsiEAUGp-unw","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":9,"prompt_tokens":3388,"total_tokens":3397}},"updated_at":{"type":"DateTime","value":"2026-09-07T21:51:44.588303+00:00"}},{"chat_id":{"type":"String","value":"ilU4gi-NaQRhgqxAWViWPQuvyW0B3qRTjmlHdJrPcWxYCY7Otb7N8Hgtjd3TVexYt5Doywuzh7zTTGODLhTOMA"},"content":{"type":"String","value":"The price of the product ekoDB is $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description","product","price"],"record":{"description":"A high-performance database product with AI capabilities","id":"Vy6XfjsIWltLiJuZGNBsYyUcV0xzAsPBvrz62VSKhLpRZD8qRqJ0SpZze_0CU5LJNMUsghqM4IrtXhdMtO656Q","price":99,"product":"ekoDB"},"score":2.0928571428571425}]},"created_at":{"type":"DateTime","value":"2026-09-07T21:51:44.606120+00:00"},"id":"YRLNYFOLueuGgRrTweUM95LuomUvyHidZIoPTPo0v8-5QHZGI_AP7ri4n9DdzKZ2qD_f5vVd5N1geNca_MmgEg","llm_model":{"type":"String","value":"gpt-4o-mini"},"llm_provider":{"type":"String","value":"openai"},"role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":9,"prompt_tokens":3388,"total_tokens":3397}},"updated_at":{"type":"DateTime","value":"2026-09-07T21:51:44.606120+00:00"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 16

=== Branching Session ===
✓ Created branched session: o8pDCw6KEFtSS9-y_VsQLDatWOx1wlqT61W02qAx5yyyEGLP0S_GawskaPzsfXV1_ijs9Bg_BURj7JZ6mIkiqg

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_sessions_example

✓ Chat session management example completed successfully

BUILD SUCCESSFUL in 7s
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
    - text-embedding-ada-002
    - whisper-1
    - gpt-3.5-turbo
    - tts-1
    - gpt-3.5-turbo-16k
    - gpt-4-0613
    - gpt-4
    - davinci-002
    - babbage-002
    - gpt-3.5-turbo-instruct
    - gpt-3.5-turbo-instruct-0914
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
    - omni-moderation-latest
    - omni-moderation-2024-09-26
    - o1-2024-12-17
    - o1
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
    - gpt-5.4
    - gpt-5.4-nano-2026-03-17
    - gpt-5.4-nano
    - gpt-5.4-mini-2026-03-17
    - gpt-5.4-mini
    - gpt-image-2
    - gpt-image-2-2026-04-21
    - gpt-5.5
    - gpt-5.5-2026-04-23
    - gpt-5.5-pro
    - gpt-5.5-pro-2026-04-23
    - chat-latest
    - gpt-realtime-translate
    - gpt-realtime-2
    - gpt-realtime-whisper
    - gpt-5.6-sol
    - gpt-5.6-terra
    - gpt-5.6-luna
    - gpt-realtime-2.1
    - gpt-realtime-2.1-mini
    - gpt-transcribe
    - gpt-live-transcribe
    - gpt-6-astra
  anthropic:
    - claude-fable-5-1
    - claude-opus-5
    - claude-sonnet-5
    - claude-fable-5
    - claude-opus-4-8
    - claude-opus-4-7
    - claude-sonnet-4-6
    - claude-opus-4-6
    - claude-opus-4-5-20251101
    - claude-haiku-4-5-20251001
    - claude-sonnet-4-5-20250929
Provider status:
  anthropic: ok 11 models
  gemini: not_configured (unverified) No Gemini API Key
  openai: ok 127 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get OpenAI Models ===
OpenAI models: text-embedding-ada-002, whisper-1, gpt-3.5-turbo, tts-1, gpt-3.5-turbo-16k, gpt-4-0613, gpt-4, davinci-002, babbage-002, gpt-3.5-turbo-instruct, gpt-3.5-turbo-instruct-0914, gpt-3.5-turbo-1106, tts-1-hd, tts-1-1106, tts-1-hd-1106, text-embedding-3-small, text-embedding-3-large, gpt-3.5-turbo-0125, gpt-4-turbo, gpt-4-turbo-2024-04-09, gpt-4o, gpt-4o-2024-05-13, gpt-4o-mini-2024-07-18, gpt-4o-mini, gpt-4o-2024-08-06, omni-moderation-latest, omni-moderation-2024-09-26, o1-2024-12-17, o1, o3-mini, o3-mini-2025-01-31, gpt-4o-2024-11-20, gpt-4o-mini-search-preview-2025-03-11, gpt-4o-mini-search-preview, gpt-4o-transcribe, gpt-4o-mini-transcribe, o1-pro-2025-03-19, o1-pro, gpt-4o-mini-tts, o3-2025-04-16, o4-mini-2025-04-16, o3, o4-mini, gpt-4.1-2025-04-14, gpt-4.1, gpt-4.1-mini-2025-04-14, gpt-4.1-mini, gpt-4.1-nano-2025-04-14, gpt-4.1-nano, gpt-image-1, o4-mini-deep-research, gpt-4o-transcribe-diarize, o4-mini-deep-research-2025-06-26, gpt-5-chat-latest, gpt-5-2025-08-07, gpt-5, gpt-5-mini-2025-08-07, gpt-5-mini, gpt-5-nano-2025-08-07, gpt-5-nano, gpt-audio-2025-08-28, gpt-realtime, gpt-realtime-2025-08-28, gpt-audio, gpt-5-codex, gpt-image-1-mini, gpt-5-pro-2025-10-06, gpt-5-pro, gpt-audio-mini, gpt-audio-mini-2025-10-06, gpt-5-search-api, gpt-realtime-mini, sora-2, sora-2-pro, gpt-5-search-api-2025-10-14, gpt-5.1-chat-latest, gpt-5.1-2025-11-13, gpt-5.1, gpt-5.1-codex, gpt-5.1-codex-mini, gpt-5.1-codex-max, gpt-image-1.5, gpt-5.2-2025-12-11, gpt-5.2, gpt-5.2-pro-2025-12-11, gpt-5.2-pro, gpt-5.2-chat-latest, gpt-4o-mini-transcribe-2025-12-15, gpt-4o-mini-transcribe-2025-03-20, gpt-4o-mini-tts-2025-03-20, gpt-4o-mini-tts-2025-12-15, gpt-realtime-mini-2025-12-15, gpt-audio-mini-2025-12-15, chatgpt-image-latest, gpt-5.2-codex, gpt-5.3-codex, gpt-realtime-1.5, gpt-audio-1.5, gpt-4o-search-preview, gpt-4o-search-preview-2025-03-11, gpt-5.3-chat-latest, gpt-5.4-2026-03-05, gpt-5.4-pro, gpt-5.4-pro-2026-03-05, gpt-5.4, gpt-5.4-nano-2026-03-17, gpt-5.4-nano, gpt-5.4-mini-2026-03-17, gpt-5.4-mini, gpt-image-2, gpt-image-2-2026-04-21, gpt-5.5, gpt-5.5-2026-04-23, gpt-5.5-pro, gpt-5.5-pro-2026-04-23, chat-latest, gpt-realtime-translate, gpt-realtime-2, gpt-realtime-whisper, gpt-5.6-sol, gpt-5.6-terra, gpt-5.6-luna, gpt-realtime-2.1, gpt-realtime-2.1-mini, gpt-transcribe, gpt-live-transcribe, gpt-6-astra

=== Get Anthropic Models ===
Anthropic models: claude-fable-5-1, claude-opus-5, claude-sonnet-5, claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Request failed with status 404: {"error":"Unknown provider 'nonexistent_provider_xyz'","error_kind":"unknown_provider"}

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
Created user function with ID: DRAYfEAU_mmKDcBG2Jl1lv-qHhUr08yaJWH4yo9YtCjZxZoPLKsQVgBI3ZpVYhqiOI2J7LEUln52U4a_Z_dfAQ

=== Get User Function ===
Retrieved: "get_active_users_kt" - "Get Active Users (Kotlin)"
Description: "Fetches all users and filters by active status"

=== List All User Functions ===
Found 11 user functions:
  - "validate_user": "Check if user exists"
  - "cache_api_call": "Cache External API Call"
  - "get_active_users_kt": "Get Active Users (Kotlin)"
  - "fetch_and_store_user": "Fetch user from API and cache in KV"
  - "fetch_user": "Fetch user by code"

=== List User Functions by Tag ===
Found 11 user functions with 'kotlin' tag:
  - "validate_user"
  - "cache_api_call"
  - "get_active_users_kt"
  - "fetch_and_store_user"
  - "fetch_user"
  - "cache_api_call_js"
  - "fetch_api_user"
  - "swr_user"
  - "cache_api_call_py"
  - "fetch_product_with_reviews"
  - "fetch_slim_user"

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
✓ Created record: EkoRecord(fields={id=StringValue(value=2cVSLLh6E8wO4yBt6FsES6w2_-77GYvAlwWr5shFENaTDgxvjapLCZD09T5PlrDEXE9C1UcndbKr-0lF1fwccA)})

=== Upsert Operation ===
✓ First upsert (update): EkoRecord(fields={active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), id=StringValue(value=2cVSLLh6E8wO4yBt6FsES6w2_-77GYvAlwWr5shFENaTDgxvjapLCZD09T5PlrDEXE9C1UcndbKr-0lF1fwccA), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), email=ObjectValue(value={value=StringValue(value=alice.j@newdomain.com), type=StringValue(value=String)})})
✓ Second upsert (insert): EkoRecord(fields={id=StringValue(value=1VB_xT4iXKvCoblChZk2jlUvtCPeBRHpsu2Yy7QGIGoSCtOBrBOzb91_j4JFAfNmYc6WeR7Q1kkeeYMDlFSJCA)})

=== Find One Operation ===
✓ Found user by email: EkoRecord(fields={email=ObjectValue(value={value=StringValue(value=alice.j@newdomain.com), type=StringValue(value=String)}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), id=StringValue(value=2cVSLLh6E8wO4yBt6FsES6w2_-77GYvAlwWr5shFENaTDgxvjapLCZD09T5PlrDEXE9C1UcndbKr-0lF1fwccA), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)})})
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
   Inserted with ripple: EkoRecord(fields={id=StringValue(value=amS8lJ8V7ebPs5Ln6LrI15dWpAqi4wCc9UqbRsulQwRYQGYRRxhaHEogrGmWbEqO3F2wYaYb00WJHV45ke0Mhw)})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: EkoRecord(fields={id=StringValue(value=f2c4sdAaUZpAreXFb2q1w93cYIxZg80X1Ko0MJBxEfTLZZKO22gSnB1utFbDARudszVhfkA9PTfkA51nTN0fAw)})

3. Update with bypass_ripple:
   Updated with bypass_ripple: EkoRecord(fields={id=StringValue(value=amS8lJ8V7ebPs5Ln6LrI15dWpAqi4wCc9UqbRsulQwRYQGYRRxhaHEogrGmWbEqO3F2wYaYb00WJHV45ke0Mhw), price=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=150)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Product 1)})})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: EkoRecord(fields={id=StringValue(value=amS8lJ8V7ebPs5Ln6LrI15dWpAqi4wCc9UqbRsulQwRYQGYRRxhaHEogrGmWbEqO3F2wYaYb00WJHV45ke0Mhw), price=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=500)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Upsert Product)})})

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
✓ kt_users_register saved
✓ kt_users_login saved
✓ kt_users_verify_token saved

=== Auth flow defined as pure stored functions ===
Call them like:
  POST /api/functions/kt_users_register { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/kt_users_login    { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/kt_users_verify_token { "token": "<jwt>" }

✓ Cleaned up demo functions

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
✓ Client created
✓ crypto_demo_hmac saved
✓ crypto_demo_aes saved
✓ crypto_demo_uuid saved
✓ crypto_demo_totp saved
✓ crypto_demo_encoding saved

Invoke them with:
  POST /api/functions/crypto_demo_hmac     { "payload": "hi" }
  POST /api/functions/crypto_demo_aes      { "plaintext": "secret" }
  POST /api/functions/crypto_demo_uuid
  POST /api/functions/crypto_demo_totp
  POST /api/functions/crypto_demo_encoding { "title": "Héllo World" }

✓ Cleaned up demo functions

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
✓ Client created
✓ conc_demo_pay saved
✓ conc_demo_rl_fail saved
✓ conc_demo_rl_skip saved
✓ conc_demo_lock saved

Invoke them like:
  POST /api/functions/conc_demo_pay        { "idempotency_key": "...", "amount": 100 }
  POST /api/functions/conc_demo_rl_fail    { "user_id": 42 }
  POST /api/functions/conc_demo_rl_skip    { "user_id": 42 }
  POST /api/functions/conc_demo_lock       { "resource": "queue:drain" }

✓ Cleaned up demo functions

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
✓ Client created
✓ kt_route_admin saved
✓ kt_route_user_by_id saved
✓ kt_route_user_posts saved
✓ kt_route_org_create_member saved

Try them with curl:
  curl http://localhost:8080/api/route/users/admin
  curl http://localhost:8080/api/route/users/42
  curl http://localhost:8080/api/route/users/42/posts/7
  curl -X POST http://localhost:8080/api/route/orgs/acme/members \
       -H 'Content-Type: application/json' -d '{"name":"alice"}'

✓ Cleaned up demo functions

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
✅ [32mKotlin client examples complete![0m
🟣 [33mKotlin Transactions...[0m
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: GRgrA8CT0n1xcQ3-_1d4tL9PGHVKo9kEcjZT2pN9Y16moULXkBqrcx4iRNPYICriG6qlKTt6SZTYMEGA10zhMg
Created Bob: $500 - ID: C5uH9h9uL8UUDVfW75QkZcw2NMiYaVg5oHdhMrvx7SD0axuoz-wDgQw3mXQjmWAkeCNAfz2OONqwm5h0J3uhvA

=== Example 1: Begin Transaction ===
Transaction ID: ac7b6ada-f268-48f2-9d0c-f676165e6616

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: de2b230f-a4a9-4005-b507-34e1fa79d079
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mAll Kotlin integration tests complete![0m
🐍 [36mBuilding Python client package...[0m
🔧 [36mEnsuring maturin is available in .venv...[0m
🔨 [36mBuilding wheel...[0m
🍹 Building a mixed python/rust project
🐍 Found CPython 3.11 at /Library/Frameworks/Python.framework/Versions/3.11/bin/python3
🔗 Found pyo3 bindings with abi3-py3.8 support
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.12s
📦 Built wheel for abi3 Python ≥ 3.8 to ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.26.1-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel into .venv...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.26.1-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.26.1
    Uninstalling ekodb_client-0.26.1:
      Successfully uninstalled ekodb_client-0.26.1
Successfully installed ekodb-client-0.26.1
🧪 [36mEnsuring test dependencies (pytest) in .venv...[0m
✅ [32mPython client package built and installed![0m
📦 [36mEnsuring Python example dependencies in .venv...[0m

🤖 [36mRAG Conversation System Examples[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m

[33mPrerequisites:[0m
  1. ekoDB server running (make run in ekodb/)
  2. OPENAI_API_KEY set in server environment
  3. API_BASE_URL and API_BASE_KEY exported in your shell

[36mBuilding Rust client library...[0m
✓ Rust client built

✓ Python client built and installed into .venv (via build-python-client prerequisite)

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
    ✓ Generated embedding: 1536 dimensions in 452.712542ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 405.642ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 330.064375ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 431.687667ms
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 675.664458ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 253.918208ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 321.615791ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 601.215333ms
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 392.086ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 245.298ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 479.486458ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 322.175125ms
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
    ✓ Generated embedding: 1536 dimensions in 198.338708ms
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 85.859375ms

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

Writing memory-safe, high-performance database code involves a combination of best practices across various programming and design disciplines, including language features, design patterns, and database techniques. Here are some key principles to consider:

### 1. Choose the Right Language and Tools
- **Use Memory-Safe Languages**: Opt for languages that provide built-in memory safety features, such as Rust or Swift. These languages help prevent common issues such as buffer overflows and memory leaks.
- **Use IDEs and Static Analysis Tools**: Leverage tools that provide static analysis, linting, and code reviews to catch potential memory safety issues.

### 2. Effective Memory Management
- **Connection Pooling**: Implement connection pooling to minimize the overhead of establishing database connections. This can help improve performance and resource management.
- **Use Prepared Statements**: Prepared statements can prevent SQL injection attacks and also improve performance since they allow the database to cache the execution plan.
- **Avoid Memory Leaks**: Ensure that any allocated memory (if using languages like C or C++) is properly deallocated. Utilize smart pointers (in C++) or garbage collection in managed languages to aid in this.

### 3. Optimize Query Performance
- **Use Indexes Wisely**: Create indexes on fields that are frequently queried to speed up search operations but avoid over-indexing as it can introduce overhead during write operations.
- **Batch Operations**: Perform batch inserts or updates to minimize database round trips and improve performance.
- **Efficient Query Construction**: Use appropriate filters to limit the amount of data being processed and fetched, reducing memory usage.

### 4. Design Patterns and Architectures
- **Repository Pattern**: Use the repository pattern to abstract data access logic, ensuring separation of concerns and making the code easier to test and maintain.
- **CQRS (Command Query Responsibility Segregation)**: Consider separating reads and writes into different models to optimize performance for each operation type.
  
### 5. Error Handling and Concurrency
- **Robust Error Handling**: Implement comprehensive error handling to ensure that database transactions are rolled back properly in case of failures, maintaining data integrity.
- **Use Transactions Properly**: Wrap multiple database operations in transactions to maintain consistency and prevent data corruption if something goes wrong.

### 6. Profiling and Benchmarking
- **Profile Code Regularly**: Use profiling tools to identify memory usage and performance bottlenecks within your database operations.
- **Benchmark Queries**: Measure performance before and after optimizations to gauge their effectiveness. Adjust based on your findings.

### 7. Follow Best Practices for Database Interaction
- **Limit Data Transfer**: Only request the data that you need. Avoid using `SELECT *` unless absolutely necessary.
- **Use Asynchronous Operations**: If your database system supports it, make use of non-blocking calls to improve responsiveness and throughput.

### Conclusion
Combining these strategies can help ensure that your database code is both memory-safe and high-performance. Continually monitor and assess your code's performance and memory usage as your application evolves, making adjustments as needed to maintain optimal operation.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 253.874833ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 3393 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 283.074584ms
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
  ✓ Text search completed in 60.561375ms

✓ Found 3 messages mentioning ownership:
  1. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  2. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  3. From conv_new_question: Writing memory-safe, high-performance database code involves a combination of best practices across various programming and design disciplines, including language features, design patterns, and database techniques. Here are some key principles to consider:

### 1. Choose the Right Language and Tools
- **Use Memory-Safe Languages**: Opt for languages that provide built-in memory safety features, such as Rust or Swift. These languages help prevent common issues such as buffer overflows and memory leaks.
- **Use IDEs and Static Analysis Tools**: Leverage tools that provide static analysis, linting, and code reviews to catch potential memory safety issues.

### 2. Effective Memory Management
- **Connection Pooling**: Implement connection pooling to minimize the overhead of establishing database connections. This can help improve performance and resource management.
- **Use Prepared Statements**: Prepared statements can prevent SQL injection attacks and also improve performance since they allow the database to cache the execution plan.
- **Avoid Memory Leaks**: Ensure that any allocated memory (if using languages like C or C++) is properly deallocated. Utilize smart pointers (in C++) or garbage collection in managed languages to aid in this.

### 3. Optimize Query Performance
- **Use Indexes Wisely**: Create indexes on fields that are frequently queried to speed up search operations but avoid over-indexing as it can introduce overhead during write operations.
- **Batch Operations**: Perform batch inserts or updates to minimize database round trips and improve performance.
- **Efficient Query Construction**: Use appropriate filters to limit the amount of data being processed and fetched, reducing memory usage.

### 4. Design Patterns and Architectures
- **Repository Pattern**: Use the repository pattern to abstract data access logic, ensuring separation of concerns and making the code easier to test and maintain.
- **CQRS (Command Query Responsibility Segregation)**: Consider separating reads and writes into different models to optimize performance for each operation type.
  
### 5. Error Handling and Concurrency
- **Robust Error Handling**: Implement comprehensive error handling to ensure that database transactions are rolled back properly in case of failures, maintaining data integrity.
- **Use Transactions Properly**: Wrap multiple database operations in transactions to maintain consistency and prevent data corruption if something goes wrong.

### 6. Profiling and Benchmarking
- **Profile Code Regularly**: Use profiling tools to identify memory usage and performance bottlenecks within your database operations.
- **Benchmark Queries**: Measure performance before and after optimizations to gauge their effectiveness. Adjust based on your findings.

### 7. Follow Best Practices for Database Interaction
- **Limit Data Transfer**: Only request the data that you need. Avoid using `SELECT *` unless absolutely necessary.
- **Use Asynchronous Operations**: If your database system supports it, make use of non-blocking calls to improve responsiveness and throughput.

### Conclusion
Combining these strategies can help ensure that your database code is both memory-safe and high-performance. Continually monitor and assess your code's performance and memory usage as your application evolves, making adjustments as needed to maintain optimal operation.

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
    ✓ Generated embedding: 1536 dimensions in 0.250s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.205s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.300s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.253s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.484s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.331s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.474s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.337s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.255s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.236s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.253s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.543s
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
    ✓ Generated embedding: 1536 dimensions in 0.235s
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.080s

✓ Found 5 related messages across all conversations:
  1. [Score: 0.504] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  2. [Score: 0.481] From conv_database_design
     What is database normalization?

  3. [Score: 0.479] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

  4. [Score: 0.467] From conv_database_design
     When should I use NoSQL over SQL?

  5. [Score: 0.446] From conv_performance
     How can I optimize database queries?

=== Step 4: Generating Context-Aware Response ===
✓ AI Response (with context from 3 conversations):

Writing memory-safe, high-performance database code involves several principles and best practices. Here are key strategies to consider:

### 1. Use a Memory-Safe Language
- **Choose languages**: Use languages like Rust, Go, or modern versions of C++ that offer memory safety features.
- **Avoid unsafe code**: Minimize or eliminate the use of unsafe constructs that can lead to memory leaks or buffer overflows.

### 2. Use Prepared Statements
- **Pre-compilation**: Prepared statements help in optimizing performance by allowing the database to compile the SQL statement once and execute it multiple times with different parameters.
- **Protection against SQL Injection**: They also protect against SQL injection attacks, providing a layer of security.

### 3. Connection Pooling
- **Reuse connections**: Implement connection pooling to maintain a pool of reusable connections, minimizing the overhead of creating a new connection for each request.
- **Manage resources**: Make sure that resources are properly managed and released when no longer needed.

### 4. Efficient Query Design
- **Use indexes**: Ensure that relevant columns are indexed to speed up query performance. Analyze your queries and optimize them based on their execution plans.
- **Limit data retrieval**: Fetch only the required fields, rather than using `SELECT *`, to reduce the amount of data transferred.
- **Batch operations**: Where possible, use batch inserts or updates to minimize the number of round trips to the database.

### 5. Error Handling
- **Handle errors properly**: Implement robust error handling. Use techniques like try/catch blocks for error handling to gracefully handle exceptions without leaking memory.
- **Log errors**: Log errors for diagnosis, but be careful not to log sensitive information.

### 6. Optimize Memory Usage
- **Data structure choice**: Use appropriate data structures that minimize memory usage while providing necessary functionality.
- **Garbage Collection**: If using a language with garbage collection, be aware of its behavior and the performance implications it may have.

### 7. Transaction Management
- **ACID Transactions**: Use transactions to ensure that your database operations are atomic, consistent, isolated, and durable—this often helps in maintaining data integrity.
- **Keep transactions short**: Minimize the time a transaction holds locks to reduce contention and increase throughput.

### 8. Asynchronous Operations
- **Non-blocking I/O**: Utilize asynchronous programming models to prevent blocking while waiting for database operations, improving the overall responsiveness of your application.

### 9. Monitor Performance
- **Database profiling**: Use database profiling tools to analyze slow queries and tune them accordingly.
- **Performance metrics**: Monitor memory usage and performance metrics regularly to identify bottlenecks or issues in real time.

### 10. Leverage Database Features
- **Advanced features**: Utilize features provided by your database system, such as replication, sharding, and caching, to enhance performance and reliability.
- **Read replicas**: Use read replicas for read-heavy workloads to distribute load without impacting write operations.

### Final Thoughts
Incorporating these strategies will help you write memory-safe, high-performance database code. The key is to strike a balance between safety, performance, and maintainability. Regular code reviews and testing can also help ensure adherence to these principles.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.232s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 3499 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.268s
    • Function auto-cleaned up by client
✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...

✓ Found 3 messages mentioning ownership:
  1. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  2. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  3. From conv_rust_programming: The borrow checker enforces Rust's ownership rules at compile time. It ensures that references don't outlive the data they point to and prevents data races by allowing either multiple immutable references or one mutable reference.

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
    ✓ Generated embedding: 1536 dimensions in 0.421s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.234s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.421s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.295s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.231s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.315s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.459s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.280s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.297s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.265s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.298s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.232s
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
    ✓ Generated embedding: 1536 dimensions in 0.400s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.072s
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

Writing memory-safe, high-performance database code involves a combination of principles and practices across several domains, including programming language features, database design, error handling, and optimization techniques. Here are some key strategies to consider:

### 1. **Choose the Right Database Type**
   - **Relational (SQL) vs. NoSQL:** Understand the characteristics of SQL databases (good for complex queries, ACID transactions, structured data) versus NoSQL databases (better for scalability, unstructured data, high write throughput). Choose based on your data model and application needs.

### 2. **Implement Efficient Schema Design**
   - **Normalization:** Use normalization to reduce redundancy and improve data integrity. Define relationships clearly using foreign keys, and avoid unnecessarily complex joins.
   - **Denormalization:** In some cases, like read-heavy applications, it may be beneficial to denormalize data for improved read performance.
   - **Indexing:** Create indexes on frequently queried columns to speed up search operations. Use composite indexes for queries that filter on multiple fields. Monitor query performance and index usage.

### 3. **Memory Management**
   - **Connection Pooling:** Use connection pooling to manage database connections efficiently, reducing the overhead of opening and closing connections repeatedly.
   - **Resource Limits:** Implement resource limits on queries to prevent excessive memory consumption, which can lead to performance degradation.

### 4. **Error Handling and Transactions**
   - **Use Transactions:** Wrap multiple operations in transactions to ensure data integrity and handle rolling back in case of failures.
   - **Error Handling:** Implement robust error handling strategies to manage exceptions, log errors, and avoid memory leaks.

### 5. **Optimize Queries**
   - **Query Profiling:** Utilize database profiling tools to analyze and optimize slow queries. Look for full table scans and non-indexed lookups.
   - **Use Prepared Statements:** Prepared statements help prevent SQL injection attacks and can improve performance due to efficient execution plans.
   - **Batch Operations:** For insertions or updates, use batch operations to minimize the number of round-trips to the database.

### 6. **Use Appropriate Data Types**
   - **Choose Data Types Wisely:** Select the most efficient data types for your columns. Smaller data types (e.g., INTEGER instead of BIGINT) can reduce memory usage.

### 7. **Leverage Modern Language Features**
   - **Memory Safety Features:** If using languages like Rust, take advantage of its ownership model and borrow-checking features to ensure memory safety at compile time.
   - **Concurrency:** Utilize concurrency features in your programming language to handle multiple database operations efficiently while maintaining data integrity.

### 8. **Monitoring and Optimization**
   - **Monitoring Tools:** Use monitoring tools to track performance metrics, query execution times, and system resource usage.
   - **Continuous Optimization:** Regularly review and optimize your database design, code, and queries as your application and its data evolve.

### 9. **Consider Caching**
   - **Cache Results:** Implement caching strategies (in-memory stores like Redis) to store frequently accessed data, reducing database load and latency.

### 10. **Testing**
   - **Stress Testing:** Perform stress tests to simulate high-load scenarios, ensuring your database code performs well under stress and observing its behavior in terms of memory usage and performance.

By combining these approaches, you can develop high-performance, memory-safe database applications that meet the needs of your users while preventing common issues associated with database operations.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.281s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 3799 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.248s
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

  3. From conv_new_question: Writing memory-safe, high-performance database code involves a combination of principles and practices across several domains, including programming language features, database design, error handling, and optimization techniques. Here are some key strategies to consider:

### 1. **Choose the Right Database Type**
   - **Relational (SQL) vs. NoSQL:** Understand the characteristics of SQL databases (good for complex queries, ACID transactions, structured data) versus NoSQL databases (better for scalability, unstructured data, high write throughput). Choose based on your data model and application needs.

### 2. **Implement Efficient Schema Design**
   - **Normalization:** Use normalization to reduce redundancy and improve data integrity. Define relationships clearly using foreign keys, and avoid unnecessarily complex joins.
   - **Denormalization:** In some cases, like read-heavy applications, it may be beneficial to denormalize data for improved read performance.
   - **Indexing:** Create indexes on frequently queried columns to speed up search operations. Use composite indexes for queries that filter on multiple fields. Monitor query performance and index usage.

### 3. **Memory Management**
   - **Connection Pooling:** Use connection pooling to manage database connections efficiently, reducing the overhead of opening and closing connections repeatedly.
   - **Resource Limits:** Implement resource limits on queries to prevent excessive memory consumption, which can lead to performance degradation.

### 4. **Error Handling and Transactions**
   - **Use Transactions:** Wrap multiple operations in transactions to ensure data integrity and handle rolling back in case of failures.
   - **Error Handling:** Implement robust error handling strategies to manage exceptions, log errors, and avoid memory leaks.

### 5. **Optimize Queries**
   - **Query Profiling:** Utilize database profiling tools to analyze and optimize slow queries. Look for full table scans and non-indexed lookups.
   - **Use Prepared Statements:** Prepared statements help prevent SQL injection attacks and can improve performance due to efficient execution plans.
   - **Batch Operations:** For insertions or updates, use batch operations to minimize the number of round-trips to the database.

### 6. **Use Appropriate Data Types**
   - **Choose Data Types Wisely:** Select the most efficient data types for your columns. Smaller data types (e.g., INTEGER instead of BIGINT) can reduce memory usage.

### 7. **Leverage Modern Language Features**
   - **Memory Safety Features:** If using languages like Rust, take advantage of its ownership model and borrow-checking features to ensure memory safety at compile time.
   - **Concurrency:** Utilize concurrency features in your programming language to handle multiple database operations efficiently while maintaining data integrity.

### 8. **Monitoring and Optimization**
   - **Monitoring Tools:** Use monitoring tools to track performance metrics, query execution times, and system resource usage.
   - **Continuous Optimization:** Regularly review and optimize your database design, code, and queries as your application and its data evolve.

### 9. **Consider Caching**
   - **Cache Results:** Implement caching strategies (in-memory stores like Redis) to store frequently accessed data, reducing database load and latency.

### 10. **Testing**
   - **Stress Testing:** Perform stress tests to simulate high-load scenarios, ensuring your database code performs well under stress and observing its behavior in terms of memory usage and performance.

By combining these approaches, you can develop high-performance, memory-safe database applications that meet the needs of your users while preventing common issues associated with database operations.

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
    ✓ Generated embedding: 1536 dimensions in 0.244s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.254s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.287s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.322s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.455s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.268s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.271s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.258s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.325s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.299s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.246s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.286s
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
    ✓ Generated embedding: 1536 dimensions in 0.249s
    • Function auto-cleaned up by client

→ Executing HybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.073s

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

To write memory-safe and high-performance database code, you should follow several best practices and principles across programming and database management. Here’s a comprehensive guide:

### 1. Use Prepared Statements
- **Benefit**: Prevent SQL injection attacks and improve performance by allowing the database to cache query plans.
- **Implementation**: Always use prepared statements when executing SQL queries. This is especially important in environments where user input is involved.

### 2. Optimize Database Queries
- **Indexes**: Use indexes strategically to improve query performance. Monitor performance and adjust as necessary.
- **Query Complexity**: Simplify queries to reduce execution time. Avoid unnecessary joins and subqueries whenever possible.
- **Batch Processing**: Instead of executing multiple queries one by one, batch them into a single transaction to reduce overhead.

### 3. Use Connection Pooling
- **Benefit**: Reduces the overhead of establishing a connection for every database operation.
- **Implementation**: Utilize a connection pool manager in your application to manage database connections efficiently.

### 4. Manage Memory Efficiently
- **Avoid Memory Leaks**: Use languages and frameworks that offer memory safety features (like Rust with ownership, or languages with garbage collection).
- **Release Resources**: Ensure that connections, statements, and other resources are released after use. In languages like C or C++, manually manage memory.

### 5. Use Database Transactions
- **Atomicity**: Group multiple database operations in a transaction to ensure data integrity. Use transactions to rollback changes in case of errors.
- **Isolation Levels**: Choose appropriate transaction isolation levels based on the application’s requirements (i.e., READ COMMITTED, SERIALIZABLE) to balance performance and data accuracy.

### 6. Monitor Performance
- **Logging**: Implement rigorous logging to capture slow queries and errors. Use tools to analyze performance bottlenecks.
- **Profiling Tools**: Use database profiling tools and EXPLAIN plans to understand query performance and optimize accordingly.

### 7. Minimize Data Retrieval
- **Projection**: Retrieve only the necessary columns instead of using SELECT *.
- **Pagination**: Implement pagination for large result sets to reduce memory consumption and improve user experience.

### 8. Use ORM Wisely
- **Selectivity**: When using Object-Relational Mapping (ORM) frameworks, be judicious about how queries are generated. Avoid “N+1 query problems” by eager loading associations where required.

### 9. Test and Benchmark
- **Unit Testing**: Create tests that validate database interactions to catch issues early.
- **Performance Testing**: Benchmark database operations under load conditions to discover any performance constraints.

### 10. Consider Data Architecture
- **Normalization**: Use database normalization to eliminate redundancy but balance this against the performance needs of the application.
- **Denormalization When Necessary**: In some scenarios where read performance is critical, denormalization may be justified.

### 11. Utilize Caching
- **In-Memory Caching**: Use caching layers (e.g., Redis, Memcached) to reduce database load for frequently accessed data.
- **Result Caching**: Cache query results when applicable to speed up data retrieval.

### 12. Use Asynchronous Operations
- **ASYNC Patterns**: Consider using asynchronous database operations where appropriate, especially for I/O bound applications, to improve responsiveness.

### Conclusion
By integrating these principles into your development workflow, you will achieve memory-safe, high-performance database code. Additionally, make regular updates to your knowledge of best practices and programming languages to stay current with advancements in technology that enhance performance and safety.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.319s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 3891 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.286s
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
  1. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  2. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

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
    ✓ Generated embedding: 1536 dimensions in 0.262s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.346s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.217s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.241s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.225s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.268s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.24s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.252s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.314s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.272s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.25s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.367s
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
    ✓ Generated embedding: 1536 dimensions in 0.32s
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
✓ Context prepared from search results
✓ AI would use this context to generate comprehensive response

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.312s
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
  ✓ Text search completed in 0.038s

✓ Found 3 messages mentioning ownership:
  1. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  2. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

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

> @ekodb/ekodb-client@0.26.1 prepare
> npm run build


> @ekodb/ekodb-client@0.26.1 build
> tsc


up to date, audited 52 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.26.1 build
> tsc

✅ [32mTypeScript client built![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning TypeScript SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
Function 'fetch_api_user' already existed — updated instead
✓ Created SWR script: fetch_api_user (BX2HCrYQTmpoVRQG2hYasvgkwFbdYhxcnkh_imFS2noL8RvM3y5B6jR3sSy-lsDoCvjCorLmEDNvk9Vy-PbLfA)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "value": {
          "phone": "1-770-736-8031 x56442",
          "username": "Bret",
          "website": "hildegard.org",
          "name": "Leanne Graham",
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "id": 1,
          "email": "Sincere@april.biz",
          "address": {
            "suite": "Apt. 556",
            "city": "Gwenborough",
            "street": "Kulas Light",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "zipcode": "92998-3874"
          }
        },
        "type": "Object"
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
Response time: 3ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "phone": "1-770-736-8031 x56442",
          "username": "Bret",
          "website": "hildegard.org",
          "name": "Leanne Graham",
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "id": 1,
          "email": "Sincere@april.biz",
          "address": {
            "suite": "Apt. 556",
            "city": "Gwenborough",
            "street": "Kulas Light",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "zipcode": "92998-3874"
          }
        }
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
Function 'fetch_product_with_reviews' already existed — updated instead
✓ Created enrichment script: fetch_product_with_reviews (fFx-0_LTbRylvUzo3f_wXTEneoLGwANn7G8UR3UdU8_lnl40ALlEjrRwMVGAiimcj_Xurl_5JBcm8qFtqyWjCw)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "shippingInformation": "Ships in 3-5 business days",
          "title": "Essence Mascara Lash Princess",
          "stock": 99,
          "warrantyInformation": "1 week warranty",
          "weight": 4,
          "sku": "BEA-ESS-ESS-001",
          "rating": 2.56,
          "id": 1,
          "category": "beauty",
          "availabilityStatus": "In Stock",
          "tags": [
            "beauty",
            "mascara"
          ],
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "price": 9.99,
          "returnPolicy": "No return policy",
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "reviews": [
            {
              "reviewerName": "Eleanor Collins",
              "date": "2025-04-30T09:41:02.053Z",
              "rating": 3,
              "comment": "Would not recommend!",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com"
            },
            {
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerName": "Lucas Gordon",
              "reviewerEmail": "lucas.gordon@x.dummyjson.com",
              "comment": "Very satisfied!",
              "rating": 4
            },
            {
              "date": "2025-04-30T09:41:02.053Z",
              "comment": "Highly impressed!",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "rating": 5,
              "reviewerName": "Eleanor Collins"
            }
          ],
          "meta": {
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
            "updatedAt": "2025-04-30T09:41:02.053Z",
            "barcode": "5784719087687",
            "createdAt": "2025-04-30T09:41:02.053Z"
          },
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "brand": "Essence",
          "discountPercentage": 10.48,
          "minimumOrderQuantity": 48,
          "dimensions": {
            "depth": 22.99,
            "width": 15.14,
            "height": 13.08
          }
        }
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
ℹ️  Function 'cache_api_call' already existed — updated instead
✓ Edge cache script updated: EQeEdu6UpIv0UOMJeQNBQK1GR6eD7uTQT3vPHFSZxRueVLqMJYxQKK34Pb78v7dEHeCFr38aNQK03At9RyctXA

Call 1: Cache miss (fetches from API)
Response time: 70ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "name": "Leanne Graham",
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets"
          },
          "id": 1,
          "phone": "1-770-736-8031 x56442",
          "email": "Sincere@april.biz",
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
Response time: 3ms (23.3x faster!)
Result: {
  "records": [
    {
      "value": {
        "value": {
          "name": "Leanne Graham",
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets"
          },
          "id": 1,
          "phone": "1-770-736-8031 x56442",
          "email": "Sincere@april.biz",
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
        },
        "type": "Object"
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
🔧 [36mEnsuring maturin is available in .venv...[0m
🔨 [36mBuilding wheel...[0m
🍹 Building a mixed python/rust project
🐍 Found CPython 3.11 at /Library/Frameworks/Python.framework/Versions/3.11/bin/python3
🔗 Found pyo3 bindings with abi3-py3.8 support
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.10s
📦 Built wheel for abi3 Python ≥ 3.8 to ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.26.1-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel into .venv...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.26.1-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.26.1
    Uninstalling ekodb_client-0.26.1:
      Successfully uninstalled ekodb_client-0.26.1
Successfully installed ekodb-client-0.26.1
🧪 [36mEnsuring test dependencies (pytest) in .venv...[0m
✅ [32mPython client package built and installed![0m
📦 [36mEnsuring Python example dependencies in .venv...[0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Python SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (YWnrP3Tr0ONuDFX7Z9BcUYVtLSDml2PcdCXmQWhKKvhERhNQyS3UCbFrHaYnr5xL3VaDwr4lc-5QmUYAWK6y4g)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 202ms
Result: [
  {
    "cached_at": {
      "type": "String",
      "value": "1788818071"
    },
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
        "followers": 321388,
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
        "public_repos": 12,
        "received_events_url": "https://api.github.com/users/torvalds/received_events",
        "repos_url": "https://api.github.com/users/torvalds/repos",
        "site_admin": false,
        "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
        "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
        "twitter_username": null,
        "type": "User",
        "updated_at": "2026-07-21T17:42:26Z",
        "url": "https://api.github.com/users/torvalds",
        "user_view_type": "public"
      }
    },
    "id": "torvalds"
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 3ms (62.0x faster!)
Result: [
  {
    "cached_at": {
      "type": "String",
      "value": "1788818071"
    },
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
        "followers": 321388,
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
        "public_repos": 12,
        "received_events_url": "https://api.github.com/users/torvalds/received_events",
        "repos_url": "https://api.github.com/users/torvalds/repos",
        "site_admin": false,
        "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
        "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
        "twitter_username": null,
        "type": "User",
        "updated_at": "2026-07-21T17:42:26Z",
        "url": "https://api.github.com/users/torvalds",
        "user_view_type": "public"
      }
    }
  }
]
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (HAd9qXWIUfhRmZ_IR99qmlf4yaSXmN-Bg7GlYeuAtUCCfstvA1vm8RSYuNkqo4zQ8kevTQAdno8gaSECi__duQ)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "enriched_at": {
      "type": "String",
      "value": "1788818071"
    },
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
Function 'fetch_github_user' already existed — updated instead
✓ Created SWR script: fetch_github_user (fetch_github_user)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 3.3255ms
Result: [
  {
    "cached_at": {
      "type": "String",
      "value": "1788818071"
    },
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
        "followers": 321388,
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
        "public_repos": 12,
        "received_events_url": "https://api.github.com/users/torvalds/received_events",
        "repos_url": "https://api.github.com/users/torvalds/repos",
        "site_admin": false,
        "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
        "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
        "twitter_username": null,
        "type": "User",
        "updated_at": "2026-07-21T17:42:26Z",
        "url": "https://api.github.com/users/torvalds",
        "user_view_type": "public"
      }
    }
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 3.02525ms (1.0x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
Function 'fetch_product_enriched' already existed — updated instead
✓ Created enrichment script: fetch_product_enriched (fetch_product_enriched)

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
✅ [32mClient build complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Rust SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.47s
     Running `target/debug/examples/swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
ℹ️  Function 'fetch_github_user' already existed — updated instead
✓ Created SWR script: fetch_github_user (-fzDzLxF8AUkZaz-x05NAuO6HpP5b71hg8_vyEFg8A955YkHrcnKbIKRBiLQhQAMHIT6yKxtBEejHuaHAySuqQ)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 2ms
Result: {
  "data": {
    "type": "Object",
    "value": {
      "created_at": "2011-09-03T15:26:22Z",
      "login": "torvalds",
      "followers_url": "https://api.github.com/users/torvalds/followers",
      "user_view_type": "public",
      "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
      "blog": "",
      "events_url": "https://api.github.com/users/torvalds/events{/privacy}",
      "company": "Linux Foundation",
      "bio": null,
      "hireable": null,
      "updated_at": "2026-07-21T17:42:26Z",
      "email": null,
      "followers": 321388,
      "public_repos": 12,
      "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
      "node_id": "MDQ6VXNlcjEwMjQwMjU=",
      "url": "https://api.github.com/users/torvalds",
      "html_url": "https://github.com/torvalds",
      "repos_url": "https://api.github.com/users/torvalds/repos",
      "received_events_url": "https://api.github.com/users/torvalds/received_events",
      "id": 1024025,
      "name": "Linus Torvalds",
      "location": "Portland, OR",
      "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
      "gravatar_id": "",
      "following": 0,
      "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
      "organizations_url": "https://api.github.com/users/torvalds/orgs",
      "type": "User",
      "site_admin": false,
      "twitter_username": null,
      "public_gists": 1,
      "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}"
    }
  }
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 2ms (1.0x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
ℹ️  Function 'fetch_product_enriched' already existed — updated instead
✓ Created enrichment script: fetch_product_enriched (x8309W83hZBTAEXuPTqNAQm3WNCQgtqE_IIcNsG7aG7BWb6XrmHINkyayTutPP_EWYsj0q5sppxVPVDPDn6zKA)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: {
  "enriched_data": {
    "type": "Object",
    "value": {
      "availabilityStatus": "In Stock",
      "rating": 2.56,
      "discountPercentage": 10.48,
      "reviews": [
        {
          "rating": 3,
          "reviewerName": "Eleanor Collins",
          "date": "2025-04-30T09:41:02.053Z",
          "comment": "Would not recommend!",
          "reviewerEmail": "eleanor.collins@x.dummyjson.com"
        },
        {
          "reviewerEmail": "lucas.gordon@x.dummyjson.com",
          "comment": "Very satisfied!",
          "date": "2025-04-30T09:41:02.053Z",
          "reviewerName": "Lucas Gordon",
          "rating": 4
        },
        {
          "rating": 5,
          "date": "2025-04-30T09:41:02.053Z",
          "comment": "Highly impressed!",
          "reviewerName": "Eleanor Collins",
          "reviewerEmail": "eleanor.collins@x.dummyjson.com"
        }
      ],
      "id": 1,
      "shippingInformation": "Ships in 3-5 business days",
      "minimumOrderQuantity": 48,
      "images": [
        "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
      ],
      "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
      "tags": [
        "beauty",
        "mascara"
      ],
      "returnPolicy": "No return policy",
      "title": "Essence Mascara Lash Princess",
      "price": 9.99,
      "weight": 4,
      "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
      "meta": {
        "createdAt": "2025-04-30T09:41:02.053Z",
        "barcode": "5784719087687",
        "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
        "updatedAt": "2025-04-30T09:41:02.053Z"
      },
      "dimensions": {
        "height": 13.08,
        "width": 15.14,
        "depth": 22.99
      },
      "warrantyInformation": "1 week warranty",
      "brand": "Essence",
      "sku": "BEA-ESS-ESS-001",
      "category": "beauty",
      "stock": 99
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
ℹ️  Function 'fetch_github_user' already existed — updated instead
✓ Created SWR function: fetch_github_user (lo0EDJTT1JJ2GrQZ4xi-uKZhGc9xxYypZNGEdEVizkV255FK5_jFw6FSgHXG0qX0Zt5XZ-X1cA-DakMOhzNs3w)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 17ms
Result: [{"cached_at":{"value":"1788818071","type":"String"},"data":{"value":{"followers_url":"https://api.github.com/users/torvalds/followers","url":"https://api.github.com/users/torvalds","user_view_type":"public","twitter_username":null,"gists_url":"https://api.github.com/users/torvalds/gists{/gist_id}","starred_url":"https://api.github.com/users/torvalds/starred{/owner}{/repo}","public_gists":1,"type":"User","updated_at":"2026-07-21T17:42:26Z","company":"Linux Foundation","email":null,"following":0,"site_admin":false,"repos_url":"https://api.github.com/users/torvalds/repos","id":1024025,"followers":321388,"following_url":"https://api.github.com/users/torvalds/following{/other_user}","hireable":null,"received_events_url":"https://api.github.com/users/torvalds/received_events","gravatar_id":"","organizations_url":"https://api.github.com/users/torvalds/orgs","location":"Portland, OR","login":"torvalds","name":"Linus Torvalds","blog":"","created_at":"2011-09-03T15:26:22Z","html_url":"https://github.com/torvalds","bio":null,"avatar_url":"https://avatars.githubusercontent.com/u/1024025?v=4","subscriptions_url":"https://api.github.com/users/torvalds/subscriptions","node_id":"MDQ6VXNlcjEwMjQwMjU=","events_url":"https://api.github.com/users/torvalds/events{/privacy}","public_repos":12},"type":"Object"}}]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 5ms
Cache hit was 3.4x faster!

✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
ℹ️  Function 'fetch_product_enriched' already existed — updated instead
✓ Created enrichment function: fetch_product_enriched (vLnkAe4twlNAlocf1Hs7rHm87GZnLyf8VFCCSFzjQb-5KQ1prvmkcagyTZjwCGB7bifnyFBsMpr-kQImlOy9xQ)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [{"enriched_data":{"type":"Object","value":{"sku":"BEA-ESS-ESS-001","warrantyInformation":"1 week warranty","weight":4,"shippingInformation":"Ships in 3-5 business days","thumbnail":"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp","rating":2.56,"dimensions":{"depth":22.99,"height":13.08,"width":15.14},"brand":"Essence","stock":99,"tags":["beauty","mascara"],"id":1,"description":"The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.","returnPolicy":"No return policy","images":["https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"],"minimumOrderQuantity":48,"discountPercentage":10.48,"category":"beauty","price":9.99,"title":"Essence Mascara Lash Princess","reviews":[{"reviewerName":"Eleanor Collins","rating":3,"date":"2025-04-30T09:41:02.053Z","reviewerEmail":"eleanor.collins@x.dummyjson.com","comment":"Would not recommend!"},{"reviewerEmail":"lucas.gordon@x.dummyjson.com","reviewerName":"Lucas Gordon","comment":"Very satisfied!","rating":4,"date":"2025-04-30T09:41:02.053Z"},{"comment":"Highly impressed!","rating":5,"date":"2025-04-30T09:41:02.053Z","reviewerEmail":"eleanor.collins@x.dummyjson.com","reviewerName":"Eleanor Collins"}],"availabilityStatus":"In Stock","meta":{"updatedAt":"2025-04-30T09:41:02.053Z","barcode":"5784719087687","qrCode":"https://cdn.dummyjson.com/public/qr-code.png","createdAt":"2025-04-30T09:41:02.053Z"}}}}]
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


BUILD SUCCESSFUL in 10s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
✅ [32mClient build complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Rust Function Composition Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_function_composition`
=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

ℹ️  Function 'fetch_user' already existed — updated instead
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

ℹ️  Function 'fetch_and_store_user' already existed — updated instead
✅ Saved reusable function: fetch_and_store_user (uses KV)
ℹ️  Function 'swr_user' already existed — updated instead
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 76.295541ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "company": {
        "name": "Romaguera-Crona",
        "bs": "harness real-time e-markets",
        "catchPhrase": "Multi-layered client-serv...

Second call (cache hit - from cache):
   ⏱️  Duration: 3.716083ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "name": "Leanne Graham",
      "email": "Sincere@april.biz",
      "address": {
        "city": "Gwenborough",
        "suite": "Apt. 556",
  ...
   🚀 Cache speedup: 25.3x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

ℹ️  Function 'validate_user' already existed — updated instead
✅ Level 1 function: validate_user
ℹ️  Function 'fetch_slim_user' already existed — updated instead
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

> @ekodb/ekodb-client@0.26.1 prepare
> npm run build


> @ekodb/ekodb-client@0.26.1 build
> tsc


up to date, audited 52 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.26.1 build
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

Function 'fetch_user' already existed — updated instead
✅ Saved reusable function: fetch_user
Function 'get_user_wrapper' already existed — updated instead
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"type":"String","value":"User 1"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

Function 'fetch_and_store_user' already existed — updated instead
✅ Saved reusable function: fetch_and_store_user (uses KV)
Function 'swr_user' already existed — updated instead
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 2ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "phone": "1-770-736-8031 x56442",
      "email": "Sincere@april.biz",
      "name": "Leanne Graham",
      "website": "hildegard.org",
      "id": 1,
      "company...

Second call (cache hit - from cache):
   ⏱️  Duration: 2ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "phone": "1-770-736-8031 x56442",
      "email": "Sincere@april.biz",
      "name": "Leanne Graham",
      "website": "hildegard.org",
      "id": 1,
      "company...
   🚀 Cache speedup: 1.0x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

Function 'validate_user' already existed — updated instead
✅ Level 1 function: validate_user
Function 'fetch_slim_user' already existed — updated instead
✅ Level 2 function: fetch_slim_user (calls validate_user)
Function 'get_verified_user' already existed — updated instead
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
🔧 [36mEnsuring maturin is available in .venv...[0m
🔨 [36mBuilding wheel...[0m
🍹 Building a mixed python/rust project
🐍 Found CPython 3.11 at /Library/Frameworks/Python.framework/Versions/3.11/bin/python3
🔗 Found pyo3 bindings with abi3-py3.8 support
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.10s
📦 Built wheel for abi3 Python ≥ 3.8 to ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.26.1-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel into .venv...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.26.1-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.26.1
    Uninstalling ekodb_client-0.26.1:
      Successfully uninstalled ekodb_client-0.26.1
Successfully installed ekodb-client-0.26.1
🧪 [36mEnsuring test dependencies (pytest) in .venv...[0m
✅ [32mPython client package built and installed![0m
📦 [36mEnsuring Python example dependencies in .venv...[0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Python Function Composition Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

ℹ️  Function 'fetch_user' already existed — updated instead
✅ Saved reusable function: fetch_user
ℹ️  Function 'get_user_wrapper' already existed — updated instead
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"type": "String", "value": "User 1"}
   Department: {"type": "String", "value": "engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

ℹ️  Function 'fetch_and_store_user' already existed — updated instead
✅ Saved reusable function: fetch_and_store_user (uses KV)
ℹ️  Function 'swr_user' already existed — updated instead
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 2.2ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "type": "Object",
            "value": {
                  "address": {
                        "city": "Gwenborough",
                        "geo": {
                 ...

Second call (cache hit - from cache):
   ⏱️  Duration: 1.8ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "type": "Object",
            "value": {
                  "address": {
                        "city": "Gwenborough",
                        "geo": {
                 ...
   🚀 Cache speedup: 1.2x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

ℹ️  Function 'validate_user' already existed — updated instead
✅ Level 1 function: validate_user
ℹ️  Function 'fetch_slim_user' already existed — updated instead
✅ Level 2 function: fetch_slim_user (calls validate_user)
ℹ️  Function 'get_verified_user' already existed — updated instead
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

Function 'fetch_user' already existed — updated instead
✅ Saved reusable function: fetch_user
Function 'get_user_wrapper' already existed — updated instead
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"type":"String","value":"User 1"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

Function 'fetch_and_store_user' already existed — updated instead
✅ Saved reusable function: fetch_and_store_user (uses KV)
Function 'swr_user' already existed — updated instead
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 2.003041ms
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
   ⏱️  Duration: 1.719875ms
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
   🚀 Cache speedup: 2.0x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

Function 'validate_user' already existed — updated instead
✅ Level 1 function: validate_user
Function 'fetch_slim_user' already existed — updated instead
✅ Level 2 function: fetch_slim_user (calls validate_user)
Function 'get_verified_user' already existed — updated instead
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

ℹ️  Function 'fetch_user' already existed — updated instead
✅ Saved reusable function: fetch_user
ℹ️  Function 'get_user_wrapper' already existed — updated instead
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"value":"User 1","type":"String"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

ℹ️  Function 'fetch_and_store_user' already existed — updated instead
✅ Saved reusable function: fetch_and_store_user (uses KV)
ℹ️  Function 'swr_user' already existed — updated instead
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 2ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "phone": "1-770-736-8031 x56442",
      "email": "Sincere@april.biz",
      "name": "Leanne Graham",
      "website": "hildegard.org",
      "...

Second call (cache hit - from cache):
   ⏱️  Duration: 2ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "phone": "1-770-736-8031 x56442",
      "email": "Sincere@april.biz",
      "name": "Leanne Graham",
      "website": "hildegard.org",
      "id": 1,
      "company...
   🚀 Cache speedup: 1.0x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

ℹ️  Function 'validate_user' already existed — updated instead
✅ Level 1 function: validate_user
ℹ️  Function 'fetch_slim_user' already existed — updated instead
✅ Level 2 function: fetch_slim_user (calls validate_user)
ℹ️  Function 'get_verified_user' already existed — updated instead
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
✅ [32mClient build complete![0m

🦀 [33mRust WebSocket Subscription Test...[0m
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.29s
     Running `target/debug/examples/client_websocket_subscribe`
✓ Authentication successful

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_rs' ===

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: "EDc-noZbLUPLvl0LMql-aOVVlgDLynd5U-zlJcDHLa8pVrK5OY95NfDYawZFVtayLBYlfVbIDq7xXFjTCOA1Dg"

  📡 Notification received:
     Event:      "insert"
     Collection: "ws_subscribe_example_rs"
     Record IDs: ["EDc-noZbLUPLvl0LMql-aOVVlgDLynd5U-zlJcDHLa8pVrK5OY95NfDYawZFVtayLBYlfVbIDq7xXFjTCOA1Dg"]
     Timestamp:  "2026-09-07T21:57:53.359447+00:00"

Inserting record 2...
✓ Inserted: "ArtzjmDWqYoxEQJNr0cwUDgWKoVVyFYYpdMyZOCCMBD_dO40kiNf63XfVViCqYFP0vuT89bMQEOW3CF0ogrDeg"

  📡 Notification received:
     Event:      "insert"
     Record IDs: ["ArtzjmDWqYoxEQJNr0cwUDgWKoVVyFYYpdMyZOCCMBD_dO40kiNf63XfVViCqYFP0vuT89bMQEOW3CF0ogrDeg"]

=== Unsubscribing ===
✓ Unsubscribed

✓ WebSocket subscription example completed successfully
✅ [32mRust subscription test complete![0m

🔷 [33mGo WebSocket Subscription Test...[0m
=== WebSocket Subscription Example (Go) ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_go' ===
✓ Subscribed (subscription_id: sub_18e53612ad884b0eb11381f4f922c8d0)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: D0bOUna8RrJJRLel5fZ8M53SvhiqBw5IDQJQ9NzoYCg9EIjZm3J6gduAEauJNkDE2eV901VkN7BUA5jKBI227w

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_go
     Record IDs: [D0bOUna8RrJJRLel5fZ8M53SvhiqBw5IDQJQ9NzoYCg9EIjZm3J6gduAEauJNkDE2eV901VkN7BUA5jKBI227w]
     Timestamp:  2026-09-07T21:57:53.939052+00:00

Inserting record 2...
✓ Inserted: GwtUrADIaOlzdlYKKUOlxEwcjV5ZG67LjMbNWRECCZkCp-WQaaH56SwkyzJm0I5ElREjVyxwds0VactMMMMVtA

  📡 Notification received:
     Event:      insert
     Record IDs: [GwtUrADIaOlzdlYKKUOlxEwcjV5ZG67LjMbNWRECCZkCp-WQaaH56SwkyzJm0I5ElREjVyxwds0VactMMMMVtA]

=== Unsubscribing ===
✓ Unsubscribed

✓ WebSocket subscription example completed successfully
✅ [32mGo subscription test complete![0m
📦 [36mEnsuring Python example dependencies in .venv...[0m

🐍 [33mPython WebSocket Subscription Test...[0m
=== WebSocket Subscription Example (Python) ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_py' ===
✓ Subscribed (subscription_id: sub_34b8ebd281924b169c7949ee9794037d)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: soedgivXWDLHJiIG9vrjmOb0SvPubiljHXLSUkamJLwqHjGMeQh2ytDSuBCZeyjdVKe0D4-QnRpWPH4mrmur_w

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_py
     Record IDs: soedgivXWDLHJiIG9vrjmOb0SvPubiljHXLSUkamJLwqHjGMeQh2ytDSuBCZeyjdVKe0D4-QnRpWPH4mrmur_w
     Timestamp:  2026-09-07T21:57:54.391218+00:00

Inserting record 2...
✓ Inserted: DQ1Yy7YtEC33ehsss0Y7eyvFYKf0Qi2qvJEmwIJob9tmia4sqVk9yaERYso7RESPBS289Hms0N8bjNzVPo6oOQ

  📡 Notification received:
     Event:      insert
     Record IDs: DQ1Yy7YtEC33ehsss0Y7eyvFYKf0Qi2qvJEmwIJob9tmia4sqVk9yaERYso7RESPBS289Hms0N8bjNzVPo6oOQ

=== Unsubscribing ===
✓ Unsubscribed: {'collection': 'ws_subscribe_example_py', 'found': True, 'unsubscribed': True}

✓ WebSocket subscription example completed successfully
✅ [32mPython subscription test complete![0m
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.26.1 prepare
> npm run build


> @ekodb/ekodb-client@0.26.1 build
> tsc


up to date, audited 52 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.26.1 build
> tsc

✅ [32mTypeScript client built![0m

📘 [33mTypeScript WebSocket Subscription Test...[0m
=== WebSocket Subscription Example ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_ts' ===
✓ Subscribed (subscription_id: sub_a398ac5ac8ee4a5fbc27e12bbd11b9a9)

=== Listening for mutation notifications ===

Inserting a record...

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_ts
     Record IDs: r8yERa9Ssr4ZHsJQX21uRtwiuI-yTP-pJqj243ms9yY8lwy63vxiOViZ_hazxEQnESQmdOY886pyfl4O_wxqwg
     Timestamp:  2026-09-07T21:57:57.237243+00:00
     Records:    {"active":{"type":"Boolean","value":true},"id":"r8yERa9Ssr4ZHsJQX21uRtwiuI-yTP-pJqj243ms9yY8lwy63vxi...
✓ Inserted record: r8yERa9Ssr4ZHsJQX21uRtwiuI-yTP-pJqj243ms9yY8lwy63vxiOViZ_hazxEQnESQmdOY886pyfl4O_wxqwg

Inserting another record...

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_ts
     Record IDs: eTI7zHkqteq0SRSyD0MyBNt8oNS_KYulXZGdbckvmeGkbFOA3qWwGDYMs1NI9mJbcyP3B8krgzI4WwlpfZF06g
     Timestamp:  2026-09-07T21:57:58.262118+00:00
     Records:    {"active":{"type":"Boolean","value":true},"id":"eTI7zHkqteq0SRSyD0MyBNt8oNS_KYulXZGdbckvmeGkbFOA3qWw...

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
✓ Subscribed (subscription_id: sub_fce6a095b64c4ec6af70e5428c41e73b)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: "SEvM4hFDzkJIgjlT-aMR3C_wMYrZhtKqrRJtBaBQ-joGA3n2QK8Pj7lT_EDSdsMwX62FsOx8kh0qiOhw7-stXA"

  📡 Notification received:
     Event:      "insert"
     Collection: "ws_subscribe_example_kt"
     Record IDs: ["SEvM4hFDzkJIgjlT-aMR3C_wMYrZhtKqrRJtBaBQ-joGA3n2QK8Pj7lT_EDSdsMwX62FsOx8kh0qiOhw7-stXA"]
     Timestamp:  "2026-09-07T21:58:03.408608+00:00"

Inserting record 2...
✓ Inserted: "Lvz0wXBo_15XQK-LxAGUBqH2GuJQ5npSNZCFT03AoU7IRvZ87mol_tl6NR8_kAcO6bm_mIBfhrlm59qbv-5LAQ"

  📡 Notification received:
     Event:      "insert"
     Record IDs: ["Lvz0wXBo_15XQK-LxAGUBqH2GuJQ5npSNZCFT03AoU7IRvZ87mol_tl6NR8_kAcO6bm_mIBfhrlm59qbv-5LAQ"]

=== Unsubscribing ===
✓ Unsubscribed

✓ WebSocket subscription example completed successfully

BUILD SUCCESSFUL in 4s
2 actionable tasks: 1 executed, 1 up-to-date
✅ [32mKotlin subscription test complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
✅ [32mAll WebSocket Subscription Tests Passed![0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
