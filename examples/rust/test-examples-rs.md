make test-examples-rust
🧪 Running Rust examples (direct HTTP/WebSocket)...
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("MfOwYMqYT-r4QWIza2MvYDkoJqOG8FL-1y2SAbpHrMb7Az7ZccQ8oMCWDRpLuI4Uyxma5XdQWRYfC_WFcbAx5w")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("MfOwYMqYT-r4QWIza2MvYDkoJqOG8FL-1y2SAbpHrMb7Az7ZccQ8oMCWDRpLuI4Uyxma5XdQWRYfC_WFcbAx5w"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("MfOwYMqYT-r4QWIza2MvYDkoJqOG8FL-1y2SAbpHrMb7Az7ZccQ8oMCWDRpLuI4Uyxma5XdQWRYfC_WFcbAx5w"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("MfOwYMqYT-r4QWIza2MvYDkoJqOG8FL-1y2SAbpHrMb7Az7ZccQ8oMCWDRpLuI4Uyxma5XdQWRYfC_WFcbAx5w"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: wtEADxlghxHGWlSJ2W-4p6D_kbKVf3ew-azD2LBauCm5pzZw8hBCXSulmxyuvuqzNQ6iqeT17RNzSdmdz_Dhrg

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1793589566463221000",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "wtEADxlghxHGWlSJ2W-4p6D_kbKVf3ew-azD2LBauCm5pzZw8hBCXSulmxyuvuqzNQ6iqeT17RNzSdmdz_Dhrg",
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
✓ Deleted test collection

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/kv_operations`
✓ Authentication successful

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: Object {"type": String("Object"), "value": Object {"userId": Number(123), "username": String("john_doe")}}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
kv_operations:direct:rs:cache:product:1: Object {"type": String("Object"), "value": Object {"name": String("Product 1"), "price": Number(29.99)}}
kv_operations:direct:rs:cache:product:2: Object {"type": String("Object"), "value": Object {"name": String("Product 2"), "price": Number(39.989999999999995)}}
kv_operations:direct:rs:cache:product:3: Object {"type": String("Object"), "value": Object {"name": String("Product 3"), "price": Number(49.989999999999995)}}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/collection_management`
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: "8Oqft103ulUXXYZ5eTZPY-Jp1qHWrl6IfGtUwrlsfIRHCXJ2xOs_QvnZWJYmPCNptXr1cIwsfuxpHxtY-qtTxQ"

=== List Collections ===
Total collections: 2
Sample collections: ["audit__ek0_testing", "demo_collection"]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/document_ttl`
=== ekoDB Document TTL Example (Rust) ===
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: y5lW7RJVTEsDc_XYKRBcx2gEPnhc-SQip5NDI6JzimYQQ8oZZxHTU0iwvMCTbBVYT28duX1oscdZWNiNAGHu4Q

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: OPw74xSxY2z-g59dBA8HfOHMPbuwFzdbHohAnJg7-IbYKnm5qz4MYN5W2CBhqvaGrdXRY7IYJLP2g726YD78tg

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: 6j9m5Mt7GMsWi76K8iJi3UzCe5UZGq2YnmfB7ycUMjR1-tH2-zN5vvBZZZ9zBUrgLyLarDqWOEE8Kssxp4dyiA

=== Query Documents ===
✓ Found 3 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/websocket_ttl`
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: String("GVABgLZPIhpklOwnfehz7Rz-8RGqiUAKrhaiT2Br0dWFhDfwzcRkuZvnCMo2vjd9xSl3doSdByAoMVV0smWa2w")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record via WebSocket

Record 1:
  id: "GVABgLZPIhpklOwnfehz7Rz-8RGqiUAKrhaiT2Br0dWFhDfwzcRkuZvnCMo2vjd9xSl3doSdByAoMVV0smWa2w"
  name: {"type":"String","value":"WebSocket TTL Test"}
  ttl: "2026-11-02T04:26:06.847381Z"
  value: {"type":"Integer","value":42}

✓ WebSocket TTL example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.36s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Functions Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: azCC4HCi3EVzQQ4b3UOUS3a-vcqYf-SdT5dIhb6OBecKoCz2scFPWpAlP8uE3Veo9pYbBrVLvegbVka2fSqN-g
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: dsp6pMqcceBoDPVe8I3rcqBAtd8fD4Fc4VbBqHZdHmti3f7HY8Z_7ks4RtPYgtfaafCtl0RB3GPEQ0IQeku1Ig
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Function saved: DUC-swEn-6st60zuT9YqEl7Qmwm-6ekvFzXqCUgHQg3WFjlOEOEl6_7OCBlbCOKqAsKjO21qI6acXjsFrTMUPA
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: 1CtjTXbmUPmwZUX2gRA5u8bvHT54lMeKpqi0dLaIe0dL-dqkahYMM07xAHfsVSnkj6DG9YHiaAY595SGqfXrNw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":{"type":"Float","value":50.0},"count":{"type":"Integer","value":5},"max_score":{"type":"Integer","value":90},"status":{"type":"String","value":"inactive"}}
   {"avg_score":{"type":"Float","value":60.0},"count":{"type":"Integer","value":5},"max_score":{"type":"Integer","value":100},"status":{"type":"String","value":"active"}}

📝 Example 4: Function Management

📋 Total scripts: 4
🔍 Retrieved script: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/examples/transactions`
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: Qer5p97RnDknPT1gUwwO__sKgirImdx0pbLSymMBG11toUwcwKl3wLNbi0y8CB79kB05d_t2HZ3HcBVPiUNvAw
Created Bob: $500 - ID: QX4FkabXRDPvTjxGaT3LUvKuuva30oLqxOSgJpNTgSpNy165uyTC9dhc7Q3fm6RcUPiJPzjx1IsUmv3nyIIFIw

=== Example 1: Begin Transaction ===
Transaction ID: 89922da6-2486-493d-ad38-c292e4b0a8e7

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
New transaction: 75a8b8e3-1231-456b-b32f-c91b4573e115
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"type":"Integer","value":700}

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✅ Rust direct examples complete!
🛠️  Building client library...
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.35s
✅ Client build complete!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/examples/client_admin`
Admin permissions read successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_advanced_crud`
=== ekoDB Advanced CRUD Example (Rust) ===

--- Inserting base document ---
Inserted: Widget Counter (id: H8h6ek5CPa6_ZEnbtnUF7KxCu2opyvZcmJgGy8jBE8vsODsb1S991awhWRwI5chiFgmJD_gZ4VOPOk_BM2Bvlg)

--- update_with_action: increment views by 10 ---
views after increment: Some(Object({"type": String("Integer"), "value": Integer(110)}))

--- update_with_action: decrement views by 3 ---
views after decrement: Some(Object({"value": Integer(107), "type": String("Integer")}))

--- update_with_action: multiply score by 2 ---
score after multiply: Some(Object({"value": Float(9.0), "type": String("Float")}))

--- update_with_action: push 'ekodb' to tags ---
tags after push: Some(Object({"type": String("Array"), "value": Array([String("rust"), String("database"), String("ekodb")])}))

--- update_with_action: append '-suffix' to label ---
label after append: Some(Object({"type": String("String"), "value": String("prefix-suffix")}))

--- update_with_action: pop last element from tags ---
tags after pop: Some(Object({"value": Array([String("rust"), String("database")]), "type": String("Array")}))

--- update_with_action: remove 'rust' from tags ---
tags after remove: Some(Object({"type": String("Array"), "value": Array([String("database")])}))

--- update_with_action_sequence: increment views + push tag + append label ---
views after sequence: Some(Object({"type": String("Integer"), "value": Integer(157)}))
tags after sequence: Some(Object({"value": Array([String("database"), String("batch")]), "type": String("Array")}))
label after sequence: Some(Object({"value": String("prefix-suffix-v2"), "type": String("String")}))

--- Cleanup ---
Deleted collection

=== All advanced CRUD operations completed ===
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.38s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.04s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 2Gy-V-k7zdj32ru-5rNequ65BfPakeCyi0V8cV79w1mzei8bUW0dYFKZgNr6UD_gM9nCeefYXhZexH61Wjdwwg

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is **ekoDB**, a high-performance database product, priced at **99**. The currency isn’t specified.

✓ Second message sent
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: ekoDB costs **99**. The currency isn’t specified.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: WRjW4WIs9wL8FTv6Z2cU3f91mMi1HfrjEBVyj-SnusvB7QBsTO0CmfZv3kpDO31sZHAlDRfFa1LTLD9r6B0Sbg
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: WRjW4WIs9wL8FTv6Z2cU3f91mMi1HfrjEBVyj-SnusvB7QBsTO0CmfZv3kpDO31sZHAlDRfFa1LTLD9r6B0Sbg
✓ Deleted chat session: 2Gy-V-k7zdj32ru-5rNequ65BfPakeCyi0V8cV79w1mzei8bUW0dYFKZgNr6UD_gM9nCeefYXhZexH61Wjdwwg
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.70s
     Running `target/debug/examples/client_chat_basic`
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: HV_ITPAyOG3wtbJg2i4aOX5VnLa0oY-uyEP_EYqcarR3yvwbE8DT3SJEX1g2GGgWHF6OCl6aRSMnHd0Syv5KUQ

=== Sending Chat Message ===
Message ID: W86wZJZIhZJ1itpubqXwQTeDRIoGUWotThXFYVVrHG59xrdWSKYMljVyCromj4yk9HAB0jmln8-Qf1XEl1vM1w

=== AI Response ===
Response 1: ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration.

Its features include:
- **AI chat:** Ask questions about your database in natural language and get AI-powered responses with relevant context.
- **Search:** Full-text, vector, and hybrid search, with automatic context retrieval.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["title", "category", "content"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("ykg-gxeX3Q4A0NqSW0tSz9LpGZJycOCnZNvkGXTsWGkBQVdHlmIWyOT9N-1vxmS8onDwqpaj2niC4s-H5dXjSQ"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "category", "content"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("aF2Ky2kgcZsbkSPKK2fL9padSbzG9m_zUeyYXAun5e5fi2IgmsSejxWR7oEPCW5e0PIPnffE7Aar9F_q_Y-eoA"), "title": String("Search Features")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "content"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("h4R-nNwK5nxIdTZBgCOh5R6_BNWH9M733fqy0k83CN33_vVVX5XYgmjnB2Ri8AcPwWpMrYPnvdLECKUgjgJMzQ"), "title": String("Introduction to ekoDB")}

Execution Time: 4977ms

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/examples/client_chat_message_stream`
=== ekoDB Chat Message Stream (SSE) Example (Rust) ===

Created session: 6ZV4luO9rk2O8LLbC6vYE-VhPR1GfWJCxtAbqKT71UCnZj2O29zZgQL0P6PK0NL4uLqZplaPGaJmiiC7NE0evA

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"gpt-6-luna","provider":"openai"}**ekoDB** is an open-source database project designed for real-time applications. It aims to make data available with low latency across applications or services. Its exact features depend on the version or project you mean; share a link if you’d like details about a specific ekoDB.

--- Stream complete ---
Message ID: ay13VubYHQ2mfva2HtML3PvoDNGvMRAAxXw_jZx9DKwL3hknmZY4ZsNWr9AUDwKUvokxhuZsqR2UuCwn4vEc_A
Execution time: 6904ms
Context window: 128000 tokens

✓ Chat message stream example completed
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/examples/client_chat_models`
✓ Client created

=== Get All Chat Models ===
Available chat models by provider:

OpenAI models (136):
  - gpt-3.5-turbo-0125
  - gpt-5.4-2026-03-05
  - gpt-5-nano
  - gpt-5.5-pro
  - gpt-audio-2025-08-28
  - gpt-4o-mini-transcribe-2025-03-20
  - gpt-4-turbo-2024-04-09
  - gpt-realtime-2.1-mini
  - gpt-realtime-2025-08-28
  - gpt-4
  - tts-1-hd
  - gpt-realtime-1.5
  - gpt-realtime
  - gpt-5.2-pro
  - gpt-5-chat-latest
  - gpt-5.4-mini-2026-03-17
  - babbage-002
  - gpt-4o-2024-05-13
  - text-embedding-3-large
  - tts-1
  - omni-moderation-2024-09-26
  - gpt-live-transcribe
  - gpt-5.6-luna
  - gpt-5-2025-08-07
  - gpt-5-mini
  - gpt-5-mini-2025-08-07
  - omni-moderation-latest
  - o3-mini-2025-01-31
  - gpt-audio-1.5
  - gpt-image-1.5
  - tts-1-1106
  - gpt-5.5-pro-2026-04-23
  - gpt-audio-mini-2025-10-06
  - gpt-5.2-pro-2025-12-11
  - o4-mini-2025-04-16
  - gpt-4o-2024-11-20
  - gpt-5.1
  - whisper-1
  - gpt-5-pro
  - gpt-realtime-whisper
  - gpt-realtime-2
  - gpt-5-search-api
  - gpt-5.5-2026-04-23
  - gpt-4.1-nano
  - gpt-image-2
  - gpt-3.5-turbo-16k
  - text-embedding-ada-002
  - gpt-realtime-2.1
  - gpt-4.1
  - gpt-5.5
  - chat-latest
  - gpt-realtime-translate
  - gpt-5.4-pro-2026-03-05
  - gpt-4.1-mini
  - gpt-4o-mini
  - gpt-4o-mini-transcribe-2025-12-15
  - gpt-realtime-mini
  - gpt-image-1-mini
  - gpt-5.6-terra
  - gpt-3.5-turbo-instruct-0914
  - gpt-4-turbo
  - gpt-4o-2024-08-06
  - o3
  - gpt-4o-mini-search-preview-2025-03-11
  - o1-2024-12-17
  - gpt-5.1-chat-latest
  - gpt-5-pro-2025-10-06
  - gpt-5.6-sol
  - gpt-4o-transcribe-diarize
  - gpt-5-codex
  - gpt-4-0613
  - gpt-5.1-2025-11-13
  - gpt-4o-search-preview
  - gpt-4o-mini-tts-2025-12-15
  - gpt-image-1
  - gpt-audio
  - gpt-5.3-codex
  - gpt-5-search-api-2025-10-14
  - gpt-5.4-pro
  - gpt-4o-mini-tts
  - gpt-realtime-mini-2025-12-15
  - gpt-5.1-codex-max
  - sora-2
  - gpt-4o
  - gpt-4o-mini-2024-07-18
  - gpt-4.1-2025-04-14
  - gpt-5
  - gpt-4o-mini-transcribe
  - gpt-image-2-2026-04-21
  - gpt-5.2-chat-latest
  - sora-2-pro
  - gpt-3.5-turbo-instruct
  - gpt-transcribe
  - gpt-4.1-nano-2025-04-14
  - gpt-3.5-turbo-1106
  - gpt-4o-mini-tts-2025-03-20
  - gpt-5.2
  - gpt-5.4-nano-2026-03-17
  - o1
  - tts-1-hd-1106
  - gpt-5.2-codex
  - gpt-5.4-mini
  - o1-pro
  - gpt-5.1-codex
  - o4-mini-deep-research
  - gpt-5.4
  - gpt-4o-search-preview-2025-03-11
  - text-embedding-3-small
  - gpt-5-nano-2025-08-07
  - gpt-4o-transcribe
  - o4-mini
  - gpt-5.1-codex-mini
  - gpt-6-astra
  - gpt-5.3-chat-latest
  - o3-2025-04-16
  - gpt-5.2-2025-12-11
  - chatgpt-image-latest
  - gpt-3.5-turbo
  - gpt-image-2.5-sunburst-2026-09-08
  - gpt-image-2.5-flare
  - davinci-002
  - gpt-5.4-nano
  - o3-mini
  - o4-mini-deep-research-2025-06-26
  - gpt-audio-mini-2025-12-15
  - o1-pro-2025-03-19
  - gpt-live-1
  - gpt-image-2.5-sunburst
  - gpt-image-2.5-flare-2026-09-08
  - gpt-6.1-terra
  - gpt-audio-mini
  - gpt-4.1-mini-2025-04-14
  - gpt-6-luna
  - gpt-4o-mini-search-preview
  - gpt-6-sol
  - gpt-6.1-sol

Anthropic models (14):
  - claude-haiku-5-5
  - claude-sonnet-5-5
  - claude-opus-5-5
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

Gemini models (60):
  - gemini-2.5-flash
  - gemini-2.5-pro
  - gemini-2.5-flash-preview-tts
  - gemini-2.5-pro-preview-tts
  - gemma-4-26b-a4b-it
  - gemma-4-31b-it
  - gemini-flash-latest
  - gemini-flash-lite-latest
  - gemini-pro-latest
  - gemini-2.5-flash-lite
  - gemini-2.5-flash-image
  - gemini-3-flash-preview
  - gemini-3.1-pro-preview
  - gemini-3.1-pro-preview-customtools
  - gemini-3.1-flash-lite-preview
  - gemini-3.1-flash-lite
  - gemini-3-pro-image-preview
  - gemini-3-pro-image
  - nano-banana-pro-preview
  - gemini-3.1-flash-image-preview
  - gemini-3.1-flash-image
  - gemini-3.1-flash-lite-image
  - gemini-nano-banana-2.1
  - gemini-3.5-flash
  - gemini-3.5-flash-lite
  - gemini-omni-flash-preview
  - gemini-omni-1.1-flash
  - gemini-3.5-transcribe
  - gemini-3.6-flash
  - gemini-3.7-flash
  - gemini-3.8-flash
  - gemini-4-argon
  - lyria-3-clip-preview
  - lyria-3-pro-preview
  - lyria-3.5
  - gemini-3.1-flash-tts-preview
  - gemini-3.8-flash-tts
  - gemini-3.8-flash-lite-tts
  - gemini-robotics-er-2-preview
  - gemini-2.5-computer-use-preview-10-2025
  - antigravity-preview-05-2026
  - antigravity-preview-09-2026
  - antigravity-preview-latest
  - deep-research-max-preview-04-2026
  - deep-research-preview-04-2026
  - deep-research-pro-preview-12-2025
  - gemini-embedding-001
  - gemini-embedding-2-preview
  - gemini-embedding-2
  - aqa
  - gemini-3.5-transcribe-live
  - gemini-2.5-flash-native-audio-latest
  - gemini-2.5-flash-native-audio-preview-09-2025
  - gemini-2.5-flash-native-audio-preview-12-2025
  - gemini-3.1-flash-live-preview
  - gemini-3.8-live
  - gemini-3.8-live-extended-thinking
  - gemini-robotics-er-2-streaming-preview
  - gemini-3.5-live-translate-preview
  - lyria-realtime-exp

Provider status:
  anthropic: ok 14 models
  gemini: ok 60 models
  openai: ok 136 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get Models for Specific Provider ===

openai models (136):
  - gpt-3.5-turbo-0125
  - gpt-5.4-2026-03-05
  - gpt-5-nano
  - gpt-5.5-pro
  - gpt-audio-2025-08-28
  ... and 131 more

anthropic models (14):
  - claude-haiku-5-5
  - claude-sonnet-5-5
  - claude-opus-5-5
  - claude-fable-5-1
  - claude-opus-5
  ... and 9 more
GetChatModel(perplexity) error: Record not found

✓ Chat Models API example complete
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.93s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: GTNXV2W_v3Qb6uyrITUvkV3UmIXdOGt7kKeWZiZJxJAkTAn5st2olV4c8v0SGterXatcONBeu0S1Qd12PQeR3g
=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is **ekoDB**, a high-performance database with AI capabilities. It’s priced at **$99**.

✓ Message 2 sent
  Response: ekoDB is priced at **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: CUSJXSv9JzqdaiEpAXK2G-Vh4Tr-5PqlCLajQKbmdwdaNgpmAx0CB_kset40414T96NreWn3FvgJIaHyId6wUQ
  Parent: GTNXV2W_v3Qb6uyrITUvkV3UmIXdOGt7kKeWZiZJxJAkTAn5st2olV4c8v0SGterXatcONBeu0S1Qd12PQeR3g

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: CUSJXSv9JzqdaiEpAXK2G-Vh4Tr-5PqlCLajQKbmdwdaNgpmAx0CB_kset40414T96NreWn3FvgJIaHyId6wUQ (Untitled)
  Session 2: GTNXV2W_v3Qb6uyrITUvkV3UmIXdOGt7kKeWZiZJxJAkTAn5st2olV4c8v0SGterXatcONBeu0S1Qd12PQeR3g (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: CUSJXSv9JzqdaiEpAXK2G-Vh4Tr-5PqlCLajQKbmdwdaNgpmAx0CB_kset40414T96NreWn3FvgJIaHyId6wUQ

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_collection_management`
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "oGNg9xBx4pMGMaKRsZxPfE6nHtdZ-rwQGxcbaqjlMZKkopd4DAanGGwEa2kbtOqMKKnDVZGh5q09uGc4o4hOxA"

=== List Collections ===
Total collections: 7
Sample collections: ["audit__ek0_testing", "functions__ek0_testing", "chat_turns__ek0_testing", "client_collection_management_rust", "agent_function_versions__ek0_testing"]

=== Count Documents ===
Document count: 1

=== Check Collection Exists ===
Collection exists: true

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_collection_utils`
✓ Client created

=== Check Collection Exists (Before Creation) ===
Collection 'collection_utils_test_rust' exists: false

=== Creating Test Documents ===
Created 5 test documents

=== Check Collection Exists (After Creation) ===
Collection 'collection_utils_test_rust' exists: true

=== Count Documents ===
Document count in 'collection_utils_test_rust': 5

=== List Collections ===
Total collections: 7
  - audit__ek0_testing
  - collection_utils_test_rust <-- our test
  - functions__ek0_testing
  - chat_turns__ek0_testing
  - agent_function_versions__ek0_testing
  - chat_messages__ek0_testing
  - chat_configurations__ek0_testing

=== Check Non-Existent Collection ===
Collection 'nonexistent_collection_xyz' exists: false

=== Cleanup ===
Deleted collection 'collection_utils_test_rust'

✓ Collection Utilities example complete
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/examples/client_convenience_methods`
=== ekoDB Convenience Methods Example ===

=== Record Builder Pattern ===
✓ Created record with fluent builder: Record({"id": String("jR4xUPPjJ6GkTObDQYV7iGJh0vDsd8QsbmGNKAye_QLmNtWO4kC12at2Pd4eivs5TD7-4OJeVBgUTWQEtfFl0A")})

=== Upsert Operation ===
✓ First upsert (insert): Record({"id": String("bob-user-id")})
✓ Second upsert (update): Record({"age": Object({"type": String("Integer"), "value": Integer(36)}), "email": Object({"type": String("String"), "value": String("bob.smith@newdomain.com")}), "id": String("bob-user-id"), "name": Object({"type": String("String"), "value": String("Bob Smith")})})

=== Find One Operation ===
✓ Found user by email: Record({"id": String("jR4xUPPjJ6GkTObDQYV7iGJh0vDsd8QsbmGNKAye_QLmNtWO4kC12at2Pd4eivs5TD7-4OJeVBgUTWQEtfFl0A"), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "email": Object({"value": String("alice@example.com"), "type": String("String")}), "age": Object({"value": Integer(28), "type": String("Integer")}), "name": Object({"value": String("Alice Johnson"), "type": String("String")})})
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_crypto_stages`
✓ Client created
✓ crypto_demo_hmac_rs saved
✓ crypto_demo_aes_rs saved
✓ crypto_demo_uuid_rs saved
✓ crypto_demo_totp_rs saved
✓ crypto_demo_encoding_rs saved

All crypto-stage demos defined. Invoke any of them with:
  POST /api/functions/crypto_demo_hmac_rs { "payload": "hi" }
  POST /api/functions/crypto_demo_aes_rs { "plaintext": "secret" }
  POST /api/functions/crypto_demo_uuid_rs
  POST /api/functions/crypto_demo_totp_rs
  POST /api/functions/crypto_demo_encoding_rs { "title": "Héllo World" }

✓ Cleaned up demo functions
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.37s
     Running `target/debug/examples/client_distinct_values`
=== ekoDB Distinct Values Example ===

=== Inserting Sample Products ===
Inserted 8 products

=== Distinct Categories (all products) ===
Found 3 distinct categories:
  - {"type":"String","value":"books"}
  - {"type":"String","value":"clothing"}
  - {"type":"String","value":"electronics"}

=== Distinct Statuses (all products) ===
Found 3 distinct statuses:
  - {"type":"String","value":"active"}
  - {"type":"String","value":"archived"}
  - {"type":"String","value":"discontinued"}

=== Distinct Statuses in Electronics ===
Found 2 distinct statuses for electronics:
  - {"type":"String","value":"active"}
  - {"type":"String","value":"discontinued"}

Cleanup done.
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.35s
     Running `target/debug/examples/client_document_ttl`
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "LlrBRngti1HU4ItD17aNggDqTyNQvNkELkJFxuDz5mVPVSRZKuodDCgtbkIoZI6IeLRjJ1F48VKItRj-78nARw"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("cJlrcxFU3fDNp1D7MPaYQhT8yOHo4M6a5LykHjuLiumCzaM5V2O6-_WbPHOyxDQv885IlgAQmSkZiLSX6OjnjA"))

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.49s
     Running `target/debug/examples/client_edge_cache`
=== ekoDB as Edge Cache - Simple Example ===

Setting up edge cache collection...
✓ Cache entry created

Creating edge cache lookup script...
✓ Edge cache script created: q50fCki41PUA9o78is4rPTCUDLq9YrrFiFeXrp0XsblHTfIkoM4D3KxALWPMC5_hTpAV3D5AmuncasiuJKyXpw

Call 1: Cache lookup
Response time: 4ms
Found 1 cached entries

Call 2: Cache lookup (connection warm)
Response time: 3ms
Found 1 cached entries

=== The Magic ===
- Your DATABASE is your EDGE
- No Redis needed
- No CDN needed
- No cache invalidation logic needed (TTL handles it)
- With ripples: All nodes auto-sync cache
- One service: Database + Cache + Edge Functions

🧹 Cleaning up...
✓ Cleanup complete

✓ Example complete!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.54s
     Running `target/debug/examples/client_function_composition`
=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

✅ Saved reusable function: fetch_user_rs
✅ Saved composed function: get_user_wrapper_rs (calls fetch_user_rs + projects fields)

📊 Result from composed function:
   Records: 1
   Name: User 1
   Department: engineering

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user_rs (uses KV)
✅ Saved SWR function using composition: swr_user_rs

First call (cache miss - will fetch from API):
   ⏱️  Duration: 89.493459ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "company": {
        "name": "Romaguera-Crona",
        "catchPhrase": "Multi-layered client-server neural-net",
        "bs": "harness real-time e-markets"
      }...

Second call (cache hit - from cache):
   ⏱️  Duration: 18.453333ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "company": {
        "name": "Romaguera-Crona",
        "catchPhrase": "Multi-layered client-server neural-net",
        "bs": "harness real-t...
   🚀 Cache speedup: 4.8x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user_rs
✅ Level 2 function: fetch_slim_user_rs (calls validate_user_rs)
✅ Level 3 function: get_verified_user_rs (calls fetch_slim_user_rs)

📊 Result from 3-level nested composition:
   Records: 1
   Name: User 1
   Department: engineering

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_enriched_user: Used in 50 workflows
   - get_engineering_user: Specific workflow


✅ Cleanup complete
✅ All composition examples completed!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/examples/client_function_contract`
client_function_contract: ok
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_functions`
=== ekoDB Rust Client - Functions Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: H7-zACpd2UebwkCKk1YX2wqFlPt4CcOZ_9NwiLqVSL-AZLKj6DsGVa3OhhTiEjPlIecA3DGFZRKPBAiumRlHRQ
📊 Found 5 active users

📝 Example 2: Parameterized Function

✅ Function saved: dxiDm70fAB3CXkhDQtk-JmnPOFwZRTe690b4k0sTXXwNdQXfCMp746IVDuDIaDiJTQi1YnmCnhfVkSSBDMXrQw
📊 Found 3 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: 0rMJGv2zBZ3LNxxa89RhTx-DD42mowtXZEmGze8N8fZ5_K-o8_mV7EhfMNqRnyaYBvWQJhk5gVIonukhGrpjyg
📊 Statistics: 2 groups

📝 Example 4: Function Management

📋 Total functions: 3
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.


✅ All examples completed!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
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
   Record({"count": Object({"value": Integer(5), "type": String("Integer")}), "avg_price": Object({"value": Float(367.0), "type": String("Float")}), "category": Object({"type": String("String"), "value": String("Electronics")})})
   Record({"avg_price": Object({"value": Float(365.6666666666667), "type": String("Float")}), "count": Object({"type": String("Integer"), "value": Integer(3)}), "category": Object({"type": String("String"), "value": String("Furniture")})})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.85s
     Running `target/debug/examples/client_functions_ai`
🚀 ekoDB Rust AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases store and search **embeddings**—numeric representations of text, images, audio, and other data. Their main benefits include:

- **Semantic search:** Find items by meaning or similarity, not just exact keywords.
- **Fast nearest-neighbor lookup:** Retrieve similar items efficiently, even across large collections.
- **Support for AI applications:** Power retrieval-augmented generation (RAG), recommendations, image search, and deduplication.
- **Flexible data handling:** Work with varied content by storing its embedding alongside metadata.
- **Filtering and hybrid search:** Combine similarity search with metadata filters or keyword search.

They complement rather than replace traditional databases, which are generally better for exact queries, transactions, and structured data.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embed script saved
📊 Generated 2 embeddings
   Dimensions: 1536
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All AI script examples finished!

💡 This example demonstrates:
   ✅ Chat completions with system/user messages
   ✅ Embedding generation for text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_functions_complete`
🚀 ekoDB Rust Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: tmPjwlh1qYK6uK8SHVaH2BFIWyh8B6FGZT046u_5sf-CwDIMOiPO7UJib5y1uM2m23AHMaaVW5yOOuqtaM-mVw
📊 Found 2 product groups
   Record({"category": Object({"type": String("String"), "value": String("Electronics")}), "count": Object({"value": Integer(3), "type": String("Integer")}), "avg_price": Object({"value": Float(575.6666666666666), "type": String("Float")})})
   Record({"count": Object({"value": Integer(2), "type": String("Integer")}), "avg_price": Object({"type": String("Float"), "value": Float(474.0)}), "category": Object({"value": String("Furniture"), "type": String("String")})})
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   Record({"count": Object({"value": Integer(2), "type": String("Integer")}), "category": Object({"type": String("String"), "value": String("Furniture")})})
   Record({"category": Object({"type": String("String"), "value": String("Electronics")}), "count": Object({"value": Integer(3), "type": String("Integer")})})
⏱️  Execution time: 0ms

📝 Example 4: High Rating Products

✅ Function saved
📊 Found 3 products
⏱️  Execution time: 0ms

📝 Example 5: function with Parameter Definition

✅ Function saved
📊 Found 3 products
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
   Record({"status": Object({"value": String("inactive"), "type": String("String")}), "count": Object({"value": Integer(3), "type": String("Integer")})})
   Record({"count": Object({"value": Integer(7), "type": String("Integer")}), "status": Object({"type": String("String"), "value": String("active")})})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_functions_kv_wrapped`
🚀 ekoDB Rust KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: Some(String("xqLTEnQZwGABBVVFsNR1bfvgtdsp2q7kH1NopNdD3SctBvBWnYyxnr0NayOQabmCyOZ8aqelLyp4l5Np017RTg"))
✅ Inserted 2 products with wrapped types

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: oEMFfE4Bn8hnWyMEpGdQT7WbA73-_pCEst0xL0XF7ViqpZLrcEU5shK_Lh71Ll_Hg7rAhCeiJUl3a5kcyFHdDQ
📊 function executed
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: Some(Object {"type": String("Object"), "value": Object {"role": String("admin"), "userId": String("user_abc")}})
🔍 Key exists: true
✅ Set cached data
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: gbgxJumxr7wUIt7xP3mM3DrqY7agTLv0l46APxSUGmKE3OSRq5QQ0VzrZak4z3Z9jO-KYvS5EcIjsprSzvQinA
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: AEzXgHkY0PmsIqJgV98DbTdQjcswfqdCoDujhTVk3HJQzY1q37-0O8yLFFOUSpEhsgiAD1zPuGuhIcogmB4hOg
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_functions_search`
🚀 ekoDB Rust Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Vector Databases Explained (Database)
   2. Getting Started with ekoDB (Database)
   3. Natural Language Processing (AI)
   4. Introduction to Machine Learning (AI)
   5. Database Design Principles (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   Record({"category": Object({"type": String("String"), "value": String("Database")}), "count": Object({"value": Integer(3), "type": String("Integer")})})
   Record({"category": Object({"value": String("AI"), "type": String("String")}), "count": Object({"type": String("Integer"), "value": Integer(2)})})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_goal_templates`
=== ekoDB Goal Template CRUD Example (Rust) ===

--- Creating goal template ---
Created template: Data Migration (id: hmeAa53wyxdWvA3FzRJlwzr55pxqZyFd7Mgj-L1ZMKEeLiEY_gSe6OUgBQpXIjLHk6HRE9Yx8-1MEi5lySenow)

--- Listing templates ---
Templates: {"count":1,"items":[{"description":{"type":"String","value":"Template for migrating data between schemas"},"id":"hmeAa53wyxdWvA3FzRJlwzr55pxqZyFd7Mgj-L1ZMKEeLiEY_gSe6OUgBQpXIjLHk6HRE9Yx8-1MEi5lySenow","owner":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"steps":{"type":"Array","value":[{"description":"Analyze source schema"},{"description":"Create target schema"},{"description":"Migrate records"},{"description":"Validate results"}]},"title":{"type":"String","value":"Data Migration"}}],"total":1}

--- Getting template ---
Fetched: {"type":"String","value":"Data Migration"}

--- Updating template ---
Updated description: {"type":"String","value":"Updated: comprehensive data migration workflow"}

--- Deleting template ---
Template deleted successfully

✓ Goal template CRUD example completed
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.33s
     Running `target/debug/examples/client_goals_tasks_agents`
=== ekoDB Goals, Tasks & Agents Integration Example (Rust) ===

--- Goal: create ---
Created goal: "Deploy v2.0 to production" (id: Za_aUA0wgTmwK5AfGhyrZaVyhW2_jImskQZ13zQddL-rMtqm8p5gDDsXsQm97KPI6zJa1XOzDMC19YsSlf4QbA)

--- Goal: list ---
Goals: {"count":1,"goals":[{"created_at":{"type":"DateTime","value":"2026-11-02T04:05:36.316504+00:00"},"description":{"type":"String","value":"Full release cycle for version 2.0"},"id":"Za_aUA0wgTmwK5AfGhyrZaVyhW2_jImskQZ13zQddL-rMtqm8p5gDDsXsQm97KPI6zJa1XOzDMC19YsSlf4QbA","owner":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"status":{"type":"String","value":"pending"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-11-02T04:05:36.316504+00:00"}}],"total":1}

--- Goal: get ---
Fetched goal: {"type":"String","value":"Deploy v2.0 to production"}

--- Goal: update ---
Updated description: {"type":"String","value":"Updated: full v2.0 release with rollback plan"}

--- Goal: search ---
Search results: {"count":1,"items":[{"_score":12.87,"created_at":{"type":"DateTime","value":"2026-11-02T04:05:36.316504+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"Za_aUA0wgTmwK5AfGhyrZaVyhW2_jImskQZ13zQddL-rMtqm8p5gDDsXsQm97KPI6zJa1XOzDMC19YsSlf4QbA","owner":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"status":{"type":"String","value":"pending"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-11-02T04:05:36.352489+00:00"}}]}

--- Goal step: start (step 0) ---
Step 0 started: {"created_at":{"type":"DateTime","value":"2026-11-02T04:05:36.316504+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"Za_aUA0wgTmwK5AfGhyrZaVyhW2_jImskQZ13zQddL-rMtqm8p5gDDsXsQm97KPI6zJa1XOzDMC19YsSlf4QbA","owner":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"status":{"type":"String","value":"in_progress"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\",\"status\":\"InProgress\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-11-02T04:05:36.362895+00:00"}}

--- Goal step: complete (step 0) ---
Step 0 completed: {"created_at":{"type":"DateTime","value":"2026-11-02T04:05:36.316504+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"Za_aUA0wgTmwK5AfGhyrZaVyhW2_jImskQZ13zQddL-rMtqm8p5gDDsXsQm97KPI6zJa1XOzDMC19YsSlf4QbA","owner":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"status":{"type":"String","value":"in_progress"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\",\"result\":\"All 247 tests passed\",\"status\":\"Completed\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-11-02T04:05:36.368597+00:00"}}

--- Goal step: start (step 1) ---
Step 1 started

--- Goal step: fail (step 1) ---
Step 1 failed: {"created_at":{"type":"DateTime","value":"2026-11-02T04:05:36.316504+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"Za_aUA0wgTmwK5AfGhyrZaVyhW2_jImskQZ13zQddL-rMtqm8p5gDDsXsQm97KPI6zJa1XOzDMC19YsSlf4QbA","owner":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"status":{"type":"String","value":"in_progress"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\",\"result\":\"All 247 tests passed\",\"status\":\"Completed\"},{\"description\":\"Build release artifacts\",\"error\":\"Build timeout after 300s\",\"status\":\"Failed\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-11-02T04:05:36.381762+00:00"}}

--- Goal: complete ---
Goal completed (pending review): {"completed_at":{"type":"DateTime","value":"2026-11-02T04:05:36.388210+00:00"},"created_at":{"type":"DateTime","value":"2026-11-02T04:05:36.316504+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"Za_aUA0wgTmwK5AfGhyrZaVyhW2_jImskQZ13zQddL-rMtqm8p5gDDsXsQm97KPI6zJa1XOzDMC19YsSlf4QbA","owner":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"status":{"type":"String","value":"pending_review"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\",\"result\":\"All 247 tests passed\",\"status\":\"Completed\"},{\"description\":\"Build release artifacts\",\"error\":\"Build timeout after 300s\",\"status\":\"Failed\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"summary":{"type":"String","value":"Partial completion — step 1 failed"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-11-02T04:05:36.388210+00:00"}}

--- Goal: approve ---
Goal approved: {"completed_at":{"type":"DateTime","value":"2026-11-02T04:05:36.388210+00:00"},"created_at":{"type":"DateTime","value":"2026-11-02T04:05:36.316504+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"Za_aUA0wgTmwK5AfGhyrZaVyhW2_jImskQZ13zQddL-rMtqm8p5gDDsXsQm97KPI6zJa1XOzDMC19YsSlf4QbA","owner":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"status":{"type":"String","value":"in_progress"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\",\"result\":\"All 247 tests passed\",\"status\":\"Completed\"},{\"description\":\"Build release artifacts\",\"error\":\"Build timeout after 300s\",\"status\":\"Failed\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"summary":{"type":"String","value":"Partial completion — step 1 failed"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-11-02T04:05:36.394527+00:00"}}

--- Goal: create (for rejection) ---
Created goal2: "Refactor auth module" (id: fo2Z8065QW1jOLAr23C01Fr9bpmjHkcLxDiZbshBovnNcexoXTkq21sFU8zN9ccRZUpR5cllOG0S1VtQwy1MaA)

--- Goal: complete (goal2) ---

--- Goal: reject ---
Goal rejected: {"completed_at":{"type":"DateTime","value":"2026-11-02T04:05:36.413+00:00"},"created_at":{"type":"DateTime","value":"2026-11-02T04:05:36.400837+00:00"},"description":{"type":"String","value":"Rewrite JWT handling"},"id":"fo2Z8065QW1jOLAr23C01Fr9bpmjHkcLxDiZbshBovnNcexoXTkq21sFU8zN9ccRZUpR5cllOG0S1VtQwy1MaA","owner":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"reason":{"type":"String","value":"Breaks backward compatibility"},"status":{"type":"String","value":"failed"},"steps":{"type":"String","value":"[]"},"summary":{"type":"String","value":"Completed refactor"},"title":{"type":"String","value":"Refactor auth module"},"updated_at":{"type":"DateTime","value":"2026-11-02T04:05:36.413+00:00"}}

--- Task: create ---
Created task: Nightly backup (id: kXZ6Di2qtdfZCm3XTJtW5x6g1c6fkkkzNRvTQgekqI5BXZxR3RBdSn9W50jMBKmJ7ZtEJPl30nhWY0WloUkvCQ)

--- Task: list ---
Tasks: {"count":1,"items":[{"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"kXZ6Di2qtdfZCm3XTJtW5x6g1c6fkkkzNRvTQgekqI5BXZxR3RBdSn9W50jMBKmJ7ZtEJPl30nhWY0WloUkvCQ","max_retries":{"type":"Integer","value":3},"owner":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"schedule":{"type":"String","value":"0 2 * * *"},"title":{"type":"String","value":"Nightly backup"}}],"total":1}

--- Task: get ---
Fetched task: {"type":"String","value":"Nightly backup"}

--- Task: start ---
Task started: {"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"kXZ6Di2qtdfZCm3XTJtW5x6g1c6fkkkzNRvTQgekqI5BXZxR3RBdSn9W50jMBKmJ7ZtEJPl30nhWY0WloUkvCQ","max_retries":{"type":"Integer","value":3},"owner":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"schedule":{"type":"String","value":"0 2 * * *"},"status":{"type":"String","value":"running"},"title":{"type":"String","value":"Nightly backup"},"updated_at":{"type":"DateTime","value":"2026-11-02T04:05:36.452039+00:00"}}

--- Task: succeed ---
Task succeeded: {"consecutive_failures":{"type":"Integer","value":0},"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"kXZ6Di2qtdfZCm3XTJtW5x6g1c6fkkkzNRvTQgekqI5BXZxR3RBdSn9W50jMBKmJ7ZtEJPl30nhWY0WloUkvCQ","last_error":{"type":"Null","value":null},"last_run":{"type":"DateTime","value":"2026-11-02T04:05:36.458356+00:00"},"max_retries":{"type":"Integer","value":3},"owner":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"run_count":{"type":"Integer","value":1},"schedule":{"type":"String","value":"0 2 * * *"},"status":{"type":"String","value":"active"},"title":{"type":"String","value":"Nightly backup"},"updated_at":{"type":"DateTime","value":"2026-11-02T04:05:36.458356+00:00"}}

--- Task: start (round 2) ---

--- Task: pause ---
Task paused: {"consecutive_failures":{"type":"Integer","value":0},"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"kXZ6Di2qtdfZCm3XTJtW5x6g1c6fkkkzNRvTQgekqI5BXZxR3RBdSn9W50jMBKmJ7ZtEJPl30nhWY0WloUkvCQ","last_error":{"type":"Null","value":null},"last_run":{"type":"DateTime","value":"2026-11-02T04:05:36.458356+00:00"},"max_retries":{"type":"Integer","value":3},"owner":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"run_count":{"type":"Integer","value":1},"schedule":{"type":"String","value":"0 2 * * *"},"status":{"type":"String","value":"paused"},"title":{"type":"String","value":"Nightly backup"},"updated_at":{"type":"DateTime","value":"2026-11-02T04:05:36.471102+00:00"}}

--- Task: resume ---
Task resumed: {"consecutive_failures":{"type":"Integer","value":0},"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"kXZ6Di2qtdfZCm3XTJtW5x6g1c6fkkkzNRvTQgekqI5BXZxR3RBdSn9W50jMBKmJ7ZtEJPl30nhWY0WloUkvCQ","last_error":{"type":"Null","value":null},"last_run":{"type":"DateTime","value":"2026-11-02T04:05:36.458356+00:00"},"max_retries":{"type":"Integer","value":3},"owner":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"run_count":{"type":"Integer","value":1},"schedule":{"type":"String","value":"0 2 * * *"},"status":{"type":"String","value":"active"},"title":{"type":"String","value":"Nightly backup"},"updated_at":{"type":"DateTime","value":"2026-11-02T04:05:36.477624+00:00"}}

--- Task: fail ---
Task failed: {"consecutive_failures":{"type":"Integer","value":1},"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"kXZ6Di2qtdfZCm3XTJtW5x6g1c6fkkkzNRvTQgekqI5BXZxR3RBdSn9W50jMBKmJ7ZtEJPl30nhWY0WloUkvCQ","last_error":{"type":"String","value":"S3 bucket access denied"},"last_run":{"type":"DateTime","value":"2026-11-02T04:05:36.487062+00:00"},"max_retries":{"type":"Integer","value":3},"owner":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"run_count":{"type":"Integer","value":1},"schedule":{"type":"String","value":"0 2 * * *"},"status":{"type":"String","value":"active"},"title":{"type":"String","value":"Nightly backup"},"updated_at":{"type":"DateTime","value":"2026-11-02T04:05:36.487062+00:00"}}

--- Task: due ---
Due tasks: {"count":0,"items":[]}

--- Task: delete ---
Task deleted

--- Agent: create ---
Created agent: backup-agent (id: MRDcr4urYxvD_bL7mRF7Bsw3nuAPZwbhnPNyoNFAYXgZICfELUkwMqXqhtYBYP1FvEsPYg5nDzq2zH-rNEZeLA)

--- Agent: list ---
Agents: {"count":1,"items":[{"capabilities":{"type":"Array","value":["backup","restore","verify"]},"deployment_id":{"type":"String","value":"deploy-abc-123"},"description":{"type":"String","value":"Handles nightly backups and restores"},"id":"MRDcr4urYxvD_bL7mRF7Bsw3nuAPZwbhnPNyoNFAYXgZICfELUkwMqXqhtYBYP1FvEsPYg5nDzq2zH-rNEZeLA","llm_model":{"type":"String","value":"gpt-6-luna"},"name":{"type":"String","value":"backup-agent"},"owner":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"run_policy":{"type":"String","value":"{\"trigger\":{\"allow_write_operations\":false,\"write_tools\":[]},\"eval\":{\"allow_write_operations\":false,\"write_tools\":[]},\"customer\":{\"allow_write_operations\":false,\"write_tools\":[]}}"}}],"total":1}

--- Agent: get ---
Fetched agent: {"type":"String","value":"backup-agent"}

--- Agent: get_by_name ---
Found by name: {"type":"String","value":"backup-agent"}

--- Agent: update ---
Updated agent description: {"type":"String","value":"Handles backups, restores, and disaster recovery"}

--- Agent: agents_by_deployment ---
Agents in deployment: {"count":0,"items":[]}
WARNING: agents_by_deployment omitted created agent MRDcr4urYxvD_bL7mRF7Bsw3nuAPZwbhnPNyoNFAYXgZICfELUkwMqXqhtYBYP1FvEsPYg5nDzq2zH-rNEZeLA; TODO: check/fix the server-side deployment lookup

--- Agent: delete ---
Agent deleted

--- Goal: delete ---
Goal 1 deleted
Goal 2 deleted

=== All goals, tasks & agents operations completed ===
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/examples/client_joins`
=== ekoDB Rust Client - Join Operations Example ===

=== Setting up sample data ===
✓ Sample data created

=== Example 1: Single collection join (users with departments) ===
✓ Found 2 users with department data
  - user 1: name present=true, joined departments=1
  - user 2: name present=true, joined departments=1

=== Example 2: Join with filtering ===
✓ Found 1 users in Engineering
  - user 1: name present=true, department location present=true

=== Example 3: Join with user profiles ===
✓ Found 2 users with profile data
  - user 1: name present=true, profile bio present=true
  - user 2: name present=true, profile bio present=true

=== Example 4: Join orders with user data ===
✓ Found 2 completed orders
  - order 1: product present=true, amount present=true, joined user name present=true
  - order 2: product present=true, amount present=true, joined user name present=true

=== Example 5: Complex join with multiple conditions ===
✓ Found 2 users with example.com emails
  - user 1: name present=true, email domain matched=true, joined departments=1
  - user 2: name present=true, email domain matched=true, joined departments=1

=== Cleanup ===
✓ Deleted test collections

✓ Join operations example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.29s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/examples/client_kv_links`
=== ekoDB KV Document Linking Example (Rust) ===

--- Inserting documents ---
Inserted doc1: Project Alpha (id: zhMj_X92QMRgvq1sA1jysi6wQTBncw4qGbneaucXznoqX0rvo6LNgxeDCbFEOvYzrtM4GIs4nNh9QaLUVDCaSQ)
Inserted doc2: Project Beta (id: v0yLZdJ2uoUZUzkXtUNrVNVolnwkD710I9pPf5-OW9As-tikpwHeKXuq382QpuRlQwSArtoMfs96-3JQ8MehYw)

--- Setting KV key ---
Set key: kv_links:rs:user:alice:projects

--- Linking documents ---
Linked doc1: null
Linked doc2: null

--- Getting links ---
Links for kv_links:rs:user:alice:projects: [{"collection":"kv_links_example_rs","created_at":"2026-11-02T04:12:36.175201Z","document_id":"zhMj_X92QMRgvq1sA1jysi6wQTBncw4qGbneaucXznoqX0rvo6LNgxeDCbFEOvYzrtM4GIs4nNh9QaLUVDCaSQ","field_path":null,"last_accessed":"2026-11-02T04:12:36.176813Z","metadata":{}},{"collection":"kv_links_example_rs","created_at":"2026-11-02T04:12:36.176006Z","document_id":"v0yLZdJ2uoUZUzkXtUNrVNVolnwkD710I9pPf5-OW9As-tikpwHeKXuq382QpuRlQwSArtoMfs96-3JQ8MehYw","field_path":null,"last_accessed":"2026-11-02T04:12:36.176813Z","metadata":{}}]

--- Unlinking doc1 ---
Unlinked doc1: null

--- Verifying remaining links ---
Remaining links: [{"collection":"kv_links_example_rs","created_at":"2026-11-02T04:12:36.176006Z","document_id":"v0yLZdJ2uoUZUzkXtUNrVNVolnwkD710I9pPf5-OW9As-tikpwHeKXuq382QpuRlQwSArtoMfs96-3JQ8MehYw","field_path":null,"last_accessed":"2026-11-02T04:12:36.178668Z","metadata":{}}]

--- Cleanup ---
Unlinked remaining document
Deleted KV key
Deleted documents
Deleted collection

=== All KV linking operations completed ===
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/examples/client_kv_operations`
✓ Client created

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: Object {"type": String("Object"), "value": Object {"userId": Number(123), "username": String("john_doe")}}

=== KV Batch Set ===
✓ Batch set 3 keys
  kv_operations:rs:cache:product:1: success
  kv_operations:rs:cache:product:2: success
  kv_operations:rs:cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  kv_operations:rs:cache:product:1: Record({"price": Float(29.99), "name": String("Product 1")})
  kv_operations:rs:cache:product:2: Record({"price": Float(39.989999999999995), "name": String("Product 2")})
  kv_operations:rs:cache:product:3: Record({"name": String("Product 3"), "price": Float(49.989999999999995)})

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
  kv_operations:rs:cache:product:1: deleted
  kv_operations:rs:cache:product:2: deleted
  kv_operations:rs:cache:product:3: deleted

✓ All KV operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_kv_precision`
=== KV Precision: Float vs Decimal ===

=== Test 1: Using f64 Floats (MAY LOSE PRECISION) ===
Stored products with float prices

Retrieved float prices:
  Widget A: $29.99 (expected $29.99) OK
  Widget B: $39.99 (expected $39.99) OK
  Widget C: $49.99 (expected $49.99) OK

=== Test 2: Using FieldType::decimal() (PRESERVES PRECISION) ===
Stored products with decimal prices

Retrieved decimal prices:
  Widget A: $29.99 (expected $29.99) OK
  Widget B: $39.99 (expected $39.99) OK
  Widget C: $49.99 (expected $49.99) OK

=== Test 3: Sum Calculation Comparison ===
  Float sum: $119.97 (expected $119.97)
  Decimal sum: $119.97 (expected $119.97)

=== Test 4: Extreme Precision Example ===
  Float 0.1 + 0.2 = 0.30000000000000004 (should be 0.3)
  Decimal "0.30" = 0.30 (exact!)

=== Cleanup ===
Cleaned up test keys

=== Summary ===
Use FieldType::decimal() for monetary values, percentages, and
any case where floating-point errors are unacceptable.
Decimal values are stored as exact strings internally,
preserving precision across all operations.
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_query_builder`
=== ekoDB Query Builder Example ===

=== Inserting Sample Data ===
✓ Inserted 5 users

=== Simple Equality Query ===
✓ Found 3 active users
  - Some(Object({"type": String("String"), "value": String("Alice")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"type": String("String"), "value": String("David")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"value": String("Eve"), "type": String("String")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"type": String("String"), "value": String("Alice")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"value": String("Eve"), "type": String("String")}))

=== NOT IN Operator ===
✓ Found 4 users not inactive

=== String Pattern Matching ===
✓ Found 5 users with @example.com email

=== Prefix Query (StartsWith) ===
✓ Found 1 users with names starting with A
  - Some(Object({"value": String("Alice"), "type": String("String")}))

=== Complex Query (active AND age >= 28 AND score > 1500) ===
✓ Found 2 users matching all conditions
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("David")}))

