make test-examples-rust
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.55s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("bCXEI7JzREa4nomIu0BrFOz68JJFU1yYTwmIOhDkE02xCqPZGaJjRaFvZ2PV37Nn5i3aX9ZIObGQYC-39j2TDw")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("bCXEI7JzREa4nomIu0BrFOz68JJFU1yYTwmIOhDkE02xCqPZGaJjRaFvZ2PV37Nn5i3aX9ZIObGQYC-39j2TDw"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("bCXEI7JzREa4nomIu0BrFOz68JJFU1yYTwmIOhDkE02xCqPZGaJjRaFvZ2PV37Nn5i3aX9ZIObGQYC-39j2TDw"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("bCXEI7JzREa4nomIu0BrFOz68JJFU1yYTwmIOhDkE02xCqPZGaJjRaFvZ2PV37Nn5i3aX9ZIObGQYC-39j2TDw"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.66s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "0u0JE-c953bBlBorh6Cut9S1tabpIzBPhR7t-HABc48qoqNB34eMmA_qbIcEsYOm65dWNv21QyDETAf-mJFcBw"

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1777261972731777000",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "0u0JE-c953bBlBorh6Cut9S1tabpIzBPhR7t-HABc48qoqNB34eMmA_qbIcEsYOm65dWNv21QyDETAf-mJFcBw",
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
        "id": "FYfKxH386rTM0fgtjkI39oijBp6_ViCTDaXBMPMWygAdt-dMVYHDPbo1sa3hHVpjUGzENPNJIQ2hhngC5E0NzQ",
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
        "id": "0I6ap8UObm2WUlNNJ_fMSt7i9mn-clX96Q1xy1jpQ8m75mCdD5F8aAM_N9YbRXKPJIn6_RORN3XWg9lyL1ExoA",
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.69s
     Running `target/debug/examples/batch_operations`
✓ Authentication successful

=== Batch Insert ===
✓ Batch inserted 5 records
✓ Verified: Found 15 total records in collection

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
✓ Batch updated 3 records
✓ Verified: Record updated with status="active"

=== Batch Delete ===
✓ Batch deleted 3 records
✓ Verified: Records successfully deleted (not found)

✓ All batch operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.90s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.79s
     Running `target/debug/examples/collection_management`
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: "lbW9qPpQfz0JbZLO6O6bet5ZSlqmiTQrTLigD-IIS69S_AjkLtvBJUDG2EK5k9xWyYmxzvzjowT7-OfjJfRNgg"

=== List Collections ===
Total collections: 68
Sample collections: ["integ_test_ingest_748f2fd4", "certificate_diagnostics_staging", "chat_goals__ek0_testing", "certificate_diagnostics_testing", "enriched_users", "chat_configurations__ek0_testing", "measure_inventory_1776866928", "chat_raw_completions__ek0_testing", "agent_triggers__ek0_testing", "products", "measure_inventory_1777127393", "schema_documents_client_py", "chat_tasks__ek0_testing", "measure_inventory_1777125865", "functions__ek0_testing", "orders", "chat_messages__ek0_testing", "agent_audit__ek0_testing", "measure_inventory_1776948968", "agent_evals__ek0_testing", "measure_inventory_1776947441", "agent_eval_runs__ek0_testing", "schedules__ek0_testing", "agent_traces__ek0_testing", "gemini_caches__ek0_testing", "customer_memories__ek0_testing", "certificate_diagnostics_production", "schema_products_client_py", "test_collection_inventory", "measure_inventory_1776866845", "measure_inventory_1777123108", "measure_inventory_1777165033", "measure_inventory_1777129886", "certificate_failures_testing", "chat_agent_configs__ek0_testing", "test_collection", "measure_inventory_1777065029", "measure_inventory_1777130146", "measure_inventory_1777161754", "measure_inventory_1777129909", "websocket_test", "measure_inventory_1777160826", "measure_inventory_1777165093", "certificate_failures_development", "cached_locations", "agent_webhooks__ek0_testing", "agent_usage__ek0_testing", "measure_inventory_1776786378", "client_chat_sessions_python", "certificate_diagnostics_development", "measure_inventory_1777124546", "users", "certificate_failures_production", "schema_employees_client_py", "batch_users", "measure_inventory_1776919713", "test_accounts", "cached_zones", "demo_collection", "measure_inventory_1777164986", "audit__ek0_testing", "ttl_cache", "ws_ttl_test", "distributed_locks", "certificate_failures_staging", "ip_locations", "cached_module_versions", "schema_users_client_py"]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.10s
     Running `target/debug/examples/document_ttl`
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "luup7beaxxSnRlgXzUfdvIYhukQ0z9dcLq_iqhRi4tXr1AlMUM7MtEZohM41EDco05LEF42xbzMa-hGYwAVaaQ"

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: "Ek_2kMpT9cvFO7o2N3FW746i7UukuRzEhMxxGXVuXtp1BKSt0RR3bm_HUUmUKohN_eo9-xQcxntYc_3ebx7xcA"

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: "3JmiUgeSeJbILaR83j4cmZYpsg5xQPALcIfcIU3Fgm7UJ99kKJakZt-lgEpx8teqQPomITg2utCfwoC24p6Ssw"

=== Query Documents ===
✓ Found 5 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.57s
     Running `target/debug/examples/websocket_ttl`
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: String("TrSwARmQ4HBWaXVu49tNLKK92rsYgNCZ9QnC6jnt8zQbhhxIMUcAce9z2IgpNvdf0Gn0ipgHGRuSE74ZtRFTLw")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 2 record(s) via WebSocket

Record 1:
  id: "TrSwARmQ4HBWaXVu49tNLKK92rsYgNCZ9QnC6jnt8zQbhhxIMUcAce9z2IgpNvdf0Gn0ipgHGRuSE74ZtRFTLw"
  name: {"type":"String","value":"WebSocket TTL Test"}
  ttl: "2026-04-27T04:59:03.970497Z"
  value: {"type":"Integer","value":42}

Record 2:
  id: "9SqqwWR9zUSDFVuwLWcnLUKBoDMi54qqD_bwkPcMUvLzpIHU9CEf-MLIb7O97gCrbeaEMwPj2fIFEDNr0nCj6Q"
  name: {"type":"String","value":"WebSocket TTL Test"}
  ttl: "2026-04-27T04:41:50.204819Z"
  value: {"type":"Integer","value":42}

✓ WebSocket TTL example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.70s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Functions Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: I3ZpoHgU0x1cu7j6wowsAS_OgMwIHvrSyx617mAGlZUZUMt2NFuPvfYXWnF-azOkqzvl4pKDGeQ2oeCAg_eTQw
📊 Found 25 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: "I9t5ire_XpCMOIsf488nIsX3ztfFQ4dt7DUfyqDjP948eNMTAXA6_aIEcHEVxe1vqPuqXOsulOv9llzQW3bbKQ"
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Function saved: YpiwP5o5lm09rsI4QBxoOEooxnRrAu2AGH91etJaG5R3LuHk5ZjklsC662y_tNsulMSb4c_VHZVR1p155tk-7Q
📊 Found 6 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: BGLR4jy432_BQuB986wwNxrmjAEn9hfBUFcL77Y9x_WzOpLezwt92OVNWxgZaYZaOOlYnKomMA7EXrK0Q0wF8g
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":50.0,"count":10,"max_score":90,"status":"inactive"}
   {"avg_score":60.0,"count":10,"max_score":100,"status":"active"}

📝 Example 4: Function Management

📋 Total scripts: 49
🔍 Retrieved script: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.66s
     Running `target/debug/examples/transactions`
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: wTamGcr_eiUhLHpEZFRv1Xl_YaRXCoSFjNfWaCR9_w0KPGLSY19OTMIy9_4ivs5hQl41i5ibQtknZlzuf4e9kA
Created Bob: $500 - ID: 76FIIAupnSyAWCGhstGqEeqEp37BAGa3DXXwfcMk318OvXFkop15zc_hbYkCqHg083Xkuu_G6JZYpASTi-gSwg

=== Example 1: Begin Transaction ===
Transaction ID: 9496e4dc-b96d-413d-96ca-492c76a7f442

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
New transaction: 038ee328-8453-4478-b93b-4cdf56b9d06f
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"type":"Integer","value":700}

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✅ [32mRust direct examples complete![0m
🛠️  [36mBuilding client library...[0m
cargo build -p ekodb_client
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.54s
✅ [32mClient build complete![0m
🧪 [36mRunning Rust client library examples...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.50s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record({"id": String("oV0bAYTQKmwf1XFRuZlJLY0Fv2eGvN_ByRcKcq74ooe1PwQSHah0yz6s0ySQ52obQ4YuHQJtMvznjEn7IoY_kQ")})

=== Find by ID ===
Found: Record({"categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "value": Object({"type": String("Integer"), "value": Integer(42)}), "price": Object({"value": Float(99.99), "type": String("Float")}), "metadata": Object({"type": String("Object"), "value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})})}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "id": String("oV0bAYTQKmwf1XFRuZlJLY0Fv2eGvN_ByRcKcq74ooe1PwQSHah0yz6s0ySQ52obQ4YuHQJtMvznjEn7IoY_kQ"), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "created_at": Object({"type": String("DateTime"), "value": String("2026-04-27T03:59:40.053830Z")}), "name": Object({"value": String("Test Record"), "type": String("String")}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-04-27T03:59:40.053830Z")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 5 dims
  categories (Set): 2 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record({"categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "value": Object({"type": String("Integer"), "value": Integer(42)}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "created_at": Object({"value": String("2026-04-27T03:59:40.053830Z"), "type": String("DateTime")}), "price": Object({"value": Float(99.99), "type": String("Float")}), "metadata": Object({"type": String("Object"), "value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")})}), "id": String("oV0bAYTQKmwf1XFRuZlJLY0Fv2eGvN_ByRcKcq74ooe1PwQSHah0yz6s0ySQ52obQ4YuHQJtMvznjEn7IoY_kQ"), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "name": Object({"type": String("String"), "value": String("Test Record")}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")})})]

=== Update Document ===
Updated: Record({"value": Object({"type": String("Integer"), "value": Integer(100)}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "name": Object({"value": String("Updated Record"), "type": String("String")}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "created_at": Object({"value": String("2026-04-27T03:59:40.053830Z"), "type": String("DateTime")}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "id": String("oV0bAYTQKmwf1XFRuZlJLY0Fv2eGvN_ByRcKcq74ooe1PwQSHah0yz6s0ySQ52obQ4YuHQJtMvznjEn7IoY_kQ"), "metadata": Object({"value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")}), "type": String("Object")}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")})})

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: O1yDdaBTK7o-5L05yrgEgnTlOKMllBTeekRpeBFyCG7GndW7mZ6aZoFuSYqYfBOLYohlNiPBsuRplJEmBEPrBA

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  name: "WebSocket Test Record"
  value: 42
  id: "O1yDdaBTK7o-5L05yrgEgnTlOKMllBTeekRpeBFyCG7GndW7mZ6aZoFuSYqYfBOLYohlNiPBsuRplJEmBEPrBA"
  active: true

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/examples/client_collection_management`
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "WxKMbPfYjo38pyMeXM_6CDZlP2ZhIffeenw_DRK7ugRbMwKTuqX2WSOF41tqmHaGf8WmCoPY6VK9N2kdOc1M8w"

=== List Collections ===
Total collections: 68
Sample collections: ["integ_test_ingest_748f2fd4", "certificate_diagnostics_staging", "chat_goals__ek0_testing", "certificate_diagnostics_testing", "enriched_users"]

=== Count Documents ===
Document count: 1

=== Check Collection Exists ===
Collection exists: true

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.29s
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
  cache:product:1: Record({"name": String("Product 1"), "price": Float(29.99)})
  cache:product:2: Record({"name": String("Product 2"), "price": Float(39.989999999999995)})
  cache:product:3: Record({"name": String("Product 3"), "price": Float(49.989999999999995)})

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 7

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== KV Batch Delete ===
✓ Batch deleted 3 keys
  cache:product:1: deleted
  cache:product:2: deleted
  cache:product:3: deleted

✓ All KV operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/examples/client_transactions`
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: q0ZUrf9ETLEZbk0RtN_oFS1zEf37F_ns8qlmVFz-Ya63dp3gaAYEb7lqtTMoSfOAwaXp6-2ZxS1zCP_BIYKfgw
Created Bob: $500 - ID: EmL8CKsZUzQ0DdC6LPDLK0tpp14-tGs3_WrnMYD0rkk8_bWC6aJBmMMKjqGQ9_oaj9sb7mDRSJhHEF6r-rFJEg

=== Example 1: Begin Transaction ===
Transaction ID: 46d37def-c27c-4949-b0cd-94bb25ae6291

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 0737f74b-2299-442c-abee-8c642aecd04b
Status before rollback: "Active"
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/examples/client_query_builder`
=== ekoDB Query Builder Example ===

=== Inserting Sample Data ===
✓ Inserted 5 users

=== Simple Equality Query ===
✓ Found 3 active users
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Alice")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"type": String("String"), "value": String("Eve")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"value": String("David"), "type": String("String")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"type": String("String"), "value": String("Eve")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Alice")}))

=== NOT IN Operator ===
✓ Found 4 users not inactive

=== String Pattern Matching ===
✓ Found 5 users with @example.com email

=== Regex Query ===
✓ Found 0 users with names starting with A-C

=== Complex Query (active AND age >= 28 AND score > 1500) ===
✓ Found 2 users matching all conditions
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("David")}))

=== OR Query ===
✓ Found 2 users with age < 28 OR age > 32
  - Some(Object({"type": String("String"), "value": String("Alice")}))
  - Some(Object({"value": String("Charlie"), "type": String("String")}))

=== Sorted Query (by score descending) ===
✓ Top 3 users by score:
  1. Some(Object({"type": String("String"), "value": String("Bob")}))
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
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
     Matched: ["title.value", "description.value", "description", "title"]
  2. Score: 2.0000
     Title: Some(Object {"type": String("String"), "value": String("Python for Data Science")})
     Matched: ["description.value", "description"]
  3. Score: 1.0000
     Title: Some(Object {"type": String("String"), "value": String("Rust Programming")})
     Matched: ["description.value", "description"]
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/examples/client_schema`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("Ix7Gsk-QdimDYGy6o97LcV4i4njCnVxWTfl2O3uZoSh-yv8Cqd_dQsw-9YvhWy4s_FxY77EiEoAyEETv9wbFGQ"))
✓ Inserted user 2: Some(String("ENQU3kC9WheYFurh3b-yfwq498ug66pu_CqkAKTWCZRay01G9jnABdUXEVxyIbidV2L2c24hjRM_56AYSxS90g"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - age: Integer
  - email: String
    (required)
  - status: String
  - title: String
    (required)

=== Listing Collections ===
✓ Total collections: 68
  Sample: ["integ_test_ingest_748f2fd4", "certificate_diagnostics_staging", "chat_goals__ek0_testing", "certificate_diagnostics_testing", "enriched_users"]

=== Cleanup ===
✓ Deleted collection

✓ All schema management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/examples/client_joins`
=== ekoDB Rust Client - Join Operations Example ===

=== Setting up sample data ===
✓ Sample data created

=== Example 1: Single collection join (users with departments) ===
✓ Found 2 users with department data
  - Alice Johnson: Engineering
  - Bob Smith: Sales

=== Example 2: Join with filtering ===
✓ Found 1 users in Engineering
  - Alice Johnson: Building A

=== Example 3: Join with user profiles ===
✓ Found 2 users with profile data
  - Alice Johnson: Senior Software Engineer
  - Bob Smith: Sales Manager

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/examples/client_document_ttl`
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "sgXZtYUTVtGBB_Um0NC1rfwg0JSQmeUMCDpKWcR5VEwZf0mSbnGpMRyLl9zVVQYtTgUVoMjdupw1tlWjY20vDg"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("YDzL2vMgRKrjX1TkAxTRfxrhJHfAIzkWW2NLR6hN0iJVRL4yGX4NWFRY0fDfoi8Mob3HDgezYP4tRwGCIypiPw"))

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running `target/debug/examples/client_websocket_ttl`
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Some(String("QDG-R85qVHI4IlNemcdrn1xdQdK8eaM-qNt17nk9usZHsk982QRaK-9jsabL7yon1XuSLVKKDXB3b5tap65bXg"))

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/examples/client_edge_cache`
=== ekoDB as Edge Cache - Simple Example ===

Setting up edge cache collection...
✓ Cache entry created

Creating edge cache lookup script...
✓ Edge cache script created: dXJ_n8AOtePBGmqkl-O1_zE30o232N8CRRqmb1c1h0fkAMjfLH-8nvzlVp1sZVBktt1OtOBptuxymDsVc3tuFA

Call 1: Cache lookup
Response time: 19ms
Found 1 cached entries

Call 2: Cache lookup (connection warm)
Response time: 17ms
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.19s
     Running `target/debug/examples/client_functions`
=== ekoDB Rust Client - Functions Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: QHBftUwdQ1AePjyHVWBE8lVe29oZ98I6P__1myWT90-JvcyyfZe_UnwNedfoc0zAVmbWTs62g1sqP40VwWVthw
📊 Found 35 active users

📝 Example 2: Parameterized Function

✅ Function saved: N2HNAmVCWxapGbpH0V-qXT0vAM1FB6l9V6N6HUISqe9POC4z4BB2PbZcRTJNODsj0OpcPQKWRsUxxDa5zH1FLg
📊 Found 35 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: bv2t2fNrSljTo-iEpXj9ky0KINAofYMFCUmZJaInyJiJR96rjeQlOAgjIQqE5Wqd4XDbxbGWxdwN87bgLxRg7A
📊 Statistics: 3 groups

📝 Example 4: Function Management

📋 Total functions: 51
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.


✅ All examples completed!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
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
   ⏱️  Duration: 44.515084ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "address": {
      "city": "Gwenborough",
      "zipcode": "92998-3874",
      "street": "Kulas Light",
      "geo": {
        "lat": "-37.3159",
        "lng": "81.1496"
      },
 ...

Second call (cache hit - from cache):
   ⏱️  Duration: 44.669541ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "phone": "1-770-736-8031 x56442",
    "name": "Leanne Graham",
    "company": {
      "catchPhrase": "Multi-layered client-server neural-net",
      "name": "Romaguera-Crona",
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
   - fetch_enriched_user: Used in 50 workflows
   - get_engineering_user: Specific workflow


✅ All composition examples completed!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running `target/debug/examples/client_functions_complete`
🚀 ekoDB Rust Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: lm489bdx-OiO-5KfxECoC3XZB0l2KkGNqj0Q-dZd4ZV7kOWurb87-eWxYBJ4BMsjwfL6yMZi4kopVb7YUlZswQ
📊 Found 2 product groups
   Record({"count": Integer(2), "category": String("Furniture"), "avg_price": Float(474.0)})
   Record({"count": Integer(3), "avg_price": Float(575.6666666666666), "category": String("Electronics")})
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   Record({"category": String("Electronics"), "count": Integer(3)})
   Record({"count": Integer(2), "category": String("Furniture")})
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.19s
     Running `target/debug/examples/client_functions_kv_wrapped`
🚀 ekoDB Rust KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: Some(String("JeOArDHSZ_BRySph0pcAPMoHagdET1IHoO5Ykpo7rEwGmQK8pIfr6YwWn7btlKaliUoKBDhzJeXjnUCU3MwXig"))
✅ Inserted 2 products with wrapped types

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: uEpvySfFtVzoEuWb0z0P04WW4guz4pmTsCFqPuK_ir9m0un_hFou8dyzwf0_DRKSN2a3GRFnyM59v7fWgCtM7g
📊 function executed
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: Some(Object {"role": String("admin"), "userId": String("user_abc")})
🔍 Key exists: true
✅ Set cached data
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: UdWS4uSWWqwmmhXKQGAz5lfiE2BF4PFgl484QYmB3hAghye7kSjwuSLm1vSFp6vQJB1qB0WcDeZjI-eDja9h_Q
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: UFL8wQ1JxfRfWV95Si1mOVaI786tBEsDlKs7SXosp70amBcsv22JFSxmnekQnXOkUakLXLxfCzGHm5Xd6FL1Qw
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/examples/client_swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_rs (LgAhPIbkPFJk3MuEhYX1qu7iMPmOiU-EAqerrkxqsqpUNgQubK_bxv8ZVRhSOWbZFNZMHccXeJYU4Vl50-aDew)

Step 2: First call - Cache miss, fetches from API
Result: FunctionStats { input_count: 0, output_count: 1, execution_time_ms: 0, stages_executed: 2, stage_stats: [] }
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 44ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/examples/client_swr_native`
=== ekoDB Native SWR Function ===

This example shows the new simplified SWR function that replaces
the manual FindById → If → HttpRequest → Insert pattern.

Example 1: Basic GitHub User Cache with Native SWR
─────────────────────────────────────────────────────

✓ Created native SWR script: github_user_native (xAMSlxvMhHth5pZfKDEwv4UUD29CBWAcBH9tIjICbr5jdqlH-2em6o52jkM-gj8JhOfkL96sGcfHdf1jxKj8xA)

First call (cache miss - will fetch from GitHub API):
  Response time: 253ms
  Records returned: 1
  ✓ Data fetched from API and cached with 15m TTL

Second call (cache hit - instant from KV store):
  Response time: 18ms
  Speedup: 14.1x faster
  ✓ Lightning fast cache hit


Example 2: SWR with Audit Trail Collection
─────────────────────────────────────────────────────

✓ Created SWR script with audit trail: product_swr_audit (qu3xP8oycSNtfq44CYqimxrzsTVxls2N6tqmfoeD1YOEBuJJYq_fnIycUITbHZetF5UCrfLRjnLEXe2XBYRPFw)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
─────────────────────────────────────────────────────

Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (I3-uKfe-cSMnktN7JE0SLCBIOCkCIu2A3Sqr3JZ9TD11uwQwQyks7e09Ga0q3FUt033N-CSZnw_YOHVDVvnD5g)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
─────────────────────────────────────────────────────

✓ Created dynamic TTL script: flexible_cache (8dzxDdU-EbeQnlt31OSiEFJCE_Kd9O5lgEv9lNBgBdY3gwRRXnjIaiBcFwhUskkYC3kuQTVywVtcj3P3KofoxA)
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

    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.38s
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
   Record({"count": Integer(5), "category": String("Electronics"), "avg_price": Float(367.0)})
   Record({"category": String("Furniture"), "avg_price": Float(365.6666666666667), "count": Integer(3)})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.19s
     Running `target/debug/examples/client_functions_ai`
🚀 ekoDB Rust AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer several benefits:

1. **High-Dimensional Data Handling**: They efficiently store and query high-dimensional vectors, making them ideal for AI applications.

2. **Similarity Search**: They enable fast nearest neighbor searches, crucial for recommendation systems and image/search retrieval.

3. **Scalability**: Designed to handle large datasets, they can scale horizontally to accommodate growing data needs.

4. **Performance**: Optimized for performance, vector databases can quickly process complex queries that involve mathematical computations on vectors.

5. **Integration with ML Models**: They seamlessly integrate with machine learning and deep learning workflows, supporting feature extraction.

6. **Flexibility**: Support for various data types (text, images, audio) enables versatility in handling different applications.

7. **Support for Hybrid Queries**: Some vector databases can support both traditional and vector-based queries, providing more comprehensive data access capabilities. 

8. **Real-time Processing**: Many vector databases provide real-time indexing and searching, which is essential for applications needing immediate results.
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.19s
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
   Record({"count": Integer(3), "status": String("inactive")})
   Record({"count": Integer(7), "status": String("active")})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/examples/client_functions_search`
🚀 ekoDB Rust Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Database Design Principles (Database)
   2. Natural Language Processing (AI)
   3. Vector Databases Explained (Database)
   4. Introduction to Machine Learning (AI)
   5. Getting Started with ekoDB (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   Record({"count": Integer(2), "category": String("AI")})
   Record({"count": Integer(3), "category": String("Database")})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_chat_basic`
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: ZJBsEqiQwqcx9ekHH1LJwIvzvzTdnC7Q3gKepN4uFDH24lQvEw86Zt4OjYemdR-kAl1wDJfQUyB27GHW_M8zAA

=== Sending Chat Message ===
Message ID: E2ru7eo7zBw2oeLJpnGpdyXOYB2amL3OV9cmxXxEfWgXVhAgyamGG0XNuOF5DL7xJkjjlnMxdrDblH4V_7Kfyw

=== AI Response ===
Response 1: **ekoDB Overview:**
ekoDB is a high-performance database that features intelligent caching, real-time capabilities, and AI integration. It allows for efficient data handling and smart processing of queries.

**Key Features:**
1. **AI Chat Integration:** The chat feature enables users to query the database using natural language, providing AI-powered responses with relevant context.
2. **Search Capabilities:** ekoDB supports various search methods, including:
   - Full-text search: Allows for searching text-based data efficiently.
   - Vector search: Utilizes vector embeddings to find semantically similar records.
   - Hybrid search: Combines text and vector searches for optimal results, including automatic context retrieval.

These features make ekoDB versatile and powerful for developers and businesses looking for modern database solutions.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["title", "category", "content"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("kPH0eMiMILYw37UV40t5V6G3To-S3ffBzFWiDqLSsfMwYFGZWlWzIUzY2NSHet0MwKIL_u53qqCk7S2d9qhquw"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "title"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("V5wbeUgKcr4HHNhHHNdoN1DQK7QvyHAU5VmIJuUTsHnygIRHeekHQ5Y_S24fHBEFUKHm0mO0C2Cze3mkQew_Bw"), "title": String("Introduction to ekoDB")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "category", "title"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("D8hqRMxfZIUaJG_BWUTHTQr-8DxIvnSnDsK-jfQ3knQ77VqOL1hH318AoZiZ8dMPojp11M81jWGZBVS2ym2weg"), "title": String("Search Features")}

Execution Time: 23817ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.53s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: g9ra5ztrVnKvMfbmd7fXqoBpivXDrhQSwHBItm_QIgSCShFYpYV66OQfdDPQ5eMua5ODx125eiFBL8XZdBDEMw

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you need more information or if there are other products you want to know about, just let me know!

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
✓ Created second session: 7QABNA53RzBcnLMmJc9amE0GVxWHMMq7Oueo8VT7gcY5763MC499atxCLTvhm5QgEwwa4IgVmrbvoP36oZt54Q
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.61s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 5XPKuizkmR3ypDQ_t5WiFCgjePHLoyt7e3lPokHKvZQOC0bdGUS1hnyBEZ31WDB6i15z1efqoeDbuUFpkJCvPw
=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Product**: ekoDB
  - **Description**: A high-performance database product with AI capabilities
  - **Price**: $99 

If you have any further questions or need more information, feel free to ask!

✓ Message 2 sent
  Response: The price of the product ekoDB is **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 9TQPdjD_URweEqBwmAB4lY12WCE8UZAi2S7M5rLvU4VRbCZRwRr7qg_zMCUUzde_WRD_wX-5M6Ixa6c5P1RK4Q
  Parent: 5XPKuizkmR3ypDQ_t5WiFCgjePHLoyt7e3lPokHKvZQOC0bdGUS1hnyBEZ31WDB6i15z1efqoeDbuUFpkJCvPw

=== Listing Sessions ===
✓ Found 135 sessions
  Session 1: 9TQPdjD_URweEqBwmAB4lY12WCE8UZAi2S7M5rLvU4VRbCZRwRr7qg_zMCUUzde_WRD_wX-5M6Ixa6c5P1RK4Q (Untitled)
  Session 2: 5XPKuizkmR3ypDQ_t5WiFCgjePHLoyt7e3lPokHKvZQOC0bdGUS1hnyBEZ31WDB6i15z1efqoeDbuUFpkJCvPw (Untitled)
  Session 3: 7QABNA53RzBcnLMmJc9amE0GVxWHMMq7Oueo8VT7gcY5763MC499atxCLTvhm5QgEwwa4IgVmrbvoP36oZt54Q (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 9TQPdjD_URweEqBwmAB4lY12WCE8UZAi2S7M5rLvU4VRbCZRwRr7qg_zMCUUzde_WRD_wX-5M6Ixa6c5P1RK4Q

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.41s
     Running `target/debug/examples/client_chat_models`
✓ Client created

=== Get All Chat Models ===
Available chat models by provider:

OpenAI models (128):
  - gpt-4-0613
  - gpt-4
  - gpt-3.5-turbo
  - gpt-5.5-pro-2026-04-23
  - gpt-image-2-2026-04-21
  - gpt-5.5
  - gpt-5.5-2026-04-23
  - gpt-5.5-pro
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
  - gpt-5.4
  - gpt-5.4-nano-2026-03-17
  - gpt-5.4-nano
  - gpt-5.4-mini-2026-03-17
  - gpt-5.4-mini
  - gpt-image-2
  - gpt-3.5-turbo-16k
  - tts-1
  - whisper-1
  - text-embedding-ada-002

Anthropic models (9):
  - claude-opus-4-7
  - claude-sonnet-4-6
  - claude-opus-4-6
  - claude-opus-4-5-20251101
  - claude-haiku-4-5-20251001
  - claude-sonnet-4-5-20250929
  - claude-opus-4-1-20250805
  - claude-opus-4-20250514
  - claude-sonnet-4-20250514

Perplexity models (5):
  - sonar
  - sonar-pro
  - sonar-deep-research
  - sonar-reasoning
  - sonar-reasoning-pro

=== Get Models for Specific Provider ===

openai models (128):
  - gpt-4-0613
  - gpt-4
  - gpt-3.5-turbo
  - gpt-5.5-pro-2026-04-23
  - gpt-image-2-2026-04-21
  ... and 123 more

anthropic models (9):
  - claude-opus-4-7
  - claude-sonnet-4-6
  - claude-opus-4-6
  - claude-opus-4-5-20251101
  - claude-haiku-4-5-20251001
  ... and 4 more

perplexity models (5):
  - sonar
  - sonar-pro
  - sonar-deep-research
  - sonar-reasoning
  - sonar-reasoning-pro

✓ Chat Models API example complete
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/examples/client_user_functions`
✓ Client created

=== Create User Function ===
Created user function with ID: hrsyXkJ0YbCWdJdlEzAxQz37QKAXzR45jz_ONe3rbg-ExOtgm6tKOAGgkuligZ2skn5Tm0-jBFj8_LqJIzbE2g

=== Get User Function ===
Retrieved: get_active_users_rs - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 58 user functions:
  - validate_user: Check if user exists
  - ingest_integ_test_ingest_748f2fd4: Ingest integ_test_ingest_748f2fd4 from API
  - fetch_slim_user: Validate and slim down user
  - get_active_users_rs: Get Active Users
  - add_measure_inventory_1776919713: Add
  - fetch_slim_user: Validate and slim down user
  - get_active_users: Get Active Users (Updated)
  - fetch_and_store_user: Fetch user from API and cache in KV
  - add_measure_inventory_1776947441: Add
  - add_measure_inventory_1777161754: Add
  - add_measure_inventory_1777127393: Add
  - get_users_by_status: Get Users By Status
  - add_measure_inventory_1777130146: Add
  - add_measure_inventory_1776866928: Add
  - add_measure_inventory_1777125865: Add
  - get_user_wrapper: Wrapper that calls fetch_user
  - get_active_users: Get Active Users (Updated)
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_active_users_paginated: Get Active Users (Paginated)
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_active_users_updated: Get Active Users (Updated)
  - add_measure_inventory_1777160826: Add
  - add_measure_inventory_1777165093: Add
  - add_measure_inventory_1777065029: Add
  - get_user_wrapper: Wrapper that calls fetch_user
  - fetch_slim_user: Validate and slim down user
  - get_verified_user: Get verified and validated user
  - get_verified_user: Get verified and validated user
  - get_high_scoring_active_users: Get High Scoring Active Users
  - fetch_user: Fetch user by code
  - get_user_wrapper: Wrapper that calls fetch_user
  - add_measure_inventory_1777123108: Add
  - get_high_scoring_active_users: Get High Scoring Active Users
  - add_measure_inventory_1777129886: Add
  - get_active_users_paginated: Get Active Users (Paginated)
  - get_verified_user: Get verified and validated user
  - add_measure_inventory_1777129909: Add
  - get_active_users_paginated: Get Active Users (Paginated)
  - get_active_users_updated: Get Active Users (Updated)
  - integ_test_bb71e36e: Integration Test Function
  - fetch_user: Fetch user by code
  - get_active_users_updated: Get Active Users (Updated)
  - cache_api_call_py: Cache External API Call
  - get_users_by_status: Get Users By Status
  - add_measure_inventory_1776786378: Add
  - get_users_by_status: Get Users By Status
  - add_measure_inventory_1777164986: Add
  - add_measure_inventory_1777165033: Add
  - validate_user: Check if user exists
  - fetch_user: Fetch user by code
  - add_measure_inventory_1776948968: Add
  - get_active_users: Get Active Users (Updated)
  - swr_user: SWR pattern for user data (KV-based)
  - add_measure_inventory_1777124546: Add
  - swr_user: SWR pattern for user data (KV-based)
  - swr_user: SWR pattern for user data (KV-based)
  - validate_user: Check if user exists
  - add_measure_inventory_1776866845: Add

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/examples/client_convenience_methods`
=== ekoDB Convenience Methods Example ===

=== Record Builder Pattern ===
✓ Created record with fluent builder: Record({"id": String("rUH_MKEMnpm9LXaPpPFZHJrPULf7D5Uh3cHcy5UfDT4W8YX9VlCgS0okCgqp1cQvVLqDZJZ-S2nW-iy0ZszNQQ")})

=== Upsert Operation ===
✓ First upsert (insert): Record({"id": String("VIwpAEKhBEQ-lc2_CY9XSj2f81ZWFBzvLkb8CO8hA-oUJsCOBr0IKsHRUnxcwNiNyCx8HgSIzbmjA8arMAVDiQ")})
✓ Second upsert (update): Record({"id": String("pBzeatuj5EVKDZleAgSHeAftlKp7m6nFMQuSX16F4iJOysXbwNiOdaZqQRijkVAp5r7peEfGxXEz8Lsvvq6ARQ")})

=== Find One Operation ===
✓ Found user by email: Record({"email": Object({"value": String("alice@example.com"), "type": String("String")}), "age": Object({"type": String("Integer"), "value": Integer(28)}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "id": String("rUH_MKEMnpm9LXaPpPFZHJrPULf7D5Uh3cHcy5UfDT4W8YX9VlCgS0okCgqp1cQvVLqDZJZ-S2nW-iy0ZszNQQ"), "name": Object({"type": String("String"), "value": String("Alice Johnson")})})
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/examples/bypass_ripple_example`
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: Record({"id": String("IhatSHSe1YPE7DOx5MNRYzUGU13Fhv0dnuaCCkVqT3z1FTIIfTYCQWv-vO_vmzrc2pc6GLl0MB5jVK-OiJyvpA")})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: Record({"id": String("irKGd0DH6Hoc1734leLUd6tHomlP_gzW4AtO7pUGor19I4SSEzd5Ceg0ubIUpS8VhCnrFxJZ7VUW4xt0YxGyuw")})

3. Update with bypass_ripple:
   Updated with bypass_ripple: Record({"price": Object({"type": String("Integer"), "value": Integer(150)}), "name": Object({"value": String("Product 1"), "type": String("String")}), "id": String("IhatSHSe1YPE7DOx5MNRYzUGU13Fhv0dnuaCCkVqT3z1FTIIfTYCQWv-vO_vmzrc2pc6GLl0MB5jVK-OiJyvpA")})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: Record({"id": String("vgHBB_WXyikiBpn69vy0FHHtKOkh4OacJ67jWtI5AwNpcTYw8hhoqUniuay_5IzDUQL2ou07oudtbfqTFMXkig")})

✅ All bypass_ripple operations completed successfully!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/projection_example`
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ["email", "id", "name"]

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["user_role", "name", "id", "avatar_url", "bio", "created_at", "status", "email", "age"]

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)

Example 4: Query inactive users with profile fields
  Found 1 inactive users

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ["created_at", "age", "bio", "user_role", "email", "secret_token", "name", "password", "api_key", "id", "avatar_url", "status"]
  Projected query:
    - 3 fields per record
    - Fields: ["name", "email", "id"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
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
