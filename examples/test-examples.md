make test-examples
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
✅ All Rust integration tests complete!
📦 Ensuring Python example dependencies in .venv...
🧪 Running Python examples (direct HTTP/WebSocket)...
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'alitHW5T4m5rSpmPyFtBkQKbUrNwft04Cn05LSlntPi8gtFbkBDV0DLMHuIgfXUa_QBex2f2yz5MWfqgrKRpdg'}

=== Find by ID ===
Found: {'value': {'value': 42, 'type': 'Integer'}, 'id': 'alitHW5T4m5rSpmPyFtBkQKbUrNwft04Cn05LSlntPi8gtFbkBDV0DLMHuIgfXUa_QBex2f2yz5MWfqgrKRpdg', 'active': {'type': 'Boolean', 'value': True}, 'name': {'type': 'String', 'value': 'Test Record'}}

=== Find with Query ===
Found documents: [{'name': {'type': 'String', 'value': 'Test Record'}, 'id': 'alitHW5T4m5rSpmPyFtBkQKbUrNwft04Cn05LSlntPi8gtFbkBDV0DLMHuIgfXUa_QBex2f2yz5MWfqgrKRpdg', 'active': {'type': 'Boolean', 'value': True}, 'value': {'value': 42, 'type': 'Integer'}}]

=== Update Document ===
Updated: {'name': {'value': 'Updated Record', 'type': 'String'}, 'value': {'type': 'Integer', 'value': 100}, 'active': {'value': True, 'type': 'Boolean'}, 'id': 'alitHW5T4m5rSpmPyFtBkQKbUrNwft04Cn05LSlntPi8gtFbkBDV0DLMHuIgfXUa_QBex2f2yz5MWfqgrKRpdg'}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: YuQn7swrJIJ8phzRH8cR5R1NHtJN1KOYmyaQE97VBiBiQAJi1xcIvEFPXa4EvqU8CQw2W3Y54gU1tMTk-rA22g

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "type": "Success",
  "payload": {
    "data": [
      {
        "value": {
          "value": 42,
          "type": "Integer"
        },
        "active": {
          "value": true,
          "type": "Boolean"
        },
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "id": "YuQn7swrJIJ8phzRH8cR5R1NHtJN1KOYmyaQE97VBiBiQAJi1xcIvEFPXa4EvqU8CQw2W3Y54gU1tMTk-rA22g"
      }
    ]
  },
  "messageId": "876269990"
}
✓ Retrieved 1 record via WebSocket

✓ WebSocket example completed successfully
🚀 ekoDB Functions Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: zFxDo1KFDsfripBEMInd_1uGJpp6ADKBcNdY_3rIWukU0AIC2Wbx4IWXIkdhUIBChrXh6iUMq2xDVLJxTzrJuA
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: _prJzrxLQKYuB5nVBD_ab-kmFacLvkC8h6XY6tzMuRt0PjB7RQLIF5mPTm4tGrX5ZIt4nlWcdnzPxjJt6jrarw
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: oc_iNXoGnZee2v4tX-McUB94kVWxM9j0VlSunPLcczkXHnxpuzNI3hyzoGRRXnvPT4QR8xhzD7RhxLUOgS6e7w
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'avg_score': {'type': 'Float', 'value': 50.0}, 'max_score': {'value': 90, 'type': 'Integer'}, 'count': {'type': 'Integer', 'value': 5}, 'status': {'value': 'inactive', 'type': 'String'}}
   {'count': {'type': 'Integer', 'value': 5}, 'avg_score': {'type': 'Float', 'value': 60.0}, 'max_score': {'value': 100, 'type': 'Integer'}, 'status': {'value': 'active', 'type': 'String'}}

📝 Example 4: Function Management

📋 Total scripts: 3
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
=== Key-Value Operations (Direct HTTP) ===

✓ Authentication successful

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: {'type': 'Object', 'value': {'username': 'john_doe', 'userId': 123}}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
kv_operations:direct:py:cache:product:1: {'value': {'name': 'Product 1', 'price': 29.99}, 'type': 'Object'}
kv_operations:direct:py:cache:product:2: {'type': 'Object', 'value': {'name': 'Product 2', 'price': 39.989999999999995}}
kv_operations:direct:py:cache:product:3: {'value': {'price': 49.989999999999995, 'name': 'Product 3'}, 'type': 'Object'}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: xSNne4Hju3jmPKjilFRqC5v8WWYgDo1ZoUKTlZiDTAn5ZRtCx1Evsjt0z4m4pFiy6z98_3lS8obqbqbWqCCR8g

=== List Collections ===
Total collections: 13
Sample collections: ['audit__ek0_testing', 'chat_raw_completions__ek0_testing', 'functions__ek0_testing', 'chat_turns__ek0_testing', 'chat_goal_templates__ek0_testing']

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
Created Alice: $1000 - ID: TcGhtZ3njo2LhblWSC1KWWSeqpyqpclHu19VtUdEmy_ahTUhRrFhs0NRt87GvoQtL1aDyzcTEyaqj1rVA2EG6g
Created Bob: $500 - ID: 3wNq_2pTCIe45D_Kd9Pq917I3hNcGppXiYghUq0-pbZ0GfPUMunCGkZqocis01sOeV6mOoauG_fID0KOZc465Q

=== Example 1: Begin Transaction ===
Transaction ID: facaae35-4804-4899-9e24-e3d7a563187f

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
New transaction: fa608ce4-fe06-4b95-9a2d-649f670abd88
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 1QEKWKeenM8i069n6RhogQHsGmneL7uvg3IFanivG33V5_e9lGz8RgPrHK0i0OZKVXrcmvwekBxhB6E2V5S4GA
Created Bob: $500 - ID: A8gRn3EZem-UlQVNakbPM5ZDV2dYXtWFGKfMyoj-8FEgkZ-OROlF5gYSgEfFTJNFWiMn6mE6AknayGi2NCDx9A

=== Example 1: Begin Transaction ===
Transaction ID: a3693e52-0e8d-4614-b84d-21006176ce52

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


🔄 Switching to NON-DURABLE mode...
   ✓ Config updated: durable_operations=false


============================================================
🔥 TEST 2: Non-Durable Mode (durable=false)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: ii_dYdINTWWoFB_uERuCa0QDOOkQNZXqYa7qkeCrZ0Y2crVn6Us0Ir1xrN0c9ZbqfwbKP4dcJGXmZxetkz3iSg
Created Bob: $500 - ID: pn6cUCDCeYTGNJLOmc_iYIS1UH5KEzW2M2NBQgUtzRQ1bdZKbvrvVCj-sNKlA5KBmI0CUprXW3NpRPe-M1ribQ

=== Example 1: Begin Transaction ===
Transaction ID: 81ac7ab3-8554-40aa-aef1-98625d18df34

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
New transaction: 3d2fc8bd-bff8-42d0-beeb-fb112700c6a8
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: Hvq782Y8biqOfdV9do3YqtegFOS5QGTq87Fnknj5pcIi3lmCXXEWUcT2cEDt2blz2CUjLzUv5q8sZPYuSDx8yw
Created Bob: $500 - ID: 9snKyXGPxb2kQpugQweNpSx_ePdOOl-KKFr9vH-Inma9nDhAAuxEfQ-38cQrFyf3XDEM7BmunmusqmMWiRfaOQ

=== Example 1: Begin Transaction ===
Transaction ID: 688de700-df68-4ae3-9abc-a60c861d0a61

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


🔄 Switching to DURABLE mode...
   ✓ Config updated: durable_operations=true


============================================================
🔥 TEST 3: Durable Mode (durable=true)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: PkJ1J4zYg01hQ99IO0y1YZ6ieDfaB3jXe7_A8DiA63mp34jrbozpcRaR1HdEltmnCcdNwupXP_mbOuTTlhu0aw
Created Bob: $500 - ID: Tq3KB8KJyaPVClvXLaFWEpxqL9h5LpWEHJALxQP1opfMkIbOIBxVVNteJvt-nH6GEFcjjAPmd2tJPhPgqmlFEA

=== Example 1: Begin Transaction ===
Transaction ID: 0ba85aae-2b5f-4560-a48f-ba442dfab519

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
New transaction: c495f19c-becf-445a-9878-b4d3d935d9ef
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: hT8Z4oJZwE35aCxm0RX1AXxe6oFu8ZhnU4StagS44JQGv-ObYG_6XUcD5qSb7BCbs7BWVVLpp4onEyc3w1Nf4Q
Created Bob: $500 - ID: OhV6N-_uxH5kz2mA8XM8WMQEhMoLtuJruqQunkX_9v6c4eFTpP4O2ymoRdIvj-YG8C0lCD5RHsAkwrAuyOT72g

=== Example 1: Begin Transaction ===
Transaction ID: dd867d43-1344-45b8-818b-d0c7b6d33a40

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
🧹 Cleanup
============================================================
   ✅ Deleted collection: crud_functions_users_py
============================================================
📝 function 1: Insert + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: j2z7ZZ42CMfJOK3JuJtsH9fjHOulyX1qOqH02xanDlVOx_PzHLS11ZAVK4dkA0l7MgTNzuEXh609NfBq28Bjqg

2️⃣ Calling function (Insert + Verify)...
   ✅ function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: cKwwOgtORilZkiQWRpyu344RnPIX0bc4OyX5rH3zY56Pi0ZEt5e3A38AiB8B154tdrgG3-_ZD3Abun76avB2yQ
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'type': 'String', 'value': 'pending'}
   📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================
📝 function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: N0-g1eAAfBJ5_Heq_PSj1J_cKCppXoR9T8YraS1Sg5X6Jbtoosun6Qad6Z4PXzL90v-yaVb4yTf11r8TN0jokA

2️⃣ Calling function (Query + Update + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {'value': 'active', 'type': 'String'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 function 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: 5pMYvEbe62ZzkXRrq2WunDfSvO4d2VaI73HtfDNENxlyxhaK_WInvlv514hvrLWJ--uyTM-bTc5SnhHYzYd9pg

2️⃣ Calling function (Query + Update Credits + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'type': 'Integer', 'value': 100}
   📋 Status: {'value': 'active', 'type': 'String'}
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}

============================================================
📝 function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: JmbI4CTua70IE20lkZWMv3J3Wt9sPfDafsO-JoshVlWzO3OEIxJ3YV7HYrBgBC1bBCVtPK9lidcPG4Z_ip3EXA

2️⃣ Calling function (Query + Delete + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: j2z7ZZ42CMfJOK3JuJts...
   ✅ Deleted script: N0-g1eAAfBJ5_Heq_PSj...
   ✅ Deleted script: 5pMYvEbe62ZzkXRrq2Wu...
   ✅ Deleted script: JmbI4CTua70IE20lkZWM...
   ✅ Deleted collection: crud_functions_users_py

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
  Output: Document ID = QN8nNb6EseCCv4XdU9mtTjSkLuOG0n-LWO-WNvRVdapCCg64U4SMgJveeezt83NWzGk_WJaI-ABPJHBjLjbiLQ
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(QN8nNb6EseCCv4XdU9mtTjSkLuOG0n-LWO-WNvRVdapCCg64U4SMgJveeezt83NWzGk_WJaI-ABPJHBjLjbiLQ)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(QN8nNb6EseCCv4XdU9mtTjSkLuOG0n-LWO-WNvRVdapCCg64U4SMgJveeezt83NWzGk_WJaI-ABPJHBjLjbiLQ)
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
  Output: Document ID = 36_X2JZi_AoPFb5dLI-yVK8tL6g4HDbJ2UshDwr2vM9OrRgN349wRTvoHsblcQuJEMmBSIo6-sda_TbUJYLr0g
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(36_X2JZi_AoPFb5dLI-yVK8tL6g4HDbJ2UshDwr2vM9OrRgN349wRTvoHsblcQuJEMmBSIo6-sda_TbUJYLr0g)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(36_X2JZi_AoPFb5dLI-yVK8tL6g4HDbJ2UshDwr2vM9OrRgN349wRTvoHsblcQuJEMmBSIo6-sda_TbUJYLr0g)
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

╔════════════════════════════════════════╗
║   ekoDB Python Examples Test Suite    ║
╚════════════════════════════════════════╝

=== Checking Server Connection ===
✓ Server is ready

=== Getting Authentication Token ===
✓ Authentication successful

=== Running 10 Examples ===

=== Running simple_crud.py ===
✓ simple_crud.py completed successfully

=== Running simple_websocket.py ===
✓ simple_websocket.py completed successfully

=== Running http_functions.py ===
✓ http_functions.py completed successfully

=== Running batch_operations.py ===
✓ batch_operations.py completed successfully

=== Running kv_operations.py ===
✓ kv_operations.py completed successfully

=== Running collection_management.py ===
✓ collection_management.py completed successfully

=== Running transactions.py ===
✓ transactions.py completed successfully

=== Running crud_functions.py ===
✓ crud_functions.py completed successfully

=== Running document_ttl.py ===
✓ document_ttl.py completed successfully

=== Running websocket_ttl.py ===
✓ websocket_ttl.py completed successfully

╔════════════════════════════════════════╗
║           Test Summary                 ║
╚════════════════════════════════════════╝
Total: 10
Passed: 10
Failed: 0
✅ Python direct examples complete!
🐍 Building Python client package...
🔧 Ensuring maturin is available in .venv...
🔨 Building wheel...
🍹 Building a mixed python/rust project
🐍 Found CPython 3.15 at /Users/tek/.local/bin/python3
🔗 Found pyo3 bindings with abi3-py3.8 support
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.20s
📦 Built wheel for abi3 Python ≥ 3.8 to ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.31.0-cp38-abi3-macosx_11_0_arm64.whl
📦 Installing Python wheel into .venv...
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.31.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.31.0
    Uninstalling ekodb_client-0.31.0:
      Successfully uninstalled ekodb_client-0.31.0
Successfully installed ekodb-client-0.31.0
🧪 Ensuring test dependencies (pytest) in .venv...
✅ Python client package built and installed!
Admin permissions read successfully
=== ekoDB Advanced CRUD Example (Python) ===

--- insert ---
Inserted: zj6WmhSGlc-QjAyzZUekj65bygXafZdV8UpcdlzzuVNyty-BXh1Z6NMiCKxIZBEIVJrQtcqhhODPU48LE1R8Vg

--- update_with_action (increment) ---
After increment count by 5: {'name': {'type': 'String', 'value': 'Counter Record'}, 'count': {'value': 15, 'type': 'Integer'}, 'id': 'zj6WmhSGlc-QjAyzZUekj65bygXafZdV8UpcdlzzuVNyty-BXh1Z6NMiCKxIZBEIVJrQtcqhhODPU48LE1R8Vg', 'tags': {'value': ['initial'], 'type': 'Array'}, 'score': {'value': 50.0, 'type': 'Float'}}

--- update_with_action (decrement) ---
After decrement score by 10: {'count': {'value': 15, 'type': 'Integer'}, 'id': 'zj6WmhSGlc-QjAyzZUekj65bygXafZdV8UpcdlzzuVNyty-BXh1Z6NMiCKxIZBEIVJrQtcqhhODPU48LE1R8Vg', 'tags': {'type': 'Array', 'value': ['initial']}, 'score': {'type': 'Float', 'value': 40.0}, 'name': {'value': 'Counter Record', 'type': 'String'}}

--- update_with_action_sequence ---
After action sequence: {'tags': {'type': 'Array', 'value': ['initial', 'sequenced']}, 'id': 'zj6WmhSGlc-QjAyzZUekj65bygXafZdV8UpcdlzzuVNyty-BXh1Z6NMiCKxIZBEIVJrQtcqhhODPU48LE1R8Vg', 'name': {'value': 'Counter Record', 'type': 'String'}, 'count': {'type': 'Integer', 'value': 115}, 'score': {'value': 40.0, 'type': 'Float'}}

--- cleanup ---
Cleaned up collection

=== Advanced CRUD example completed ===
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
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: zUwx0vKmIh84zFj9lAtm01XgJSZW1muwLwjvy-jjQVOMNIATtwTFPovnZ3E9rnsdgdCmKa9-EoF90247bC0hLQ

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is **ekoDB**, described as a high-performance database product, priced at **99**.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['created_at', 'author', 'content', 'llm_provider', 'chat_id', 'llm_model', 'role', 'token_usage', 'updated_at', 'context_snippets', 'id'])
Debug: First message role: {'type': 'String', 'value': 'assistant'}
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: ekoDB costs **99**. The currency isn’t specified.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: LKpuiQnTchck8TFLIkaX-kzGdOIYlc2as4mWtUrUANGa78wseWx8ZmeLLsL7YPoTyvPDMdLs6uRpME-9X2lHOg
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: LKpuiQnTchck8TFLIkaX-kzGdOIYlc2as4mWtUrUANGa78wseWx8ZmeLLsL7YPoTyvPDMdLs6uRpME-9X2lHOg
✓ Deleted chat session: zUwx0vKmIh84zFj9lAtm01XgJSZW1muwLwjvy-jjQVOMNIATtwTFPovnZ3E9rnsdgdCmKa9-EoF90247bC0hLQ
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: r6z7B8cRIjiwcF8guMFZa1buacDA4pQlGrSjuff22lDfxDDVnOpucURKLnBT5hh4T3tW3JMNNkUGPirmTBbxqQ

=== Sending Chat Message ===
Message ID: ZTvVBwp27GL1rlFFVxGZKMozdrh2S9gD-oKoMxQVAECS-RlQbgAXeK20tXVB_Z_-sXtQm665uf2_QtvwR0AmAg

=== AI Response ===
Available products and listed prices:

- **ekoDB** — 99
- **ekoDB Pro** — 299
- **ekoDB Cloud** — 499

The currency isn’t specified in the data.

Execution Time: 2101ms

=== Token Usage ===
Prompt tokens: 13716
Completion tokens: 80
Total tokens: 13796

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Message Stream (SSE) Example (Python) ===

Created session: 1ulK0_bmI-Lt0Wjz9u-MsLCKMVChyI5ajbT0KB6HX5Frh6cyWljtn7YVRn_N7_6xPfLOPmmvvHoMBvmGa8A8hA

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"gpt-6-luna","provider":"openai"}ekoDB is a lightweight database project for storing and querying application data. It’s generally described as an embedded database, meaning it can run alongside an app rather than requiring a separate database server. There may be multiple projects with that name, so a link would help if you mean a specific one.

--- Stream complete ---
Message ID: m1xsZPpflmKuXOs_8o9Bx12dRRR7CblNjxu8ozjtA1m0ImHs8rP9eY6PeeRMvEKJc7OaReiPQneAlS87OeSknA
Execution time: 10120ms
Context window: 128000 tokens

✓ Chat message stream example completed
✓ Client created

=== Get All Chat Models ===
OpenAI models: ['gpt-3.5-turbo-0125', 'gpt-5.4-2026-03-05', 'gpt-5-nano', 'gpt-5.5-pro', 'gpt-audio-2025-08-28', 'gpt-4o-mini-transcribe-2025-03-20', 'gpt-4-turbo-2024-04-09', 'gpt-realtime-2.1-mini', 'gpt-realtime-2025-08-28', 'gpt-4', 'tts-1-hd', 'gpt-realtime-1.5', 'gpt-realtime', 'gpt-5.2-pro', 'gpt-5-chat-latest', 'gpt-5.4-mini-2026-03-17', 'babbage-002', 'gpt-4o-2024-05-13', 'text-embedding-3-large', 'tts-1', 'omni-moderation-2024-09-26', 'gpt-live-transcribe', 'gpt-5.6-luna', 'gpt-5-2025-08-07', 'gpt-5-mini', 'gpt-5-mini-2025-08-07', 'omni-moderation-latest', 'o3-mini-2025-01-31', 'gpt-audio-1.5', 'gpt-image-1.5', 'tts-1-1106', 'gpt-5.5-pro-2026-04-23', 'gpt-audio-mini-2025-10-06', 'gpt-5.2-pro-2025-12-11', 'o4-mini-2025-04-16', 'gpt-4o-2024-11-20', 'gpt-5.1', 'whisper-1', 'gpt-5-pro', 'gpt-realtime-whisper', 'gpt-realtime-2', 'gpt-5-search-api', 'gpt-5.5-2026-04-23', 'gpt-4.1-nano', 'gpt-image-2', 'gpt-3.5-turbo-16k', 'text-embedding-ada-002', 'gpt-realtime-2.1', 'gpt-4.1', 'gpt-5.5', 'chat-latest', 'gpt-realtime-translate', 'gpt-5.4-pro-2026-03-05', 'gpt-4.1-mini', 'gpt-4o-mini', 'gpt-4o-mini-transcribe-2025-12-15', 'gpt-realtime-mini', 'gpt-image-1-mini', 'gpt-5.6-terra', 'gpt-3.5-turbo-instruct-0914', 'gpt-4-turbo', 'gpt-4o-2024-08-06', 'o3', 'gpt-4o-mini-search-preview-2025-03-11', 'o1-2024-12-17', 'gpt-5.1-chat-latest', 'gpt-5-pro-2025-10-06', 'gpt-5.6-sol', 'gpt-4o-transcribe-diarize', 'gpt-5-codex', 'gpt-4-0613', 'gpt-5.1-2025-11-13', 'gpt-4o-search-preview', 'gpt-4o-mini-tts-2025-12-15', 'gpt-image-1', 'gpt-audio', 'gpt-5.3-codex', 'gpt-5-search-api-2025-10-14', 'gpt-5.4-pro', 'gpt-4o-mini-tts', 'gpt-realtime-mini-2025-12-15', 'gpt-5.1-codex-max', 'sora-2', 'gpt-4o', 'gpt-4o-mini-2024-07-18', 'gpt-4.1-2025-04-14', 'gpt-5', 'gpt-4o-mini-transcribe', 'gpt-image-2-2026-04-21', 'gpt-5.2-chat-latest', 'sora-2-pro', 'gpt-3.5-turbo-instruct', 'gpt-transcribe', 'gpt-4.1-nano-2025-04-14', 'gpt-3.5-turbo-1106', 'gpt-4o-mini-tts-2025-03-20', 'gpt-5.2', 'gpt-5.4-nano-2026-03-17', 'o1', 'tts-1-hd-1106', 'gpt-5.2-codex', 'gpt-5.4-mini', 'o1-pro', 'gpt-5.1-codex', 'o4-mini-deep-research', 'gpt-5.4', 'gpt-4o-search-preview-2025-03-11', 'text-embedding-3-small', 'gpt-5-nano-2025-08-07', 'gpt-4o-transcribe', 'o4-mini', 'gpt-5.1-codex-mini', 'gpt-6-astra', 'gpt-5.3-chat-latest', 'o3-2025-04-16', 'gpt-5.2-2025-12-11', 'chatgpt-image-latest', 'gpt-3.5-turbo', 'gpt-image-2.5-sunburst-2026-09-08', 'gpt-image-2.5-flare', 'davinci-002', 'gpt-5.4-nano', 'o3-mini', 'o4-mini-deep-research-2025-06-26', 'gpt-audio-mini-2025-12-15', 'o1-pro-2025-03-19', 'gpt-live-1', 'gpt-image-2.5-sunburst', 'gpt-image-2.5-flare-2026-09-08', 'gpt-6.1-terra', 'gpt-audio-mini', 'gpt-4.1-mini-2025-04-14', 'gpt-6-luna', 'gpt-4o-mini-search-preview', 'gpt-6-sol', 'gpt-6.1-sol']
Anthropic models: ['claude-haiku-5-5', 'claude-sonnet-5-5', 'claude-opus-5-5', 'claude-fable-5-1', 'claude-opus-5', 'claude-sonnet-5', 'claude-fable-5', 'claude-opus-4-8', 'claude-opus-4-7', 'claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929']
Perplexity models: []
Gemini models: []

=== Get OpenAI Models ===
OpenAI models: ['gpt-3.5-turbo-0125', 'gpt-5.4-2026-03-05', 'gpt-5-nano', 'gpt-5.5-pro', 'gpt-audio-2025-08-28', 'gpt-4o-mini-transcribe-2025-03-20', 'gpt-4-turbo-2024-04-09', 'gpt-realtime-2.1-mini', 'gpt-realtime-2025-08-28', 'gpt-4', 'tts-1-hd', 'gpt-realtime-1.5', 'gpt-realtime', 'gpt-5.2-pro', 'gpt-5-chat-latest', 'gpt-5.4-mini-2026-03-17', 'babbage-002', 'gpt-4o-2024-05-13', 'text-embedding-3-large', 'tts-1', 'omni-moderation-2024-09-26', 'gpt-live-transcribe', 'gpt-5.6-luna', 'gpt-5-2025-08-07', 'gpt-5-mini', 'gpt-5-mini-2025-08-07', 'omni-moderation-latest', 'o3-mini-2025-01-31', 'gpt-audio-1.5', 'gpt-image-1.5', 'tts-1-1106', 'gpt-5.5-pro-2026-04-23', 'gpt-audio-mini-2025-10-06', 'gpt-5.2-pro-2025-12-11', 'o4-mini-2025-04-16', 'gpt-4o-2024-11-20', 'gpt-5.1', 'whisper-1', 'gpt-5-pro', 'gpt-realtime-whisper', 'gpt-realtime-2', 'gpt-5-search-api', 'gpt-5.5-2026-04-23', 'gpt-4.1-nano', 'gpt-image-2', 'gpt-3.5-turbo-16k', 'text-embedding-ada-002', 'gpt-realtime-2.1', 'gpt-4.1', 'gpt-5.5', 'chat-latest', 'gpt-realtime-translate', 'gpt-5.4-pro-2026-03-05', 'gpt-4.1-mini', 'gpt-4o-mini', 'gpt-4o-mini-transcribe-2025-12-15', 'gpt-realtime-mini', 'gpt-image-1-mini', 'gpt-5.6-terra', 'gpt-3.5-turbo-instruct-0914', 'gpt-4-turbo', 'gpt-4o-2024-08-06', 'o3', 'gpt-4o-mini-search-preview-2025-03-11', 'o1-2024-12-17', 'gpt-5.1-chat-latest', 'gpt-5-pro-2025-10-06', 'gpt-5.6-sol', 'gpt-4o-transcribe-diarize', 'gpt-5-codex', 'gpt-4-0613', 'gpt-5.1-2025-11-13', 'gpt-4o-search-preview', 'gpt-4o-mini-tts-2025-12-15', 'gpt-image-1', 'gpt-audio', 'gpt-5.3-codex', 'gpt-5-search-api-2025-10-14', 'gpt-5.4-pro', 'gpt-4o-mini-tts', 'gpt-realtime-mini-2025-12-15', 'gpt-5.1-codex-max', 'sora-2', 'gpt-4o', 'gpt-4o-mini-2024-07-18', 'gpt-4.1-2025-04-14', 'gpt-5', 'gpt-4o-mini-transcribe', 'gpt-image-2-2026-04-21', 'gpt-5.2-chat-latest', 'sora-2-pro', 'gpt-3.5-turbo-instruct', 'gpt-transcribe', 'gpt-4.1-nano-2025-04-14', 'gpt-3.5-turbo-1106', 'gpt-4o-mini-tts-2025-03-20', 'gpt-5.2', 'gpt-5.4-nano-2026-03-17', 'o1', 'tts-1-hd-1106', 'gpt-5.2-codex', 'gpt-5.4-mini', 'o1-pro', 'gpt-5.1-codex', 'o4-mini-deep-research', 'gpt-5.4', 'gpt-4o-search-preview-2025-03-11', 'text-embedding-3-small', 'gpt-5-nano-2025-08-07', 'gpt-4o-transcribe', 'o4-mini', 'gpt-5.1-codex-mini', 'gpt-6-astra', 'gpt-5.3-chat-latest', 'o3-2025-04-16', 'gpt-5.2-2025-12-11', 'chatgpt-image-latest', 'gpt-3.5-turbo', 'gpt-image-2.5-sunburst-2026-09-08', 'gpt-image-2.5-flare', 'davinci-002', 'gpt-5.4-nano', 'o3-mini', 'o4-mini-deep-research-2025-06-26', 'gpt-audio-mini-2025-12-15', 'o1-pro-2025-03-19', 'gpt-live-1', 'gpt-image-2.5-sunburst', 'gpt-image-2.5-flare-2026-09-08', 'gpt-6.1-terra', 'gpt-audio-mini', 'gpt-4.1-mini-2025-04-14', 'gpt-6-luna', 'gpt-4o-mini-search-preview', 'gpt-6-sol', 'gpt-6.1-sol']

=== Get Anthropic Models ===
Anthropic models: ['claude-haiku-5-5', 'claude-sonnet-5-5', 'claude-opus-5-5', 'claude-fable-5-1', 'claude-opus-5', 'claude-sonnet-5', 'claude-fable-5', 'claude-opus-4-8', 'claude-opus-4-7', 'claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929']

✓ Chat Models API example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: aLgm6JdePd7jVZ6OWB1z1BKhbLNwam0_dNn6jliIEPC_TfzyGNgOOOJLIAx5p4oGlSkqWQK2zOabzozRSPF-hg

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is **ekoDB**, a high-performance database product, priced at **$99**.

✓ Message 2 sent
  Response: ekoDB is priced at **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: d11JMIlSIdub24NV5MZBNAVWX07siRK7CwB_ANhukfpyLZ6Em6x87gWqqJEZYSMWTaEhXP_j_l5TAoPyNllRlA
  Parent: aLgm6JdePd7jVZ6OWB1z1BKhbLNwam0_dNn6jliIEPC_TfzyGNgOOOJLIAx5p4oGlSkqWQK2zOabzozRSPF-hg

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: d11JMIlSIdub24NV5MZBNAVWX07siRK7CwB_ANhukfpyLZ6Em6x87gWqqJEZYSMWTaEhXP_j_l5TAoPyNllRlA (Untitled)
  Session 2: aLgm6JdePd7jVZ6OWB1z1BKhbLNwam0_dNn6jliIEPC_TfzyGNgOOOJLIAx5p4oGlSkqWQK2zOabzozRSPF-hg (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: d11JMIlSIdub24NV5MZBNAVWX07siRK7CwB_ANhukfpyLZ6Em6x87gWqqJEZYSMWTaEhXP_j_l5TAoPyNllRlA

=== Cleanup ===
✓ Deleted sessions and collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "am27GBve294XMevPVYYaM-Bf8B44XJXqyThATX3LORxcRt_FdUa-E4swbYUIeoG6bCDmDgphqIjvbVaS1VE_DA"

=== List Collections ===
Total collections: 13
Sample collections: ['audit__ek0_testing', 'client_collection_management_python', 'chat_raw_completions__ek0_testing', 'functions__ek0_testing', 'chat_turns__ek0_testing']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
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
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': 'aDI-pKNn-onni_hYaX8diXs1MCzm1G-KXZqqc2TJu5GmPr8G6b89iktIY7oVp2fVAs64MdSUnY3h7eF9ytKSWw'}

=== Upsert Operation ===
✓ Upsert (update existing record): aDI-pKNn-onni_hYaX8diXs1MCzm1G-KXZqqc2TJu5GmPr8G6b89iktIY7oVp2fVAs64MdSUnY3h7eF9ytKSWw
✓ Inserted second record: LZhHmFVvHT5W4mXEoxV5lr6_ob6jv9SU8E4-U2lNbRwpXITx30HnMXOOrDmTdkrF2sIr0uN_n1T32BVpbMPH6A
✓ Upsert (update second record): LZhHmFVvHT5W4mXEoxV5lr6_ob6jv9SU8E4-U2lNbRwpXITx30HnMXOOrDmTdkrF2sIr0uN_n1T32BVpbMPH6A

=== Find One Operation ===
✓ Found user by email: {'id': 'aDI-pKNn-onni_hYaX8diXs1MCzm1G-KXZqqc2TJu5GmPr8G6b89iktIY7oVp2fVAs64MdSUnY3h7eF9ytKSWw', 'active': {'type': 'Boolean', 'value': True}, 'name': {'type': 'String', 'value': 'Alice Johnson'}, 'age': {'type': 'Integer', 'value': 29}, 'email': {'type': 'String', 'value': 'alice.j@newdomain.com'}}
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
✓ Client created
✓ crypto_demo_hmac_py saved
✓ crypto_demo_aes_py saved
✓ crypto_demo_uuid_py saved
✓ crypto_demo_totp_py saved
✓ crypto_demo_encoding_py saved

Invoke them with:
  POST /api/functions/crypto_demo_hmac_py { "payload": "hi" }
  POST /api/functions/crypto_demo_aes_py { "plaintext": "secret" }
  POST /api/functions/crypto_demo_uuid_py
  POST /api/functions/crypto_demo_totp_py
  POST /api/functions/crypto_demo_encoding_py { "title": "Héllo World" }

✓ Cleaned up demo functions
=== Distinct Values Example ===

Inserting sample products...
Inserted 8 products

=== Distinct Categories (all products) ===
Found 3 distinct categories:
  - {'type': 'String', 'value': 'books'}
  - {'type': 'String', 'value': 'clothing'}
  - {'type': 'String', 'value': 'electronics'}

=== Distinct Statuses (all products) ===
Found 3 distinct statuses:
  - {'type': 'String', 'value': 'active'}
  - {'type': 'String', 'value': 'archived'}
  - {'type': 'String', 'value': 'discontinued'}

=== Distinct Statuses in Electronics ===
Found 2 distinct statuses for electronics:
  - {'type': 'String', 'value': 'active'}
  - {'type': 'String', 'value': 'discontinued'}

Cleanup done.
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: st32LXg_kNS-IElmlq2Kwxua9Lk4S2S6RaP0FNpVadhEadK-ItznR0ufT8KCgiIqdgIf9avHP8-V0_LhCBks0Q

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: KcEYJqR19ZFsTGJuCU6mqXJ_51j0iRxfHfTRwRLDgakRgftJP35o8zoMqfFJN0e5HRJK2dOig4bU89TNlYt-kw

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
=== ekoDB as Edge Cache - Simple Example ===

Creating edge cache function...
✓ Edge cache script created: u7YqYRio7EdaTJYHgmQBKBjTzhyvYeOkPXgLVE03nZO1XD4Rnr-lzQfQ3pZm9ZLhZVEFym817N_lpQD1z2MGLw

Call 1: Cache miss (fetches from API)
Response time: 161ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "current": {
            "interval": 900,
            "temperature_2m": 8.5,
            "time": "2026-11-02T04:30"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "\u00b0C",
            "time": "iso8601"
          },
          "elevation": 32.0,
          "generationtime_ms": 0.030875205993652344,
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
Response time: 2ms (69.7x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "current": {
            "interval": 900,
            "temperature_2m": 8.5,
            "time": "2026-11-02T04:30"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "\u00b0C",
            "time": "iso8601"
          },
          "elevation": 32.0,
          "generationtime_ms": 0.030875205993652344,
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

=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

✅ Saved reusable function: fetch_user_py
✅ Saved composed function: get_user_wrapper_py (calls fetch_user_py + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"type": "String", "value": "User 1"}
   Department: {"type": "String", "value": "engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user_py (uses KV)
✅ Saved SWR function using composition: swr_user_py

First call (cache miss - will fetch from API):
   ⏱️  Duration: 77.3ms
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
   ⏱️  Duration: 2.1ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "type": "Object",
            "value": {
                  "address": {
                        "city": "Gwenborough",
                        "geo": {
                 ...
   🚀 Cache speedup: 37.5x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user_py
✅ Level 2 function: fetch_slim_user_py (calls validate_user_py)
✅ Level 3 function: get_verified_user_py (calls fetch_slim_user_py)

📊 Result from 3-level nested composition:
   Records: 1
   Name: User 1
   Department: engineering

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - get_verified_user: Specific workflow


✅ Cleanup complete
✅ All composition examples completed!
client_function_contract: ok
🚀 ekoDB Functions Example (Python)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: VKbrBoKSnaJ-SojTXxZh3VjsfrEI57DmzTNhhdYqXhM5EnqPXqvNQzh9XoZfgBWHap1wvLcZtnnKTHGdD6vZ3g
📊 Found 5 active users

📝 Example 2: Parameterized Function

✅ Function saved: GWUMeJSjdoFtbO1nWlHI3kvNRtjUN657mEiWlg098P7GqBl2oc_S2vdTggrgqcWNgczWXaSQE4m74U7eIPOf5g
📊 Found 3 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: -IjBHxboF7flAyQhHiqQXnONf3gKmvQ9x1ICEr0gvQXP79dYPEmOKzoUJPebgEUL7bJr5bg8CqWMpAeILys8lQ
📊 Statistics: 2 groups
   {'avg_score': {'type': 'Float', 'value': 50.0}, 'count': {'type': 'Integer', 'value': 5}, 'status': {'type': 'String', 'value': 'inactive'}}

   {'avg_score': {'type': 'Float', 'value': 60.0}, 'count': {'type': 'Integer', 'value': 5}, 'status': {'type': 'String', 'value': 'active'}}

📝 Example 4: Function Management

📋 Total functions: 3
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

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
   Vector databases are designed to store and search **embeddings**—numeric representations of text, images, audio, or other data. Their main benefits include:

- **Semantic search:** Find items by meaning or similarity, not just exact keywords.
- **Fast similarity search at scale:** Use specialized indexes to find close matches across large collections.
- **RAG and AI applications:** Retrieve relevant documents to provide context to language models.
- **Recommendations and matching:** Suggest similar products, content, images, or users.
- **Metadata filtering:** Combine similarity search with conditions such as category, date, or access permissions.
- **Less custom infrastructure:** Manage vector storage, indexing, and retrieval without building those systems from scratch.

For small datasets or simple keyword search, a traditional database may be sufficient; vector databases are most useful when similarity search is central to the application.
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
🚀 ekoDB Python Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: cUaK47jNEocnnSen9quJMcg0-rNWqLF8OnAk4Oa_VE7dQNRX2T5rCAGnNUzugVj9wY71L3LNQ-E4PsGgozvleA
📊 Found 2 product groups
   {'avg_price': {'type': 'Float', 'value': 575.6666666666666}, 'category': {'type': 'String', 'value': 'Electronics'}, 'count': {'type': 'Integer', 'value': 3}}
   {'avg_price': {'type': 'Float', 'value': 474.0}, 'category': {'type': 'String', 'value': 'Furniture'}, 'count': {'type': 'Integer', 'value': 2}}
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

✅ All complete script examples finished!

💡 This example demonstrates ekoDB's function system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Parameter definitions
   ✅ Function management (save, call, delete)
🧹 Cleaning up...
✅ Cleanup complete

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
🚀 ekoDB Python KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: YSFKFz8wDSOJz3h6w9KMiXT9jZemollBnmGXUAo-chRIiWAHX5qSudQpfC0yJBQIzs1665s7lPSDpwra3RQ7GA
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

✅ Inserted order: OvS1zUaIObd_i1-YN8NIq2RB0pouAjqEeevrB2VNwKLV9Ak2BaIofjw2qfjhDWCW4SKPyXRHagShlRbgM1A2Mw
✅ Cached order status
📊 Quick status lookup: {'value': '{"type":"Object","value":{"status":"processing","updated_at":"2026-11-02T04:42:37.821584+00:00"}}'}

🧹 Cleaning up...
✅ Cleanup complete

✅ All KV & Wrapped Types examples completed!

💡 Key takeaways:
   ✅ Use field_* helpers for type-safe wrapped values
   ✅ field_decimal() preserves precision (no floating point errors)
   ✅ KV store is great for caching and quick lookups
   ✅ Combine KV caching with collection inserts for real workflows
🚀 ekoDB Python Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Getting Started with ekoDB (Database)
   2. Vector Databases Explained (Database)
   3. Database Design Principles (Database)
   4. Introduction to Machine Learning (AI)
   5. Natural Language Processing (AI)
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
=== ekoDB Goal Template CRUD Example (Python) ===

--- Creating goal template ---
Created template: Data Migration (id: vD_Z68XCHynfkZdhQeLPoisa2uiave5hRJUW9fNCrXLNjWzyrV5sidMKt_M1WDG-coR9Hk6_LJke0SgQKRH9_Q)

--- Listing templates ---
Templates: {'count': 1, 'items': [{'description': {'type': 'String', 'value': 'Template for migrating data between schemas'}, 'id': 'vD_Z68XCHynfkZdhQeLPoisa2uiave5hRJUW9fNCrXLNjWzyrV5sidMKt_M1WDG-coR9Hk6_LJke0SgQKRH9_Q', 'owner': {'type': 'Object', 'value': {'key_id': '3a0cdba0da7c060610042783cc758f1f', 'key_label': 'default_admin'}}, 'steps': {'type': 'Array', 'value': [{'description': 'Analyze source schema'}, {'description': 'Create target schema'}, {'description': 'Migrate records'}, {'description': 'Validate results'}]}, 'title': {'type': 'String', 'value': 'Data Migration'}}], 'total': 1}

--- Getting template ---
Fetched: {'type': 'String', 'value': 'Data Migration'}

--- Updating template ---
Updated description: {'type': 'String', 'value': 'Updated: comprehensive data migration workflow'}

--- Deleting template ---
Template deleted successfully

✓ Goal template CRUD example completed
=== ekoDB Goals / Tasks / Agents Integration Example (Python) ===

--- goal_create ---
Created goal: Deploy v3 (id: yhDXbv1--zSbUnde0ywEQTXiWtvmCeql1tIanySIgOa9hYqSwpKJ905tBlmQ0x4pawUJzgszqlbKpcxkjOAYcg)

--- goal_list ---
Goals: {'count': 1, 'goals': [{'created_at': {'type': 'DateTime', 'value': '2026-11-02T04:42:38.271359+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production'}, 'id': 'yhDXbv1--zSbUnde0ywEQTXiWtvmCeql1tIanySIgOa9hYqSwpKJ905tBlmQ0x4pawUJzgszqlbKpcxkjOAYcg', 'owner': {'type': 'Object', 'value': {'key_id': '3a0cdba0da7c060610042783cc758f1f', 'key_label': 'default_admin'}}, 'status': {'type': 'String', 'value': 'pending'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","status":"pending"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-11-02T04:42:38.271359+00:00'}}], 'total': 1}

--- goal_get ---
Fetched: {'type': 'String', 'value': 'Deploy v3'}

--- goal_update ---
Updated description: {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}

--- goal_search ---
Search results: {'count': 1, 'items': [{'_score': 13.2, 'created_at': {'type': 'DateTime', 'value': '2026-11-02T04:42:38.271359+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'yhDXbv1--zSbUnde0ywEQTXiWtvmCeql1tIanySIgOa9hYqSwpKJ905tBlmQ0x4pawUJzgszqlbKpcxkjOAYcg', 'owner': {'type': 'Object', 'value': {'key_id': '3a0cdba0da7c060610042783cc758f1f', 'key_label': 'default_admin'}}, 'status': {'type': 'String', 'value': 'pending'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","status":"pending"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-11-02T04:42:38.282001+00:00'}}]}

--- goal_step_start (step 0) ---
Step 0 started: {'created_at': {'type': 'DateTime', 'value': '2026-11-02T04:42:38.271359+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'yhDXbv1--zSbUnde0ywEQTXiWtvmCeql1tIanySIgOa9hYqSwpKJ905tBlmQ0x4pawUJzgszqlbKpcxkjOAYcg', 'owner': {'type': 'Object', 'value': {'key_id': '3a0cdba0da7c060610042783cc758f1f', 'key_label': 'default_admin'}}, 'status': {'type': 'String', 'value': 'in_progress'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","status":"InProgress"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-11-02T04:42:38.291192+00:00'}}

--- goal_step_complete (step 0) ---
Step 0 completed: {'created_at': {'type': 'DateTime', 'value': '2026-11-02T04:42:38.271359+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'yhDXbv1--zSbUnde0ywEQTXiWtvmCeql1tIanySIgOa9hYqSwpKJ905tBlmQ0x4pawUJzgszqlbKpcxkjOAYcg', 'owner': {'type': 'Object', 'value': {'key_id': '3a0cdba0da7c060610042783cc758f1f', 'key_label': 'default_admin'}}, 'status': {'type': 'String', 'value': 'in_progress'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","result":"Docker image built: sha256:abc123","status":"Completed"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-11-02T04:42:38.297834+00:00'}}

--- goal_step_fail (step 1) ---
Step 1 failed: {'created_at': {'type': 'DateTime', 'value': '2026-11-02T04:42:38.271359+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'yhDXbv1--zSbUnde0ywEQTXiWtvmCeql1tIanySIgOa9hYqSwpKJ905tBlmQ0x4pawUJzgszqlbKpcxkjOAYcg', 'owner': {'type': 'Object', 'value': {'key_id': '3a0cdba0da7c060610042783cc758f1f', 'key_label': 'default_admin'}}, 'status': {'type': 'String', 'value': 'in_progress'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","result":"Docker image built: sha256:abc123","status":"Completed"},{"description":"Run migrations","error":"Migration failed: column already exists","status":"Failed"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-11-02T04:42:38.304249+00:00'}}

--- goal_complete ---
Goal completed: {'type': 'String', 'value': 'pending_review'}

--- goal_approve ---
Goal approved: {'type': 'String', 'value': 'in_progress'}

--- goal_reject ---
Goal rejected: {'type': 'String', 'value': 'failed'}

--- task_create ---
Created task: Nightly Backup (id: wQ9omT9JOet6Duhtkd8t_kwQLSPO6CkEKlC9oNsdpS2fLYvp6CwiHQNJaHGvxpPU-ol5FtmMChOF6W5fk5Lb-Q)

--- task_list ---
Tasks: {'count': 1, 'items': [{'cron': {'type': 'String', 'value': '0 2 * * *'}, 'description': {'type': 'String', 'value': 'Backup all databases nightly'}, 'id': 'wQ9omT9JOet6Duhtkd8t_kwQLSPO6CkEKlC9oNsdpS2fLYvp6CwiHQNJaHGvxpPU-ol5FtmMChOF6W5fk5Lb-Q', 'max_consecutive_failures': {'type': 'Integer', 'value': 3}, 'name': {'type': 'String', 'value': 'Nightly Backup'}, 'owner': {'type': 'Object', 'value': {'key_id': '3a0cdba0da7c060610042783cc758f1f', 'key_label': 'default_admin'}}}], 'total': 1}

--- task_get ---
Fetched: {'type': 'String', 'value': 'Nightly Backup'}

--- task_start ---
Task started: {'type': 'String', 'value': 'running'}

--- task_succeed ---
Task succeeded: {'consecutive_failures': {'type': 'Integer', 'value': 0}, 'cron': {'type': 'String', 'value': '0 2 * * *'}, 'description': {'type': 'String', 'value': 'Backup all databases nightly'}, 'id': 'wQ9omT9JOet6Duhtkd8t_kwQLSPO6CkEKlC9oNsdpS2fLYvp6CwiHQNJaHGvxpPU-ol5FtmMChOF6W5fk5Lb-Q', 'last_error': {'type': 'Null', 'value': None}, 'last_run': {'type': 'DateTime', 'value': '2026-11-02T04:42:38.365499+00:00'}, 'max_consecutive_failures': {'type': 'Integer', 'value': 3}, 'name': {'type': 'String', 'value': 'Nightly Backup'}, 'owner': {'type': 'Object', 'value': {'key_id': '3a0cdba0da7c060610042783cc758f1f', 'key_label': 'default_admin'}}, 'run_count': {'type': 'Integer', 'value': 1}, 'status': {'type': 'String', 'value': 'active'}, 'updated_at': {'type': 'DateTime', 'value': '2026-11-02T04:42:38.365499+00:00'}}

--- task_pause ---
Task paused: {'type': 'String', 'value': 'paused'}

--- task_resume ---
Task resumed: {'type': 'String', 'value': 'active'}

--- task_fail ---
Task failed: {'consecutive_failures': {'type': 'Integer', 'value': 1}, 'cron': {'type': 'String', 'value': '0 2 * * *'}, 'description': {'type': 'String', 'value': 'Backup all databases nightly'}, 'id': 'wQ9omT9JOet6Duhtkd8t_kwQLSPO6CkEKlC9oNsdpS2fLYvp6CwiHQNJaHGvxpPU-ol5FtmMChOF6W5fk5Lb-Q', 'last_error': {'type': 'String', 'value': 'Disk full'}, 'last_run': {'type': 'DateTime', 'value': '2026-11-02T04:42:38.394121+00:00'}, 'max_consecutive_failures': {'type': 'Integer', 'value': 3}, 'name': {'type': 'String', 'value': 'Nightly Backup'}, 'next_run': {'type': 'DateTime', 'value': '2026-03-22T02:00:00+00:00'}, 'owner': {'type': 'Object', 'value': {'key_id': '3a0cdba0da7c060610042783cc758f1f', 'key_label': 'default_admin'}}, 'run_count': {'type': 'Integer', 'value': 1}, 'status': {'type': 'String', 'value': 'active'}, 'updated_at': {'type': 'DateTime', 'value': '2026-11-02T04:42:38.394121+00:00'}}

--- task_due ---
Due tasks: {'count': 0, 'items': []}

--- task_delete ---
Task deleted

--- agent_create ---
Created agent: CodeReviewer (id: 7GLW_UXJ7GOv1ypDcueMGj_BEkLBRt8oIQCfWwYCrFZwuQsjFw8DfLaXScxVKrwEcCY-gU_1VdJr5Z9cvvAuew)

--- agent_list ---
Agents: {'count': 1, 'items': [{'deployment_id': {'type': 'String', 'value': 'deploy_test'}, 'id': '7GLW_UXJ7GOv1ypDcueMGj_BEkLBRt8oIQCfWwYCrFZwuQsjFw8DfLaXScxVKrwEcCY-gU_1VdJr5Z9cvvAuew', 'llm_model': {'type': 'String', 'value': 'gpt-6-luna'}, 'name': {'type': 'String', 'value': 'CodeReviewer'}, 'owner': {'type': 'Object', 'value': {'key_id': '3a0cdba0da7c060610042783cc758f1f', 'key_label': 'default_admin'}}, 'run_policy': {'type': 'String', 'value': '{"trigger":{"allow_write_operations":false,"write_tools":[]},"eval":{"allow_write_operations":false,"write_tools":[]},"customer":{"allow_write_operations":false,"write_tools":[]}}'}, 'system_prompt': {'type': 'String', 'value': 'You review code for correctness and style.'}, 'tools': {'type': 'Array', 'value': ['web_search', 'file_read']}}], 'total': 1}

--- agent_get ---
Fetched: {'type': 'String', 'value': 'CodeReviewer'}

--- agent_get_by_name ---
By name: {'type': 'String', 'value': 'CodeReviewer'}

--- agent_update ---
Updated agent prompt: {'type': 'String', 'value': 'You review code. Be concise.'}

--- agents_by_deployment ---
Agents in deploy_test: {'count': 0, 'items': []}
WARNING: agents_by_deployment omitted created agent 7GLW_UXJ7GOv1ypDcueMGj_BEkLBRt8oIQCfWwYCrFZwuQsjFw8DfLaXScxVKrwEcCY-gU_1VdJr5Z9cvvAuew; TODO: check/fix the server-side deployment lookup

--- agent_delete ---
Agent deleted

--- cleanup ---
Goals cleaned up

=== All goals/tasks/agents operations completed successfully ===
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
✓ py_users_register saved
✓ py_users_login saved
✓ py_users_verify_token saved

=== Auth flow defined as pure stored functions ===
Call them like:
  POST /api/functions/py_users_register { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/py_users_login    { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/py_users_verify_token { "token": "<jwt>" }

✓ Cleaned up demo functions
=== ekoDB KV Links Example (Python) ===

--- kv_set ---
Set KV key: app:config

--- insert document ---
Inserted document: xfPO_MBWecWeMm_Rf58f1fmdQ_QM1I1A3ADgG56deYOz5mF9mtsqr7IMyvUJmkDQc7Xy8RqKT2ZFzeQqjFVauw

--- kv_link ---
Linked: None

--- kv_get_links ---
Links for app:config: [{'collection': 'kv_links_example_py', 'created_at': '2026-11-02T04:42:39.057Z', 'document_id': 'xfPO_MBWecWeMm_Rf58f1fmdQ_QM1I1A3ADgG56deYOz5mF9mtsqr7IMyvUJmkDQc7Xy8RqKT2ZFzeQqjFVauw', 'field_path': None, 'last_accessed': '2026-11-02T04:42:39.057939Z', 'metadata': {}}]

--- kv_unlink ---
Unlinked: None

--- kv_get_links (verify empty) ---
Links after unlink: []

--- cleanup ---
Cleaned up KV key and collection

=== KV links example completed ===
✓ Client created

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: {'value': '{"type":"Object","value":{"userId":123,"username":"john_doe"}}'}

=== KV Batch Set ===
✓ Batch set 3 keys
  kv_operations:py:cache:product:1: success
  kv_operations:py:cache:product:2: success
  kv_operations:py:cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  kv_operations:py:cache:product:1: {'price': 29.99, 'name': 'Product 1'}
  kv_operations:py:cache:product:2: {'name': 'Product 2', 'price': 39.99}
  kv_operations:py:cache:product:3: {'price': 49.99, 'name': 'Product 3'}

=== KV Exists ===
Key exists: True

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 4

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: False

=== KV Batch Delete ===
✓ Batch deleted 3 keys
  kv_operations:py:cache:product:1: deleted
  kv_operations:py:cache:product:2: deleted
  kv_operations:py:cache:product:3: deleted

✓ All KV operations completed successfully
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
=== ekoDB Raw Completion Stream (SSE) Example ===

--- Basic SSE Raw Completion ---
Response: For traditional painting, the primary colors are **red, yellow, and blue**. For light and screens, they’re **red, green, and blue**.

--- Structured Output via SSE ---
JSON response: [
  {"name": "Earth", "diameter_km": 12742},
  {"name": "Mars", "diameter_km": 6779},
  {"name": "Jupiter", "diameter_km": 139820}
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello! 👋

=== Done ===
=== ekoDB Schedule Management Example (Python) ===

--- create_schedule ---
Created schedule: Nightly Report (id: 212f40aa-c7da-43ca-aacb-bfa41ee024e5)

--- list_schedules ---
Schedules: {'count': 1, 'schedules': [{'created_at': '2026-11-02T04:42:44.441846Z', 'cron_expression': '0 0 0 * * *', 'description': 'Generate and email nightly analytics report', 'enabled': True, 'function_label': 'schedule_noop_python_25170', 'id': '212f40aa-c7da-43ca-aacb-bfa41ee024e5', 'last_execution': None, 'name': 'Nightly Report', 'next_execution': '2026-11-03T00:00:00Z', 'parameters': {}, 'stats': {'avg_execution_time_ms': 0.0, 'failed_executions': 0, 'last_error': None, 'successful_executions': 0, 'total_executions': 0}, 'timezone': 'UTC', 'updated_at': '2026-11-02T04:42:44.441846Z'}]}

--- get_schedule ---
Fetched: Nightly Report cron=0 0 0 * * *

--- update_schedule ---
Updated cron: 0 30 1 * * *

--- trigger_schedule ---
Triggered: {'schedule_id': '212f40aa-c7da-43ca-aacb-bfa41ee024e5', 'status': 'triggered'}

--- pause_schedule ---
Paused: enabled=False

--- resume_schedule ---
Resumed: enabled=True
Schedule deleted successfully

=== Schedule management example completed ===
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
  4. Score: 13.200, Matched: title, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400, Matched: bio, title
  2. Score: 26.400, Matched: title, bio
  3. Score: 26.400, Matched: bio, title
  4. Score: 26.400, Matched: title, bio

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
  1. Score: 0.760
  2. Score: 0.759
  3. Score: 0.733

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.367, Matched: title, content
  2. Score: 0.879, Matched: content, title
  3. Score: 0.380, Matched:

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 6.600, Matched: title

9. Vector search with a metadata pre-filter (category = ml):
Found 2 documents in category "ml" (NLP excluded)
  1. Deep Learning Fundamentals (category: ml)
  2. Introduction to Machine Learning (category: ml)

Paired vector search: document=Rust Programming Answer, query=Rust Programming Question
=== Cleanup ===
✅ Deleted test collections

✅ Search examples completed!
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': '7vlRWqin3s-e6aTi3ZquRAunfZuARkmphnGSkwFfmanV2xIupP47MQLwGAGyt1tE3PUZlEnZ6KqkuNhWr46gIA'}

=== Find by ID ===
Found: {'active': {'value': True, 'type': 'Boolean'}, 'created_at': {'type': 'String', 'value': '2026-11-01T23:42:45.054465'}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'id': '7vlRWqin3s-e6aTi3ZquRAunfZuARkmphnGSkwFfmanV2xIupP47MQLwGAGyt1tE3PUZlEnZ6KqkuNhWr46gIA', 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'metadata': {'value': {'key': 'value', 'nested': {'deep': True}}, 'type': 'Object'}, 'name': {'type': 'String', 'value': 'Test Record'}, 'value': {'type': 'Integer', 'value': 42}, 'price': {'value': 99.99, 'type': 'Float'}, 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-11-01 23:42:45.054465
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'key': 'value', 'nested': {'deep': True}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'active': True, 'created_at': '2026-11-01T23:42:45.054465', 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'id': '7vlRWqin3s-e6aTi3ZquRAunfZuARkmphnGSkwFfmanV2xIupP47MQLwGAGyt1tE3PUZlEnZ6KqkuNhWr46gIA', 'categories': ['electronics', 'computers'], 'data': 'aGVsbG8gd29ybGQ=', 'metadata': {'key': 'value', 'nested': {'deep': True}}, 'name': 'Test Record', 'value': 42, 'price': 99.99, 'tags': ['tag1', 'tag2', 'tag3']}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'active': {'value': True, 'type': 'Boolean'}, 'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}, 'value': {'value': 100, 'type': 'Integer'}, 'price': {'value': 99.99, 'type': 'Float'}, 'created_at': {'value': '2026-11-01T23:42:45.054465', 'type': 'String'}, 'metadata': {'value': {'key': 'value', 'nested': {'deep': True}}, 'type': 'Object'}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'name': {'type': 'String', 'value': 'Updated Record'}, 'id': '7vlRWqin3s-e6aTi3ZquRAunfZuARkmphnGSkwFfmanV2xIupP47MQLwGAGyt1tE3PUZlEnZ6KqkuNhWr46gIA'}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: qi5a5d-BbhywNourD6NwXp3PW1n8mEX4iFlXIf5xP7vsQPz9w4yAjkfTX4ooQPcsxNrUak3IUE6-BNW7Ujmqlw

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
🚀 ekoDB Python Client - Native SWR Function Examples

📋 Demonstrates:
   • Single-function SWR pattern (replaces 4-step pipeline)
   • Automatic cache checking, HTTP fetching, and cache setting
   • Built-in audit trail support
   • Duration string TTLs ('15m', '1h', '30s')
   • Multi-function pipeline integration
   • Dynamic TTL configuration


🧹 Cleaning up...
✓ Deleted 0 test scripts and owned SWR resources

Example 1: Basic Native SWR
────────────────────────────────────────────────────────────────────────────────
Single function replaces KvGet → If → HttpRequest → KvSet pipeline
✓ Created native SWR script: github_user_native_py (cmk44uzSvkc160_NW4--yN-BO3pPghfWHywAt0u5j8MGjqJp9a9qyvGratV9sNJzSlq5MNl4eL6gu1GMwM9cKg)

First call (cache miss - will fetch from GitHub API):
  Response time: 311ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 3ms
  Speedup: 97.7x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_py (X3IIbSJpgGxOeJ48o_LrskZ3lsS2qqCj5ZdeQ_xaXFkLyfBAvN0z5flc_to3o1HUVvg4JELfbqRoiikF_0nf1w)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_py' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_py (yfR2YSKzR3qDV4wGo8JIn_yarlaO8ICq6nk1_-Bz3dtqszqXbZjOO_9oYvnLFoApv3hoTmf7uOBi-Mx-7alNyg)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_py' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_py (dWPE3_-b5XjvCCgTQB3U6ASxqIFijYhBk_4RZYk9SlEhtWSP5Pz7ZTebJHq8syhXbLZGoSfwSAIYLz7GiCEPaA)
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
✓ Deleted 4 test scripts and owned SWR resources

✅ All examples completed!
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_py (6l901tmyq0vSyDKn4UUFHKxUIAfxLydeP6Q8WPLA8mo-lOp07QfAKhS82nrL0RdgpdV8-lD7S2O2e6e6nRcL3g)

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
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: r-XXWvHX5kQQ0apB3qpYKlEOmZrycT_B1Yv7ABl-LNf9y1ZM37wQQF2ICJvTVl7qBVgdkEyhzl8xB9uMrJef4Q
Created Bob: $500 - ID: RxPRWoEaLOITwkGbMS7jiZHsRvM_3Thdwr5ibV_QM1LAhyW6Nig_L3ZC2UswaHOJ-SrLUwivucnDycltz27fvA

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 9935eff9-a2eb-49b3-936f-1b849b29d457

=== Example 2: Operations within Transaction ===
Alice in transaction: {'id': 'r-XXWvHX5kQQ0apB3qpYKlEOmZrycT_B1Yv7ABl-LNf9y1ZM37wQQF2ICJvTVl7qBVgdkEyhzl8xB9uMrJef4Q', 'name': {'type': 'String', 'value': 'Alice'}, 'account_id': {'type': 'String', 'value': 'ACC001'}, 'balance': {'value': 800, 'type': 'Integer'}}
Bob in transaction: {'name': {'type': 'String', 'value': 'Bob'}, 'id': 'RxPRWoEaLOITwkGbMS7jiZHsRvM_3Thdwr5ibV_QM1LAhyW6Nig_L3ZC2UswaHOJ-SrLUwivucnDycltz27fvA', 'balance': {'type': 'Integer', 'value': 700}, 'account_id': {'value': 'ACC002', 'type': 'String'}}
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

✓ Verified committed balances: Alice=$800, Bob=$700

=== Example 5: Rollback Demo ===
New transaction: f957fef2-6827-43ce-be54-f2964c536361
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Create User Function ===
Created user function with ID: ADheaPBRNzZx5DWV8tTAuUO-oIHQ5kXw3YKJ9Fax7wWeBgmuIV-USkrxtCp7y8_kihpZOd3pkirdl9zZOLhtbA

=== Get User Function ===
Retrieved: get_active_users_py - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 1 user functions:
  - get_active_users_py: Get Active Users

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
=== WebSocket Chat Streaming Example (Python) ===

Created chat session: 30JSPiqVQUQWoIC12fnlUB4aQ8NCewHNOvyYFF0gCvt83o0kahizcCqp-GLjC2cWf8mocxpTkPc14ABZPr6UYw

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"gpt-5.6-luna","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: Cuqz-SLr845ALQppRnMtSSTsFGo5HBvVyYfk8g-DjlTorXEiVYUUgorIjxkEDkxpW8895mtfvvKaYXotHpO_-A
Execution time: 2734ms
Token usage: {'cached_input_tokens': 0, 'completion_tokens': 11, 'prompt_tokens': 23, 'thinking_tokens': 0, 'total_tokens': 34}

Full response: {"__progress":"received"}{"__progress":"generating","model":"gpt-5.6-luna","provider":"openai"}The capital of France is Paris....
=== WebSocket Subscription Example (Python) ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_py' ===
✓ Subscribed (subscription_id: sub_6beb1c5a47004aff98957934b9106327)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: KSDRvZI9BX-X25KfZugWEUlh-C-3Ri4GhuKkCkV3cs0LbhAcc_gXD3MJVX4XgaNqcsIo_Pz4uA47VaCYm5PJ6Q

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_py
     Record IDs: KSDRvZI9BX-X25KfZugWEUlh-C-3Ri4GhuKkCkV3cs0LbhAcc_gXD3MJVX4XgaNqcsIo_Pz4uA47VaCYm5PJ6Q
     Timestamp:  2026-11-02T04:42:53.099716+00:00

Inserting record 2...
✓ Inserted: bnz0mQifvzArzt5AzIqNTklrePfrvvDGt-vyd0j-lKApaU8QpgX5FsrAwO7tuUJcIi5bak9Qmcrsv7wa7EPnjA

  📡 Notification received:
     Event:      insert
     Record IDs: bnz0mQifvzArzt5AzIqNTklrePfrvvDGt-vyd0j-lKApaU8QpgX5FsrAwO7tuUJcIi5bak9Qmcrsv7wa7EPnjA

=== Unsubscribing ===
✓ Unsubscribed: {'collection': 'ws_subscribe_example_py', 'found': True, 'unsubscribed': True}

=== Cleanup ===
✓ Deleted collection 'ws_subscribe_example_py'

✓ WebSocket subscription example completed successfully
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: TpUcvz9_LjvXBWOxzanWr4Xb_DJGWg8cPavErWZXokamYtLSp84WtTx0p8FydTpyd4eb9Y3jSFtA62HJIGl7_Q

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: {'id': 'Dppt1xz-VSgZHiW820j83FD9yDKox7mpXLn_KPZBm_DNffLcrTMU4UqznFRBpUzrk_dTFOejnbh5KDxPe0OASQ'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'XxEpgDGA9LjyurhAZl8uLj7W5J2OVVfi7NR8p9AwVSVpFTQajeElk2iNhIZEJxdM-Kjmwyn3r6SLsfAKuCJrLg'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'price': {'type': 'Integer', 'value': 150}, 'id': 'Dppt1xz-VSgZHiW820j83FD9yDKox7mpXLn_KPZBm_DNffLcrTMU4UqznFRBpUzrk_dTFOejnbh5KDxPe0OASQ', 'name': {'type': 'String', 'value': 'Product 1'}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'name': {'type': 'String', 'value': 'Upsert Product'}, 'id': 'Dppt1xz-VSgZHiW820j83FD9yDKox7mpXLn_KPZBm_DNffLcrTMU4UqznFRBpUzrk_dTFOejnbh5KDxPe0OASQ', 'price': {'type': 'Integer', 'value': 500}}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ['id', 'name', 'email']
  First user: {'type': 'String', 'value': 'Alice Johnson'} <{'type': 'String', 'value': 'alice@example.com'}>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ['age', 'status', 'avatar_url', 'id', 'created_at', 'user_role', 'name', 'bio', 'email']

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - {'value': 'Dave Brown', 'type': 'String'} (age {'type': 'Integer', 'value': 45})
    - {'value': 'Alice Johnson', 'type': 'String'} (age {'type': 'Integer', 'value': 30})
    - {'value': 'Bob Smith', 'type': 'String'} (age {'value': 25, 'type': 'Integer'})

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - {'type': 'String', 'value': 'Carol White'}: {'type': 'String', 'value': 'Manager'}

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ['avatar_url', 'created_at', 'name', 'bio', 'password', 'email', 'id', 'user_role', 'age', 'secret_token', 'status', 'api_key']
  Projected query:
    - 3 fields per record
    - Fields: ['id', 'name', 'email']
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
✅ All Python integration tests complete!
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
Inserted: map[id:EQYuMW-fPjk12aS7b71aqGSFGLm3w6yNrnUq3FUdzRfb6hGM8rxr2vqFHwr6Z-L8YGW1Ntk1TdqO6TQOQTyFyA]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:EQYuMW-fPjk12aS7b71aqGSFGLm3w6yNrnUq3FUdzRfb6hGM8rxr2vqFHwr6Z-L8YGW1Ntk1TdqO6TQOQTyFyA name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found documents: [map[active:map[type:Boolean value:true] id:EQYuMW-fPjk12aS7b71aqGSFGLm3w6yNrnUq3FUdzRfb6hGM8rxr2vqFHwr6Z-L8YGW1Ntk1TdqO6TQOQTyFyA name:map[type:String value:Test Record] value:map[type:Integer value:42]]]

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:EQYuMW-fPjk12aS7b71aqGSFGLm3w6yNrnUq3FUdzRfb6hGM8rxr2vqFHwr6Z-L8YGW1Ntk1TdqO6TQOQTyFyA name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ simple_crud.go completed successfully

=== Running simple_websocket.go ===
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: 2607uzqLiNgA3EHl9LyxK5NVUvt89nwvEaMhDMaFrKA8Xtv_FEmk9dHI0YXFL8zCkfVGau0Imseo6D8hCx8Wpw

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1793594576329357000",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "2607uzqLiNgA3EHl9LyxK5NVUvt89nwvEaMhDMaFrKA8Xtv_FEmk9dHI0YXFL8zCkfVGau0Imseo6D8hCx8Wpw",
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

✅ Function saved: xrMp58xkPuYGtRIKVF1nQ0TnWneUvYaxP_VhgeXqf3JlC_5eJtwK8RrMBqYZhNvN3iFBAXDN4iHKpDClCGHfkQ
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: 9O02ILb4zwDad4TWwGP9HpiOURQClrQ_Z_um8UVstrZeZ3Jr8QfQ9SrGHUKMDdcNu7oGwh0f4g44aOn8pY-CiQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: XMxsUfkmeYeXFhgtKLd2-sYnVu0bF54w_HIH9c3qEEvMXeITzuqH9OWqJ7VSPZUnbAGhgAaztWEKU2WbMSRfnA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":{"type":"Float","value":50},"count":{"type":"Integer","value":5},"max_score":{"type":"Integer","value":90},"status":{"type":"String","value":"inactive"}}
   {"avg_score":{"type":"Float","value":60},"count":{"type":"Integer","value":5},"max_score":{"type":"Integer","value":100},"status":{"type":"String","value":"active"}}

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
Collection created with first record: 5vqr15uQm8FVlnRO6PzS9cVszU8MaMmPLuPXR-non5UE9gjzlbdNA2yRQB_1CipLfZmlECwMZc0dVAPja91EtA

=== List Collections ===
Total collections: 13
Sample collections: [audit__ek0_testing chat_raw_completions__ek0_testing functions__ek0_testing chat_turns__ek0_testing chat_goal_templates__ek0_testing]

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ collection_management.go completed successfully

=== Running transactions.go ===
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: hxVdl8WzUV2NLrXarhnIkuA8Dzf9V8YNG_jLmm2mLrCcIrGaa9uKzJgIG0LgXUUFw7iu0yP0X0HbM48UW459ZA
Created Bob: $500 - ID: O3cXHlSrkIf2httdrytWuI7iNu17Afj1FCAXoNlzeJbjBAaPf9PP5ZA0tRRX7Vjpyeu5DiqKWQZRgok_hYJFlg

=== Example 1: Begin Transaction ===
Transaction ID: e0526af9-441e-48ae-a5ae-6a64533238e7

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
New transaction: 153f81ba-66d3-437f-acbc-f1a40c5fe0fc
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
   ✅ Function saved: haB-gObnH_13iOXZn3XPC-XEwhDz28HhMK0kJehVX9QBjevLPHD7YAlC4nAxbxAEdxyekfNRJwnaLJavUpfMDw

2️⃣ Calling function (Insert + Verify)...
   ✅ function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: MKt3g9vW2V2eqmaNle_4huRTBCYLtCyg-t_VaE1oJZgCrkkAkNSeEhARolKPvI_hW0Mp3Y-g9HO1S96ePAR-sQ
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: fHLd6Zi7XFmPFZY3YTrjU3Rc_raM1usPaB-HQGlPJC9AOs4Z8ICpBBXUdY5oRPVdzE4wz5mByr1bLdIcTs3mlA

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
   ✅ Function saved: Y9rAfB4CrLX41gVoEFsMLKALbpt6hl5Z2U30fFj3vtKgDaTqW79ya1hzP__pDKTW7YwgUtIVsLqBghK5573B-Q

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
   ✅ Function saved: GaVNV5d6a9EpN6Jf0ryug_2IooHMnG4LHHmsjs3KXn3kmSGV1yo8YsOoOw7_X3V7mIWPJm2QP1xqKE0dMi5L2w

2️⃣ Calling function (Query + Delete + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: haB-gObnH_13iOXZn3XP...
   ✅ Deleted script: fHLd6Zi7XFmPFZY3YTrj...
   ✅ Deleted script: Y9rAfB4CrLX41gVoEFsM...
   ✅ Deleted script: GaVNV5d6a9EpN6Jf0ryu...
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
  Output: Document ID = HTxC5RSAgimk2vK8Kvz37w0ptZUKaLk0wjAIuHchS9pFbKo8mORgCKycdir-59uCYSw0lVFCyZzEFWeB2FbT2g
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: FindByID(HTxC5RSAgimk2vK8Kvz37w0ptZUKaLk0wjAIuHchS9pFbKo8mORgCKycdir-59uCYSw0lVFCyZzEFWeB2FbT2g)
  Output: Found document with name = map[type:String value:TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: FindByID(HTxC5RSAgimk2vK8Kvz37w0ptZUKaLk0wjAIuHchS9pFbKo8mORgCKycdir-59uCYSw0lVFCyZzEFWeB2FbT2g)
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
  Output: Document ID = y8vM3kLRBsasloFrz2h9BQq4xJ7hS2IAOHExrGEZzaXF8QeYFbwosHHxgQRPiD-bjlxduntVhxoigqCJxXJ13g
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: FindByID(y8vM3kLRBsasloFrz2h9BQq4xJ7hS2IAOHExrGEZzaXF8QeYFbwosHHxgQRPiD-bjlxduntVhxoigqCJxXJ13g)
  Output: Found document with name = map[type:String value:WS TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: FindByID(y8vM3kLRBsasloFrz2h9BQq4xJ7hS2IAOHExrGEZzaXF8QeYFbwosHHxgQRPiD-bjlxduntVhxoigqCJxXJ13g)
  Output: Error (expected) - request failed with status 404: ��error�Record not found (expired)
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
Admin permissions read successfully
=== ekoDB Advanced CRUD Integration Example (Go) ===

--- Inserting base record ---
Inserted record: YiJWtzPQou0LIfr9j6UIySHSGWQo2gWt8w8DVIY7sYTkaOBxcc3o570A8kXdylSWuyLs4Sh42R5BhyyLlgwIng

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
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: Z6u0tGu1Ax9dBIXOvok_rFTWBT02qDG8nGJyAYg7c7v33FtClqf9QZszZFCShutBuhwwGRh_A1vdXZD6jCKJmA

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is **ekoDB** — a high-performance database product priced at **$99**.

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: ekoDB is priced at **99**. The context doesn’t specify a currency.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: VYZ46HVLPGdZjMlU197xC8ac7OxjlLA4B3Qg3pFWEZ8kI2Ib5rf23u0R9m8lUnWYmXFUbCrhVGTFijIhjZkDdA
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: VYZ46HVLPGdZjMlU197xC8ac7OxjlLA4B3Qg3pFWEZ8kI2Ib5rf23u0R9m8lUnWYmXFUbCrhVGTFijIhjZkDdA
✓ Deleted chat session: Z6u0tGu1Ax9dBIXOvok_rFTWBT02qDG8nGJyAYg7c7v33FtClqf9QZszZFCShutBuhwwGRh_A1vdXZD6jCKJmA
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: 3GyADvXNysEYxB-JzwbdOApYnpgdYRm0IQtMRtIvoSMTYUq6I5TX_yjbkQPOOzv2V7RxroTBP9ZXhuX8Kf_6tw

=== Sending Chat Message ===
Message ID: n8AlslRnAxSwVymeBXHE5l_S0zABtaTegX6IFfO7BoVXTF12yk1BJgudTi4sfU68IuVrwn6XnKVH75IGi0UWoA

=== AI Response ===
- **ekoDB** — $99
- **ekoDB Pro** — $299
- **ekoDB Cloud** — $499

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:PW9LwXnAlYZILP3Qyr5tMX0Su3XoFsc_pXgAlvwkVa1FHRtBoLbVbkIIVS5CSwKfyXuZQ8JDOuVNhloche6vVg name:ekoDB Cloud price:499] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:fmrZ4NUI7vwqTjgejmSwsL7uNmhKPdjlW3ZGTQsX1GBW1p-dDkpE6JlcXApYLFkA5xi_0QOzKEkAIEaIjuUi2A name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:hnlE2x-69uIOT42DGBQGQgHO-OO5pTr-ZkR-2VOAiBTNGvDBJY2Z-E3Kf50nmTZXY1TUzzQZ-DzaqkuxRajl-A name:ekoDB price:99] score:0.1111111111111111]

Execution Time: 1438ms

=== Token Usage ===
Prompt tokens: 13717
Completion tokens: 32
Total tokens: 13749

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Message Stream (SSE) Example (Go) ===

Created session: dpxJ2khdMAzimL8fQ3xFSDuj2AKlX3OGX6Tq4IuVA-ON8ri7I6FgMiWoF2A3xJpawxsvior8Go-5fRiaB9MdtA

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"gpt-6-luna","provider":"openai"}**ekoDB** is a database project for storing application data. The name can refer to different products or projects, so the exact features depend on which one you mean. If you share a link or where you encountered it, I can identify that specific ekoDB.

--- Stream complete ---
Message ID: VZkzfRvmJwndyWQ4fKao11Y1p3MYIKULaHXst1tYykSg6kUXBarDddpWiNBDJEJp9CbY24kFX07PPB7R2rSuyw
Execution time: 9273ms
Context window: 128000 tokens

✓ Chat message stream example completed
✓ Client created

=== Get All Chat Models ===
OpenAI models: [gpt-3.5-turbo-0125 gpt-5.4-2026-03-05 gpt-5-nano gpt-5.5-pro gpt-audio-2025-08-28 gpt-4o-mini-transcribe-2025-03-20 gpt-4-turbo-2024-04-09 gpt-realtime-2.1-mini gpt-realtime-2025-08-28 gpt-4 tts-1-hd gpt-realtime-1.5 gpt-realtime gpt-5.2-pro gpt-5-chat-latest gpt-5.4-mini-2026-03-17 babbage-002 gpt-4o-2024-05-13 text-embedding-3-large tts-1 omni-moderation-2024-09-26 gpt-live-transcribe gpt-5.6-luna gpt-5-2025-08-07 gpt-5-mini gpt-5-mini-2025-08-07 omni-moderation-latest o3-mini-2025-01-31 gpt-audio-1.5 gpt-image-1.5 tts-1-1106 gpt-5.5-pro-2026-04-23 gpt-audio-mini-2025-10-06 gpt-5.2-pro-2025-12-11 o4-mini-2025-04-16 gpt-4o-2024-11-20 gpt-5.1 whisper-1 gpt-5-pro gpt-realtime-whisper gpt-realtime-2 gpt-5-search-api gpt-5.5-2026-04-23 gpt-4.1-nano gpt-image-2 gpt-3.5-turbo-16k text-embedding-ada-002 gpt-realtime-2.1 gpt-4.1 gpt-5.5 chat-latest gpt-realtime-translate gpt-5.4-pro-2026-03-05 gpt-4.1-mini gpt-4o-mini gpt-4o-mini-transcribe-2025-12-15 gpt-realtime-mini gpt-image-1-mini gpt-5.6-terra gpt-3.5-turbo-instruct-0914 gpt-4-turbo gpt-4o-2024-08-06 o3 gpt-4o-mini-search-preview-2025-03-11 o1-2024-12-17 gpt-5.1-chat-latest gpt-5-pro-2025-10-06 gpt-5.6-sol gpt-4o-transcribe-diarize gpt-5-codex gpt-4-0613 gpt-5.1-2025-11-13 gpt-4o-search-preview gpt-4o-mini-tts-2025-12-15 gpt-image-1 gpt-audio gpt-5.3-codex gpt-5-search-api-2025-10-14 gpt-5.4-pro gpt-4o-mini-tts gpt-realtime-mini-2025-12-15 gpt-5.1-codex-max sora-2 gpt-4o gpt-4o-mini-2024-07-18 gpt-4.1-2025-04-14 gpt-5 gpt-4o-mini-transcribe gpt-image-2-2026-04-21 gpt-5.2-chat-latest sora-2-pro gpt-3.5-turbo-instruct gpt-transcribe gpt-4.1-nano-2025-04-14 gpt-3.5-turbo-1106 gpt-4o-mini-tts-2025-03-20 gpt-5.2 gpt-5.4-nano-2026-03-17 o1 tts-1-hd-1106 gpt-5.2-codex gpt-5.4-mini o1-pro gpt-5.1-codex o4-mini-deep-research gpt-5.4 gpt-4o-search-preview-2025-03-11 text-embedding-3-small gpt-5-nano-2025-08-07 gpt-4o-transcribe o4-mini gpt-5.1-codex-mini gpt-6-astra gpt-5.3-chat-latest o3-2025-04-16 gpt-5.2-2025-12-11 chatgpt-image-latest gpt-3.5-turbo gpt-image-2.5-sunburst-2026-09-08 gpt-image-2.5-flare davinci-002 gpt-5.4-nano o3-mini o4-mini-deep-research-2025-06-26 gpt-audio-mini-2025-12-15 o1-pro-2025-03-19 gpt-live-1 gpt-image-2.5-sunburst gpt-image-2.5-flare-2026-09-08 gpt-6.1-terra gpt-audio-mini gpt-4.1-mini-2025-04-14 gpt-6-luna gpt-4o-mini-search-preview gpt-6-sol gpt-6.1-sol]
Anthropic models: [claude-haiku-5-5 claude-sonnet-5-5 claude-opus-5-5 claude-fable-5-1 claude-opus-5 claude-sonnet-5 claude-fable-5 claude-opus-4-8 claude-opus-4-7 claude-sonnet-4-6 claude-opus-4-6 claude-opus-4-5-20251101 claude-haiku-4-5-20251001 claude-sonnet-4-5-20250929]
Perplexity models: []
Gemini models: [gemini-2.5-flash gemini-2.5-pro gemini-2.5-flash-preview-tts gemini-2.5-pro-preview-tts gemma-4-26b-a4b-it gemma-4-31b-it gemini-flash-latest gemini-flash-lite-latest gemini-pro-latest gemini-2.5-flash-lite gemini-2.5-flash-image gemini-3-flash-preview gemini-3.1-pro-preview gemini-3.1-pro-preview-customtools gemini-3.1-flash-lite-preview gemini-3.1-flash-lite gemini-3-pro-image-preview gemini-3-pro-image nano-banana-pro-preview gemini-3.1-flash-image-preview gemini-3.1-flash-image gemini-3.1-flash-lite-image gemini-nano-banana-2.1 gemini-3.5-flash gemini-3.5-flash-lite gemini-omni-flash-preview gemini-omni-1.1-flash gemini-3.5-transcribe gemini-3.6-flash gemini-3.7-flash gemini-3.8-flash gemini-4-argon lyria-3-clip-preview lyria-3-pro-preview lyria-3.5 gemini-3.1-flash-tts-preview gemini-3.8-flash-tts gemini-3.8-flash-lite-tts gemini-robotics-er-2-preview gemini-2.5-computer-use-preview-10-2025 antigravity-preview-05-2026 antigravity-preview-09-2026 antigravity-preview-latest deep-research-max-preview-04-2026 deep-research-preview-04-2026 deep-research-pro-preview-12-2025 gemini-embedding-001 gemini-embedding-2-preview gemini-embedding-2 aqa gemini-3.5-transcribe-live gemini-2.5-flash-native-audio-latest gemini-2.5-flash-native-audio-preview-09-2025 gemini-2.5-flash-native-audio-preview-12-2025 gemini-3.1-flash-live-preview gemini-3.8-live gemini-3.8-live-extended-thinking gemini-robotics-er-2-streaming-preview gemini-3.5-live-translate-preview lyria-realtime-exp]
  anthropic: ok 14 models
  gemini: ok 60 models
  openai: ok 136 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get OpenAI Models ===
OpenAI models: [gpt-3.5-turbo-0125 gpt-5.4-2026-03-05 gpt-5-nano gpt-5.5-pro gpt-audio-2025-08-28 gpt-4o-mini-transcribe-2025-03-20 gpt-4-turbo-2024-04-09 gpt-realtime-2.1-mini gpt-realtime-2025-08-28 gpt-4 tts-1-hd gpt-realtime-1.5 gpt-realtime gpt-5.2-pro gpt-5-chat-latest gpt-5.4-mini-2026-03-17 babbage-002 gpt-4o-2024-05-13 text-embedding-3-large tts-1 omni-moderation-2024-09-26 gpt-live-transcribe gpt-5.6-luna gpt-5-2025-08-07 gpt-5-mini gpt-5-mini-2025-08-07 omni-moderation-latest o3-mini-2025-01-31 gpt-audio-1.5 gpt-image-1.5 tts-1-1106 gpt-5.5-pro-2026-04-23 gpt-audio-mini-2025-10-06 gpt-5.2-pro-2025-12-11 o4-mini-2025-04-16 gpt-4o-2024-11-20 gpt-5.1 whisper-1 gpt-5-pro gpt-realtime-whisper gpt-realtime-2 gpt-5-search-api gpt-5.5-2026-04-23 gpt-4.1-nano gpt-image-2 gpt-3.5-turbo-16k text-embedding-ada-002 gpt-realtime-2.1 gpt-4.1 gpt-5.5 chat-latest gpt-realtime-translate gpt-5.4-pro-2026-03-05 gpt-4.1-mini gpt-4o-mini gpt-4o-mini-transcribe-2025-12-15 gpt-realtime-mini gpt-image-1-mini gpt-5.6-terra gpt-3.5-turbo-instruct-0914 gpt-4-turbo gpt-4o-2024-08-06 o3 gpt-4o-mini-search-preview-2025-03-11 o1-2024-12-17 gpt-5.1-chat-latest gpt-5-pro-2025-10-06 gpt-5.6-sol gpt-4o-transcribe-diarize gpt-5-codex gpt-4-0613 gpt-5.1-2025-11-13 gpt-4o-search-preview gpt-4o-mini-tts-2025-12-15 gpt-image-1 gpt-audio gpt-5.3-codex gpt-5-search-api-2025-10-14 gpt-5.4-pro gpt-4o-mini-tts gpt-realtime-mini-2025-12-15 gpt-5.1-codex-max sora-2 gpt-4o gpt-4o-mini-2024-07-18 gpt-4.1-2025-04-14 gpt-5 gpt-4o-mini-transcribe gpt-image-2-2026-04-21 gpt-5.2-chat-latest sora-2-pro gpt-3.5-turbo-instruct gpt-transcribe gpt-4.1-nano-2025-04-14 gpt-3.5-turbo-1106 gpt-4o-mini-tts-2025-03-20 gpt-5.2 gpt-5.4-nano-2026-03-17 o1 tts-1-hd-1106 gpt-5.2-codex gpt-5.4-mini o1-pro gpt-5.1-codex o4-mini-deep-research gpt-5.4 gpt-4o-search-preview-2025-03-11 text-embedding-3-small gpt-5-nano-2025-08-07 gpt-4o-transcribe o4-mini gpt-5.1-codex-mini gpt-6-astra gpt-5.3-chat-latest o3-2025-04-16 gpt-5.2-2025-12-11 chatgpt-image-latest gpt-3.5-turbo gpt-image-2.5-sunburst-2026-09-08 gpt-image-2.5-flare davinci-002 gpt-5.4-nano o3-mini o4-mini-deep-research-2025-06-26 gpt-audio-mini-2025-12-15 o1-pro-2025-03-19 gpt-live-1 gpt-image-2.5-sunburst gpt-image-2.5-flare-2026-09-08 gpt-6.1-terra gpt-audio-mini gpt-4.1-mini-2025-04-14 gpt-6-luna gpt-4o-mini-search-preview gpt-6-sol gpt-6.1-sol]

=== Get Anthropic Models ===
Anthropic models: [claude-haiku-5-5 claude-sonnet-5-5 claude-opus-5-5 claude-fable-5-1 claude-opus-5 claude-sonnet-5 claude-fable-5 claude-opus-4-8 claude-opus-4-7 claude-sonnet-4-6 claude-opus-4-6 claude-opus-4-5-20251101 claude-haiku-4-5-20251001 claude-sonnet-4-5-20250929]

✓ Chat Models API example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: S0D0VTWzlmBKodithumXLurHAO5ffrDKcPn554xzZ23M4dP6ZRjwcil2a61okyxoQJgxer6VQv6WnC97_9m8lA

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is **ekoDB**, described as a high-performance database product. It’s priced at **$99**.

✓ Message 2 sent
  Response: ekoDB is priced at **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: I8MAdV0cMudO3W91DiFsfT-HB2yXrPoOiQJvk-DVJhAlIJB_ckVwvK-SLIXSbJBXbjO65i9pMJ3rPwrnOde_pA
  Parent: S0D0VTWzlmBKodithumXLurHAO5ffrDKcPn554xzZ23M4dP6ZRjwcil2a61okyxoQJgxer6VQv6WnC97_9m8lA

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: I8MAdV0cMudO3W91DiFsfT-HB2yXrPoOiQJvk-DVJhAlIJB_ckVwvK-SLIXSbJBXbjO65i9pMJ3rPwrnOde_pA (Untitled)
  Session 2: S0D0VTWzlmBKodithumXLurHAO5ffrDKcPn554xzZ23M4dP6ZRjwcil2a61okyxoQJgxer6VQv6WnC97_9m8lA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: I8MAdV0cMudO3W91DiFsfT-HB2yXrPoOiQJvk-DVJhAlIJB_ckVwvK-SLIXSbJBXbjO65i9pMJ3rPwrnOde_pA

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: lR-UwRVouVsYSr_AQJyVuDdkY7-zuYR4bRaA4VMMiw7GWfKhdZfIS5fIeQF-LlU_77oXi3YXHrhbPmqKCuosjA

=== List Collections ===
Total collections: 13
Sample collections: [audit__ek0_testing chat_raw_completions__ek0_testing functions__ek0_testing chat_turns__ek0_testing chat_goal_templates__ek0_testing]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
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
=== ekoDB Convenience Methods Example ===

=== Native Map Creation ===
✓ Created record with native map: map[id:U1FYw5EZjKIkCCngtW4JVKkM4ToUO_PCXbrpaOm0Og8IZ-q4omZqyMuvjEUclZB-cOyV2f7vUiLAG9IG6fqp3A]

=== Upsert Operation ===
✓ First upsert (update): map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:U1FYw5EZjKIkCCngtW4JVKkM4ToUO_PCXbrpaOm0Og8IZ-q4omZqyMuvjEUclZB-cOyV2f7vUiLAG9IG6fqp3A name:map[type:String value:Alice Johnson]]
✓ Second upsert (insert): map[id:new-user-1793594636951667000]

=== Find One Operation ===
✓ Found user by email: map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:U1FYw5EZjKIkCCngtW4JVKkM4ToUO_PCXbrpaOm0Og8IZ-q4omZqyMuvjEUclZB-cOyV2f7vUiLAG9IG6fqp3A name:map[type:String value:Alice Johnson]]
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
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: Sgkk_Euqpr7RvVbvxqSRQc5rR7MTWHU_kq0lvNFWE8vghzvZPdx0agd57DSGJfPXAOMlT5PkxshBn22TIyYSgw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: kqE5Mz5c0EshGCepK-_P97WFI1-eK_scj_XiI76TKsHQWk4BKbPRycuxFysIZpT2Ni5C7UuTi4ArgOZU0DLqnA

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
=== ekoDB as Edge Cache - Simple Example ===

Setting up edge cache collection...
✓ Cache entry created

Creating edge cache lookup script...
✓ Edge cache script created: vOzh4GaVFzxvBSHKBlLItEEZWklHwLdLA4R2wj_KkALyUQ_3jHHfw5DJHAK7GBTXjHKvZAHUgj8tbKHG5k516g

Call 1: Cache lookup
Response time: 1ms
Found 1 cached entries

Call 2: Cache lookup (connection warm)
Response time: 1ms
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
   ⏱️  Duration: 59.932333ms
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
   ⏱️  Duration: 2.30475ms
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
   🚀 Cache speedup: 29.5x faster!

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
client_function_contract: ok
🚀 ekoDB Functions Example (Go Client)

✅ Client initialized

🧹 Cleaning up...
✅ Deleted collection
✅ Deleted test scripts

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: FZVLIQaugVCnpDnyHCZHkyS8Cuu5KhkqzFMhNu1z1vpXENw085Y8S2kmG43MrcIc5RZYnmikXr0WAcFAkIuXtw
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
   map[avg_price:map[type:Float value:367] category:map[type:String value:Electronics] count:map[type:Integer value:5]]
   map[avg_price:map[type:Float value:365.6666666666667] category:map[type:String value:Furniture] count:map[type:Integer value:3]]
⏱️  Execution time: 0ms

✅ All advanced script examples finished!
🧹 Cleaning up...
✅ Cleanup complete
🚀 ekoDB Go AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases are useful for searching data by **meaning or similarity**, rather than only by exact keywords or matching fields. Benefits include:

- **Semantic search:** Finds relevant text, images, audio, or other content even when it uses different wording.
- **Fast similarity lookup at scale:** Indexes and approximate-nearest-neighbor algorithms make searches over large embedding collections practical.
- **Metadata filtering:** Combines similarity search with filters such as date, category, or access permissions.
- **AI application support:** Helps retrieve relevant context for retrieval-augmented generation (RAG), recommendations, and personalization.
- **Multimodal search:** Can search across different data types when they’re represented as vectors.

They’re most useful when similarity-based retrieval is central; they don’t replace relational databases for transactional queries or structured data management.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embed script saved
📊 Generated 2 embeddings
   Dimensions: 1536
⏱️  Execution time: 0ms

✅ All AI script examples finished!

💡 This example demonstrates:
   ✅ Chat completions with system/user messages
   ✅ Embedding generation for text
🧹 Cleaning up...
✅ Cleanup complete
🚀 ekoDB Go Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: RzDGgMaPNJIIb4faNv9qXPnyJpuT1tqJD3z0DRcVT-sCq2s-zZGOThu9ruOP_37t6czwUyCnfRrcdMs5lJg1vw
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

✅ All CRUD script examples finished!
🧹 Cleaning up...
✅ Cleanup complete
🚀 ekoDB Go KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: 0MteaWOLkeNSc_ZZpslYXzCMorKxNTr5NXHWenza7-bJOIWCSaLdh4d3VDx4xAhHkpIKA79QSalAhxpyTnJd4g
✅ Inserted 2 products with wrapped types

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: NydKlN7Ht6Q9YFa91Lo-BnNQuw3wGxwNLzevumLw-tQVx3Osk8a1SnwkPduioXAGcSdaBRlqzn85MFt6NuP4iw
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: map[type:Object value:map[role:admin userId:user_abc]]
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session
📝 Example 4: KV Operations in Functions

✅ Function saved: iFpjyhUG1Ve59GL9AVgwS5WpAYihp5CfadIUO1f4hzRD6gdQCcgQ9z_WyyL-yY0rM1IrserASE_ERGwDt1FQwA
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: 5mqYQA-o4hrJxalGOB2EDfgRqGSyc0nerqn5jwDC7Z7lrUBLUbmp_PvlHAL_70JSzhR7bSYjmLJktxAQcubKYg
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

🚀 ekoDB Go Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. map[type:String value:Vector Databases Explained] (map[type:String value:Database])
   2. map[type:String value:Natural Language Processing] (map[type:String value:AI])
   3. map[type:String value:Database Design Principles] (map[type:String value:Database])
   4. map[type:String value:Getting Started with ekoDB] (map[type:String value:Database])
   5. map[type:String value:Introduction to Machine Learning] (map[type:String value:AI])
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   map[category:map[type:String value:AI] count:map[type:Integer value:2]]
   map[category:map[type:String value:Database] count:map[type:Integer value:3]]
⏱️  Execution time: 0ms

✅ All search script examples finished!
🧹 Cleaning up...
✅ Cleanup complete
=== ekoDB Goal Template CRUD Example (Go) ===

--- Creating goal template ---
Created template: Data Migration (id: OfGQSeWDFmFvgjznnQxqd6pzsRZrG7fwXxtVwjZ5p0swKsHoB5SmZCPxRgHAO4VKsGalIaEc4bB2HSU9Lp61fA)

--- Listing templates ---
Templates: map[count:1 items:[map[description:map[type:String value:Template for migrating data between schemas] id:OfGQSeWDFmFvgjznnQxqd6pzsRZrG7fwXxtVwjZ5p0swKsHoB5SmZCPxRgHAO4VKsGalIaEc4bB2HSU9Lp61fA owner:map[type:Object value:map[key_id:3a0cdba0da7c060610042783cc758f1f key_label:default_admin]] steps:map[type:Array value:[map[description:Analyze source schema] map[description:Create target schema] map[description:Migrate records] map[description:Validate results]]] title:map[type:String value:Data Migration]]] total:1]

--- Getting template ---
Fetched: Data Migration

--- Updating template ---
Updated description: Updated: comprehensive data migration workflow

--- Deleting template ---
Template deleted successfully

✓ Goal template CRUD example completed
=== ekoDB Goals, Tasks & Agents Integration Example (Go) ===

--- Creating goal ---
Created goal: Deploy v2.0 (id: 3a24Hs4sj2uVhws7n7Q3tJIArDnDhudpqjJD549jtQSvwjwlkpv1EJ6zWhkO4c7g4mTmBqb0wqcmdHzrTjxSFg)

--- Listing goals ---
Goals response: map[count:1 goals:[map[created_at:map[type:DateTime value:2026-11-02T04:44:05.340516+00:00] description:map[type:String value:Ship the v2.0 release to production] id:3a24Hs4sj2uVhws7n7Q3tJIArDnDhudpqjJD549jtQSvwjwlkpv1EJ6zWhkO4c7g4mTmBqb0wqcmdHzrTjxSFg owner:map[type:Object value:map[key_id:3a0cdba0da7c060610042783cc758f1f key_label:default_admin]] status:map[type:String value:pending] steps:map[type:String value:[{"description":"Run integration tests"},{"description":"Build release artifacts"},{"description":"Deploy to staging"}]] title:map[type:String value:Deploy v2.0] updated_at:map[type:DateTime value:2026-11-02T04:44:05.340516+00:00]]] total:1]

--- Getting goal ---
Fetched goal: Deploy v2.0

--- Updating goal ---
Updated description: Ship the v2.0 release to production (with hotfix)

--- Searching goals ---
Search result: map[count:1 items:[map[_score:12.870000000000001 created_at:map[type:DateTime value:2026-11-02T04:44:05.340516+00:00] description:map[type:String value:Ship the v2.0 release to production (with hotfix)] id:3a24Hs4sj2uVhws7n7Q3tJIArDnDhudpqjJD549jtQSvwjwlkpv1EJ6zWhkO4c7g4mTmBqb0wqcmdHzrTjxSFg owner:map[type:Object value:map[key_id:3a0cdba0da7c060610042783cc758f1f key_label:default_admin]] status:map[type:String value:pending] steps:map[type:String value:[{"description":"Run integration tests"},{"description":"Build release artifacts"},{"description":"Deploy to staging"}]] title:map[type:String value:Deploy v2.0] updated_at:map[type:DateTime value:2026-11-02T04:44:05.350904+00:00]]]]

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
Created task: Run benchmarks (id: ZR734MorV7nDFuKbeapDHNKnJzecpjDSkaxqNNf2RQ8bTN0ZXzTZgaCQDY0jvrTP0NBqDr_3NmxFTQ0jqsxJ7g)

--- Listing tasks ---
Tasks: map[count:1 items:[map[description:map[type:String value:Execute YCSB benchmarks against staging] due_at:map[type:DateTime value:2026-11-03T04:44:05+00:00] id:ZR734MorV7nDFuKbeapDHNKnJzecpjDSkaxqNNf2RQ8bTN0ZXzTZgaCQDY0jvrTP0NBqDr_3NmxFTQ0jqsxJ7g name:map[type:String value:Run benchmarks] owner:map[type:Object value:map[key_id:3a0cdba0da7c060610042783cc758f1f key_label:default_admin]]]] total:1]

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
Created agent: benchmark-runner (id: isSsFdf3IlQUQou3k6zOfe9ITJbECnClFkJnPckWBRCrYMconXznmOlb6u045nXd-Vcq3ITpjZ_GoSQAuz2Fjw)

--- Listing agents ---
Agents: map[count:1 items:[map[deployment_id:map[type:String value:deploy_prod_1] description:map[type:String value:Runs periodic benchmarks] id:isSsFdf3IlQUQou3k6zOfe9ITJbECnClFkJnPckWBRCrYMconXznmOlb6u045nXd-Vcq3ITpjZ_GoSQAuz2Fjw llm_model:map[type:String value:gpt-6-luna] name:map[type:String value:benchmark-runner] owner:map[type:Object value:map[key_id:3a0cdba0da7c060610042783cc758f1f key_label:default_admin]] run_policy:map[type:String value:{"trigger":{"allow_write_operations":false,"write_tools":[]},"eval":{"allow_write_operations":false,"write_tools":[]},"customer":{"allow_write_operations":false,"write_tools":[]}}]]] total:1]

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
=== ekoDB KV Links Integration Example (Go) ===

--- Inserting documents ---
Inserted doc1: HUgR484VcS7aSlpYBG3b8BijCx9QL4p7lP1MluSyPffLf9Tey5yaogkHLQ_b03lN3Ltves3T91z1E2yFYb8IDw
Inserted doc2: 0A40WuFJPoYjCv6w2epcSWBE33Yg1OC_cHPQK6WbidvNLJ79670auG-SCzHbooTFUfdh4hbZ9ZajjvUUDXlgSQ

--- Setting KV entry ---
KV entry set: team:engineering:go

--- Linking KV key to documents ---
Linked doc1: map[]
Linked doc2: map[]

--- Getting links ---
Links for team:engineering:go: [map[collection:kv_links_example_go created_at:2026-11-02T04:44:06.156809Z document_id:HUgR484VcS7aSlpYBG3b8BijCx9QL4p7lP1MluSyPffLf9Tey5yaogkHLQ_b03lN3Ltves3T91z1E2yFYb8IDw field_path:<nil> last_accessed:2026-11-02T04:44:06.158108Z metadata:map[]] map[collection:kv_links_example_go created_at:2026-11-02T04:44:06.157540Z document_id:0A40WuFJPoYjCv6w2epcSWBE33Yg1OC_cHPQK6WbidvNLJ79670auG-SCzHbooTFUfdh4hbZ9ZajjvUUDXlgSQ field_path:<nil> last_accessed:2026-11-02T04:44:06.158108Z metadata:map[]]]

--- Unlinking doc2 ---
Unlink result: map[]

--- Verifying links after unlink ---
Links after unlink: [map[collection:kv_links_example_go created_at:2026-11-02T04:44:06.156809Z document_id:HUgR484VcS7aSlpYBG3b8BijCx9QL4p7lP1MluSyPffLf9Tey5yaogkHLQ_b03lN3Ltves3T91z1E2yFYb8IDw field_path:<nil> last_accessed:2026-11-02T04:44:06.159392Z metadata:map[]]]

--- Cleanup ---
Cleaned up all resources

=== All KV link operations completed successfully ===
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
=== ekoDB Raw Completion Stream (SSE) Example ===

--- Basic SSE Raw Completion ---
Response: For traditional painting, the primary colors are **red, yellow, and blue**. For light and screens, they’re **red, green, and blue**.

--- Structured Output via SSE ---
JSON response: [
  {"name": "Earth", "diameter_km": 12742},
  {"name": "Mars", "diameter_km": 6779},
  {"name": "Venus", "diameter_km": 12104}
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello!

=== Done ===
=== ekoDB Schedules Integration Example (Go) ===

--- Creating schedule ---
Created schedule: nightly-backup (id: 81ac37ec-63c9-41f9-b479-2ab614e8951c)

--- Listing schedules ---
Schedules: map[count:1 schedules:[map[created_at:2026-11-02T04:44:11.674728Z cron_expression:0 0 0 * * * description:Runs a database backup every night at midnight enabled:true function_label:schedule_noop_go_41786 id:81ac37ec-63c9-41f9-b479-2ab614e8951c last_execution:<nil> name:nightly-backup next_execution:2026-11-03T00:00:00Z parameters:map[] stats:map[avg_execution_time_ms:0 failed_executions:0 last_error:<nil> successful_executions:0 total_executions:0] timezone:UTC updated_at:2026-11-02T04:44:11.674728Z]]]

--- Getting schedule ---
Fetched schedule: nightly-backup (cron: 0 0 0 * * *)

--- Updating schedule ---
Updated cron: 0 0 2 * * *

--- Triggering schedule ---
Trigger response: map[schedule_id:81ac37ec-63c9-41f9-b479-2ab614e8951c status:triggered]

--- Pausing schedule ---
Schedule enabled: false

--- Resuming schedule ---
Schedule enabled: true

--- Deleting schedule ---
Schedule deleted

=== All schedule operations completed successfully ===
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
  1. Score: 0.750
  2. Score: 0.749
  3. Score: 0.745

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.500
  2. Score: 0.900
  3. Score: 0.298

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200

9. Vector search with a metadata pre-filter (category = ml):
Found 2 documents in category "ml" (NLP excluded)
  1. Introduction to Machine Learning (category: ml)
  2. Deep Learning Fundamentals (category: ml)

Paired vector search: document=Rust Programming Answer, query=Rust Programming Question
Document search execution path: index
Query search execution path: index

✅ Search examples completed!
=== Cleanup ===
✅ Deleted test collections
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: map[id:8OMhIHYOBgKbId93b9vSWAnVmsiFp2kw4dKF32bT0Bf1AcdeCEEVUF2JDVwgxKycZ9wfn19_RSvFIjfYjX066Q]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-11-02T04:44:12+00:00] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:8OMhIHYOBgKbId93b9vSWAnVmsiFp2kw4dKF32bT0Bf1AcdeCEEVUF2JDVwgxKycZ9wfn19_RSvFIjfYjX066Q metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Test Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:42]]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-11-02 04:44:12 +0000 +0000
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-11-02T04:44:12+00:00 data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:8OMhIHYOBgKbId93b9vSWAnVmsiFp2kw4dKF32bT0Bf1AcdeCEEVUF2JDVwgxKycZ9wfn19_RSvFIjfYjX066Q metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-11-02T04:44:12+00:00] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:8OMhIHYOBgKbId93b9vSWAnVmsiFp2kw4dKF32bT0Bf1AcdeCEEVUF2JDVwgxKycZ9wfn19_RSvFIjfYjX066Q metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: pLvVNJfXuCqClt-nLlfgXq1Aa3BtHTErXjSNJJ9zX8kV9OcIXu70qti1hkbTMDbQxCENsdW4-5lpCcOUrN6mbQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
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
✓ Created native SWR script: github_user_native_go (2JpYVYFX5PACspq21Saq4XP2C8voxBYsEzsPK7BDKtz-BG6-rtVGyAN03g0zuM70xcyuaI6BOnJr4FJv9DwXHg)

First call (cache miss - will fetch from GitHub API):
  Response time: 149ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 2ms
  Speedup: 74.5x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_go (oz0UFUTYL2_ltdDB_K9EgcQQ8OZgyqdTeST6y94tcwKl0vZxABItYyhMAILL1lHF_xS_YuKTVKB1lmQ-YtQJbw)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_go' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_go (_TVP5kmP6ha1sJJ74ZBIJlQXeP1PyJJJ1R6WoaPV3eNM3RC4dSojcrhyXMHfHt-i2ha4sPz-G_EAARq4vFi3HQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_go' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_go (p6BwTW2lP1QBIu210KBURF1wCBGrM3oV6F3ScbT2CnJ-wOfhCJ3Sgh6pUuX4k8lL82CCx4qODHRBj60tvr2RtQ)
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
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_go (EteSWU9UkvgQc9Vd39diI4CT6rPdDJCt_H39Kcl3Sm9xG9TZn_56-VceYhXz2Ym8QnEaKbgOqaxj9KRjifjScA)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "cached_at": {
        "type": "DateTime",
        "value": "2026-11-02T04:44:13+00:00"
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

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
🧹 Cleaning up...
✓ Cleanup complete

✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: _LSmNIPn8F9rp1JMeXqfqcoIqscFcRxqbJAmygmW6bdxsdAuE3WrqGeuNByMowzKyjLWwvps1ls2Dg0JAQRTVA
Created Bob: $500 - ID: J8F4Lq17uUYGMX0DJdMEJxmmztITbnGQqP3CFEpY-QdCtm5eXUkWWOVXcEoe49jCg1qLTDdw5n_M_rMXvDskgw

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): ecce0d48-fa38-4bac-9f38-dd788ca9bd99

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
New transaction: 8ebcdc4e-22a0-4fc1-93ce-38ba8fbb0af7
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Create User Function ===
Created user function with ID: UzFNtN-USr8z5VQ0g9Qmpk7EoLARZzfjPnakgO2OO7xYrliLjmWKcC-gIWfvm7oExeNcowPtLgTjeMnYxLHlkQ

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
=== WebSocket Chat Streaming Example (Go) ===

Created chat session: mFA_KK8USO_mudkJ4uhDwedS0T0eTOQI2jXqL1jmTD72r5cPn-5D2g_k5xY2Zx6R8bnbgJ5Pz5qZMxY3nJQkgQ

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"gpt-5.6-luna","provider":"openai"}The capital of France is **Paris**.

--- Stream ended ---
Message ID: O_IZRCia9mllJoaNsjIjq6imEpf5nR81L3glqTDe3TUNO6O8pfe_5lrncIxtg_cIHeVXDnw_TG3L6Tv03O2Dqw
Execution time: 905ms
Token usage: {"cached_input_tokens":0,"completion_tokens":13,"prompt_tokens":23,"thinking_tokens":0,"total_tokens":36}

Full response: {"__progress":"received"}{"__progress":"generating","model":"gpt-5.6-luna","provider":"openai"}The capital of France is **Paris**....
=== WebSocket Subscription Example (Go) ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_go' ===
✓ Subscribed (subscription_id: sub_3363d3230a2b47ea9067b8f520d38ca9)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: 58J8ThlZkftLA2CXMV1vHa0yP-oPSJCDZXxx4Wu6ChDlQjH0JKBgRf5IWkxrVSJA9lhm-xeJ8f9C5Iaefv4UUA

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_go
     Record IDs: [58J8ThlZkftLA2CXMV1vHa0yP-oPSJCDZXxx4Wu6ChDlQjH0JKBgRf5IWkxrVSJA9lhm-xeJ8f9C5Iaefv4UUA]
     Timestamp:  2026-11-02T04:44:15.596856+00:00

Inserting record 2...
✓ Inserted: WPUmFo3xP8k4WJ51chy_6SF4r3b42lehQGufwvL5epDkgVCDDiz-Eqgegi3aGi9SpIeHFnGpavxpC0Hvb1G_XA

  📡 Notification received:
     Event:      insert
     Record IDs: [WPUmFo3xP8k4WJ51chy_6SF4r3b42lehQGufwvL5epDkgVCDDiz-Eqgegi3aGi9SpIeHFnGpavxpC0Hvb1G_XA]

=== Unsubscribing ===
✓ Unsubscribed

✓ WebSocket subscription example completed successfully
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: WgcLd5dLo2U73i96z5tnhBVEEc15-8FTSmmIjIvftLlZAHSWDSDupgpi-Bn8czZ-r0pZHpGRlAzni8bjE9maAg

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
Inserted with ripple: map[id:bjFLMpRjvbIFP2qXQ8s5nuXruqr1KXxeP0fqShBH8Ry5p8jrajDrAEiRtNXhAWlGTMJG1LGP6OEBaTt59UIbGQ]
Inserted with bypass_ripple: map[id:bFKbfdKN4xIKH0J53G4dKQtoGmdDTjDEPpUlcl1ZCQGYaT-sIm8qnKtG9ntX-v5ocsAepG5a1UkJIMg-fKRhJw]
Inserted with TTL and bypass_ripple: map[id:cRioqVuvZiFBlGlKv_QRPpn2HJaVNrhwtiYttMaawkSF0WigqH8XT_DxZNUrz24dN5O373LNkIRtiWAAqcaC1A]
Updated with bypass_ripple: map[id:bjFLMpRjvbIFP2qXQ8s5nuXruqr1KXxeP0fqShBH8Ry5p8jrajDrAEiRtNXhAWlGTMJG1LGP6OEBaTt59UIbGQ name:map[type:String value:Product 1] price:map[type:Integer value:150]]
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
✅ All Go integration tests complete!
📦 Building TypeScript client library...

> @ekodb/ekodb-client@0.31.0 prepare
> npm run build


> @ekodb/ekodb-client@0.31.0 build
> tsc


up to date, audited 44 packages in 4s

12 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
npm warn install-scripts 1 package has install scripts not yet covered by allowScripts:
npm warn install-scripts   fsevents@2.3.3 (install: (install scripts present))
npm warn install-scripts
npm warn install-scripts Run `npm install-scripts ls` to review, or `npm install-scripts approve <pkg>` to allow.

> @ekodb/ekodb-client@0.31.0 build
> tsc

✅ TypeScript client built!

added 1 package, removed 1 package, and audited 13 packages in 775ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
npm warn install-scripts 3 packages have install scripts not yet covered by allowScripts:
npm warn install-scripts   ekodb-client-ts@0.31.0 (prepare: npm run build)
npm warn install-scripts   esbuild@0.28.1 (postinstall: node install.js)
npm warn install-scripts   fsevents@2.3.3 (install: (install scripts present))
npm warn install-scripts
npm warn install-scripts Run `npm install-scripts ls` to review, or `npm install-scripts approve <pkg>` to allow.
Admin permissions read successfully
=== ekoDB Advanced CRUD Example (TypeScript) ===

--- Inserting base record ---
Inserted: LdwdpHFJ_kcTQfr95jA_wFiSfrivQcjbkbCawSMh_1Qzol9UH1w5G7HshLjflwRKnIy4zfNxMMV2Mab6oL35nw

--- updateWithAction: increment ---
Score after increment: 150

--- updateWithAction: push ---
Tags after push: ["beginner","pro"]

--- updateWithAction: clear ---
temp_data after clear: null

--- updateWithActionSequence: multiple atomic actions ---
After sequence - score: 160, lives: 2, tags: ["beginner","pro","veteran"]

--- Cleanup ---
Deleted collection

=== All advanced CRUD operations completed ===
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
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: -bamzbsvq9mYP1AJIK4CT7C1ehFJOcaV2A4uF6dYvQo7G6XvX3Letr7UDU-GvQt_1Dz-6V73NxUk5ThZbTcKPg

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is **ekoDB** — a high-performance database product priced at **$99**.

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: ekoDB costs **$99**.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: xCgy5lVNs_khzmvt3q5g0B5Wc04kyl9ePI2KBi9NKvwVg8ytlJjYk-fsq34eHGwEigTOkVhOeREUT2p9h2Bj2g
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: xCgy5lVNs_khzmvt3q5g0B5Wc04kyl9ePI2KBi9NKvwVg8ytlJjYk-fsq34eHGwEigTOkVhOeREUT2p9h2Bj2g
✓ Deleted chat session: -bamzbsvq9mYP1AJIK4CT7C1ehFJOcaV2A4uF6dYvQo7G6XvX3Letr7UDU-GvQt_1Dz-6V73NxUk5ThZbTcKPg
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: H9u0DwaTH9WKlMDqxnjxFSlb0hqtel2MEq4xcELm309islmv9Sy0GnwIpEZpbXXl8MLpbDa0MOxtP2vH3rxzKA

=== Sending Chat Message ===
Message ID: lah_oQSmqvKWlpip6bYIe3nUs3ZoieF0_qH7SNr2Z5QVZiofCf6AOpmlJ_GSXn3aaPVqWHkkuNohcJMpZo2mig

=== AI Response ===
Available products and prices:

- **ekoDB Cloud** — $499
- **ekoDB Pro** — $299
- **ekoDB** — $99

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 499,
    id: '0hBVp-ywjwj3Ibio9TXwTR_vUQF94mnrnlYMUzBXq06AAhjjnlT-0lOG1-7gUfZUu3pczqb0zACndnij85vXQw',
    name: 'ekoDB Cloud',
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB Pro',
    price: 299,
    id: 'XEaYjWivy1e4F0fqo06p4nqPydXVUI_t0puRlgmdeCqCa0meT49zAQ9x6qlRRFFdNsYOU6IN4kPlJkvKBl1t5Q',
    description: 'Enterprise edition product with advanced features'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 99,
    description: 'A high-performance database product with AI capabilities',
    name: 'ekoDB',
    id: 'nolUf21a90sXyvGPgV40dTMX0QQMapPjjKONLBJGumlkZ7KNrrAUXEQdtIHAPwBzprPnX0de9aonV7E0pNQNfg'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 1499ms

=== Token Usage ===
Prompt tokens: 13709
Completion tokens: 37
Total tokens: 13746

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Message Stream (SSE) Example (TypeScript) ===

Created session: mDNCcEl4X7WA6julR2V12sTK4uyJY3JRDguWSD1fE-T6A-HhQwP2eWHYJ7NIaMWFOdeBjx1skhrf-9tWcDobbQ

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"gpt-6-luna","provider":"openai"}**ekoDB** is a real-time database service for application data. It lets apps store and retrieve structured data, and can push updates to connected clients as data changes—similar in broad purpose to Firebase. If you mean a particular ekoDB project or product, share a link and I can describe that specific one.

--- Stream complete ---
Message ID: 8TmNLQ2yVw1JScSDHypo_1ghO2yuXWEHb2jPML0fEQ6cpZocnjRc-9hH5Ggiv4fz2MEO2NXyZbLAoWJOolHsYg
Execution time: 7664ms
Context window: 128000 tokens

✓ Chat message stream example completed
✓ Client created

=== List Chat Models ===
Available chat models by provider:
  openai:
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
  anthropic:
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
  gemini:
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

=== Get Specific Provider Models ===
OpenAI models: gpt-3.5-turbo-0125, gpt-5.4-2026-03-05, gpt-5-nano, gpt-5.5-pro, gpt-audio-2025-08-28, gpt-4o-mini-transcribe-2025-03-20, gpt-4-turbo-2024-04-09, gpt-realtime-2.1-mini, gpt-realtime-2025-08-28, gpt-4, tts-1-hd, gpt-realtime-1.5, gpt-realtime, gpt-5.2-pro, gpt-5-chat-latest, gpt-5.4-mini-2026-03-17, babbage-002, gpt-4o-2024-05-13, text-embedding-3-large, tts-1, omni-moderation-2024-09-26, gpt-live-transcribe, gpt-5.6-luna, gpt-5-2025-08-07, gpt-5-mini, gpt-5-mini-2025-08-07, omni-moderation-latest, o3-mini-2025-01-31, gpt-audio-1.5, gpt-image-1.5, tts-1-1106, gpt-5.5-pro-2026-04-23, gpt-audio-mini-2025-10-06, gpt-5.2-pro-2025-12-11, o4-mini-2025-04-16, gpt-4o-2024-11-20, gpt-5.1, whisper-1, gpt-5-pro, gpt-realtime-whisper, gpt-realtime-2, gpt-5-search-api, gpt-5.5-2026-04-23, gpt-4.1-nano, gpt-image-2, gpt-3.5-turbo-16k, text-embedding-ada-002, gpt-realtime-2.1, gpt-4.1, gpt-5.5, chat-latest, gpt-realtime-translate, gpt-5.4-pro-2026-03-05, gpt-4.1-mini, gpt-4o-mini, gpt-4o-mini-transcribe-2025-12-15, gpt-realtime-mini, gpt-image-1-mini, gpt-5.6-terra, gpt-3.5-turbo-instruct-0914, gpt-4-turbo, gpt-4o-2024-08-06, o3, gpt-4o-mini-search-preview-2025-03-11, o1-2024-12-17, gpt-5.1-chat-latest, gpt-5-pro-2025-10-06, gpt-5.6-sol, gpt-4o-transcribe-diarize, gpt-5-codex, gpt-4-0613, gpt-5.1-2025-11-13, gpt-4o-search-preview, gpt-4o-mini-tts-2025-12-15, gpt-image-1, gpt-audio, gpt-5.3-codex, gpt-5-search-api-2025-10-14, gpt-5.4-pro, gpt-4o-mini-tts, gpt-realtime-mini-2025-12-15, gpt-5.1-codex-max, sora-2, gpt-4o, gpt-4o-mini-2024-07-18, gpt-4.1-2025-04-14, gpt-5, gpt-4o-mini-transcribe, gpt-image-2-2026-04-21, gpt-5.2-chat-latest, sora-2-pro, gpt-3.5-turbo-instruct, gpt-transcribe, gpt-4.1-nano-2025-04-14, gpt-3.5-turbo-1106, gpt-4o-mini-tts-2025-03-20, gpt-5.2, gpt-5.4-nano-2026-03-17, o1, tts-1-hd-1106, gpt-5.2-codex, gpt-5.4-mini, o1-pro, gpt-5.1-codex, o4-mini-deep-research, gpt-5.4, gpt-4o-search-preview-2025-03-11, text-embedding-3-small, gpt-5-nano-2025-08-07, gpt-4o-transcribe, o4-mini, gpt-5.1-codex-mini, gpt-6-astra, gpt-5.3-chat-latest, o3-2025-04-16, gpt-5.2-2025-12-11, chatgpt-image-latest, gpt-3.5-turbo, gpt-image-2.5-sunburst-2026-09-08, gpt-image-2.5-flare, davinci-002, gpt-5.4-nano, o3-mini, o4-mini-deep-research-2025-06-26, gpt-audio-mini-2025-12-15, o1-pro-2025-03-19, gpt-live-1, gpt-image-2.5-sunburst, gpt-image-2.5-flare-2026-09-08, gpt-6.1-terra, gpt-audio-mini, gpt-4.1-mini-2025-04-14, gpt-6-luna, gpt-4o-mini-search-preview, gpt-6-sol, gpt-6.1-sol

=== Get Anthropic Models ===
Anthropic models: claude-haiku-5-5, claude-sonnet-5-5, claude-opus-5-5, claude-fable-5-1, claude-opus-5, claude-sonnet-5, claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 404: {"error":"Unknown provider 'nonexistent_provider_xyz'","error_kind":"unknown_provider"}

✓ Chat Models example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 6QRiTp_aLu1OqYVSODrcUM9RhDYQzz9huCs05MPYqCadsVWr4jl4abmlp7A-WvNhcGnDAevGoY0jpWsDGh9UCw

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is **ekoDB**, described as a high-performance database product. It’s priced at **$99**.

✓ Message 2 sent
  Response: ekoDB is priced at **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: ksGWasNLmTUOMeVXNgpCDaTvG1yZMGM07VwfX9tqYAR8CmsOslOeoBBgM2tGorsWfbfJT20PUkUtEYovTVWV5w
  Parent: 6QRiTp_aLu1OqYVSODrcUM9RhDYQzz9huCs05MPYqCadsVWr4jl4abmlp7A-WvNhcGnDAevGoY0jpWsDGh9UCw

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: ksGWasNLmTUOMeVXNgpCDaTvG1yZMGM07VwfX9tqYAR8CmsOslOeoBBgM2tGorsWfbfJT20PUkUtEYovTVWV5w (Untitled)
  Session 2: 6QRiTp_aLu1OqYVSODrcUM9RhDYQzz9huCs05MPYqCadsVWr4jl4abmlp7A-WvNhcGnDAevGoY0jpWsDGh9UCw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: ksGWasNLmTUOMeVXNgpCDaTvG1yZMGM07VwfX9tqYAR8CmsOslOeoBBgM2tGorsWfbfJT20PUkUtEYovTVWV5w

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: 9peNqSrynzGsZ6GJz8xWAe1iPvStDUTcDssIDIGtyV383d9zy5oxH7L_GRexo2ZoSP6e7M_Y41I-pz_IY9-Bzw

=== List Collections ===
Total collections: 17
Sample collections: schema_users_client_go,audit__ek0_testing,schema_employees_client_go,chat_raw_completions__ek0_testing,schema_documents_client_go

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
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
✓ conc_demo_pay saved
✓ conc_demo_rl_fail saved
✓ conc_demo_rl_skip saved
✓ conc_demo_lock saved

Invoke them like:
  POST /api/functions/conc_demo_pay_ts_50152_1793594691316 { "idempotency_key": "...", "amount": 100 }
  POST /api/functions/conc_demo_rl_fail_ts_50152_1793594691316 { "user_id": 42 }
  POST /api/functions/conc_demo_rl_skip_ts_50152_1793594691316 { "user_id": 42 }
  POST /api/functions/conc_demo_lock_ts_50152_1793594691316 { "resource": "queue:drain" }

✓ Cleaned up demo functions
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'lqRwO-8QBT04esMqoefLPfapy20CkMGjF9cqZNqaeEO0JiqabAQ-gCaBu-JeS_ao2SSOuaQTawIvjiyHnbABsg'
}

=== Upsert Operation ===
✓ First upsert (update): {
  email: { value: 'alice.j@newdomain.com', type: 'String' },
  name: { value: 'Alice Johnson', type: 'String' },
  id: 'lqRwO-8QBT04esMqoefLPfapy20CkMGjF9cqZNqaeEO0JiqabAQ-gCaBu-JeS_ao2SSOuaQTawIvjiyHnbABsg',
  active: { type: 'Boolean', value: true },
  age: { type: 'Integer', value: 29 }
}
✓ Second upsert (insert): { id: 'new-user-id' }

=== Find One Operation ===
✓ Found user by email: {
  id: 'lqRwO-8QBT04esMqoefLPfapy20CkMGjF9cqZNqaeEO0JiqabAQ-gCaBu-JeS_ao2SSOuaQTawIvjiyHnbABsg',
  email: { value: 'alice.j@newdomain.com', type: 'String' },
  name: { type: 'String', value: 'Alice Johnson' },
  active: { type: 'Boolean', value: true },
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
✓ Client created
✓ crypto_demo_hmac_ts saved
✓ crypto_demo_aes_ts saved
✓ crypto_demo_uuid_ts saved
✓ crypto_demo_totp_ts saved
✓ crypto_demo_encoding_ts saved

Invoke them with:
  POST /api/functions/crypto_demo_hmac_ts { "payload": "hi" }
  POST /api/functions/crypto_demo_aes_ts { "plaintext": "secret" }
  POST /api/functions/crypto_demo_uuid_ts
  POST /api/functions/crypto_demo_totp_ts
  POST /api/functions/crypto_demo_encoding_ts { "title": "Héllo World" }

✓ Cleaned up demo functions
=== Distinct Values Example ===

Inserting sample products...
Inserted 8 products

=== Distinct Categories (all products) ===
Found 3 distinct categories:
  - books
  - clothing
  - electronics

=== Distinct Statuses (all products) ===
Found 3 distinct statuses:
  - active
  - archived
  - discontinued

=== Distinct Statuses in Electronics ===
Found 2 distinct statuses for electronics:
  - active
  - discontinued

Cleanup done.
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: IcSKhKmWtB8q-sMc8gvkBFch5g6h6SCIOyWQemTNTGuhhy5Zt-A0Zaos7v0qU5xJwo9JZyOen53h0sMV13A4uA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: e3ECXs9m5MrwNz_Dmr4QGJ6bpTgw7GFTnPbAPNfH5d78yUbHd5xv0dEWg5Fg3EYAwzVnY3yOs2D4o7b_umvnUA

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
=== ekoDB as Edge Cache - Simple Example ===

Creating edge cache function...
✓ Edge cache script created: wbWdV7SVn2ZVH730TCN0r4Lg-yl2Nc54sdG_-1v3pZAMYnlkMer-1mpqQoh0Nqn-jCBf2WHUDdfCE2l6BCgIeA

Call 1: Cache miss (fetches from API)
Response time: 76ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "username": "Bret",
          "email": "Sincere@april.biz",
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "phone": "1-770-736-8031 x56442",
          "id": 1,
          "website": "hildegard.org",
          "address": {
            "suite": "Apt. 556",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "zipcode": "92998-3874",
            "street": "Kulas Light",
            "city": "Gwenborough"
          },
          "name": "Leanne Graham"
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
Response time: 4ms (19x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "username": "Bret",
          "email": "Sincere@april.biz",
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "phone": "1-770-736-8031 x56442",
          "id": 1,
          "website": "hildegard.org",
          "address": {
            "suite": "Apt. 556",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "zipcode": "92998-3874",
            "street": "Kulas Light",
            "city": "Gwenborough"
          },
          "name": "Leanne Graham"
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
   ⏱️  Duration: 58ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "website": "hildegard.org",
      "username": "Bret",
      "address": {
        "zipcode": "92998-3874",
        "geo": {
          "lat": "-...

Second call (cache hit - from cache):
   ⏱️  Duration: 3ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "website": "hildegard.org",
      "username": "Bret",
      "address": {
        "zipcode": "92998-3874",
        "geo": {
          "lat": "-37.3159",
          "l...
   🚀 Cache speedup: 19.3x faster!

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
client_function_contract: ok
🚀 ekoDB Functions Example (TypeScript)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: tB9qwLU5ALuBa83x_vKS9yw1JV_pkR5g6oZ8l3HVVKZSIWXP2SP9CjGk7vZmUMJpA38_0HH7K35BtDbIcFID8A
📊 Found 5 active users

📝 Example 2: Parameterized Function

✅ Function saved: 2677WcuJtheD5Ijw4mgBt7oehk8jWQqivy1gmAKEJQlaNrtUcta573BecIsDvO6r0SJ-g8ay1P0x2Kq_VZQxHQ
📊 Found 3 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: nCOuvRExQL7g9bX5VUS9rsl83axsBE5XMouN6FDeW1ZbP27kU-W5hu7eMhHFqss3ZxlQZfRGNFzaxJK0CDS-2Q
📊 Statistics: 2 groups
   {"count":{"value":5,"type":"Integer"},"avg_score":{"type":"Float","value":60},"status":{"type":"String","value":"active"}}
   {"avg_score":{"type":"Float","value":50},"status":{"type":"String","value":"inactive"},"count":{"type":"Integer","value":5}}

📝 Example 4: UserFunction Management

📋 Total scripts: 5
🔍 Retrieved script: Get Active Users
✏️  function updated
🗑️  function deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

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
   {"category":{"value":"Furniture","type":"String"},"count":{"type":"Integer","value":3},"avg_price":{"value":365.6666666666667,"type":"Float"}}
   {"avg_price":{"value":367,"type":"Float"},"count":{"type":"Integer","value":5},"category":{"value":"Electronics","type":"String"}}
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
   Vector databases store embeddings—numeric representations of text, images, audio, or other data—and make it efficient to find items that are *similar in meaning or content*, not just exact matches.

Key benefits:
- **Semantic search:** Finds relevant results even when they use different wording.
- **Fast similarity matching at scale:** Uses specialized indexes to search large collections efficiently.
- **AI application support:** Helps retrieve relevant context for chatbots, recommendation systems, and retrieval-augmented generation (RAG).
- **Multimodal search:** Can compare embeddings from text, images, audio, and other data types.
- **Metadata filtering:** Combines similarity search with filters such as date, category, or permissions.

They complement rather than replace traditional databases, which are usually better for exact lookups and structured transactions.
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
🚀 ekoDB TypeScript Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: pjlIOWl5ftvL7WrPubIkqT2YjNEXFqhXmpX9WiVHLgasxx5wnove5ac37DTE_Y2y_wEcu8tcG0GOz74tExvESg
📊 Found 2 product groups
   {"avg_price":{"type":"Float","value":575.6666666666666},"category":{"type":"String","value":"Electronics"},"count":{"type":"Integer","value":3}}
   {"count":{"type":"Integer","value":2},"avg_price":{"type":"Float","value":474},"category":{"value":"Furniture","type":"String"}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"count":{"type":"Integer","value":2},"category":{"value":"Furniture","type":"String"}}
   {"count":{"value":3,"type":"Integer"},"category":{"value":"Electronics","type":"String"}}
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
   {"role":{"value":"admin","type":"String"},"avg_score":{"type":"Float","value":20},"count":{"type":"Integer","value":3}}
   {"count":{"value":7,"type":"Integer"},"avg_score":{"type":"Float","value":70},"role":{"type":"String","value":"user"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
🚀 ekoDB TypeScript KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: EO-mMzMybp4svoY4UpuPMM9ygZjaRkc6kelu4idA8REcZfxbP2qvLSbk0HsvDpNyhrpCx5MXL1gTkHAxXaW_pA
✅ Inserted 2 products with wrapped types

📝 Example 2: UserFunction with Wrapped Type Parameters

✅ Function saved: TIGHbO-dUsKoAbL0pq_i_b8X2sRdiXTq1HDYrbgahQ5aEDtg-Q0Kddb5_Ua-6K2wYcqiOwOfYKWY8N1dC3xwkg
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"type":"Object","value":{"role":"admin","userId":"user_abc"}}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: OqMcNFN5sTW3dn-0HmGnIb4dFylLcmbo000Vh5cYikEKKVXfCeJnbSbGlAguV17AMCJUU4VCNJnOfcR8DTW0tg
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: DL105Jv2LD7tpyd4So9dwrKuax2vW9RtSRjXu4eMSGpBH6yJH_tFl63WT6ImODSc182Lm6DsAshir0UK3BhXXg
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
🚀 ekoDB TypeScript Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Getting Started with ekoDB (Database)
   2. Database Design Principles (Database)
   3. Vector Databases Explained (Database)
   4. Natural Language Processing (AI)
   5. Introduction to Machine Learning (AI)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"count":{"value":2,"type":"Integer"},"category":{"value":"AI","type":"String"}}
   {"count":{"type":"Integer","value":3},"category":{"value":"Database","type":"String"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Goal Template CRUD Example (TypeScript) ===

--- Creating goal template ---
Created template: Data Migration (id: IdsfeZYlFCHefyI4XVolZV7GNmB2xwarrgiBj0EWWuvYtWvK17ROms0h8UVbvzPpFfTP1YV8b-ZtZN8aq4rtrA)

--- Listing templates ---
Templates: {
  count: 1,
  items: [
    {
      description: [Object],
      id: 'IdsfeZYlFCHefyI4XVolZV7GNmB2xwarrgiBj0EWWuvYtWvK17ROms0h8UVbvzPpFfTP1YV8b-ZtZN8aq4rtrA',
      owner: [Object],
      steps: [Object],
      title: [Object]
    }
  ],
  total: 1
}

--- Getting template ---
Fetched: Data Migration

--- Updating template ---
Updated description: Updated: comprehensive data migration workflow

--- Deleting template ---
Template deleted successfully

✓ Goal template CRUD example completed
=== ekoDB Goals, Tasks & Agents Example (TypeScript) ===

--- Creating goal ---
Created goal: Deploy v2.0 (id: KJl0aQd8B-rViEWc6UxU-NYfbZPvcs1VBh4P3PtCmS3K-hLZnunZXvKex6yYWKbH5mYs2OZNm1JcxkrjY-ZmRA)

--- Listing goals ---
Goals: {
  "count": 1,
  "goals": [
    {
      "created_at": {
        "type": "DateTime",
        "value": "2026-11-02T04:45:02.290500+00:00"
      },
      "description": {
        "type": "String",
        "value": "Ship version 2.0 to production"
      },
      "id": "KJl0aQd8B-rViEWc6UxU-NYfbZPvcs1VBh4P3PtCmS3K-hLZnunZXvKex6yYWKbH5mYs2OZNm1JcxkrjY-ZmRA",
      "owner": {
        "type": "Object",
        "value": {
          "key_id": "3a0cdba0da7c060610042783cc758f1f",
          "key_label": "default_admin"
        }
      },
      "status": {
        "type": "String",
        "value": "pending"
      },
      "steps": {
        "type": "String",
        "value": "[{\"description\":\"Run test suite\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"
      },
      "title": {
        "type": "String",
        "value": "Deploy v2.0"
      },
      "updated_at": {
        "type": "DateTime",
        "value": "2026-11-02T04:45:02.290500+00:00"
      }
    }
  ],
  "total": 1
}

--- Getting goal ---
Fetched: Deploy v2.0

--- Updating goal ---
Updated description: Ship version 2.0 with hot-fix patches

--- Searching goals ---
Search results: {
  "count": 1,
  "items": [
    {
      "_score": 12.870000000000001,
      "created_at": {
        "type": "DateTime",
        "value": "2026-11-02T04:45:02.290500+00:00"
      },
      "description": {
        "type": "String",
        "value": "Ship version 2.0 with hot-fix patches"
      },
      "id": "KJl0aQd8B-rViEWc6UxU-NYfbZPvcs1VBh4P3PtCmS3K-hLZnunZXvKex6yYWKbH5mYs2OZNm1JcxkrjY-ZmRA",
      "owner": {
        "type": "Object",
        "value": {
          "key_id": "3a0cdba0da7c060610042783cc758f1f",
          "key_label": "default_admin"
        }
      },
      "status": {
        "type": "String",
        "value": "pending"
      },
      "steps": {
        "type": "String",
        "value": "[{\"description\":\"Run test suite\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"
      },
      "title": {
        "type": "String",
        "value": "Deploy v2.0"
      },
      "updated_at": {
        "type": "DateTime",
        "value": "2026-11-02T04:45:02.303160+00:00"
      }
    }
  ]
}

--- Goal step: start step 0 ---
Step 0 started on goal KJl0aQd8B-rViEWc6UxU-NYfbZPvcs1VBh4P3PtCmS3K-hLZnunZXvKex6yYWKbH5mYs2OZNm1JcxkrjY-ZmRA
--- Goal step: complete step 0 ---
Step 0 completed on goal KJl0aQd8B-rViEWc6UxU-NYfbZPvcs1VBh4P3PtCmS3K-hLZnunZXvKex6yYWKbH5mYs2OZNm1JcxkrjY-ZmRA
--- Goal step: fail step 1 ---
Step 1 failed on goal KJl0aQd8B-rViEWc6UxU-NYfbZPvcs1VBh4P3PtCmS3K-hLZnunZXvKex6yYWKbH5mYs2OZNm1JcxkrjY-ZmRA

--- Completing goal ---
Goal status: pending_review
--- Approving goal ---
Goal status after approve: in_progress

--- Creating goal to reject ---
--- Rejecting goal ---
Goal status after reject: failed


--- Creating task ---
Created task: Hourly Health Check (id: G4rwx3NLk7OgmrSCNkzzxgSf-2tpHLrSKDgQg6gfS3lpnalYSCAgDoYAEeNOOI1NdwdYH4K5mfMbio9Q1CysDQ)

--- Listing tasks ---
Tasks: {
  "count": 1,
  "items": [
    {
      "action": {
        "type": "String",
        "value": "health_check"
      },
      "config": {
        "type": "Object",
        "value": {
          "endpoint": "/health"
        }
      },
      "cron": {
        "type": "String",
        "value": "0 * * * *"
      },
      "id": "G4rwx3NLk7OgmrSCNkzzxgSf-2tpHLrSKDgQg6gfS3lpnalYSCAgDoYAEeNOOI1NdwdYH4K5mfMbio9Q1CysDQ",
      "name": {
        "type": "String",
        "value": "Hourly Health Check"
      },
      "owner": {
        "type": "Object",
        "value": {
          "key_id": "3a0cdba0da7c060610042783cc758f1f",
          "key_label": "default_admin"
        }
      }
    }
  ],
  "total": 1
}

--- Getting task ---
Fetched: Hourly Health Check

--- Starting task ---
Task status: running

--- Succeeding task ---
Task status after succeed: active

--- Pausing task ---
Task status after pause: paused

--- Resuming task ---
Task status after resume: active

--- Failing task ---
Task status after fail: active

--- Getting due tasks ---
Due tasks: {
  "count": 0,
  "items": []
}

--- Deleting task ---
Task deleted successfully


--- Creating agent ---
Created agent: SupportBot (id: US9Co6gSIImFW8TILtCU9yl8CndRcyofoUSM5kH_DZDHPUnJyp3P0TaSClOo4J_UNwzPemGd_MOBVvR70VI38g)

--- Listing agents ---
Agents: {
  "count": 1,
  "items": [
    {
      "deployment_id": {
        "type": "String",
        "value": "deploy_prod_1"
      },
      "id": "US9Co6gSIImFW8TILtCU9yl8CndRcyofoUSM5kH_DZDHPUnJyp3P0TaSClOo4J_UNwzPemGd_MOBVvR70VI38g",
      "llm_model": {
        "type": "String",
        "value": "gpt-6-luna"
      },
      "name": {
        "type": "String",
        "value": "SupportBot"
      },
      "owner": {
        "type": "Object",
        "value": {
          "key_id": "3a0cdba0da7c060610042783cc758f1f",
          "key_label": "default_admin"
        }
      },
      "run_policy": {
        "type": "String",
        "value": "{\"trigger\":{\"allow_write_operations\":false,\"write_tools\":[]},\"eval\":{\"allow_write_operations\":false,\"write_tools\":[]},\"customer\":{\"allow_write_operations\":false,\"write_tools\":[]}}"
      },
      "system_prompt": {
        "type": "String",
        "value": "You are a helpful customer support agent."
      }
    }
  ],
  "total": 1
}

--- Getting agent by ID ---
Fetched: SupportBot

--- Getting agent by name ---
By name: SupportBot (id: US9Co6gSIImFW8TILtCU9yl8CndRcyofoUSM5kH_DZDHPUnJyp3P0TaSClOo4J_UNwzPemGd_MOBVvR70VI38g)

--- Updating agent ---
Updated agent: SupportBot

--- Getting agents by deployment ---
Agents in deployment: {
  "count": 0,
  "items": []
}
WARNING: agents-by-deployment omitted created agent US9Co6gSIImFW8TILtCU9yl8CndRcyofoUSM5kH_DZDHPUnJyp3P0TaSClOo4J_UNwzPemGd_MOBVvR70VI38g; TODO: check/fix the server-side deployment lookup

--- Deleting agent ---
Agent deleted successfully

--- Cleanup: deleting goals ---
Goals deleted successfully

=== All goals, tasks & agents operations completed ===
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


✅ Join operations examples completed!
=== Cleanup ===
✅ Deleted test collections
✓ Client created
✓ ts_users_register saved
✓ ts_users_login saved
✓ ts_users_verify_token saved

=== Auth flow defined as pure stored functions ===
Call them like:
  POST /api/functions/jwt_register_ts_52767_1793594703397 { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/jwt_login_ts_52767_1793594703397 { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/jwt_verify_ts_52767_1793594703397 { "token": "<jwt>" }

Set JWT_SECRET in ekoDB's environment_vars whitelist before invoking.

✓ Cleaned up demo functions
=== ekoDB KV Document Linking Example (TypeScript) ===

--- Setup: creating KV key and documents ---
Set KV key: session:admin
Inserted document 1: jV7-3vJJ9mY1h22VijEWnscb59uPIpqyoN3_33QC48Ib8bEw6yR9PUtQ_4UR5oE1pph2kIMFsgYE6GSTqQ19Fw
Inserted document 2: ugtKpu9QNn8cvnFDT3wPhEY2_888tiC2Kb_R08u1W04xy1a28SsCis79meKsQ9DiKBN3x8MuLtUkstjgFbOeeQ

--- Linking documents to KV key ---
Linked doc jV7-3vJJ9mY1h22VijEWnscb59uPIpqyoN3_33QC48Ib8bEw6yR9PUtQ_4UR5oE1pph2kIMFsgYE6GSTqQ19Fw: null
Linked doc ugtKpu9QNn8cvnFDT3wPhEY2_888tiC2Kb_R08u1W04xy1a28SsCis79meKsQ9DiKBN3x8MuLtUkstjgFbOeeQ: null

--- Getting links for KV key ---
Links: [
  {
    "collection": "kv_links_example_ts_52846_1793594703753",
    "document_id": "jV7-3vJJ9mY1h22VijEWnscb59uPIpqyoN3_33QC48Ib8bEw6yR9PUtQ_4UR5oE1pph2kIMFsgYE6GSTqQ19Fw",
    "field_path": null,
    "created_at": "2026-11-02T04:45:03.836730Z",
    "last_accessed": "2026-11-02T04:45:03.839945Z",
    "metadata": {}
  },
  {
    "collection": "kv_links_example_ts_52846_1793594703753",
    "document_id": "ugtKpu9QNn8cvnFDT3wPhEY2_888tiC2Kb_R08u1W04xy1a28SsCis79meKsQ9DiKBN3x8MuLtUkstjgFbOeeQ",
    "field_path": null,
    "created_at": "2026-11-02T04:45:03.838461Z",
    "last_accessed": "2026-11-02T04:45:03.839945Z",
    "metadata": {}
  }
]

--- Unlinking document ---
Unlinked doc ugtKpu9QNn8cvnFDT3wPhEY2_888tiC2Kb_R08u1W04xy1a28SsCis79meKsQ9DiKBN3x8MuLtUkstjgFbOeeQ: null

--- Verifying remaining links ---
Remaining links: [
  {
    "collection": "kv_links_example_ts_52846_1793594703753",
    "document_id": "jV7-3vJJ9mY1h22VijEWnscb59uPIpqyoN3_33QC48Ib8bEw6yR9PUtQ_4UR5oE1pph2kIMFsgYE6GSTqQ19Fw",
    "field_path": null,
    "created_at": "2026-11-02T04:45:03.836730Z",
    "last_accessed": "2026-11-02T04:45:03.842454Z",
    "metadata": {}
  }
]

=== All KV linking operations completed ===

--- Cleanup ---
Cleanup complete
✓ Client created

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: { type: 'Object', value: { userId: 123, username: 'john_doe' } }

=== KV Batch Set ===
✓ Batch set 3 keys
  kv_ops_ts_52931_1793594704219:cache:product:1: success
  kv_ops_ts_52931_1793594704219:cache:product:2: success
  kv_ops_ts_52931_1793594704219:cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  kv_ops_ts_52931_1793594704219:cache:product:1: { name: 'Product 1', price: 29.99 }
  kv_ops_ts_52931_1793594704219:cache:product:2: { price: 39.99, name: 'Product 2' }
  kv_ops_ts_52931_1793594704219:cache:product:3: { name: 'Product 3', price: 49.99 }

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
  kv_ops_ts_52931_1793594704219:cache:product:1: deleted
  kv_ops_ts_52931_1793594704219:cache:product:2: deleted
  kv_ops_ts_52931_1793594704219:cache:product:3: deleted

✓ All KV operations completed successfully
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

=== Summary ===
✅ Use Field.decimal() for monetary values, percentages, and
   any case where floating-point errors are unacceptable.
✅ Field.decimal() stores values as strings internally,
   preserving exact precision across all operations.

=== Cleanup ===
✓ Cleaned up test keys
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
=== ekoDB Raw Completion Stream (SSE) Example ===

--- Basic SSE Raw Completion ---
Response: For traditional painting, the three primary colors are **red, yellow, and blue**. For light, they’re **red, green, and blue**.

--- Structured Output via SSE ---
JSON response: [
  {"name": "Earth", "diameter_km": 12742},
  {"name": "Mars", "diameter_km": 6779},
  {"name": "Jupiter", "diameter_km": 139820}
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello!

=== Done ===
=== ekoDB Schedule Management Example (TypeScript) ===

--- Creating schedule ---
Created schedule: Nightly Database Backup (id: 273bea77-df49-4b70-a627-959ed5fceb27, cron: 0 0 2 * * *)

--- Listing schedules ---
Schedules: {
  "count": 1,
  "schedules": [
    {
      "created_at": "2026-11-02T04:45:10.995195Z",
      "cron_expression": "0 0 2 * * *",
      "description": null,
      "enabled": true,
      "function_label": "schedule_noop_typescript_54427_1793594710962",
      "id": "273bea77-df49-4b70-a627-959ed5fceb27",
      "last_execution": null,
      "name": "Nightly Database Backup",
      "next_execution": "2026-11-03T02:00:00Z",
      "parameters": {},
      "stats": {
        "avg_execution_time_ms": 0,
        "failed_executions": 0,
        "last_error": null,
        "successful_executions": 0,
        "total_executions": 0
      },
      "timezone": "UTC",
      "updated_at": "2026-11-02T04:45:10.995195Z"
    }
  ]
}

--- Getting schedule ---
Fetched: Nightly Database Backup (cron: 0 0 2 * * *)

--- Updating schedule ---
Updated: Nightly Full Backup (new cron: 0 0 3 * * *)

--- Triggering schedule ---
Trigger response: {
  "schedule_id": "273bea77-df49-4b70-a627-959ed5fceb27",
  "status": "triggered"
}

--- Pausing schedule ---
Schedule enabled after pause: false

--- Resuming schedule ---
Schedule enabled after resume: true

--- Deleting schedule ---
Schedule deleted successfully

=== All schedule operations completed ===
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
  4. Score: 26.400, Matched: title, bio

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 6.600, Matched: bio
  2. Score: 6.600, Matched: bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 6.600, Matched: bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.776, Matched:
  2. Score: 0.747, Matched:
  3. Score: 0.734, Matched:

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.499, Matched: title, content
  2. Score: 0.910, Matched: title, content
  3. Score: 0.294, Matched:

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: skills, bio

9. Vector search with a metadata pre-filter (category = ml):
Found 2 documents in category "ml" (NLP excluded)
  1. Deep Learning Fundamentals (category: ml)
  2. Introduction to Machine Learning (category: ml)

Paired vector search: document=Rust Programming Answer, query=Rust Programming Question

✅ Search examples completed!
=== Cleanup ===
✅ Deleted test collections
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'eSACgRwxC-hCE699EW8t410_yM-kCwrS7BygcQ1MccB6l694UrKjpwZ7wlZzSlsUq02OWil50RBHir8gsb9EKA'
}

=== Find by ID ===
Found: {
  metadata: { value: { nested: [Object], key: 'value' }, type: 'Object' },
  active: { type: 'Boolean', value: true },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  id: 'eSACgRwxC-hCE699EW8t410_yM-kCwrS7BygcQ1MccB6l694UrKjpwZ7wlZzSlsUq02OWil50RBHir8gsb9EKA',
  created_at: { value: '2026-11-02T04:45:12.416+00:00', type: 'DateTime' },
  value: { value: 42, type: 'Integer' },
  name: { type: 'String', value: 'Test Record' },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  price: { type: 'Float', value: 99.99 },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-11-02T04:45:12.416Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { nested: { deep: true }, key: 'value' }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  metadata: { nested: { deep: true }, key: 'value' },
  active: true,
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  id: 'eSACgRwxC-hCE699EW8t410_yM-kCwrS7BygcQ1MccB6l694UrKjpwZ7wlZzSlsUq02OWil50RBHir8gsb9EKA',
  created_at: '2026-11-02T04:45:12.416+00:00',
  value: 42,
  name: 'Test Record',
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  price: 99.99,
  data: 'aGVsbG8gd29ybGQ=',
  tags: [ 'tag1', 'tag2', 'tag3' ],
  categories: [ 'electronics', 'computers' ]
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  value: { type: 'Integer', value: 100 },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  created_at: { value: '2026-11-02T04:45:12.416+00:00', type: 'DateTime' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  id: 'eSACgRwxC-hCE699EW8t410_yM-kCwrS7BygcQ1MccB6l694UrKjpwZ7wlZzSlsUq02OWil50RBHir8gsb9EKA',
  price: { type: 'Float', value: 99.99 },
  metadata: { value: { nested: [Object], key: 'value' }, type: 'Object' },
  active: { type: 'Boolean', value: true },
  name: { type: 'String', value: 'Updated Record' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: HnIKbWcirZP4ow_dA_VRuyuFlbvCnjuKh4dV526n5bGHdmo2Ppqan38s1NRK7iA8MYmiVbII5LWxNHxszHjk3A

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
🚀 ekoDB TypeScript Client - Native SWR Function Examples

📋 Demonstrates:
   • Single-function SWR pattern (replaces 4-step pipeline)
   • Automatic cache checking, HTTP fetching, and cache setting
   • Built-in audit trail support
   • Duration string TTLs ('15m', '1h', '30s')
   • Multi-function pipeline integration
   • Dynamic TTL configuration


🧹 Cleaning up...
✓ Deleted 0 test scripts and owned SWR resources

Example 1: Basic Native SWR
────────────────────────────────────────────────────────────────────────────────
Single function replaces KvGet → If → HttpRequest → KvSet pipeline
✓ Created native SWR script: github_user_native_ts (Kqwvk2PyvGnZo87CEbEGApzc2vJIkjr6wvitBU4HXSOk6shzXMTqjYSLcitT0MOaBY_skmYvGEI3uN42HoxLgQ)

First call (cache miss - will fetch from GitHub API):
  Response time: 170ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 3ms
  Speedup: 56.7x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_ts (M2TwV0qNLH-61AOUPqQk0uPCwWteN7EmeNzhTg7np0NSYCwcV7ksHHN7bLfsNw8Fn9OMXpGtQDMX9UwvjTTZ3g)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_ts' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_ts (fSC5vic8jOs8CIFAB5cljZ3gpA09IY1WhsVavPIXmQRNA7Kk9rFayDhiI5JgdXVm_UQlQaoB_LkjsG-6fU6JLg)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_ts' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_ts (FQPXReVEMaLL7CIIjHzWZEz4D_o7HKBLogo1XhqdYrX55TBnUNrztf9-oiwDlse4tIr4iTabZrvp5j2xVmWsCg)
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
✓ Deleted 4 test scripts and owned SWR resources

✅ All examples completed!
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_ts_55079_1793594714450 (w-sBKieHcaEwNdVXyhGxfqVQhAmTDzTv-83tHleVfniE8dx9E5gwWY8Jwqru8W6up86IsK1PqYLVv5lzrkz_gg)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "id": 1,
          "address": {
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "suite": "Apt. 556",
            "street": "Kulas Light",
            "city": "Gwenborough",
            "zipcode": "92998-3874"
          },
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets"
          },
          "email": "Sincere@april.biz",
          "name": "Leanne Graham",
          "phone": "1-770-736-8031 x56442",
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
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 3ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "value": {
          "id": 1,
          "address": {
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "suite": "Apt. 556",
            "street": "Kulas Light",
            "city": "Gwenborough",
            "zipcode": "92998-3874"
          },
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets"
          },
          "email": "Sincere@april.biz",
          "name": "Leanne Graham",
          "phone": "1-770-736-8031 x56442",
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
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_reviews_ts_55079_1793594714450 (jweTlPsQrl8qRuCmdYXU2a8LkzzHHbVwJ5TY6ErFZBQmeVZCBIxhpgcn9YbOYQlYAqLquuM0_-FPR2T2xXKW2A)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "brand": "Essence",
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "dimensions": {
            "depth": 22.99,
            "height": 13.08,
            "width": 15.14
          },
          "meta": {
            "createdAt": "2025-10-09T14:47:01.588Z",
            "barcode": "5784719087687",
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
            "updatedAt": "2026-05-23T11:27:41.868Z"
          },
          "rating": 2.56,
          "category": "beauty",
          "minimumOrderQuantity": 48,
          "id": 1,
          "discountPercentage": 10.48,
          "availabilityStatus": "In Stock",
          "tags": [
            "beauty",
            "mascara"
          ],
          "title": "Essence Mascara Lash Princess",
          "sku": "BEA-ESS-ESS-001",
          "price": 9.99,
          "shippingInformation": "Ships in 3-5 business days",
          "reviews": [
            {
              "rating": 3,
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "comment": "Would not recommend!",
              "reviewerName": "Eleanor Collins",
              "date": "2025-04-30T09:41:02.053Z"
            },
            {
              "comment": "Very satisfied!",
              "reviewerName": "Lucas Gordon",
              "date": "2025-04-30T09:41:02.053Z",
              "rating": 4,
              "reviewerEmail": "lucas.gordon@x.dummyjson.com"
            },
            {
              "date": "2025-04-30T09:41:02.053Z",
              "rating": 5,
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "comment": "Highly impressed!",
              "reviewerName": "Eleanor Collins"
            }
          ],
          "stock": 99,
          "returnPolicy": "No return policy",
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "warrantyInformation": "1 week warranty",
          "weight": 4
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

✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: a0O1_y8foIsBOko4z_r9kWsX5DDiA9V2MCfOyDQhkfRptLwAQHviVq-6FsPPjnsNNFG1XNF8ljNOsMlgFvFclg
Created Bob: $500 - ID: 6ZeYvlf-3ztnWQpDPVVGHaZpysRUG1u208C6hfu86y0ySzEdGtaSyji7L6viBdSgb1-EHzgC9MXP34eTntdHuA

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 42f21aa7-4ab9-40f8-877c-22f937e2a857

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
New transaction: a622c1aa-08ed-4755-bc8c-823b4ff6eb50
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Create User Function ===
Created user function with ID: tLeSdZTFqV1zHwPyGa7ldgT-hwlPbT2UHbtgiF-scuMxsAO4Eg_ykH49sbWBOFidYtml0gT7nooodxswWxMXUA

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 5 user functions:
  - conc_demo_rl_skip_ts_50152_1793594691316: Rate-limit (skip mode)
  - conc_demo_rl_fail_ts_50152_1793594691316: Rate-limit (fail mode)
  - fetch_product_reviews_ts_55079_1793594714450: Fetch Product with Reviews (Multi-API)
  - get_active_users_ts: Get Active Users
  - get_active_users_client_ts: Get Active Users (Updated)

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
=== WebSocket Chat Streaming Example (TypeScript) ===

Created chat session: sCr31eUvVFvjTL_7HTeZatrjoopjEF-8Chb3jle9xF3smpTbBwhL2QG-rI9XUvg1ErYCSIfXZNxLPBBkqgpm7g

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"gpt-5.6-luna","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: pMZubX4b67KLhg40H_by8qlqBR3eil1Sm78dmWYI6KvTD9x_0KtAI52Ge7I8y0dsLE3llzw7YcZ5ei5sPG60TQ
Execution time: 816ms
Token usage: {"cached_input_tokens":0,"completion_tokens":11,"prompt_tokens":23,"thinking_tokens":0,"total_tokens":34}

Full response: {"__progress":"received"}{"__progress":"generating","model":"gpt-5.6-luna","provider":"openai"}The capital of France is Paris....
=== WebSocket Subscription Example ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_ts' ===
✓ Subscribed (subscription_id: sub_1b7e74c4a6cc40beb4afc68147861c97)

=== Performing mutations to trigger notifications ===
Inserting a record...
✓ Inserted record: j56xxkFM8IFvM6J9uljiRHWXJRdqsEXIoJsm1S5J7dWMBWTDGm-yokhCsARA4PDQfsCy7IeFAD_GeLpaee5m4Q
  📡 Notification received for j56xxkFM8IFvM6J9uljiRHWXJRdqsEXIoJsm1S5J7dWMBWTDGm-yokhCsARA4PDQfsCy7IeFAD_GeLpaee5m4Q

Inserting another record...
✓ Inserted record: lP2dOo_jM3RUHV47QdI4UE06s2oVcHZfe7LLo2X8LF_SvksqfwWfYgpwHtZZcgiyspTxEpiJYVDXuN8wUXL2fQ
  📡 Notification received for lP2dOo_jM3RUHV47QdI4UE06s2oVcHZfe7LLo2X8LF_SvksqfwWfYgpwHtZZcgiyspTxEpiJYVDXuN8wUXL2fQ

=== Unsubscribing ===
✓ Unsubscribed: {"collection":"ws_subscribe_example_ts","found":true,"unsubscribed":true}

✓ WebSocket subscription example completed successfully
✓ Deleted collection 'ws_subscribe_example_ts'
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Hxh3SLN5ELNf6pdEZ22EArVsHRv0TEbnAIDpkC787g58qn1BZzzTzHhzDmTb3YPOjj4iPLMZKcco6m06acXIYQ

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: {"id":"ScMIPbxNJrsqw0pwQZKLvnTj-yqaeRh0qmLKcxpyv86uUqAMp1dGms_gcJvy8e2GtAsvieSsbtgwkDddtcRSbA"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"gB9VqkW5JHBp2ht4ThIK8BLImm-4PUCXsT-SHvrBWPWI5oBI4mGioUEY-b4FXjrjRfV1fgB29NyrkaUpJhV54g"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"price":{"value":150,"type":"Integer"},"id":"ScMIPbxNJrsqw0pwQZKLvnTj-yqaeRh0qmLKcxpyv86uUqAMp1dGms_gcJvy8e2GtAsvieSsbtgwkDddtcRSbA","name":{"value":"Product 1","type":"String"}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"id":"custom-id"}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ["id","email","name"]
  First user: Alice Johnson <alice@example.com>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["id","bio","user_role","age","email","status","avatar_url","created_at","name"]

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
    - Fields: ["age","email","api_key","password","created_at","id","secret_token","user_role","name","status","bio","avatar_url"]
  Projected query:
    - 3 fields per record
    - Fields: ["email","name","id"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
✅ All TypeScript integration tests complete!
🧪 Running JavaScript examples (direct HTTP/WebSocket)...

added 1 package, removed 1 package, and audited 9 packages in 479ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
npm warn install-scripts 1 package has install scripts not yet covered by allowScripts:
npm warn install-scripts   ekodb-client-ts@0.31.0 (prepare: npm run build)
npm warn install-scripts
npm warn install-scripts Run `npm install-scripts ls` to review, or `npm install-scripts approve <pkg>` to allow.

╔════════════════════════════════════════╗
║  ekoDB JavaScript Examples Test Suite ║
╚════════════════════════════════════════╝

=== Checking Server Connection ===
✓ Server is ready

=== Getting Authentication Token ===
✓ Authentication successful

=== Running 10 Examples ===

=== Running ekoDB/ekodb-client/examples/javascript/simple_crud.js ===
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {
  id: '6Nl87InT5xuS_O9AMlMWrN9_ISMwinpWaNHLjYHj2BFzRH4iC8Tue2p9UZ_rYS9EF0vh1EvFW6GAG0Nk4yLAqw'
}

=== Find by ID ===
Found: {
  name: { type: 'String', value: 'Test Record' },
  id: '6Nl87InT5xuS_O9AMlMWrN9_ISMwinpWaNHLjYHj2BFzRH4iC8Tue2p9UZ_rYS9EF0vh1EvFW6GAG0Nk4yLAqw',
  active: { value: true, type: 'Boolean' },
  value: { value: 42, type: 'Integer' }
}

=== Find with Query ===
Found documents: [
  {
    active: { type: 'Boolean', value: true },
    id: '6Nl87InT5xuS_O9AMlMWrN9_ISMwinpWaNHLjYHj2BFzRH4iC8Tue2p9UZ_rYS9EF0vh1EvFW6GAG0Nk4yLAqw',
    value: { value: 42, type: 'Integer' },
    name: { value: 'Test Record', type: 'String' }
  }
]

=== Update Document ===
Updated: {
  id: '6Nl87InT5xuS_O9AMlMWrN9_ISMwinpWaNHLjYHj2BFzRH4iC8Tue2p9UZ_rYS9EF0vh1EvFW6GAG0Nk4yLAqw',
  active: { value: true, type: 'Boolean' },
  name: { value: 'Updated Record', type: 'String' },
  value: { type: 'Integer', value: 100 }
}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ simple_crud.js completed successfully

=== Running ekoDB/ekodb-client/examples/javascript/simple_websocket.js ===
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: jNy2s8Ks8Bt2QJ--hAI3yylTOyt5MwxgGLxtpGqMaze7ZnE8u2DJZ43rBbxIzF2PgKnSXgDkFlWxd7MabFteIA

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "type": "Success",
  "payload": {
    "data": [
      {
        "value": {
          "type": "Integer",
          "value": 42
        },
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "active": {
          "value": true,
          "type": "Boolean"
        },
        "id": "jNy2s8Ks8Bt2QJ--hAI3yylTOyt5MwxgGLxtpGqMaze7ZnE8u2DJZ43rBbxIzF2PgKnSXgDkFlWxd7MabFteIA"
      }
    ]
  },
  "messageId": "1793594721529"
}
✓ Retrieved 1 record via WebSocket

✓ WebSocket example completed successfully
WebSocket closed
✓ simple_websocket.js completed successfully

=== Running ekoDB/ekodb-client/examples/javascript/http_functions.js ===
🚀 ekoDB Functions Example (JavaScript/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: w6Yf7HDFlUxUAKGl3HAKd9wtayyw-vFNYeMMHXVJjKLLbz30yi6b6T_iExdka1CmUv7j3zeR9TBs6gA8BNcvSw
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: txQulunq4KAIq9RIpuAcVSH6kJ1JnMOnsQgM_Io9axzR6jGHTfc8U1kS6tHa-93aAR4WcYMuwQWuVVjlvslxRg
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: GFKHl_GQ9HauwPyxR1FBU4NWUgn88DqfAdD0TG1W89kBS2o92UfTchV0CiEwpdr_loDZH_INQte1pbD2Eb42Qw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"count":{"type":"Integer","value":5},"avg_score":{"value":60,"type":"Float"},"status":{"type":"String","value":"active"},"max_score":{"value":100,"type":"Integer"}}
   {"count":{"value":5,"type":"Integer"},"max_score":{"type":"Integer","value":90},"avg_score":{"value":50,"type":"Float"},"status":{"type":"String","value":"inactive"}}

📝 Example 4: Function Management

📋 Total functions: 7
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
✓ http_functions.js completed successfully

=== Running ekoDB/ekodb-client/examples/javascript/batch_operations.js ===
=== Batch Operations (Direct HTTP) ===

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
✓ batch_operations.js completed successfully

=== Running ekoDB/ekodb-client/examples/javascript/kv_operations.js ===
=== Key-Value Operations (Direct HTTP) ===

✓ Authentication successful

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: { type: 'Object', value: { username: 'john_doe', userId: 123 } }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { value: { price: 29.99, name: 'Product 1' }, type: 'Object' }
cache:product:2: {
  value: { name: 'Product 2', price: 39.989999999999995 },
  type: 'Object'
}
cache:product:3: {
  value: { name: 'Product 3', price: 49.989999999999995 },
  type: 'Object'
}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ kv_operations.js completed successfully

=== Running ekoDB/ekodb-client/examples/javascript/collection_management.js ===
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: XdNdHTaIMIq8gKk8Eh_zaEpCuDN36RIAVv_vbWiBZOTwIygyKqtnAJmRw2bE6_PKSGJUGomHSE7ofLVVx4Yv_w

=== List Collections ===
Total collections: 22
Sample collections: [
  'schema_users_client_go',
  'audit__ek0_testing',
  'schema_documents_client_ts',
  'schema_employees_client_ts',
  'schema_employees_client_go'
]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ collection_management.js completed successfully

=== Running ekoDB/ekodb-client/examples/javascript/transactions.js ===
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: GcZfZwFc5yzyz3mI1itAnRZse9LWvnXVO3XeYGdyCMJ-fl3xfb3Hp1U91dotK9QVVGWsCJQNvZAWvOTS6Pw_bw
Created Bob: $500 - ID: l1lJJqry1tXDlXP2l9-ECSbLKooZR8QuJAGjcrm9vK0TDZca5I-_2X0xz3GjVNSm3Pck2WuxbCO9BTschQu15w

=== Example 1: Begin Transaction ===
Transaction ID: 76df91b1-3249-42c2-80f7-d098606bcb5c

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
Bob: {"type":"Integer","value":700}

=== Example 5: Rollback ===
New transaction: 0cd059bc-0038-4550-b826-21b850aa674e
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"value":700,"type":"Integer"}

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✓ transactions.js completed successfully

=== Running ekoDB/ekodb-client/examples/javascript/crud_functions.js ===
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
   ✅ Deleted collection: crud_functions_users_js

============================================================
📝 Function 1: Insert + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: y-LsU3R0yDCG2Y0GprGuKl_8Cw2faOcnm54HAEDws0JzeItPoyZwlUWSYFjXAfLoxxGQTzPK3UsMvCA3NXMJDw

2️⃣ Calling Function (Insert + Verify)...
   ✅ Function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Name: Alice Smith
   📋 Email: alice@example.com
   📋 Status: pending
   📋 Credits: 0

============================================================
📝 Function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: dtFGqmbgOFTLaZiVfMZST-LQxIAF1Pq78wzRwq5fcCuSXI4hKlCKZuvGsZVMk94bkXhAB7y9tvxd4VKcHVoh1A

2️⃣ Calling Function (Query + Update + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {"value":"active","type":"String"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Function 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: jpgssmfItfaMqyCdY736LhuwlrXqYao6XRVIz9Mrunb99DDbHrQ_LiEMLByZscFZA8U5F34hkP2zJOmv77CjIw

2️⃣ Calling Function (Query + Update Credits + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"type":"Integer","value":100}
   📋 Status: {"value":"active","type":"String"}
   📋 Name: {"type":"String","value":"Alice Smith"}

============================================================
📝 Function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: y2hjrKaJjW9eHsFS_cNU88JtMOQPs4FKSYDtSRbox5G3-afZlW5vmbTKGbFEHWu5jZ-NTN8l3WxKB9HQpQoUqQ

2️⃣ Calling Function (Query + Delete + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted function: y-LsU3R0yDCG2Y0GprGu...
   ✅ Deleted function: dtFGqmbgOFTLaZiVfMZS...
   ✅ Deleted function: jpgssmfItfaMqyCdY736...
   ✅ Deleted function: y2hjrKaJjW9eHsFS_cNU...
   ✅ Deleted collection: crud_functions_users_js

============================================================
✅ Complete CRUD Functions Example Finished!
============================================================

💡 Key Takeaways:
   ✅ Functions chain Functions together
   ✅ Each function demonstrates operation + verification
   ✅ Parameters make functions reusable
   ✅ Verification is built into the function itself
   ✅ Complete CRUD lifecycle in 4 focused functions
✓ crud_functions.js completed successfully

=== Running ekoDB/ekodb-client/examples/javascript/document_ttl.js ===
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
  Output: Document ID = XcxWAYWWNPYNl_st1kgxf6j4OVu1yNdlZU5zoqFiqxeypDN-Ocr1EP9vsDkPPpeimUEMRWBqiuYF2Bo79JCoRA
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(XcxWAYWWNPYNl_st1kgxf6j4OVu1yNdlZU5zoqFiqxeypDN-Ocr1EP9vsDkPPpeimUEMRWBqiuYF2Bo79JCoRA)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(XcxWAYWWNPYNl_st1kgxf6j4OVu1yNdlZU5zoqFiqxeypDN-Ocr1EP9vsDkPPpeimUEMRWBqiuYF2Bo79JCoRA)
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
✓ document_ttl.js completed successfully

=== Running ekoDB/ekodb-client/examples/javascript/websocket_ttl.js ===
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
  Output: Document ID = 0tHOg0QM-TuUbYiYS24pPhCWD9MlFSuVY9CZJ39biyqwLa2N4NjlCWNXt86Iq0UHWy0MiAJzjb3YvPhzJYQR8A
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(0tHOg0QM-TuUbYiYS24pPhCWD9MlFSuVY9CZJ39biyqwLa2N4NjlCWNXt86Iq0UHWy0MiAJzjb3YvPhzJYQR8A)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(0tHOg0QM-TuUbYiYS24pPhCWD9MlFSuVY9CZJ39biyqwLa2N4NjlCWNXt86Iq0UHWy0MiAJzjb3YvPhzJYQR8A)
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
✓ websocket_ttl.js completed successfully

╔════════════════════════════════════════╗
║           Test Summary                 ║
╚════════════════════════════════════════╝
Total: 10
Passed: 10
Failed: 0
✅ JavaScript direct examples complete!
📦 Building TypeScript client library...

> @ekodb/ekodb-client@0.31.0 prepare
> npm run build


> @ekodb/ekodb-client@0.31.0 build
> tsc


up to date, audited 44 packages in 596ms

12 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
npm warn install-scripts 1 package has install scripts not yet covered by allowScripts:
npm warn install-scripts   fsevents@2.3.3 (install: (install scripts present))
npm warn install-scripts
npm warn install-scripts Run `npm install-scripts ls` to review, or `npm install-scripts approve <pkg>` to allow.

> @ekodb/ekodb-client@0.31.0 build
> tsc

✅ TypeScript client built!

added 1 package, removed 1 package, and audited 9 packages in 417ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
npm warn install-scripts 1 package has install scripts not yet covered by allowScripts:
npm warn install-scripts   ekodb-client-ts@0.31.0 (prepare: npm run build)
npm warn install-scripts
npm warn install-scripts Run `npm install-scripts ls` to review, or `npm install-scripts approve <pkg>` to allow.

added 1 package, removed 1 package, and audited 13 packages in 428ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
npm warn install-scripts 3 packages have install scripts not yet covered by allowScripts:
npm warn install-scripts   ekodb-client-ts@0.31.0 (prepare: npm run build)
npm warn install-scripts   esbuild@0.28.1 (postinstall: node install.js)
npm warn install-scripts   fsevents@2.3.3 (install: (install scripts present))
npm warn install-scripts
npm warn install-scripts Run `npm install-scripts ls` to review, or `npm install-scripts approve <pkg>` to allow.

> ekodb-typescript-examples@1.0.0 build
> tsc

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
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: jz9QPY6BLrmY_lgPCThGQt6JHTeu1asDQKWWPFLpSn4BsQpu3yIvjewGwHsKy5wi62aclcBMiju0E8_NORwk0A

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is **ekoDB**, a high-performance database product, priced at **$99**.

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
✓ Created second session: owaftWBhJb3vgtXqGTkcSZ6294kRpbD7lIyMtEoqv1bFlxEPauvhf30p1m8o5-oQPqSQOvLVEpmL7szteZv9lA
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted second session
✓ Deleted session
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: hvzIVyja9ggFktxel9TixkIu-xW_aGIq-tu0TjP5jGgx_853aHAaQ88Nk1N96L41MP5a6VeAPMgeU-jvUAhhwg

=== Sending Chat Message ===
Message ID: mcQ30hwpuU3tlvN_04R5FmvCUF51W7A88KVoLBJITjxt8bNMJMsPXzarIyrxitbNSiNIuBhjp67fqUGC6_Pp4w

=== AI Response ===
Available products and prices:

- ekoDB — $99
- ekoDB Pro — $299
- ekoDB Cloud — $499

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    id: 'Y_p4nwxrjfiWIn-d7cWneyldaaRtg7pkUCrtk6WCUBPJq-F8ieHu2j0XpJU0c1ytZ3Bjt3LevN9Rv_jT_IzpMg',
    price: 299,
    name: 'ekoDB Pro',
    description: 'Enterprise edition product with advanced features'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    description: 'A high-performance database product with AI capabilities',
    price: 99,
    name: 'ekoDB',
    id: 'Yi1IDugORLhETtn7Dzlg59xzeHi5jaqXWG3tqRVR_h3C2-LJM76Cv5xYfJ1jom8b5orp2WPn-f0ftQu2YlNqTQ'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    description: 'Fully managed cloud database service product',
    id: 'y2gK3bP5qsbvRUXjZackdNKH7iUYTOXf7JTk8e9MSx5fWWKOen4bfCBQ-YZ62Y9fV3Mu3pC38MnRoM7CaSPZSA',
    price: 499,
    name: 'ekoDB Cloud'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 1990ms

=== Token Usage ===
Prompt tokens: 13714
Completion tokens: 34
Total tokens: 13748

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
✓ Client created

=== List Chat Models ===
Available chat models by provider:
  openai:
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
  anthropic:
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
  gemini:
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

=== Get Specific Provider Models ===
OpenAI models: gpt-3.5-turbo-0125, gpt-5.4-2026-03-05, gpt-5-nano, gpt-5.5-pro, gpt-audio-2025-08-28, gpt-4o-mini-transcribe-2025-03-20, gpt-4-turbo-2024-04-09, gpt-realtime-2.1-mini, gpt-realtime-2025-08-28, gpt-4, tts-1-hd, gpt-realtime-1.5, gpt-realtime, gpt-5.2-pro, gpt-5-chat-latest, gpt-5.4-mini-2026-03-17, babbage-002, gpt-4o-2024-05-13, text-embedding-3-large, tts-1, omni-moderation-2024-09-26, gpt-live-transcribe, gpt-5.6-luna, gpt-5-2025-08-07, gpt-5-mini, gpt-5-mini-2025-08-07, omni-moderation-latest, o3-mini-2025-01-31, gpt-audio-1.5, gpt-image-1.5, tts-1-1106, gpt-5.5-pro-2026-04-23, gpt-audio-mini-2025-10-06, gpt-5.2-pro-2025-12-11, o4-mini-2025-04-16, gpt-4o-2024-11-20, gpt-5.1, whisper-1, gpt-5-pro, gpt-realtime-whisper, gpt-realtime-2, gpt-5-search-api, gpt-5.5-2026-04-23, gpt-4.1-nano, gpt-image-2, gpt-3.5-turbo-16k, text-embedding-ada-002, gpt-realtime-2.1, gpt-4.1, gpt-5.5, chat-latest, gpt-realtime-translate, gpt-5.4-pro-2026-03-05, gpt-4.1-mini, gpt-4o-mini, gpt-4o-mini-transcribe-2025-12-15, gpt-realtime-mini, gpt-image-1-mini, gpt-5.6-terra, gpt-3.5-turbo-instruct-0914, gpt-4-turbo, gpt-4o-2024-08-06, o3, gpt-4o-mini-search-preview-2025-03-11, o1-2024-12-17, gpt-5.1-chat-latest, gpt-5-pro-2025-10-06, gpt-5.6-sol, gpt-4o-transcribe-diarize, gpt-5-codex, gpt-4-0613, gpt-5.1-2025-11-13, gpt-4o-search-preview, gpt-4o-mini-tts-2025-12-15, gpt-image-1, gpt-audio, gpt-5.3-codex, gpt-5-search-api-2025-10-14, gpt-5.4-pro, gpt-4o-mini-tts, gpt-realtime-mini-2025-12-15, gpt-5.1-codex-max, sora-2, gpt-4o, gpt-4o-mini-2024-07-18, gpt-4.1-2025-04-14, gpt-5, gpt-4o-mini-transcribe, gpt-image-2-2026-04-21, gpt-5.2-chat-latest, sora-2-pro, gpt-3.5-turbo-instruct, gpt-transcribe, gpt-4.1-nano-2025-04-14, gpt-3.5-turbo-1106, gpt-4o-mini-tts-2025-03-20, gpt-5.2, gpt-5.4-nano-2026-03-17, o1, tts-1-hd-1106, gpt-5.2-codex, gpt-5.4-mini, o1-pro, gpt-5.1-codex, o4-mini-deep-research, gpt-5.4, gpt-4o-search-preview-2025-03-11, text-embedding-3-small, gpt-5-nano-2025-08-07, gpt-4o-transcribe, o4-mini, gpt-5.1-codex-mini, gpt-6-astra, gpt-5.3-chat-latest, o3-2025-04-16, gpt-5.2-2025-12-11, chatgpt-image-latest, gpt-3.5-turbo, gpt-image-2.5-sunburst-2026-09-08, gpt-image-2.5-flare, davinci-002, gpt-5.4-nano, o3-mini, o4-mini-deep-research-2025-06-26, gpt-audio-mini-2025-12-15, o1-pro-2025-03-19, gpt-live-1, gpt-image-2.5-sunburst, gpt-image-2.5-flare-2026-09-08, gpt-6.1-terra, gpt-audio-mini, gpt-4.1-mini-2025-04-14, gpt-6-luna, gpt-4o-mini-search-preview, gpt-6-sol, gpt-6.1-sol

=== Get Anthropic Models ===
Anthropic models: claude-haiku-5-5, claude-sonnet-5-5, claude-opus-5-5, claude-fable-5-1, claude-opus-5, claude-sonnet-5, claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 404: {"error":"Unknown provider 'nonexistent_provider_xyz'","error_kind":"unknown_provider"}

✓ Chat Models example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: SP5n20tjsam_1Ba6qoMynmqwtRu2-uzvCCEKEdPBb9sKV7mRJf8qC3pcDkdwELQtXPTEaKkRKSwh4iZJSMkkBA

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is **ekoDB**, described as a high-performance database product. Its listed price is **$99**.

✓ Message 2 sent
  Response: ekoDB is priced at **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: N1NWqQbHBe2qBIPUnHGit_SndMn-6Lh4DbwEMIlffryFH5e_f-4wKqnUOyv1Ti8nt2mSDEQKpi27k_U8t0-pPA
  Parent: SP5n20tjsam_1Ba6qoMynmqwtRu2-uzvCCEKEdPBb9sKV7mRJf8qC3pcDkdwELQtXPTEaKkRKSwh4iZJSMkkBA

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: N1NWqQbHBe2qBIPUnHGit_SndMn-6Lh4DbwEMIlffryFH5e_f-4wKqnUOyv1Ti8nt2mSDEQKpi27k_U8t0-pPA (Untitled)
  Session 2: SP5n20tjsam_1Ba6qoMynmqwtRu2-uzvCCEKEdPBb9sKV7mRJf8qC3pcDkdwELQtXPTEaKkRKSwh4iZJSMkkBA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: N1NWqQbHBe2qBIPUnHGit_SndMn-6Lh4DbwEMIlffryFH5e_f-4wKqnUOyv1Ti8nt2mSDEQKpi27k_U8t0-pPA

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: hzoi3T7T0T6jTzfoPgUlWGxZi1DPWfDTvDvhrXCQGmI0w8r7IyMu5QZGOilasnrIyxftouxvPnRytUKJ_eo_lA

=== List Collections ===
Total collections: 23
Sample collections: schema_users_client_go,audit__ek0_testing,schema_documents_client_ts,schema_employees_client_ts,schema_employees_client_go

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: GpbIz6IQo9e_QI1NhqdGGwtMcGHcahLnmsxj-oFPgeWErUopqESGO-nZR1gTFz_32v2TyteU4VmQL2CfZJoYkA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: QhwwkTRcaQmgGcBqCHdn-UnWg6BGesNCf4twNn9n4ILcCyxZKuYDa0GIUUZ5Um6FS_fbYq6DiSuCZl2BlK4zDg

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
=== ekoDB as Edge Cache - Simple Example ===

Creating edge cache function...
✓ Edge cache script created: 4tHCebwI09JC0f6HZk3FjrX8HAmGucM-GA4T8Cb1LqaeHh9zqMJQHchu9xb4_GpNcP-4AK-Cs4sSaLSJW4n3lA

Call 1: Cache miss (fetches from API)
Response time: 165ms
Result: {
  "records": [
    {
      "value": {
        "value": {
          "generationtime_ms": 0.033020973205566406,
          "latitude": 40.710335,
          "timezone": "GMT",
          "timezone_abbreviation": "GMT",
          "current": {
            "time": "2026-11-02T04:45",
            "interval": 900,
            "temperature_2m": 8.4
          },
          "current_units": {
            "temperature_2m": "°C",
            "interval": "seconds",
            "time": "iso8601"
          },
          "elevation": 32,
          "longitude": -73.99308,
          "utc_offset_seconds": 0
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
Response time: 4ms (41.3x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "generationtime_ms": 0.033020973205566406,
          "latitude": 40.710335,
          "timezone": "GMT",
          "timezone_abbreviation": "GMT",
          "current": {
            "time": "2026-11-02T04:45",
            "interval": 900,
            "temperature_2m": 8.4
          },
          "current_units": {
            "temperature_2m": "°C",
            "interval": "seconds",
            "time": "iso8601"
          },
          "elevation": 32,
          "longitude": -73.99308,
          "utc_offset_seconds": 0
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
   ⏱️  Duration: 76ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "id": 1,
      "email": "Sincere@april.biz",
      "website": "hildegard.org",
      "name": "Leanne Graham",
      "username": "Bret",
      ...

Second call (cache hit - from cache):
   ⏱️  Duration: 2ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "id": 1,
      "email": "Sincere@april.biz",
      "website": "hildegard.org",
      "name": "Leanne Graham",
      "username": "Bret",
      "address": {
        "...
   🚀 Cache speedup: 38.0x faster!

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
🚀 ekoDB Functions Example (JavaScript Client)

✅ Client initialized (token exchange automatic)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: KXAiHTiUxBuvnAdctdezHQQdx5ilDOysMg6KzqIoM619SOQLOLtcg-RptDRtVT-xYaEE6PqC2Msf7viMu1KkSw
📊 Found 5 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

✅ Function saved
📊 Found 3 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 2 groups
   {"status":{"value":"inactive","type":"String"},"count":{"value":5,"type":"Integer"},"avg_score":{"type":"Float","value":50}}
   {"count":{"type":"Integer","value":5},"status":{"type":"String","value":"active"},"avg_score":{"type":"Float","value":60}}
⏱️  Execution time: 0ms

📝 Example 4: Function Management

📋 Total functions: 7
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

✅ All examples completed successfully!

💡 Key Advantages of Using the Client:
   • Automatic token management
   • Type-safe Stage builders
   • ChatMessage helpers
   • Cleaner, more maintainable code
   • Built-in error handling
🧹 Cleaning up...
✅ Deleted collection
✅ Deleted test functions

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
   Furniture:
      Products: 4 | Stock: 43 | Avg Rating: ⭐4.26
   Electronics:
      Products: 6 | Stock: 232 | Avg Rating: ⭐4.52

⏱️  Total execution time: 0ms
📈 Pipeline stages:

📝 Example 5: Project Specific Fields

✅ Function saved
📊 Product summaries (10 items, showing first 3):
   1. Bookshelf - $149 (⭐4.1)
   2. Webcam HD - $119 (⭐4.5)
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
   Vector databases are designed to store and search **embeddings**—numeric representations of text, images, audio, or other data. Their main benefits include:

- **Semantic search:** Find items with similar meaning or content, even when they use different wording.
- **Fast similarity search at scale:** Specialized indexes make nearest-neighbor searches practical across large collections.
- **Support for AI applications:** They provide a foundation for recommendation systems, image and document search, and retrieval-augmented generation (RAG).
- **Multimodal search:** Compare embeddings from different media types, such as matching a text query to images.
- **Filtering and hybrid search:** Combine vector similarity with metadata filters or keyword search to improve relevance.
- **Production-ready operations:** Many offer persistence, indexing, updates, and distributed scaling for applications.

They complement rather than replace relational databases: use a vector database when similarity search is central, and a conventional database for structured data, transactions, and exact queries.
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
🚀 ekoDB JavaScript Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: n8W3X0ZC19P-DF0IBHW3_94_Zi3H0OntNb4sv3IIGu2m_97VSfExuaNrzl9FooFXFMIGOLEImd-HU9lnLwJbZg
📊 Found 2 product groups
   {"count":{"type":"Integer","value":2},"category":{"value":"Furniture","type":"String"},"avg_price":{"value":474,"type":"Float"}}
   {"avg_price":{"value":575.6666666666666,"type":"Float"},"category":{"value":"Electronics","type":"String"},"count":{"value":3,"type":"Integer"}}
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
   1. User 4 - Score: 40
   2. User 1 - Score: 10
   3. User 5 - Score: 50
   4. User 9 - Score: 90
   5. User 3 - Score: 30
⏱️  Execution time: 0ms

📝 Example 5: User Summary Statistics

✅ Function saved
📊 User summary (2 groups):
   active: 7 users, Total Score: 370
   inactive: 3 users, Total Score: 180
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD function examples completed!
🚀 ekoDB JavaScript KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: dYmoVH-N4fVnScpEArrizpY0QO_WWlltM4TlStRnhw3SuXXUVbPoiIOLFYVccN7ocJSVsiWJz0Vps9PTlDAiyA
✅ Inserted 2 products with wrapped types

📝 Example 2: Function with Wrapped Type Parameters

✅ Function saved: PmeotJbySJXvUL3n7HpQ1b6ddNq6TLS-cNSVyWp7GY8VFVEbv8yAt_iQfftrho952nM6C0Umm55FUrgSYtLrbg
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"value":{"userId":"user_abc","role":"admin"},"type":"Object"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: H0faZwkr0r_ph5u7RRPYF9PItRx9qoq7ygHTHJdk31rbQU_KXl8mnaSN6z88UPlU7kvtcbXCTblJTRfp868uhw
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: Zo0gIGWsoE-dhwQqLCSRVoIWzzPTSEtLjirMpkDUwK6JMLfhsmP1FntbkTOtL8hEs94p1o1KTtiFrk1ECV9Ibg
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
🚀 ekoDB Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Getting Started with ekoDB (Database)
   2. Database Design Principles (Database)
   3. Vector Databases Explained (Database)
   4. Natural Language Processing (AI)
   5. Introduction to Machine Learning (AI)
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
   1. Getting Started with ekoDB
   2. Database Design Principles
   3. Vector Databases Explained
   4. Natural Language Processing
   5. Introduction to Machine Learning
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Function saved
📊 Projected documents (showing first 3):
   1. Getting Started with ekoDB
   2. Database Design Principles
   3. Vector Databases Explained
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Function saved
📊 All documents (5 total, showing first 2):
   1. Getting Started with ekoDB (Database)
   2. Database Design Principles (Database)
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search function examples completed!
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

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: { value: { userId: 123, username: 'john_doe' }, type: 'Object' }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { value: { price: 29.99, name: 'Product 1' }, type: 'Object' }
cache:product:2: {
  type: 'Object',
  value: { name: 'Product 2', price: 39.989999999999995 }
}
cache:product:3: {
  value: { name: 'Product 3', price: 49.989999999999995 },
  type: 'Object'
}

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
=== Search Examples ===

Setting up test data...
✅ Test data created

1. Basic full-text search:
Found 2 results
  1. Score: 12.870, Matched: name, email
  2. Score: 6.270, Matched: name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 13.200, Matched: bio, title
  2. Score: 13.200, Matched: bio, title
  3. Score: 13.200, Matched: bio, title
  4. Score: 13.200, Matched: bio, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400, Matched: title, bio
  2. Score: 26.400, Matched: title, bio
  3. Score: 26.400, Matched: title, bio
  4. Score: 26.400, Matched: title, bio

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
  2. Score: 0.741, Matched:
  3. Score: 0.739, Matched:

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.496, Matched: content, title
  2. Score: 0.896, Matched: content, title
  3. Score: 0.298, Matched:

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: skills, bio

=== Cleanup ===
✅ Deleted test collections

✅ Search examples completed!
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'eEOKE27vPhELGoDsG6X9-7JcKAnisWkBEdNATe6SJx94ShljTUUPN7omoOnRq7Hp0J4CYdYy98YqSIuDPs3gvw'
}

=== Find by ID ===
Found: {
  name: { value: 'Test Record', type: 'String' },
  id: 'eEOKE27vPhELGoDsG6X9-7JcKAnisWkBEdNATe6SJx94ShljTUUPN7omoOnRq7Hp0J4CYdYy98YqSIuDPs3gvw',
  active: { type: 'Boolean', value: true },
  value: { value: 42, type: 'Integer' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  active: { type: 'Boolean', value: true },
  name: { value: 'Updated Record', type: 'String' },
  id: 'eEOKE27vPhELGoDsG6X9-7JcKAnisWkBEdNATe6SJx94ShljTUUPN7omoOnRq7Hp0J4CYdYy98YqSIuDPs3gvw',
  value: { value: 100, type: 'Integer' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: ImPWUT9whE-xY49Uf8Hoqhz7dD4kxSVxmzu6vPK78jdUUv5K_QBVEBwpcsp8fUtaqQ5B_2EPYjlRY3XiM_XPpg

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
🚀 ekoDB JavaScript Client - Native SWR Function Examples

📋 Demonstrates:
   • Single-function SWR pattern (replaces 4-step pipeline)
   • Automatic cache checking, HTTP fetching, and cache setting
   • Built-in audit trail support
   • Duration string TTLs ('15m', '1h', '30s')
   • Multi-function pipeline integration
   • Dynamic TTL configuration


🧹 Cleaning up...
✓ Deleted 0 test functions and owned SWR resources

Example 1: Basic Native SWR
────────────────────────────────────────────────────────────────────────────────
Single function replaces KvGet → If → HttpRequest → KvSet pipeline
✓ Created native SWR script: github_user_native_js (25oeZPFC1PCyKp2BZl2nWLj9jsvXVrCzT6MHeD0ku21-pYkRi2wt_JAccpcRNdgzf36L0ZZQEQCJ2CSDlU_mJA)

First call (cache miss - will fetch from GitHub API):
  Response time: 125ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 3ms
  Speedup: 41.7x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_js (Y7GZCu5ff154RdqsezFrj1da34RI1kxq-rucxtqi8lecPbiRegg-KMoxxmhyxxDhLkD6KZwoRHOes8c0g6KaIQ)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_js' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_js (VUicGH4RnDOyLyHXDwatRNV--brFdunOKGf6IcEGyGQ8mDNtLW6RBrUzmgRH-Hwi5mk_S-LHFv-HHBS1VKxzqA)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_js' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_js (RV17JEDpOhuDf1kqNjubO1Xkb82MHSQN5Zy59Cbe2yZK7k-GKw_Hh8l0kRiCE0ukO4E4Ci3QroroXv-tP_EoTQ)
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
✓ Deleted 4 test functions and owned SWR resources

✅ All examples completed!
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_js_70662_1793594795441 (kNl2d-Kcdn15dDRjVkxk-zCcD9yAeVJ_8Q1ckQm0TL-Gsv_ZWmQ6pVGkJGbNZsZXDC4_ZddqpSuozEKxkhHbig)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "address": {
            "suite": "Apt. 556",
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "zipcode": "92998-3874",
            "street": "Kulas Light"
          },
          "email": "Sincere@april.biz",
          "name": "Leanne Graham",
          "website": "hildegard.org",
          "username": "Bret",
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "phone": "1-770-736-8031 x56442",
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
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 5ms (served from cache)
✓ Lightning fast cache hit

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
🧹 Cleaning up...
✓ Cleanup complete

✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: d0Jhm67xK0UHRh7Xy1KRaugM4ZWwCuYwP690979EnXUBKJrWpPNmeh0VIP8lOrE130dUEOfGPrQ2WlWyLwO3oA
Created Bob: $500 - ID: xN6CsFzr9B-eMZ1k5ulVhZ-II4ISj-7pCrasqbdRWBHl1pj9MvTVTrjT7ElfO-hoHzs4tXwWfFm_GSnhtEsRwg

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 5c51a5c7-20f4-46b0-a1c3-d03786a4ece3

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
New transaction: 485985e8-1e04-4253-8782-81741b27e545
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: 2TGW6ti0lU5pcbNAzWdDgStnTYouuLLpzrwZ8zmtJp1Tpcc4bmGFdJ_MfvpM-qY_ZXCqx4NdmCRf70lRsrfWbw

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
Admin permissions read successfully
=== ekoDB Advanced CRUD Example (TypeScript) ===

--- Inserting base record ---
Inserted: KIHSjU9_0XJPaTNuGdz95PqvcKlN0iMpyMpXUf1lp9nPDgPuFk1dJCjD2NW3KIeLROKqEAjcAjWGl8WeumU5Tg

--- updateWithAction: increment ---
Score after increment: 150

--- updateWithAction: push ---
Tags after push: ["beginner","pro"]

--- updateWithAction: clear ---
temp_data after clear: null

--- updateWithActionSequence: multiple atomic actions ---
After sequence - score: 160, lives: 2, tags: ["beginner","pro","veteran"]

--- Cleanup ---
Deleted collection

=== All advanced CRUD operations completed ===
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
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: ck8F-npLlDLaTH5LoxgN7rzWiNwl-pet2BKBoennreGFI5FxLqYoSYaFTqGgyJfWEXwQaljwvUyq0m7tYnpWbw

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is **ekoDB** — a high-performance database product priced at **$99**.

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: ekoDB is priced at **$99**.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: Tx0nLywYAGogDZLzfvhRCfFxT_fodrCBXKB8w35YEEodAhMQhh7X-mch0Fw5xrbKJ4uImgTLmjjC2V95ARkOcg
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: Tx0nLywYAGogDZLzfvhRCfFxT_fodrCBXKB8w35YEEodAhMQhh7X-mch0Fw5xrbKJ4uImgTLmjjC2V95ARkOcg
✓ Deleted chat session: ck8F-npLlDLaTH5LoxgN7rzWiNwl-pet2BKBoennreGFI5FxLqYoSYaFTqGgyJfWEXwQaljwvUyq0m7tYnpWbw
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: NQD8V_zjQ2jKyJkMNrfNYxYhwl9MIyM_n2gCMlnUQWCA_bJyoBvbiJC40PlMszfplyhY1MsZ-YHQIaU8Qn94ew

=== Sending Chat Message ===
Message ID: U_FGNHREy3YI02_8px32QfLFrCzuEzvkq7nmuGtqxyRb2sfGh_DtNSXOFPm2PN5lcuCDD8v6prqmicYuo7JIdw

=== AI Response ===
- **ekoDB Cloud** — 499
- **ekoDB Pro** — 299
- **ekoDB** — 99

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 499,
    description: 'Fully managed cloud database service product',
    id: 'DPpsBiL3gylnWTztRIIDvSZHx7baEXKw3xCI65jzUhiJqXrMfGHTj4gbvOcn0sN0H656Vb_rO1fRzcZbzRzn8g',
    name: 'ekoDB Cloud'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB Pro',
    price: 299,
    description: 'Enterprise edition product with advanced features',
    id: 'IK8hP_eYedD5WT5sFVTZq85EOM0TkoQ98dbNGXBw9cVEwXXcn6snWFtXZ__FjQSfPP0z35DsjNOZ3_yXno69NA'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'p5PcwtKR6x4NXLMl1TVO0qZEPudWIE74N7pRHy0yENM1nQnviEZQaEYbEAmPsHSq2C3_YFN1MF1v0bqzRkrH0g',
    price: 99,
    name: 'ekoDB',
    description: 'A high-performance database product with AI capabilities'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 1678ms

=== Token Usage ===
Prompt tokens: 13716
Completion tokens: 32
Total tokens: 13748

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Message Stream (SSE) Example (TypeScript) ===

Created session: xFKV2VgP6BbI9PiFnzJka9Z8qYeSzlSnmhFqST_em7wZGsDLPeOucLjpGzOs40W_noyD930ohMF1LGByCoYB1g

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"gpt-6-luna","provider":"openai"}“ekoDB” isn’t a standard database category, and the name may refer to different products or projects. If you share where you saw it—a link, app, or article—I can explain that specific ekoDB.

--- Stream complete ---
Message ID: RafUui2O2cyM2vYANaeP4N-gJ5AR6HgXenzk2eVUo_vP5Kh5CabtxhUF1g_TUSFDNabYhv8VanNBeWM0eyTmxQ
Execution time: 13294ms
Context window: 128000 tokens

✓ Chat message stream example completed
✓ Client created

=== List Chat Models ===
Available chat models by provider:
  openai:
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
  anthropic:
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
  gemini:
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

=== Get Specific Provider Models ===
OpenAI models: gpt-3.5-turbo-0125, gpt-5.4-2026-03-05, gpt-5-nano, gpt-5.5-pro, gpt-audio-2025-08-28, gpt-4o-mini-transcribe-2025-03-20, gpt-4-turbo-2024-04-09, gpt-realtime-2.1-mini, gpt-realtime-2025-08-28, gpt-4, tts-1-hd, gpt-realtime-1.5, gpt-realtime, gpt-5.2-pro, gpt-5-chat-latest, gpt-5.4-mini-2026-03-17, babbage-002, gpt-4o-2024-05-13, text-embedding-3-large, tts-1, omni-moderation-2024-09-26, gpt-live-transcribe, gpt-5.6-luna, gpt-5-2025-08-07, gpt-5-mini, gpt-5-mini-2025-08-07, omni-moderation-latest, o3-mini-2025-01-31, gpt-audio-1.5, gpt-image-1.5, tts-1-1106, gpt-5.5-pro-2026-04-23, gpt-audio-mini-2025-10-06, gpt-5.2-pro-2025-12-11, o4-mini-2025-04-16, gpt-4o-2024-11-20, gpt-5.1, whisper-1, gpt-5-pro, gpt-realtime-whisper, gpt-realtime-2, gpt-5-search-api, gpt-5.5-2026-04-23, gpt-4.1-nano, gpt-image-2, gpt-3.5-turbo-16k, text-embedding-ada-002, gpt-realtime-2.1, gpt-4.1, gpt-5.5, chat-latest, gpt-realtime-translate, gpt-5.4-pro-2026-03-05, gpt-4.1-mini, gpt-4o-mini, gpt-4o-mini-transcribe-2025-12-15, gpt-realtime-mini, gpt-image-1-mini, gpt-5.6-terra, gpt-3.5-turbo-instruct-0914, gpt-4-turbo, gpt-4o-2024-08-06, o3, gpt-4o-mini-search-preview-2025-03-11, o1-2024-12-17, gpt-5.1-chat-latest, gpt-5-pro-2025-10-06, gpt-5.6-sol, gpt-4o-transcribe-diarize, gpt-5-codex, gpt-4-0613, gpt-5.1-2025-11-13, gpt-4o-search-preview, gpt-4o-mini-tts-2025-12-15, gpt-image-1, gpt-audio, gpt-5.3-codex, gpt-5-search-api-2025-10-14, gpt-5.4-pro, gpt-4o-mini-tts, gpt-realtime-mini-2025-12-15, gpt-5.1-codex-max, sora-2, gpt-4o, gpt-4o-mini-2024-07-18, gpt-4.1-2025-04-14, gpt-5, gpt-4o-mini-transcribe, gpt-image-2-2026-04-21, gpt-5.2-chat-latest, sora-2-pro, gpt-3.5-turbo-instruct, gpt-transcribe, gpt-4.1-nano-2025-04-14, gpt-3.5-turbo-1106, gpt-4o-mini-tts-2025-03-20, gpt-5.2, gpt-5.4-nano-2026-03-17, o1, tts-1-hd-1106, gpt-5.2-codex, gpt-5.4-mini, o1-pro, gpt-5.1-codex, o4-mini-deep-research, gpt-5.4, gpt-4o-search-preview-2025-03-11, text-embedding-3-small, gpt-5-nano-2025-08-07, gpt-4o-transcribe, o4-mini, gpt-5.1-codex-mini, gpt-6-astra, gpt-5.3-chat-latest, o3-2025-04-16, gpt-5.2-2025-12-11, chatgpt-image-latest, gpt-3.5-turbo, gpt-image-2.5-sunburst-2026-09-08, gpt-image-2.5-flare, davinci-002, gpt-5.4-nano, o3-mini, o4-mini-deep-research-2025-06-26, gpt-audio-mini-2025-12-15, o1-pro-2025-03-19, gpt-live-1, gpt-image-2.5-sunburst, gpt-image-2.5-flare-2026-09-08, gpt-6.1-terra, gpt-audio-mini, gpt-4.1-mini-2025-04-14, gpt-6-luna, gpt-4o-mini-search-preview, gpt-6-sol, gpt-6.1-sol

=== Get Anthropic Models ===
Anthropic models: claude-haiku-5-5, claude-sonnet-5-5, claude-opus-5-5, claude-fable-5-1, claude-opus-5, claude-sonnet-5, claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 404: {"error":"Unknown provider 'nonexistent_provider_xyz'","error_kind":"unknown_provider"}

✓ Chat Models example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 37ZyDVEyQ3lYvfAQlI-o7jp5TgZHb5-AzcnzeN7e8lnqZEF2FL1uPWLJkeyIsts4ZHApga1liE9iCn7GfbG-qg

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is **ekoDB** — a high-performance database product, priced at **$99**.

✓ Message 2 sent
  Response: ekoDB costs **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: K6CFFsIMDaPwC3vFTmP90fIBWbJ6vMbD9zhA4dFAElJz2u3DPJ9Wi3YyqS-4c7vO97KQnm4hM9c4im3eKP_ljg
  Parent: 37ZyDVEyQ3lYvfAQlI-o7jp5TgZHb5-AzcnzeN7e8lnqZEF2FL1uPWLJkeyIsts4ZHApga1liE9iCn7GfbG-qg

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: K6CFFsIMDaPwC3vFTmP90fIBWbJ6vMbD9zhA4dFAElJz2u3DPJ9Wi3YyqS-4c7vO97KQnm4hM9c4im3eKP_ljg (Untitled)
  Session 2: 37ZyDVEyQ3lYvfAQlI-o7jp5TgZHb5-AzcnzeN7e8lnqZEF2FL1uPWLJkeyIsts4ZHApga1liE9iCn7GfbG-qg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: K6CFFsIMDaPwC3vFTmP90fIBWbJ6vMbD9zhA4dFAElJz2u3DPJ9Wi3YyqS-4c7vO97KQnm4hM9c4im3eKP_ljg

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: C0FfhgKUWSOkZdGbaKfuvGPvBMb5YGHyD3UuIojrWfP-koGol4X7c3DNcg2PlB1l2afCAPyae23ICcctXKpsDA

=== List Collections ===
Total collections: 27
Sample collections: schema_users_client_go,audit__ek0_testing,schema_documents_client_js,schema_documents_client_ts,schema_employees_client_ts

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
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
✓ conc_demo_pay saved
✓ conc_demo_rl_fail saved
✓ conc_demo_rl_skip saved
✓ conc_demo_lock saved

Invoke them like:
  POST /api/functions/conc_demo_pay_ts_76798_1793594830125 { "idempotency_key": "...", "amount": 100 }
  POST /api/functions/conc_demo_rl_fail_ts_76798_1793594830125 { "user_id": 42 }
  POST /api/functions/conc_demo_rl_skip_ts_76798_1793594830125 { "user_id": 42 }
  POST /api/functions/conc_demo_lock_ts_76798_1793594830125 { "resource": "queue:drain" }

✓ Cleaned up demo functions
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'R-W7LMfk9dPUiKaaxoIRInZmJGONFlvUTPKZWB4tzHz50YCAAdgQ61sVIYeENhdFAkhPPa4YqnrKHnHNydjPaA'
}

=== Upsert Operation ===
✓ First upsert (update): {
  name: { type: 'String', value: 'Alice Johnson' },
  age: { type: 'Integer', value: 29 },
  id: 'R-W7LMfk9dPUiKaaxoIRInZmJGONFlvUTPKZWB4tzHz50YCAAdgQ61sVIYeENhdFAkhPPa4YqnrKHnHNydjPaA',
  email: { value: 'alice.j@newdomain.com', type: 'String' },
  active: { type: 'Boolean', value: true }
}
✓ Second upsert (insert): { id: 'new-user-id' }

=== Find One Operation ===
✓ Found user by email: {
  email: { value: 'alice.j@newdomain.com', type: 'String' },
  name: { type: 'String', value: 'Alice Johnson' },
  age: { value: 29, type: 'Integer' },
  active: { type: 'Boolean', value: true },
  id: 'R-W7LMfk9dPUiKaaxoIRInZmJGONFlvUTPKZWB4tzHz50YCAAdgQ61sVIYeENhdFAkhPPa4YqnrKHnHNydjPaA'
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
✓ Client created
✓ crypto_demo_hmac_ts saved
✓ crypto_demo_aes_ts saved
✓ crypto_demo_uuid_ts saved
✓ crypto_demo_totp_ts saved
✓ crypto_demo_encoding_ts saved

Invoke them with:
  POST /api/functions/crypto_demo_hmac_ts { "payload": "hi" }
  POST /api/functions/crypto_demo_aes_ts { "plaintext": "secret" }
  POST /api/functions/crypto_demo_uuid_ts
  POST /api/functions/crypto_demo_totp_ts
  POST /api/functions/crypto_demo_encoding_ts { "title": "Héllo World" }

✓ Cleaned up demo functions
=== Distinct Values Example ===

Inserting sample products...
Inserted 8 products

=== Distinct Categories (all products) ===
Found 3 distinct categories:
  - books
  - clothing
  - electronics

=== Distinct Statuses (all products) ===
Found 3 distinct statuses:
  - active
  - archived
  - discontinued

=== Distinct Statuses in Electronics ===
Found 2 distinct statuses for electronics:
  - active
  - discontinued

Cleanup done.
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: W2OSiQAV_frj_pANWSvxpqisGwm7zk9KSCI0FUAXj4WZXrqQOA1fxLvH2MTkxzTuGnpK-T5ubRI1QBUZHJCeBQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: tRSJ70k-L9lBJCXQczWgiwqAo4nGdTbmk05-W91Z0TNt_C4OtWH036j9I-NmM6ijzzVmhkPnTAaojzb_DlZv2g

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
=== ekoDB as Edge Cache - Simple Example ===

Creating edge cache function...
✓ Edge cache script created: USs_jv7_74UW4wfvfgGclmUYIJjaTWn50X4geZo-p6KUXwi1tzLuIYa0JtM74t8dlvYgVqnPW2oh7I17xArIBw

Call 1: Cache miss (fetches from API)
Response time: 71ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets"
          },
          "website": "hildegard.org",
          "id": 1,
          "username": "Bret",
          "name": "Leanne Graham",
          "phone": "1-770-736-8031 x56442",
          "email": "Sincere@april.biz",
          "address": {
            "zipcode": "92998-3874",
            "street": "Kulas Light",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "suite": "Apt. 556",
            "city": "Gwenborough"
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

Call 2: Cache hit (served from ekoDB)
Response time: 3ms (23.7x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets"
          },
          "website": "hildegard.org",
          "id": 1,
          "username": "Bret",
          "name": "Leanne Graham",
          "phone": "1-770-736-8031 x56442",
          "email": "Sincere@april.biz",
          "address": {
            "zipcode": "92998-3874",
            "street": "Kulas Light",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "suite": "Apt. 556",
            "city": "Gwenborough"
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

=== The Magic ===
- Your DATABASE is your EDGE
- No Redis needed
- No CDN needed
- No cache invalidation logic needed (TTL handles it)
- With ripples: All nodes auto-sync cache
- One service: Database + Cache + Edge Functions

✓ Example complete!

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
   ⏱️  Duration: 72ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "email": "Sincere@april.biz",
      "phone": "1-770-736-8031 x56442",
      "username": "Bret",
      "id": 1,
      "address": {
        "geo": {
          "lng": ...

Second call (cache hit - from cache):
   ⏱️  Duration: 3ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "email": "Sincere@april.biz",
      "phone": "1-770-736-8031 x56442",
      "username": "Bret",
      "id": 1,
      "address": {
        "geo": {
          "lng": ...
   🚀 Cache speedup: 24.0x faster!

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
client_function_contract: ok
🚀 ekoDB Functions Example (TypeScript)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

ℹ️  Function 'get_active_users_client_ts' already existed — updated instead
✅ Function saved: xOrKhF-29hy2j8GEvAiPGODiNV-o7jUwiLi9usRTD-59J9DnAFKeJr8VtSJ-YH505bSSHns4al5mbipcfK3Qqw
📊 Found 5 active users

📝 Example 2: Parameterized Function

✅ Function saved: 3_01AZ0Pyh8a0U5x83lXqIDLhINiLARVukxQ6ZQtYkHxk7Dy5gmRDS7cTcuQfp2CXvjsM8gWzk_5Cyti5lZA3Q
📊 Found 3 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: Vzyd-sfTt6w4LgaaVMjSPCjfB-HhANeQgDaMC0ljgUmntMjw_QHk5jS5XTXdwLyYoMJD9StTRX0_pJLXVMg6cw
📊 Statistics: 2 groups
   {"avg_score":{"value":60,"type":"Float"},"count":{"value":5,"type":"Integer"},"status":{"type":"String","value":"active"}}
   {"count":{"type":"Integer","value":5},"avg_score":{"value":50,"type":"Float"},"status":{"value":"inactive","type":"String"}}

📝 Example 4: UserFunction Management

📋 Total scripts: 9
🔍 Retrieved script: Get Active Users
✏️  function updated
🗑️  function deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

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
   {"avg_price":{"value":365.6666666666667,"type":"Float"},"count":{"type":"Integer","value":3},"category":{"value":"Furniture","type":"String"}}
   {"count":{"value":5,"type":"Integer"},"category":{"value":"Electronics","type":"String"},"avg_price":{"value":367,"type":"Float"}}
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
   Vector databases are designed to store and search **embeddings**—numeric representations of text, images, audio, or other data. Their main benefits are:

- **Meaning-based search:** Find items that are conceptually similar, even when they don’t share exact keywords.
- **Fast search at scale:** Specialized indexes make nearest-neighbor searches practical across large collections.
- **Richer retrieval:** Combine similarity search with metadata filters, such as date, category, or access permissions.
- **Useful for AI applications:** They can retrieve relevant information for recommendation systems, image search, and retrieval-augmented generation (RAG).
- **Flexible data types:** The same approach can support embeddings from different kinds of content.

They’re most useful when similarity matters more than exact matching; traditional databases are often better for structured queries and exact lookups.
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
🚀 ekoDB TypeScript Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: 6GsLNMCAl6jOXKEv80L1QUWAuoKJ_FWI-8insFgiTAshcX2XJkDQq0YKpRvY5QtZojpWQJaarhctotT7h15eyw
📊 Found 2 product groups
   {"category":{"type":"String","value":"Furniture"},"avg_price":{"type":"Float","value":474},"count":{"type":"Integer","value":2}}
   {"category":{"value":"Electronics","type":"String"},"count":{"type":"Integer","value":3},"avg_price":{"value":575.6666666666666,"type":"Float"}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"category":{"type":"String","value":"Furniture"},"count":{"value":2,"type":"Integer"}}
   {"category":{"value":"Electronics","type":"String"},"count":{"type":"Integer","value":3}}
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
   {"avg_score":{"value":20,"type":"Float"},"count":{"type":"Integer","value":3},"role":{"type":"String","value":"admin"}}
   {"avg_score":{"type":"Float","value":70},"count":{"value":7,"type":"Integer"},"role":{"value":"user","type":"String"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
🚀 ekoDB TypeScript KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: 5EX8QAz8vPCG2JogWTHuRGRwelfmz1iEhS5Wf0kFVLt9YcDTA0kGCPcNZpvxfOPv5ODsCjWtOLhDkrQArt81wg
✅ Inserted 2 products with wrapped types

📝 Example 2: UserFunction with Wrapped Type Parameters

✅ Function saved: A8xeCPIoNiUZyeLWMQj7tMgwk3Q87KpqeRqZ0saPu2h-8mp4Q_8iWTYvJ2COdgbLDnPCHJb7EVq_DLet8HsIjw
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"type":"Object","value":{"userId":"user_abc","role":"admin"}}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: M74kiCeCNLcJJfjwIK96nlvha5nns7kebirxr5Gep7pbz0-hpJ0RoASfwP3lJ3UFH1M3x-iPvHvzXtwe3Hr-mw
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: G3KMjsT7mK-KiL0ih7l3xH2gHIlAd02fxqQzzITKnbd1Uv00SO74ptOPkmyoD5skEmjGepgogjwDmKimSUqTKQ
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
🚀 ekoDB TypeScript Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Database Design Principles (Database)
   2. Natural Language Processing (AI)
   3. Getting Started with ekoDB (Database)
   4. Introduction to Machine Learning (AI)
   5. Vector Databases Explained (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"category":{"type":"String","value":"AI"},"count":{"value":2,"type":"Integer"}}
   {"category":{"value":"Database","type":"String"},"count":{"type":"Integer","value":3}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Goal Template CRUD Example (TypeScript) ===

--- Creating goal template ---
Created template: Data Migration (id: DJr7eNxJP8cnt2wkaMzO2QvxD7Le7f-8b5McaZbWzwYfQWAhmZentJk9JVHbB80lSv6oU2OhFG1FwJvvryoRPQ)

--- Listing templates ---
Templates: {
  count: 1,
  items: [
    {
      description: [Object],
      id: 'DJr7eNxJP8cnt2wkaMzO2QvxD7Le7f-8b5McaZbWzwYfQWAhmZentJk9JVHbB80lSv6oU2OhFG1FwJvvryoRPQ',
      owner: [Object],
      steps: [Object],
      title: [Object]
    }
  ],
  total: 1
}

--- Getting template ---
Fetched: Data Migration

--- Updating template ---
Updated description: Updated: comprehensive data migration workflow

--- Deleting template ---
Template deleted successfully

✓ Goal template CRUD example completed
=== ekoDB Goals, Tasks & Agents Example (TypeScript) ===

--- Creating goal ---
Created goal: Deploy v2.0 (id: 4OV3E9QO4PNEmSxSCsXNFWxOmz04AdcPaMMsr6Ly_NkE6ruYGHvKwC6zv9nx3lkl6x60-qU3TJ5uAwtGt-5GOA)

--- Listing goals ---
Goals: {
  "count": 1,
  "goals": [
    {
      "created_at": {
        "type": "DateTime",
        "value": "2026-11-02T04:47:17.319877+00:00"
      },
      "description": {
        "type": "String",
        "value": "Ship version 2.0 to production"
      },
      "id": "4OV3E9QO4PNEmSxSCsXNFWxOmz04AdcPaMMsr6Ly_NkE6ruYGHvKwC6zv9nx3lkl6x60-qU3TJ5uAwtGt-5GOA",
      "owner": {
        "type": "Object",
        "value": {
          "key_id": "3a0cdba0da7c060610042783cc758f1f",
          "key_label": "default_admin"
        }
      },
      "status": {
        "type": "String",
        "value": "pending"
      },
      "steps": {
        "type": "String",
        "value": "[{\"description\":\"Run test suite\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"
      },
      "title": {
        "type": "String",
        "value": "Deploy v2.0"
      },
      "updated_at": {
        "type": "DateTime",
        "value": "2026-11-02T04:47:17.319877+00:00"
      }
    }
  ],
  "total": 1
}

--- Getting goal ---
Fetched: Deploy v2.0

--- Updating goal ---
Updated description: Ship version 2.0 with hot-fix patches

--- Searching goals ---
Search results: {
  "count": 1,
  "items": [
    {
      "_score": 12.870000000000001,
      "created_at": {
        "type": "DateTime",
        "value": "2026-11-02T04:47:17.319877+00:00"
      },
      "description": {
        "type": "String",
        "value": "Ship version 2.0 with hot-fix patches"
      },
      "id": "4OV3E9QO4PNEmSxSCsXNFWxOmz04AdcPaMMsr6Ly_NkE6ruYGHvKwC6zv9nx3lkl6x60-qU3TJ5uAwtGt-5GOA",
      "owner": {
        "type": "Object",
        "value": {
          "key_id": "3a0cdba0da7c060610042783cc758f1f",
          "key_label": "default_admin"
        }
      },
      "status": {
        "type": "String",
        "value": "pending"
      },
      "steps": {
        "type": "String",
        "value": "[{\"description\":\"Run test suite\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"
      },
      "title": {
        "type": "String",
        "value": "Deploy v2.0"
      },
      "updated_at": {
        "type": "DateTime",
        "value": "2026-11-02T04:47:17.333655+00:00"
      }
    }
  ]
}

--- Goal step: start step 0 ---
Step 0 started on goal 4OV3E9QO4PNEmSxSCsXNFWxOmz04AdcPaMMsr6Ly_NkE6ruYGHvKwC6zv9nx3lkl6x60-qU3TJ5uAwtGt-5GOA
--- Goal step: complete step 0 ---
Step 0 completed on goal 4OV3E9QO4PNEmSxSCsXNFWxOmz04AdcPaMMsr6Ly_NkE6ruYGHvKwC6zv9nx3lkl6x60-qU3TJ5uAwtGt-5GOA
--- Goal step: fail step 1 ---
Step 1 failed on goal 4OV3E9QO4PNEmSxSCsXNFWxOmz04AdcPaMMsr6Ly_NkE6ruYGHvKwC6zv9nx3lkl6x60-qU3TJ5uAwtGt-5GOA

--- Completing goal ---
Goal status: pending_review
--- Approving goal ---
Goal status after approve: in_progress

--- Creating goal to reject ---
--- Rejecting goal ---
Goal status after reject: failed


--- Creating task ---
Created task: Hourly Health Check (id: 2IS-hi9SoEQdNf7qx24wqmzsBLDDQlofSdYKpTBnmeZIhsHUXemGo08xmna8UxGHBAmortCtzyg3Dcp9M8u7Sg)

--- Listing tasks ---
Tasks: {
  "count": 1,
  "items": [
    {
      "action": {
        "type": "String",
        "value": "health_check"
      },
      "config": {
        "type": "Object",
        "value": {
          "endpoint": "/health"
        }
      },
      "cron": {
        "type": "String",
        "value": "0 * * * *"
      },
      "id": "2IS-hi9SoEQdNf7qx24wqmzsBLDDQlofSdYKpTBnmeZIhsHUXemGo08xmna8UxGHBAmortCtzyg3Dcp9M8u7Sg",
      "name": {
        "type": "String",
        "value": "Hourly Health Check"
      },
      "owner": {
        "type": "Object",
        "value": {
          "key_id": "3a0cdba0da7c060610042783cc758f1f",
          "key_label": "default_admin"
        }
      }
    }
  ],
  "total": 1
}

--- Getting task ---
Fetched: Hourly Health Check

--- Starting task ---
Task status: running

--- Succeeding task ---
Task status after succeed: active

--- Pausing task ---
Task status after pause: paused

--- Resuming task ---
Task status after resume: active

--- Failing task ---
Task status after fail: active

--- Getting due tasks ---
Due tasks: {
  "count": 0,
  "items": []
}

--- Deleting task ---
Task deleted successfully


--- Creating agent ---
Created agent: SupportBot (id: jCSR6JQqPNa9YBEEAGGEgQ30Z9TMXS2Gn-PZ-woy87N2msByfJWjjEk5PU_QgRn0dtmmItsR2TDaD8gnzRyJIg)

--- Listing agents ---
Agents: {
  "count": 1,
  "items": [
    {
      "deployment_id": {
        "type": "String",
        "value": "deploy_prod_1"
      },
      "id": "jCSR6JQqPNa9YBEEAGGEgQ30Z9TMXS2Gn-PZ-woy87N2msByfJWjjEk5PU_QgRn0dtmmItsR2TDaD8gnzRyJIg",
      "llm_model": {
        "type": "String",
        "value": "gpt-6-luna"
      },
      "name": {
        "type": "String",
        "value": "SupportBot"
      },
      "owner": {
        "type": "Object",
        "value": {
          "key_id": "3a0cdba0da7c060610042783cc758f1f",
          "key_label": "default_admin"
        }
      },
      "run_policy": {
        "type": "String",
        "value": "{\"trigger\":{\"allow_write_operations\":false,\"write_tools\":[]},\"eval\":{\"allow_write_operations\":false,\"write_tools\":[]},\"customer\":{\"allow_write_operations\":false,\"write_tools\":[]}}"
      },
      "system_prompt": {
        "type": "String",
        "value": "You are a helpful customer support agent."
      }
    }
  ],
  "total": 1
}

--- Getting agent by ID ---
Fetched: SupportBot

--- Getting agent by name ---
By name: SupportBot (id: jCSR6JQqPNa9YBEEAGGEgQ30Z9TMXS2Gn-PZ-woy87N2msByfJWjjEk5PU_QgRn0dtmmItsR2TDaD8gnzRyJIg)

--- Updating agent ---
Updated agent: SupportBot

--- Getting agents by deployment ---
Agents in deployment: {
  "count": 0,
  "items": []
}
WARNING: agents-by-deployment omitted created agent jCSR6JQqPNa9YBEEAGGEgQ30Z9TMXS2Gn-PZ-woy87N2msByfJWjjEk5PU_QgRn0dtmmItsR2TDaD8gnzRyJIg; TODO: check/fix the server-side deployment lookup

--- Deleting agent ---
Agent deleted successfully

--- Cleanup: deleting goals ---
Goals deleted successfully

=== All goals, tasks & agents operations completed ===
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


✅ Join operations examples completed!
=== Cleanup ===
✅ Deleted test collections
✓ Client created
✓ ts_users_register saved
✓ ts_users_login saved
✓ ts_users_verify_token saved

=== Auth flow defined as pure stored functions ===
Call them like:
  POST /api/functions/jwt_register_ts_78254_1793594837865 { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/jwt_login_ts_78254_1793594837865 { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/jwt_verify_ts_78254_1793594837865 { "token": "<jwt>" }

Set JWT_SECRET in ekoDB's environment_vars whitelist before invoking.

✓ Cleaned up demo functions
=== ekoDB KV Document Linking Example (TypeScript) ===

--- Setup: creating KV key and documents ---
Set KV key: session:admin
Inserted document 1: mku1kiv1Tv9tLDv7oc-dIg6t5BDFFM6HCRFRBDuPnlUOpH5BMl-fN-bmQxxDEiN1pj-T5RbB3AuS5N5YVEYgeg
Inserted document 2: 5ql-EhhqMuS5XQ6aP0KxT95b0_bunSThjvdCZn_u_e8iENXu8cp0PzKnroq6d5tMoyqw6EBfXPnyQMsj0uq-gg

--- Linking documents to KV key ---
Linked doc mku1kiv1Tv9tLDv7oc-dIg6t5BDFFM6HCRFRBDuPnlUOpH5BMl-fN-bmQxxDEiN1pj-T5RbB3AuS5N5YVEYgeg: null
Linked doc 5ql-EhhqMuS5XQ6aP0KxT95b0_bunSThjvdCZn_u_e8iENXu8cp0PzKnroq6d5tMoyqw6EBfXPnyQMsj0uq-gg: null

--- Getting links for KV key ---
Links: [
  {
    "collection": "kv_links_example_ts_78261_1793594837950",
    "document_id": "5ql-EhhqMuS5XQ6aP0KxT95b0_bunSThjvdCZn_u_e8iENXu8cp0PzKnroq6d5tMoyqw6EBfXPnyQMsj0uq-gg",
    "field_path": null,
    "created_at": "2026-11-02T04:47:18.036492Z",
    "last_accessed": "2026-11-02T04:47:18.038112Z",
    "metadata": {}
  },
  {
    "collection": "kv_links_example_ts_78261_1793594837950",
    "document_id": "mku1kiv1Tv9tLDv7oc-dIg6t5BDFFM6HCRFRBDuPnlUOpH5BMl-fN-bmQxxDEiN1pj-T5RbB3AuS5N5YVEYgeg",
    "field_path": null,
    "created_at": "2026-11-02T04:47:18.034788Z",
    "last_accessed": "2026-11-02T04:47:18.038112Z",
    "metadata": {}
  }
]

--- Unlinking document ---
Unlinked doc 5ql-EhhqMuS5XQ6aP0KxT95b0_bunSThjvdCZn_u_e8iENXu8cp0PzKnroq6d5tMoyqw6EBfXPnyQMsj0uq-gg: null

--- Verifying remaining links ---
Remaining links: [
  {
    "collection": "kv_links_example_ts_78261_1793594837950",
    "document_id": "mku1kiv1Tv9tLDv7oc-dIg6t5BDFFM6HCRFRBDuPnlUOpH5BMl-fN-bmQxxDEiN1pj-T5RbB3AuS5N5YVEYgeg",
    "field_path": null,
    "created_at": "2026-11-02T04:47:18.034788Z",
    "last_accessed": "2026-11-02T04:47:18.041067Z",
    "metadata": {}
  }
]

=== All KV linking operations completed ===

--- Cleanup ---
Cleanup complete
✓ Client created

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: { value: { username: 'john_doe', userId: 123 }, type: 'Object' }

=== KV Batch Set ===
✓ Batch set 3 keys
  kv_ops_ts_78286_1793594838133:cache:product:1: success
  kv_ops_ts_78286_1793594838133:cache:product:2: success
  kv_ops_ts_78286_1793594838133:cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  kv_ops_ts_78286_1793594838133:cache:product:1: { name: 'Product 1', price: 29.99 }
  kv_ops_ts_78286_1793594838133:cache:product:2: { name: 'Product 2', price: 39.99 }
  kv_ops_ts_78286_1793594838133:cache:product:3: { price: 49.99, name: 'Product 3' }

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
  kv_ops_ts_78286_1793594838133:cache:product:1: deleted
  kv_ops_ts_78286_1793594838133:cache:product:2: deleted
  kv_ops_ts_78286_1793594838133:cache:product:3: deleted

✓ All KV operations completed successfully
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

=== Summary ===
✅ Use Field.decimal() for monetary values, percentages, and
   any case where floating-point errors are unacceptable.
✅ Field.decimal() stores values as strings internally,
   preserving exact precision across all operations.

=== Cleanup ===
✓ Cleaned up test keys
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
=== ekoDB Raw Completion Stream (SSE) Example ===

--- Basic SSE Raw Completion ---
Response: In traditional art, the primary colors are **red, yellow, and blue**. For light, they’re **red, green, and blue**.

--- Structured Output via SSE ---
JSON response: [
  {"name": "Earth", "diameter_km": 12742},
  {"name": "Jupiter", "diameter_km": 139820},
  {"name": "Mars", "diameter_km": 6779}
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello! 👋

=== Done ===
=== ekoDB Schedule Management Example (TypeScript) ===

--- Creating schedule ---
Created schedule: Nightly Database Backup (id: 408d24b6-796c-4103-aaa6-c5e4088e0a47, cron: 0 0 2 * * *)

--- Listing schedules ---
Schedules: {
  "count": 1,
  "schedules": [
    {
      "created_at": "2026-11-02T04:47:23.912916Z",
      "cron_expression": "0 0 2 * * *",
      "description": null,
      "enabled": true,
      "function_label": "schedule_noop_typescript_79281_1793594843875",
      "id": "408d24b6-796c-4103-aaa6-c5e4088e0a47",
      "last_execution": null,
      "name": "Nightly Database Backup",
      "next_execution": "2026-11-03T02:00:00Z",
      "parameters": {},
      "stats": {
        "avg_execution_time_ms": 0,
        "failed_executions": 0,
        "last_error": null,
        "successful_executions": 0,
        "total_executions": 0
      },
      "timezone": "UTC",
      "updated_at": "2026-11-02T04:47:23.912916Z"
    }
  ]
}

--- Getting schedule ---
Fetched: Nightly Database Backup (cron: 0 0 2 * * *)

--- Updating schedule ---
Updated: Nightly Full Backup (new cron: 0 0 3 * * *)

--- Triggering schedule ---
Trigger response: {
  "schedule_id": "408d24b6-796c-4103-aaa6-c5e4088e0a47",
  "status": "triggered"
}

--- Pausing schedule ---
Schedule enabled after pause: false

--- Resuming schedule ---
Schedule enabled after resume: true

--- Deleting schedule ---
Schedule deleted successfully

=== All schedule operations completed ===
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
  3. Score: 13.200, Matched: title, bio
  4. Score: 13.200, Matched: title, bio

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
  1. Score: 0.771, Matched:
  2. Score: 0.763, Matched:
  3. Score: 0.759, Matched:

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.505, Matched: title, content
  2. Score: 0.904, Matched: content, title
  3. Score: 0.308, Matched:

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: bio, skills

9. Vector search with a metadata pre-filter (category = ml):
Found 2 documents in category "ml" (NLP excluded)
  1. Introduction to Machine Learning (category: ml)
  2. Deep Learning Fundamentals (category: ml)

Paired vector search: document=Rust Programming Answer, query=Rust Programming Question

✅ Search examples completed!
=== Cleanup ===
✅ Deleted test collections
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'NObYoJXW1psJBoEn3vvLS8KK8HMluz3DDK-kMeVmV4QDAZHjpT0ZYQsRZfkHHGNNLP0ndtRtS8jAlLqnB4cARA'
}

=== Find by ID ===
Found: {
  active: { type: 'Boolean', value: true },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  metadata: { value: { nested: [Object], key: 'value' }, type: 'Object' },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  price: { value: 99.99, type: 'Float' },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  value: { type: 'Integer', value: 42 },
  created_at: { type: 'DateTime', value: '2026-11-02T04:47:24.651+00:00' },
  name: { value: 'Test Record', type: 'String' },
  id: 'NObYoJXW1psJBoEn3vvLS8KK8HMluz3DDK-kMeVmV4QDAZHjpT0ZYQsRZfkHHGNNLP0ndtRtS8jAlLqnB4cARA',
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-11-02T04:47:24.651Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { nested: { deep: true }, key: 'value' }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  active: true,
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  metadata: { nested: { deep: true }, key: 'value' },
  categories: [ 'electronics', 'computers' ],
  price: 99.99,
  tags: [ 'tag1', 'tag2', 'tag3' ],
  value: 42,
  created_at: '2026-11-02T04:47:24.651+00:00',
  name: 'Test Record',
  id: 'NObYoJXW1psJBoEn3vvLS8KK8HMluz3DDK-kMeVmV4QDAZHjpT0ZYQsRZfkHHGNNLP0ndtRtS8jAlLqnB4cARA',
  data: 'aGVsbG8gd29ybGQ=',
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  id: 'NObYoJXW1psJBoEn3vvLS8KK8HMluz3DDK-kMeVmV4QDAZHjpT0ZYQsRZfkHHGNNLP0ndtRtS8jAlLqnB4cARA',
  created_at: { value: '2026-11-02T04:47:24.651+00:00', type: 'DateTime' },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  price: { value: 99.99, type: 'Float' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  name: { type: 'String', value: 'Updated Record' },
  active: { value: true, type: 'Boolean' },
  value: { value: 100, type: 'Integer' },
  metadata: { value: { nested: [Object], key: 'value' }, type: 'Object' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: pQOg2z86JNapleGOTjcllYQJh_pBKjJ6jtZEsF7X4c-FgFObfz1qL9V8yeo9A957uAPK-81Pb_DQaPHKLkYF3w

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
🚀 ekoDB TypeScript Client - Native SWR Function Examples

📋 Demonstrates:
   • Single-function SWR pattern (replaces 4-step pipeline)
   • Automatic cache checking, HTTP fetching, and cache setting
   • Built-in audit trail support
   • Duration string TTLs ('15m', '1h', '30s')
   • Multi-function pipeline integration
   • Dynamic TTL configuration


🧹 Cleaning up...
✓ Deleted 0 test scripts and owned SWR resources

Example 1: Basic Native SWR
────────────────────────────────────────────────────────────────────────────────
Single function replaces KvGet → If → HttpRequest → KvSet pipeline
✓ Created native SWR script: github_user_native_ts (2Xlq7Tt9P2jWy1Epgrk8FTwUjaxDNcN0bsod-b92fBELwCSGYwAnj7buaWq2rN7crpDzTUt03SbI7maW1R1u7w)

First call (cache miss - will fetch from GitHub API):
  Response time: 134ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 3ms
  Speedup: 44.7x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_ts (QAkNWeLQcTxO8VrZFYSnsyRYJFdfb35No2GLjEaoDOCxKju7yBIG1GLDyJlAlrM6IkAfc5C7oxAPj8_EQgQzjg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_ts' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_ts (J4f48TbS3VTSt6N7nTPq4CGo8NpeolxMuYAvS84jC-mKp4bKZwIjeSZzytax8vykKyrE1Pq-CUoXsFLCSsB1nQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_ts' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_ts (l-tXZibIFn86Xr8gdk4PFNuTXmPpxDFxMfb8RwNZ1ktHJ5ge1vmomUvsxg4fL1-uz5sj7i5dqwlbHy-KVHj44Q)
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
✓ Deleted 4 test scripts and owned SWR resources

✅ All examples completed!
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_ts_79580_1793594845872 (9-SS6MX-uciljJ31RpQKmGQ4QZHezuRTjYsBMmAx3mbq3Pps1QjeAi1RcTl-xbtYIaicHpMQTNGu3MIexRyjVA)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "phone": "1-770-736-8031 x56442",
          "id": 1,
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "bs": "harness real-time e-markets",
            "name": "Romaguera-Crona"
          },
          "email": "Sincere@april.biz",
          "name": "Leanne Graham",
          "username": "Bret",
          "address": {
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "suite": "Apt. 556",
            "street": "Kulas Light",
            "zipcode": "92998-3874",
            "city": "Gwenborough"
          },
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
          "id": 1,
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "bs": "harness real-time e-markets",
            "name": "Romaguera-Crona"
          },
          "email": "Sincere@april.biz",
          "name": "Leanne Graham",
          "username": "Bret",
          "address": {
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "suite": "Apt. 556",
            "street": "Kulas Light",
            "zipcode": "92998-3874",
            "city": "Gwenborough"
          },
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
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_reviews_ts_79580_1793594845872 (bHNu8IWx57a6ksDyR-cSYFR333KbZRujTL4Wu9ozke-AqRwfEC0KgEYq72H6iu1IM9nwFZBaTP7zChEBuRv5pg)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "weight": 4,
          "availabilityStatus": "In Stock",
          "brand": "Essence",
          "id": 1,
          "returnPolicy": "No return policy",
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "stock": 99,
          "tags": [
            "beauty",
            "mascara"
          ],
          "dimensions": {
            "height": 13.08,
            "width": 15.14,
            "depth": 22.99
          },
          "category": "beauty",
          "title": "Essence Mascara Lash Princess",
          "warrantyInformation": "1 week warranty",
          "reviews": [
            {
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerName": "Eleanor Collins",
              "comment": "Would not recommend!",
              "rating": 3,
              "reviewerEmail": "eleanor.collins@x.dummyjson.com"
            },
            {
              "reviewerEmail": "lucas.gordon@x.dummyjson.com",
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerName": "Lucas Gordon",
              "rating": 4,
              "comment": "Very satisfied!"
            },
            {
              "comment": "Highly impressed!",
              "rating": 5,
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerName": "Eleanor Collins"
            }
          ],
          "shippingInformation": "Ships in 3-5 business days",
          "meta": {
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
            "createdAt": "2025-10-09T14:47:01.588Z",
            "updatedAt": "2026-05-23T11:27:41.868Z",
            "barcode": "5784719087687"
          },
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "price": 9.99,
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "minimumOrderQuantity": 48,
          "discountPercentage": 10.48,
          "sku": "BEA-ESS-ESS-001",
          "rating": 2.56
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

✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: EOsBqByPbPQQ_UhnWM69V85ZFBnMY3kKAIC9DaMLxviuuokSOcd6kr8yjl8mSnJezwZf7LEJMSZGc2LZivZLgQ
Created Bob: $500 - ID: WbsSkCDdPY-K63ID7jbsVVhbK4Ncp3Bkf9JeyrIBYVvVkOL0JdSj6mlAvo3pCiKFgr-LmtNHqvpuRwgyT7hvHg

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 91b3d619-e8fa-4f2b-a21b-8b69b6a7010d

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
New transaction: 275cdc00-1671-43a1-87e3-a656bc31e888
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Create User Function ===
Created user function with ID: YpmVW2Df1ESFOhZksENdFaLAqecmGlurEaPPrjm2AUOqImuqGhdc3qk434F3XFl5TwX68gjRL7Q0wAKfle0zlg

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 9 user functions:
  - conc_demo_rl_skip_ts_50152_1793594691316: Rate-limit (skip mode)
  - conc_demo_rl_fail_ts_50152_1793594691316: Rate-limit (fail mode)
  - get_active_users_client_ts: Get Active Users (Updated)
  - get_active_users_ts: Get Active Users
  - get_active_users_client_js: Get Active Users (Updated)
  - fetch_product_reviews_ts_79580_1793594845872: Fetch Product with Reviews (Multi-API)
  - conc_demo_rl_fail_ts_76798_1793594830125: Rate-limit (fail mode)
  - fetch_product_reviews_ts_55079_1793594714450: Fetch Product with Reviews (Multi-API)
  - conc_demo_rl_skip_ts_76798_1793594830125: Rate-limit (skip mode)

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
=== WebSocket Chat Streaming Example (TypeScript) ===

Created chat session: kZjYfbZMNt5RFihvODNwyZpDZnD5SypljeIt5Sk7GUmUo7g_ObWknWLlrot0OvD5BkJ5ji1UPbUctQ94rbNaqA

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"gpt-5.6-luna","provider":"openai"}The capital of France is **Paris**.

--- Stream ended ---
Message ID: pm8g_n9cb4QULQn0HoMh3EOAYPkAYm4h4-j6MTlqnqTSuT7aU-Dd9kcOzEQywtlooTAz1llxKVVW8eF_L80Fow
Execution time: 980ms
Token usage: {"cached_input_tokens":0,"completion_tokens":13,"prompt_tokens":23,"thinking_tokens":0,"total_tokens":36}

Full response: {"__progress":"received"}{"__progress":"generating","model":"gpt-5.6-luna","provider":"openai"}The capital of France is **Paris**....
=== WebSocket Subscription Example ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_ts' ===
✓ Subscribed (subscription_id: sub_706942bd73194d8a8dee012a62f4f863)

=== Performing mutations to trigger notifications ===
Inserting a record...
✓ Inserted record: Ck5zIDztEqVwVEbO6ll11ThuRcBMb35u5OfjKS3zPhvo2G8D0qENgxAL2V_0KT1x79aIKP_b7Bl-ZNZm4u6_sQ
  📡 Notification received for Ck5zIDztEqVwVEbO6ll11ThuRcBMb35u5OfjKS3zPhvo2G8D0qENgxAL2V_0KT1x79aIKP_b7Bl-ZNZm4u6_sQ

Inserting another record...
✓ Inserted record: itq4TVs-eR8Q02axzec_7nXqtoeCD88AjrL1ibPJrAkwE8Bq4IY0bbRjRWIPi9HBOshbkRy4N6wYsWyX_A34JQ
  📡 Notification received for itq4TVs-eR8Q02axzec_7nXqtoeCD88AjrL1ibPJrAkwE8Bq4IY0bbRjRWIPi9HBOshbkRy4N6wYsWyX_A34JQ

=== Unsubscribing ===
✓ Unsubscribed: {"collection":"ws_subscribe_example_ts","found":true,"unsubscribed":true}

✓ WebSocket subscription example completed successfully
✓ Deleted collection 'ws_subscribe_example_ts'
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: kNcl_2h00A3D2gRMBgzh1S9lVjo3wa4SYjPSkADyiVulOO3AM1vTJx2nVQGaRo_1vikN4Jp500VeadyQn7gxMQ

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: {"id":"YQbwAe7EpQvnVuf-4Rtgdepc2IG4v3oC6ONVh8b4biQPd6M0E74cN90EhJD97BtMMTaWDXEuXFcHRzw00BtJDQ"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"vjLeYu0nJxyYO6l0PXjFrvw0dCv1420cQNwnDPqpqwD7u338rQzSzAmGkU7tNK3QVKjvW-KLL9GJZOoAqG1XzQ"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"name":{"value":"Product 1","type":"String"},"id":"YQbwAe7EpQvnVuf-4Rtgdepc2IG4v3oC6ONVh8b4biQPd6M0E74cN90EhJD97BtMMTaWDXEuXFcHRzw00BtJDQ","price":{"value":150,"type":"Integer"}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"id":"custom-id"}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ["email","id","name"]
  First user: Dave Brown <dave@example.com>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["name","email","avatar_url","id","status","bio","age","created_at","user_role"]

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
    - Fields: ["bio","email","secret_token","user_role","status","id","password","created_at","avatar_url","name","api_key","age"]
  Projected query:
    - 3 fields per record
    - Fields: ["email","name","id"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
✅ All JavaScript integration tests complete!
🟣 Building Kotlin client library...
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.2/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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

BUILD SUCCESSFUL in 6s
11 actionable tasks: 11 up-to-date
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.2/userguide/configuration_cache_enabling.html
✅ Kotlin client built!
=== Running Kotlin example: ClientAdmin.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
Admin permissions read successfully

BUILD SUCCESSFUL in 6s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientAdvancedCrud.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Advanced CRUD Example ===

--- Inserting base record ---
Inserted: jPnlat3J0EKMXkavFMHP7LUoiDOXmRbvUwZBHh263_KWkn9914ascEjy5wa8E9cJ7xtOhB78-Zn5n0QItfarKQ

--- updateWithAction: increment views ---
After increment: EkoRecord(fields={views=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=15)}), id=StringValue(value=jPnlat3J0EKMXkavFMHP7LUoiDOXmRbvUwZBHh263_KWkn9914ascEjy5wa8E9cJ7xtOhB78-Zn5n0QItfarKQ), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Counter Record)}), score=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=100.0)}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=kotlin), StringValue(value=example)]), type=StringValue(value=Array)})})

--- updateWithAction: push to tags ---
After push: EkoRecord(fields={views=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=15)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=kotlin), StringValue(value=example), StringValue(value=advanced)])}), id=StringValue(value=jPnlat3J0EKMXkavFMHP7LUoiDOXmRbvUwZBHh263_KWkn9914ascEjy5wa8E9cJ7xtOhB78-Zn5n0QItfarKQ), name=ObjectValue(value={value=StringValue(value=Counter Record), type=StringValue(value=String)}), score=ObjectValue(value={value=FloatValue(value=100.0), type=StringValue(value=Float)})})

--- updateWithActionSequence: multiple operations ---
After sequence: EkoRecord(fields={id=StringValue(value=jPnlat3J0EKMXkavFMHP7LUoiDOXmRbvUwZBHh263_KWkn9914ascEjy5wa8E9cJ7xtOhB78-Zn5n0QItfarKQ), views=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=115)}), score=ObjectValue(value={value=FloatValue(value=75.0), type=StringValue(value=Float)}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=kotlin), StringValue(value=example), StringValue(value=advanced), StringValue(value=sequenced)]), type=StringValue(value=Array)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Counter Record)})})

--- Cleanup ---
Deleted collection: kotlin_advanced_crud_example

=== Example Complete ===

BUILD SUCCESSFUL in 5s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientBatchOperations.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Batch Operations Example ===

=== Batch Insert ===
✓ Inserted 5 records
  IDs: 9Q_97J8CAxuV8YfEXUnwMmZ1nb2xJb_7zI7HwVqcuRziVX_1qyAmCA1agmRVNBvBvqIA660RlFnIMT3Swrvw_w, 9fsAgO-em3TcYvSie1oCBeR_J8JyZqqpw2CyCUPPMoCefv2PQ_DxK2kZCSmeh0CsMSpU2o1vmzKt3rCKYvJZPw, Gz3jNk78AEuXS52kPj3P_lTMt0B-IOn1szqa0sY5UkkxXKWjWJp5zP70bgKnKrjy8MYybLmVglawvhdLH1NuIA...

=== Batch Update ===
✓ Updated 3 records; 0 failed

=== Batch Delete ===
✓ Deleted 2 records

=== Cleanup ===
✓ Deleted collection: kotlin_batch_example

=== Example Complete ===

BUILD SUCCESSFUL in 5s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientChatAdvanced.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 0S6nl6FNjxQY_KTjfPN9UxepUo5R0alZ1M4qxKDlan5EOSaAriABBAF7ZEarrwx6K47x5m6KZdah_HMUg0fQ1g

=== Sending Initial Message ===
✓ Message sent
  Responses: ["The available product is **ekoDB**, a high-performance database product priced at **$99**."]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["ekoDB is priced at **$99**."]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: ukLyTzgG7QuBISE_ItqzrdyRnp3SHtrPQZlym4rWBGrGlNUsc9I1Zy8Ieyac8wUgFyaZemU_ZJozFCGgqSfkKA

=== Merging Sessions ===
✓ Merged sessions
  Total messages in merged session: 5

=== Deleting Message ===
✓ Deleted message

=== Cleanup ===
✓ Deleted chat session: ukLyTzgG7QuBISE_ItqzrdyRnp3SHtrPQZlym4rWBGrGlNUsc9I1Zy8Ieyac8wUgFyaZemU_ZJozFCGgqSfkKA
✓ Deleted chat session: 0S6nl6FNjxQY_KTjfPN9UxepUo5R0alZ1M4qxKDlan5EOSaAriABBAF7ZEarrwx6K47x5m6KZdah_HMUg0fQ1g
✓ Deleted collection: kotlin_chat_advanced_example

✓ Advanced chat features example completed successfully

BUILD SUCCESSFUL in 13s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientChatBasic.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Basic Chat Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: 1meJN2HlBH7eYVQ18IDGvhm4PPSfO-BCLAL6BoD-7-8ym8skP4CyaICJdupI3bZ99q8ldLGYnt-uN1nQZt41mQ

=== Sending Chat Message ===
✓ Chat response:
  Message ID: "6OOhMK0IL6GPI8mTCCCEAwj0o5mt6IOeDJsn-I7PD78Xrjo290NU9O-_3PAGa_f0dhuAQ_EEfRvhKpUCiCKbAg"
  Responses: ["ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration. Its features include:\n\n- **Natural-language AI chat** to query your database and get AI-powered responses with relevant context.\n- **Full-text search, vector search, and hybrid search**, with automatic context retrieval."]

=== Cleanup ===
✓ Deleted chat session
✓ Deleted collection: kotlin_chat_basic_example

✓ Basic chat example completed successfully

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientChatMessageStream.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
=== ekoDB Chat Message Stream (SSE) Example (Kotlin) ===

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
Created session: L7Q9XZHrDSIfSKKg_XK_CrE_6t-waYyngoyjoeCwEnhRTIBqKlMC_cutzTAZxuT72nJew7nFSXjGOYaU87hUZQ

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"gpt-6-luna","provider":"openai"}**ekoDB** is a name used by more than one software project. If you mean the database product, it generally refers to a database for storing and working with application data; the exact features depend on the project. Share a link or where you saw the name, and I can identify the specific one.

--- Stream complete ---
Message ID: ww1LTRJwVe1KX_ri0V4hYFtdsiOUN4K1hZdn5SwsJSbKlEsWoOEq01SnX4k3pqk1v0NwE7enNzNNA8dMkyQfDQ
Execution time: 7257ms
Context window: 128000 tokens

✓ Chat message stream example completed

BUILD SUCCESSFUL in 12s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientChatModels.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Chat Models Example ===

=== List Chat Models ===
Available chat models by provider:
  openai:
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
  anthropic:
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
  gemini:
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

=== Get OpenAI Models ===
OpenAI models: gpt-3.5-turbo-0125, gpt-5.4-2026-03-05, gpt-5-nano, gpt-5.5-pro, gpt-audio-2025-08-28, gpt-4o-mini-transcribe-2025-03-20, gpt-4-turbo-2024-04-09, gpt-realtime-2.1-mini, gpt-realtime-2025-08-28, gpt-4, tts-1-hd, gpt-realtime-1.5, gpt-realtime, gpt-5.2-pro, gpt-5-chat-latest, gpt-5.4-mini-2026-03-17, babbage-002, gpt-4o-2024-05-13, text-embedding-3-large, tts-1, omni-moderation-2024-09-26, gpt-live-transcribe, gpt-5.6-luna, gpt-5-2025-08-07, gpt-5-mini, gpt-5-mini-2025-08-07, omni-moderation-latest, o3-mini-2025-01-31, gpt-audio-1.5, gpt-image-1.5, tts-1-1106, gpt-5.5-pro-2026-04-23, gpt-audio-mini-2025-10-06, gpt-5.2-pro-2025-12-11, o4-mini-2025-04-16, gpt-4o-2024-11-20, gpt-5.1, whisper-1, gpt-5-pro, gpt-realtime-whisper, gpt-realtime-2, gpt-5-search-api, gpt-5.5-2026-04-23, gpt-4.1-nano, gpt-image-2, gpt-3.5-turbo-16k, text-embedding-ada-002, gpt-realtime-2.1, gpt-4.1, gpt-5.5, chat-latest, gpt-realtime-translate, gpt-5.4-pro-2026-03-05, gpt-4.1-mini, gpt-4o-mini, gpt-4o-mini-transcribe-2025-12-15, gpt-realtime-mini, gpt-image-1-mini, gpt-5.6-terra, gpt-3.5-turbo-instruct-0914, gpt-4-turbo, gpt-4o-2024-08-06, o3, gpt-4o-mini-search-preview-2025-03-11, o1-2024-12-17, gpt-5.1-chat-latest, gpt-5-pro-2025-10-06, gpt-5.6-sol, gpt-4o-transcribe-diarize, gpt-5-codex, gpt-4-0613, gpt-5.1-2025-11-13, gpt-4o-search-preview, gpt-4o-mini-tts-2025-12-15, gpt-image-1, gpt-audio, gpt-5.3-codex, gpt-5-search-api-2025-10-14, gpt-5.4-pro, gpt-4o-mini-tts, gpt-realtime-mini-2025-12-15, gpt-5.1-codex-max, sora-2, gpt-4o, gpt-4o-mini-2024-07-18, gpt-4.1-2025-04-14, gpt-5, gpt-4o-mini-transcribe, gpt-image-2-2026-04-21, gpt-5.2-chat-latest, sora-2-pro, gpt-3.5-turbo-instruct, gpt-transcribe, gpt-4.1-nano-2025-04-14, gpt-3.5-turbo-1106, gpt-4o-mini-tts-2025-03-20, gpt-5.2, gpt-5.4-nano-2026-03-17, o1, tts-1-hd-1106, gpt-5.2-codex, gpt-5.4-mini, o1-pro, gpt-5.1-codex, o4-mini-deep-research, gpt-5.4, gpt-4o-search-preview-2025-03-11, text-embedding-3-small, gpt-5-nano-2025-08-07, gpt-4o-transcribe, o4-mini, gpt-5.1-codex-mini, gpt-6-astra, gpt-5.3-chat-latest, o3-2025-04-16, gpt-5.2-2025-12-11, chatgpt-image-latest, gpt-3.5-turbo, gpt-image-2.5-sunburst-2026-09-08, gpt-image-2.5-flare, davinci-002, gpt-5.4-nano, o3-mini, o4-mini-deep-research-2025-06-26, gpt-audio-mini-2025-12-15, o1-pro-2025-03-19, gpt-live-1, gpt-image-2.5-sunburst, gpt-image-2.5-flare-2026-09-08, gpt-6.1-terra, gpt-audio-mini, gpt-4.1-mini-2025-04-14, gpt-6-luna, gpt-4o-mini-search-preview, gpt-6-sol, gpt-6.1-sol

=== Get Anthropic Models ===
Anthropic models: claude-haiku-5-5, claude-sonnet-5-5, claude-opus-5-5, claude-fable-5-1, claude-opus-5, claude-sonnet-5, claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Request failed with status 404: {"error":"Unknown provider 'nonexistent_provider_xyz'","error_kind":"unknown_provider"}

=== Chat Models Example Complete ===

BUILD SUCCESSFUL in 5s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientChatSessions.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: LZ6AdQKA_WqM7G93UXv8Tje3L_4nNBSkxEnhyHHGQU5vk-slbhkZdpcjYj1UmCXDm9PqHXRtyvJQdT31hwGTEA

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["The available product is **ekoDB**, a high-performance database with AI capabilities, priced at **$99**."]

✓ Message 2 sent
  Responses: ["ekoDB is priced at **$99**."]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"author":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"chat_id":{"type":"String","value":"LZ6AdQKA_WqM7G93UXv8Tje3L_4nNBSkxEnhyHHGQU5vk-slbhkZdpcjYj1UmCXDm9PqHXRtyvJQdT31hwGTEA"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"62O0wq0nabjPWzJ7YeN-tCmotEl2rjTYIxnvUs32y3kFM1_RAc-vXFHIpRPOOWGCK-BzcmdNSQH23E0CxGwd_Q","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-11-02T04:48:42.867834+00:00"},"id":"y7u6ph993RCAuWaRfsilZ-ENtu9bNCiaVKuhqwGVFmbtc5CURrrkcfcJjDlDq8iUe2C52CKILo8oZz5BmJDEAA","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":72,"prompt_tokens":13556,"total_tokens":13628}},"updated_at":{"type":"DateTime","value":"2026-11-02T04:48:42.867834+00:00"}},{"author":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"chat_id":{"type":"String","value":"LZ6AdQKA_WqM7G93UXv8Tje3L_4nNBSkxEnhyHHGQU5vk-slbhkZdpcjYj1UmCXDm9PqHXRtyvJQdT31hwGTEA"},"content":{"type":"String","value":"The available product is **ekoDB**, a high-performance database with AI capabilities, priced at **$99**."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"62O0wq0nabjPWzJ7YeN-tCmotEl2rjTYIxnvUs32y3kFM1_RAc-vXFHIpRPOOWGCK-BzcmdNSQH23E0CxGwd_Q","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-11-02T04:48:42.888532+00:00"},"id":"nmcpZ3wyQYyolX8Cp7F-igUx31EuCkJ-5D2NJbtA87q_zf6D-ufPywhYNgaRBxKNDgvw5h-GHjFOtiDwWwmBbg","llm_model":{"type":"String","value":"gpt-6-luna"},"llm_provider":{"type":"String","value":"openai"},"role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":72,"prompt_tokens":13556,"total_tokens":13628}},"updated_at":{"type":"DateTime","value":"2026-11-02T04:48:42.888532+00:00"}},{"author":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"chat_id":{"type":"String","value":"LZ6AdQKA_WqM7G93UXv8Tje3L_4nNBSkxEnhyHHGQU5vk-slbhkZdpcjYj1UmCXDm9PqHXRtyvJQdT31hwGTEA"},"content":{"type":"String","value":"What is the price?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description","product","price"],"record":{"description":"A high-performance database product with AI capabilities","id":"62O0wq0nabjPWzJ7YeN-tCmotEl2rjTYIxnvUs32y3kFM1_RAc-vXFHIpRPOOWGCK-BzcmdNSQH23E0CxGwd_Q","price":99,"product":"ekoDB"},"score":0.9954545454545454}]},"created_at":{"type":"DateTime","value":"2026-11-02T04:48:44.290172+00:00"},"id":"h5URVwHdeTd-Oge16bIqLdUcmqHoJ6PoPXvzBIDCi3JEQiNQlSIt6tSi3P7qClWkXuU9-xMjM3NWbccdbaaH5A","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":14,"prompt_tokens":13598,"total_tokens":13612}},"updated_at":{"type":"DateTime","value":"2026-11-02T04:48:44.290172+00:00"}},{"author":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"chat_id":{"type":"String","value":"LZ6AdQKA_WqM7G93UXv8Tje3L_4nNBSkxEnhyHHGQU5vk-slbhkZdpcjYj1UmCXDm9PqHXRtyvJQdT31hwGTEA"},"content":{"type":"String","value":"ekoDB is priced at **$99**."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description","product","price"],"record":{"description":"A high-performance database product with AI capabilities","id":"62O0wq0nabjPWzJ7YeN-tCmotEl2rjTYIxnvUs32y3kFM1_RAc-vXFHIpRPOOWGCK-BzcmdNSQH23E0CxGwd_Q","price":99,"product":"ekoDB"},"score":0.9954545454545454}]},"created_at":{"type":"DateTime","value":"2026-11-02T04:48:44.304721+00:00"},"id":"mfuWXJCma4FXVvKZfqhxN3pWgWN2wtIxSu2IDWOIS23Lwi7uUxjYXlEU-S6fqcFm7cgSlY_SuSIdeD-ffZxvRw","llm_model":{"type":"String","value":"gpt-6-luna"},"llm_provider":{"type":"String","value":"openai"},"role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":14,"prompt_tokens":13598,"total_tokens":13612}},"updated_at":{"type":"DateTime","value":"2026-11-02T04:48:44.304721+00:00"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 1

=== Branching Session ===
✓ Created branched session: DzgpytpcOrQpLsR-rx1qpyS4O4-BqU2D2cGNpXxSAA5I8xNMaGpxK0OO8cUt9ybfsBz6N2LhBZ4rmcwuK3x-Jg

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_sessions_example

✓ Chat session management example completed successfully

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientCollectionManagement.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Collection Management Example ===

=== List Collections ===
✓ Found 26 collections
  - schema_users_client_go
  - audit__ek0_testing
  - schema_documents_client_js
  - schema_documents_client_ts
  - schema_employees_client_ts

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
✓ Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
✓ Schema: {"fields":{"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2026-11-02T04:48:50.820699Z","last_modified":"2026-11-02T04:48:50.820699Z","bypass_ripple":false,"primary_key_alias":"id"}

=== Cleanup ===
✓ Deleted collection: kotlin_collection_example

=== Example Complete ===

BUILD SUCCESSFUL in 5s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientCollectionUtils.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Collection Utilities Example ===

=== Check Collection Exists (Before Creation) ===
Collection 'collection_utils_test_kt' exists: false

=== Creating Test Documents ===
Created 5 test documents

=== Check Collection Exists (After Creation) ===
Collection 'collection_utils_test_kt' exists: true

=== Count Documents ===
Document count in 'collection_utils_test_kt': 5

=== Get Collection Metadata ===
Collection metadata: {"analytics":["collection_utils_test_kt",{"manifest_backed":false,"record_count":5,"resident_record_ids":5,"total_size":675}],"collection":{"bypass_ripple":false,"created_at":"2026-11-02T04:48:57.229679Z","fields":{"index":{"default":null,"enums":[],"field_type":"Integer","max":null,"min":null,"regex":null,"required":false,"unique":false},"name":{"default":null,"enums":[],"field_type":"String","max":null,"min":null,"regex":null,"required":false,"unique":false}},"last_modified":"2026-11-02T04:48:57.229972Z","primary_key_alias":"id","version":1}}

=== List Collections ===
All collections (27):
  - schema_users_client_go
  - audit__ek0_testing
  - schema_documents_client_js
  - schema_documents_client_ts
  - schema_employees_client_ts
  - schema_employees_client_go
  - chat_raw_completions__ek0_testing
  - schema_documents_client_go
  - functions__ek0_testing
  - test_accounts
  - chat_turns__ek0_testing
  - chat_goal_templates__ek0_testing
  - test_collection
  - schema_users_client_ts
  - chat_goals__ek0_testing
  - schema_products_client_js
  - chat_agent_configs__ek0_testing
  - schedules__ek0_testing
  - agent_function_versions__ek0_testing
  - schema_employees_client_js
  - chat_tasks__ek0_testing
  - schema_products_client_ts
  - schema_users_client_js
  - schema_products_client_go
  - chat_messages__ek0_testing
  - chat_configurations__ek0_testing
  - collection_utils_test_kt

=== Check Non-Existent Collection ===
Collection 'nonexistent_collection_xyz_876720980723458' exists: false

=== Cleanup ===
Deleted collection 'collection_utils_test_kt'

=== Collection Utilities Example Complete ===

BUILD SUCCESSFUL in 5s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientConcurrencyStages.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
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

BUILD SUCCESSFUL in 6s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientConvenienceMethods.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
=== ekoDB Convenience Methods Example ===

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== Native Object Creation ===
✓ Created record: EkoRecord(fields={id=StringValue(value=WctYmmJ_p9o2d8UGbZVNxUfGj-b2cK1LrIvAdi9RKhExu5EhK_-8KPMi_OLvwxGEsD6h-tdOZTPgIyrXvNXRXg)})

=== Upsert Operation ===
✓ First upsert (update): EkoRecord(fields={email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice.j@newdomain.com)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), id=StringValue(value=WctYmmJ_p9o2d8UGbZVNxUfGj-b2cK1LrIvAdi9RKhExu5EhK_-8KPMi_OLvwxGEsD6h-tdOZTPgIyrXvNXRXg), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)})})
✓ Second upsert (insert): EkoRecord(fields={id=StringValue(value=new-user-id)})

=== Find One Operation ===
✓ Found user by email: EkoRecord(fields={name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), id=StringValue(value=WctYmmJ_p9o2d8UGbZVNxUfGj-b2cK1LrIvAdi9RKhExu5EhK_-8KPMi_OLvwxGEsD6h-tdOZTPgIyrXvNXRXg), email=ObjectValue(value={value=StringValue(value=alice.j@newdomain.com), type=StringValue(value=String)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)})})
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

BUILD SUCCESSFUL in 6s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientCryptoStages.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✓ Client created
✓ crypto_demo_hmac_kt saved
✓ crypto_demo_aes_kt saved
✓ crypto_demo_uuid_kt saved
✓ crypto_demo_totp_kt saved
✓ crypto_demo_encoding_kt saved

Invoke them with:
  POST /api/functions/crypto_demo_hmac_kt { "payload": "hi" }
  POST /api/functions/crypto_demo_aes_kt { "plaintext": "secret" }
  POST /api/functions/crypto_demo_uuid_kt
  POST /api/functions/crypto_demo_totp_kt
  POST /api/functions/crypto_demo_encoding_kt { "title": "Héllo World" }

✓ Cleaned up demo functions

BUILD SUCCESSFUL in 5s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientDistinctValues.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
Kotlin distinct-values example passed

BUILD SUCCESSFUL in 5s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientDocumentTtl.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Document TTL Example ===

=== Insert with TTL ===
✓ Inserted document with 10s TTL
  Document ID: Tfivm-iT53C-u55eQSNZlN9_MnziAMOTB_Rpk9m1g4ZP8fgWc_TbgqizbkBEm6BI3UdPIi457rDe9766QN80oQ

=== Verify Document Exists ===
✓ Document found: id, session_id, created_at, user_id, ttl

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: CcxxxVHIB15EY439DEWTgtlg6GYjCsl6dcuy_fRvHY0WHh90SbdVSaH9RqFIshcUK3pvOpfl5gEHpoFxiHnYBA

=== TTL Expiration ===
✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document ===
✓ Long TTL document still exists: cache_key, id, value, ttl

=== Delete Document ===
✓ Deleted document

=== Cleanup ===
✓ Deleted collection: kotlin_ttl_example

=== Example Complete ===

BUILD SUCCESSFUL in 5s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientEdgeCache.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB as Edge Cache - Simple Example ===

Setting up edge cache collection...
✓ Cache entry created

Creating edge cache lookup function...
✓ Edge cache function created: EvpGqy7ZQyT_GjpuhtUg8Fn--VpYbA6j5TUdXW5QiJ09QyponjR68myLxgoVH4-RG5qJYzE2i2BU0R_M5RcvSw

Call 1: Cache lookup
Found 1 cached entries
Response time: 15ms

Call 2: Cache lookup (connection warm)
Found 1 cached entries
Response time: 5ms

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

BUILD SUCCESSFUL in 5s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientFunctionComposition.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
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
   ⏱️  Duration: 68ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 8ms
   📊 Records: 1
   🚀 Cache speedup: 8.5x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: get_verified_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: {"value":"User 1","type":"String"}
   Department: {"value":"engineering","type":"String"}

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - get_verified_user: Specific workflow

🧹 Cleaning up...
✅ Cleanup complete

✅ All composition examples completed!

BUILD SUCCESSFUL in 6s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientFunctionContract.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
client_function_contract: ok

BUILD SUCCESSFUL in 5s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientFunctions.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
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

✅ Function saved: qrxFsE3M1aUcJDmdzB9Sg9uidgW9q_xYw7v3hqO6-IZQuN_yMUm5nPX7YT1ulcAP38yyWI9Igc2LP71osFjYTQ
📊 Found 5 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

✅ Function saved
📊 Found 3 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 2 groups
   {"status":{"value":"active","type":"String"},"count":{"value":5,"type":"Integer"},"avg_score":{"type":"Float","value":60.0}}
   {"status":{"type":"String","value":"inactive"},"count":{"type":"Integer","value":5},"avg_score":{"value":50.0,"type":"Float"}}
⏱️  Execution time: 0ms

📝 Example 4: function Management

📋 Total functions: 11
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
📊 Total user count: {"type":"Integer","value":10}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Deleted test functions
✅ Deleted collection

✅ All examples completed successfully!

💡 Key Advantages of Using the Client:
   • Automatic token management
   • Type-safe Stage builders
   • Built-in error handling

BUILD SUCCESSFUL in 5s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientFunctionsAdvanced.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
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
   {"count":{"type":"Integer","value":3},"avg_price":{"value":365.6666666666667,"type":"Float"},"category":{"value":"Furniture","type":"String"}}
   {"category":{"type":"String","value":"Electronics"},"avg_price":{"value":367.0,"type":"Float"},"count":{"value":5,"type":"Integer"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced function examples finished!

BUILD SUCCESSFUL in 5s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientFunctionsAi.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
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
   Vector databases are designed to store and search **embeddings**—numeric representations of text, images, audio, or other data. Their main benefits are:

- **Semantic search:** Find items by meaning or similarity, even when they don’t share exact keywords.
- **Fast similarity matching at scale:** Use specialized indexes to search large collections of vectors efficiently.
- **Better AI applications:** Retrieve relevant information for chatbots, recommendation systems, image search, and retrieval-augmented generation (RAG).
- **Flexible data matching:** Compare different types of content in a shared embedding space, such as a text query against images.
- **Metadata filtering:** Combine similarity search with filters such as date, category, or access permissions.
- **Practical data management:** Many systems support adding, updating, and deleting vectors as data changes.

They’re often used alongside traditional databases: a relational or document database handles structured records, while a vector database handles similarity search.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embed function saved
📊 Generated 2 embeddings
   Dimensions: 1536
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All AI function examples finished!

💡 This example demonstrates:
   ✅ Chat completions with system/user messages
   ✅ Embedding generation for text

BUILD SUCCESSFUL in 10s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientFunctionsComplete.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
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

✅ Function saved: iEfSsEx5GIPgqUFUU-wanoVZlb2zXTY49Hwnwdob9P8qjuxTmeWnxtBkGhs5GGSCXrJm77jlmGqocLbPLDrh7A
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

BUILD SUCCESSFUL in 5s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientFunctionsCrud.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
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
   {"count":{"type":"Integer","value":7},"status":{"value":"active","type":"String"}}
   {"status":{"type":"String","value":"inactive"},"count":{"type":"Integer","value":3}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD function examples finished!

BUILD SUCCESSFUL in 5s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientFunctionsKvWrapped.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
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

✅ Inserted order: StringValue(value=Nq1RoyGs7Op7-AD32FVq8WnsXc-faoKRxB7vCyPbOWc8Pn2PWBI9ayWUy3E6FDyk8eSppACMWHd6ULVenWvJqw)
✅ Inserted 2 products

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: fkyg4D_cXlQRSzyv8PuJ1EJJXfjpbEuLGL1ifwr1TH44QOVnAwghzjiGXtO50cwjjU6TMg5NMOMeBpisCLNPBQ
📊 Created order via function
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"type":"Object","value":{"role":"admin","userId":"user_abc"}}
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: TPGyCIXkTkMgFEKxCuhrYFzGUR1Q17nB_OPVZGchwwhFSCKoV_1ziWVheVRQrqvdPfSjEXZdNUW9b9ewHt_UFA
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: T7NPSCevY9T-j7hB7_HWXCcEkvSnR2-8DfYk0XZUQtekdAuZO4fewC1uB9jrV1iYuwm7LxcSdFH4moHbucBlQw
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

BUILD SUCCESSFUL in 6s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientFunctionsSearch.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
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
   1. {"value":"Introduction to Machine Learning","type":"String"} ({"type":"String","value":"AI"})
   2. {"type":"String","value":"Vector Databases Explained"} ({"type":"String","value":"Database"})
   3. {"value":"Database Design Principles","type":"String"} ({"type":"String","value":"Database"})
   4. {"value":"Getting Started with ekoDB","type":"String"} ({"type":"String","value":"Database"})
   5. {"type":"String","value":"Natural Language Processing"} ({"type":"String","value":"AI"})
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"category":{"type":"String","value":"Database"},"count":{"value":3,"type":"Integer"}}
   {"count":{"value":2,"type":"Integer"},"category":{"value":"AI","type":"String"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search function examples finished!

BUILD SUCCESSFUL in 5s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientGoalTemplates.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
=== ekoDB Goal Template CRUD Example (Kotlin) ===

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
--- Creating goal template ---
Created template: Data Migration (id: gSV0sFqDWnpCnU2XtWBn2dFTK728A-kBjiAfyqgf8ztE8YSNIjjsPEVifKWnmWkHuuFIxizfbx9TWfz9hOZUEQ)

--- Listing templates ---
Templates: {"count":1,"items":[{"description":{"type":"String","value":"Template for migrating data between schemas"},"id":"gSV0sFqDWnpCnU2XtWBn2dFTK728A-kBjiAfyqgf8ztE8YSNIjjsPEVifKWnmWkHuuFIxizfbx9TWfz9hOZUEQ","owner":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"steps":{"type":"Array","value":[{"description":"Analyze source schema"},{"description":"Create target schema"},{"description":"Migrate records"},{"description":"Validate results"}]},"title":{"type":"String","value":"Data Migration"}}],"total":1}

--- Getting template ---
Fetched: {"type":"String","value":"Data Migration"}

--- Updating template ---
Updated description: {"type":"String","value":"Updated: comprehensive data migration workflow"}

--- Deleting template ---
Template deleted successfully

✓ Goal template CRUD example completed

BUILD SUCCESSFUL in 5s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientGoalsTasksAgents.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Goals, Tasks & Agents Example ===

--- Creating goal ---
Created goal: 3J0NsokkfNSswqf5AznOvlNXEbGzmLpJTlmGkoG3ZqJITMBDN5j3FDjNDagZ2Hz7cV5HW5O1adl1oFAk0Aj6rA

--- Listing goals ---
Goals: {"count":1,"goals":[{"created_at":{"type":"DateTime","value":"2026-11-02T04:50:51.861368+00:00"},"description":{"type":"String","value":"Ship the next major release"},"id":"3J0NsokkfNSswqf5AznOvlNXEbGzmLpJTlmGkoG3ZqJITMBDN5j3FDjNDagZ2Hz7cV5HW5O1adl1oFAk0Aj6rA","owner":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"status":{"type":"String","value":"pending"},"steps":{"type":"String","value":"[{\"description\":\"Run test suite\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0"},"updated_at":{"type":"DateTime","value":"2026-11-02T04:50:51.861368+00:00"}}],"total":1}

--- Getting goal ---
Fetched: {"type":"String","value":"Deploy v2.0"}

--- Updating goal ---
Updated description: {"type":"String","value":"Ship v2.0 with full test coverage"}

--- Searching goals ---
Search results: {"count":1,"items":[{"_score":12.870000000000001,"created_at":{"type":"DateTime","value":"2026-11-02T04:50:51.861368+00:00"},"description":{"type":"String","value":"Ship v2.0 with full test coverage"},"id":"3J0NsokkfNSswqf5AznOvlNXEbGzmLpJTlmGkoG3ZqJITMBDN5j3FDjNDagZ2Hz7cV5HW5O1adl1oFAk0Aj6rA","owner":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"status":{"type":"String","value":"pending"},"steps":{"type":"String","value":"[{\"description\":\"Run test suite\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0"},"updated_at":{"type":"DateTime","value":"2026-11-02T04:50:51.891048+00:00"}}]}

--- Goal step lifecycle ---
Step 0 started: "3J0NsokkfNSswqf5AznOvlNXEbGzmLpJTlmGkoG3ZqJITMBDN5j3FDjNDagZ2Hz7cV5HW5O1adl1oFAk0Aj6rA"
Step 0 completed: "3J0NsokkfNSswqf5AznOvlNXEbGzmLpJTlmGkoG3ZqJITMBDN5j3FDjNDagZ2Hz7cV5HW5O1adl1oFAk0Aj6rA"
Step 1 started: "3J0NsokkfNSswqf5AznOvlNXEbGzmLpJTlmGkoG3ZqJITMBDN5j3FDjNDagZ2Hz7cV5HW5O1adl1oFAk0Aj6rA"
Step 1 failed: "3J0NsokkfNSswqf5AznOvlNXEbGzmLpJTlmGkoG3ZqJITMBDN5j3FDjNDagZ2Hz7cV5HW5O1adl1oFAk0Aj6rA"

--- Completing goal ---
Goal status after complete: {"type":"String","value":"pending_review"}

--- Approving goal ---
Goal status after approve: {"type":"String","value":"in_progress"}

--- Creating goal for rejection ---
Goal status after reject: {"type":"String","value":"failed"}

--- Creating task ---
Created task: nLN8A1Xm82iIQVqd0oajlmTSW17GAQeMGGd8PTEA_HZAXxrcMEB2d6vgEFWHNK7o_4xobd2stvOqbFPVkPaeFA

--- Listing tasks ---
Tasks: {"count":1,"items":[{"cron":{"type":"String","value":"0 2 * * *"},"description":{"type":"String","value":"Full database backup every night at 2 AM"},"id":"nLN8A1Xm82iIQVqd0oajlmTSW17GAQeMGGd8PTEA_HZAXxrcMEB2d6vgEFWHNK7o_4xobd2stvOqbFPVkPaeFA","name":{"type":"String","value":"Nightly Backup"},"owner":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}}}],"total":1}

--- Getting task ---
Task name: {"type":"String","value":"Nightly Backup"}

--- Starting task ---
Task status: {"type":"String","value":"running"}

--- Succeeding task ---
Task status after succeed: {"type":"String","value":"active"}

--- Pausing task ---
Task status: {"type":"String","value":"paused"}

--- Resuming task ---
Task status: {"type":"String","value":"active"}

--- Failing task ---
Task marked as failed

--- Checking due tasks ---
Due tasks: {"count":0,"items":[]}

--- Deleting tasks ---
Tasks deleted

--- Creating agent ---
Created agent: CnMA24mUCtQKNBANbxpA-JMYP1mRsrNxtGCi1aTXcQ96aibBQ8HvRTkPQkJdEiaaFQFpNVT4RxYq0cU_obd9Ew — null

--- Listing agents ---
Agents: {"count":1,"items":[{"deployment_id":{"type":"String","value":"deploy_kt_example"},"id":"CnMA24mUCtQKNBANbxpA-JMYP1mRsrNxtGCi1aTXcQ96aibBQ8HvRTkPQkJdEiaaFQFpNVT4RxYq0cU_obd9Ew","llm_model":{"type":"String","value":"gpt-6-luna"},"name":{"type":"String","value":"DataBot"},"owner":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"run_policy":{"type":"String","value":"{\"trigger\":{\"allow_write_operations\":false,\"write_tools\":[]},\"eval\":{\"allow_write_operations\":false,\"write_tools\":[]},\"customer\":{\"allow_write_operations\":false,\"write_tools\":[]}}"},"system_prompt":{"type":"String","value":"You are a data analysis assistant."}}],"total":1}

--- Getting agent by ID ---
Agent: {"type":"String","value":"DataBot"}

--- Getting agent by name ---
By name: {"type":"String","value":"DataBot"}

--- Updating agent ---
Updated agent system_prompt

--- Agents by deployment ---
Agents for deployment: {"count":0,"items":[]}
WARNING: agents-by-deployment omitted created agent CnMA24mUCtQKNBANbxpA-JMYP1mRsrNxtGCi1aTXcQ96aibBQ8HvRTkPQkJdEiaaFQFpNVT4RxYq0cU_obd9Ew; TODO: check/fix the server-side deployment lookup

--- Deleting agent ---
Agent deleted

--- Cleanup: deleting goals ---
Goals deleted

=== Example Complete ===

BUILD SUCCESSFUL in 6s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientJoins.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
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
  - Laptop ($1200) by Alice Johnson
  - Mouse ($25) by Alice Johnson

=== Example 5: Complex join with multiple conditions ===
✓ Found 2 users with example.com emails
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✓ Deleted test collections

✓ Join operations example completed successfully

BUILD SUCCESSFUL in 6s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientJwtAuthFlow.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
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

BUILD SUCCESSFUL in 6s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientKvLinks.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - KV Links Example ===

--- Setting KV entry ---
Set key: user:alice

--- Linking documents ---
Linked order_001: null
Linked order_002: null
Linked inv_100: null

--- Getting links ---
Links for user:alice: [{"collection":"invoices","document_id":"inv_100","field_path":null,"created_at":"2026-11-02T04:51:12.531071Z","last_accessed":"2026-11-02T04:51:12.535231Z","metadata":{}},{"collection":"orders","document_id":"order_001","field_path":null,"created_at":"2026-11-02T04:51:12.523246Z","last_accessed":"2026-11-02T04:51:12.535231Z","metadata":{}},{"collection":"orders","document_id":"order_002","field_path":null,"created_at":"2026-11-02T04:51:12.527512Z","last_accessed":"2026-11-02T04:51:12.535231Z","metadata":{}}]

--- Unlinking document ---
Unlinked order_002: null

--- Verifying remaining links ---
Remaining links: [{"collection":"orders","document_id":"order_001","field_path":null,"created_at":"2026-11-02T04:51:12.523246Z","last_accessed":"2026-11-02T04:51:12.549942Z","metadata":{}},{"collection":"invoices","document_id":"inv_100","field_path":null,"created_at":"2026-11-02T04:51:12.531071Z","last_accessed":"2026-11-02T04:51:12.549942Z","metadata":{}}]

--- Cleanup ---
Unlinked remaining documents
Deleted key: user:alice

=== Example Complete ===

BUILD SUCCESSFUL in 5s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientKvOperations.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - KV Operations Example ===

=== KV Set ===
✓ Set key: user:123

=== KV Get ===
✓ Retrieved value: {"type":"Object","value":{"role":"admin","name":"Alice","email":"alice@example.com"}}

=== KV Set with TTL ===
✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key ===
✓ Session value: {"value":{"created_at":1793595078744,"user_id":"123"},"type":"Object"}
  (Will expire in 10 seconds)

=== KV Batch Set ===
✓ Batch set 3 keys
  kv_ops_kt_1793595078620:config:db: success
  kv_ops_kt_1793595078620:config:cache: success
  kv_ops_kt_1793595078620:config:api: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  kv_ops_kt_1793595078620:config:db: {"port":5432,"host":"localhost"}
  kv_ops_kt_1793595078620:config:cache: {"enabled":true,"ttl":3600}
  kv_ops_kt_1793595078620:config:api: {"timeout":30,"retries":3}

=== KV Exists ===
✓ Key exists: true

=== KV Find (Pattern Query) ===
✓ Found 3 keys matching 'config:.*'

=== KV Query (Alias for Find) ===
✓ Total keys in store: 5

=== KV Delete ===
✓ Deleted key: user:123

=== Verify Deletion ===
✓ Key exists after delete: false

=== KV Batch Delete ===
✓ Batch deleted 3 keys
  kv_ops_kt_1793595078620:config:db: deleted
  kv_ops_kt_1793595078620:config:cache: deleted
  kv_ops_kt_1793595078620:config:api: deleted

=== Example Complete ===

BUILD SUCCESSFUL in 5s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientKvPrecision.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - KV Precision: Float vs Decimal ===

=== Test 1: Using Kotlin Doubles (LOSES PRECISION) ===
Stored products with float prices

Retrieved float prices:
  Widget A: $29.99 (expected $29.99) MATCH
  Widget B: $39.99 (expected $39.99) MATCH
  Widget C: $49.99 (expected $49.99) MATCH

=== Test 2: Using fieldDecimal() (PRESERVES PRECISION) ===
Stored products with decimal prices

Retrieved decimal prices:
  Widget A: $29.99 (expected $29.99)
  Widget B: $39.99 (expected $39.99)
  Widget C: $49.99 (expected $49.99)

=== Test 3: Sum Calculation Comparison ===
  Float sum: $119.97 (expected $119.97)
  Decimal sum: $119.97 (expected $119.97)

=== Test 4: Extreme Precision Example ===
  Float 0.1 + 0.2 = 0.30000000000000004 (should be 0.3)
  Decimal "0.30" = 0.30 (exact!)

=== Cleanup ===
Cleaned up 8 test keys

=== Summary ===
Use fieldDecimal() for monetary values, percentages, and
any case where floating-point errors are unacceptable.
fieldDecimal() stores values as strings internally,
preserving exact precision across all operations.

=== Example Complete ===

BUILD SUCCESSFUL in 5s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientPathRoutedFunction.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
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

BUILD SUCCESSFUL in 5s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientProjection.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
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

BUILD SUCCESSFUL in 5s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientQueryBuilder.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
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
  - Score: ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=92)})
  - Score: ObjectValue(value={value=IntegerValue(value=88), type=StringValue(value=Integer)})

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

BUILD SUCCESSFUL in 5s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientRawCompletionStream.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Raw Completion Stream (SSE) Example ===

--- Basic SSE Raw Completion ---
Response: It depends on the context: **red, yellow, and blue** are the traditional painting primaries; **red, green, and blue** are the primaries of light.

--- Structured Output via SSE ---
JSON response: [
  {"name": "Earth", "diameter_km": 12742},
  {"name": "Mars", "diameter_km": 6779},
  {"name": "Jupiter", "diameter_km": 139820}
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello!

=== Done ===

BUILD SUCCESSFUL in 10s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientSchedules.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Schedules Example ===

--- Creating schedule ---
Created schedule: 4cc04d83-2e74-4838-8607-a9422a9014a4 — "Hourly Health Check"

--- Listing schedules ---
Schedules: {"count":1,"schedules":[{"created_at":"2026-11-02T04:52:01.609234Z","cron_expression":"0 0 * * * *","description":"Ping all services every hour","enabled":true,"function_label":"schedule_noop_kotlin_49266_1793595121460","id":"4cc04d83-2e74-4838-8607-a9422a9014a4","last_execution":null,"name":"Hourly Health Check","next_execution":"2026-11-02T05:00:00Z","parameters":{},"stats":{"avg_execution_time_ms":0.0,"failed_executions":0,"last_error":null,"successful_executions":0,"total_executions":0},"timezone":"UTC","updated_at":"2026-11-02T04:52:01.609234Z"}]}

--- Getting schedule ---
Fetched: "Hourly Health Check" (cron: "0 0 * * * *")

--- Updating schedule ---
Updated cron: "0 */30 * * * *"

--- Triggering schedule ---
Trigger response: {"schedule_id":"4cc04d83-2e74-4838-8607-a9422a9014a4","status":"triggered"}

--- Pausing schedule ---
Enabled after pause: false

--- Resuming schedule ---
Enabled after resume: true

--- Deleting schedule ---
Schedule deleted successfully

=== Example Complete ===

BUILD SUCCESSFUL in 5s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientSchemaManagement.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'kotlin_schema_example' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: StringValue(value=XN_qBPW59ii6cUY060Xk8FkrL7fc3i7JgTI0puIgSTXoxPNpkiVINy6R4lYMyd5IrkBBrbz9jrcmbcfd1TEKRQ)
✓ Inserted user 2: StringValue(value=FaUkJZkbvGkL0RzDbWoJCqPiY_AGgzGs7QXFvmINB9LrDBCuJ0jSfiy1JWZVNUq7pw_455Te1YOgSRNjBkcLIQ)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections ===
✓ Total collections: 27
  Sample: [schema_users_client_go, audit__ek0_testing, schema_documents_client_js, schema_documents_client_ts, schema_employees_client_ts]

=== Cleanup ===
✓ Deleted collection: kotlin_schema_example

✓ All schema management operations completed successfully

BUILD SUCCESSFUL in 5s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientSearch.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
Text results: 2; execution time: 0 ms
{"type":"String","value":"Rust Programming Question"}: score=0.9162907318741551, matched=[title]
{"value":"Rust Programming Answer","type":"String"}: score=0.9162907318741551, matched=[title]
Paired vector search: document=Rust Programming Answer, query=Rust Programming Question
Filtered vector results: 2
Custom-weight hybrid results: 3
Raw search: {"results":[{"record":{"category":{"type":"String","value":"programming"},"query_embedding":{"value":[1.0,0.0,0.0],"type":"Vector"},"title":{"type":"String","value":"Rust Programming Question"},"document_embedding":{"value":[0.0,1.0,0.0],"type":"Vector"},"id":"nhKcqV8-cp9m5vrIXr3mBUrA-7Rlxv1xyeDfo1CM8ajLnGXE80SUjISRvLg4DFPSiZh3tchSCaLCg147h_06EQ"},"score":1.0,"matched_fields":[]},{"record":{"category":{"value":"programming","type":"String"},"id":"ZmSJNxNRKgX96u5-QEmKrzzUCB--SNXgxKtswWkequ_0DMrZ1SGfwDs4PSZm3D7j1H0LI92m_monnQuVAkioaQ","title":{"type":"String","value":"Rust Programming Answer"},"document_embedding":{"type":"Vector","value":[1.0,0.0,0.0]},"query_embedding":{"type":"Vector","value":[0.0,1.0,0.0]}},"score":1.0,"matched_fields":[]}],"total":2,"execution_time_ms":0}

BUILD SUCCESSFUL in 5s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientSimpleCrud.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Simple CRUD Example ===

=== Create ===
✓ Inserted user: EkoRecord(fields={id=StringValue(value=LiyCRruRgK5q6McyXXgt92kRok2khKsMDCAr2oZnwzzAqVQpmipMFQMkAev3_hxnFJyJZ-tqtRSz00CVQf7EEw)})
  User ID: LiyCRruRgK5q6McyXXgt92kRok2khKsMDCAr2oZnwzzAqVQpmipMFQMkAev3_hxnFJyJZ-tqtRSz00CVQf7EEw

=== Read ===
✓ Found user by ID: EkoRecord(fields={categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), metadata=ObjectValue(value={value=ObjectValue(value={nested=ObjectValue(value={deep=BooleanValue(value=true)}), key=StringValue(value=value)}), type=StringValue(value=Object)}), id=StringValue(value=LiyCRruRgK5q6McyXXgt92kRok2khKsMDCAr2oZnwzzAqVQpmipMFQMkAev3_hxnFJyJZ-tqtRSz00CVQf7EEw), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=28)}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), created_at=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Sun Nov 01 23:52:20 EST 2026)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), embedding=VectorValue(value=[0.1, 0.2, 0.3, 0.4, 0.5])})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Alice Johnson
  email (String): alice@example.com
  age (Integer): 28
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): Sun Nov 01 23:52:20 EST 2026
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1, tag2, tag3]
  metadata (Object): {nested={deep=true}, key=value}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): [electronics, computers]
  data (Bytes): 11 bytes
Record fields: categories, data, metadata, id, active, user_id, age, price, email, created_at, name, tags, embedding

=== Update ===
✓ Updated user: EkoRecord(fields={created_at=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Sun Nov 01 23:52:20 EST 2026)}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), embedding=VectorValue(value=[0.1, 0.2, 0.3, 0.4, 0.5]), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)]), type=StringValue(value=Array)}), price=ObjectValue(value={value=FloatValue(value=99.99), type=StringValue(value=Float)}), id=StringValue(value=LiyCRruRgK5q6McyXXgt92kRok2khKsMDCAr2oZnwzzAqVQpmipMFQMkAev3_hxnFJyJZ-tqtRSz00CVQf7EEw), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={nested=ObjectValue(value={deep=BooleanValue(value=true)}), key=StringValue(value=value)})}), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])})})

=== Query ===
✓ Found 1 users matching query
  - EkoRecord(fields={name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), embedding=VectorValue(value=[0.1, 0.2, 0.3, 0.4, 0.5]), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={nested=ObjectValue(value={deep=BooleanValue(value=true)}), key=StringValue(value=value)})}), price=ObjectValue(value={value=FloatValue(value=99.99), type=StringValue(value=Float)}), created_at=ObjectValue(value={value=StringValue(value=Sun Nov 01 23:52:20 EST 2026), type=StringValue(value=String)}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)}), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])}), id=StringValue(value=LiyCRruRgK5q6McyXXgt92kRok2khKsMDCAr2oZnwzzAqVQpmipMFQMkAev3_hxnFJyJZ-tqtRSz00CVQf7EEw), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])})})

=== Delete ===
✓ Deleted user with ID: LiyCRruRgK5q6McyXXgt92kRok2khKsMDCAr2oZnwzzAqVQpmipMFQMkAev3_hxnFJyJZ-tqtRSz00CVQf7EEw

✓ Confirmed user was deleted

=== Cleanup ===
✓ Deleted collection: kotlin_users_example

=== Example Complete ===

BUILD SUCCESSFUL in 5s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientSimpleWebsocket.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
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
  {"data":[{"name":{"value":"Test User","type":"String"},"id":"fQiH6FlphQ0GC51Hvvnaq3NYogJgsine-eCWhMzLD2k_gTdwezSA_bxsqV_3Ehgd6SorRTI8yTfGnWqagaHXMA","status":{"value":"active","type":"String"}}]}

=== Close WebSocket ===
✓ WebSocket closed

=== Cleanup ===
✓ Deleted collection: kotlin_websocket_example

=== Example Complete ===

BUILD SUCCESSFUL in 5s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientSwrNative.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
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

🧹 Cleaning up...
✓ Deleted 0 test functions and owned SWR collections

Example 1: Basic Native SWR
────────────────────────────────────────────────────────────────────────────────
Single function replaces KvGet → If → HttpRequest → KvSet pipeline
✓ Created native SWR function: github_user_native_kt (_PBRuqLf2H_HGFB7x1U6onW1K8kN-9u7R1ntc0zMglFt-7feT8iGl8AQ97K8iqem_D_cEpyY5BeEVyx5twWIig)

First call (cache miss - will fetch from GitHub API):
  Response time: 180ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 5ms
  Speedup: 36.0x faster 🚀


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR function with audit trail: product_swr_audit_kt (F_HzxH5JlKgfd6Mo_FhssTgNgLVj4YE-bXTTkFJ0kqzVs0zikTb03sZYPVsJx5y8scGFRJT327zkr0m2Ej6xTA)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_kt' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_kt (09tcv4MPRhP_n39-HHdN9RDKuiwZEeBLSbrJByy808Bodn5S0nI6LfUpXv-AoSjXmoTQCYbhMiVf3arYALyc_Q)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_kt' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL function: flexible_cache_kt (bc7p6Ol5OSwL59zyDTgf_dc9zbzY0O5TEpgokf9VHcCWyf-GjQRZLjFOM166YumYaloPekj3x-OyoiE3OPjWwg)
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
✓ Deleted 4 test functions and owned SWR collections

✅ All examples completed!

BUILD SUCCESSFUL in 6s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientSwrPattern.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Setting up cache collection...
✓ Cache entry created

Step 2: Create SWR cache lookup function
✓ Created SWR function: swr_cache_lookup_kt_1793595160344 (GHdl3bOimiQmO5dei7Zw_3DG8clCZWiej_CSwxYc5ZqG9AlAxWocUsEOVrPTLm1TJCaLLAsNopG6Mn8TpEU_yQ)

Step 3: First call - Cache lookup
Found 1 cached entries
✓ Cache lookup complete

Step 4: Second call - Fast cache hit
Response time: 4ms (served from cache)
✓ Lightning fast cache hit

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
🧹 Cleaning up...
✓ Cleanup complete


BUILD SUCCESSFUL in 5s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientTransactions.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: urhPwbzDVcXy-UuHaQqR-467T3WAPHpi9nmhinqNA7c5cC39Aa4iS7y5GVX_p4iS_nLgysu4eGaFnFQ6bAS0Jw
Created Bob: $500 - ID: PdZ2RUGm-FJoju43MQneEZFTCaR0C6GPV05VUIs7nDyFlzBKhxzFY0qlXGmQBWr3hhemU-96-ZHDUG0xGM4VMg

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 2830f429-5a2a-4355-b8d6-9ded3ce84c05

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
New transaction: 5c0023bc-9f3c-4b91-b498-4437bf608ad9
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed

BUILD SUCCESSFUL in 5s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientUserFunctions.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - User Functions Example ===

=== Create User Function ===
Created user function with ID: x2HI5WDBvWDcgTA9jFfEPd-FglUiVxDMqWrlFMVk3ifud-Qau176rcTgUlKQs4BHRtuCRnHf4PdBOL0ym1x3zA

=== Get User Function ===
Retrieved: "get_active_users_kt" - "Get Active Users (Kotlin)"
Description: "Fetches all users and filters by active status"

=== List All User Functions ===
Found 10 user functions:
  - "client_functions_get_active_users_kt": "Get Active Users (Updated)"
  - "conc_demo_rl_skip_ts_50152_1793594691316": "Rate-limit (skip mode)"
  - "conc_demo_rl_fail_ts_50152_1793594691316": "Rate-limit (fail mode)"
  - "get_active_users_client_ts": "Get Active Users (Updated)"
  - "get_active_users_client_js": "Get Active Users (Updated)"

=== List User Functions by Tag ===
Found 1 user functions with 'kotlin' tag:
  - "get_active_users_kt"

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

=== User Functions Example Complete ===

BUILD SUCCESSFUL in 5s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientWebsocketChatStream.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== WebSocket Chat Streaming Example (Kotlin) ===

Created chat session: i3GITuNew1Th22J-B5VC4fs5J27MNhr7_VkklO46ZDBshfEHXKfiilUBfZL3CRxEWplSM1nANirc-JtqBFs7jg

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"gpt-5.6-luna","provider":"openai"}The capital of France is **Paris**.

--- Stream ended ---
Message ID: KzyEubEu-IA_jRoLx_oQIPks7ApG5vo47I63EPg9nfJ2pJatm-zLdhn8yWu5Tafxrqkx3CKoL572D53DIMeFrQ
Execution time: 1189ms
Token usage: {"cached_input_tokens":0,"completion_tokens":13,"prompt_tokens":23,"thinking_tokens":0,"total_tokens":36}

Full response: {"__progress":"received"}{"__progress":"generating","model":"gpt-5.6-luna","provider":"openai"}The capital of France is **Paris**....

BUILD SUCCESSFUL in 6s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientWebsocketSubscribe.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
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
✓ Subscribed (subscription_id: sub_f5d9e507c2a94c2ab0aa5d6bedf9fcb7)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: "Gkma3OkoXYKdgBLVoNqnS-zCw6KXPOoNVyrTQTkWbQMEInR0w8HWZ6PZTFB93ULxOyEAqISsG-P6xa7n-hPLgA"

  📡 Notification received:
     Event:      "insert"
     Collection: "ws_subscribe_example_kt"
     Record IDs: ["Gkma3OkoXYKdgBLVoNqnS-zCw6KXPOoNVyrTQTkWbQMEInR0w8HWZ6PZTFB93ULxOyEAqISsG-P6xa7n-hPLgA"]
     Timestamp:  "2026-11-02T04:53:07.033078+00:00"

Inserting record 2...
✓ Inserted: "2TGzJLBYOkxaQYsvcgf9riYDj6y1OnyGzBQZsVw8FpevAQPlqv5UPTrTlUnfNll5MOcD6X_oJqx3Tu5qjfo34g"

  📡 Notification received:
     Event:      "insert"
     Record IDs: ["2TGzJLBYOkxaQYsvcgf9riYDj6y1OnyGzBQZsVw8FpevAQPlqv5UPTrTlUnfNll5MOcD6X_oJqx3Tu5qjfo34g"]

=== Unsubscribing ===
✓ Unsubscribed: {"payload":{"data":{"collection":"ws_subscribe_example_kt","found":true,"unsubscribed":true}},"type":"Success"}

✓ WebSocket subscription example completed successfully
✓ Deleted collection 'ws_subscribe_example_kt'

BUILD SUCCESSFUL in 5s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: ClientWebsocketTtl.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - WebSocket TTL Example ===

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: StringValue(value=neCgaFK7ScvDIPqciZ-FGgTU_qRFGXB05tgpgpT3ur9pUxmtDn2WGIqf6WeUrg_5xNjSWtQQNlhUvIE5uprf5A)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"data":[{"value":{"type":"Integer","value":42},"created_at":{"value":1793595193140,"type":"Integer"},"id":"neCgaFK7ScvDIPqciZ-FGgTU_qRFGXB05tgpgpT3ur9pUxmtDn2WGIqf6WeUrg_5xNjSWtQQNlhUvIE5uprf5A","name":{"value":"WebSocket TTL Test","type":"String"},"ttl":"2026-11-02T05:53:13.255277Z"}]}

✓ WebSocket closed

=== Cleanup ===
✓ Deleted collection: kotlin_websocket_ttl_example

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration

BUILD SUCCESSFUL in 5s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
=== Running Kotlin example: BypassRippleExample.kt ===
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
=== Bypass Ripple Example ===

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
1. Basic insert (ripple enabled):
   Inserted with ripple: EkoRecord(fields={id=StringValue(value=vIzkbknAa1OQ6_iZzf1v7OwLWfgxhY1jFIR6Yz9qFTyYft-_AWqflxkuyL_KJbqCGJPkyUROJzar9Um4R9wdlw)})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: EkoRecord(fields={id=StringValue(value=XbijY8p3_lqwZtoKW9-ig9n_NHemCWLn7BtY1Uer6qoryyw8A_Z8t4bzRgPFDraKCckVwl0pSz-KpmweTmjsyg)})

3. Update with bypass_ripple:
   Updated with bypass_ripple: EkoRecord(fields={name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Product 1)}), price=ObjectValue(value={value=IntegerValue(value=150), type=StringValue(value=Integer)}), id=StringValue(value=vIzkbknAa1OQ6_iZzf1v7OwLWfgxhY1jFIR6Yz9qFTyYft-_AWqflxkuyL_KJbqCGJPkyUROJzar9Um4R9wdlw)})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: EkoRecord(fields={price=ObjectValue(value={value=IntegerValue(value=500), type=StringValue(value=Integer)}), id=StringValue(value=vIzkbknAa1OQ6_iZzf1v7OwLWfgxhY1jFIR6Yz9qFTyYft-_AWqflxkuyL_KJbqCGJPkyUROJzar9Um4R9wdlw), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Upsert Product)})})

✅ All bypass_ripple operations completed successfully!

BUILD SUCCESSFUL in 5s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
✅ All Kotlin integration tests complete!
🐍 Building Python client package...
🔧 Ensuring maturin is available in .venv...
🔨 Building wheel...
🍹 Building a mixed python/rust project
🐍 Found CPython 3.15 at /Users/tek/.local/bin/python3
🔗 Found pyo3 bindings with abi3-py3.8 support
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.15s
📦 Built wheel for abi3 Python ≥ 3.8 to ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.31.0-cp38-abi3-macosx_11_0_arm64.whl
📦 Installing Python wheel into .venv...
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.31.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.31.0
    Uninstalling ekodb_client-0.31.0:
      Successfully uninstalled ekodb_client-0.31.0
Successfully installed ekodb-client-0.31.0
🧪 Ensuring test dependencies (pytest) in .venv...
✅ Python client package built and installed!
📦 Ensuring Python example dependencies in .venv...

🤖 RAG Conversation System Examples
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Prerequisites:
  1. ekoDB server running (make run in ekodb/)
  2. OPENAI_API_KEY set in server environment
  3. API_BASE_URL and API_BASE_KEY exported in your shell

Building Rust client library...
✓ Rust client built

✓ Python client built and installed into .venv (via build-python-client prerequisite)

Building TypeScript client library...
✓ TypeScript client built

Installing TypeScript client in examples...
✓ TypeScript client installed

Building TypeScript example...
✓ TypeScript example built

Building Go client library...
✓ Go client built

Building Go RAG example...
✓ Go example built

Building Kotlin client library...
✓ Kotlin client built

Building Kotlin RAG example...
✓ Kotlin example built

Running Rust RAG Example...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
=== ekoDB RAG Conversation System ===

This example shows how ekoDB can power a self-improving AI system
that learns from its own conversation history.

=== Step 1: Building Conversation History ===
Storing previous conversations with embeddings...

  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 34 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 967.265459ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 643.677667ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 315.196833ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 330.049167ms
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 358.847791ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 270.113834ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 303.01425ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 314.942916ms
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 381.30725ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 341.626459ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 278.760208ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 310.380209ms
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
    ✓ Generated embedding: 1536 dimensions in 333.013833ms
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages_rust
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 70.6735ms

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

“Memory-safe” and “high-performance” are related but different goals: memory safety prevents invalid access and leaks; performance comes from measuring and controlling allocations, I/O, contention, and query work.

### Keep memory use bounded
- **Stream or page through large results** instead of loading an entire table into memory. Use a stable sort key for pagination so records aren’t skipped or repeated as data changes.
- **Put limits on every potentially large operation**: result size, batch size, request body, concurrency, and timeouts.
- **Apply filters in the database** rather than fetching lots of rows and filtering them in application code.
- Reuse buffers and temporary objects where it’s worthwhile, but avoid complicated pooling unless profiling shows allocation costs matter.

### Use the language’s safety features
- Prefer owned values and safe borrowing/lifetimes over raw pointers or manual memory management. In Rust, avoid `unsafe` unless it is necessary, isolated, and carefully reviewed.
- Make resource lifetimes explicit: close cursors and connections promptly, and use scoped cleanup or RAII-style guards.
- Don’t retain references to temporary data or buffers after their owner can be released.
- Bound queues and worker pools. An unbounded queue can turn a traffic spike into an out-of-memory failure even when the code has no memory-safety bugs.

### Reduce database work
- Select only the fields you need; avoid fetching whole records by default.
- Avoid N+1 queries. Fetch related data in batches or use an appropriate join/aggregation when supported.
- Use transactions for operations that must be atomic, but keep transactions short—don’t hold them open while waiting on network calls or user input.
- Batch writes where possible, while keeping batches small enough to limit memory use, lock duration, and retry cost.
- Use connection pooling with a sensible cap. Too many concurrent connections can hurt the database rather than speed it up.

### Measure before tuning
- Inspect query plans and look for full scans, unnecessary sorting, and missing indexes.
- Index fields used frequently for filtering or sorting, but remember that indexes consume space and make writes more expensive.
- Benchmark with realistic data sizes and concurrency. Track latency percentiles, allocation rate, peak memory, pool waits, and database load—not just average response time.
- Add timeouts, cancellation, and backpressure so slow database work doesn’t accumulate indefinitely.

A useful shape for a data path is: **validate input → issue a bounded, parameterized query → process results incrementally → release resources promptly**. The exact APIs depend on your language and database; if you share those, I can give a concrete implementation pattern.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 453.545375ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2806 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 324.920458ms
    • Function auto-cleaned up by client
✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...


→ Executing text_search()...
  • Collection: rag_messages_rust
  • Query: "ownership system"
  • Limit: 3 results
  • Search method: Full-text with fuzzy matching & stemming
  • No vector embeddings needed - pure keyword search
  ✓ Text search completed in 47.815041ms

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


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Running Python RAG Example...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
ekoDB/ekodb-client/examples/python/rag_conversation_system.py:56: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).
  "created_at": datetime.utcnow().isoformat(),
ekoDB/ekodb-client/examples/python/rag_conversation_system.py:102: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).
  "timestamp": datetime.utcnow().isoformat(),
=== ekoDB RAG Conversation System ===

This example shows how ekoDB can power a self-improving AI system
that learns from its own conversation history.

=== Step 1: Building Conversation History ===
Storing previous conversations with embeddings...

  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 34 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.428s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.314s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.325s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.397s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.632s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.372s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.283s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.306s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.306s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.343s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.325s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.307s
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
    ✓ Generated embedding: 1536 dimensions in 0.310s
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages_py
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.061s

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

Write database code so that **memory use is bounded, data access is predictable, and work is performed in batches**. The exact techniques depend on your language and database, but these principles apply broadly:

- **Fetch only what you need.** Project the required fields instead of loading entire records. Avoid fetching large result sets just to discard most of them.
- **Paginate large reads.** Use a stable sort and a page size you can keep in memory. Prefer cursor-based pagination when offset-based pages become slow or records change during traversal.
- **Avoid N+1 queries.** Don’t issue one query per item in a result set. Fetch related data in batches or use an appropriate join/query pattern.
- **Use indexes that match real access patterns.** Index fields you filter or sort on; avoid adding indexes indiscriminately, since they consume storage and make writes more expensive. Check the query plan rather than guessing.
- **Bound concurrency and buffering.** A large pool of concurrent queries can exhaust memory, connections, or the database. Set limits on in-flight work, queue sizes, and batch sizes.
- **Reuse connections through a pool.** Don’t open a new database connection for every operation. Configure pool limits and timeouts for your workload.
- **Keep transactions short.** Fetch or prepare data before starting a transaction when possible, and commit or roll back promptly. Long transactions can hold locks and retain database resources.
- **Use parameterized operations.** Pass values as parameters rather than building query strings from user input; this helps prevent injection and often improves plan reuse.
- **Handle resource lifetimes explicitly.** Close cursors, statements, connections, and transactions—or use language features such as RAII, `defer`, or context managers. In garbage-collected languages, still avoid retaining large result objects longer than needed.
- **Measure before optimizing.** Track latency, memory, query counts, pool waits, and errors. Test with realistic data and concurrency.

For example, in Rust, a typical safe pattern is to borrow data where possible, fetch bounded pages, and let scoped values release resources automatically:

```rust
async fn load_page(pool: &DbPool, cursor: Option<Cursor>) -> Result<Vec<Item>, DbError> {
    let rows = query_items(pool, cursor, 200).await?;
    Ok(rows)
}
```

The key is that `query_items` should enforce the page limit and select only the fields the caller needs; callers should process one page at a time rather than accumulating the whole dataset.

In ekoDB, the same approach means using `select_fields` and a reasonable `limit` with `query_collection`. Use `query_explain` to inspect whether a filter or sort is doing a full scan, then add an index if the observed access pattern warrants it. A useful rule of thumb: **bound every collection of work—rows, buffers, concurrency, and time.**

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.277s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2903 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.317s
    • Function auto-cleaned up by client
✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...

✓ Found 3 messages mentioning ownership:
  1. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  2. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

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

✓ Cleanup complete


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Running TypeScript RAG Example...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
=== ekoDB RAG Conversation System ===

This example shows how ekoDB can power a self-improving AI system
that learns from its own conversation history.

=== Step 1: Building Conversation History ===
Storing previous conversations with embeddings...

  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 34 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.351s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.322s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.363s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.286s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.371s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.322s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.316s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.322s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.288s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.298s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.313s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.361s
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
    ✓ Generated embedding: 1536 dimensions in 0.341s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages_ts
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.069s
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

Write database code that keeps **both data volume and concurrency bounded**, then measure the actual bottlenecks. “Memory-safe” is about preventing invalid memory access and leaks; “memory-efficient” is about not retaining or copying more data than necessary.

### Core practices

- **Stream or page large result sets.** Don’t load an unbounded query into memory. Use a cursor or keyset pagination, and process a bounded number of rows at a time. Large `OFFSET` pages can become slow; keyset pagination is often better.
- **Select only what you need.** Avoid fetching every column or materializing large blobs when the operation uses only a few fields.
- **Batch writes sensibly.** Batching reduces round trips, but enormous batches can consume memory and hold locks too long. Choose a bounded batch size and tune it with measurements.
- **Avoid N+1 queries.** Fetch related data in a join or a bounded bulk query rather than issuing one query per row.
- **Use indexes that match real access patterns.** Check the query plan for common filters and sorts. Indexes can speed reads, but too many slow writes and use extra storage.
- **Bound concurrency and queues.** A connection pool, worker count, and request queue should all have limits. Backpressure is safer than letting requests pile up in memory.
- **Keep transactions short.** Don’t hold a transaction or lock while doing slow network calls or unrelated work. Use retries only where operations are safe to retry.
- **Use parameterized queries.** This prevents injection and often lets the database reuse query plans.
- **Avoid unnecessary copies and unbounded caches.** In Rust, borrow data where practical and let RAII release resources; in C++, use RAII and smart pointers rather than manual ownership. Keep caches and buffers capped.
- **Profile before optimizing.** Measure allocations, peak memory, query latency, rows read, and connection-pool waits under realistic load.

A useful general pattern is:

```text
for each bounded page of rows:
    process rows
    accumulate a bounded write batch
    persist the batch
    release page and batch memory
```

For Rust specifically, prefer safe database-client APIs, bounded streams, and explicit limits on concurrent tasks; avoid `unsafe` unless profiling demonstrates a need and you can carefully justify and test the invariants.

The biggest wins usually come from controlling result size, eliminating redundant queries, and fixing the query plan—not from low-level memory tricks.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.337s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2492 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.374s
    • Function auto-cleaned up by client
✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...


→ Executing textSearch()...
  • Collection: rag_messages_ts
  • Query: "ownership system"
  • Limit: 3 results
  • Search method: Full-text with fuzzy matching & stemming
  • No vector embeddings needed - pure keyword search
  ✓ Text search completed in 0.038s
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

=== Cleanup ===
✓ Cleanup complete


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Running Go RAG Example...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
=== ekoDB RAG Conversation System ===

This example shows how ekoDB can power a self-improving AI system
that learns from its own conversation history.

=== Step 1: Building Conversation History ===
Storing previous conversations with embeddings...

  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 34 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.258s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.328s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.317s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.333s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.518s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.300s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.290s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.317s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.303s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.310s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.338s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.282s
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
    ✓ Generated embedding: 1536 dimensions in 0.265s
    • Function auto-cleaned up by client

→ Executing HybridSearch()...
  • Collection: rag_messages_go
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.050s

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

Memory-safe, high-performance database code is mostly about **bounded resource use, clear ownership, and efficient query shapes**. The exact implementation depends on your language and database, but these practices apply broadly:

### Bound memory use
- **Stream or page through large result sets** instead of loading everything into a list at once.
- Fetch only the fields you need; avoid large blobs and unnecessary copies.
- Put explicit limits on queries, request sizes, batch sizes, and concurrency.
- Reuse buffers where appropriate, but don’t keep them alive longer than needed.
- Close cursors, statements, connections, and transactions reliably—using RAII, `defer`, `using`, or `finally`, depending on the language.

For example, prefer processing one page at a time:

```text
cursor = open_query(page_size = 500)
try:
    while page = cursor.next_page():
        process(page)
finally:
    cursor.close()
```

### Make ownership and concurrency explicit
- In memory-safe languages, use the language’s ownership, borrowing, or garbage-collection model rather than unsafe pointers or manual lifetime tricks.
- Avoid retaining references to request-scoped data after the request ends.
- Don’t share mutable database objects across threads unless the driver explicitly supports it. Use a connection pool and follow the driver’s concurrency rules.
- Bound concurrent queries: too many individually reasonable queries can exhaust memory, connections, or the database.

### Keep queries efficient
- Avoid N+1 query patterns; fetch related data in batches where possible.
- Use indexes that match your actual filters and sort order, and verify query plans rather than guessing.
- Paginate with a stable ordering. For deep pagination, consider a cursor/keyset approach instead of increasingly large offsets.
- Use parameterized queries when using SQL. They help prevent injection and let drivers/database engines handle values safely.
- Select only needed fields and avoid repeatedly fetching the same data.

### Keep writes and transactions controlled
- Batch writes when it reduces overhead, but keep batches small enough to avoid large in-memory buffers and long-running transactions.
- Keep transactions short; don’t hold one open while doing network calls or slow application work.
- Make retries safe: use idempotency keys or otherwise prevent a retry from duplicating a write.

### Measure before optimizing
Track query latency, memory use, allocation rates, pool saturation, and error rates under realistic load. Use query-plan tools and a profiler to locate the actual bottleneck; optimization without measurement often just moves the problem.

If you’re using ekoDB, the same ideas mean using `limit` and `select_fields` to keep reads bounded, checking `query_explain` for scans and index recommendations, and using `collection_info` to inspect indexes and data volume.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.309s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2894 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.346s
    • Function auto-cleaned up by client
✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...


→ Executing TextSearch()...
  • Collection: rag_messages_go
  • Query: "ownership system"
  • Limit: 3 results
  • Search method: Full-text with fuzzy matching & stemming
  • No vector embeddings needed - pure keyword search
  ✓ Text search completed in 0.042s

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

=== Cleanup ===
✓ Cleanup complete


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Running Kotlin RAG Example...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
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
    ✓ Generated embedding: 1536 dimensions in 0.356s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.305s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.347s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.27s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.308s
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
    ✓ Generated embedding: 1536 dimensions in 0.312s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.424s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.31s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.332s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.287s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.316s
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
    ✓ Generated embedding: 1536 dimensions in 0.404s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages_kt
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.07s

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
    ✓ Generated embedding: 1536 dimensions in 0.276s
    • Function auto-cleaned up by client
✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...


→ Executing textSearch()...
  • Collection: rag_messages_kt
  • Query: "ownership system"
  • Limit: 3 results
  • Search method: Full-text with fuzzy matching & stemming
  • No vector embeddings needed - pure keyword search
  ✓ Text search completed in 0.034s

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

=== Cleanup ===
✓ Cleanup complete


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ RAG Examples Complete!
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

What you just saw across 5 languages:
  ✓ Embeddings generated via ekoDB Functions
  ✓ Hybrid search (semantic + keyword)
  ✓ Text search with stemming
  ✓ Cross-conversation context retrieval
  ✓ Simple client helpers wrapping powerful AI

Mission: AI for All 🚀 - Making RAG accessible to everyone!

✅ All RAG examples complete! Output saved to test-examples-rag.md

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🌐 Testing SWR (Stale-While-Revalidate) Pattern Examples
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📦 Building TypeScript client library...

> @ekodb/ekodb-client@0.31.0 prepare
> npm run build


> @ekodb/ekodb-client@0.31.0 build
> tsc


up to date, audited 44 packages in 569ms

12 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
npm warn install-scripts 1 package has install scripts not yet covered by allowScripts:
npm warn install-scripts   fsevents@2.3.3 (install: (install scripts present))
npm warn install-scripts
npm warn install-scripts Run `npm install-scripts ls` to review, or `npm install-scripts approve <pkg>` to allow.

> @ekodb/ekodb-client@0.31.0 build
> tsc

✅ TypeScript client built!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Running TypeScript SWR Examples...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_ts_96007_1793595343397 (dgexS1D0alV3urNwCwXRwNST7Xw9XvhmEi-E-ly4eqSpAL16W_x2Mapk6LjHK2s2qWz54vQ05Tb4nc5W8VG7-A)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "name": "Leanne Graham",
          "username": "Bret",
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "bs": "harness real-time e-markets",
            "name": "Romaguera-Crona"
          },
          "phone": "1-770-736-8031 x56442",
          "address": {
            "city": "Gwenborough",
            "zipcode": "92998-3874",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "suite": "Apt. 556",
            "street": "Kulas Light"
          },
          "email": "Sincere@april.biz",
          "id": 1,
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
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 4ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "value": {
          "name": "Leanne Graham",
          "username": "Bret",
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "bs": "harness real-time e-markets",
            "name": "Romaguera-Crona"
          },
          "phone": "1-770-736-8031 x56442",
          "address": {
            "city": "Gwenborough",
            "zipcode": "92998-3874",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "suite": "Apt. 556",
            "street": "Kulas Light"
          },
          "email": "Sincere@april.biz",
          "id": 1,
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
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_reviews_ts_96007_1793595343397 (F7O18PHDX4ds-Le-79VZ2hFJ9SF0P7bI0Hoe0dS5LwJBnpHPqOelHgVBIYiEEfS8mpIA2e1wjqcidvfXjQfHfQ)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "value": {
          "stock": 99,
          "tags": [
            "beauty",
            "mascara"
          ],
          "discountPercentage": 10.48,
          "returnPolicy": "No return policy",
          "title": "Essence Mascara Lash Princess",
          "category": "beauty",
          "shippingInformation": "Ships in 3-5 business days",
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "warrantyInformation": "1 week warranty",
          "id": 1,
          "dimensions": {
            "height": 13.08,
            "width": 15.14,
            "depth": 22.99
          },
          "minimumOrderQuantity": 48,
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "price": 9.99,
          "rating": 2.56,
          "sku": "BEA-ESS-ESS-001",
          "meta": {
            "createdAt": "2025-10-09T14:47:01.588Z",
            "barcode": "5784719087687",
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
            "updatedAt": "2026-05-23T11:27:41.868Z"
          },
          "weight": 4,
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "reviews": [
            {
              "reviewerName": "Eleanor Collins",
              "rating": 3,
              "comment": "Would not recommend!",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "date": "2025-04-30T09:41:02.053Z"
            },
            {
              "rating": 4,
              "reviewerName": "Lucas Gordon",
              "reviewerEmail": "lucas.gordon@x.dummyjson.com",
              "comment": "Very satisfied!",
              "date": "2025-04-30T09:41:02.053Z"
            },
            {
              "reviewerName": "Eleanor Collins",
              "rating": 5,
              "comment": "Highly impressed!",
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com"
            }
          ],
          "brand": "Essence",
          "availabilityStatus": "In Stock"
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

=== ekoDB as Edge Cache - Simple Example ===

Creating edge cache function...
✓ Edge cache script created: YBzqit7HOv-ONAS9y5ek4ChT9hDH0OOhvWx9IN71Kavno0sAoV1j33yo-AB4qj_xNq5BnmjH5GsQI0DkvcxPxw

Call 1: Cache miss (fetches from API)
Response time: 61ms
Result: {
  "records": [
    {
      "value": {
        "value": {
          "address": {
            "suite": "Apt. 556",
            "zipcode": "92998-3874",
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "street": "Kulas Light"
          },
          "username": "Bret",
          "name": "Leanne Graham",
          "id": 1,
          "website": "hildegard.org",
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets"
          },
          "email": "Sincere@april.biz",
          "phone": "1-770-736-8031 x56442"
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
Response time: 3ms (20.3x faster!)
Result: {
  "records": [
    {
      "value": {
        "value": {
          "address": {
            "suite": "Apt. 556",
            "zipcode": "92998-3874",
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "street": "Kulas Light"
          },
          "username": "Bret",
          "name": "Leanne Graham",
          "id": 1,
          "website": "hildegard.org",
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets"
          },
          "email": "Sincere@april.biz",
          "phone": "1-770-736-8031 x56442"
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

✅ TypeScript SWR examples complete!
🐍 Building Python client package...
🔧 Ensuring maturin is available in .venv...
🔨 Building wheel...
🍹 Building a mixed python/rust project
🐍 Found CPython 3.15 at /Users/tek/.local/bin/python3
🔗 Found pyo3 bindings with abi3-py3.8 support
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.15s
📦 Built wheel for abi3 Python ≥ 3.8 to ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.31.0-cp38-abi3-macosx_11_0_arm64.whl
📦 Installing Python wheel into .venv...
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.31.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.31.0
    Uninstalling ekodb_client-0.31.0:
      Successfully uninstalled ekodb_client-0.31.0
Successfully installed ekodb-client-0.31.0
🧪 Ensuring test dependencies (pytest) in .venv...
✅ Python client package built and installed!
📦 Ensuring Python example dependencies in .venv...

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Running Python SWR Examples...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user_swr_py (bRyXY_h9mYQOR76X54XY-3Kc3L6W5uTBuXfdW9OODc-7HIvzHcSDsqzWwG8PB_CjxB5LX8yGWquKUYjjHDbQqA)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 185ms
Result: [
  {
    "cached_at": {
      "type": "String",
      "value": "1793595347"
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
        "followers": 330103,
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
        "updated_at": "2026-11-01T20:12:14Z",
        "url": "https://api.github.com/users/torvalds",
        "user_view_type": "public"
      }
    },
    "id": "torvalds"
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 6ms (28.6x faster!)
Result: [
  {
    "cached_at": {
      "type": "String",
      "value": "1793595347"
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
        "followers": 330103,
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
        "updated_at": "2026-11-01T20:12:14Z",
        "url": "https://api.github.com/users/torvalds",
        "user_view_type": "public"
      }
    }
  }
]
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched_swr_py (3f8r2v98izk-8UPqc-49AEOd2Qy5q7rPq_uZEHLPu6FIBLFSjeMmNb_0wp5M7W1U3ufTElj7WCJ2Xx0v9hQjrA)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "enriched_at": {
      "type": "String",
      "value": "1793595347"
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
          "createdAt": "2025-10-09T14:47:01.588Z",
          "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
          "updatedAt": "2026-05-23T11:27:41.868Z"
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

✅ Python SWR examples complete!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Running Go SWR Examples...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (BNN2xokJkqiZjUlBZXZII6MDg77GEYtJ9p_ZVGdaJJzGGytM-WnExxrHz-WSZXrVz1cqU1BPJdON1cfPzgaWbg)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 82.030458ms
Result: [
  {
    "cached_at": {
      "type": "DateTime",
      "value": "2026-11-02T04:55:48+00:00"
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
        "followers": 330103,
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
        "updated_at": "2026-11-01T20:12:14Z",
        "url": "https://api.github.com/users/torvalds",
        "user_view_type": "public"
      }
    },
    "id": "torvalds"
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 5.572167ms (16.4x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (x5kRoOdp6o5Gze4j8J3CZRV-4Ai_7YqVnO8SeFZ8HNi4Dgx3JPB5wX5BCuNtcMmhV9G5vRokD7H3C0hnKBVOVA)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "enriched_at": {
      "type": "DateTime",
      "value": "2026-11-02T04:55:48+00:00"
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
          "createdAt": "2025-10-09T14:47:01.588Z",
          "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
          "updatedAt": "2026-05-23T11:27:41.868Z"
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

✅ Go SWR examples complete!
🛠️  Building client library...
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
✅ Client build complete!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Running Rust SWR Examples...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.21s
     Running `target/debug/examples/swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (bnzen1NW6kni0DxO_WFP5v6bY99yJ-0Lnm3C27iW2_N625lSyi1DK_REsMazh6LwgVRv-05i4bHB76EZGnrWoA)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 182ms
Result: {
  "data": {
    "type": "Object",
    "value": {
      "location": "Portland, OR",
      "hireable": null,
      "followers": 330103,
      "company": "Linux Foundation",
      "blog": "",
      "created_at": "2011-09-03T15:26:22Z",
      "followers_url": "https://api.github.com/users/torvalds/followers",
      "following": 0,
      "repos_url": "https://api.github.com/users/torvalds/repos",
      "site_admin": false,
      "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
      "id": 1024025,
      "login": "torvalds",
      "received_events_url": "https://api.github.com/users/torvalds/received_events",
      "twitter_username": null,
      "organizations_url": "https://api.github.com/users/torvalds/orgs",
      "public_repos": 12,
      "bio": null,
      "updated_at": "2026-11-01T20:12:14Z",
      "html_url": "https://github.com/torvalds",
      "gravatar_id": "",
      "public_gists": 1,
      "user_view_type": "public",
      "type": "User",
      "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
      "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
      "name": "Linus Torvalds",
      "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
      "email": null,
      "events_url": "https://api.github.com/users/torvalds/events{/privacy}",
      "node_id": "MDQ6VXNlcjEwMjQwMjU=",
      "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
      "url": "https://api.github.com/users/torvalds"
    }
  }
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 7ms (26.0x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (7XNf7XKKZIr9sIFn8H8E4X3B3YBZP5YvVbU35Fck_tXHZv6ljzLZNbVerYFstkaOX9kW8EcEDSVW-13bx9G0jg)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: {
  "enriched_data": {
    "type": "Object",
    "value": {
      "warrantyInformation": "1 week warranty",
      "sku": "BEA-ESS-ESS-001",
      "stock": 99,
      "weight": 4,
      "id": 1,
      "brand": "Essence",
      "meta": {
        "updatedAt": "2026-05-23T11:27:41.868Z",
        "barcode": "5784719087687",
        "createdAt": "2025-10-09T14:47:01.588Z",
        "qrCode": "https://cdn.dummyjson.com/public/qr-code.png"
      },
      "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
      "discountPercentage": 10.48,
      "minimumOrderQuantity": 48,
      "category": "beauty",
      "images": [
        "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
      ],
      "rating": 2.56,
      "availabilityStatus": "In Stock",
      "shippingInformation": "Ships in 3-5 business days",
      "tags": [
        "beauty",
        "mascara"
      ],
      "reviews": [
        {
          "comment": "Would not recommend!",
          "reviewerName": "Eleanor Collins",
          "date": "2025-04-30T09:41:02.053Z",
          "rating": 3,
          "reviewerEmail": "eleanor.collins@x.dummyjson.com"
        },
        {
          "comment": "Very satisfied!",
          "rating": 4,
          "date": "2025-04-30T09:41:02.053Z",
          "reviewerName": "Lucas Gordon",
          "reviewerEmail": "lucas.gordon@x.dummyjson.com"
        },
        {
          "comment": "Highly impressed!",
          "rating": 5,
          "reviewerName": "Eleanor Collins",
          "date": "2025-04-30T09:41:02.053Z",
          "reviewerEmail": "eleanor.collins@x.dummyjson.com"
        }
      ],
      "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
      "title": "Essence Mascara Lash Princess",
      "price": 9.99,
      "dimensions": {
        "width": 15.14,
        "height": 13.08,
        "depth": 22.99
      },
      "returnPolicy": "No return policy"
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

✅ Rust SWR examples complete!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Running Kotlin SWR Examples...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR function: swr_fetch_github_user_kt_1793595398858 (wrWoel9mRLDc75WDJz5i-LJF4HkPucMOKGFEZomGZEsYIMZ8x86Aq-TWvs26b4ZJbVyvRUX46dKwCZd51orfTg)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 101ms
Result: [{"data":{"type":"Object","value":{"bio":null,"repos_url":"https://api.github.com/users/torvalds/repos","following":0,"following_url":"https://api.github.com/users/torvalds/following{/other_user}","gists_url":"https://api.github.com/users/torvalds/gists{/gist_id}","starred_url":"https://api.github.com/users/torvalds/starred{/owner}{/repo}","company":"Linux Foundation","followers":330103,"login":"torvalds","site_admin":false,"twitter_username":null,"user_view_type":"public","public_gists":1,"email":null,"id":1024025,"blog":"","location":"Portland, OR","node_id":"MDQ6VXNlcjEwMjQwMjU=","updated_at":"2026-11-01T20:12:14Z","public_repos":12,"html_url":"https://github.com/torvalds","name":"Linus Torvalds","organizations_url":"https://api.github.com/users/torvalds/orgs","received_events_url":"https://api.github.com/users/torvalds/received_events","gravatar_id":"","subscriptions_url":"https://api.github.com/users/torvalds/subscriptions","events_url":"https://api.github.com/users/torvalds/events{/privacy}","avatar_url":"https://avatars.githubusercontent.com/u/1024025?v=4","followers_url":"https://api.github.com/users/torvalds/followers","hireable":null,"type":"User","url":"https://api.github.com/users/torvalds","created_at":"2011-09-03T15:26:22Z"}},"id":"torvalds","cached_at":{"value":"1793595398867","type":"String"}}]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 7ms
Cache hit was 14.4x faster!

✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment function: swr_fetch_product_kt_1793595398858 (izuFjhBCVs0YRUNYu4e4bM--yun2surSYYi4AUgOWDPcrriM2n7GBRIneOGi4Vbkpei08GahyjPmLuSko3dfVQ)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [{"enriched_at":{"type":"String","value":"1793595399137"},"enriched_data":{"value":{"minimumOrderQuantity":48,"weight":4,"dimensions":{"height":13.08,"depth":22.99,"width":15.14},"reviews":[{"rating":3,"reviewerEmail":"eleanor.collins@x.dummyjson.com","date":"2025-04-30T09:41:02.053Z","reviewerName":"Eleanor Collins","comment":"Would not recommend!"},{"rating":4,"comment":"Very satisfied!","reviewerEmail":"lucas.gordon@x.dummyjson.com","date":"2025-04-30T09:41:02.053Z","reviewerName":"Lucas Gordon"},{"rating":5,"comment":"Highly impressed!","date":"2025-04-30T09:41:02.053Z","reviewerEmail":"eleanor.collins@x.dummyjson.com","reviewerName":"Eleanor Collins"}],"discountPercentage":10.48,"category":"beauty","brand":"Essence","meta":{"barcode":"5784719087687","qrCode":"https://cdn.dummyjson.com/public/qr-code.png","updatedAt":"2026-05-23T11:27:41.868Z","createdAt":"2025-10-09T14:47:01.588Z"},"shippingInformation":"Ships in 3-5 business days","stock":99,"sku":"BEA-ESS-ESS-001","tags":["beauty","mascara"],"availabilityStatus":"In Stock","title":"Essence Mascara Lash Princess","returnPolicy":"No return policy","description":"The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.","rating":2.56,"warrantyInformation":"1 week warranty","thumbnail":"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp","id":1,"price":9.99,"images":["https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"]},"type":"Object"},"id":"1"}]
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


BUILD SUCCESSFUL in 6s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
✅ Kotlin SWR examples complete!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ All SWR Examples Complete!
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

What you just saw - ekoDB as Edge Cache:
  ✓ FindById → Check cache
  ✓ If/Else → Conditional execution
  ✓ HttpRequest → External API calls
  ✓ Insert with TTL → Auto-expiring cache
  ✓ Sub-millisecond cache hits
  ✓ No Redis, no CDN, no cache invalidation logic needed

Your DATABASE is your EDGE! 🚀


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🔗 Testing Function Composition Examples
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

🛠️  Building client library...
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.30s
✅ Client build complete!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Running Rust Function Composition Examples...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
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
   ⏱️  Duration: 79.670917ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "phone": "1-770-736-8031 x56442",
      "id": 1,
      "email": "Sincere@april.biz",
      "company": {
        "name": "Romaguera-Crona",
        "bs": "harness re...

Second call (cache hit - from cache):
   ⏱️  Duration: 3.449958ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "address": {
        "city": "Gwenborough",
        "street": "Kulas Light",
        "suite": "Apt. 556",
        "zipcode": "92998-3874",
        "geo": {
        ...
   🚀 Cache speedup: 23.1x faster!

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
✅ Rust function composition examples complete!
📦 Building TypeScript client library...

> @ekodb/ekodb-client@0.31.0 prepare
> npm run build


> @ekodb/ekodb-client@0.31.0 build
> tsc


up to date, audited 44 packages in 844ms

12 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
npm warn install-scripts 1 package has install scripts not yet covered by allowScripts:
npm warn install-scripts   fsevents@2.3.3 (install: (install scripts present))
npm warn install-scripts
npm warn install-scripts Run `npm install-scripts ls` to review, or `npm install-scripts approve <pkg>` to allow.

> @ekodb/ekodb-client@0.31.0 build
> tsc

✅ TypeScript client built!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Running TypeScript Function Composition Examples...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
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
   ⏱️  Duration: 55ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "name": "Leanne Graham",
      "address": {
        "city": "Gwenborough",
        "geo": {
          "lat": "-37.3159",
          "lng": "81.1496"
        },
     ...

Second call (cache hit - from cache):
   ⏱️  Duration: 4ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "name": "Leanne Graham",
      "address": {
        "city": "Gwenborough",
        "geo": {
          "lat": "-37.3159",
          "lng": "81.1496"
        },
     ...
   🚀 Cache speedup: 13.8x faster!

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
✅ TypeScript function composition examples complete!
🐍 Building Python client package...
🔧 Ensuring maturin is available in .venv...
🔨 Building wheel...
🍹 Building a mixed python/rust project
🐍 Found CPython 3.15 at /Users/tek/.local/bin/python3
🔗 Found pyo3 bindings with abi3-py3.8 support
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.16s
📦 Built wheel for abi3 Python ≥ 3.8 to ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.31.0-cp38-abi3-macosx_11_0_arm64.whl
📦 Installing Python wheel into .venv...
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.31.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.31.0
    Uninstalling ekodb_client-0.31.0:
      Successfully uninstalled ekodb_client-0.31.0
Successfully installed ekodb-client-0.31.0
🧪 Ensuring test dependencies (pytest) in .venv...
✅ Python client package built and installed!
📦 Ensuring Python example dependencies in .venv...

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Running Python Function Composition Examples...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

✅ Saved reusable function: fetch_user_py
✅ Saved composed function: get_user_wrapper_py (calls fetch_user_py + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"type": "String", "value": "User 1"}
   Department: {"type": "String", "value": "engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user_py (uses KV)
✅ Saved SWR function using composition: swr_user_py

First call (cache miss - will fetch from API):
   ⏱️  Duration: 71.2ms
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
   🚀 Cache speedup: 28.1x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user_py
✅ Level 2 function: fetch_slim_user_py (calls validate_user_py)
✅ Level 3 function: get_verified_user_py (calls fetch_slim_user_py)

📊 Result from 3-level nested composition:
   Records: 1
   Name: User 1
   Department: engineering

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - get_verified_user: Specific workflow


✅ Cleanup complete
✅ All composition examples completed!
✅ Python function composition examples complete!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Running Go Function Composition Examples...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
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
   ⏱️  Duration: 63.095167ms
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
   ⏱️  Duration: 2.615ms
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
   🚀 Cache speedup: 31.5x faster!

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
✅ Go function composition examples complete!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Running JavaScript Function Composition Examples...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
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
   ⏱️  Duration: 68ms
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
   ⏱️  Duration: 2ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "company": {
        "name": "Romaguera-Crona",
        "bs": "harness real-time e-markets",
        "catchPhrase": "Multi-layered client-serv...
   🚀 Cache speedup: 34.0x faster!

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
✅ JavaScript function composition examples complete!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ All Function Composition Examples Complete!
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

What you just saw - CallFunction composability:
  ✓ Reusable Function building blocks
  ✓ Functions calling other Functions
  ✓ Clean SWR patterns via composition
  ✓ Multi-level nesting (arbitrary depth)
  ✓ No code duplication
  ✓ Single source of truth

Build complex workflows from simple pieces! 🚀


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📡 WebSocket Subscription Tests
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

These tests verify real-time WebSocket subscriptions by:
  1. Authenticating and connecting via WebSocket
  2. Subscribing to a collection
  3. Inserting records via REST to trigger notifications
  4. Verifying MutationNotification push messages arrive
  5. Unsubscribing and cleaning up

🛠️  Building client library...
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.33s
✅ Client build complete!

🦀 Rust WebSocket Subscription Test...
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/examples/client_websocket_subscribe`
✓ Authentication successful

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_rs' ===
✓ Subscribed (subscription_id: sub_cb21f03c9f1a4e10a9f221f445fc8023)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: xqoKw67OOwBL5KQIYUExjvKywzsI2i-1NEMpIa1x1MQ_LyL4Dtm7ofie3aC8ice1F-zb7OUX9UmU2nr2kyivFg

  📡 Notification received:
     Event:      "insert"
     Collection: "ws_subscribe_example_rs"
     Record IDs: ["xqoKw67OOwBL5KQIYUExjvKywzsI2i-1NEMpIa1x1MQ_LyL4Dtm7ofie3aC8ice1F-zb7OUX9UmU2nr2kyivFg"]
     Timestamp:  "2026-11-02T05:01:05.302160+00:00"

Inserting record 2...
✓ Inserted: G3d0kqs7KdYxzvTDTniD0_OKg3sHhTY82dImJ1-oZsfl1BwU4msO8QLPynCeluPQKiAl_DcnLZsfjgcnYJbHTw

  📡 Notification received:
     Event:      "insert"
     Record IDs: ["G3d0kqs7KdYxzvTDTniD0_OKg3sHhTY82dImJ1-oZsfl1BwU4msO8QLPynCeluPQKiAl_DcnLZsfjgcnYJbHTw"]

=== Unsubscribing ===
✓ Unsubscribed

=== Cleanup ===
✓ Deleted collection 'ws_subscribe_example_rs'

✓ WebSocket subscription example completed successfully
✅ Rust subscription test complete!

🔷 Go WebSocket Subscription Test...
=== WebSocket Subscription Example (Go) ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_go' ===
✓ Subscribed (subscription_id: sub_fd803fc251bf4de59c196d0636b20c91)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: Brxx6Wcp11HZ-GG8w0YCZIy02IqjYjvvCTMjkslJp6ja_HcHMmSatJdvms30gCuMi7NTwDVSGO_Gn76rWvBo7Q

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_go
     Record IDs: [Brxx6Wcp11HZ-GG8w0YCZIy02IqjYjvvCTMjkslJp6ja_HcHMmSatJdvms30gCuMi7NTwDVSGO_Gn76rWvBo7Q]
     Timestamp:  2026-11-02T05:01:05.584867+00:00

Inserting record 2...
✓ Inserted: MCoPqy2_aPw-KX8sWk5pdZzF1HpisKH6xqRgc-BlJrAdrpYV8P5GbeQGcfsydKU1AD4VcEqp556enwNytT41-w

  📡 Notification received:
     Event:      insert
     Record IDs: [MCoPqy2_aPw-KX8sWk5pdZzF1HpisKH6xqRgc-BlJrAdrpYV8P5GbeQGcfsydKU1AD4VcEqp556enwNytT41-w]

=== Unsubscribing ===
✓ Unsubscribed

✓ WebSocket subscription example completed successfully
✅ Go subscription test complete!
📦 Ensuring Python example dependencies in .venv...

🐍 Python WebSocket Subscription Test...
=== WebSocket Subscription Example (Python) ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_py' ===
✓ Subscribed (subscription_id: sub_26330dd180744edda72400105082bb0b)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: gMO8je6bpVpne3SJ6ztHebSZ3a9Q6v3xcvrs1fOzrwPGVDVyk8E1atRk0XNtq8Mgd6Fj_WoGFJ5X2sFdjkBTzg

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_py
     Record IDs: gMO8je6bpVpne3SJ6ztHebSZ3a9Q6v3xcvrs1fOzrwPGVDVyk8E1atRk0XNtq8Mgd6Fj_WoGFJ5X2sFdjkBTzg
     Timestamp:  2026-11-02T05:01:06.166544+00:00

Inserting record 2...
✓ Inserted: Ny9DPm_AmlRAArhX80EQcZs6gB0QUz4US3fDW-AlFRI7EvyWhJcraqo2UhTntFJQPqC_zdU3npR4UMTllQpxTg

  📡 Notification received:
     Event:      insert
     Record IDs: Ny9DPm_AmlRAArhX80EQcZs6gB0QUz4US3fDW-AlFRI7EvyWhJcraqo2UhTntFJQPqC_zdU3npR4UMTllQpxTg

=== Unsubscribing ===
✓ Unsubscribed: {'collection': 'ws_subscribe_example_py', 'found': True, 'unsubscribed': True}

=== Cleanup ===
✓ Deleted collection 'ws_subscribe_example_py'

✓ WebSocket subscription example completed successfully
✅ Python subscription test complete!
📦 Building TypeScript client library...

> @ekodb/ekodb-client@0.31.0 prepare
> npm run build


> @ekodb/ekodb-client@0.31.0 build
> tsc


up to date, audited 44 packages in 602ms

12 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
npm warn install-scripts 1 package has install scripts not yet covered by allowScripts:
npm warn install-scripts   fsevents@2.3.3 (install: (install scripts present))
npm warn install-scripts
npm warn install-scripts Run `npm install-scripts ls` to review, or `npm install-scripts approve <pkg>` to allow.

> @ekodb/ekodb-client@0.31.0 build
> tsc

✅ TypeScript client built!

📘 TypeScript WebSocket Subscription Test...
=== WebSocket Subscription Example ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_ts' ===
✓ Subscribed (subscription_id: sub_ff0a39d8765f497f9844f34211c9373f)

=== Performing mutations to trigger notifications ===
Inserting a record...
✓ Inserted record: 72RSgm_eIBUS_egxBuq91cSCHWKe-vDiR2lmAr1bwaqUxPoQGBl3PFXfNtjTOhsTXbzlxsKcbQ5x5g_CBBPm3g
  📡 Notification received for 72RSgm_eIBUS_egxBuq91cSCHWKe-vDiR2lmAr1bwaqUxPoQGBl3PFXfNtjTOhsTXbzlxsKcbQ5x5g_CBBPm3g

Inserting another record...
✓ Inserted record: Xsmxp0xehcObS-S87RFi34EgmwieWRhcYhiobxkVEwtakhcVO-nt8_30_FhmZ4v9Q_RZ_l8-F2qa0JQWiz2Sbw
  📡 Notification received for Xsmxp0xehcObS-S87RFi34EgmwieWRhcYhiobxkVEwtakhcVO-nt8_30_FhmZ4v9Q_RZ_l8-F2qa0JQWiz2Sbw

=== Unsubscribing ===
✓ Unsubscribed: {"collection":"ws_subscribe_example_ts","found":true,"unsubscribed":true}

✓ WebSocket subscription example completed successfully
✓ Deleted collection 'ws_subscribe_example_ts'
✅ TypeScript subscription test complete!

🟣 Kotlin WebSocket Subscription Test...
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
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
✓ Subscribed (subscription_id: sub_e93a996b0c854ed4a24143ad75c6b0b2)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: "LE3vfLkPdRJo1v9MU3BbqgdMsFi6g9Un-FEB7_WuwmBagMnp-tY0_ldHZgKU4IwSkk9UQxiFfVmQWVbLaPBybA"

  📡 Notification received:
     Event:      "insert"
     Collection: "ws_subscribe_example_kt"
     Record IDs: ["LE3vfLkPdRJo1v9MU3BbqgdMsFi6g9Un-FEB7_WuwmBagMnp-tY0_ldHZgKU4IwSkk9UQxiFfVmQWVbLaPBybA"]
     Timestamp:  "2026-11-02T05:01:12.660660+00:00"

Inserting record 2...
✓ Inserted: "hpInNGIs1yS4y2BSmlCTOTHwxh-Lo8SFksUFsWyQusrJiIngk4y1fXpkLQ-eeaXURq-ns3evsCJIHt4jOQy4yQ"

  📡 Notification received:
     Event:      "insert"
     Record IDs: ["hpInNGIs1yS4y2BSmlCTOTHwxh-Lo8SFksUFsWyQusrJiIngk4y1fXpkLQ-eeaXURq-ns3evsCJIHt4jOQy4yQ"]

=== Unsubscribing ===
✓ Unsubscribed: {"payload":{"data":{"collection":"ws_subscribe_example_kt","found":true,"unsubscribed":true}},"type":"Success"}

✓ WebSocket subscription example completed successfully
✓ Deleted collection 'ws_subscribe_example_kt'

BUILD SUCCESSFUL in 5s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
✅ Kotlin subscription test complete!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ All WebSocket Subscription Tests Passed!
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