=== OR Query ===
✓ Found 2 users with age < 28 OR age > 32
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Charlie")}))

=== Sorted Query (by score descending) ===
✓ Top 3 users by score:
  1. Some(Object({"type": String("String"), "value": String("Bob")}))
  2. Some(Object({"type": String("String"), "value": String("David")}))
  3. Some(Object({"value": String("Alice"), "type": String("String")}))

=== Pagination (page 2, size 2) ===
✓ Page 2 results:
  - Some(Object({"value": String("Charlie"), "type": String("String")}))
  - Some(Object({"value": String("David"), "type": String("String")}))

=== NOT Operator ===
✓ Found 4 users NOT inactive

=== Complex Chained Query ===
✓ Found 3 users with all conditions

=== Cleanup ===
✓ Deleted collection

✓ All query builder operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/examples/client_raw_completion_stream`
=== ekoDB Raw Completion Stream (SSE) Example ===

--- Basic SSE Raw Completion ---
Response: For traditional paint, the three primary colors are **red, yellow, and blue**. For light, they’re **red, green, and blue**.

--- Structured Output via SSE ---
JSON response: [
  {"name": "Earth", "diameter_km": 12742},
  {"name": "Mars", "diameter_km": 6779},
  {"name": "Jupiter", "diameter_km": 139820}
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello!

=== Done ===
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_schedules`
=== ekoDB Schedule Management Example (Rust) ===

