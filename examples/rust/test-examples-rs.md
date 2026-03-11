make test-examples-rust
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("j5WFGSbWLJPU8X9TQOcq1GJD5oS_deEuQ1AgpGayGBQs28lsaDtJ5xDqtp_q0tT3KEYeCdUGnor7KGx9mN0DKg")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("j5WFGSbWLJPU8X9TQOcq1GJD5oS_deEuQ1AgpGayGBQs28lsaDtJ5xDqtp_q0tT3KEYeCdUGnor7KGx9mN0DKg"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("j5WFGSbWLJPU8X9TQOcq1GJD5oS_deEuQ1AgpGayGBQs28lsaDtJ5xDqtp_q0tT3KEYeCdUGnor7KGx9mN0DKg"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("j5WFGSbWLJPU8X9TQOcq1GJD5oS_deEuQ1AgpGayGBQs28lsaDtJ5xDqtp_q0tT3KEYeCdUGnor7KGx9mN0DKg"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "Zc7PqlMFMO49_B56vubOWFv8H9g5CTInhjVIV0lpLko9dqy1wx-tOd6_XeZVKvkAupa_wFsi-bkSpqVKZi10nQ"

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
        "id": "3iNacHlOrLs5T9hDNOvghQkwkRQEp81IZ51fafX7mvm0W7-T_AZYHMyPzDRppGPdUb895ykT2KkFB-AigS1bvA",
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
        "id": "Sb7wwI0SJG9yEJuFRPtwizcAVp3BIN9p3UyfWjfyM5n9cEdWvf9ZUo0KW0Ec26o-aNqu9lX6Zba7BE9ObFt3Rg",
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
        "id": "Zc7PqlMFMO49_B56vubOWFv8H9g5CTInhjVIV0lpLko9dqy1wx-tOd6_XeZVKvkAupa_wFsi-bkSpqVKZi10nQ",
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
        "id": "biFqh34zaiKvmVm_AsobaPP65GYmXSLsFcsLJGV9TsX701PhiY4YQht5Ic36wfZ3pL3js_GmW27bm485ujU8TQ",
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
        "id": "mWL_XtxbXMBH3hWNMk9n3vMoBGe_1M5OEi7hTywqjnKnkJ0V2cgt-RIlZwazqu4TZoKlkSyciuKWunFUsgBttw",
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
✓ Retrieved 5 record(s) via WebSocket

✓ WebSocket example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/batch_operations`
✓ Authentication successful

=== Batch Insert ===
✓ Batch inserted 5 records
✓ Verified: Found 25 total records in collection

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
✓ Batch updated 3 records
✓ Verified: Record updated with status="active"

=== Batch Delete ===
✓ Batch deleted 3 records
✓ Verified: Records successfully deleted (not found)

✓ All batch operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/collection_management`
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: "w_99c9_LzdKZGgiwRCo0VZuuKSZqMPtnDjo4luEr37t0_Btz_h6VJkNANIdTP1-0UBQfC6ToS3YIE6DWlsR4Rw"

=== List Collections ===
Total collections: 36
Sample collections: ["schema_documents_client_go", "ws_subscribe_example_go", "schema_employees_client_js", "schema_products_client_js", "chat_configurations__ek0_testing", "demo_collection", "schema_users_client_py", "enriched_users", "ws_ttl_test", "ws_subscribe_example_py", "scripts__ek0_testing", "batch_users", "github_cache", "schema_employees_client_ts", "schema_products_client_py", "users", "schema_products_client_ts", "schema_users_client_go", "test_collection", "schema_documents_client_js", "schema_products_client_go", "product_cache", "websocket_test", "ws_subscribe_example_rs", "ws_subscribe_example_ts", "ws_subscribe_example_kt", "test_accounts", "schema_documents_client_ts", "schema_users_client_ts", "audit__ek0_testing", "schema_users_client_js", "schema_employees_client_go", "ttl_cache", "chat_messages__ek0_testing", "schema_documents_client_py", "schema_employees_client_py"]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/document_ttl`
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "m_XXPHtVVXmU2gWnBJbyJosIJY7LcNBBGgnfMQz0H2Qpw5RaI0g5HDVEmKEYZN3k-Y2UgbPhD-eCDCCXm6x1OQ"

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: "uYoJPMlDiupM7MckUZeHmFCjWMjMRiGCvHSTq2fzMZ1rFMP0etfYyE9cVgcqUJBnCUw2LWkv0I6w1PAiUD5n4Q"

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: "Cr0WGjxTqN-r7dgv8kUh3e4VQXvMkgoyHyUyOpVri5IJrbdzK7PQwip6PnxFi2OVg_QHyXXKLv0mfD2GmLzwFA"

=== Query Documents ===
✓ Found 5 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/examples/websocket_ttl`
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: String("YNcn9r9kJJwXaQBKRRDt6-FIHULVlCxsYtBJs0B9ZvEqXq0H6FVTk1MPJuuDG200xcxggrKrao38L8c4feB9LA")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 2 record(s) via WebSocket

Record 1:
  id: "O0PFdN4pjHacvUqSqDC0ttCEth6UppYmlpx483FWEZ_3fqrWdDe0yl1daDH0CJlu2x5_KangarCltZIwGHCsUQ"
  name: {"type":"String","value":"WebSocket TTL Test"}
  ttl: "2026-03-11T05:38:25.972126Z"
  value: {"type":"Integer","value":42}

Record 2:
  id: "YNcn9r9kJJwXaQBKRRDt6-FIHULVlCxsYtBJs0B9ZvEqXq0H6FVTk1MPJuuDG200xcxggrKrao38L8c4feB9LA"
  name: {"type":"String","value":"WebSocket TTL Test"}
  ttl: "2026-03-11T06:04:46.987688Z"
  value: {"type":"Integer","value":42}

✓ WebSocket TTL example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Scripts Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: 05w9EPHgLnRF97QtRf1KQC4GxOd_RXyFjh6Ph_YGXxPct_ogy7O0wHzorho88OGmnHptjHLCAuPwFe8C0hyKwA
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: "eLyxZuwKs6aa4sFWuldL-Paws5Yc4E7zhnwvG9i0wB2lEiEpvHgJA6ixLdeZHNTR-SKdMnw8P3ErC5qtrpRRew"
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Script saved: YfPNNJIUdsyg-Ep0ZPhWkoTAruWpHrWbvBLhL0DQlpW4-MfpOyeoSEC5xMNlD_wj3xZwOcsrTj_rdGWGoDvCqw
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: T38AAX9oeEpQxeqTn-_BymQF8hsRI7pC92e9ccncPXdu66927fFjOZdWdY_vTtEs1sFs-eYJVsi6Us1BIFZssg
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":50.0,"count":5,"max_score":90,"status":"inactive"}
   {"avg_score":60.0,"count":5,"max_score":100,"status":"active"}

📝 Example 4: Function Management

📋 Total scripts: 80
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/transactions`
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: T9TpDKpdFN1i_ruJ0rlNKPsdD9FRg_1zrYSGoc1qHUNk-p31Qrw3NHzxDyirT93GOJ7yGs2BJDsvJKuEnaPD5g
Created Bob: $500 - ID: UNcCcQB0SMWfjMaZ2xr7PoKNPnvdHbPG9FAKwTYcoZDUa4MvgfmPzw3XqpJRjpH8-cLtZKC5tWLidu3DaGuCcw

=== Example 1: Begin Transaction ===
Transaction ID: 57f37be8-2d4e-4af4-b3f3-a8baf3f08acb

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
New transaction: c2f91d8a-0378-42f0-b15e-144e89991060
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"type":"Integer","value":700}

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✅ [32mRust direct examples complete![0m
🛠️  [36mBuilding client library...[0m
cargo build -p ekodb_client
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
✅ [32mClient build complete![0m
🧪 [36mRunning Rust client library examples...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record({"id": String("ej1MeYy1EJIgGxDJJ0bnt_Y0GdlY_3kTsT3PnPfiTKJnNK-a0ksr8JImGqY95ljLNR7A3_QZDOP4hhIPsqhXgg")})

=== Find by ID ===
Found: Record({"user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "id": String("ej1MeYy1EJIgGxDJJ0bnt_Y0GdlY_3kTsT3PnPfiTKJnNK-a0ksr8JImGqY95ljLNR7A3_QZDOP4hhIPsqhXgg"), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "metadata": Object({"value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")}), "type": String("Object")}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "created_at": Object({"value": String("2026-03-11T05:04:50.022926Z"), "type": String("DateTime")}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "name": Object({"value": String("Test Record"), "type": String("String")}), "value": Object({"type": String("Integer"), "value": Integer(42)})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-03-11T05:04:50.022926Z")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 5 dims
  categories (Set): 2 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record({"active": Object({"type": String("Boolean"), "value": Boolean(true)}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "id": String("ej1MeYy1EJIgGxDJJ0bnt_Y0GdlY_3kTsT3PnPfiTKJnNK-a0ksr8JImGqY95ljLNR7A3_QZDOP4hhIPsqhXgg"), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "created_at": Object({"type": String("DateTime"), "value": String("2026-03-11T05:04:50.022926Z")}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "value": Object({"type": String("Integer"), "value": Integer(42)}), "metadata": Object({"value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})}), "type": String("Object")}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "price": Object({"value": Float(99.99), "type": String("Float")}), "name": Object({"type": String("String"), "value": String("Test Record")})})]

=== Update Document ===
Updated: Record({"id": String("ej1MeYy1EJIgGxDJJ0bnt_Y0GdlY_3kTsT3PnPfiTKJnNK-a0ksr8JImGqY95ljLNR7A3_QZDOP4hhIPsqhXgg"), "value": Object({"type": String("Integer"), "value": Integer(100)}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "created_at": Object({"type": String("DateTime"), "value": String("2026-03-11T05:04:50.022926Z")}), "name": Object({"type": String("String"), "value": String("Updated Record")}), "price": Object({"value": Float(99.99), "type": String("Float")}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "metadata": Object({"value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")}), "type": String("Object")})})

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: FgL48UqUOcDja3cYlZJRAXC8bw7o-jaYJHmTGNnvEgdJXoyjJd5H1JlQ--03EdgN50JTzvYJ2p2RmOMjEp_6kA

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  value: 42
  name: "WebSocket Test Record"
  active: true
  id: "FgL48UqUOcDja3cYlZJRAXC8bw7o-jaYJHmTGNnvEgdJXoyjJd5H1JlQ--03EdgN50JTzvYJ2p2RmOMjEp_6kA"

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_collection_management`
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "ujYEQStC2xGIZ6gxpav_Y_-gRV_XWFtD0Zg1zT15cmv3kT5cvFTP_qkbkI2qcv6lEaeHnoX2qAESNHqO-QgOAw"

=== List Collections ===
Total collections: 36
Sample collections: ["schema_documents_client_go", "ws_subscribe_example_go", "schema_employees_client_js", "schema_products_client_js", "chat_configurations__ek0_testing"]

=== Count Documents ===
Document count: 1

=== Check Collection Exists ===
Collection exists: true

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
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
  cache:product:2: Record({"price": Float(39.989999999999995), "name": String("Product 2")})
  cache:product:3: Record({"price": Float(49.989999999999995), "name": String("Product 3")})

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 9

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== KV Batch Delete ===
✓ Batch deleted 3 keys
  cache:product:1: deleted
  cache:product:2: deleted
  cache:product:3: deleted

✓ All KV operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_transactions`
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: lv5PT9zNsL6I4b0ztOCh4E2pnZnK574i1XrENdwmCiUvTFbRmIrCbrGZMQro32-dhcqxaGE8oJoihsaCETYOnw
Created Bob: $500 - ID: PatPmpg6rMpBmgzb2xff3ftWTIbiPtxRhu6obK7j1Dx3_Evw0hrZItjasvTRKsfBOwEByQoGQL74TI9z-ot6tA

=== Example 1: Begin Transaction ===
Transaction ID: f4104088-f75f-4e6e-b57f-f8c1ddd51a1f

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 555cdb10-6648-4cb7-835d-23b50188c142
Status before rollback: "Active"
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_query_builder`
=== ekoDB Query Builder Example ===

=== Inserting Sample Data ===
✓ Inserted 5 users

=== Simple Equality Query ===
✓ Found 3 active users
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"value": String("Eve"), "type": String("String")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Eve")}))

=== NOT IN Operator ===
✓ Found 4 users not inactive

=== String Pattern Matching ===
✓ Found 5 users with @example.com email

=== Regex Query ===
✓ Found 0 users with names starting with A-C

=== Complex Query (active AND age >= 28 AND score > 1500) ===
✓ Found 2 users matching all conditions
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"value": String("David"), "type": String("String")}))