--- Creating schedule ---
Created schedule: "nightly-cleanup" (id: fc22532b-f10e-4230-9b17-ab12ac30fe02)

--- Listing schedules ---
Schedules: {"count":1,"schedules":[{"created_at":"2026-11-02T04:19:26.226147Z","cron_expression":"0 0 3 * * *","description":"Remove expired sessions and temp files","enabled":true,"function_label":"schedule_noop_rust_58288","id":"fc22532b-f10e-4230-9b17-ab12ac30fe02","last_execution":null,"name":"nightly-cleanup","next_execution":"2026-11-03T03:00:00Z","parameters":{},"stats":{"avg_execution_time_ms":0.0,"failed_executions":0,"last_error":null,"successful_executions":0,"total_executions":0},"timezone":"UTC","updated_at":"2026-11-02T04:19:26.226147Z"}]}

--- Getting schedule ---
Fetched schedule: "nightly-cleanup"

--- Updating schedule ---
Updated cron: "0 0 4 * * *"
Updated description: "Remove expired sessions, temp files, and orphaned uploads"

--- Triggering schedule ---
Trigger response: {"schedule_id":"fc22532b-f10e-4230-9b17-ab12ac30fe02","status":"triggered"}

--- Pausing schedule ---
Schedule paused: {"created_at":"2026-11-02T04:19:26.226147Z","cron_expression":"0 0 4 * * *","description":"Remove expired sessions, temp files, and orphaned uploads","enabled":false,"function_label":"schedule_noop_rust_58288","id":"fc22532b-f10e-4230-9b17-ab12ac30fe02","last_execution":"2026-11-02T04:19:26.259436Z","name":"nightly-cleanup","next_execution":null,"parameters":{"type":"Object","value":{}},"stats":{"avg_execution_time_ms":0.0,"failed_executions":0,"last_error":null,"successful_executions":0,"total_executions":0},"timezone":"UTC","updated_at":"2026-11-02T04:19:26.262143Z"}

--- Resuming schedule ---
Schedule resumed: {"created_at":"2026-11-02T04:19:26.226147Z","cron_expression":"0 0 4 * * *","description":"Remove expired sessions, temp files, and orphaned uploads","enabled":true,"function_label":"schedule_noop_rust_58288","id":"fc22532b-f10e-4230-9b17-ab12ac30fe02","last_execution":"2026-11-02T04:19:26.259436Z","name":"nightly-cleanup","next_execution":"2026-11-03T04:00:00Z","parameters":{"type":"Object","value":{}},"stats":{"avg_execution_time_ms":0.0,"failed_executions":0,"last_error":null,"successful_executions":0,"total_executions":0},"timezone":"UTC","updated_at":"2026-11-02T04:19:26.264657Z"}

--- Deleting schedule ---
Schedule deleted successfully

=== All schedule operations completed ===
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_schema`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("sJtswd7xPqEqb3sObVlcJfMp7pMbjXESJcBoECZfEl8Qq_h01QmV7JQUEC3uGUiBklwkHgC5k0AC8cXB-fHv3A"))
✓ Inserted user 2: Some(String("WkZByZ0Digxd7nsjKBe1s1sstjb9JQcVnP-5zo2--JzmdbVBN7WrO8SjRLjJZ98DLlFiqUUpijBOwE1GFTWCfg"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - title: String
    (required)
  - status: String
  - email: String
    (required)
  - age: Integer

=== Listing Collections ===
✓ Total collections: 13
  Sample: ["audit__ek0_testing", "schema_client_rust", "chat_raw_completions__ek0_testing", "functions__ek0_testing", "chat_turns__ek0_testing"]

=== Cleanup ===
✓ Deleted collection

✓ All schema management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_search`
=== ekoDB Search Example ===