=== OR Query ===
✓ Found 2 users with age < 28 OR age > 32
  - Some(Object({"value": String("Charlie"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Alice")}))

=== Sorted Query (by score descending) ===
✓ Top 3 users by score:
  1. Some(Object({"value": String("Bob"), "type": String("String")}))
  2. Some(Object({"value": String("David"), "type": String("String")}))
  3. Some(Object({"type": String("String"), "value": String("Alice")}))

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_search`
=== ekoDB Search Example ===

=== Inserting Sample Documents ===
✓ Inserted 5 sample documents

=== Basic Text Search ===
✓ Found 3 results for 'programming'
  1. Score: 19.8000 - Some(Object {"type": String("String"), "value": String("Rust Programming")})
  2. Score: 6.6000 - Some(Object {"type": String("String"), "value": String("JavaScript Web Development")})
  3. Score: 6.6000 - Some(Object {"type": String("String"), "value": String("Python for Data Science")})

=== Fuzzy Search ===
✓ Found 3 results for 'progamming' (typo)
  1. Score: 1.6500 - Some(Object {"type": String("String"), "value": String("Rust Programming")})
  2. Score: 0.5500 - Some(Object {"type": String("String"), "value": String("JavaScript Web Development")})
  3. Score: 0.5500 - Some(Object {"type": String("String"), "value": String("Python for Data Science")})

=== Field-Specific Search ===
✓ Found 4 results in title/description
  1. Score: 4.0000
     Title: Some(Object {"type": String("String"), "value": String("Machine Learning Basics")})
     Matched: ["description", "title", "title.value", "description.value"]
  2. Score: 2.0000
     Title: Some(Object {"type": String("String"), "value": String("Python for Data Science")})
     Matched: ["description.value", "description"]
  3. Score: 1.0000
     Title: Some(Object {"type": String("String"), "value": String("Database Design")})
     Matched: ["description.value", "description"]
  4. Score: 1.0000
     Title: Some(Object {"type": String("String"), "value": String("Rust Programming")})
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
  2. Score: 6.6000 - Some(Object {"type": String("String"), "value": String("JavaScript Web Development")})

=== Cleanup ===
✓ Deleted collection

✓ All search operations completed successfully
Execution time: 1ms
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_schema`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("pLlC72xP0KSVapI_3zqNcCCCF_e94_mO6T1EfGpwO19rmYM568u6LJNQjinxgqUu4y8YF9pr6PH7LULewVGepA"))
✓ Inserted user 2: Some(String("v0Lz9tmvQ4IvuX7fIcreCV4Ns4Ip5pekHAFLw2h3g8CLA5eYw7YLPpLSsTef5p3ElZRe4C03Mc15KavoKKpgHw"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - title: String
    (required)
  - status: String
  - email: String
    (required)
  - age: Integer

=== Listing Collections ===
✓ Total collections: 36
  Sample: ["schema_documents_client_go", "ws_subscribe_example_go", "schema_employees_client_js", "schema_products_client_js", "chat_configurations__ek0_testing"]

=== Cleanup ===
✓ Deleted collection

✓ All schema management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_document_ttl`
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "sLMclf-h2hukmqjlkHO9j3ue1BkABFxTXROMKiiIpfI5OVGvt94ufySO8AAfHA_fAxBz783Itc90Mxlg-_Mp3Q"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("9dxFidojpx82z7pwCjI2_FJ4QMry5MB1Dr2p-yfPgIOE9t_pBWSx-lHnAUzSgQuF-Ngd18haPgTnL0__kIb5kA"))

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_websocket_ttl`
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Some(String("oNNzj2XDqgAgHzbiGp4uzrh63MkqMoZsG3HTFWJTeHVdXXTSsADu7BegFa9wqe9g4SmX4DhamB2ugY7zv1OqPw"))

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_edge_cache`
=== ekoDB as Edge Cache - Simple Example ===

Setting up edge cache collection...
✓ Cache entry created

Creating edge cache lookup script...
✓ Edge cache script created: uLyDsKUi7FqypU66MrMEEkA8DQ1QAF2qBdr6hVooRd23viM1CrDIx09BnmUh_HRtnIP3avwy4O17YOl4OMdQJw

Call 1: Cache lookup
Response time: 4ms
Found 1 cached entries

Call 2: Cache lookup (connection warm)
Response time: 3ms
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_functions`
=== ekoDB Rust Client - Scripts Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: 1Nar81WVtOax5wIZm6CC74XER6nI_rjpxDgqFyYAKot6GTO665zI9E8R9RLiawp_J3mrZczIzKEe-wmhuloOsQ
📊 Found 45 active users

📝 Example 2: Parameterized Script

✅ Script saved: jVx8O2YIfp0coWaqcV4tdG7UBZJ9zLMdVbdM6qf-08OtdklSbe_kFTjcrmwORtagHgp2YlNJEvQNX31KEgxVxA
📊 Found 45 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: ZKG34TeEGfrS5KNFDiADMW5gMP1K-YU1I1te3W9RqRSwtVU3p_k1EIqWH6zJSZ0LBKcV9CoY1QloD2HexepI0w
📊 Statistics: 3 groups

📝 Example 4: Script Management

📋 Total scripts: 82
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.


✅ All examples completed!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
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
   ⏱️  Duration: 84.906625ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "address": {
      "geo": {
        "lat": "-37.3159",
        "lng": "81.1496"
      },
      "suite": "Apt. 556",
      "city": "Gwenborough",
      "street": "Kulas Light",
     ...

Second call (cache hit - from cache):
   ⏱️  Duration: 5.868375ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "address": {
      "zipcode": "92998-3874",
      "geo": {
        "lat": "-37.3159",
        "lng": "81.1496"
      },
      "suite": "Apt. 556",
      "street": "Kulas Light",
   ...
   🚀 Cache speedup: 16.8x faster!

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_functions_complete`
🚀 ekoDB Rust Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: DSU90szaOb1uYea6NHo_xqK6h_IvovMHHLQ7WPKLt8PEapxrViR6KkbDdZ5G6Q-iVqox-aFZ2TZig_zTx1_KlA
📊 Found 2 product groups
   Record({"category": String("Furniture"), "count": Integer(2), "avg_price": Float(474.0)})
   Record({"category": String("Electronics"), "count": Integer(3), "avg_price": Float(575.6666666666666)})
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   Record({"category": String("Electronics"), "count": Integer(3)})
   Record({"count": Integer(2), "category": String("Furniture")})
⏱️  Execution time: 0ms

📝 Example 4: High Rating Products

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 5: Script with Parameter Definition

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 6: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Script saved
📊 Pipeline executed 3 stages
⏱️  Total execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All complete script examples finished!

💡 This example demonstrates ekoDB's Script system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Parameter definitions
   ✅ Script management (save, call, delete)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_functions_kv_wrapped`
🚀 ekoDB Rust KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: Some(String("6M10dWdL7W9Sehgcs1ntcTJAXWG39pmwscGzpA-0mvhwslvspL-PcmsMhlIBel4oPAPJdbk-XAu_xntU1trlqw"))
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: rDmrpFEzGohehPUMSE9lUHyHlWZbhQfzqhKdsYN9JmwbQPkgSFWx3YzY0iBeF1EllCi0FOd5efLPNkDjcJYWqw
📊 Script executed
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: Some(Object {"role": String("admin"), "userId": String("user_abc")})
🔍 Key exists: true
✅ Set cached data
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: _sVcjrZnPg8XeiOU__5RtRZv85Uaz8a6mOKVNxrKagGCZfgM21uv_eI0Ul8SjFPDqAX6uo1JH6UYfkuhyMT2-w
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: wdvdf1V_36IrfEz7fOOlAdIoFldYiEnEeEbzE70YJxmQjY0PaKPVqoPgnKk1wRwiZdvU6aP-E6zjGDYNjGvvew
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
✓ Created SWR script: fetch_api_user_rs (Ic9zfmYNWxwN73dxt3qnRrvZ6wrrYoRP0vEgjBfVBOPKQxyYIY689MGxZytcokj6qDw6fj7XIbkViZ20VCheXw)

Step 2: First call - Cache miss, fetches from API
Result: FunctionStats { input_count: 0, output_count: 1, execution_time_ms: 0, stages_executed: 2, stage_stats: [] }
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 67ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_swr_native`
=== ekoDB Native SWR Function ===

This example shows the new simplified SWR function that replaces
the manual FindById → If → HttpRequest → Insert pattern.

Example 1: Basic GitHub User Cache with Native SWR
─────────────────────────────────────────────────────

✓ Created native SWR script: github_user_native (Epa9DduCiABnfX3fEvB0AlrnJfE1Q_D6Yfa_Q7gz0mjiFrppN1rQkl9VdBkqxtStbIDtw55jUOh79qs02bNxsQ)

First call (cache miss - will fetch from GitHub API):
  Response time: 116ms
  Records returned: 1
  ✓ Data fetched from API and cached with 15m TTL

Second call (cache hit - instant from KV store):
  Response time: 8ms
  Speedup: 14.5x faster
  ✓ Lightning fast cache hit


Example 2: SWR with Audit Trail Collection
─────────────────────────────────────────────────────

✓ Created SWR script with audit trail: product_swr_audit (sZ5OjrQPEWJJCGx8DYJUtEM7Ir5F7DvjXwXD5ptuVxYxeHNyHuqrOMNL2B4srq4DI6SHoGZrsZ3cMUTVaLUWhQ)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
─────────────────────────────────────────────────────

Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (qet2dhmrl7VjTE_sipx4PKQjrh3l9SQVKQMPmnVRsl_ORjVUjqihxRXH7KnhI8vtmz7B9qUrVzTHukcyOOUfQg)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
─────────────────────────────────────────────────────

✓ Created dynamic TTL script: flexible_cache (7g-H4XaBSBI8S2Ys9LY8J_TWEO8omyLb_VM5mv8XCAY7GZfIzoskVwEIQLokMBkJLqcuSz61gftKL-mvGFnXGw)
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

    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_functions_advanced`
🚀 ekoDB Rust Advanced Scripts Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Script saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Script saved
📊 Category breakdown:
   Record({"count": Integer(5), "avg_price": Float(367.0), "category": String("Electronics")})
   Record({"category": String("Furniture"), "count": Integer(3), "avg_price": Float(365.6666666666667)})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_functions_ai`
🚀 ekoDB Rust AI Scripts Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   1. High Precision: Vector databases provide accurate data representation due to use of geometrical shapes like points, lines and polygons.

2. Scalability: Elements or objects in a vector database can be easily scaled up or down without loss of quality or resolution.

3. Less Disk Space: Vector data is typically more compact in terms of storage compared with raster data. 

4. Easy to Edit: Individual elements in a vector database can be easily added, removed, or modified.

5. Supports Complex Analysis: Vector databases are well-suited for complex spatial analysis, such as determining proximity, performing network analysis, etc.
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_functions_crud`
🚀 ekoDB Rust CRUD Scripts Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Script saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Script saved
📊 User counts by status:
   Record({"status": String("inactive"), "count": Integer(3)})
   Record({"count": Integer(7), "status": String("active")})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_functions_search`
🚀 ekoDB Rust Search Scripts Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Script saved
📊 Found 5 documents
   1. Getting Started with ekoDB (Database)
   2. Vector Databases Explained (Database)
   3. Natural Language Processing (AI)
   4. Introduction to Machine Learning (AI)
   5. Database Design Principles (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   Record({"category": String("AI"), "count": Integer(2)})
   Record({"category": String("Database"), "count": Integer(3)})
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
✓ Created session: 8IOhGsKhzWZ5pJ2jOMsad4ch7Ygibm8LTPidQoC0fR-AY-sP1NcEYJrMBdJbYCGjUMVAN5oUeCwVmBkz57l7fQ

=== Sending Chat Message ===
Message ID: mTfIQ5Yh0rapq8dG-KoAnRLtMoQlPQJZSn-eB4hW4T35GLTK7rjLTJ7DToDb6fjU3mt3WT24nfrfDcFZd5zhNA

=== AI Response ===
Response 1: ekoDB is a high-performance database known for its intelligent caching, real-time capabilities, and AI integration. It features an AI Chat Integration, which allows you to query your database using natural language and get AI-powered responses with relevant context. Additionally, ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval, making it a versatile tool for handling different types of data queries.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["category", "content", "title"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("SjmTFr3kzDvh0kVQmOJ8qj8BmQAGHpWKSecS3FccxPXNHafJf0w6Md138uI-zZb3vq8LKkqHJRQpHM4_l2tg4Q"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "title"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("9DDDHHPK02h9QGcN4n0clfYVwDeNjue05brecTZgwVDzGxWr-gilE6VL8FThT2r2xTNhqfF20wXsqkjdFwegaw"), "title": String("Introduction to ekoDB")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "category", "title"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("PhoKi6b5_Rz0UF30n4jCbGaCMDfcGB88ktMqMnq334QSsgKiYBKbUlf3kyv1ZqjFqlPjyAeXuQVYPX4lslOelQ"), "title": String("Search Features")}

Execution Time: 2480ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 2sMIWTi9x1Hd9TS_hvcseq-8-mnLuh14Y781dmuqO6UXxI99Vzrb7XWBNbFDV26QLcyOEeHEC7UTqiOrALESaQ

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, the product available is a high-performance database product named "ekoDB". It is priced at $99.

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
✓ Created second session: ax2TZvpXSW6UDWu0Xd8rZaoDJrgv69xu9g5fS83AE1dSontfpcLCH6P3BE-CLYXeCok1uzWoZpK28TjUifVgkg
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: CPPoqpxkc85Q7aA0TR4rdrFfCaB5JLrF473_duRXGUWp5tSYwkpxBvajc-LBkJIMN7bUFbDDrSwMz0U-WJC1xg
=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is "ekoDB," a high-performance database product with AI capabilities. The price for this product is $99.

✓ Message 2 sent
  Response: The price for the "ekoDB" product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: hEPQXNGz09SmTuJulZ03V3WzvdcOy31-Gf-356kvK-krjgGyCzEJ50OPBBbrlWnmsA7JeleLsgGJzh3jF3o47w
  Parent: CPPoqpxkc85Q7aA0TR4rdrFfCaB5JLrF473_duRXGUWp5tSYwkpxBvajc-LBkJIMN7bUFbDDrSwMz0U-WJC1xg

=== Listing Sessions ===
✓ Found 20 sessions
  Session 1: hEPQXNGz09SmTuJulZ03V3WzvdcOy31-Gf-356kvK-krjgGyCzEJ50OPBBbrlWnmsA7JeleLsgGJzh3jF3o47w (Untitled)
  Session 2: CPPoqpxkc85Q7aA0TR4rdrFfCaB5JLrF473_duRXGUWp5tSYwkpxBvajc-LBkJIMN7bUFbDDrSwMz0U-WJC1xg (Untitled)
  Session 3: ax2TZvpXSW6UDWu0Xd8rZaoDJrgv69xu9g5fS83AE1dSontfpcLCH6P3BE-CLYXeCok1uzWoZpK28TjUifVgkg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: hEPQXNGz09SmTuJulZ03V3WzvdcOy31-Gf-356kvK-krjgGyCzEJ50OPBBbrlWnmsA7JeleLsgGJzh3jF3o47w

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_chat_models`
✓ Client created

=== Get All Chat Models ===
Available chat models by provider:

OpenAI models (121):
  - gpt-4-0613
  - gpt-4
  - gpt-3.5-turbo
  - gpt-5.4
  - gpt-5.3-chat-latest
  - gpt-5.4-2026-03-05
  - gpt-5.4-pro
  - gpt-5.4-pro-2026-03-05
  - davinci-002
  - babbage-002
  - gpt-3.5-turbo-instruct
  - gpt-3.5-turbo-instruct-0914
  - dall-e-3
  - dall-e-2
  - gpt-4-1106-preview
  - gpt-3.5-turbo-1106
  - tts-1-hd
  - tts-1-1106
  - tts-1-hd-1106
  - text-embedding-3-small
  - text-embedding-3-large
  - gpt-4-0125-preview
  - gpt-4-turbo-preview
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

openai models (121):
  - gpt-4-0613
  - gpt-4
  - gpt-3.5-turbo
  - gpt-5.4
  - gpt-5.3-chat-latest
  ... and 116 more

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_user_functions`
✓ Client created

=== Create User Function ===
Created user function with ID: Rz__UvNSPBZMRA_gHrVdwbmo9v8d1RWzIbOcxckYYiKLH9e-xI55PnpNFbvvdb6GtT2w4KsaRPfwIX929aDyMw

=== Get User Function ===
Retrieved: get_active_users_rs - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 89 user functions:
  - swr_user: SWR pattern for user data (KV-based)
  - get_verified_user: Get verified and validated user
  - fetch_user: Fetch user by code
  - get_verified_user: Get verified and validated user
  - validate_user: Check if user exists
  - swr_user: SWR pattern for user data (KV-based)
  - validate_user: Check if user exists
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_product_with_reviews: Fetch Product with Reviews (Multi-API)
  - swr_user: SWR pattern for user data (KV-based)
  - get_active_users_paginated: Get Active Users (Paginated)
  - fetch_slim_user: Validate and slim down user
  - fetch_product_with_reviews: Fetch Product with Reviews (Multi-API)
  - swr_user: SWR pattern for user data (KV-based)
  - swr_user: SWR pattern for user data (KV-based)
  - fetch_slim_user: Validate and slim down user
  - cache_api_call: Cache External API Call
  - get_active_users: Get Active Users (Updated)
  - get_user_wrapper: Wrapper that calls fetch_user
  - get_user_wrapper: Wrapper that calls fetch_user
  - fetch_user: Fetch user by code
  - get_user_wrapper: Wrapper that calls fetch_user
  - cache_api_call: Cache External API Call
  - validate_user: Check if user exists
  - swr_user: SWR pattern for user data (KV-based)
  - fetch_slim_user: Validate and slim down user
  - fetch_product_enriched: Fetch Product with Enrichment
  - validate_user: Check if user exists
  - validate_user: Check if user exists
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_verified_user: Get verified and validated user
  - fetch_slim_user: Validate and slim down user
  - fetch_slim_user: Validate and slim down user
  - fetch_api_user: Fetch User with Cache
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_user: Fetch user by code
  - fetch_user: Fetch user by code
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_slim_user: Validate and slim down user
  - swr_user: SWR pattern for user data (KV-based)
  - cache_api_call_js: Cache External API Call
  - swr_user: SWR pattern for user data (KV-based)
  - get_active_users_rs: Get Active Users
  - validate_user: Check if user exists
  - fetch_user: Fetch user by code
  - get_active_users_updated: Get Active Users (Updated)
  - fetch_user: Fetch user by code
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_user: Fetch user by code
  - fetch_product_enriched: Fetch Product with Enrichment
  - fetch_slim_user: Validate and slim down user
  - get_verified_user: Get verified and validated user
  - get_high_scoring_active_users: Get High Scoring Active Users
  - fetch_product_enriched: Fetch Product with Enrichment
  - fetch_slim_user: Validate and slim down user
  - validate_user: Check if user exists
  - validate_user: Check if user exists
  - swr_user: SWR pattern for user data (KV-based)
  - fetch_github_user: Fetch GitHub User with Cache
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_user_wrapper: Wrapper that calls fetch_user
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_github_user: Fetch GitHub User with Cache
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_user: Fetch user by code
  - fetch_slim_user: Validate and slim down user
  - fetch_github_user: Fetch GitHub User with Cache
  - fetch_github_user: Fetch GitHub User with Cache
  - fetch_product_enriched: Fetch Product with Enrichment
  - validate_user: Check if user exists
  - swr_user: SWR pattern for user data (KV-based)
  - get_verified_user: Get verified and validated user
  - validate_user: Check if user exists
  - get_user_wrapper: Wrapper that calls fetch_user
  - fetch_user: Fetch user by code
  - fetch_user: Fetch user by code
  - get_user_wrapper: Wrapper that calls fetch_user
  - cache_api_call_py: Cache External API Call
  - fetch_slim_user: Validate and slim down user
  - fetch_user: Fetch user by code
  - fetch_slim_user: Validate and slim down user
  - swr_user: SWR pattern for user data (KV-based)
  - fetch_api_user: Fetch User with Cache
  - get_verified_user: Get verified and validated user
  - validate_user: Check if user exists
  - get_users_by_status: Get Users By Status

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_convenience_methods`
=== ekoDB Convenience Methods Example ===

=== Record Builder Pattern ===
✓ Created record with fluent builder: Record({"id": String("Unn8ZpPoZd51JvhWKEy6MzNkZy90IOr9G0UD0IJOvLs78e880i3kX_AxWZkPGTJgB44zDxyVtCp9Bo6pNHe3oA")})

=== Upsert Operation ===
✓ First upsert (insert): Record({"id": String("3cgsEiTL3pqPg2ZxmShRC26nYyLVUWrHLMMUbd72Pd77gd4eMGVZ22R8GejkYRG3xhDgKOyllfPkP_K9GLLcsw")})
✓ Second upsert (update): Record({"id": String("nWsVmgzwZZ2yQPoOG4e21BSIjrqqBDGuBz79kf3DfZD3i50ZxeoOgoJ1yKZBYLHpEOI_4sD8blshkP5jVbHlFg")})

=== Find One Operation ===
✓ Found user by email: Record({"name": Object({"type": String("String"), "value": String("Alice Johnson")}), "age": Object({"type": String("Integer"), "value": Integer(28)}), "email": Object({"type": String("String"), "value": String("alice@example.com")}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "id": String("Unn8ZpPoZd51JvhWKEy6MzNkZy90IOr9G0UD0IJOvLs78e880i3kX_AxWZkPGTJgB44zDxyVtCp9Bo6pNHe3oA")})
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
   Inserted with ripple: Record({"id": String("I4jjdsssq8zH3K8fRsPjRAvtuFSu5K9siXOrSTuNKWfLUBp0c-FMmqu2fqUWSUXqvPheO0B5Y12NzZaMWshLPw")})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: Record({"id": String("NisYBFgn36xS04mzBNIugyETNM5ndU6FuRQM_bCAOAEuxvMBVNPGm0UMG9P16nbWtZBWjAnunbifGsso_8rNeQ")})

3. Update with bypass_ripple:
   Updated with bypass_ripple: Record({"name": Object({"type": String("String"), "value": String("Product 1")}), "id": String("I4jjdsssq8zH3K8fRsPjRAvtuFSu5K9siXOrSTuNKWfLUBp0c-FMmqu2fqUWSUXqvPheO0B5Y12NzZaMWshLPw"), "price": Object({"type": String("Integer"), "value": Integer(150)})})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: Record({"price": Object({"value": Integer(500), "type": String("Integer")}), "id": String("custom-id"), "name": Object({"type": String("String"), "value": String("Upsert Product")})})

✅ All bypass_ripple operations completed successfully!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/projection_example`
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ["name", "email", "id"]

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["name", "avatar_url", "status", "user_role", "email", "bio", "age", "id", "created_at"]

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)

Example 4: Query inactive users with profile fields
  Found 1 inactive users

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ["bio", "secret_token", "status", "age", "id", "password", "api_key", "created_at", "avatar_url", "user_role", "name", "email"]
  Projected query:
    - 3 fields per record
    - Fields: ["name", "id", "email"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
✅ [32mRust client examples complete![0m