=== Inserting Sample Documents ===
✓ Inserted 5 sample documents

=== Basic Text Search ===
✓ Found 3 results for 'programming'
  1. Score: 26.4000 - Some(Object {"type": String("String"), "value": String("Rust Programming")})
  2. Score: 13.2000 - Some(Object {"type": String("String"), "value": String("JavaScript Web Development")})
  3. Score: 13.2000 - Some(Object {"type": String("String"), "value": String("Python for Data Science")})

=== Fuzzy Search ===
✓ Found 3 results for 'progamming' (typo)
  1. Score: 2.2000 - Some(Object {"type": String("String"), "value": String("Rust Programming")})
  2. Score: 1.1000 - Some(Object {"type": String("String"), "value": String("JavaScript Web Development")})
  3. Score: 1.1000 - Some(Object {"type": String("String"), "value": String("Python for Data Science")})

=== Field-Specific Search ===
✓ Found 4 results in title/description
  1. Score: 2.0000
     Title: Some(Object {"type": String("String"), "value": String("Machine Learning Basics")})
     Matched: ["description", "title"]
  2. Score: 1.0000
     Title: Some(Object {"type": String("String"), "value": String("Python for Data Science")})
     Matched: ["description"]
  3. Score: 0.5000
     Title: Some(Object {"type": String("String"), "value": String("Rust Programming")})
     Matched: ["description"]
  4. Score: 0.5000
     Title: Some(Object {"type": String("String"), "value": String("Database Design")})
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
  2. Score: 13.2000 - Some(Object {"type": String("String"), "value": String("JavaScript Web Development")})

=== Search with a metadata pre-filter (category = programming) ===
✓ Found 2 results in category 'programming' (database/ai excluded)
  1. Some(Object {"type": String("String"), "value": String("Rust Programming")}) (category: Some(Object {"type": String("String"), "value": String("programming")}))
  2. Some(Object {"type": String("String"), "value": String("Python for Data Science")}) (category: Some(Object {"type": String("String"), "value": String("programming")}))

Execution time: 2ms
Paired vector search: document=Some("Rust Programming Answer"), query=Some("Rust Programming Question")
=== Cleanup ===
✓ Deleted collection

✓ All search operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record({"id": String("XQS0-Kf83mTc3XnbslczA3kSmkiVERP8D8COMD8_N70l4FR_RB6iwsZeplGFby204JhCJztE8rtbJRBwstPCzA")})

=== Find by ID ===
Found: Record({"created_at": Object({"type": String("DateTime"), "value": String("2026-11-02T04:21:47.545659+00:00")}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "embedding": Vector([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "metadata": Object({"type": String("Object"), "value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")})}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "id": String("XQS0-Kf83mTc3XnbslczA3kSmkiVERP8D8COMD8_N70l4FR_RB6iwsZeplGFby204JhCJztE8rtbJRBwstPCzA"), "name": Object({"type": String("String"), "value": String("Test Record")}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "value": Object({"type": String("Integer"), "value": Integer(42)})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-11-02T04:21:47.545659+00:00")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 5 dims
  categories (Set): 2 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record({"user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "value": Object({"type": String("Integer"), "value": Integer(42)}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "embedding": Vector([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "id": String("XQS0-Kf83mTc3XnbslczA3kSmkiVERP8D8COMD8_N70l4FR_RB6iwsZeplGFby204JhCJztE8rtbJRBwstPCzA"), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "metadata": Object({"type": String("Object"), "value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")})}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "name": Object({"type": String("String"), "value": String("Test Record")}), "created_at": Object({"type": String("DateTime"), "value": String("2026-11-02T04:21:47.545659+00:00")})})]

=== Update Document ===
Updated: Record({"categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "price": Object({"value": Float(99.99), "type": String("Float")}), "value": Object({"type": String("Integer"), "value": Integer(100)}), "name": Object({"value": String("Updated Record"), "type": String("String")}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "created_at": Object({"type": String("DateTime"), "value": String("2026-11-02T04:21:47.545659+00:00")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "id": String("XQS0-Kf83mTc3XnbslczA3kSmkiVERP8D8COMD8_N70l4FR_RB6iwsZeplGFby204JhCJztE8rtbJRBwstPCzA"), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "metadata": Object({"type": String("Object"), "value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})})}), "embedding": Vector([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])})

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: Apo4duJHLhWaJQyRpjThA5ABICyXYA9n-pFnkbV-vA5Jjc2S-1-12m_gPEA_sEShAvdUOsi0FqGMwF38e3bL6Q

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  active: true
  name: "WebSocket Test Record"
  id: "Apo4duJHLhWaJQyRpjThA5ABICyXYA9n-pFnkbV-vA5Jjc2S-1-12m_gPEA_sEShAvdUOsi0FqGMwF38e3bL6Q"
  value: 42

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_swr_native`
=== ekoDB Native SWR Function ===

This example shows the new simplified SWR function that replaces
the manual FindById → If → HttpRequest → Insert pattern.

Example 1: Basic GitHub User Cache with Native SWR
─────────────────────────────────────────────────────

✓ Created native SWR script: github_user_native_rs (SPCyyf_cTgY_QTg51xfRSD0c8zdyd6GFpQ6xdctC8l9vpL-ZC6P2ZXeMg57nfuPK1i4vSPGMYMGEAjCWGHTwog)

First call (cache miss - will fetch from GitHub API):
  Response time: 174ms
  Records returned: 1
  ✓ Data fetched from API and cached with 15m TTL

Second call (cache hit - instant from KV store):
  Response time: 5ms
  Speedup: 34.8x faster
  ✓ Lightning fast cache hit


Example 2: SWR with Audit Trail Collection
─────────────────────────────────────────────────────

✓ Created SWR script with audit trail: product_swr_audit_rs (_5qmkdrSS7Xql4IEMGFrWToJYOThEBHI1jMusBT00Vb7_FBy_7m6npei21Xqun5mAJhjJTn07yTWe5b1Xq3umg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_rs' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
─────────────────────────────────────────────────────

Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_rs (QHZFMGxLH3kCW_4Zy8SK8LyNHGgaWusP3zxaW9KdDKzeCFLxo-5TiU3juZN7QpsXBfd5ph0BR7TsrMomDOTgDQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_rs' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
─────────────────────────────────────────────────────

✓ Created dynamic TTL script: flexible_cache_rs (HEaPPp4T35Dt7lN_z8jL6yZJHOij9A0rcp054ckPfy9PJoNfhQCOmmXbFxSyzmK1rsGa_U8NP5brsQvr6547Bg)
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
     Running `target/debug/examples/client_swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_rs (o57zELHybk_RlW7U-4IoSy19CNVySb5yUj0Qoocac7NPO2X7X5Mli7xuqXnGtePa7Mo2pdmaa3Q5UJVb1zN4Lw)

Step 2: First call - Cache miss, fetches from API
Result: FunctionStats { input_count: 0, output_count: 1, execution_time_ms: 0, stages_executed: 2, stage_stats: [] }
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 3ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_transactions`
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: P0ZPgYBLrwNie89ULKMvPmNG_Kms0UkAszOGqOZxXeIZFeMXytC6p2wlLapRVAYcdkf4c639LzPJmisQi1D7Sg
Created Bob: $500 - ID: wd33Qo1zmwidfg0UF2E1xbTgALHrD_TViDzsgiQvHSGVC_-fMqTMrxqRL8hV0heNq_Jzl1B9ENsoRJGhYAgsQA

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 407da11e-aee2-40ab-bc3b-0ea1a179d0c3

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

✓ Verified committed balances: Alice=$800, Bob=$700

=== Example 5: Rollback Demo ===
New transaction: 62df23b3-0587-421d-a756-cb6c5447ee86
Updated Bob: $700 → $600 (in transaction)
Status before rollback: "Active"
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_user_functions`
✓ Client created

=== Create User Function ===
Created user function with ID: PlKjQ1pEN3nM3jViFE9LG1Ekf0Ao-LublrVP-GGiPxPBclqGd7VIi0Yj7PwjGPci_2K24wDEJQK-nJeWw3pK_Q

=== Get User Function ===
Retrieved: get_active_users_rs - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 1 user functions:
  - get_active_users_rs: Get Active Users

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_websocket_chat_stream`
=== WebSocket Chat Streaming Example (Rust) ===

✓ Authentication successful
✓ Created chat session: oADt_wcQ-jWdqdIckGSfH35vo6VH5mhRuzNZNbvPjxze3DUkBFxNlXFxYKGte8u5Gm0BYO1Os-hXOPHOgcEvXQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Sending message: 'What is the capital of France?' ===
✓ Message accepted, streaming...

{"__progress":"received"}{"__progress":"generating","model":"gpt-5.6-luna","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: "GoHWK5IGtlsn1hrCJKw4aJxsFkw1_87PEf5z1DE5a-y7x0TeJVkXRIpVHzLVq9C6C9fw86lUFBeWdFsFZp2fyQ"
Execution time: 1450ms
Token usage: {"cached_input_tokens":0,"completion_tokens":11,"prompt_tokens":23,"thinking_tokens":0,"total_tokens":34}
Context window: 128000 tokens

Full response: {"__progress":"received"}{"__progress":"generating","model":"gpt-5.6-luna","provider":"openai"}The capital of France is Paris....

✓ WebSocket chat streaming example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_websocket_subscribe`
✓ Authentication successful

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_rs' ===
✓ Subscribed (subscription_id: sub_5e5de5bdfeee4112b1dd87cebe032c17)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: B6s16GlFDWIC9uCs9dB4fe7EUreVxDnacRhFwedBPzfCG5oXHq996eMSbomsQPk7yZj1AolWYz9e4qhAIMQ0Zw

  📡 Notification received:
     Event:      "insert"
     Collection: "ws_subscribe_example_rs"
     Record IDs: ["B6s16GlFDWIC9uCs9dB4fe7EUreVxDnacRhFwedBPzfCG5oXHq996eMSbomsQPk7yZj1AolWYz9e4qhAIMQ0Zw"]
     Timestamp:  "2026-11-02T04:35:02.007204+00:00"

Inserting record 2...
✓ Inserted: uuAxWJVdpv551wdgpuAuLnox7xmYk6HgiHwllK5AtI7gNOfRAlNun2Fzf7Pp08fSLYHrepSeG_WHHD1HjRm4VQ

  📡 Notification received:
     Event:      "insert"
     Record IDs: ["uuAxWJVdpv551wdgpuAuLnox7xmYk6HgiHwllK5AtI7gNOfRAlNun2Fzf7Pp08fSLYHrepSeG_WHHD1HjRm4VQ"]

=== Unsubscribing ===
✓ Unsubscribed

=== Cleanup ===
✓ Deleted collection 'ws_subscribe_example_rs'

✓ WebSocket subscription example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_websocket_ttl`
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Some(String("SJF0IulhE4vV_Ju30cQWza5w_Y7BHXNzKhqpaebP8MH5fHcPXBA7EPWYNjNFBsvPo4prSPlJo1Q4C075OwQaRA"))

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/bypass_ripple_example`
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: Record({"id": String("hU_BORISYmzpQSDLA9N01pOM4ZHymIVQ-bwi5w-hZSlcKN8S4uGMB-hhI2k1lijI-mcclQUkAUPXZyBiaFKqsA")})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: Record({"id": String("Ht09ys85rBUyDLvHweZO3F7C-STFvXudi5NHbeGPG7TymMTTzCZmJAM2m0CClwaiHs8Xqz9oKLqiOpn5UTMdmQ")})

3. Update with bypass_ripple:
   Updated with bypass_ripple: Record({"name": Object({"value": String("Product 1"), "type": String("String")}), "id": String("hU_BORISYmzpQSDLA9N01pOM4ZHymIVQ-bwi5w-hZSlcKN8S4uGMB-hhI2k1lijI-mcclQUkAUPXZyBiaFKqsA"), "price": Object({"type": String("Integer"), "value": Integer(150)})})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: Record({"id": String("custom-id")})

✅ All bypass_ripple operations completed successfully!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
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
  Fields returned: ["email", "avatar_url", "user_role", "bio", "created_at", "name", "status", "age", "id"]

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)

Example 4: Query inactive users with profile fields
  Found 1 inactive users

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ["secret_token", "user_role", "created_at", "avatar_url", "status", "age", "name", "email", "id", "password", "api_key", "bio"]
  Projected query:
    - 3 fields per record
    - Fields: ["name", "email", "id"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
