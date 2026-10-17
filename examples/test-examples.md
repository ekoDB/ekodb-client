make test-examples
🧪 Running Rust examples (direct HTTP/WebSocket)...
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("jWdS5Hu3hiDEzsHCfeKRTHOj3EM3D-R2PSCSYxjz8DjafnyflLvdriqL-azFJXhJ22ZFtXe2zrEbAptSKy-z4A")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("jWdS5Hu3hiDEzsHCfeKRTHOj3EM3D-R2PSCSYxjz8DjafnyflLvdriqL-azFJXhJ22ZFtXe2zrEbAptSKy-z4A"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("jWdS5Hu3hiDEzsHCfeKRTHOj3EM3D-R2PSCSYxjz8DjafnyflLvdriqL-azFJXhJ22ZFtXe2zrEbAptSKy-z4A"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("jWdS5Hu3hiDEzsHCfeKRTHOj3EM3D-R2PSCSYxjz8DjafnyflLvdriqL-azFJXhJ22ZFtXe2zrEbAptSKy-z4A"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: p4coXcYPfeShxvjr7DbHAqzmz7opn8eQz3lFbpUml435E8ro84aTQPnHfY17J6rm1al5vg_VE0O9pJvhLWPh5Q

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1792222977096354000",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "p4coXcYPfeShxvjr7DbHAqzmz7opn8eQz3lFbpUml435E8ro84aTQPnHfY17J6rm1al5vg_VE0O9pJvhLWPh5Q",
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/collection_management`
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: "qyIGCHvxbue6wRf5OFSuYFmZ8vr9j990pIxcKPjwvCKYLoF-LGX3eJ2Od-bLP0yze6b2h6t49GAg22oW9rbziw"

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
✓ Inserted document: 8PPQirPcRp9w5MVJA88Fqc-Vjj4-sQBsKgQfGK2JVnc6J5eDpne0Gun7fk9BihBpMhUlCLYVa5QBnZRFj___Xw

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: CFfzjDpyvR3R9NKkk9WURBkAt8_b0HwyK9jPKoD29S140ZColGNo-jDjgpky0HSbsI9OtaRpFIACf1UKNwXJog

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: 8aMB-e41ZsndFYitCrSvvW9UOzCe2LCTVJzHkscHtcdRGsYYJstzGujY8BaxLt15n-tEfeedTtUfX-CZ5pr8TQ

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
✓ Inserted document with TTL: String("RsTDFEEO6NJRPBsyOdh0_1x6szY2pgNcF5oXFrT7rwJ_JE4oTne6SU8e1FiyARuxj7BqRsiQX4eRzw72VGfOeQ")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record via WebSocket

Record 1:
  id: "RsTDFEEO6NJRPBsyOdh0_1x6szY2pgNcF5oXFrT7rwJ_JE4oTne6SU8e1FiyARuxj7BqRsiQX4eRzw72VGfOeQ"
  name: {"type":"String","value":"WebSocket TTL Test"}
  ttl: "2026-10-17T08:46:53.781982Z"
  value: {"type":"Integer","value":42}

✓ WebSocket TTL example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Functions Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: baZeKdGYvXiMyLNn424F1FjdLNp1OQUoHMdE3gTc4xVS95rpJPIaS3L0MIzFxt3Su_XnKKwM83EwufWc4zTc1A
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: aPmb-ZxhXMWyjll-ZpNsi12wC8l3X73ZG55lTGkroAeM8t39Z4p7GvwlHfT7RP9sdh9jCxJcJu7etElJIbkbRw
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Function saved: 7HK_-k6mKHF_EX15cS6EHYfIH6extPF4X7P2FqjLEmwVAhHKN3OdE_OCzj3_7WfywnhULa83-xnXraphO70c_g
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: Gxdp9pils8gAoKZ0lRoBIcJ8q9Aw8eo6rlH-MGNRtls-g6Rwfye9rreKoR-WG-Ls7-C72NMdBIxTTNJu7NgeVw
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/transactions`
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: RXT38DhorCKNhpbb3GIfwI77oxIl1sCjQOhRaf_ldZnxNR94sflPYHv6IKYSM2qVhsNrPWzjpWUl_NhzcLoXFg
Created Bob: $500 - ID: hZ6MS39_23z58sp4WnqyYsFM0dK1_vxg4jsi-1WWwuMLsDfGKbjkaAt-wii_PkcH70siXc-jDM1cwvwYpUofEw

=== Example 1: Begin Transaction ===
Transaction ID: 7df6e4e0-6572-4273-8f60-1f8815f6956d

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
New transaction: cdc9ec32-a5fa-4ea0-9cb8-ce6c01f9cf2a
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"type":"Integer","value":700}

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✅ Rust direct examples complete!
🛠️  Building client library...
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.36s
✅ Client build complete!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_advanced_crud`
=== ekoDB Advanced CRUD Example (Rust) ===

--- Inserting base document ---
Inserted: Widget Counter (id: OZeZav7FnxSN78RHof1koIG9m-7gshfEm5p8MM0Qj9MtMbffhk2PUxggl75Xd3Zzb0Uq35_6MTkoX1oFjN967g)

--- update_with_action: increment views by 10 ---
views after increment: Some(Object({"type": String("Integer"), "value": Integer(110)}))

--- update_with_action: decrement views by 3 ---
views after decrement: Some(Object({"type": String("Integer"), "value": Integer(107)}))

--- update_with_action: multiply score by 2 ---
score after multiply: Some(Object({"type": String("Float"), "value": Float(9.0)}))

--- update_with_action: push 'ekodb' to tags ---
tags after push: Some(Object({"type": String("Array"), "value": Array([String("rust"), String("database"), String("ekodb")])}))

--- update_with_action: append '-suffix' to label ---
label after append: Some(Object({"value": String("prefix-suffix"), "type": String("String")}))

--- update_with_action: pop last element from tags ---
tags after pop: Some(Object({"type": String("Array"), "value": Array([String("rust"), String("database")])}))

--- update_with_action: remove 'rust' from tags ---
tags after remove: Some(Object({"type": String("Array"), "value": Array([String("database")])}))

--- update_with_action_sequence: increment views + push tag + append label ---
views after sequence: Some(Object({"value": Integer(157), "type": String("Integer")}))
tags after sequence: Some(Object({"value": Array([String("database"), String("batch")]), "type": String("Array")}))
label after sequence: Some(Object({"value": String("prefix-suffix-v2"), "type": String("String")}))

--- Cleanup ---
Deleted collection

=== All advanced CRUD operations completed ===
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: iEhPRlIZ04UABikMSwMcdSMWYG4wimBJegJoSRhaqWuUHiawTj6TyBtLc3idTZH4NFzobVyaXLwSBhRA5cbNsw

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you need more information or have other inquiries, feel free to ask!

✓ Second message sent
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you need any further information or assistance, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: e8EXQSaSeSAIoKSghHsb-1CuQxarTlYcRYyehO1WlGHIa3eHfNwT8_fzDBbHzsfEkS8wDbUhDDBgidrmSeOAmg
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: e8EXQSaSeSAIoKSghHsb-1CuQxarTlYcRYyehO1WlGHIa3eHfNwT8_fzDBbHzsfEkS8wDbUhDDBgidrmSeOAmg
✓ Deleted chat session: iEhPRlIZ04UABikMSwMcdSMWYG4wimBJegJoSRhaqWuUHiawTj6TyBtLc3idTZH4NFzobVyaXLwSBhRA5cbNsw
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_chat_basic`
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: PjW67vbMvyuXcO9K37ejJV2010OGwWTa_dB6k5IyVyOk3WYAtSq61tPmLYiDprMKvtpr5V36JK0ZIBq5Xmjsjw

=== Sending Chat Message ===
Message ID: fv5GKa1nnVIpr2kTd8YAgwgI4CPRuXFjmb2Y7HboWQjB0rk4vx5FSBDztZ2qrlTmhEcqO51fNaTW6O6Oo6yQsg

=== AI Response ===
Response 1: **ekoDB Overview:**
ekoDB is described as a high-performance database that integrates intelligent caching, real-time capabilities, and AI technologies. This makes it well-suited for applications that require fast data retrieval and AI-enhanced processing.

**Key Features of ekoDB:**
1. **AI Chat Integration:** ekoDB includes a chat feature that allows users to query the database using natural language. This feature provides AI-powered responses along with relevant contextual information.

2. **Search Capabilities:**
   - Supports **full-text search**, allowing for efficient text-based queries.
   - Implements **vector search**, enhancing search capabilities through embeddings.
   - Offers **hybrid search**, which combines both keyword and semantic search methods, with automatic context retrieval.

These features highlight ekoDB's strengths in performance, search functionalities, and integration capabilities with AI.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["category", "title", "content"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("ZH86RHcUWzD62JEKWzZTLCmWBmszopTwVBvAc2G0qCYjRPG7x8-RTuJAzeHJvAq-O62LtiQC8b9emG7YGSMSsw"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "content"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("_pM4VbjOve6zxElhmLjjgv535p16e6oZ-lO8T1iINUl9U3cO1YKV4SlqvFVAeH2G_aaxfkHbuuN4D-Q-H1Z6yQ"), "title": String("Introduction to ekoDB")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "category", "content"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("kB6ONCJFogNX7cVezUJe5tMfdxZpg0Jx_rqOMs6g1pR7IHRVqf4Yc8AlF_jZuW3GlW0HZNIHFJCWahY72uCLuA"), "title": String("Search Features")}

Execution Time: 3974ms

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_chat_message_stream`
=== ekoDB Chat Message Stream (SSE) Example (Rust) ===

Created session: iwQ7soC5JvymqBnVwFZtUDPvbKG--OTne0IbaxotVzFJk9XWaMbVB_PgieoGFE8PNMyZhTkrpRX6sCMHzAhoGg

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}**ekoDB** is a comprehensive, open-source database specifically designed for sustainable construction materials. Its primary goal is to aid architects, engineers, builders, and researchers in making informed decisions to reduce the environmental impact of buildings and infrastructure.

### Key Features of ekoDB:

1. **Material Data Repository**:
   ekoDB compiles detailed information about various construction materials, including physical properties, durability, embodied carbon, recyclability, origin, certifications, and other life-cycle analysis data.

2. **Sustainability Metrics**:
   It emphasizes the environmental aspects of materials, such as greenhouse gas emissions (CO2 footprint), energy use, toxicity, and water consumption associated with production and use.

3. **Open-Source and Collaborative**:
   ekoDB is an open-source platform, which means its data and code are freely accessible. This encourages collaboration among professionals and allows users to contribute data, case studies, and improvements.

4. **Decision Support Tools**:
   The database often includes tools for comparing materials, calculating environmental impacts, and generating reports for sustainability certification (e.g., LEED, BREEAM).

5. **Global Scope**:
   While it started with a focus on local or regional materials, ekoDB aims to include data from international sources, making it applicable to a wide range of projects worldwide.

---

**In summary:**
ekoDB is an open, collaborative digital resource supporting sustainable architecture and construction through transparent, accessible information on eco-friendly building materials.

If you meant a different "ekoDB" (such as in another industry), please provide more context!

--- Stream complete ---
Message ID: jcLSC8-PX7XwfzpAOyiF6wGYKWidr_68HcRP9LZhqe-5-jXVqNZnj_JiVaLxD9Iief-cRt4agRCArcHsEIutMQ
Execution time: 2421ms
Context window: 1000000 tokens

✓ Chat message stream example completed
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_chat_models`
✓ Client created

=== Get All Chat Models ===
Available chat models by provider:

OpenAI models (135):
  - gpt-3.5-turbo-0125
  - gpt-5.4-2026-03-05
  - gpt-5-nano
  - gpt-5.4-mini
  - o1-pro
  - gpt-5.1-codex
  - tts-1
  - gpt-5-mini
  - gpt-5-mini-2025-08-07
  - gpt-4-turbo-2024-04-09
  - gpt-realtime-2.1-mini
  - gpt-realtime-2025-08-28
  - gpt-4
  - tts-1-hd
  - gpt-realtime-1.5
  - gpt-realtime
  - gpt-5.2-pro
  - gpt-5-chat-latest
  - gpt-5.5-pro
  - gpt-audio-2025-08-28
  - gpt-4o-mini-transcribe-2025-03-20
  - omni-moderation-2024-09-26
  - gpt-live-transcribe
  - gpt-5.6-luna
  - omni-moderation-latest
  - o4-mini-2025-04-16
  - gpt-4o-2024-11-20
  - gpt-5.1
  - whisper-1
  - gpt-5-pro
  - gpt-realtime-whisper
  - gpt-5-2025-08-07
  - gpt-5.4-mini-2026-03-17
  - babbage-002
  - gpt-4o-2024-05-13
  - text-embedding-3-large
  - chat-latest
  - gpt-realtime-translate
  - gpt-5.4-pro-2026-03-05
  - gpt-realtime-2
  - gpt-5-search-api
  - gpt-5.5-2026-04-23
  - tts-1-1106
  - gpt-5.5-pro-2026-04-23
  - gpt-audio-mini-2025-10-06
  - gpt-5.2-pro-2025-12-11
  - o3-mini-2025-01-31
  - gpt-audio-1.5
  - gpt-image-1.5
  - gpt-4-turbo
  - gpt-4o-2024-08-06
  - o3
  - gpt-4o-mini-search-preview-2025-03-11
  - gpt-4o-mini-transcribe-2025-12-15
  - gpt-4.1-mini
  - gpt-4o-mini
  - gpt-4.1-nano
  - gpt-image-2
  - gpt-3.5-turbo-16k
  - text-embedding-ada-002
  - gpt-realtime-2.1
  - gpt-4.1
  - gpt-5.5
  - gpt-4o-transcribe-diarize
  - gpt-5-codex
  - gpt-4-0613
  - gpt-5.1-2025-11-13
  - gpt-4o-search-preview
  - o1-2024-12-17
  - gpt-5-pro-2025-10-06
  - gpt-5.6-sol
  - gpt-4o-mini-tts-2025-12-15
  - gpt-image-1
  - gpt-audio
  - gpt-5.3-codex
  - gpt-5-search-api-2025-10-14
  - gpt-realtime-mini
  - gpt-image-1-mini
  - gpt-5.6-terra
  - gpt-3.5-turbo-instruct-0914
  - sora-2
  - gpt-5.4-pro
  - gpt-4o-mini-tts
  - gpt-realtime-mini-2025-12-15
  - gpt-5.1-codex-max
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
  - gpt-5.1-chat-latest
  - tts-1-hd-1106
  - gpt-5.2-codex
  - gpt-4o
  - o4-mini-deep-research
  - gpt-5.4
  - gpt-4o-search-preview-2025-03-11
  - text-embedding-3-small
  - gpt-5-nano-2025-08-07
  - gpt-4o-transcribe
  - o4-mini
  - gpt-5.1-codex-mini
  - gpt-image-2.5-sunburst-2026-09-08
  - gpt-5.3-chat-latest
  - o3-2025-04-16
  - gpt-5.2-2025-12-11
  - chatgpt-image-latest
  - gpt-3.5-turbo
  - gpt-3.5-turbo-1106
  - gpt-4o-mini-tts-2025-03-20
  - gpt-5.2
  - gpt-5.4-nano-2026-03-17
  - o1
  - davinci-002
  - gpt-5.4-nano
  - o3-mini
  - o4-mini-deep-research-2025-06-26
  - gpt-audio-mini-2025-12-15
  - o1-pro-2025-03-19
  - gpt-6-luna
  - gpt-image-2.5-flare
  - gpt-6-astra
  - gpt-audio-mini
  - gpt-4.1-mini-2025-04-14
  - gpt-image-2.5-sunburst
  - gpt-image-2.5-flare-2026-09-08
  - gpt-4o-mini-search-preview
  - gpt-6-sol
  - gpt-live-1
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

Gemini models (63):
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
  - veo-3.1-generate-preview
  - veo-3.1-fast-generate-preview
  - veo-3.1-lite-generate-preview
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
  gemini: ok 63 models
  openai: ok 135 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get Models for Specific Provider ===

openai models (135):
  - gpt-3.5-turbo-0125
  - gpt-5.4-2026-03-05
  - gpt-5-nano
  - gpt-5.4-mini
  - o1-pro
  ... and 130 more

anthropic models (14):
  - claude-haiku-5-5
  - claude-sonnet-5-5
  - claude-opus-5-5
  - claude-fable-5-1
  - claude-opus-5
  ... and 9 more
GetChatModel(perplexity) error: Record not found

✓ Chat Models API example complete
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: kUeyKtql8cvUcbcZTIutYj71u7o55jqP1_DXM-_oBFi3TfMKYLB_61Ljj9yvVxwBoY8h6rr3VxaZzlnsGoqQag
=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Product:** ekoDB
- **Description:** A high-performance database product with AI capabilities
- **Price:** $99

If you have any further questions or need more information, feel free to ask!

✓ Message 2 sent
  Response: The price of the product ekoDB is **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: pnTiARv9MdqRt5dN67OH68_lpkCkJ2AO-RQ4dRN9-iVlCc_-ejmHEDT0vwbVuUcG58C-aft6GQNlXDSmkPArvQ
  Parent: kUeyKtql8cvUcbcZTIutYj71u7o55jqP1_DXM-_oBFi3TfMKYLB_61Ljj9yvVxwBoY8h6rr3VxaZzlnsGoqQag

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: pnTiARv9MdqRt5dN67OH68_lpkCkJ2AO-RQ4dRN9-iVlCc_-ejmHEDT0vwbVuUcG58C-aft6GQNlXDSmkPArvQ (Untitled)
  Session 2: kUeyKtql8cvUcbcZTIutYj71u7o55jqP1_DXM-_oBFi3TfMKYLB_61Ljj9yvVxwBoY8h6rr3VxaZzlnsGoqQag (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: pnTiARv9MdqRt5dN67OH68_lpkCkJ2AO-RQ4dRN9-iVlCc_-ejmHEDT0vwbVuUcG58C-aft6GQNlXDSmkPArvQ

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_collection_management`
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "mmRa0F3z-bkjUXQryy-jl2p67eglA10tGss8REWJQCql4vytITNYA5T6EPARxuJEVGY8WrZ9QHRgTddG9Jn_wQ"

=== List Collections ===
Total collections: 7
Sample collections: ["chat_messages__ek0_testing", "client_collection_management_rust", "audit__ek0_testing", "functions__ek0_testing", "chat_configurations__ek0_testing"]

=== Count Documents ===
Document count: 1

=== Check Collection Exists ===
Collection exists: true

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
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
  - chat_messages__ek0_testing
  - audit__ek0_testing
  - functions__ek0_testing
  - chat_configurations__ek0_testing
  - chat_turns__ek0_testing
  - collection_utils_test_rust <-- our test
  - agent_function_versions__ek0_testing

=== Check Non-Existent Collection ===
Collection 'nonexistent_collection_xyz' exists: false

=== Cleanup ===
Deleted collection 'collection_utils_test_rust'

✓ Collection Utilities example complete
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_convenience_methods`
=== ekoDB Convenience Methods Example ===

=== Record Builder Pattern ===
✓ Created record with fluent builder: Record({"id": String("CucK7lhNBt4dGiwHjzVLeS_xijZEWyC5_-8Wod4ulyhAQD-l_rvl5tfjVCF3ELFqIW2QvsvgEn5a5Edm5YnN7Q")})

=== Upsert Operation ===
✓ First upsert (insert): Record({"id": String("bob-user-id")})
✓ Second upsert (update): Record({"age": Object({"value": Integer(36), "type": String("Integer")}), "name": Object({"type": String("String"), "value": String("Bob Smith")}), "email": Object({"type": String("String"), "value": String("bob.smith@newdomain.com")}), "id": String("bob-user-id")})

=== Find One Operation ===
✓ Found user by email: Record({"name": Object({"value": String("Alice Johnson"), "type": String("String")}), "id": String("CucK7lhNBt4dGiwHjzVLeS_xijZEWyC5_-8Wod4ulyhAQD-l_rvl5tfjVCF3ELFqIW2QvsvgEn5a5Edm5YnN7Q"), "email": Object({"type": String("String"), "value": String("alice@example.com")}), "age": Object({"type": String("Integer"), "value": Integer(28)}), "active": Object({"type": String("Boolean"), "value": Boolean(true)})})
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_document_ttl`
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "nAGtjRYlOp__jInBkWy_wYJO938HsHvVKhzgnaMY2tn_ZZ_EcTX7spXjQoe537YptbNvzF2dJE_FGUNKg6y9rw"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("E-Uao5Xd9Q3--mmkzXnZd2T7vcbh0MjgrBA7VdWdosdXNF7SEIwtw1Qtano5AY6vxgpsWfjkInRlGY3gIU3jCg"))

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
     Running `target/debug/examples/client_edge_cache`
=== ekoDB as Edge Cache - Simple Example ===

Setting up edge cache collection...
✓ Cache entry created

Creating edge cache lookup script...
✓ Edge cache script created: Na998lmlFEiW-6TGUq2TrRO7un4aMFIfjAlT7SDhoRYH6al8Pv01KU1saci4pT8y7x7CcTijVIv1tnrUY4YS5g

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

🧹 Cleaning up...
✓ Cleanup complete

✓ Example complete!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
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
   ⏱️  Duration: 95.444292ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "email": "Sincere@april.biz",
      "phone": "1-770-736-8031 x56442",
      "company": {
        "catchPhrase": "Multi-layered client-server neural-net",
        "b...

Second call (cache hit - from cache):
   ⏱️  Duration: 6.290416ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "email": "Sincere@april.biz",
      "website": "hildegard.org",
      "company": {
        "name": "Romaguera-Crona",
        "catchPhrase": "Multi-layered client-s...
   🚀 Cache speedup: 15.2x faster!

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_function_contract`
client_function_contract: ok
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_functions`
=== ekoDB Rust Client - Functions Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: 9aVtM-P4BQnoWtNqECKpEfXhCRSZJ1l2HQIDZgDE0VHYV3BQqYVBx40ecxz3c0gl1NgGBAvgZFYrmAyFuhlL2Q
📊 Found 5 active users

📝 Example 2: Parameterized Function

✅ Function saved: 0I8jfyaEwyA9520vh561Uz5LCgaMcbWwjGINpJDHt4VVpOaW6dAow8ePAu3xjStkbP_Nb4sQg13UhcsKX_yWdw
📊 Found 3 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: Sv949c_bArcEbSkfYhFjcadw6UZgtuGO4HhOiu3zKLwQs6Gu1aBiBqsVvtpiHtYdTzu3L7JlgOzVN17k_neQUQ
📊 Statistics: 2 groups

📝 Example 4: Function Management

📋 Total functions: 3
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.


✅ All examples completed!
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
   Record({"count": Object({"type": String("Integer"), "value": Integer(3)}), "avg_price": Object({"type": String("Float"), "value": Float(365.6666666666667)}), "category": Object({"type": String("String"), "value": String("Furniture")})})
   Record({"category": Object({"type": String("String"), "value": String("Electronics")}), "avg_price": Object({"value": Float(367.0), "type": String("Float")}), "count": Object({"value": Integer(5), "type": String("Integer")})})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_functions_ai`
🚀 ekoDB Rust AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer several benefits, including:

1. **Efficient Similarity Search**: They enable fast nearest neighbor searches in high-dimensional spaces, making them ideal for applications like image and text similarity.

2. **Scalability**: Designed to handle large volumes of vector data, they can easily scale as data grows.

3. **Support for Unstructured Data**: Vector databases can efficiently manage unstructured data formats, such as images, audio, and text embeddings.

4. **Real-time Processing**: They facilitate real-time querying and updates, essential for dynamic applications like recommendation systems.

5. **Integration with Machine Learning**: They support machine learning model outputs, allowing for seamless integration with AI applications and workflows.

6. **Advanced Query Capabilities**: Users can perform complex queries based on vector similarities, enabling nuanced data retrieval strategies.

7. **Data Compression**: Many vector databases use techniques to reduce storage requirements for high-dimensional data.

8. **Flexibility**: They can work with various data types and support hybrid queries involving both traditional and vector data.
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

✅ Function saved: y-xNI1LBXvN9npQOyDJ_saeM7KHR9MNFBCh6fM8It31EZtxOXOnu4vu85QMeBvxcpU1y8Sv-nB4xpfXO0-dRzg
📊 Found 2 product groups
   Record({"category": Object({"type": String("String"), "value": String("Furniture")}), "count": Object({"value": Integer(2), "type": String("Integer")}), "avg_price": Object({"value": Float(474.0), "type": String("Float")})})
   Record({"avg_price": Object({"type": String("Float"), "value": Float(575.6666666666666)}), "count": Object({"value": Integer(3), "type": String("Integer")}), "category": Object({"type": String("String"), "value": String("Electronics")})})
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   Record({"count": Object({"value": Integer(3), "type": String("Integer")}), "category": Object({"type": String("String"), "value": String("Electronics")})})
   Record({"count": Object({"type": String("Integer"), "value": Integer(2)}), "category": Object({"value": String("Furniture"), "type": String("String")})})
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
   Record({"status": Object({"value": String("inactive"), "type": String("String")}), "count": Object({"type": String("Integer"), "value": Integer(3)})})
   Record({"count": Object({"type": String("Integer"), "value": Integer(7)}), "status": Object({"value": String("active"), "type": String("String")})})
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

✅ Inserted order: Some(String("LaOMBbV0SfwHuPceQp5O-tHRQ-rfQ6wP75isc4YUxnVhpW3mjlCr36_gZo_EwA43V2bqLP935Mjlbihmgs5kng"))
✅ Inserted 2 products with wrapped types

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: 4D3Y_d2iVkKf-db2oj-ikQki8Ar_FEuaUGiTyjvQM16ae0KWZuqZJ0Vv5rgkXDyS0GsZy3xSPhSnPTWHI0vQ4A
📊 function executed
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: Some(Object {"type": String("Object"), "value": Object {"role": String("admin"), "userId": String("user_abc")}})
🔍 Key exists: true
✅ Set cached data
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: _13o-qPDj-Mxv2A2WxXS04enD7XvbovRBuoS_ldHbnyLTa5uI1SUjNbBX0NTSMUaWaMvvHLbIh2JsvHuIYYenQ
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: FU2qt041QQ5an8j0ziTc6iZDus258Fz3a09alXQYrqhdEnZsDD-_OJq1PAhOWNWkSGsiqFdpfMpf_7CiDpCYuQ
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
   1. Getting Started with ekoDB (Database)
   2. Natural Language Processing (AI)
   3. Vector Databases Explained (Database)
   4. Database Design Principles (Database)
   5. Introduction to Machine Learning (AI)
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_goal_templates`
=== ekoDB Goal Template CRUD Example (Rust) ===

--- Creating goal template ---
Created template: Data Migration (id: IQj38d4yyPRAW24A3IoYb9zLowmSw9pR--5iQoCgFV8R7Y6y5ROTIo_VktxOnygTcm569VQorP5p4ZmJFGEKOw)

--- Listing templates ---
Templates: {"count":1,"items":[{"description":{"type":"String","value":"Template for migrating data between schemas"},"id":"IQj38d4yyPRAW24A3IoYb9zLowmSw9pR--5iQoCgFV8R7Y6y5ROTIo_VktxOnygTcm569VQorP5p4ZmJFGEKOw","steps":{"type":"Array","value":[{"description":"Analyze source schema"},{"description":"Create target schema"},{"description":"Migrate records"},{"description":"Validate results"}]},"title":{"type":"String","value":"Data Migration"}}]}

--- Getting template ---
Fetched: {"type":"String","value":"Data Migration"}

--- Updating template ---
Updated description: {"type":"String","value":"Updated: comprehensive data migration workflow"}

--- Deleting template ---
Template deleted successfully

✓ Goal template CRUD example completed
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_goals_tasks_agents`
=== ekoDB Goals, Tasks & Agents Integration Example (Rust) ===

--- Goal: create ---
Created goal: "Deploy v2.0 to production" (id: DzmXGUy357k94FQ4ATP6tlXwL_UoknVjAlCPw4Ycezi7ScNdouIUqBoFT1oSR1FTjOS7YAxV3R5-Hx-kBJvU5A)

--- Goal: list ---
Goals: {"count":1,"goals":[{"created_at":"2026-10-17T08:08:57.722876+00:00","description":"Full release cycle for version 2.0","id":"DzmXGUy357k94FQ4ATP6tlXwL_UoknVjAlCPw4Ycezi7ScNdouIUqBoFT1oSR1FTjOS7YAxV3R5-Hx-kBJvU5A","status":"pending","steps":"[{\"description\":\"Run integration tests\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]","title":"Deploy v2.0 to production","updated_at":"2026-10-17T08:08:57.722876+00:00"}]}

--- Goal: get ---
Fetched goal: {"type":"String","value":"Deploy v2.0 to production"}

--- Goal: update ---
Updated description: {"type":"String","value":"Updated: full v2.0 release with rollback plan"}

--- Goal: search ---
Search results: {"count":1,"items":[{"_score":12.87,"created_at":{"type":"DateTime","value":"2026-10-17T08:08:57.722876+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"DzmXGUy357k94FQ4ATP6tlXwL_UoknVjAlCPw4Ycezi7ScNdouIUqBoFT1oSR1FTjOS7YAxV3R5-Hx-kBJvU5A","status":{"type":"String","value":"pending"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-10-17T08:08:57.768748+00:00"}}]}

--- Goal step: start (step 0) ---
Step 0 started: {"created_at":{"type":"DateTime","value":"2026-10-17T08:08:57.722876+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"DzmXGUy357k94FQ4ATP6tlXwL_UoknVjAlCPw4Ycezi7ScNdouIUqBoFT1oSR1FTjOS7YAxV3R5-Hx-kBJvU5A","status":{"type":"String","value":"in_progress"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\",\"status\":\"InProgress\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-10-17T08:08:57.780028+00:00"}}

--- Goal step: complete (step 0) ---
Step 0 completed: {"created_at":{"type":"DateTime","value":"2026-10-17T08:08:57.722876+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"DzmXGUy357k94FQ4ATP6tlXwL_UoknVjAlCPw4Ycezi7ScNdouIUqBoFT1oSR1FTjOS7YAxV3R5-Hx-kBJvU5A","status":{"type":"String","value":"in_progress"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\",\"result\":\"All 247 tests passed\",\"status\":\"Completed\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-10-17T08:08:57.786128+00:00"}}

--- Goal step: start (step 1) ---
Step 1 started

--- Goal step: fail (step 1) ---
Step 1 failed: {"created_at":{"type":"DateTime","value":"2026-10-17T08:08:57.722876+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"DzmXGUy357k94FQ4ATP6tlXwL_UoknVjAlCPw4Ycezi7ScNdouIUqBoFT1oSR1FTjOS7YAxV3R5-Hx-kBJvU5A","status":{"type":"String","value":"in_progress"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\",\"result\":\"All 247 tests passed\",\"status\":\"Completed\"},{\"description\":\"Build release artifacts\",\"error\":\"Build timeout after 300s\",\"status\":\"Failed\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-10-17T08:08:57.798177+00:00"}}

--- Goal: complete ---
Goal completed (pending review): {"completed_at":{"type":"DateTime","value":"2026-10-17T08:08:57.809700+00:00"},"created_at":{"type":"DateTime","value":"2026-10-17T08:08:57.722876+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"DzmXGUy357k94FQ4ATP6tlXwL_UoknVjAlCPw4Ycezi7ScNdouIUqBoFT1oSR1FTjOS7YAxV3R5-Hx-kBJvU5A","status":{"type":"String","value":"pending_review"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\",\"result\":\"All 247 tests passed\",\"status\":\"Completed\"},{\"description\":\"Build release artifacts\",\"error\":\"Build timeout after 300s\",\"status\":\"Failed\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"summary":{"type":"String","value":"Partial completion — step 1 failed"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-10-17T08:08:57.809700+00:00"}}

--- Goal: approve ---
Goal approved: {"completed_at":{"type":"DateTime","value":"2026-10-17T08:08:57.809700+00:00"},"created_at":{"type":"DateTime","value":"2026-10-17T08:08:57.722876+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"DzmXGUy357k94FQ4ATP6tlXwL_UoknVjAlCPw4Ycezi7ScNdouIUqBoFT1oSR1FTjOS7YAxV3R5-Hx-kBJvU5A","status":{"type":"String","value":"in_progress"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\",\"result\":\"All 247 tests passed\",\"status\":\"Completed\"},{\"description\":\"Build release artifacts\",\"error\":\"Build timeout after 300s\",\"status\":\"Failed\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"summary":{"type":"String","value":"Partial completion — step 1 failed"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-10-17T08:08:57.816467+00:00"}}

--- Goal: create (for rejection) ---
Created goal2: "Refactor auth module" (id: dz5WXXqrMUt1HCiIa_CPrmhgJKXwoKcNHqHtTzcxg11uItDEclGioWHmxPaZyFeshc0jCQua-kaibMXNJUuMlg)

--- Goal: complete (goal2) ---

--- Goal: reject ---
Goal rejected: {"completed_at":{"type":"DateTime","value":"2026-10-17T08:08:57.836551+00:00"},"created_at":{"type":"DateTime","value":"2026-10-17T08:08:57.824352+00:00"},"description":{"type":"String","value":"Rewrite JWT handling"},"id":"dz5WXXqrMUt1HCiIa_CPrmhgJKXwoKcNHqHtTzcxg11uItDEclGioWHmxPaZyFeshc0jCQua-kaibMXNJUuMlg","reason":{"type":"String","value":"Breaks backward compatibility"},"status":{"type":"String","value":"failed"},"steps":{"type":"String","value":"[]"},"summary":{"type":"String","value":"Completed refactor"},"title":{"type":"String","value":"Refactor auth module"},"updated_at":{"type":"DateTime","value":"2026-10-17T08:08:57.836551+00:00"}}

--- Task: create ---
Created task: Nightly backup (id: TJqMtZzEcNanCErYtI4Ht6nPN3Fni6Ee4rMU1DHXz80j0tkwXvLamlhoPutEYj63LnS5XX9ZCv2xTWqqJquwcg)

--- Task: list ---
Tasks: {"count":1,"items":[{"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"TJqMtZzEcNanCErYtI4Ht6nPN3Fni6Ee4rMU1DHXz80j0tkwXvLamlhoPutEYj63LnS5XX9ZCv2xTWqqJquwcg","max_retries":{"type":"Integer","value":3},"schedule":{"type":"String","value":"0 2 * * *"},"title":{"type":"String","value":"Nightly backup"}}]}

--- Task: get ---
Fetched task: {"type":"String","value":"Nightly backup"}

--- Task: start ---
Task started: {"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"TJqMtZzEcNanCErYtI4Ht6nPN3Fni6Ee4rMU1DHXz80j0tkwXvLamlhoPutEYj63LnS5XX9ZCv2xTWqqJquwcg","max_retries":{"type":"Integer","value":3},"schedule":{"type":"String","value":"0 2 * * *"},"status":{"type":"String","value":"running"},"title":{"type":"String","value":"Nightly backup"},"updated_at":{"type":"DateTime","value":"2026-10-17T08:08:57.879543+00:00"}}

--- Task: succeed ---
Task succeeded: {"consecutive_failures":{"type":"Integer","value":0},"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"TJqMtZzEcNanCErYtI4Ht6nPN3Fni6Ee4rMU1DHXz80j0tkwXvLamlhoPutEYj63LnS5XX9ZCv2xTWqqJquwcg","last_error":{"type":"Null","value":null},"last_run":{"type":"DateTime","value":"2026-10-17T08:08:57.889247+00:00"},"max_retries":{"type":"Integer","value":3},"run_count":{"type":"Integer","value":1},"schedule":{"type":"String","value":"0 2 * * *"},"status":{"type":"String","value":"active"},"title":{"type":"String","value":"Nightly backup"},"updated_at":{"type":"DateTime","value":"2026-10-17T08:08:57.889247+00:00"}}

--- Task: start (round 2) ---

--- Task: pause ---
Task paused: {"consecutive_failures":{"type":"Integer","value":0},"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"TJqMtZzEcNanCErYtI4Ht6nPN3Fni6Ee4rMU1DHXz80j0tkwXvLamlhoPutEYj63LnS5XX9ZCv2xTWqqJquwcg","last_error":{"type":"Null","value":null},"last_run":{"type":"DateTime","value":"2026-10-17T08:08:57.889247+00:00"},"max_retries":{"type":"Integer","value":3},"run_count":{"type":"Integer","value":1},"schedule":{"type":"String","value":"0 2 * * *"},"status":{"type":"String","value":"paused"},"title":{"type":"String","value":"Nightly backup"},"updated_at":{"type":"DateTime","value":"2026-10-17T08:08:57.906072+00:00"}}

--- Task: resume ---
Task resumed: {"consecutive_failures":{"type":"Integer","value":0},"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"TJqMtZzEcNanCErYtI4Ht6nPN3Fni6Ee4rMU1DHXz80j0tkwXvLamlhoPutEYj63LnS5XX9ZCv2xTWqqJquwcg","last_error":{"type":"Null","value":null},"last_run":{"type":"DateTime","value":"2026-10-17T08:08:57.889247+00:00"},"max_retries":{"type":"Integer","value":3},"run_count":{"type":"Integer","value":1},"schedule":{"type":"String","value":"0 2 * * *"},"status":{"type":"String","value":"active"},"title":{"type":"String","value":"Nightly backup"},"updated_at":{"type":"DateTime","value":"2026-10-17T08:08:57.916642+00:00"}}

--- Task: fail ---
Task failed: {"consecutive_failures":{"type":"Integer","value":1},"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"TJqMtZzEcNanCErYtI4Ht6nPN3Fni6Ee4rMU1DHXz80j0tkwXvLamlhoPutEYj63LnS5XX9ZCv2xTWqqJquwcg","last_error":{"type":"String","value":"S3 bucket access denied"},"last_run":{"type":"DateTime","value":"2026-10-17T08:08:57.922804+00:00"},"max_retries":{"type":"Integer","value":3},"run_count":{"type":"Integer","value":1},"schedule":{"type":"String","value":"0 2 * * *"},"status":{"type":"String","value":"active"},"title":{"type":"String","value":"Nightly backup"},"updated_at":{"type":"DateTime","value":"2026-10-17T08:08:57.922804+00:00"}}

--- Task: due ---
Due tasks: {"count":0,"items":[]}

--- Task: delete ---
Task deleted

--- Agent: create ---
Created agent: backup-agent (id: FIhcTLLeewVVvVgp5TeRtXJm09oGUuL6sNtOVxlpJhaOM-hoccySVlaD6KaHRdS_dlmXjjtCFtpYjj83U0Sp8w)

--- Agent: list ---
Agents: {"count":1,"items":[{"capabilities":{"type":"Array","value":["backup","restore","verify"]},"deployment_id":{"type":"String","value":"deploy-abc-123"},"description":{"type":"String","value":"Handles nightly backups and restores"},"id":"FIhcTLLeewVVvVgp5TeRtXJm09oGUuL6sNtOVxlpJhaOM-hoccySVlaD6KaHRdS_dlmXjjtCFtpYjj83U0Sp8w","llm_model":{"type":"String","value":"gpt-4.1"},"name":{"type":"String","value":"backup-agent"}}]}

--- Agent: get ---
Fetched agent: {"type":"String","value":"backup-agent"}

--- Agent: get_by_name ---
Found by name: {"type":"String","value":"backup-agent"}

--- Agent: update ---
Updated agent description: {"type":"String","value":"Handles backups, restores, and disaster recovery"}

--- Agent: agents_by_deployment ---
Agents in deployment: {"count":0,"items":[]}
WARNING: agents_by_deployment omitted created agent FIhcTLLeewVVvVgp5TeRtXJm09oGUuL6sNtOVxlpJhaOM-hoccySVlaD6KaHRdS_dlmXjjtCFtpYjj83U0Sp8w; TODO: check/fix the server-side deployment lookup

--- Agent: delete ---
Agent deleted

--- Goal: delete ---
Goal 1 deleted
Goal 2 deleted

=== All goals, tasks & agents operations completed ===
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_kv_links`
=== ekoDB KV Document Linking Example (Rust) ===

--- Inserting documents ---
Inserted doc1: Project Alpha (id: Eyzas9NF5YHvFjtewdqE-qW8WAcL8wcm1IPldgSv4_o2ksFTmYXwtrwlUnOmluvjtwrY9OyWTE9F6h99DMX9fw)
Inserted doc2: Project Beta (id: n2NMIkFIzgALVX8jGnHYzg3OAiCrR2woaZHH0H7Im8J9bfavUQR-scIngziKrQbYKoi3JE0sM_ukw9YRIkKKjA)

--- Setting KV key ---
Set key: kv_links:rs:user:alice:projects

--- Linking documents ---
Linked doc1: null
Linked doc2: null

--- Getting links ---
Links for kv_links:rs:user:alice:projects: [{"collection":"kv_links_example_rs","created_at":"2026-10-17T08:11:31.424103Z","document_id":"Eyzas9NF5YHvFjtewdqE-qW8WAcL8wcm1IPldgSv4_o2ksFTmYXwtrwlUnOmluvjtwrY9OyWTE9F6h99DMX9fw","field_path":null,"last_accessed":"2026-10-17T08:11:31.428394Z","metadata":{}},{"collection":"kv_links_example_rs","created_at":"2026-10-17T08:11:31.426867Z","document_id":"n2NMIkFIzgALVX8jGnHYzg3OAiCrR2woaZHH0H7Im8J9bfavUQR-scIngziKrQbYKoi3JE0sM_ukw9YRIkKKjA","field_path":null,"last_accessed":"2026-10-17T08:11:31.428394Z","metadata":{}}]

--- Unlinking doc1 ---
Unlinked doc1: null

--- Verifying remaining links ---
Remaining links: [{"collection":"kv_links_example_rs","created_at":"2026-10-17T08:11:31.426867Z","document_id":"n2NMIkFIzgALVX8jGnHYzg3OAiCrR2woaZHH0H7Im8J9bfavUQR-scIngziKrQbYKoi3JE0sM_ukw9YRIkKKjA","field_path":null,"last_accessed":"2026-10-17T08:11:31.429438Z","metadata":{}}]

--- Cleanup ---
Unlinked remaining document
Deleted KV key
Deleted documents
Deleted collection

=== All KV linking operations completed ===
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
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
  kv_operations:rs:cache:product:1: Record({"name": String("Product 1"), "price": Float(29.99)})
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
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
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"value": String("Eve"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"value": String("Eve"), "type": String("String")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))

=== NOT IN Operator ===
✓ Found 4 users not inactive

=== String Pattern Matching ===
✓ Found 5 users with @example.com email

=== Prefix Query (StartsWith) ===
✓ Found 1 users with names starting with A
  - Some(Object({"type": String("String"), "value": String("Alice")}))

=== Complex Query (active AND age >= 28 AND score > 1500) ===
✓ Found 2 users matching all conditions
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))

=== OR Query ===
✓ Found 2 users with age < 28 OR age > 32
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"value": String("Charlie"), "type": String("String")}))

=== Sorted Query (by score descending) ===
✓ Top 3 users by score:
  1. Some(Object({"type": String("String"), "value": String("Bob")}))
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_raw_completion_stream`
=== ekoDB Raw Completion Stream (SSE) Example ===

--- Basic SSE Raw Completion ---
Response: The three primary colors are red, blue, and yellow.

--- Structured Output via SSE ---
JSON response: [
  {
    "name": "Earth",
    "diameter_km": 12742
  },
  {
    "name": "Mars",
    "diameter_km": 6779
  },
  {
    "name": "Jupiter",
    "diameter_km": 139820
  }
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello! I hope you're having a wonderful day.

=== Done ===
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/examples/client_schedules`
=== ekoDB Schedule Management Example (Rust) ===

--- Creating schedule ---
Created schedule: "nightly-cleanup" (id: 0171a7d8-eb74-4348-b09c-8484486558c7)

--- Listing schedules ---
Schedules: {"count":1,"schedules":[{"created_at":"2026-10-17T08:16:40.805872Z","cron_expression":"0 0 3 * * *","description":"Remove expired sessions and temp files","enabled":true,"function_label":"schedule_noop_rust_48372","id":"0171a7d8-eb74-4348-b09c-8484486558c7","last_execution":null,"name":"nightly-cleanup","next_execution":"2026-10-18T03:00:00Z","parameters":{},"stats":{"avg_execution_time_ms":0.0,"failed_executions":0,"last_error":null,"successful_executions":0,"total_executions":0},"timezone":"UTC","updated_at":"2026-10-17T08:16:40.805872Z"}]}

--- Getting schedule ---
Fetched schedule: "nightly-cleanup"

--- Updating schedule ---
Updated cron: "0 0 4 * * *"
Updated description: "Remove expired sessions, temp files, and orphaned uploads"

--- Triggering schedule ---
Trigger response: {"schedule_id":"0171a7d8-eb74-4348-b09c-8484486558c7","status":"triggered"}

--- Pausing schedule ---
Schedule paused: {"created_at":"2026-10-17T08:16:40.805872Z","cron_expression":"0 0 4 * * *","description":"Remove expired sessions, temp files, and orphaned uploads","enabled":false,"function_label":"schedule_noop_rust_48372","id":"0171a7d8-eb74-4348-b09c-8484486558c7","last_execution":"2026-10-17T08:16:40.847516Z","name":"nightly-cleanup","next_execution":null,"parameters":{"type":"Object","value":{}},"stats":{"avg_execution_time_ms":0.0,"failed_executions":0,"last_error":null,"successful_executions":0,"total_executions":0},"timezone":"UTC","updated_at":"2026-10-17T08:16:40.851284Z"}

--- Resuming schedule ---
Schedule resumed: {"created_at":"2026-10-17T08:16:40.805872Z","cron_expression":"0 0 4 * * *","description":"Remove expired sessions, temp files, and orphaned uploads","enabled":true,"function_label":"schedule_noop_rust_48372","id":"0171a7d8-eb74-4348-b09c-8484486558c7","last_execution":"2026-10-17T08:16:40.847516Z","name":"nightly-cleanup","next_execution":"2026-10-18T04:00:00Z","parameters":{"type":"Object","value":{}},"stats":{"avg_execution_time_ms":0.0,"failed_executions":0,"last_error":null,"successful_executions":0,"total_executions":0},"timezone":"UTC","updated_at":"2026-10-17T08:16:40.854314Z"}

--- Deleting schedule ---
Schedule deleted successfully

=== All schedule operations completed ===
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_schema`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("shL2yfLbSDSr6c1zUyMIyLDAGV78drW5JBBah33q6Vh52LfKb5LE-v9-_stclbjs1iL0flIy-4KFM5qUzoLUQA"))
✓ Inserted user 2: Some(String("OsZi_cg9A_-EWzqpLqVa2yd_BdRzXBWv3l_TSY5sWS0j3vaH32c9Zp84r7BIKaM4fCJ6QGg0n87Qy8RIoDFCEg"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - title: String
    (required)
  - status: String
  - age: Integer
  - email: String
    (required)

=== Listing Collections ===
✓ Total collections: 13
  Sample: ["chat_goals__ek0_testing", "chat_messages__ek0_testing", "schedules__ek0_testing", "chat_raw_completions__ek0_testing", "schema_client_rust"]

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
     Matched: ["title", "description"]
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
Inserted: Record({"id": String("R0LvzK_GFXeqntMhXJhpbRvkgoOyjmrGsBKuqEVoSBakzvHsdyURehrikrCQ10fbL9Nw5ykVWQdDHvVrDovSyQ")})

=== Find by ID ===
Found: Record({"value": Object({"value": Integer(42), "type": String("Integer")}), "price": Object({"value": Float(99.99), "type": String("Float")}), "id": String("R0LvzK_GFXeqntMhXJhpbRvkgoOyjmrGsBKuqEVoSBakzvHsdyURehrikrCQ10fbL9Nw5ykVWQdDHvVrDovSyQ"), "created_at": Object({"type": String("DateTime"), "value": String("2026-10-17T08:19:41.277959+00:00")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "embedding": Vector([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "name": Object({"type": String("String"), "value": String("Test Record")}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "metadata": Object({"type": String("Object"), "value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})})}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-10-17T08:19:41.277959+00:00")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 5 dims
  categories (Set): 2 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record({"price": Object({"type": String("Float"), "value": Float(99.99)}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "created_at": Object({"type": String("DateTime"), "value": String("2026-10-17T08:19:41.277959+00:00")}), "id": String("R0LvzK_GFXeqntMhXJhpbRvkgoOyjmrGsBKuqEVoSBakzvHsdyURehrikrCQ10fbL9Nw5ykVWQdDHvVrDovSyQ"), "name": Object({"type": String("String"), "value": String("Test Record")}), "embedding": Vector([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "metadata": Object({"type": String("Object"), "value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")})}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "value": Object({"type": String("Integer"), "value": Integer(42)}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")})})]

=== Update Document ===
Updated: Record({"id": String("R0LvzK_GFXeqntMhXJhpbRvkgoOyjmrGsBKuqEVoSBakzvHsdyURehrikrCQ10fbL9Nw5ykVWQdDHvVrDovSyQ"), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "created_at": Object({"type": String("DateTime"), "value": String("2026-10-17T08:19:41.277959+00:00")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "name": Object({"type": String("String"), "value": String("Updated Record")}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "value": Object({"type": String("Integer"), "value": Integer(100)}), "metadata": Object({"type": String("Object"), "value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")})}), "embedding": Vector([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")})})

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: coGQDclSzpodVDSEDRXC2Gs-lXBz2rwbvmSX6TOOR8M_8wNML7Ej3m_Oqz4UYt5Uzos4vTVHQGZ6SOw-hS-A2A

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  id: "coGQDclSzpodVDSEDRXC2Gs-lXBz2rwbvmSX6TOOR8M_8wNML7Ej3m_Oqz4UYt5Uzos4vTVHQGZ6SOw-hS-A2A"
  name: "WebSocket Test Record"
  value: 42
  active: true

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_swr_native`
=== ekoDB Native SWR Function ===

This example shows the new simplified SWR function that replaces
the manual FindById → If → HttpRequest → Insert pattern.

Example 1: Basic GitHub User Cache with Native SWR
─────────────────────────────────────────────────────

✓ Created native SWR script: github_user_native_rs (N5C_yo5cAGDQBgpQvfa_K6ay90uVrw8goP_h9_X5_Bqlv5WAshcgQaEvEiyTjqyE58yJ6ZXBJN7-so8ZhSMNgg)

First call (cache miss - will fetch from GitHub API):
  Response time: 120ms
  Records returned: 1
  ✓ Data fetched from API and cached with 15m TTL

Second call (cache hit - instant from KV store):
  Response time: 6ms
  Speedup: 20.0x faster
  ✓ Lightning fast cache hit


Example 2: SWR with Audit Trail Collection
─────────────────────────────────────────────────────

✓ Created SWR script with audit trail: product_swr_audit_rs (A-TbBLghopaWG6RJKGuFerNDV3NMO5ekAR7QpY6MLQ_qt5Y3YYASBrPuOh4lH9ZKG20L4lWmJDuMFfqTkcITNQ)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_rs' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
─────────────────────────────────────────────────────

Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_rs (qI2s8Q_1YtblzLZSJBubNhPny8qj6XCwjZ-GXPE9VogmDnLbcvwTLAuyWBoHoWgG-IsqspevNiU5EIffNqmDPA)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_rs' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
─────────────────────────────────────────────────────

✓ Created dynamic TTL script: flexible_cache_rs (lU2ICMmu2lr3yf6yx9TEyK9qOR63j4dceHy0kd3R8iUjB8Am4yoQ4YaH3YdYgaJZ0UU3-BtoV-0WaQEvByhKwQ)
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

    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/examples/client_swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_rs (D20wz4D4ZSArp5K7ujF4tDD3yBp121XRXsUdy2KopzHLDJ0P2G01QBJSFhUCho_y-mgz3nCiypFHdfGW8NmDwg)

Step 2: First call - Cache miss, fetches from API
Result: FunctionStats { input_count: 0, output_count: 1, execution_time_ms: 0, stages_executed: 2, stage_stats: [] }
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 9ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_transactions`
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 3z7_I75sne-5YWWoIYPfLMCbg4KIQOBbmqDkeKQZlkxIeCArB4jygrwFUFDtOKSfOTWoTV3QegOD83dElV33rQ
Created Bob: $500 - ID: b255IcUEEjfGGZfp5stJSwfn_71Gs7Q8iLrEAK6vDw-7uOfmF8BgjMCzi2pyDgAQu1YfHeYSgXRaWVByiaCtBw

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): c3a9b496-4427-4e6f-b07c-3a07c9cc0091

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
New transaction: e552af97-5aeb-4d40-915b-818c16e1d748
Updated Bob: $700 → $600 (in transaction)
Status before rollback: "Active"
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_user_functions`
✓ Client created

=== Create User Function ===
Created user function with ID: 9otS4N3Q2U2AdAeNSbwtuREp0b5dPfa1rtfuHL5d6C7EkrhAjhgY15Z0zXv-FeuHkmV6No3lXgWY7eZY4PDmGw

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_websocket_chat_stream`
=== WebSocket Chat Streaming Example (Rust) ===

✓ Authentication successful
✓ Created chat session: adkpoS6nptB2B4vM4zT0RAY5fPAepgrpOPzHu1oJoK8FDZH7smAFIZFkeCRVuNsehqWoO8e7ThopJH-zvm2JbQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Sending message: 'What is the capital of France?' ===
✓ Message accepted, streaming...

{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: "N-SXYNKUbK3ujSjwjNsFvH67xcOHhgoOpMs6ir0x6VKIfh_dl4ef7lJXeQ3SZrkRzrGIGBgAXslU-_r4W35FSg"
Execution time: 958ms
Token usage: {"completion_tokens":8,"prompt_tokens":15,"total_tokens":23}
Context window: 1000000 tokens

Full response: {"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris....

✓ WebSocket chat streaming example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_websocket_subscribe`
✓ Authentication successful

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_rs' ===
✓ Subscribed (subscription_id: sub_b8ef74e7d3814477bf7e3954743e0ac0)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: V0yS4bjHi52MA2nMOKuwPwQSNAWnzTvgFz3yGQlegEOaguCqEEDLOw8rMjc0Ds40mgwpxmcPgR5WqaoHnNdQJg

  📡 Notification received:
     Event:      "insert"
     Collection: "ws_subscribe_example_rs"
     Record IDs: ["V0yS4bjHi52MA2nMOKuwPwQSNAWnzTvgFz3yGQlegEOaguCqEEDLOw8rMjc0Ds40mgwpxmcPgR5WqaoHnNdQJg"]
     Timestamp:  "2026-10-17T08:26:24.804715+00:00"

Inserting record 2...
✓ Inserted: 2_oLNUh8bOKLNlxQQj6VNEs5oAq-PGd9Avf3nd1RiMID3w0LrB6NkDXMzNpHY-1ROLtZL3_Xc0ETj4v5U0kySA

  📡 Notification received:
     Event:      "insert"
     Record IDs: ["2_oLNUh8bOKLNlxQQj6VNEs5oAq-PGd9Avf3nd1RiMID3w0LrB6NkDXMzNpHY-1ROLtZL3_Xc0ETj4v5U0kySA"]

=== Unsubscribing ===
✓ Unsubscribed

=== Cleanup ===
✓ Deleted collection 'ws_subscribe_example_rs'

✓ WebSocket subscription example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/examples/client_websocket_ttl`
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Some(String("dv0y1l6PzZyU5ACelD5S4snGVEUQEpqJ0dWnUMNYxw01IoYNoo8IPht06aziNNTxNrjE9J3qVgqWrUIe4TJeuQ"))

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
   Inserted with ripple: Record({"id": String("vMHFY-VNxUB_cZFH9Wr5tJt7-BYuGjLMPfxWRULOosAhrsdGohy9u5JXcb0yJ7ISjWtZmwn8skHvvdISapvfgA")})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: Record({"id": String("eQc4vUMpbeTeFD2NUWJTKB6BxOlp1zadQIc3LfsEpE5Am59MKw1Z_VXlyeU4kjyAb-2-QAJuvAa9qQX0XhH6EQ")})

3. Update with bypass_ripple:
   Updated with bypass_ripple: Record({"price": Object({"type": String("Integer"), "value": Integer(150)}), "name": Object({"type": String("String"), "value": String("Product 1")}), "id": String("vMHFY-VNxUB_cZFH9Wr5tJt7-BYuGjLMPfxWRULOosAhrsdGohy9u5JXcb0yJ7ISjWtZmwn8skHvvdISapvfgA")})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: Record({"id": String("custom-id")})

✅ All bypass_ripple operations completed successfully!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
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
  Fields returned: ["id", "avatar_url", "bio", "email", "name", "status", "user_role", "age", "created_at"]

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)

Example 4: Query inactive users with profile fields
  Found 1 inactive users

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ["id", "age", "status", "avatar_url", "user_role", "api_key", "password", "created_at", "bio", "name", "email", "secret_token"]
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
Inserted: {'id': 'y5y8P1XKid63YlC0a6arE1MUg3Wxh9X5g1SkYN--_irWMWQaWZggpza7z8TN0jFrQT-p7LRPZrT6gkPPZb0fwg'}

=== Find by ID ===
Found: {'value': {'type': 'Integer', 'value': 42}, 'id': 'y5y8P1XKid63YlC0a6arE1MUg3Wxh9X5g1SkYN--_irWMWQaWZggpza7z8TN0jFrQT-p7LRPZrT6gkPPZb0fwg', 'active': {'value': True, 'type': 'Boolean'}, 'name': {'type': 'String', 'value': 'Test Record'}}

=== Find with Query ===
Found documents: [{'name': {'type': 'String', 'value': 'Test Record'}, 'active': {'type': 'Boolean', 'value': True}, 'value': {'type': 'Integer', 'value': 42}, 'id': 'y5y8P1XKid63YlC0a6arE1MUg3Wxh9X5g1SkYN--_irWMWQaWZggpza7z8TN0jFrQT-p7LRPZrT6gkPPZb0fwg'}]

=== Update Document ===
Updated: {'name': {'type': 'String', 'value': 'Updated Record'}, 'id': 'y5y8P1XKid63YlC0a6arE1MUg3Wxh9X5g1SkYN--_irWMWQaWZggpza7z8TN0jFrQT-p7LRPZrT6gkPPZb0fwg', 'value': {'type': 'Integer', 'value': 100}, 'active': {'type': 'Boolean', 'value': True}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: IjIB7yLYrTRr3IqAzcyizJ2CAHvZbEMy1XjT3rtDNsjq3GdzOKEXzgTKMJRXnmb8sdPOBfz6zBcTFm3HvyFHmQ

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
        "id": "IjIB7yLYrTRr3IqAzcyizJ2CAHvZbEMy1XjT3rtDNsjq3GdzOKEXzgTKMJRXnmb8sdPOBfz6zBcTFm3HvyFHmQ",
        "active": {
          "type": "Boolean",
          "value": true
        },
        "name": {
          "value": "WebSocket Test Record",
          "type": "String"
        }
      }
    ]
  },
  "messageId": "1847759617"
}
✓ Retrieved 1 record via WebSocket

✓ WebSocket example completed successfully
🚀 ekoDB Functions Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: XfsA6SC8Bm-Qiw9hmBem9Zz411suiu7vEEafSptHsmXPvNJfk9mT1KbWpWl9omMliGOl_UGG7fV3-y9hOB5gSw
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: Ge-EcNy1isFw9I38XyJ_I8nlnEWJLcDavX9skUfmk6wZLuP3Up5GZWncBMVSiM3BXNG9VrdqHTQqJPFIzE_jeg
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: IJlsrYZE8TuKpMoqOBesNrx4J2g9bpTcxJMJJXwRLiE1Vz_7dXk60r_jejSTCsPf2_Q8V8NwphghLykVhqZ3xw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'status': {'type': 'String', 'value': 'active'}, 'max_score': {'value': 100, 'type': 'Integer'}, 'avg_score': {'value': 60.0, 'type': 'Float'}, 'count': {'type': 'Integer', 'value': 5}}
   {'avg_score': {'value': 50.0, 'type': 'Float'}, 'max_score': {'type': 'Integer', 'value': 90}, 'status': {'type': 'String', 'value': 'inactive'}, 'count': {'value': 5, 'type': 'Integer'}}

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
kv_operations:direct:py:cache:product:1: {'type': 'Object', 'value': {'name': 'Product 1', 'price': 29.99}}
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
Collection created with first record: IUE2MTLXM8GJ8Ckia3W3z34sSBNCQX4OI4Y-lDjO51hfU2dn3B5l3C5zT-TnUETMCH4EC8cbiBpUzxsF6jyP9w

=== List Collections ===
Total collections: 13
Sample collections: ['chat_goals__ek0_testing', 'chat_messages__ek0_testing', 'schedules__ek0_testing', 'chat_raw_completions__ek0_testing', 'chat_agent_configs__ek0_testing']

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
Created Alice: $1000 - ID: sl9W8LqkD73HDYpSy3eFnyd20e_vklpjQEtga7rhYXUTK08FoV2-Y5IrVrdPkfaAQYwZIeQiyAKuMoyEso5c_Q
Created Bob: $500 - ID: lOyd0EzLzFZqZTYLZob0MzWUSgmjyIjZFJrVuGfQuJQ6hogXA1lceokMiI2JBfydyL4iRXTOu6OASlekwks_vw

=== Example 1: Begin Transaction ===
Transaction ID: 971da3e0-1fcc-4616-8a19-cef2ca130181

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
New transaction: a9c1cf1a-3c1b-432f-8c8a-1dd2d6796883
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: K5QueZ3ik7-_8dIDcJTAl_okPH678QlXFxCGLnw7SOUzPvUieqjBHA8qPVNdzDnptupUCEZdhlf60I2CHHbRLg
Created Bob: $500 - ID: unoteICJmL6d_CMtxQI_qYwywzhkU9dHJe4OapOyhJZPAzBAEG_WYDWq1CMk45ddrw9C0dmebNoo9XDF74Mwnw

=== Example 1: Begin Transaction ===
Transaction ID: f6a3ac1c-23c8-4101-a651-680402acacf4

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
Created Alice: $1000 - ID: 31mlY4iVos5aRqnbYxxupUd960wZK--efhMteihP0hSeloAcXCUKRsj5SniYaVpvQMRS_7kMdwoK-sZcGsBmbw
Created Bob: $500 - ID: j9YDnyg-oH-qmxbz3iLWMBl3pP5AcH3Hd1a-Ra1vZZMM6h81dFEvRIwZQGxLmuLk7wIB5vfUCROROjaYrLCwcQ

=== Example 1: Begin Transaction ===
Transaction ID: 35204f5a-f563-4084-9ae3-87d022e568d5

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
New transaction: f16bfbb8-b1b8-4e48-855e-01f70e4a05ae
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: vF7Ix6fi9XblqAmbnrf7jvraJlLs2NpbugN6ajXPX0_E2vLear11Ftxmbd9jrkrtKuFYWUeHlR8tPh53o8xJrw
Created Bob: $500 - ID: 6I-6HYyQP-JMUYwsrCTyy23jgSSWWLHAO8d8eKZiygUr8W0vVZ0ztMLdP3lSlWZ1h8Qi_d8oM4WLD9FebhvTbA

=== Example 1: Begin Transaction ===
Transaction ID: eee68f08-5511-4383-877a-c464092f4dd1

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
Created Alice: $1000 - ID: vlaCmiTzhaWwMelCtWL7dywOoujARYoPnVX4Wl1mpcgBTxDxe87l-Kr53Ewx2nJNMLjC2UJvo-aWAyQf14Hubw
Created Bob: $500 - ID: F_WcxbNrU_8-VCN0b7W2sLJCDsoeIKhtM1fk_SPs2O9T2SeWN7cglnPgUjABxNfR3FKFWv9rIgXmAgNJXHCSOw

=== Example 1: Begin Transaction ===
Transaction ID: c81e7ecd-b2d1-4f41-bc56-ba40762f725b

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
New transaction: 683a4b3d-c781-4a36-808e-66672e44d12a
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: pLWrof2r7YmRF_GlUUMd6-lWskiFdVT7tWSexumxwLlJBjg0iLzE9n2smi2X2tT02rkNjPXcSR-10885gVkgAQ
Created Bob: $500 - ID: YbqnPqEiLuU88CH2Zf3JK7DWc4ErebxB5qz_zBxjHPyyxKH21ViNnRio4itZXTwHZlDPB68ZiX6rgu_tyXyJ5g

=== Example 1: Begin Transaction ===
Transaction ID: d40516a2-84ea-44f1-8066-db8d0f120b43

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
🧹 Cleanup
============================================================
   ✅ Deleted collection: crud_functions_users_py
============================================================
📝 function 1: Insert + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: fflcCbalndEO89Sx9k9qdbtgYHuJTicPivdFdB9dqQE9Gleciq2mQdf_mPxYfowG9_gIQnxcqLkREOSIYU3xQQ

2️⃣ Calling function (Insert + Verify)...
   ✅ function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: tYOvfSJPeBRioJOCdb3tLansZp0RNbn7T2ShZ-3bMPeOkKuInRSiKF4ouZ-UTwkrukWnAF22SUxFvZYY3d_h-A
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'type': 'String', 'value': 'pending'}
   📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================
📝 function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: 1YvuxzMXFVGj1L8OlcJo_L368zlVz18-QMDqOa19IL2y2XkFq7x2F67w2ceMI1xBdTYlVEGLkHBA7vKXFnp5Hg

2️⃣ Calling function (Query + Update + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {'type': 'String', 'value': 'active'}
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}

============================================================
📝 function 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: UrQFEei8u5VPKSPRXMbQ3yxBRLhQJkMtgTY2JE1of-ZHulFsyhSYolaqlGY5aN77TDidY_VtLg6tMt1-CyNv_g

2️⃣ Calling function (Query + Update Credits + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'value': 100, 'type': 'Integer'}
   📋 Status: {'value': 'active', 'type': 'String'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: 51yfZCOAynnuqq60qqxb9pSWznrAwzLKvgJWTTOQsx02h2YIfCTTKfdBuccZTqXAl-NDKEG5Bu8nNrJLs5i7sA

2️⃣ Calling function (Query + Delete + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: fflcCbalndEO89Sx9k9q...
   ✅ Deleted script: 1YvuxzMXFVGj1L8OlcJo...
   ✅ Deleted script: UrQFEei8u5VPKSPRXMbQ...
   ✅ Deleted script: 51yfZCOAynnuqq60qqxb...
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
  Output: Document ID = qXs1YEnrTYR7o0_7Uso9M31MNgcJsRGeduLxYnOihJc9kGVI3FPbQc5ekDiNh2h_5QBZrAcm-dcXgDcTPU-_fA
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(qXs1YEnrTYR7o0_7Uso9M31MNgcJsRGeduLxYnOihJc9kGVI3FPbQc5ekDiNh2h_5QBZrAcm-dcXgDcTPU-_fA)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(qXs1YEnrTYR7o0_7Uso9M31MNgcJsRGeduLxYnOihJc9kGVI3FPbQc5ekDiNh2h_5QBZrAcm-dcXgDcTPU-_fA)
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
  Output: Document ID = GRa59bhFgU0OIAiE4pKqT-yZTeBj_OPEeNn9aUExQc5gE-xhr1nXPWovVWT8TnwdtELu3ElfXdH9_ovho_1Kow
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(GRa59bhFgU0OIAiE4pKqT-yZTeBj_OPEeNn9aUExQc5gE-xhr1nXPWovVWT8TnwdtELu3ElfXdH9_ovho_1Kow)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(GRa59bhFgU0OIAiE4pKqT-yZTeBj_OPEeNn9aUExQc5gE-xhr1nXPWovVWT8TnwdtELu3ElfXdH9_ovho_1Kow)
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
🐍 Found CPython 3.11 at /Library/Frameworks/Python.framework/Versions/3.11/bin/python3
🔗 Found pyo3 bindings with abi3-py3.8 support
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.21s
📦 Built wheel for abi3 Python ≥ 3.8 to ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.30.0-cp38-abi3-macosx_11_0_arm64.whl
📦 Installing Python wheel into .venv...
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.30.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.30.0
    Uninstalling ekodb_client-0.30.0:
      Successfully uninstalled ekodb_client-0.30.0
Successfully installed ekodb-client-0.30.0
🧪 Ensuring test dependencies (pytest) in .venv...
✅ Python client package built and installed!
=== ekoDB Advanced CRUD Example (Python) ===

--- insert ---
Inserted: DFXu_oRwFE_9a0TRrK8mmxs9wP1q9j8T_tTpFWzLw3a59o6fA4UPMMtsfm__B-4QDqfp7EDfrRuy9muigBVDVg

--- update_with_action (increment) ---
After increment count by 5: {'score': {'type': 'Float', 'value': 50.0}, 'tags': {'type': 'Array', 'value': ['initial']}, 'name': {'type': 'String', 'value': 'Counter Record'}, 'id': 'DFXu_oRwFE_9a0TRrK8mmxs9wP1q9j8T_tTpFWzLw3a59o6fA4UPMMtsfm__B-4QDqfp7EDfrRuy9muigBVDVg', 'count': {'type': 'Integer', 'value': 15}}

--- update_with_action (decrement) ---
After decrement score by 10: {'name': {'type': 'String', 'value': 'Counter Record'}, 'tags': {'value': ['initial'], 'type': 'Array'}, 'id': 'DFXu_oRwFE_9a0TRrK8mmxs9wP1q9j8T_tTpFWzLw3a59o6fA4UPMMtsfm__B-4QDqfp7EDfrRuy9muigBVDVg', 'count': {'value': 15, 'type': 'Integer'}, 'score': {'type': 'Float', 'value': 40.0}}

--- update_with_action_sequence ---
After action sequence: {'name': {'value': 'Counter Record', 'type': 'String'}, 'id': 'DFXu_oRwFE_9a0TRrK8mmxs9wP1q9j8T_tTpFWzLw3a59o6fA4UPMMtsfm__B-4QDqfp7EDfrRuy9muigBVDVg', 'count': {'type': 'Integer', 'value': 115}, 'score': {'type': 'Float', 'value': 40.0}, 'tags': {'type': 'Array', 'value': ['initial', 'sequenced']}}

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
✓ Created session: -PQUFSVnd7-dDMyRbV1hLqT4brIg8URLkBVuV4LLPn4YSMMVc6VJCE1RGprqPK5CAdy9Fz8LJAwDAO2D2ABmuw

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you need more details or have other questions, feel free to ask!

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['llm_provider', 'content', 'created_at', 'llm_model', 'context_snippets', 'role', 'id', 'updated_at', 'token_usage', 'chat_id'])
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
✓ Created second session: t3aTseu_CQ01Mp_eXeiA67hAaeuZorY6KCZ30X-aez7I-falwUQ_HQDQURYkvtFdGk7dnfSpiXRriFXTRMd1MA
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: t3aTseu_CQ01Mp_eXeiA67hAaeuZorY6KCZ30X-aez7I-falwUQ_HQDQURYkvtFdGk7dnfSpiXRriFXTRMd1MA
✓ Deleted chat session: -PQUFSVnd7-dDMyRbV1hLqT4brIg8URLkBVuV4LLPn4YSMMVc6VJCE1RGprqPK5CAdy9Fz8LJAwDAO2D2ABmuw
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: TL850Ud2qN1ikcEN5-kh2pXzHtq2PRf1fR9Ld2iCv2TPB6Mw9QugEl_s--uDc12_BKmitZHLnkGTDcRGrSv9wQ

=== Sending Chat Message ===
Message ID: RIM1ONAFhgiMoKNCjEGJa5J5rl1_gRPUFM1ooehdYQpUdH6BSX33okQelsGeMXx41H63KzGtcGdTrFxVGUT8gQ

=== AI Response ===
The available products and their prices are as follows:

1. **ekoDB Cloud**
   - Price: $499
   - Description: Fully managed cloud database service product.

2. **ekoDB**
   - Price: $99
   - Description: A high-performance database product with AI capabilities.

3. **ekoDB Pro**
   - Price: $299
   - Description: Enterprise edition product with advanced features.

Execution Time: 2903ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 91
Total tokens: 3504

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Message Stream (SSE) Example (Python) ===

Created session: TI_JRjNjvkQ7H_ofgJTaa93kGSmCQzu4RHqOd3Zpld78FFdEIMUceXQG84ilyz7HXKp5XBJayw9OdKdRcHKipQ

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}EkoDB is an API-friendly database designed to provide fast access to data. It is built to be lightweight and provides a simple interface for data storage and retrieval. As of my last knowledge update in October 2023, EkoDB may support various features suited for developers looking to handle data efficiently in applications that require quick read/write capabilities.

EkoDB could also be tailored for specific uses, such as real-time applications or projects with high-performance requirements. However, since specific features and updates can vary over time, I recommend checking the official EkoDB website or documentation for the latest information on its capabilities, use cases, and any recent developments.

--- Stream complete ---
Message ID: L4Zuc40lnYu6mMnyHI7_vj6V-zEGO5X-CREJrTMaxOVa9-fY3PLwrF41G2SLmdk4dUJNARBtAQzAGSjXAmtvyQ
Execution time: 1444ms
Context window: 128000 tokens

✓ Chat message stream example completed
✓ Client created

=== Get All Chat Models ===
OpenAI models: ['gpt-3.5-turbo-0125', 'gpt-5.4-2026-03-05', 'gpt-5-nano', 'gpt-5.4-mini', 'o1-pro', 'gpt-5.1-codex', 'tts-1', 'gpt-5-mini', 'gpt-5-mini-2025-08-07', 'gpt-4-turbo-2024-04-09', 'gpt-realtime-2.1-mini', 'gpt-realtime-2025-08-28', 'gpt-4', 'tts-1-hd', 'gpt-realtime-1.5', 'gpt-realtime', 'gpt-5.2-pro', 'gpt-5-chat-latest', 'gpt-5.5-pro', 'gpt-audio-2025-08-28', 'gpt-4o-mini-transcribe-2025-03-20', 'omni-moderation-2024-09-26', 'gpt-live-transcribe', 'gpt-5.6-luna', 'omni-moderation-latest', 'o4-mini-2025-04-16', 'gpt-4o-2024-11-20', 'gpt-5.1', 'whisper-1', 'gpt-5-pro', 'gpt-realtime-whisper', 'gpt-5-2025-08-07', 'gpt-5.4-mini-2026-03-17', 'babbage-002', 'gpt-4o-2024-05-13', 'text-embedding-3-large', 'chat-latest', 'gpt-realtime-translate', 'gpt-5.4-pro-2026-03-05', 'gpt-realtime-2', 'gpt-5-search-api', 'gpt-5.5-2026-04-23', 'tts-1-1106', 'gpt-5.5-pro-2026-04-23', 'gpt-audio-mini-2025-10-06', 'gpt-5.2-pro-2025-12-11', 'o3-mini-2025-01-31', 'gpt-audio-1.5', 'gpt-image-1.5', 'gpt-4-turbo', 'gpt-4o-2024-08-06', 'o3', 'gpt-4o-mini-search-preview-2025-03-11', 'gpt-4o-mini-transcribe-2025-12-15', 'gpt-4.1-mini', 'gpt-4o-mini', 'gpt-4.1-nano', 'gpt-image-2', 'gpt-3.5-turbo-16k', 'text-embedding-ada-002', 'gpt-realtime-2.1', 'gpt-4.1', 'gpt-5.5', 'gpt-4o-transcribe-diarize', 'gpt-5-codex', 'gpt-4-0613', 'gpt-5.1-2025-11-13', 'gpt-4o-search-preview', 'o1-2024-12-17', 'gpt-5-pro-2025-10-06', 'gpt-5.6-sol', 'gpt-4o-mini-tts-2025-12-15', 'gpt-image-1', 'gpt-audio', 'gpt-5.3-codex', 'gpt-5-search-api-2025-10-14', 'gpt-realtime-mini', 'gpt-image-1-mini', 'gpt-5.6-terra', 'gpt-3.5-turbo-instruct-0914', 'sora-2', 'gpt-5.4-pro', 'gpt-4o-mini-tts', 'gpt-realtime-mini-2025-12-15', 'gpt-5.1-codex-max', 'gpt-4o-mini-2024-07-18', 'gpt-4.1-2025-04-14', 'gpt-5', 'gpt-4o-mini-transcribe', 'gpt-image-2-2026-04-21', 'gpt-5.2-chat-latest', 'sora-2-pro', 'gpt-3.5-turbo-instruct', 'gpt-transcribe', 'gpt-4.1-nano-2025-04-14', 'gpt-5.1-chat-latest', 'tts-1-hd-1106', 'gpt-5.2-codex', 'gpt-4o', 'o4-mini-deep-research', 'gpt-5.4', 'gpt-4o-search-preview-2025-03-11', 'text-embedding-3-small', 'gpt-5-nano-2025-08-07', 'gpt-4o-transcribe', 'o4-mini', 'gpt-5.1-codex-mini', 'gpt-image-2.5-sunburst-2026-09-08', 'gpt-5.3-chat-latest', 'o3-2025-04-16', 'gpt-5.2-2025-12-11', 'chatgpt-image-latest', 'gpt-3.5-turbo', 'gpt-3.5-turbo-1106', 'gpt-4o-mini-tts-2025-03-20', 'gpt-5.2', 'gpt-5.4-nano-2026-03-17', 'o1', 'davinci-002', 'gpt-5.4-nano', 'o3-mini', 'o4-mini-deep-research-2025-06-26', 'gpt-audio-mini-2025-12-15', 'o1-pro-2025-03-19', 'gpt-6-luna', 'gpt-image-2.5-flare', 'gpt-6-astra', 'gpt-audio-mini', 'gpt-4.1-mini-2025-04-14', 'gpt-image-2.5-sunburst', 'gpt-image-2.5-flare-2026-09-08', 'gpt-4o-mini-search-preview', 'gpt-6-sol', 'gpt-live-1', 'gpt-6.1-sol']
Anthropic models: ['claude-haiku-5-5', 'claude-sonnet-5-5', 'claude-opus-5-5', 'claude-fable-5-1', 'claude-opus-5', 'claude-sonnet-5', 'claude-fable-5', 'claude-opus-4-8', 'claude-opus-4-7', 'claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929']
Perplexity models: []
Gemini models: []

=== Get OpenAI Models ===
OpenAI models: ['gpt-3.5-turbo-0125', 'gpt-5.4-2026-03-05', 'gpt-5-nano', 'gpt-5.4-mini', 'o1-pro', 'gpt-5.1-codex', 'tts-1', 'gpt-5-mini', 'gpt-5-mini-2025-08-07', 'gpt-4-turbo-2024-04-09', 'gpt-realtime-2.1-mini', 'gpt-realtime-2025-08-28', 'gpt-4', 'tts-1-hd', 'gpt-realtime-1.5', 'gpt-realtime', 'gpt-5.2-pro', 'gpt-5-chat-latest', 'gpt-5.5-pro', 'gpt-audio-2025-08-28', 'gpt-4o-mini-transcribe-2025-03-20', 'omni-moderation-2024-09-26', 'gpt-live-transcribe', 'gpt-5.6-luna', 'omni-moderation-latest', 'o4-mini-2025-04-16', 'gpt-4o-2024-11-20', 'gpt-5.1', 'whisper-1', 'gpt-5-pro', 'gpt-realtime-whisper', 'gpt-5-2025-08-07', 'gpt-5.4-mini-2026-03-17', 'babbage-002', 'gpt-4o-2024-05-13', 'text-embedding-3-large', 'chat-latest', 'gpt-realtime-translate', 'gpt-5.4-pro-2026-03-05', 'gpt-realtime-2', 'gpt-5-search-api', 'gpt-5.5-2026-04-23', 'tts-1-1106', 'gpt-5.5-pro-2026-04-23', 'gpt-audio-mini-2025-10-06', 'gpt-5.2-pro-2025-12-11', 'o3-mini-2025-01-31', 'gpt-audio-1.5', 'gpt-image-1.5', 'gpt-4-turbo', 'gpt-4o-2024-08-06', 'o3', 'gpt-4o-mini-search-preview-2025-03-11', 'gpt-4o-mini-transcribe-2025-12-15', 'gpt-4.1-mini', 'gpt-4o-mini', 'gpt-4.1-nano', 'gpt-image-2', 'gpt-3.5-turbo-16k', 'text-embedding-ada-002', 'gpt-realtime-2.1', 'gpt-4.1', 'gpt-5.5', 'gpt-4o-transcribe-diarize', 'gpt-5-codex', 'gpt-4-0613', 'gpt-5.1-2025-11-13', 'gpt-4o-search-preview', 'o1-2024-12-17', 'gpt-5-pro-2025-10-06', 'gpt-5.6-sol', 'gpt-4o-mini-tts-2025-12-15', 'gpt-image-1', 'gpt-audio', 'gpt-5.3-codex', 'gpt-5-search-api-2025-10-14', 'gpt-realtime-mini', 'gpt-image-1-mini', 'gpt-5.6-terra', 'gpt-3.5-turbo-instruct-0914', 'sora-2', 'gpt-5.4-pro', 'gpt-4o-mini-tts', 'gpt-realtime-mini-2025-12-15', 'gpt-5.1-codex-max', 'gpt-4o-mini-2024-07-18', 'gpt-4.1-2025-04-14', 'gpt-5', 'gpt-4o-mini-transcribe', 'gpt-image-2-2026-04-21', 'gpt-5.2-chat-latest', 'sora-2-pro', 'gpt-3.5-turbo-instruct', 'gpt-transcribe', 'gpt-4.1-nano-2025-04-14', 'gpt-5.1-chat-latest', 'tts-1-hd-1106', 'gpt-5.2-codex', 'gpt-4o', 'o4-mini-deep-research', 'gpt-5.4', 'gpt-4o-search-preview-2025-03-11', 'text-embedding-3-small', 'gpt-5-nano-2025-08-07', 'gpt-4o-transcribe', 'o4-mini', 'gpt-5.1-codex-mini', 'gpt-image-2.5-sunburst-2026-09-08', 'gpt-5.3-chat-latest', 'o3-2025-04-16', 'gpt-5.2-2025-12-11', 'chatgpt-image-latest', 'gpt-3.5-turbo', 'gpt-3.5-turbo-1106', 'gpt-4o-mini-tts-2025-03-20', 'gpt-5.2', 'gpt-5.4-nano-2026-03-17', 'o1', 'davinci-002', 'gpt-5.4-nano', 'o3-mini', 'o4-mini-deep-research-2025-06-26', 'gpt-audio-mini-2025-12-15', 'o1-pro-2025-03-19', 'gpt-6-luna', 'gpt-image-2.5-flare', 'gpt-6-astra', 'gpt-audio-mini', 'gpt-4.1-mini-2025-04-14', 'gpt-image-2.5-sunburst', 'gpt-image-2.5-flare-2026-09-08', 'gpt-4o-mini-search-preview', 'gpt-6-sol', 'gpt-live-1', 'gpt-6.1-sol']

=== Get Anthropic Models ===
Anthropic models: ['claude-haiku-5-5', 'claude-sonnet-5-5', 'claude-opus-5-5', 'claude-fable-5-1', 'claude-opus-5', 'claude-sonnet-5', 'claude-fable-5', 'claude-opus-4-8', 'claude-opus-4-7', 'claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929']

✓ Chat Models API example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 1TDljyFM6zVElbWR3KJPFWw2oCVSA2cS5WxU-1Id-RKnOzGgqnweN9R3EK8abe8xUuMPb_ZS3ADX3qEbNtBIig

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** A high-performance database product
- **Price:** $99

If you have any more questions or need further assistance, feel free to ask!

✓ Message 2 sent
  Response: The price of the product ekoDB is **$99**. If you have any more questions, feel free to ask!

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 0LoG9gPNp860hzUZ-9sxcNB5E7aF2PGHup9rQpBMe7uzF30u-nzrzHo-UrGkzBmNxI_eiwoiGu1TpFHCMPKsBg
  Parent: 1TDljyFM6zVElbWR3KJPFWw2oCVSA2cS5WxU-1Id-RKnOzGgqnweN9R3EK8abe8xUuMPb_ZS3ADX3qEbNtBIig

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: 0LoG9gPNp860hzUZ-9sxcNB5E7aF2PGHup9rQpBMe7uzF30u-nzrzHo-UrGkzBmNxI_eiwoiGu1TpFHCMPKsBg (Untitled)
  Session 2: 1TDljyFM6zVElbWR3KJPFWw2oCVSA2cS5WxU-1Id-RKnOzGgqnweN9R3EK8abe8xUuMPb_ZS3ADX3qEbNtBIig (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: 0LoG9gPNp860hzUZ-9sxcNB5E7aF2PGHup9rQpBMe7uzF30u-nzrzHo-UrGkzBmNxI_eiwoiGu1TpFHCMPKsBg

=== Cleanup ===
✓ Deleted sessions and collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "1ZENvSUSUHlNoqJhHS4wsY96QK_hUfpWHesCi9bAP_8sySDJI_2G3VH7id5O67pbmt2o5jYd0RSeBfM6vSh4Xw"

=== List Collections ===
Total collections: 13
Sample collections: ['chat_goals__ek0_testing', 'chat_messages__ek0_testing', 'schedules__ek0_testing', 'chat_raw_completions__ek0_testing', 'client_collection_management_python']

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
✓ Created record with plain dict: {'id': 'O_g2mTPjxrdNJBA2uuTqkPuqJ4EdSXUXXuUYPcI52wfLx-AiLfZ8VO7YztRZ5Pqb3Tl6Xb3G_mNZrB180BjwlA'}

=== Upsert Operation ===
✓ Upsert (update existing record): O_g2mTPjxrdNJBA2uuTqkPuqJ4EdSXUXXuUYPcI52wfLx-AiLfZ8VO7YztRZ5Pqb3Tl6Xb3G_mNZrB180BjwlA
✓ Inserted second record: 18DzZnuIAJpLPwkfCFJNEB6xjRnuyE2sm8Wht5grZ1mIoCfpSSht_bm9yor4cCdxWtLZb9-zTfgnGB9cdLA7qg
✓ Upsert (update second record): 18DzZnuIAJpLPwkfCFJNEB6xjRnuyE2sm8Wht5grZ1mIoCfpSSht_bm9yor4cCdxWtLZb9-zTfgnGB9cdLA7qg

=== Find One Operation ===
✓ Found user by email: {'email': {'value': 'alice.j@newdomain.com', 'type': 'String'}, 'name': {'value': 'Alice Johnson', 'type': 'String'}, 'active': {'type': 'Boolean', 'value': True}, 'age': {'type': 'Integer', 'value': 29}, 'id': 'O_g2mTPjxrdNJBA2uuTqkPuqJ4EdSXUXXuUYPcI52wfLx-AiLfZ8VO7YztRZ5Pqb3Tl6Xb3G_mNZrB180BjwlA'}
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
✓ Inserted document: 9k0W1oypJSKvimoTbFn9qcYTeEwPH4VnrAVjDO6JoDQjLV7MI95nWSJPOYsWNJwPL2Np0znbfxd8HFl_OPmY-g

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: xED3Dr2XasFqxgg-MrbI6XIcLyHEsNTmiRkS1rw_ZSr7Douk9ehfFJ9z0DoApyEWSR6q2JXOezn1AR_5gQKIbA

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
✓ Edge cache script created: ZvGtCWxq2c6BdkUp7Nf53dUgDvtSZX7NmJCM197VJktaxSysvUFnXNCsUMY_MycR01k_89j5Zowqz5l9KdWy7w

Call 1: Cache miss (fetches from API)
Response time: 1278ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "current": {
            "interval": 900,
            "temperature_2m": 11.4,
            "time": "2026-10-17T08:30"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "\u00b0C",
            "time": "iso8601"
          },
          "elevation": 32.0,
          "generationtime_ms": 9.208202362060549,
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
Response time: 10ms (122.5x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "current": {
            "interval": 900,
            "temperature_2m": 11.4,
            "time": "2026-10-17T08:30"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "\u00b0C",
            "time": "iso8601"
          },
          "elevation": 32.0,
          "generationtime_ms": 9.208202362060549,
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
   ⏱️  Duration: 72.2ms
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
   ⏱️  Duration: 1.9ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "type": "Object",
            "value": {
                  "address": {
                        "city": "Gwenborough",
                        "geo": {
                 ...
   🚀 Cache speedup: 37.8x faster!

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

✅ Function saved: elnBV6UGi-JmgPv3SGv7t6SKwV3EGOBm7fTKFYLd65tjHa2lLNJrljMTqLYdKtlVZjAVcY0UjQU4f8sgFzbM9Q
📊 Found 5 active users

📝 Example 2: Parameterized Function

✅ Function saved: k3JiWydENs49C7PRlJ7WC3ssRryyRLObSt7HPy6vonTG43ntIs5YfxLjhqWLSvOSCNPk0ovYkVqvYBrzKilq6Q
📊 Found 3 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: l27gxobpHWr6uy_RI4n7JP5XdJNInysUGAYctbiPi5qTFf7VJT55_ESu2OQbzCQNmASLR2bsI8D0TKFKDIy2NQ
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
   {'avg_price': {'type': 'Float', 'value': 365.6666666666667}, 'category': {'type': 'String', 'value': 'Furniture'}, 'count': {'type': 'Integer', 'value': 3}}
   {'avg_price': {'type': 'Float', 'value': 367.0}, 'category': {'type': 'String', 'value': 'Electronics'}, 'count': {'type': 'Integer', 'value': 5}}
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
   Vector databases offer several benefits:

1. **Efficient Similarity Search**: They excel in nearest neighbor searches, making them ideal for applications like recommendation systems and image retrieval.

2. **High Dimensional Handling**: They manage high-dimensional data effectively, which is crucial for processing embeddings from machine learning models.

3. **Scalability**: Many vector databases are designed to scale horizontally, accommodating large datasets seamlessly.

4. **Performance**: They provide fast query performance through optimized indexing techniques, such as HNSW (Hierarchical Navigable Small World) or IVF (Inverted File).

5. **Flexibility**: Vector databases can store and process various data types, including text, images, and audio, allowing for multi-modal applications.

6. **Integration with ML Models**: They easily integrate with machine learning pipelines for real-time inference and data retrieval.

7. **Advanced Querying**: Support for complex queries, such as filtering and combining vector similarities with traditional conditions.

8. **Community and Tooling**: A growing ecosystem of tools and libraries supports development and integration, enhancing usability.

Overall, vector databases are tailored for applications that rely on the representation of data as vectors, particularly in AI and machine learning contexts.
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

✅ Function saved: AHsS_okd-xYqRWn4wXBobOVhQP8SuVHOPBcPrSguBxTLOrBaSEvZMDjLjGmpnYEfWRt7xbPrPchgn3HU677CKw
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
   active: 7 users
   inactive: 3 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Function saved
📊 Average score by role:
   {'avg_score': {'type': 'Float', 'value': 20.0}, 'count': {'type': 'Integer', 'value': 3}, 'role': {'type': 'String', 'value': 'admin'}}
   {'avg_score': {'type': 'Float', 'value': 70.0}, 'count': {'type': 'Integer', 'value': 7}, 'role': {'type': 'String', 'value': 'user'}}
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

✅ Inserted order: ODk2Cs6imHAKLtxXUib-EzIjvU-GyFEkRNs3MCqVD4Bax9DuOfVLH52u2DlIRsRRzk7hyRRyjorbEGMcIE6vdQ
✅ Inserted 2 products with wrapped types

📝 Example 2: Querying and Extracting Wrapped Types

📊 Found 2 products
   • Laptop Pro
   • Wireless Mouse

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

✅ Inserted order: BkSKZ_CyoNtGu8IWKBNgfZM9J6JEvXzoBDrDj8cKlZdF4VIVe1CciVqdmG9FWq9MqOojS_UCy9cJ7YZ_aac9dw
✅ Cached order status
📊 Quick status lookup: {'value': '{"type":"Object","value":{"status":"processing","updated_at":"2026-10-17T08:30:14.491698+00:00"}}'}

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
   2. Database Design Principles (Database)
   3. Vector Databases Explained (Database)
   4. Natural Language Processing (AI)
   5. Introduction to Machine Learning (AI)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {'category': {'type': 'String', 'value': 'AI'}, 'count': {'type': 'Integer', 'value': 2}}
   {'category': {'type': 'String', 'value': 'Database'}, 'count': {'type': 'Integer', 'value': 3}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Goal Template CRUD Example (Python) ===

--- Creating goal template ---
Created template: Data Migration (id: YJIbnspxTbGRec_hMUZfBHbLyN1oivJBlYm50NIox8_By3wIPJfJ-osZa_vS-vpC0RhNFjHWWCXVFfaJUkc-YQ)

--- Listing templates ---
Templates: {'count': 1, 'items': [{'description': {'type': 'String', 'value': 'Template for migrating data between schemas'}, 'id': 'YJIbnspxTbGRec_hMUZfBHbLyN1oivJBlYm50NIox8_By3wIPJfJ-osZa_vS-vpC0RhNFjHWWCXVFfaJUkc-YQ', 'steps': {'type': 'Array', 'value': [{'description': 'Analyze source schema'}, {'description': 'Create target schema'}, {'description': 'Migrate records'}, {'description': 'Validate results'}]}, 'title': {'type': 'String', 'value': 'Data Migration'}}]}

--- Getting template ---
Fetched: {'type': 'String', 'value': 'Data Migration'}

--- Updating template ---
Updated description: {'type': 'String', 'value': 'Updated: comprehensive data migration workflow'}

--- Deleting template ---
Template deleted successfully

✓ Goal template CRUD example completed
=== ekoDB Goals / Tasks / Agents Integration Example (Python) ===

--- goal_create ---
Created goal: Deploy v3 (id: ajN7etekCJ-twEcBwHlcjJIgv7VL_MIM1PG22QrM5OC5J7oym4iGsbZ9FhOuz0bRKRJ5reoHGSJR3x5I4feTsg)

--- goal_list ---
Goals: {'count': 1, 'goals': [{'created_at': '2026-10-17T08:30:14.855009+00:00', 'description': 'Deploy version 3 to production', 'id': 'ajN7etekCJ-twEcBwHlcjJIgv7VL_MIM1PG22QrM5OC5J7oym4iGsbZ9FhOuz0bRKRJ5reoHGSJR3x5I4feTsg', 'status': 'pending', 'steps': '[{"description":"Build Docker image","status":"pending"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]', 'title': 'Deploy v3', 'updated_at': '2026-10-17T08:30:14.855009+00:00'}]}

--- goal_get ---
Fetched: {'type': 'String', 'value': 'Deploy v3'}

--- goal_update ---
Updated description: {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}

--- goal_search ---
Search results: {'count': 1, 'items': [{'_score': 13.2, 'created_at': {'type': 'DateTime', 'value': '2026-10-17T08:30:14.855009+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'ajN7etekCJ-twEcBwHlcjJIgv7VL_MIM1PG22QrM5OC5J7oym4iGsbZ9FhOuz0bRKRJ5reoHGSJR3x5I4feTsg', 'status': {'type': 'String', 'value': 'pending'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","status":"pending"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-10-17T08:30:14.864739+00:00'}}]}

--- goal_step_start (step 0) ---
Step 0 started: {'created_at': {'type': 'DateTime', 'value': '2026-10-17T08:30:14.855009+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'ajN7etekCJ-twEcBwHlcjJIgv7VL_MIM1PG22QrM5OC5J7oym4iGsbZ9FhOuz0bRKRJ5reoHGSJR3x5I4feTsg', 'status': {'type': 'String', 'value': 'in_progress'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","status":"InProgress"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-10-17T08:30:14.873106+00:00'}}

--- goal_step_complete (step 0) ---
Step 0 completed: {'created_at': {'type': 'DateTime', 'value': '2026-10-17T08:30:14.855009+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'ajN7etekCJ-twEcBwHlcjJIgv7VL_MIM1PG22QrM5OC5J7oym4iGsbZ9FhOuz0bRKRJ5reoHGSJR3x5I4feTsg', 'status': {'type': 'String', 'value': 'in_progress'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","result":"Docker image built: sha256:abc123","status":"Completed"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-10-17T08:30:14.879429+00:00'}}

--- goal_step_fail (step 1) ---
Step 1 failed: {'created_at': {'type': 'DateTime', 'value': '2026-10-17T08:30:14.855009+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'ajN7etekCJ-twEcBwHlcjJIgv7VL_MIM1PG22QrM5OC5J7oym4iGsbZ9FhOuz0bRKRJ5reoHGSJR3x5I4feTsg', 'status': {'type': 'String', 'value': 'in_progress'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","result":"Docker image built: sha256:abc123","status":"Completed"},{"description":"Run migrations","error":"Migration failed: column already exists","status":"Failed"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-10-17T08:30:14.885732+00:00'}}

--- goal_complete ---
Goal completed: {'type': 'String', 'value': 'pending_review'}

--- goal_approve ---
Goal approved: {'type': 'String', 'value': 'in_progress'}

--- goal_reject ---
Goal rejected: {'type': 'String', 'value': 'failed'}

--- task_create ---
Created task: Nightly Backup (id: vEB9o-_wKYIhw37lthEaVsOfkLOiLyXN-QLtlS0t523h_qsvuq5UD72cCD15TG1wEWQqk8ebDxFX0On4SVK92Q)

--- task_list ---
Tasks: {'count': 1, 'items': [{'cron': {'type': 'String', 'value': '0 2 * * *'}, 'description': {'type': 'String', 'value': 'Backup all databases nightly'}, 'id': 'vEB9o-_wKYIhw37lthEaVsOfkLOiLyXN-QLtlS0t523h_qsvuq5UD72cCD15TG1wEWQqk8ebDxFX0On4SVK92Q', 'max_consecutive_failures': {'type': 'Integer', 'value': 3}, 'name': {'type': 'String', 'value': 'Nightly Backup'}}]}

--- task_get ---
Fetched: {'type': 'String', 'value': 'Nightly Backup'}

--- task_start ---
Task started: {'type': 'String', 'value': 'running'}

--- task_succeed ---
Task succeeded: {'consecutive_failures': {'type': 'Integer', 'value': 0}, 'cron': {'type': 'String', 'value': '0 2 * * *'}, 'description': {'type': 'String', 'value': 'Backup all databases nightly'}, 'id': 'vEB9o-_wKYIhw37lthEaVsOfkLOiLyXN-QLtlS0t523h_qsvuq5UD72cCD15TG1wEWQqk8ebDxFX0On4SVK92Q', 'last_error': {'type': 'Null', 'value': None}, 'last_run': {'type': 'DateTime', 'value': '2026-10-17T08:30:14.947843+00:00'}, 'max_consecutive_failures': {'type': 'Integer', 'value': 3}, 'name': {'type': 'String', 'value': 'Nightly Backup'}, 'run_count': {'type': 'Integer', 'value': 1}, 'status': {'type': 'String', 'value': 'active'}, 'updated_at': {'type': 'DateTime', 'value': '2026-10-17T08:30:14.947843+00:00'}}

--- task_pause ---
Task paused: {'type': 'String', 'value': 'paused'}

--- task_resume ---
Task resumed: {'type': 'String', 'value': 'active'}

--- task_fail ---
Task failed: {'consecutive_failures': {'type': 'Integer', 'value': 1}, 'cron': {'type': 'String', 'value': '0 2 * * *'}, 'description': {'type': 'String', 'value': 'Backup all databases nightly'}, 'id': 'vEB9o-_wKYIhw37lthEaVsOfkLOiLyXN-QLtlS0t523h_qsvuq5UD72cCD15TG1wEWQqk8ebDxFX0On4SVK92Q', 'last_error': {'type': 'String', 'value': 'Disk full'}, 'last_run': {'type': 'DateTime', 'value': '2026-10-17T08:30:14.979490+00:00'}, 'max_consecutive_failures': {'type': 'Integer', 'value': 3}, 'name': {'type': 'String', 'value': 'Nightly Backup'}, 'next_run': {'type': 'DateTime', 'value': '2026-03-22T02:00:00+00:00'}, 'run_count': {'type': 'Integer', 'value': 1}, 'status': {'type': 'String', 'value': 'active'}, 'updated_at': {'type': 'DateTime', 'value': '2026-10-17T08:30:14.979490+00:00'}}

--- task_due ---
Due tasks: {'count': 0, 'items': []}

--- task_delete ---
Task deleted

--- agent_create ---
Created agent: CodeReviewer (id: wke-rqajOTZLD79AI0G4gdy5Ak6ptnV1kGVbkFc_tjXksXkVaFLaPPMqj_3fmq1govqGo87REwXDmFUBJqSN0g)

--- agent_list ---
Agents: {'count': 1, 'items': [{'deployment_id': {'type': 'String', 'value': 'deploy_test'}, 'id': 'wke-rqajOTZLD79AI0G4gdy5Ak6ptnV1kGVbkFc_tjXksXkVaFLaPPMqj_3fmq1govqGo87REwXDmFUBJqSN0g', 'llm_model': {'type': 'String', 'value': 'gpt-4o'}, 'name': {'type': 'String', 'value': 'CodeReviewer'}, 'system_prompt': {'type': 'String', 'value': 'You review code for correctness and style.'}, 'tools': {'type': 'Array', 'value': ['web_search', 'file_read']}}]}

--- agent_get ---
Fetched: {'type': 'String', 'value': 'CodeReviewer'}

--- agent_get_by_name ---
By name: {'type': 'String', 'value': 'CodeReviewer'}

--- agent_update ---
Updated agent prompt: {'type': 'String', 'value': 'You review code. Be concise.'}

--- agents_by_deployment ---
Agents in deploy_test: {'count': 0, 'items': []}
WARNING: agents_by_deployment omitted created agent wke-rqajOTZLD79AI0G4gdy5Ak6ptnV1kGVbkFc_tjXksXkVaFLaPPMqj_3fmq1govqGo87REwXDmFUBJqSN0g; TODO: check/fix the server-side deployment lookup

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
Inserted document: oA5XGhnc2mhFi1dOPaYHqdBbGxWU827yKZVT9om9-WRyFHA2LxUbekg3t5VlWNj02-Ouvsdt0fZiQQMPPc1qow

--- kv_link ---
Linked: None

--- kv_get_links ---
Links for app:config: [{'collection': 'kv_links_example_py', 'created_at': '2026-10-17T08:30:15.501757Z', 'document_id': 'oA5XGhnc2mhFi1dOPaYHqdBbGxWU827yKZVT9om9-WRyFHA2LxUbekg3t5VlWNj02-Ouvsdt0fZiQQMPPc1qow', 'field_path': None, 'last_accessed': '2026-10-17T08:30:15.502404Z', 'metadata': {}}]

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
  kv_operations:py:cache:product:1: {'name': 'Product 1', 'price': 29.99}
  kv_operations:py:cache:product:2: {'price': 39.99, 'name': 'Product 2'}
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
Response: The three primary colors are red, blue, and yellow.

--- Structured Output via SSE ---
JSON response: [
  {
    "name": "Earth",
    "diameter_km": 12742
  },
  {
    "name": "Mars",
    "diameter_km": 6779
  },
  {
    "name": "Jupiter",
    "diameter_km": 139820
  }
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello! I hope you're having a wonderful day.

=== Done ===
=== ekoDB Schedule Management Example (Python) ===

--- create_schedule ---
Created schedule: Nightly Report (id: b8499574-d731-4177-bf46-cfbf58d7cc8a)

--- list_schedules ---
Schedules: {'count': 1, 'schedules': [{'created_at': '2026-10-17T08:30:18.336378Z', 'cron_expression': '0 0 0 * * *', 'description': 'Generate and email nightly analytics report', 'enabled': True, 'function_label': 'schedule_noop_python_84987', 'id': 'b8499574-d731-4177-bf46-cfbf58d7cc8a', 'last_execution': None, 'name': 'Nightly Report', 'next_execution': '2026-10-18T00:00:00Z', 'parameters': {}, 'stats': {'avg_execution_time_ms': 0.0, 'failed_executions': 0, 'last_error': None, 'successful_executions': 0, 'total_executions': 0}, 'timezone': 'UTC', 'updated_at': '2026-10-17T08:30:18.336378Z'}]}

--- get_schedule ---
Fetched: Nightly Report cron=0 0 0 * * *

--- update_schedule ---
Updated cron: 0 30 1 * * *

--- trigger_schedule ---
Triggered: {'schedule_id': 'b8499574-d731-4177-bf46-cfbf58d7cc8a', 'status': 'triggered'}

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
  1. Score: 12.870, Matched: name, email
  2. Score: 6.270, Matched: name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 13.200, Matched: bio, title
  2. Score: 13.200, Matched: bio, title
  3. Score: 13.200, Matched: title, bio
  4. Score: 13.200, Matched: title, bio

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
Found 2 results (matches: run, running, runs)
  1. Score: 6.600, Matched: bio
  2. Score: 6.600, Matched: bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.781
  2. Score: 0.771
  3. Score: 0.763

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.391, Matched: content, title
  2. Score: 0.886, Matched: title, content
  3. Score: 0.382, Matched:

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 6.600, Matched: title

9. Vector search with a metadata pre-filter (category = ml):
Found 2 documents in category "ml" (NLP excluded)
  1. Introduction to Machine Learning (category: ml)
  2. Deep Learning Fundamentals (category: ml)

Paired vector search: document=Rust Programming Answer, query=Rust Programming Question
=== Cleanup ===
✅ Deleted test collections

✅ Search examples completed!
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': 'FYOBaqxDqCbqS4oDRr_op7gKOCbiaik2aGEPE67ffhkzWWdtuxrrDba6815N7ZCMkFybdkD7ZiyU06fqiG9a5Q'}

=== Find by ID ===
Found: {'id': 'FYOBaqxDqCbqS4oDRr_op7gKOCbiaik2aGEPE67ffhkzWWdtuxrrDba6815N7ZCMkFybdkD7ZiyU06fqiG9a5Q', 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'price': {'type': 'Float', 'value': 99.99}, 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'value': {'value': 42, 'type': 'Integer'}, 'name': {'type': 'String', 'value': 'Test Record'}, 'metadata': {'value': {'nested': {'deep': True}, 'key': 'value'}, 'type': 'Object'}, 'active': {'type': 'Boolean', 'value': True}, 'created_at': {'type': 'String', 'value': '2026-10-17T04:30:18.883863'}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-10-17 04:30:18.883863
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'nested': {'deep': True}, 'key': 'value'}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'id': 'FYOBaqxDqCbqS4oDRr_op7gKOCbiaik2aGEPE67ffhkzWWdtuxrrDba6815N7ZCMkFybdkD7ZiyU06fqiG9a5Q', 'tags': ['tag1', 'tag2', 'tag3'], 'price': 99.99, 'categories': ['electronics', 'computers'], 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'value': 42, 'name': 'Test Record', 'metadata': {'nested': {'deep': True}, 'key': 'value'}, 'active': True, 'created_at': '2026-10-17T04:30:18.883863', 'data': 'aGVsbG8gd29ybGQ=', 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5]}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'id': 'FYOBaqxDqCbqS4oDRr_op7gKOCbiaik2aGEPE67ffhkzWWdtuxrrDba6815N7ZCMkFybdkD7ZiyU06fqiG9a5Q', 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'metadata': {'type': 'Object', 'value': {'nested': {'deep': True}, 'key': 'value'}}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'price': {'value': 99.99, 'type': 'Float'}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'name': {'value': 'Updated Record', 'type': 'String'}, 'active': {'type': 'Boolean', 'value': True}, 'value': {'type': 'Integer', 'value': 100}, 'created_at': {'type': 'String', 'value': '2026-10-17T04:30:18.883863'}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: Hvr4MdqEghPfmWAhZA7I27t412cr98bGUt63K1CX4ywpXTo5GzhakJVut_JRzgKFHwi4iplfQOv9zBOCPhGiWQ

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
✓ Created native SWR script: github_user_native_py (FAqzaqWdul8s5ly9t4iSS5bQf9amtKnbKOcd3b9joQYDPSw8GhiL6_w9DWhqbfW2GPb-txhCXqvtVAbnCfq9oA)

First call (cache miss - will fetch from GitHub API):
  Response time: 131ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 2ms
  Speedup: 65.0x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_py (LNzR6lHoxflU3Rc94GKWD_jjCMu5fSpiiObHxxwzC-dyKptRyPivxkKF0Sy3J_y4rSmXC7NlwpUeNRt8VXMMTQ)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_py' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_py (PXHI5rioI-98b9NUZpVziQD65v-LDwWG6FTHksxuFhfxPDCX870ru5auIgSClFdQQnZc7OtStt2eBfcmIEmDJw)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_py' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_py (Sspn0WpD8Ch-2-qaZpBoI4Mc39INUnkethFaY_y5AIR6F8qhaW5zniU09vYQXYl-x1YOiyz5QwIffr8pHrX30A)
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
✓ Created SWR script: fetch_api_user_py (7FAGFJjtCLmqukRk0v3Kh3AmdOjllBx_DsurCnNimEN8VRJF0WkAAPqgeYD0-rZZmAutXV11Oba8LQbOU7eFHw)

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
Created Alice: $1000 - ID: 4Nq2WzM3cME9P2ZSgZpoW4oS2yUW22OOtqGR14_vvi9h31sF2HHtqbnr0kJVCMVYJGD2RMwsCxwAXoMNpcrEtg
Created Bob: $500 - ID: FklGwV9BU8cy7RS3EHaCr-ATGgr9Te-S4l36yALrXULIaW4Mp3criY2ZR8lsRIQvS6OWKudJTgR_A_N84CQApQ

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): da32f9b1-3eb5-4adb-a1f2-2745fc7949cb

=== Example 2: Operations within Transaction ===
Alice in transaction: {'name': {'type': 'String', 'value': 'Alice'}, 'id': '4Nq2WzM3cME9P2ZSgZpoW4oS2yUW22OOtqGR14_vvi9h31sF2HHtqbnr0kJVCMVYJGD2RMwsCxwAXoMNpcrEtg', 'balance': {'value': 800, 'type': 'Integer'}, 'account_id': {'value': 'ACC001', 'type': 'String'}}
Bob in transaction: {'balance': {'value': 700, 'type': 'Integer'}, 'id': 'FklGwV9BU8cy7RS3EHaCr-ATGgr9Te-S4l36yALrXULIaW4Mp3criY2ZR8lsRIQvS6OWKudJTgR_A_N84CQApQ', 'name': {'value': 'Bob', 'type': 'String'}, 'account_id': {'value': 'ACC002', 'type': 'String'}}
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

✓ Verified committed balances: Alice=$800, Bob=$700

=== Example 5: Rollback Demo ===
New transaction: 4736e78f-86ee-42ac-bf02-666153b8549d
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Create User Function ===
Created user function with ID: SPG8sUVVzYNSlSdtXl4rz-zLsfJuhtJ5lM4QnmBDBd9CiLDqP3lAxNflOBDZx9wlOTxoQs5j3vRVzsYwRdU4hQ

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

Created chat session: MjvfFWbX5Kowdyk-tQGbsprzqfOLm5BOW91efO6oiNgv6PbgPzAmVOnBdOuK7HapSLeBVau3IXsPy7ggJbi5-w

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: UvcPUy8wdCKh_HbWN-qiF7EDYzPPZCAX-4g_AMkMAwcvuhur5htHo0qbrC9q_MX2PI0HjI5yUG6qBkA6GHGU9w
Execution time: 530ms
Token usage: {'completion_tokens': 8, 'prompt_tokens': 15, 'total_tokens': 23}

Full response: {"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}The capital of France is Paris....
=== WebSocket Subscription Example (Python) ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_py' ===
✓ Subscribed (subscription_id: sub_c9f0e9818eb14c2895a8190c191214d8)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: psguO-f9UVCCHW0AaOePR38noxiAK6U_oYU5v5cp1NLqdABoHDeMkkjG7fr9VPeOS-145VdljUdYFOCzQTLszQ

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_py
     Record IDs: psguO-f9UVCCHW0AaOePR38noxiAK6U_oYU5v5cp1NLqdABoHDeMkkjG7fr9VPeOS-145VdljUdYFOCzQTLszQ
     Timestamp:  2026-10-17T08:30:21.408432+00:00

Inserting record 2...
✓ Inserted: 1pr7flEonrtikCaYjgZe6WJTb-aN1ZVbc8P0mlsX7KHgdsXrBpohC5vnIXBmdeOJhfcqSsT_XfAl1joIHhAnjQ

  📡 Notification received:
     Event:      insert
     Record IDs: 1pr7flEonrtikCaYjgZe6WJTb-aN1ZVbc8P0mlsX7KHgdsXrBpohC5vnIXBmdeOJhfcqSsT_XfAl1joIHhAnjQ

=== Unsubscribing ===
✓ Unsubscribed: {'collection': 'ws_subscribe_example_py', 'found': True, 'unsubscribed': True}

=== Cleanup ===
✓ Deleted collection 'ws_subscribe_example_py'

✓ WebSocket subscription example completed successfully
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: 9mcEAqtypN06ekeuVmpKANBGVrPrTk63-mPY9G7rRhKFXx3xgDclor-KBLGG11NoHq1Ai2GZy_P-TNu9Ozt_Cg

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
   Inserted with ripple: {'id': 'E14xf4fkkiChWitN6rt0rJcrIH4d5lEnVshKwDM_2PC0vBOkFNBWhJsg8gveO4vn1zoLLK-SuAfrKG5lFQlD3A'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'sHNfEZ-LgPeHYiW9vOx6jKPHqY3ksuD3mWiHs0ycD-gTIs2FNKSpJ1l8XpoH3i3AdyeNsdtYoHQOmUcBPKXLKg'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'id': 'E14xf4fkkiChWitN6rt0rJcrIH4d5lEnVshKwDM_2PC0vBOkFNBWhJsg8gveO4vn1zoLLK-SuAfrKG5lFQlD3A', 'name': {'value': 'Product 1', 'type': 'String'}, 'price': {'value': 150, 'type': 'Integer'}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'id': 'E14xf4fkkiChWitN6rt0rJcrIH4d5lEnVshKwDM_2PC0vBOkFNBWhJsg8gveO4vn1zoLLK-SuAfrKG5lFQlD3A', 'price': {'type': 'Integer', 'value': 500}, 'name': {'value': 'Upsert Product', 'type': 'String'}}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ['name', 'id', 'email']
  First user: {'type': 'String', 'value': 'Alice Johnson'} <{'value': 'alice@example.com', 'type': 'String'}>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ['avatar_url', 'user_role', 'status', 'created_at', 'age', 'bio', 'id', 'name', 'email']

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - {'value': 'Dave Brown', 'type': 'String'} (age {'type': 'Integer', 'value': 45})
    - {'value': 'Alice Johnson', 'type': 'String'} (age {'type': 'Integer', 'value': 30})
    - {'type': 'String', 'value': 'Bob Smith'} (age {'type': 'Integer', 'value': 25})

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - {'value': 'Carol White', 'type': 'String'}: {'type': 'String', 'value': 'Manager'}

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ['secret_token', 'password', 'bio', 'id', 'api_key', 'email', 'created_at', 'age', 'status', 'user_role', 'name', 'avatar_url']
  Projected query:
    - 3 fields per record
    - Fields: ['name', 'id', 'email']
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
Inserted: map[id:QiM74F03jcm5g7W8N_c97lIziV-0dJSDGpuDjulc5T_YQXuILHE6JcVLX8JM-LWslS-VHBKugMoqAa6oNcvZFA]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:QiM74F03jcm5g7W8N_c97lIziV-0dJSDGpuDjulc5T_YQXuILHE6JcVLX8JM-LWslS-VHBKugMoqAa6oNcvZFA name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found documents: [map[active:map[type:Boolean value:true] id:QiM74F03jcm5g7W8N_c97lIziV-0dJSDGpuDjulc5T_YQXuILHE6JcVLX8JM-LWslS-VHBKugMoqAa6oNcvZFA name:map[type:String value:Test Record] value:map[type:Integer value:42]]]

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:QiM74F03jcm5g7W8N_c97lIziV-0dJSDGpuDjulc5T_YQXuILHE6JcVLX8JM-LWslS-VHBKugMoqAa6oNcvZFA name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ simple_crud.go completed successfully

=== Running simple_websocket.go ===
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: 23WSSAaznTYCABR4MQPRX6LMkUJPic1P5rH1jsS7c7VyhgbIkVDDqUYZ_OJIEiEUNP61gXydRTpV-wibOhqlNA

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1792225826244340000",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "23WSSAaznTYCABR4MQPRX6LMkUJPic1P5rH1jsS7c7VyhgbIkVDDqUYZ_OJIEiEUNP61gXydRTpV-wibOhqlNA",
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

✅ Function saved: iPuuUqfyNm6OpgbqhTwzq1kAozzkqR3EkpAcq2nPCZhQrb6PBcGALoAkSBjxeQlS9_oLtbw25HjDmTs3yeff4Q
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: LLp7NPSs2FcN8a1hyk7sJoF16wI4fY3TYh538ypJpnqV1Pr4ohR_Rq_LYz9hLxdzcF-vbSzJ20C_ZIt-4u9LGQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: G8gs5PwH3rWjwuuvydwFI-AB7nuevlQcvi_6WvfanDRtKnfBEWvRRH90MINDBcfU9EkjMnosnoAO9dOKhAk24g
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":{"type":"Float","value":60},"count":{"type":"Integer","value":5},"max_score":{"type":"Integer","value":100},"status":{"type":"String","value":"active"}}
   {"avg_score":{"type":"Float","value":50},"count":{"type":"Integer","value":5},"max_score":{"type":"Integer","value":90},"status":{"type":"String","value":"inactive"}}

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
Collection created with first record: 6lJkgQJ1Y0MHs9jcnM9Raa_1MIYJz6lDAGODXgKf3qOexZYuxT8ftVq1_2Rzl5l_q95Pt9rIZRwQt-Wn55XuEA

=== List Collections ===
Total collections: 13
Sample collections: [chat_goals__ek0_testing chat_messages__ek0_testing schedules__ek0_testing chat_raw_completions__ek0_testing chat_agent_configs__ek0_testing]

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ collection_management.go completed successfully

=== Running transactions.go ===
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: q_Lwhp09Qsy0QmYIkkri3dHAwz_c1odZm09XOFzza0BlP8a6Do5Wml8VIJnlhBmvf33LWJkobc85QS49sQ3CsQ
Created Bob: $500 - ID: PMkRF6IbCGnHnWd4tA23Z0GtJpW-4BroUE3iy1k4gDnKU-_WGzOkdGEG_sGcQtu0-eOQ01YZ_lvLnTyRJ8Q59Q

=== Example 1: Begin Transaction ===
Transaction ID: 6f54edcc-8d89-4f56-8744-ccce57f51db2

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
New transaction: 18345085-fa4f-46a6-ad83-69b80d71af73
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
   ✅ Function saved: TGu4O6LEgycCz-M4qrXMzDR5feI13e-59qQjyfu29kPZSLVNbrEj3gW09yc60tZDSnDxx7s7WoGPj_1ItH7LQQ

2️⃣ Calling function (Insert + Verify)...
   ✅ function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: 8dLKM7Wewqknzq7XygKAdil9QLSeEQAZqcXNIqUTiAevm7qcIXBOCzW_YxlvIiS_mopv7wY7_TtWT0r1UQjIHg
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: hhk5GfIzJSHN9IsE4pBpDzeAsP6SXqnLNh3dStcb7eYbnVallJT5x12LGl0EApWyuXDQCKmj9yezSiaPHndqHg

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
   ✅ Function saved: TDikveK7do0TEhgYU1YJHqvVywbIe0fXL_WcTdJAaPjmvwm3wXkRiVa4mom4A8djFFHq1ZE2qbZLmbFtbRkung

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
   ✅ Function saved: GGDkIvfmQr6cgEnKKfshXTNgdMTjNSJXuoru59dMBt4iwfqXpGXZCRuts5xlOZTohw7Vh0_wViJtcK0hG5t9eg

2️⃣ Calling function (Query + Delete + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: TGu4O6LEgycCz-M4qrXM...
   ✅ Deleted script: hhk5GfIzJSHN9IsE4pBp...
   ✅ Deleted script: TDikveK7do0TEhgYU1YJ...
   ✅ Deleted script: GGDkIvfmQr6cgEnKKfsh...
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
  Output: Document ID = uVIrGZKN6HJ74C8Gt74Z52M03JXDSwjoKr1m4rGd-uQStaFacXTbTecc-DVHfe32ySBWHbpKTJgp4tZExGxWoQ
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: FindByID(uVIrGZKN6HJ74C8Gt74Z52M03JXDSwjoKr1m4rGd-uQStaFacXTbTecc-DVHfe32ySBWHbpKTJgp4tZExGxWoQ)
  Output: Found document with name = map[type:String value:TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: FindByID(uVIrGZKN6HJ74C8Gt74Z52M03JXDSwjoKr1m4rGd-uQStaFacXTbTecc-DVHfe32ySBWHbpKTJgp4tZExGxWoQ)
  Output: Error (expected) - request failed with status 404: ��error�Record has been deleted
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
  Output: Document ID = UQ-EGvRfAz6URuvy4_XHdK_4ajioy8e7LyGn3komrFF2wtfovqaP5dVhEw_ThHPKOunupOxFdYT-dJXT0-ZHRg
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: FindByID(UQ-EGvRfAz6URuvy4_XHdK_4ajioy8e7LyGn3komrFF2wtfovqaP5dVhEw_ThHPKOunupOxFdYT-dJXT0-ZHRg)
  Output: Found document with name = map[type:String value:WS TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: FindByID(UQ-EGvRfAz6URuvy4_XHdK_4ajioy8e7LyGn3komrFF2wtfovqaP5dVhEw_ThHPKOunupOxFdYT-dJXT0-ZHRg)
  Output: Error (expected) - request failed with status 404: ��error�Record has been deleted
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
=== ekoDB Advanced CRUD Integration Example (Go) ===

--- Inserting base record ---
Inserted record: DKCri98z-JiDpd0g0bzRTbKXwthmuYzNgiTzX3PUvxB7M5MEdTwZEGbZNYYL2opgZaJqawKgGwztO8xhtuwN0w

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
✓ Created session: Yk3mOBt6KhajZ4i7EZblaqirarf5Tn4d2zeSJ4jpqyKfUvgeURCuljFfoO7SoC_AJgp1hkMQzm0gd04opwGj1g

=== Sending Initial Message ===
✓ Message sent
  Response: It appears that there are currently no products marked as "available" in the database. However, I can tell you about the available product we have:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you need more details about this or if there's anything specific you're looking for, let me know!

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
✓ Created second session: JEFfuFX_LPUgAUCDJEmA8Kz4I6-BOCi8FkpEzyJaGaAtvD4F3zuO4wg8W1RFqesm36DzHanBEO87s8rVpfT_aQ
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: JEFfuFX_LPUgAUCDJEmA8Kz4I6-BOCi8FkpEzyJaGaAtvD4F3zuO4wg8W1RFqesm36DzHanBEO87s8rVpfT_aQ
✓ Deleted chat session: Yk3mOBt6KhajZ4i7EZblaqirarf5Tn4d2zeSJ4jpqyKfUvgeURCuljFfoO7SoC_AJgp1hkMQzm0gd04opwGj1g
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: iYvOuzUm7CAg9RA_SuiaXz3MbA_5OtNQn7RcKOfIJoxRKtolbwvxZjYxi3ydgAL_inj4sBHZ7rWN3NezTWqr_g

=== Sending Chat Message ===
Message ID: H2uYrBXkz561dz2fPXNT_TEsUs17xPnsl7no-Wxud7zddznVJ-2nMsa6SYv_IMtyttJKRsavRbNFcZ2czMz6OQ

=== AI Response ===
The available products and their prices are as follows:

1. **ekoDB Cloud**: $499
   - Description: Fully managed cloud database service product

2. **ekoDB Pro**: $299
   - Description: Enterprise edition product with advanced features

3. **ekoDB**: $99
   - Description: A high-performance database product with AI capabilities

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:Dnd7jyhy8H3kP03UI0ilGjekWKlc1JcM-oDQ5VHYF_y6RKpu2FqGqwfXphTArhT8WR_Q1Tb6qKw_04MHBh05Vg name:ekoDB Cloud price:499] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:EfsH7gAW39fZgRKZRUi58h8vXQlXNdS1vk7-D56v_6czPRks4OMqxXLmZ_F9JdVfk_6qGkLJD7eG-vRwjnKd2A name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:VrlYnQKcc0pVR5H_s9lj5UbtKzj9BioFl7j9A-bLrLdMis3juStf9sq0eBJWNRgfNOrX0w85okWHhpj5W5bp-w name:ekoDB price:99] score:0.1111111111111111]

Execution Time: 2296ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 82
Total tokens: 3495

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Message Stream (SSE) Example (Go) ===

Created session: 2bKr4vYxla4JJZVyqpR9S9tnAdJlOjnb0jO74TGwGZwHp26J9Y_6CUz213m1h3H3V223AYgmVbz6saWzlhG7VQ

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}EkoDB is a high-performance embedded database designed for handling large amounts of data. It aims to provide efficient storage and retrieval capabilities with a focus on speed and scalability. EkoDB is often embedded within applications to provide database functionalities without requiring a separate database server.

Some key features of EkoDB may include:

1. **In-Memory Processing**: It often utilizes in-memory processing for faster data access while supporting durable storage options.

2. **ACID Compliance**: EkoDB typically provides ACID (Atomicity, Consistency, Isolation, Durability) transactions to ensure data integrity, though specific implementations may vary.

3. **Flexible Data Models**: It can support various data structures and types, accommodating different data storage and retrieval needs.

4. **Concurrency Control**: EkoDB may offer various mechanisms for managing concurrent access to the database, allowing multiple threads or processes to interact with the database efficiently.

5. **Embedded Nature**: As an embedded database, it can be integrated directly into applications, which can simplify deployment and reduce overhead.

6. **Cross-Platform Support**: EkoDB may be designed to work across various operating systems and environments.

It's worth noting that while EkoDB may not be as widely known as other databases like SQLite or MongoDB, it caters to specific use cases that benefit from its design and performance characteristics. If you need more specific information or any updates on EkoDB, consider checking its official documentation or repositories.

--- Stream complete ---
Message ID: magzsHvPe60StxfFjBVb7U5nhNGETzJDfIOdbQUz6TH1vGMhLvU6NKfFaXnb_QAh9fnVWAazDhBCH-oyLRqc9g
Execution time: 2578ms
Context window: 128000 tokens

✓ Chat message stream example completed
✓ Client created

=== Get All Chat Models ===
OpenAI models: [gpt-3.5-turbo-0125 gpt-5.4-2026-03-05 gpt-5-nano gpt-5.4-mini o1-pro gpt-5.1-codex tts-1 gpt-5-mini gpt-5-mini-2025-08-07 gpt-4-turbo-2024-04-09 gpt-realtime-2.1-mini gpt-realtime-2025-08-28 gpt-4 tts-1-hd gpt-realtime-1.5 gpt-realtime gpt-5.2-pro gpt-5-chat-latest gpt-5.5-pro gpt-audio-2025-08-28 gpt-4o-mini-transcribe-2025-03-20 omni-moderation-2024-09-26 gpt-live-transcribe gpt-5.6-luna omni-moderation-latest o4-mini-2025-04-16 gpt-4o-2024-11-20 gpt-5.1 whisper-1 gpt-5-pro gpt-realtime-whisper gpt-5-2025-08-07 gpt-5.4-mini-2026-03-17 babbage-002 gpt-4o-2024-05-13 text-embedding-3-large chat-latest gpt-realtime-translate gpt-5.4-pro-2026-03-05 gpt-realtime-2 gpt-5-search-api gpt-5.5-2026-04-23 tts-1-1106 gpt-5.5-pro-2026-04-23 gpt-audio-mini-2025-10-06 gpt-5.2-pro-2025-12-11 o3-mini-2025-01-31 gpt-audio-1.5 gpt-image-1.5 gpt-4-turbo gpt-4o-2024-08-06 o3 gpt-4o-mini-search-preview-2025-03-11 gpt-4o-mini-transcribe-2025-12-15 gpt-4.1-mini gpt-4o-mini gpt-4.1-nano gpt-image-2 gpt-3.5-turbo-16k text-embedding-ada-002 gpt-realtime-2.1 gpt-4.1 gpt-5.5 gpt-4o-transcribe-diarize gpt-5-codex gpt-4-0613 gpt-5.1-2025-11-13 gpt-4o-search-preview o1-2024-12-17 gpt-5-pro-2025-10-06 gpt-5.6-sol gpt-4o-mini-tts-2025-12-15 gpt-image-1 gpt-audio gpt-5.3-codex gpt-5-search-api-2025-10-14 gpt-realtime-mini gpt-image-1-mini gpt-5.6-terra gpt-3.5-turbo-instruct-0914 sora-2 gpt-5.4-pro gpt-4o-mini-tts gpt-realtime-mini-2025-12-15 gpt-5.1-codex-max gpt-4o-mini-2024-07-18 gpt-4.1-2025-04-14 gpt-5 gpt-4o-mini-transcribe gpt-image-2-2026-04-21 gpt-5.2-chat-latest sora-2-pro gpt-3.5-turbo-instruct gpt-transcribe gpt-4.1-nano-2025-04-14 gpt-5.1-chat-latest tts-1-hd-1106 gpt-5.2-codex gpt-4o o4-mini-deep-research gpt-5.4 gpt-4o-search-preview-2025-03-11 text-embedding-3-small gpt-5-nano-2025-08-07 gpt-4o-transcribe o4-mini gpt-5.1-codex-mini gpt-image-2.5-sunburst-2026-09-08 gpt-5.3-chat-latest o3-2025-04-16 gpt-5.2-2025-12-11 chatgpt-image-latest gpt-3.5-turbo gpt-3.5-turbo-1106 gpt-4o-mini-tts-2025-03-20 gpt-5.2 gpt-5.4-nano-2026-03-17 o1 davinci-002 gpt-5.4-nano o3-mini o4-mini-deep-research-2025-06-26 gpt-audio-mini-2025-12-15 o1-pro-2025-03-19 gpt-6-luna gpt-image-2.5-flare gpt-6-astra gpt-audio-mini gpt-4.1-mini-2025-04-14 gpt-image-2.5-sunburst gpt-image-2.5-flare-2026-09-08 gpt-4o-mini-search-preview gpt-6-sol gpt-live-1 gpt-6.1-sol]
Anthropic models: [claude-haiku-5-5 claude-sonnet-5-5 claude-opus-5-5 claude-fable-5-1 claude-opus-5 claude-sonnet-5 claude-fable-5 claude-opus-4-8 claude-opus-4-7 claude-sonnet-4-6 claude-opus-4-6 claude-opus-4-5-20251101 claude-haiku-4-5-20251001 claude-sonnet-4-5-20250929]
Perplexity models: []
Gemini models: [gemini-2.5-flash gemini-2.5-pro gemini-2.5-flash-preview-tts gemini-2.5-pro-preview-tts gemma-4-26b-a4b-it gemma-4-31b-it gemini-flash-latest gemini-flash-lite-latest gemini-pro-latest gemini-2.5-flash-lite gemini-2.5-flash-image gemini-3-flash-preview gemini-3.1-pro-preview gemini-3.1-pro-preview-customtools gemini-3.1-flash-lite-preview gemini-3.1-flash-lite gemini-3-pro-image-preview gemini-3-pro-image nano-banana-pro-preview gemini-3.1-flash-image-preview gemini-3.1-flash-image gemini-3.1-flash-lite-image gemini-nano-banana-2.1 gemini-3.5-flash gemini-3.5-flash-lite gemini-omni-flash-preview gemini-omni-1.1-flash gemini-3.5-transcribe gemini-3.6-flash gemini-3.7-flash gemini-3.8-flash gemini-4-argon lyria-3-clip-preview lyria-3-pro-preview lyria-3.5 gemini-3.1-flash-tts-preview gemini-3.8-flash-tts gemini-3.8-flash-lite-tts gemini-robotics-er-2-preview gemini-2.5-computer-use-preview-10-2025 antigravity-preview-05-2026 antigravity-preview-09-2026 antigravity-preview-latest deep-research-max-preview-04-2026 deep-research-preview-04-2026 deep-research-pro-preview-12-2025 gemini-embedding-001 gemini-embedding-2-preview gemini-embedding-2 aqa veo-3.1-generate-preview veo-3.1-fast-generate-preview veo-3.1-lite-generate-preview gemini-3.5-transcribe-live gemini-2.5-flash-native-audio-latest gemini-2.5-flash-native-audio-preview-09-2025 gemini-2.5-flash-native-audio-preview-12-2025 gemini-3.1-flash-live-preview gemini-3.8-live gemini-3.8-live-extended-thinking gemini-robotics-er-2-streaming-preview gemini-3.5-live-translate-preview lyria-realtime-exp]
  anthropic: ok 14 models
  gemini: ok 63 models
  openai: ok 135 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get OpenAI Models ===
OpenAI models: [gpt-3.5-turbo-0125 gpt-5.4-2026-03-05 gpt-5-nano gpt-5.4-mini o1-pro gpt-5.1-codex tts-1 gpt-5-mini gpt-5-mini-2025-08-07 gpt-4-turbo-2024-04-09 gpt-realtime-2.1-mini gpt-realtime-2025-08-28 gpt-4 tts-1-hd gpt-realtime-1.5 gpt-realtime gpt-5.2-pro gpt-5-chat-latest gpt-5.5-pro gpt-audio-2025-08-28 gpt-4o-mini-transcribe-2025-03-20 omni-moderation-2024-09-26 gpt-live-transcribe gpt-5.6-luna omni-moderation-latest o4-mini-2025-04-16 gpt-4o-2024-11-20 gpt-5.1 whisper-1 gpt-5-pro gpt-realtime-whisper gpt-5-2025-08-07 gpt-5.4-mini-2026-03-17 babbage-002 gpt-4o-2024-05-13 text-embedding-3-large chat-latest gpt-realtime-translate gpt-5.4-pro-2026-03-05 gpt-realtime-2 gpt-5-search-api gpt-5.5-2026-04-23 tts-1-1106 gpt-5.5-pro-2026-04-23 gpt-audio-mini-2025-10-06 gpt-5.2-pro-2025-12-11 o3-mini-2025-01-31 gpt-audio-1.5 gpt-image-1.5 gpt-4-turbo gpt-4o-2024-08-06 o3 gpt-4o-mini-search-preview-2025-03-11 gpt-4o-mini-transcribe-2025-12-15 gpt-4.1-mini gpt-4o-mini gpt-4.1-nano gpt-image-2 gpt-3.5-turbo-16k text-embedding-ada-002 gpt-realtime-2.1 gpt-4.1 gpt-5.5 gpt-4o-transcribe-diarize gpt-5-codex gpt-4-0613 gpt-5.1-2025-11-13 gpt-4o-search-preview o1-2024-12-17 gpt-5-pro-2025-10-06 gpt-5.6-sol gpt-4o-mini-tts-2025-12-15 gpt-image-1 gpt-audio gpt-5.3-codex gpt-5-search-api-2025-10-14 gpt-realtime-mini gpt-image-1-mini gpt-5.6-terra gpt-3.5-turbo-instruct-0914 sora-2 gpt-5.4-pro gpt-4o-mini-tts gpt-realtime-mini-2025-12-15 gpt-5.1-codex-max gpt-4o-mini-2024-07-18 gpt-4.1-2025-04-14 gpt-5 gpt-4o-mini-transcribe gpt-image-2-2026-04-21 gpt-5.2-chat-latest sora-2-pro gpt-3.5-turbo-instruct gpt-transcribe gpt-4.1-nano-2025-04-14 gpt-5.1-chat-latest tts-1-hd-1106 gpt-5.2-codex gpt-4o o4-mini-deep-research gpt-5.4 gpt-4o-search-preview-2025-03-11 text-embedding-3-small gpt-5-nano-2025-08-07 gpt-4o-transcribe o4-mini gpt-5.1-codex-mini gpt-image-2.5-sunburst-2026-09-08 gpt-5.3-chat-latest o3-2025-04-16 gpt-5.2-2025-12-11 chatgpt-image-latest gpt-3.5-turbo gpt-3.5-turbo-1106 gpt-4o-mini-tts-2025-03-20 gpt-5.2 gpt-5.4-nano-2026-03-17 o1 davinci-002 gpt-5.4-nano o3-mini o4-mini-deep-research-2025-06-26 gpt-audio-mini-2025-12-15 o1-pro-2025-03-19 gpt-6-luna gpt-image-2.5-flare gpt-6-astra gpt-audio-mini gpt-4.1-mini-2025-04-14 gpt-image-2.5-sunburst gpt-image-2.5-flare-2026-09-08 gpt-4o-mini-search-preview gpt-6-sol gpt-live-1 gpt-6.1-sol]

=== Get Anthropic Models ===
Anthropic models: [claude-haiku-5-5 claude-sonnet-5-5 claude-opus-5-5 claude-fable-5-1 claude-opus-5 claude-sonnet-5 claude-fable-5 claude-opus-4-8 claude-opus-4-7 claude-sonnet-4-6 claude-opus-4-6 claude-opus-4-5-20251101 claude-haiku-4-5-20251001 claude-sonnet-4-5-20250929]

✓ Chat Models API example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: R_X6OYSgXqXTGwAssMNY_Bmn6gJM_YP3fx19z9YDG9AmeHgm80PMtBVuE60ZpLTDrhLB2nvlpWNxofZGmgeb2Q

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99

If you need more details or assistance, let me know!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: sMfFOuwxdLahgeq78vhKJTgX6BCziHlUxJQdXocURdxflquHp448o2XR85YH21seECUdJ-jaEbnGn1k8TvQcrA
  Parent: R_X6OYSgXqXTGwAssMNY_Bmn6gJM_YP3fx19z9YDG9AmeHgm80PMtBVuE60ZpLTDrhLB2nvlpWNxofZGmgeb2Q

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: sMfFOuwxdLahgeq78vhKJTgX6BCziHlUxJQdXocURdxflquHp448o2XR85YH21seECUdJ-jaEbnGn1k8TvQcrA (Untitled)
  Session 2: R_X6OYSgXqXTGwAssMNY_Bmn6gJM_YP3fx19z9YDG9AmeHgm80PMtBVuE60ZpLTDrhLB2nvlpWNxofZGmgeb2Q (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: sMfFOuwxdLahgeq78vhKJTgX6BCziHlUxJQdXocURdxflquHp448o2XR85YH21seECUdJ-jaEbnGn1k8TvQcrA

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: 1e0v2oe0k0Wd-2J5blXKdpZYu2ULqHfM3gbimIuxyxqDLGyAfR_8z_6HBmQHvXrMEyKvRmm84Qh1Yg3ciWR1MQ

=== List Collections ===
Total collections: 13
Sample collections: [chat_goals__ek0_testing chat_messages__ek0_testing schedules__ek0_testing chat_raw_completions__ek0_testing client_collection_management_go]

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
✓ Created record with native map: map[id:Uc3r611KzvH5roplXCAyVTJsl6PclJPXWzfvYW6hgrlI-iDN_lMqm78oa80SB_6QD799UDLFxT3JG_qHPgqr9Q]

=== Upsert Operation ===
✓ First upsert (update): map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:Uc3r611KzvH5roplXCAyVTJsl6PclJPXWzfvYW6hgrlI-iDN_lMqm78oa80SB_6QD799UDLFxT3JG_qHPgqr9Q name:map[type:String value:Alice Johnson]]
✓ Second upsert (insert): map[id:new-user-1792225885613372000]

=== Find One Operation ===
✓ Found user by email: map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:Uc3r611KzvH5roplXCAyVTJsl6PclJPXWzfvYW6hgrlI-iDN_lMqm78oa80SB_6QD799UDLFxT3JG_qHPgqr9Q name:map[type:String value:Alice Johnson]]
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
✓ Inserted document: KlzbYSJL8PQWaIcBSFERsvX-6l4qGyDPtthtfWKtGWndrV_v44EBAUhJNUtSR86T466l256t4hqy-8Z5nHd83w

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: oM761wav9K0_PJJsS9JhCGFv9Ua4mq57KsAaJPcRlYCpt_ig6HNf0NuLqte7O4eNTro5JQI2SKCe6aLQ6Aim5w

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
✓ Edge cache script created: -6yGhVFsh1winLh_peDM2ZgPmrrqH-J2L4eNGqN2-uCsQ4uHDgWkzBlL_WfX170Duy2BEuKwPzEQkPlXEAu_zA

Call 1: Cache lookup
Response time: 6ms
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
   ⏱️  Duration: 61.77325ms
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
   ⏱️  Duration: 5.2895ms
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
   🚀 Cache speedup: 12.2x faster!

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

✅ Function saved: semsoJxK0TvxY2x5lobISQ9Tz_QLj7rjsHVOCaG8RsUYziGrTfTow2QHCHW272Ha3_smr93knlmvNm2KrZMDpg
📊 Found 5 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

✅ Function saved
📊 Found 3 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 2 groups
   map[avg_score:map[type:Float value:50] count:map[type:Integer value:5] status:map[type:String value:inactive]]
   map[avg_score:map[type:Float value:60] count:map[type:Integer value:5] status:map[type:String value:active]]
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
   map[avg_price:map[type:Float value:365.6666666666667] category:map[type:String value:Furniture] count:map[type:Integer value:3]]
   map[avg_price:map[type:Float value:367] category:map[type:String value:Electronics] count:map[type:Integer value:5]]
⏱️  Execution time: 0ms

✅ All advanced script examples finished!
🧹 Cleaning up...
✅ Cleanup complete
🚀 ekoDB Go AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
📊 AI Response generated
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embed script saved
📊 Embedding generated
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

✅ Function saved: za50xpQu1qVufuyvCpqOcVkqu2nTsEhUZGU9lZp8ha5CdiHX0_vdEUFupE_bEnssUtRF8PraHE4jmXBrUoW94w
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
   map[count:map[type:Integer value:3] status:map[type:String value:inactive]]
   map[count:map[type:Integer value:7] status:map[type:String value:active]]
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

✅ Inserted order: BG0gaeTRUFkd5fLQMMg74y5jp-OJ-p34zpMPh407tH6c0icmGS92TaiHCxTCtgI-s6-gOkX3eAlQw0WbyVfxlg
✅ Inserted 2 products with wrapped types

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: jIexamZkLe0hSR3UkiQ37Fs7StzEIkYxwsyi5sQLaxG5lewKTsiqGYUPhxhk-C5vTI_vfiZYMlZ0vlCBAgX9hw
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: map[type:Object value:map[role:admin userId:user_abc]]
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session
📝 Example 4: KV Operations in Functions

✅ Function saved: DLhjxMEd-hoMPKeAewg8zHHHglGbes-ztyMgUIBYTNJ7ftacXcomL1AUHHip2qAnR0Y2BO3YqtG6oaeErDMZ4Q
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: tSJ2AHPR3DcWGqhUxKSmuG265H-U4o0PZsYaStDwnHM4G0kaZCVgcU7ygdsc4qGrTUSCrxrbyqo2XjjCF0bpUg
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
   1. map[type:String value:Introduction to Machine Learning] (map[type:String value:AI])
   2. map[type:String value:Database Design Principles] (map[type:String value:Database])
   3. map[type:String value:Vector Databases Explained] (map[type:String value:Database])
   4. map[type:String value:Natural Language Processing] (map[type:String value:AI])
   5. map[type:String value:Getting Started with ekoDB] (map[type:String value:Database])
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
Created template: Data Migration (id: uFfcepZshgoltK4BQWyfV8TPjip7G9yScUBngF6ZNmnG15mFw34S3Q4nBahPcZ32aqevMJaMw240cpghPASlFQ)

--- Listing templates ---
Templates: map[count:1 items:[map[description:map[type:String value:Template for migrating data between schemas] id:uFfcepZshgoltK4BQWyfV8TPjip7G9yScUBngF6ZNmnG15mFw34S3Q4nBahPcZ32aqevMJaMw240cpghPASlFQ steps:map[type:Array value:[map[description:Analyze source schema] map[description:Create target schema] map[description:Migrate records] map[description:Validate results]]] title:map[type:String value:Data Migration]]]]

--- Getting template ---
Fetched: Data Migration

--- Updating template ---
Updated description: Updated: comprehensive data migration workflow

--- Deleting template ---
Template deleted successfully

✓ Goal template CRUD example completed
=== ekoDB Goals, Tasks & Agents Integration Example (Go) ===

--- Creating goal ---
Created goal: Deploy v2.0 (id: 4k_MyO1vuR3dk2StuhmWtGFPYjMXQZ8F8Juh7HPGZ3qlm9bJ6BxfWGeqL0lZKBUrpgx1qhmo0fr5oekwYRFhew)

--- Listing goals ---
Goals response: map[count:1 goals:[map[created_at:2026-10-17T08:31:34.430927+00:00 description:Ship the v2.0 release to production id:4k_MyO1vuR3dk2StuhmWtGFPYjMXQZ8F8Juh7HPGZ3qlm9bJ6BxfWGeqL0lZKBUrpgx1qhmo0fr5oekwYRFhew status:pending steps:[{"description":"Run integration tests"},{"description":"Build release artifacts"},{"description":"Deploy to staging"}] title:Deploy v2.0 updated_at:2026-10-17T08:31:34.430927+00:00]]]

--- Getting goal ---
Fetched goal: Deploy v2.0

--- Updating goal ---
Updated description: Ship the v2.0 release to production (with hotfix)

--- Searching goals ---
Search result: map[count:1 items:[map[_score:12.870000000000001 created_at:map[type:DateTime value:2026-10-17T08:31:34.430927+00:00] description:map[type:String value:Ship the v2.0 release to production (with hotfix)] id:4k_MyO1vuR3dk2StuhmWtGFPYjMXQZ8F8Juh7HPGZ3qlm9bJ6BxfWGeqL0lZKBUrpgx1qhmo0fr5oekwYRFhew status:map[type:String value:pending] steps:map[type:String value:[{"description":"Run integration tests"},{"description":"Build release artifacts"},{"description":"Deploy to staging"}]] title:map[type:String value:Deploy v2.0] updated_at:map[type:DateTime value:2026-10-17T08:31:34.440631+00:00]]]]

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
Created task: Run benchmarks (id: NC0FfABNScxQvegikxq64Y_PObJlUG_9N0FOatImhlFBsDhZgnjVeuB5u0bVNjYMlt5RDPa6foJRIW51fUagYg)

--- Listing tasks ---
Tasks: map[count:1 items:[map[description:map[type:String value:Execute YCSB benchmarks against staging] due_at:map[type:DateTime value:2026-10-18T08:31:34+00:00] id:NC0FfABNScxQvegikxq64Y_PObJlUG_9N0FOatImhlFBsDhZgnjVeuB5u0bVNjYMlt5RDPa6foJRIW51fUagYg name:map[type:String value:Run benchmarks]]]]

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
Created agent: benchmark-runner (id: pibHXP9iyt0ZmDLSgVO6GlErmt_WMw2z47IQTG9uyKOst26u7xs-CHD--q2U0trQ6zWvYAkDqPuynFFrjSKCDA)

--- Listing agents ---
Agents: map[count:1 items:[map[deployment_id:map[type:String value:deploy_prod_1] description:map[type:String value:Runs periodic benchmarks] id:pibHXP9iyt0ZmDLSgVO6GlErmt_WMw2z47IQTG9uyKOst26u7xs-CHD--q2U0trQ6zWvYAkDqPuynFFrjSKCDA llm_model:map[type:String value:gpt-4.1] name:map[type:String value:benchmark-runner]]]]

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
  - Mouse ($25) by Alice Johnson
  - Laptop ($1200) by Alice Johnson

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
Inserted doc1: Q_eD-ZKcCdbkCJ_aWWfoxAGgBQLyQAgC87_VqxdIAhwHLGJtQiKCHteR395MgBoDlK3CuYAkifWGk1Jh6tIw9A
Inserted doc2: 9_x1xQt62ynC0EOSqqQy7u7OR557UbCy9mjiXcaTnPBa3nsyZhqV0Kh6J9VCa8hKdzwwY1-eWz93L8AfoLAMKg

--- Setting KV entry ---
KV entry set: team:engineering:go

--- Linking KV key to documents ---
Linked doc1: map[]
Linked doc2: map[]

--- Getting links ---
Links for team:engineering:go: [map[collection:kv_links_example_go created_at:2026-10-17T08:31:35.906616Z document_id:Q_eD-ZKcCdbkCJ_aWWfoxAGgBQLyQAgC87_VqxdIAhwHLGJtQiKCHteR395MgBoDlK3CuYAkifWGk1Jh6tIw9A field_path:<nil> last_accessed:2026-10-17T08:31:35.907866Z metadata:map[]] map[collection:kv_links_example_go created_at:2026-10-17T08:31:35.907351Z document_id:9_x1xQt62ynC0EOSqqQy7u7OR557UbCy9mjiXcaTnPBa3nsyZhqV0Kh6J9VCa8hKdzwwY1-eWz93L8AfoLAMKg field_path:<nil> last_accessed:2026-10-17T08:31:35.907866Z metadata:map[]]]

--- Unlinking doc2 ---
Unlink result: map[]

--- Verifying links after unlink ---
Links after unlink: [map[collection:kv_links_example_go created_at:2026-10-17T08:31:35.906616Z document_id:Q_eD-ZKcCdbkCJ_aWWfoxAGgBQLyQAgC87_VqxdIAhwHLGJtQiKCHteR395MgBoDlK3CuYAkifWGk1Jh6tIw9A field_path:<nil> last_accessed:2026-10-17T08:31:35.909124Z metadata:map[]]]

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
Response: The three primary colors are red, blue, and yellow.

--- Structured Output via SSE ---
JSON response: [
  {
    "name": "Earth",
    "diameter_km": 12742
  },
  {
    "name": "Mars",
    "diameter_km": 6779
  },
  {
    "name": "Jupiter",
    "diameter_km": 139820
  }
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello! I hope you're having a wonderful day.

=== Done ===
=== ekoDB Schedules Integration Example (Go) ===

--- Creating schedule ---
Created schedule: nightly-backup (id: 388df629-fb8b-43a4-bc29-dc58f5b8f62b)

--- Listing schedules ---
Schedules: map[count:1 schedules:[map[created_at:2026-10-17T08:31:39.969648Z cron_expression:0 0 0 * * * description:Runs a database backup every night at midnight enabled:true function_label:schedule_noop_go_89464 id:388df629-fb8b-43a4-bc29-dc58f5b8f62b last_execution:<nil> name:nightly-backup next_execution:2026-10-18T00:00:00Z parameters:map[] stats:map[avg_execution_time_ms:0 failed_executions:0 last_error:<nil> successful_executions:0 total_executions:0] timezone:UTC updated_at:2026-10-17T08:31:39.969648Z]]]

--- Getting schedule ---
Fetched schedule: nightly-backup (cron: 0 0 0 * * *)

--- Updating schedule ---
Updated cron: 0 0 2 * * *

--- Triggering schedule ---
Trigger response: map[schedule_id:388df629-fb8b-43a4-bc29-dc58f5b8f62b status:triggered]

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
  1. Score: 0.762
  2. Score: 0.748
  3. Score: 0.746

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.498
  2. Score: 0.905
  3. Score: 0.299

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200

9. Vector search with a metadata pre-filter (category = ml):
Found 2 documents in category "ml" (NLP excluded)
  1. Deep Learning Fundamentals (category: ml)
  2. Introduction to Machine Learning (category: ml)

Paired vector search: document=Rust Programming Answer, query=Rust Programming Question
Document search execution path: index
Query search execution path: index

✅ Search examples completed!
=== Cleanup ===
✅ Deleted test collections
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: map[id:gBBdxYT6G0ZtZcndEGYcWGA7MDoz8nY0VSQgfPBbisZeS9i7IfrDtBoOwCVpaHn4Px9g2XiQ9-n0ok4IO8aJjw]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-10-17T08:31:41+00:00] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:gBBdxYT6G0ZtZcndEGYcWGA7MDoz8nY0VSQgfPBbisZeS9i7IfrDtBoOwCVpaHn4Px9g2XiQ9-n0ok4IO8aJjw metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Test Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:42]]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-10-17 08:31:41 +0000 +0000
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-10-17T08:31:41+00:00 data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:gBBdxYT6G0ZtZcndEGYcWGA7MDoz8nY0VSQgfPBbisZeS9i7IfrDtBoOwCVpaHn4Px9g2XiQ9-n0ok4IO8aJjw metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-10-17T08:31:41+00:00] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:gBBdxYT6G0ZtZcndEGYcWGA7MDoz8nY0VSQgfPBbisZeS9i7IfrDtBoOwCVpaHn4Px9g2XiQ9-n0ok4IO8aJjw metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: nkv2pg5YxCynJ8ADQHajmGOPKlS6WvWod3jznLE4zIEqPD0Ium82uwbkQPZTQy7JvHom2dfs0tXXyFDmskdVKw

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
✓ Created native SWR script: github_user_native_go (5s4_xRw9C-HK5SG4YXo3ld6Xb5hsegSWseh1bNSWk-RAViyoNdPcR-2q5hJH53LhNNEpmni7DbaEqXwze6xw-Q)

First call (cache miss - will fetch from GitHub API):
  Response time: 172ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 1ms
  Speedup: 172.0x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_go (LbcjTcBHnz3kPR6IwacJY-nGzHmtLotmvqKQq0eQM6CXLBNQBWtgHYPPktqppci6DkYdjNH5zWaAxjk0jchhig)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_go' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_go (sav4aSzinVB04n0BBWkYKx-_6H9wlLEGVuvWGdveDaksjwzOIDXjfErzIgujNUmPGM2IYetwYejZ0wPcTQt2NA)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_go' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_go (dJ-yq-SlByC2QV1slvVA82Mth3wA98HBBKZm-xQYm_tyS2R9A-XKGlMZgy3gS0VbSjyM1ARwQbdnbkCNLoFMzw)
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
✓ Created SWR script: fetch_api_user_go (8BRSnkc9SHmZfnahfw__q_Nf32T60uNK4qjkgXEogqNDFUsntVvdRFC1xXA7HAf6yq8Aq4172qgKacpJYq8i4w)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "cached_at": {
        "type": "DateTime",
        "value": "2026-10-17T08:31:43+00:00"
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
Response time: 8ms (served from cache)
✓ Lightning fast cache hit

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
🧹 Cleaning up...
✓ Cleanup complete

✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: T9fZhw27RJ1orm7_xNkeytThxEBFZkesZGCzqtBEVEvcqgPAqiBoT9j5QMbIz6k9YImu1JctBcKKnsJQt6MS0A
Created Bob: $500 - ID: wbxnYaUC3OMJn32D9LH-lgkMMqBlzhyunJDKjwLG7_UWzBTpujKaN-3JQpKfNku2bwaPC7zMUwFCbQvEPHcY-A

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): aaf616dd-2434-4388-a744-666d71b5dbb8

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
New transaction: cd73ecc8-5a43-4686-833e-6ed7cbd00f86
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Create User Function ===
Created user function with ID: U_itaWxh_DX22l94Q1QTlPXg4qOULrnGBvS02247Q5rPJhyDV2uK8rppyNYWuTAYWljidbutz9mXqkzIajC7yA

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

Created chat session: xnDHjubTMHq3swTicBw0bWA5zc-T4cnkOs5SRhmOJuZuj6p6nkaL8ORuLXeKtBW2uY23sGMpeP6-tsSk3SdrhA

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: iPjSNme-p7kMzB1HZm6STHMBd0AnhM3vkVJsHp0JMZ6vMingMwPUlWsAOfvX23N5fGxLhi5dtdr1ojEa3K_Q8Q
Execution time: 555ms
Token usage: {"completion_tokens":8,"prompt_tokens":15,"total_tokens":23}

Full response: {"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}The capital of France is Paris....
=== WebSocket Subscription Example (Go) ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_go' ===
✓ Subscribed (subscription_id: sub_d2552701bfe341af92464aa171f104f2)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: TGS8c4sTy_n-fLCWEt-8c9NvKvm5I2dtCpX7FtqTwLmXO956e8vqfsh4alfzP5Tagm_bq2YNVktZiQrMgbYfJA

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_go
     Record IDs: [TGS8c4sTy_n-fLCWEt-8c9NvKvm5I2dtCpX7FtqTwLmXO956e8vqfsh4alfzP5Tagm_bq2YNVktZiQrMgbYfJA]
     Timestamp:  2026-10-17T08:31:45.490548+00:00

Inserting record 2...
✓ Inserted: v3zcdhh4QBN_25t155jVMGHusB03ZPRDeVSsW3bklPJBEpv5ENxP0rJFrHeOPVRB0bDZJzHo3RJOItRWKmH0EQ

  📡 Notification received:
     Event:      insert
     Record IDs: [v3zcdhh4QBN_25t155jVMGHusB03ZPRDeVSsW3bklPJBEpv5ENxP0rJFrHeOPVRB0bDZJzHo3RJOItRWKmH0EQ]

=== Unsubscribing ===
✓ Unsubscribed

✓ WebSocket subscription example completed successfully
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: WvB4GISytPiyri8GthiyoZ0rYHdIxteJEL8m1krGXXk9pVKWK4GgOmXEwrmyjaElo333gMsUKwTeLmjWwUDi2A

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
Inserted with ripple: map[id:hyUEp0xnGCaWCz5xuk1gnR41-fBV5hRZDh1i0ixEfEiTpanbyQ_guEhefvHGqI0VhrywUpsT_OknR3AWxut6wA]
Inserted with bypass_ripple: map[id:9MGRVF9U374yyCT23WpCgRkipf8RsKNmDHLIQEoPtNytR3RvLKst9DuCdJ7ME7RHxNymZJ37A0dQC4qowfWbpQ]
Inserted with TTL and bypass_ripple: map[id:E0aURMgHQ7md6M_Jd_zFQKKz2aMn3N740XGCs54P9JAgebau2WL3wxcgOgK_BTnZHeNAlsOrVfsAlii1smnFyA]
Updated with bypass_ripple: map[id:hyUEp0xnGCaWCz5xuk1gnR41-fBV5hRZDh1i0ixEfEiTpanbyQ_guEhefvHGqI0VhrywUpsT_OknR3AWxut6wA name:map[type:String value:Product 1] price:map[type:Integer value:150]]
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
✅ All Go integration tests complete!
📦 Building TypeScript client library...

> @ekodb/ekodb-client@0.30.0 prepare
> npm run build


> @ekodb/ekodb-client@0.30.0 build
> tsc


up to date, audited 44 packages in 830ms

12 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.30.0 build
> tsc

✅ TypeScript client built!

added 1 package, removed 1 package, and audited 13 packages in 393ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
=== ekoDB Advanced CRUD Example (TypeScript) ===

--- Inserting base record ---
Inserted: AyxJs7ijmEUPrjCsm-x3F7CMgkniAxJVBz-JamVE7iTxwYR93njm2el41qkp8BWBfZKxbzoVI4gDu0CBUkBqYA

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
✓ Created session: q84oMiT3ltor5fJSuW1SSXYzgagF8l1Wp9gAdkzzFCfD1KoFadVyVavM7BSCBtIlEs1NEBHSlV-qnx5yhWtDFg

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you need more information or details about other products, please let me know!

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
✓ Created second session: QoRHQ2ZgOVZF5LUkxUOm1dx_3RO_DPOCHzhfNGf11VKZHzKV5kcMJy_-EnF-lPktBKJ4xTNpPIJLT4xyvruQAw
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: QoRHQ2ZgOVZF5LUkxUOm1dx_3RO_DPOCHzhfNGf11VKZHzKV5kcMJy_-EnF-lPktBKJ4xTNpPIJLT4xyvruQAw
✓ Deleted chat session: q84oMiT3ltor5fJSuW1SSXYzgagF8l1Wp9gAdkzzFCfD1KoFadVyVavM7BSCBtIlEs1NEBHSlV-qnx5yhWtDFg
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: QU4TYNdJZzawBTNExcVAZl_HevZTAR3bBZK7jmB3Gkj7eizssK8aGPfgEiNHKG7pk8EbNnEtLn1Of80YZz2sNw

=== Sending Chat Message ===
Message ID: IHI9c8KSh2u8oH3FGKPpVByGjpdujUSJkCIsxSOPmFHFQnmayqyFOOuXeHLF9Bnb9XYREikoigwWJmv5C-gxrw

=== AI Response ===
Here are the available products along with their prices:

1. **ekoDB**: $99
   - Description: A high-performance database product with AI capabilities.

2. **ekoDB Cloud**: $499
   - Description: Fully managed cloud database service product.

3. **ekoDB Pro**: $299
   - Description: Enterprise edition product with advanced features.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'A high-performance database product with AI capabilities',
    id: '5o7-f8D3tj4EUy-5yYd7svfB1WyzVfDglklEwTYUXXuMD7QXdWtJJ3Otm9QpnImsxiqfXKnEyOnpoK1YNWlysA',
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
    id: 'EJia6CjvYSiPHg8pEaSIJG-J50v7PYe6K3ScO6uxlcxIhA9xj40FWNg4HLRn43yOs8MJuEJAy9K3K1JlILnb8Q',
    name: 'ekoDB Cloud',
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB Pro',
    id: 'ZaGVTh4Nw4HYwun4YTrQouSo9nE3QkP7f3CIT-XqoIX-GNqYPFZreuCXCIx2Kw4zljvaw6n7xYseuyF1BaT6dg',
    description: 'Enterprise edition product with advanced features',
    price: 299
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2389ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 84
Total tokens: 3497

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Message Stream (SSE) Example (TypeScript) ===

Created session: p3nU_6ONVZrMEYrtXABKTxce9fP0McDHKon_W1BVxX3mKjs0Jz3TWXWDpvgUqeJ7Zf31oolT3Lqu1eLpS-bcnw

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}**ekoDB** is a reference to a knowledge base project developed by **eko.ai**, a company specializing in AI-driven solutions for echocardiography. In general context, **ekoDB** may refer to:

1. **eko.ai's Knowledge Database:**
   - **ekoDB** is often associated with eko.ai’s large curated database containing annotated echocardiography (heart ultrasound) images and videos, along with metadata and clinical measurements.
   - It serves as a resource for training machine learning models, benchmarking AI algorithms, and developing diagnostic tools for cardiac imaging.
   - This database enables researchers and clinicians to access high-quality, anonymized echocardiography data to improve cardiovascular disease detection and management.

2. **Other Possible Usages:**
   - While there are a few unrelated products or software packages named similarly (for instance, some experimental or proprietary software tools in different industries), in the context of medical AI or digital health, **ekoDB** almost always refers to eko.ai’s data resources.

**Summary:**
**ekoDB** is a comprehensive echocardiography knowledgebase developed by eko.ai, used for AI research, model training, and improving heart disease detection from ultrasound data.

If you meant a different context or industry by "ekoDB," please provide more information for a more tailored answer!

--- Stream complete ---
Message ID: mcKjd9vd5k5GNTq19s1Ek-vRFFqH-wzZEYXKjWiLMypp5MbENNfeeFg1B3Froigg9PzqIjk1Bmk6BNJwtHdAgA
Execution time: 1875ms
Context window: 1000000 tokens

✓ Chat message stream example completed
✓ Client created

=== List Chat Models ===
Available chat models by provider:
  openai:
    - gpt-3.5-turbo-0125
    - gpt-5.4-2026-03-05
    - gpt-5-nano
    - gpt-5.4-mini
    - o1-pro
    - gpt-5.1-codex
    - tts-1
    - gpt-5-mini
    - gpt-5-mini-2025-08-07
    - gpt-4-turbo-2024-04-09
    - gpt-realtime-2.1-mini
    - gpt-realtime-2025-08-28
    - gpt-4
    - tts-1-hd
    - gpt-realtime-1.5
    - gpt-realtime
    - gpt-5.2-pro
    - gpt-5-chat-latest
    - gpt-5.5-pro
    - gpt-audio-2025-08-28
    - gpt-4o-mini-transcribe-2025-03-20
    - omni-moderation-2024-09-26
    - gpt-live-transcribe
    - gpt-5.6-luna
    - omni-moderation-latest
    - o4-mini-2025-04-16
    - gpt-4o-2024-11-20
    - gpt-5.1
    - whisper-1
    - gpt-5-pro
    - gpt-realtime-whisper
    - gpt-5-2025-08-07
    - gpt-5.4-mini-2026-03-17
    - babbage-002
    - gpt-4o-2024-05-13
    - text-embedding-3-large
    - chat-latest
    - gpt-realtime-translate
    - gpt-5.4-pro-2026-03-05
    - gpt-realtime-2
    - gpt-5-search-api
    - gpt-5.5-2026-04-23
    - tts-1-1106
    - gpt-5.5-pro-2026-04-23
    - gpt-audio-mini-2025-10-06
    - gpt-5.2-pro-2025-12-11
    - o3-mini-2025-01-31
    - gpt-audio-1.5
    - gpt-image-1.5
    - gpt-4-turbo
    - gpt-4o-2024-08-06
    - o3
    - gpt-4o-mini-search-preview-2025-03-11
    - gpt-4o-mini-transcribe-2025-12-15
    - gpt-4.1-mini
    - gpt-4o-mini
    - gpt-4.1-nano
    - gpt-image-2
    - gpt-3.5-turbo-16k
    - text-embedding-ada-002
    - gpt-realtime-2.1
    - gpt-4.1
    - gpt-5.5
    - gpt-4o-transcribe-diarize
    - gpt-5-codex
    - gpt-4-0613
    - gpt-5.1-2025-11-13
    - gpt-4o-search-preview
    - o1-2024-12-17
    - gpt-5-pro-2025-10-06
    - gpt-5.6-sol
    - gpt-4o-mini-tts-2025-12-15
    - gpt-image-1
    - gpt-audio
    - gpt-5.3-codex
    - gpt-5-search-api-2025-10-14
    - gpt-realtime-mini
    - gpt-image-1-mini
    - gpt-5.6-terra
    - gpt-3.5-turbo-instruct-0914
    - sora-2
    - gpt-5.4-pro
    - gpt-4o-mini-tts
    - gpt-realtime-mini-2025-12-15
    - gpt-5.1-codex-max
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
    - gpt-5.1-chat-latest
    - tts-1-hd-1106
    - gpt-5.2-codex
    - gpt-4o
    - o4-mini-deep-research
    - gpt-5.4
    - gpt-4o-search-preview-2025-03-11
    - text-embedding-3-small
    - gpt-5-nano-2025-08-07
    - gpt-4o-transcribe
    - o4-mini
    - gpt-5.1-codex-mini
    - gpt-image-2.5-sunburst-2026-09-08
    - gpt-5.3-chat-latest
    - o3-2025-04-16
    - gpt-5.2-2025-12-11
    - chatgpt-image-latest
    - gpt-3.5-turbo
    - gpt-3.5-turbo-1106
    - gpt-4o-mini-tts-2025-03-20
    - gpt-5.2
    - gpt-5.4-nano-2026-03-17
    - o1
    - davinci-002
    - gpt-5.4-nano
    - o3-mini
    - o4-mini-deep-research-2025-06-26
    - gpt-audio-mini-2025-12-15
    - o1-pro-2025-03-19
    - gpt-6-luna
    - gpt-image-2.5-flare
    - gpt-6-astra
    - gpt-audio-mini
    - gpt-4.1-mini-2025-04-14
    - gpt-image-2.5-sunburst
    - gpt-image-2.5-flare-2026-09-08
    - gpt-4o-mini-search-preview
    - gpt-6-sol
    - gpt-live-1
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
    - veo-3.1-generate-preview
    - veo-3.1-fast-generate-preview
    - veo-3.1-lite-generate-preview
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
  gemini: ok 63 models
  openai: ok 135 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get Specific Provider Models ===
OpenAI models: gpt-3.5-turbo-0125, gpt-5.4-2026-03-05, gpt-5-nano, gpt-5.4-mini, o1-pro, gpt-5.1-codex, tts-1, gpt-5-mini, gpt-5-mini-2025-08-07, gpt-4-turbo-2024-04-09, gpt-realtime-2.1-mini, gpt-realtime-2025-08-28, gpt-4, tts-1-hd, gpt-realtime-1.5, gpt-realtime, gpt-5.2-pro, gpt-5-chat-latest, gpt-5.5-pro, gpt-audio-2025-08-28, gpt-4o-mini-transcribe-2025-03-20, omni-moderation-2024-09-26, gpt-live-transcribe, gpt-5.6-luna, omni-moderation-latest, o4-mini-2025-04-16, gpt-4o-2024-11-20, gpt-5.1, whisper-1, gpt-5-pro, gpt-realtime-whisper, gpt-5-2025-08-07, gpt-5.4-mini-2026-03-17, babbage-002, gpt-4o-2024-05-13, text-embedding-3-large, chat-latest, gpt-realtime-translate, gpt-5.4-pro-2026-03-05, gpt-realtime-2, gpt-5-search-api, gpt-5.5-2026-04-23, tts-1-1106, gpt-5.5-pro-2026-04-23, gpt-audio-mini-2025-10-06, gpt-5.2-pro-2025-12-11, o3-mini-2025-01-31, gpt-audio-1.5, gpt-image-1.5, gpt-4-turbo, gpt-4o-2024-08-06, o3, gpt-4o-mini-search-preview-2025-03-11, gpt-4o-mini-transcribe-2025-12-15, gpt-4.1-mini, gpt-4o-mini, gpt-4.1-nano, gpt-image-2, gpt-3.5-turbo-16k, text-embedding-ada-002, gpt-realtime-2.1, gpt-4.1, gpt-5.5, gpt-4o-transcribe-diarize, gpt-5-codex, gpt-4-0613, gpt-5.1-2025-11-13, gpt-4o-search-preview, o1-2024-12-17, gpt-5-pro-2025-10-06, gpt-5.6-sol, gpt-4o-mini-tts-2025-12-15, gpt-image-1, gpt-audio, gpt-5.3-codex, gpt-5-search-api-2025-10-14, gpt-realtime-mini, gpt-image-1-mini, gpt-5.6-terra, gpt-3.5-turbo-instruct-0914, sora-2, gpt-5.4-pro, gpt-4o-mini-tts, gpt-realtime-mini-2025-12-15, gpt-5.1-codex-max, gpt-4o-mini-2024-07-18, gpt-4.1-2025-04-14, gpt-5, gpt-4o-mini-transcribe, gpt-image-2-2026-04-21, gpt-5.2-chat-latest, sora-2-pro, gpt-3.5-turbo-instruct, gpt-transcribe, gpt-4.1-nano-2025-04-14, gpt-5.1-chat-latest, tts-1-hd-1106, gpt-5.2-codex, gpt-4o, o4-mini-deep-research, gpt-5.4, gpt-4o-search-preview-2025-03-11, text-embedding-3-small, gpt-5-nano-2025-08-07, gpt-4o-transcribe, o4-mini, gpt-5.1-codex-mini, gpt-image-2.5-sunburst-2026-09-08, gpt-5.3-chat-latest, o3-2025-04-16, gpt-5.2-2025-12-11, chatgpt-image-latest, gpt-3.5-turbo, gpt-3.5-turbo-1106, gpt-4o-mini-tts-2025-03-20, gpt-5.2, gpt-5.4-nano-2026-03-17, o1, davinci-002, gpt-5.4-nano, o3-mini, o4-mini-deep-research-2025-06-26, gpt-audio-mini-2025-12-15, o1-pro-2025-03-19, gpt-6-luna, gpt-image-2.5-flare, gpt-6-astra, gpt-audio-mini, gpt-4.1-mini-2025-04-14, gpt-image-2.5-sunburst, gpt-image-2.5-flare-2026-09-08, gpt-4o-mini-search-preview, gpt-6-sol, gpt-live-1, gpt-6.1-sol

=== Get Anthropic Models ===
Anthropic models: claude-haiku-5-5, claude-sonnet-5-5, claude-opus-5-5, claude-fable-5-1, claude-opus-5, claude-sonnet-5, claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 404: {"error":"Unknown provider 'nonexistent_provider_xyz'","error_kind":"unknown_provider"}

✓ Chat Models example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: VkqF_cy4ewdnIntB9xkIMknHbLNHVwQCjupXl_8ZHukV68xNliMuoFenwbJfdk9SQAI7krRRzSvZhvZRvhkpPQ

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** A high-performance database product
- **Price:** $99

If you need more details or specific information, feel free to ask!

✓ Message 2 sent
  Response: The price of the ekoDB product is **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: F9Af0nPmwHuEDC18GcF4JJU0fmlSLktbDcffsyPDwEeyeW-b2LX_NyVMXcsqI-8TctyM0AkM8cwz6JwwaJzSng
  Parent: VkqF_cy4ewdnIntB9xkIMknHbLNHVwQCjupXl_8ZHukV68xNliMuoFenwbJfdk9SQAI7krRRzSvZhvZRvhkpPQ

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: F9Af0nPmwHuEDC18GcF4JJU0fmlSLktbDcffsyPDwEeyeW-b2LX_NyVMXcsqI-8TctyM0AkM8cwz6JwwaJzSng (Untitled)
  Session 2: VkqF_cy4ewdnIntB9xkIMknHbLNHVwQCjupXl_8ZHukV68xNliMuoFenwbJfdk9SQAI7krRRzSvZhvZRvhkpPQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: F9Af0nPmwHuEDC18GcF4JJU0fmlSLktbDcffsyPDwEeyeW-b2LX_NyVMXcsqI-8TctyM0AkM8cwz6JwwaJzSng

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: IbkgOfkd289ndFwLAbqQmHbWHB5QrFzq4AutO_Dx-7Hq4eBLQ2Zpz2-8HXWp-vk_YfstGY_EeOElHWbSpKQ37g

=== List Collections ===
Total collections: 17
Sample collections: chat_goals__ek0_testing,chat_messages__ek0_testing,schedules__ek0_testing,schema_documents_client_go,chat_raw_completions__ek0_testing

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
  POST /api/functions/conc_demo_pay_ts_95985_1792225931695 { "idempotency_key": "...", "amount": 100 }
  POST /api/functions/conc_demo_rl_fail_ts_95985_1792225931695 { "user_id": 42 }
  POST /api/functions/conc_demo_rl_skip_ts_95985_1792225931695 { "user_id": 42 }
  POST /api/functions/conc_demo_lock_ts_95985_1792225931695 { "resource": "queue:drain" }

✓ Cleaned up demo functions
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'hn_jZhjJsr_pP2P3vuR2533axLRVO8xfk8fPHtlr3ADdnhzKTOQNIDEzmMqyQqKs1gfydVH3ehTd_qDE_cKU1g'
}

=== Upsert Operation ===
✓ First upsert (update): {
  active: { value: true, type: 'Boolean' },
  id: 'hn_jZhjJsr_pP2P3vuR2533axLRVO8xfk8fPHtlr3ADdnhzKTOQNIDEzmMqyQqKs1gfydVH3ehTd_qDE_cKU1g',
  age: { type: 'Integer', value: 29 },
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  name: { type: 'String', value: 'Alice Johnson' }
}
✓ Second upsert (insert): { id: 'new-user-id' }

=== Find One Operation ===
✓ Found user by email: {
  active: { type: 'Boolean', value: true },
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  name: { type: 'String', value: 'Alice Johnson' },
  id: 'hn_jZhjJsr_pP2P3vuR2533axLRVO8xfk8fPHtlr3ADdnhzKTOQNIDEzmMqyQqKs1gfydVH3ehTd_qDE_cKU1g',
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
✓ Inserted document: G70I8skV9TFoBmdxj0uF2Rx6iyaW0-agAf_-AyRKezxk1v4ZwwoZlIIPEoWJmiNujcPzjYPvf5AKJGRxLV65ow

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Pa3CQrjH6P-YvMx15vMMU7ViZQVIbozA_pqvwVEqlEgzxDK8ARo8vG7WUAeLHET0AdfNhU6CUoqmzNW_WEHycA

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
✓ Edge cache script created: h5kJOvMDdWv1BHqhWvcigUIsMclTop23necoggymNj0WvGDBPKmnKSbJOqMg_RGZp1XIKUwVRuAIkFFavzL2Ag

Call 1: Cache miss (fetches from API)
Response time: 60ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "username": "Bret",
          "website": "hildegard.org",
          "id": 1,
          "company": {
            "bs": "harness real-time e-markets",
            "name": "Romaguera-Crona",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "name": "Leanne Graham",
          "phone": "1-770-736-8031 x56442",
          "address": {
            "street": "Kulas Light",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "zipcode": "92998-3874",
            "suite": "Apt. 556",
            "city": "Gwenborough"
          },
          "email": "Sincere@april.biz"
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
Response time: 3ms (20x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "username": "Bret",
          "website": "hildegard.org",
          "id": 1,
          "company": {
            "bs": "harness real-time e-markets",
            "name": "Romaguera-Crona",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "name": "Leanne Graham",
          "phone": "1-770-736-8031 x56442",
          "address": {
            "street": "Kulas Light",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "zipcode": "92998-3874",
            "suite": "Apt. 556",
            "city": "Gwenborough"
          },
          "email": "Sincere@april.biz"
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
   ⏱️  Duration: 57ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "address": {
        "geo": {
          "lat": "-37.3159",
          "lng": "81.1496"
        },
        "street": "Kulas Light",
        "zip...

Second call (cache hit - from cache):
   ⏱️  Duration: 3ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "address": {
        "geo": {
          "lat": "-37.3159",
          "lng": "81.1496"
        },
        "street": "Kulas Light",
        "zipcode": "92998-3874",
 ...
   🚀 Cache speedup: 19.0x faster!

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

✅ Function saved: n0weVFJnlJLLccUS1KcE16xPKndaPcJsuWJ59GBWTcSSbiVKFe26JSeLFbmvwwu2JRL2XLkNsIK1Q5NfWudVMA
📊 Found 5 active users

📝 Example 2: Parameterized Function

✅ Function saved: AsZp-fIsANd3Af8baJO3OtqcnEUFz8josPmWI4vFab4A0LquACbs-UPCrKaFAvh7UNQ2HbtATLG_H5IIuoFqtQ
📊 Found 3 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: 8DfcBGCR7Le-S6C9osBBa19ER5qhqXOzyvkKZXM0u6ZrEFn9qrPuDCIpHxszBQokMRz-Kd5xxRNbhdF7ehXuDQ
📊 Statistics: 2 groups
   {"count":{"value":5,"type":"Integer"},"status":{"value":"inactive","type":"String"},"avg_score":{"type":"Float","value":50}}
   {"count":{"type":"Integer","value":5},"avg_score":{"type":"Float","value":60},"status":{"value":"active","type":"String"}}

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
   {"count":{"value":5,"type":"Integer"},"avg_price":{"type":"Float","value":367},"category":{"type":"String","value":"Electronics"}}
   {"count":{"type":"Integer","value":3},"avg_price":{"value":365.6666666666667,"type":"Float"},"category":{"type":"String","value":"Furniture"}}
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

1. **Efficient Similarity Search**: They excel in performing nearest neighbor searches in high-dimensional spaces, making it easier to find similar items.

2. **Handling Unstructured Data**: Vector databases can manage unstructured data like text, images, and audio by converting them into vector representations.

3. **Scalability**: They are designed to scale with large datasets and high-dimensional data efficiently.

4. **Performance**: Designed for speed, they can quickly process large volumes of queries and data.

5. **Flexibility**: Support for various machine learning models and algorithms, accommodating diverse applications.

6. **Integration with ML**: They work well with machine learning workflows, facilitating tasks like recommendation systems and natural language processing.

7. **Real-Time Data Processing**: Many vector databases can handle real-time data, making them suitable for dynamic applications.

8. **Multi-modal Capabilities**: Support for different data types, enabling complex applications that utilize various data forms.

Overall, vector databases enhance the handling of complex data and support advanced applications in AI and machine learning.
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

✅ Function saved: 2UDqdfHFomXDAkWo80_hyLxQqaGvR9bP1BoRAlfYjii7WKtf-8sdZ_AaMDJqR3AUpH27eHwHLeHUpjpMy46njw
📊 Found 2 product groups
   {"avg_price":{"value":474,"type":"Float"},"category":{"value":"Furniture","type":"String"},"count":{"type":"Integer","value":2}}
   {"avg_price":{"type":"Float","value":575.6666666666666},"category":{"value":"Electronics","type":"String"},"count":{"value":3,"type":"Integer"}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"count":{"value":3,"type":"Integer"},"category":{"type":"String","value":"Electronics"}}
   {"count":{"value":2,"type":"Integer"},"category":{"type":"String","value":"Furniture"}}
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
   inactive: 3 users
   active: 7 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Function saved
📊 Average score by role:
   {"count":{"value":3,"type":"Integer"},"avg_score":{"type":"Float","value":20},"role":{"type":"String","value":"admin"}}
   {"avg_score":{"value":70,"type":"Float"},"count":{"type":"Integer","value":7},"role":{"value":"user","type":"String"}}
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

✅ Inserted order: o93RXUucQvarXqRTIcyEP7PnYwFUPWxCHXyMYV8quCBA0cICrh_FrG6dBRWlSI27isJTQWlD3DaQT3yCCW-w3w
✅ Inserted 2 products with wrapped types

📝 Example 2: UserFunction with Wrapped Type Parameters

✅ Function saved: i4ZPY3v1h4-VFuMo_kOnoybq97vbMvj0mndbe8rPDs1zvto0-6FhTE-DV0YFoCzDoWVIkE3p_SsmSnNKtlbbDw
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"value":{"userId":"user_abc","role":"admin"},"type":"Object"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: i4q-5hED43vi1-KpTDwYyTft-PkQDZyq-DwMBQvFHGVCHuiHZNvZO_QtVaNfGl3eliidQwCH9GRxhdTtjqfyrw
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: a9QxgwHHTkSi0rfruzbSHaPxlaUFUikFY1altdKr79A9bBt3j45nFp-eB5bg0IiwnbGJ-M6aRi3-TIshmuBDdg
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
   2. Introduction to Machine Learning (AI)
   3. Vector Databases Explained (Database)
   4. Natural Language Processing (AI)
   5. Getting Started with ekoDB (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"category":{"type":"String","value":"Database"},"count":{"type":"Integer","value":3}}
   {"count":{"type":"Integer","value":2},"category":{"type":"String","value":"AI"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Goal Template CRUD Example (TypeScript) ===

--- Creating goal template ---
Created template: Data Migration (id: PMXk48aOwiHxRvMoNmW6zWqxfk4BRazBSFJc48LIVQ_C2tucfa25itZZOhvAWa4PN7JzQXy6_9mlkpPJqkpdSQ)

--- Listing templates ---
Templates: {
  count: 1,
  items: [
    {
      description: [Object],
      id: 'PMXk48aOwiHxRvMoNmW6zWqxfk4BRazBSFJc48LIVQ_C2tucfa25itZZOhvAWa4PN7JzQXy6_9mlkpPJqkpdSQ',
      steps: [Object],
      title: [Object]
    }
  ]
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
Created goal: Deploy v2.0 (id: Y1fpM2jnLhqu-xpos2eagM9Edj2svwMeJJJAcBPfZJ72T9Ky6oY_Lib8B1x5kGrHbz9NMwTPW6djU0SjMxCRYw)

--- Listing goals ---
Goals: {
  "count": 1,
  "goals": [
    {
      "created_at": "2026-10-17T08:32:22.157322+00:00",
      "description": "Ship version 2.0 to production",
      "id": "Y1fpM2jnLhqu-xpos2eagM9Edj2svwMeJJJAcBPfZJ72T9Ky6oY_Lib8B1x5kGrHbz9NMwTPW6djU0SjMxCRYw",
      "status": "pending",
      "steps": "[{\"description\":\"Run test suite\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]",
      "title": "Deploy v2.0",
      "updated_at": "2026-10-17T08:32:22.157322+00:00"
    }
  ]
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
        "value": "2026-10-17T08:32:22.157322+00:00"
      },
      "description": {
        "type": "String",
        "value": "Ship version 2.0 with hot-fix patches"
      },
      "id": "Y1fpM2jnLhqu-xpos2eagM9Edj2svwMeJJJAcBPfZJ72T9Ky6oY_Lib8B1x5kGrHbz9NMwTPW6djU0SjMxCRYw",
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
        "value": "2026-10-17T08:32:22.168896+00:00"
      }
    }
  ]
}

--- Goal step: start step 0 ---
Step 0 started on goal Y1fpM2jnLhqu-xpos2eagM9Edj2svwMeJJJAcBPfZJ72T9Ky6oY_Lib8B1x5kGrHbz9NMwTPW6djU0SjMxCRYw
--- Goal step: complete step 0 ---
Step 0 completed on goal Y1fpM2jnLhqu-xpos2eagM9Edj2svwMeJJJAcBPfZJ72T9Ky6oY_Lib8B1x5kGrHbz9NMwTPW6djU0SjMxCRYw
--- Goal step: fail step 1 ---
Step 1 failed on goal Y1fpM2jnLhqu-xpos2eagM9Edj2svwMeJJJAcBPfZJ72T9Ky6oY_Lib8B1x5kGrHbz9NMwTPW6djU0SjMxCRYw

--- Completing goal ---
Goal status: pending_review
--- Approving goal ---
Goal status after approve: in_progress

--- Creating goal to reject ---
--- Rejecting goal ---
Goal status after reject: failed


--- Creating task ---
Created task: Hourly Health Check (id: Zv_SvXJwjfFxBCdkEKVRRShnYp52bjCcPKKeNcxN7ur_LY4XbhpuprJSIpPZSiYJkB3051Nl3-YsFrFSEtfoUg)

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
      "id": "Zv_SvXJwjfFxBCdkEKVRRShnYp52bjCcPKKeNcxN7ur_LY4XbhpuprJSIpPZSiYJkB3051Nl3-YsFrFSEtfoUg",
      "name": {
        "type": "String",
        "value": "Hourly Health Check"
      }
    }
  ]
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
Created agent: SupportBot (id: GPV131qhGew1_7HbCweRyl-5Y0nFG1d3Xalh19CpbnTrgUTTKlqiLJg180_0-jg13cn0wiWNPEyRzGmlQ4nJfg)

--- Listing agents ---
Agents: {
  "count": 1,
  "items": [
    {
      "deployment_id": {
        "type": "String",
        "value": "deploy_prod_1"
      },
      "id": "GPV131qhGew1_7HbCweRyl-5Y0nFG1d3Xalh19CpbnTrgUTTKlqiLJg180_0-jg13cn0wiWNPEyRzGmlQ4nJfg",
      "llm_model": {
        "type": "String",
        "value": "gpt-4"
      },
      "name": {
        "type": "String",
        "value": "SupportBot"
      },
      "system_prompt": {
        "type": "String",
        "value": "You are a helpful customer support agent."
      }
    }
  ]
}

--- Getting agent by ID ---
Fetched: SupportBot

--- Getting agent by name ---
By name: SupportBot (id: GPV131qhGew1_7HbCweRyl-5Y0nFG1d3Xalh19CpbnTrgUTTKlqiLJg180_0-jg13cn0wiWNPEyRzGmlQ4nJfg)

--- Updating agent ---
Updated agent: SupportBot

--- Getting agents by deployment ---
Agents in deployment: {
  "count": 0,
  "items": []
}
WARNING: agents-by-deployment omitted created agent GPV131qhGew1_7HbCweRyl-5Y0nFG1d3Xalh19CpbnTrgUTTKlqiLJg180_0-jg13cn0wiWNPEyRzGmlQ4nJfg; TODO: check/fix the server-side deployment lookup

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


✅ Join operations examples completed!
=== Cleanup ===
✅ Deleted test collections
✓ Client created
✓ ts_users_register saved
✓ ts_users_login saved
✓ ts_users_verify_token saved

=== Auth flow defined as pure stored functions ===
Call them like:
  POST /api/functions/jwt_register_ts_97411_1792225943256 { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/jwt_login_ts_97411_1792225943256 { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/jwt_verify_ts_97411_1792225943256 { "token": "<jwt>" }

Set JWT_SECRET in ekoDB's environment_vars whitelist before invoking.

✓ Cleaned up demo functions
=== ekoDB KV Document Linking Example (TypeScript) ===

--- Setup: creating KV key and documents ---
Set KV key: session:admin
Inserted document 1: YKqFa_H0Hun4EnfaesvFdHrDiqY9cjU2suBSmpjJHKsomdE4fG7LhfbfnYeXWtI1EjGozu0lKRZxK79P-0px4Q
Inserted document 2: X_y4k3PUvlRLGLGaZxk3tW1W3odAupy9KYoAUOV8VxNz6OO_A7TXbY3DaoFdiBJZKwebCyHuZPyxIczgMsnXkA

--- Linking documents to KV key ---
Linked doc YKqFa_H0Hun4EnfaesvFdHrDiqY9cjU2suBSmpjJHKsomdE4fG7LhfbfnYeXWtI1EjGozu0lKRZxK79P-0px4Q: null
Linked doc X_y4k3PUvlRLGLGaZxk3tW1W3odAupy9KYoAUOV8VxNz6OO_A7TXbY3DaoFdiBJZKwebCyHuZPyxIczgMsnXkA: null

--- Getting links for KV key ---
Links: [
  {
    "collection": "kv_links_example_ts_97441_1792225943605",
    "document_id": "X_y4k3PUvlRLGLGaZxk3tW1W3odAupy9KYoAUOV8VxNz6OO_A7TXbY3DaoFdiBJZKwebCyHuZPyxIczgMsnXkA",
    "field_path": null,
    "created_at": "2026-10-17T08:32:23.683840Z",
    "last_accessed": "2026-10-17T08:32:23.684726Z",
    "metadata": {}
  },
  {
    "collection": "kv_links_example_ts_97441_1792225943605",
    "document_id": "YKqFa_H0Hun4EnfaesvFdHrDiqY9cjU2suBSmpjJHKsomdE4fG7LhfbfnYeXWtI1EjGozu0lKRZxK79P-0px4Q",
    "field_path": null,
    "created_at": "2026-10-17T08:32:23.682712Z",
    "last_accessed": "2026-10-17T08:32:23.684726Z",
    "metadata": {}
  }
]

--- Unlinking document ---
Unlinked doc X_y4k3PUvlRLGLGaZxk3tW1W3odAupy9KYoAUOV8VxNz6OO_A7TXbY3DaoFdiBJZKwebCyHuZPyxIczgMsnXkA: null

--- Verifying remaining links ---
Remaining links: [
  {
    "collection": "kv_links_example_ts_97441_1792225943605",
    "document_id": "YKqFa_H0Hun4EnfaesvFdHrDiqY9cjU2suBSmpjJHKsomdE4fG7LhfbfnYeXWtI1EjGozu0lKRZxK79P-0px4Q",
    "field_path": null,
    "created_at": "2026-10-17T08:32:23.682712Z",
    "last_accessed": "2026-10-17T08:32:23.686604Z",
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
  kv_ops_ts_97471_1792225944047:cache:product:1: success
  kv_ops_ts_97471_1792225944047:cache:product:2: success
  kv_ops_ts_97471_1792225944047:cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  kv_ops_ts_97471_1792225944047:cache:product:1: { price: 29.99, name: 'Product 1' }
  kv_ops_ts_97471_1792225944047:cache:product:2: { name: 'Product 2', price: 39.99 }
  kv_ops_ts_97471_1792225944047:cache:product:3: { name: 'Product 3', price: 49.99 }

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
  kv_ops_ts_97471_1792225944047:cache:product:1: deleted
  kv_ops_ts_97471_1792225944047:cache:product:2: deleted
  kv_ops_ts_97471_1792225944047:cache:product:3: deleted

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
Response: The three primary colors are red, blue, and yellow.

--- Structured Output via SSE ---
JSON response: [
  {
    "name": "Mercury",
    "diameter_km": 4879
  },
  {
    "name": "Venus",
    "diameter_km": 12104
  },
  {
    "name": "Earth",
    "diameter_km": 12742
  }
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello! I hope you're having a wonderful day.

=== Done ===
=== ekoDB Schedule Management Example (TypeScript) ===

--- Creating schedule ---
Created schedule: Nightly Database Backup (id: bd0dca7f-ba59-4621-867d-132afc93ae18, cron: 0 0 2 * * *)

--- Listing schedules ---
Schedules: {
  "count": 1,
  "schedules": [
    {
      "created_at": "2026-10-17T08:32:27.698458Z",
      "cron_expression": "0 0 2 * * *",
      "description": null,
      "enabled": true,
      "function_label": "schedule_noop_typescript_97629_1792225947668",
      "id": "bd0dca7f-ba59-4621-867d-132afc93ae18",
      "last_execution": null,
      "name": "Nightly Database Backup",
      "next_execution": "2026-10-18T02:00:00Z",
      "parameters": {},
      "stats": {
        "avg_execution_time_ms": 0,
        "failed_executions": 0,
        "last_error": null,
        "successful_executions": 0,
        "total_executions": 0
      },
      "timezone": "UTC",
      "updated_at": "2026-10-17T08:32:27.698458Z"
    }
  ]
}

--- Getting schedule ---
Fetched: Nightly Database Backup (cron: 0 0 2 * * *)

--- Updating schedule ---
Updated: Nightly Full Backup (new cron: 0 0 3 * * *)

--- Triggering schedule ---
Trigger response: {
  "schedule_id": "bd0dca7f-ba59-4621-867d-132afc93ae18",
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
  1. Score: 12.870, Matched: name, email
  2. Score: 6.270, Matched: name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 13.200, Matched: bio, title
  2. Score: 13.200, Matched: bio, title
  3. Score: 13.200, Matched: title, bio
  4. Score: 13.200, Matched: title, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400, Matched: bio, title
  2. Score: 26.400, Matched: bio, title
  3. Score: 26.400, Matched: bio, title
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
  1. Score: 0.772, Matched:
  2. Score: 0.748, Matched:
  3. Score: 0.736, Matched:

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.495, Matched: title, content
  2. Score: 0.899, Matched: content, title
  3. Score: 0.309, Matched:

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
  id: 'OE98UdTvYJR9dwfeoJ6VJsaHDnL6vElfAETpdJ3_dGLl5JRRmhnRnL7ne_TclbXXtWyreay675gXcSM5Vic_ZQ'
}

=== Find by ID ===
Found: {
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  name: { type: 'String', value: 'Test Record' },
  value: { type: 'Integer', value: 42 },
  metadata: { type: 'Object', value: { nested: [Object], key: 'value' } },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  id: 'OE98UdTvYJR9dwfeoJ6VJsaHDnL6vElfAETpdJ3_dGLl5JRRmhnRnL7ne_TclbXXtWyreay675gXcSM5Vic_ZQ',
  price: { type: 'Float', value: 99.99 },
  active: { value: true, type: 'Boolean' },
  created_at: { value: '2026-10-17T08:32:29.072+00:00', type: 'DateTime' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-10-17T08:32:29.072Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { nested: { deep: true }, key: 'value' }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  categories: [ 'electronics', 'computers' ],
  name: 'Test Record',
  value: 42,
  metadata: { nested: { deep: true }, key: 'value' },
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  id: 'OE98UdTvYJR9dwfeoJ6VJsaHDnL6vElfAETpdJ3_dGLl5JRRmhnRnL7ne_TclbXXtWyreay675gXcSM5Vic_ZQ',
  price: 99.99,
  active: true,
  created_at: '2026-10-17T08:32:29.072+00:00',
  tags: [ 'tag1', 'tag2', 'tag3' ],
  data: 'aGVsbG8gd29ybGQ='
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  value: { type: 'Integer', value: 100 },
  price: { value: 99.99, type: 'Float' },
  active: { type: 'Boolean', value: true },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  created_at: { type: 'DateTime', value: '2026-10-17T08:32:29.072+00:00' },
  id: 'OE98UdTvYJR9dwfeoJ6VJsaHDnL6vElfAETpdJ3_dGLl5JRRmhnRnL7ne_TclbXXtWyreay675gXcSM5Vic_ZQ',
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  name: { value: 'Updated Record', type: 'String' },
  metadata: { type: 'Object', value: { nested: [Object], key: 'value' } }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: zIPBXwxvxtlwqU1-0oR0lDVv9H914ZzC981hNRr8LYFeEas-usQn_P-cB_baGj87qwRQlgf8aHmeOLDWEMvIhQ

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
✓ Created native SWR script: github_user_native_ts (rYiU9Xs-JcVIzz0g3LemLERY-qYbJo1SFImuExC55hA5Up4z9GOELQMX3_RD3EabjO-nVn2o-tIZ0xL-M6h-Zw)

First call (cache miss - will fetch from GitHub API):
  Response time: 148ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 2ms
  Speedup: 74.0x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_ts (uduxbHs6hdJKETJw_ltSjUD1XIuVsInqVP6rgCkV86B5ESTpHZFNSVKHNJEEFKtEgPp4lYARtb8FpxcwHrnVmg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_ts' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_ts (pFDPgB30Thzh_xGs_MfO1HRQa85RQOiq3qKakU9HCDK1Vq-6quyX4uKhfPMoYx4pF90bHpazoKdE7FpL8IeiAw)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_ts' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_ts (9rjrMBIDamDiDP_yF9fj5O-S2dABp-PjKzvpHVeXSnO1u7b1jBcoU0YkmV01ov1JyG-4WOI67jjZW0ZuzenNlw)
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
✓ Created SWR script: fetch_api_user_ts_97810_1792225951079 (AEfWhpmGvCJVEbdlDPEhO-ZZQYZhy392ZFBwahHswmh6Fy_rh6aILrVrVLd-L34BFhtz0Zt7m28FYdp8jRrzoA)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "address": {
            "city": "Gwenborough",
            "zipcode": "92998-3874",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "street": "Kulas Light",
            "suite": "Apt. 556"
          },
          "phone": "1-770-736-8031 x56442",
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "id": 1,
          "name": "Leanne Graham",
          "website": "hildegard.org",
          "email": "Sincere@april.biz",
          "username": "Bret"
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
          "address": {
            "city": "Gwenborough",
            "zipcode": "92998-3874",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "street": "Kulas Light",
            "suite": "Apt. 556"
          },
          "phone": "1-770-736-8031 x56442",
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "id": 1,
          "name": "Leanne Graham",
          "website": "hildegard.org",
          "email": "Sincere@april.biz",
          "username": "Bret"
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
✓ Created enrichment script: fetch_product_reviews_ts_97810_1792225951079 (fbjqZ_Ubol2bR-kNphksBelYGwqNGD-mhvGUPxvAS_zg65LdBEhpnlUtiwCRjT18iydM5gT64XfrV_g7KacxJg)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "value": {
          "availabilityStatus": "In Stock",
          "discountPercentage": 10.48,
          "price": 9.99,
          "dimensions": {
            "depth": 22.99,
            "height": 13.08,
            "width": 15.14
          },
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "warrantyInformation": "1 week warranty",
          "weight": 4,
          "reviews": [
            {
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "date": "2025-04-30T09:41:02.053Z",
              "rating": 3,
              "comment": "Would not recommend!",
              "reviewerName": "Eleanor Collins"
            },
            {
              "date": "2025-04-30T09:41:02.053Z",
              "rating": 4,
              "reviewerName": "Lucas Gordon",
              "reviewerEmail": "lucas.gordon@x.dummyjson.com",
              "comment": "Very satisfied!"
            },
            {
              "reviewerName": "Eleanor Collins",
              "rating": 5,
              "comment": "Highly impressed!",
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com"
            }
          ],
          "rating": 2.56,
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "sku": "BEA-ESS-ESS-001",
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "shippingInformation": "Ships in 3-5 business days",
          "brand": "Essence",
          "meta": {
            "updatedAt": "2026-05-23T11:27:41.868Z",
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
            "barcode": "5784719087687",
            "createdAt": "2025-10-09T14:47:01.588Z"
          },
          "minimumOrderQuantity": 48,
          "category": "beauty",
          "id": 1,
          "returnPolicy": "No return policy",
          "stock": 99,
          "tags": [
            "beauty",
            "mascara"
          ],
          "title": "Essence Mascara Lash Princess"
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

✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 6rC1jhdo-bj5bxWnbz3OCigL80_5p2y44K9U6qhmizzksiVIGbWNNiooLbY9rhlOj7BTcL2eLHJfMNNOfrWI3Q
Created Bob: $500 - ID: kd-VAMszURHRpo2BNlFD-VYl7xtaj9xrgblUrMu-M9x57bKVUqVMN8xHYeqVvLbXG-ElieU3tMYl-LeS5OLEAg

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 504e41cb-b85c-4bd5-aec8-00019c93f1c8

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
New transaction: ecbf7dcc-3de7-4628-9c24-5346198b612e
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Create User Function ===
Created user function with ID: z2Cv59qZrXz10c3NUG6t6Run6razRF0RpuLGZcUcag8Tg0AcgMuG7AlmhD0oIZp8wM9b1-3WshoKx-ZwFFKA1g

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 5 user functions:
  - get_active_users_client_ts_updated: Get Active Users (Updated)
  - fetch_product_reviews_ts_97810_1792225951079: Fetch Product with Reviews (Multi-API)
  - conc_demo_rl_fail_ts_95985_1792225931695: Rate-limit (fail mode)
  - conc_demo_rl_skip_ts_95985_1792225931695: Rate-limit (skip mode)
  - get_active_users_ts: Get Active Users

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
=== WebSocket Chat Streaming Example (TypeScript) ===

Created chat session: jzVqEtE_pityYIS__MgjShVrSHDJQEMsrdMMlrZKrmSQB8UExxqouGDvyJL98pxfDlMQ3bzRefAfNj8Oz7TF2g

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: bCQ0poi4GfYQ4hk_TR4cCFFJg1FmS6u1Uas4Euv7cQ6araNAglKIec7QzxDQbBatIQr02bNj82RF31spQopEzg
Execution time: 610ms
Token usage: {"completion_tokens":8,"prompt_tokens":15,"total_tokens":23}

Full response: {"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris....
=== WebSocket Subscription Example ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_ts' ===
✓ Subscribed (subscription_id: sub_7697274054a64e5c9574e693b98054c6)

=== Performing mutations to trigger notifications ===
Inserting a record...
✓ Inserted record: H9fTKTg_3esA7zBUWqt6s0VBD_lRpVedXq3QE9HymyR2YsQuSlvB1HCajV3veQ8i9wBIhLwT8tXfg-fVrPGRKw
  📡 Notification received for H9fTKTg_3esA7zBUWqt6s0VBD_lRpVedXq3QE9HymyR2YsQuSlvB1HCajV3veQ8i9wBIhLwT8tXfg-fVrPGRKw

Inserting another record...
✓ Inserted record: 8Gka_0junEvCEsT3R4TOCdqM5Tl4ahYXYPzHZgP6FPDuRR3aJ9TmELlyLNE5rVAIq05utoAr3QYbzSJHzvOvUQ
  📡 Notification received for 8Gka_0junEvCEsT3R4TOCdqM5Tl4ahYXYPzHZgP6FPDuRR3aJ9TmELlyLNE5rVAIq05utoAr3QYbzSJHzvOvUQ

=== Unsubscribing ===
✓ Unsubscribed: {"collection":"ws_subscribe_example_ts","found":true,"unsubscribed":true}

✓ WebSocket subscription example completed successfully
✓ Deleted collection 'ws_subscribe_example_ts'
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: JRzbLOWTboCFTDn2eg1XJH2Fpo8pNZgmE0QRk5FzT3aiX07NlqKdQEz9YKr_8y3BzsL0FCUrQ2cKZ2SUSt1-Gw

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
   Inserted with ripple: {"id":"1K3iWdsgrzaW2IiJz4KsBrAs-GbGcMQIekodRxo2BQE5uCT0nLwxsPUgO-fdhHYqu5C6SttMPAYoTuNYGvrpqA"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"BtLB5dSQMhoFNZLk7N7DZYyGjiPwuF3q1U8FleMx2XSdVZzM10HB-6k4NFTBQKQkupUMEIdRpQ4lSAuwyM4NFw"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"price":{"value":150,"type":"Integer"},"name":{"type":"String","value":"Product 1"},"id":"1K3iWdsgrzaW2IiJz4KsBrAs-GbGcMQIekodRxo2BQE5uCT0nLwxsPUgO-fdhHYqu5C6SttMPAYoTuNYGvrpqA"}

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
  Fields returned: ["email","name","id"]
  First user: Dave Brown <dave@example.com>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["created_at","bio","name","status","user_role","id","age","email","avatar_url"]

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
    - Fields: ["bio","avatar_url","api_key","password","id","user_role","name","created_at","status","email","age","secret_token"]
  Projected query:
    - 3 fields per record
    - Fields: ["name","email","id"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
✅ All TypeScript integration tests complete!
🧪 Running JavaScript examples (direct HTTP/WebSocket)...

added 1 package, removed 1 package, and audited 9 packages in 399ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities

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
  id: '2KXR9L4e3MGW2LSKTScyzN7fKGC6ETzaXfzdBBQNBgwODUrd20_4X7NbJ9-2imog3CjeNOUcGZUc9y6aNSGa0w'
}

=== Find by ID ===
Found: {
  name: { type: 'String', value: 'Test Record' },
  value: { value: 42, type: 'Integer' },
  id: '2KXR9L4e3MGW2LSKTScyzN7fKGC6ETzaXfzdBBQNBgwODUrd20_4X7NbJ9-2imog3CjeNOUcGZUc9y6aNSGa0w',
  active: { value: true, type: 'Boolean' }
}

=== Find with Query ===
Found documents: [
  {
    active: { value: true, type: 'Boolean' },
    name: { value: 'Test Record', type: 'String' },
    value: { value: 42, type: 'Integer' },
    id: '2KXR9L4e3MGW2LSKTScyzN7fKGC6ETzaXfzdBBQNBgwODUrd20_4X7NbJ9-2imog3CjeNOUcGZUc9y6aNSGa0w'
  }
]

=== Update Document ===
Updated: {
  active: { type: 'Boolean', value: true },
  value: { value: 100, type: 'Integer' },
  name: { type: 'String', value: 'Updated Record' },
  id: '2KXR9L4e3MGW2LSKTScyzN7fKGC6ETzaXfzdBBQNBgwODUrd20_4X7NbJ9-2imog3CjeNOUcGZUc9y6aNSGa0w'
}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ simple_crud.js completed successfully

=== Running ekoDB/ekodb-client/examples/javascript/simple_websocket.js ===
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: QoFraBADew3StK1_yKjjOrxb8VvF1iTMsCkOwuiBnjLfS2mydY-lF5It8Ciw7jSt_yt4Q2gsZv7P6kFjkPoT9w

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
        "active": {
          "value": true,
          "type": "Boolean"
        },
        "id": "QoFraBADew3StK1_yKjjOrxb8VvF1iTMsCkOwuiBnjLfS2mydY-lF5It8Ciw7jSt_yt4Q2gsZv7P6kFjkPoT9w",
        "name": {
          "value": "WebSocket Test Record",
          "type": "String"
        }
      }
    ]
  },
  "messageId": "1792225957658"
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

✅ Function saved: 3gLdiSDW9KYdRRKhejGhKUSJyQglanQnI9C0iJVnuW97HAer4yLBdKtc7bSgB8PLzBfGH8XTHB7XMRvHXMvPrQ
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: OqJTxdgo5b_b5iB2TXzZM1L1ZbPRMaztoh6_1HqEX0ht9cRSZf2kbl6-JnWJpx6aU6hPnI7QDsYBnYPYKQ_nqA
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: 6Why51EzmoAqhBUZV8GmH_t85Y-ohV1BEVKmIYUbt7zaU3tDHsSBfV1yhRC6RqaarLiz7m3fgUJOlO1imzcKcw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":{"value":60,"type":"Float"},"count":{"type":"Integer","value":5},"status":{"type":"String","value":"active"},"max_score":{"type":"Integer","value":100}}
   {"status":{"value":"inactive","type":"String"},"avg_score":{"type":"Float","value":50},"count":{"value":5,"type":"Integer"},"max_score":{"type":"Integer","value":90}}

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
Retrieved value: { value: { userId: 123, username: 'john_doe' }, type: 'Object' }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { type: 'Object', value: { price: 29.99, name: 'Product 1' } }
cache:product:2: {
  value: { name: 'Product 2', price: 39.989999999999995 },
  type: 'Object'
}
cache:product:3: {
  type: 'Object',
  value: { price: 49.989999999999995, name: 'Product 3' }
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
Collection created with first record: x07LJVXhVef1w23l_hBFU6bRxXWp7QqNirfLSG1j-NOqKphxVGHeKlIvt0Iyu2fv2Au4ukMqKC3DGgKbeFq4YQ

=== List Collections ===
Total collections: 22
Sample collections: [
  'chat_goals__ek0_testing',
  'chat_messages__ek0_testing',
  'schedules__ek0_testing',
  'schema_documents_client_go',
  'schema_users_client_ts'
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
Created Alice: $1000 - ID: vTzvW_tkn7YBBKfStua7IxbuD19ybsM1vUFXuVWqEJDAQyXlJuZOWwoALxS_cKMvMOAowPvqb5Dhat7WKL7IaQ
Created Bob: $500 - ID: cM85IH0Zq4a5j2mh9iK_suVqC0NIlO1g-0PgfWYl2-G8ou328z-FVoGjFjrtbHHcc4aYdjIxG1j52m2hQbaDOg

=== Example 1: Begin Transaction ===
Transaction ID: 2f06c7dc-5ba9-46bb-b5f5-801245704a23

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
Bob: {"value":700,"type":"Integer"}

=== Example 5: Rollback ===
New transaction: a3c9b5d3-140c-41dc-a662-1be51e20d870
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"type":"Integer","value":700}

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
   ✅ Function saved: _ZTAkPcDp4jJ7CVhoHwtXYJZRCR29D7QOixunDzY_M-qA5tQ8tXwdBk5Ee1NlMMDYwX8psjKHRPHE1t6yBs4OA

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
   ✅ Function saved: p7-L7FNDHbhCY9UPB0ubl_tkG0ljjcOccLafVZsflpd_YAwcPz5CaFnB3g12k6SE-9Q8QLh6euRXBumiAUugHw

2️⃣ Calling Function (Query + Update + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {"type":"String","value":"active"}
   📋 Name: {"type":"String","value":"Alice Smith"}

============================================================
📝 Function 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: G4ElS-VCwB7dKPGhdw0I_XFFwTVzkeVxQ57isOtAdW9NKxTc_ZUokqOgOjkLX5dE5Y3Gn030MzhPYSBVKCvIZA

2️⃣ Calling Function (Query + Update Credits + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"type":"Integer","value":100}
   📋 Status: {"type":"String","value":"active"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: h1Y6GcRppQdT6sKIYEq-CFpoOhdFEFCRFiHhRAdMGJmQdoU0DEjGVxediXG767BG8vLnBYl-S3XsqqILCYx5BQ

2️⃣ Calling Function (Query + Delete + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted function: _ZTAkPcDp4jJ7CVhoHwt...
   ✅ Deleted function: p7-L7FNDHbhCY9UPB0ub...
   ✅ Deleted function: G4ElS-VCwB7dKPGhdw0I...
   ✅ Deleted function: h1Y6GcRppQdT6sKIYEq-...
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
  Output: Document ID = w5XLz9KUKzppsf7h6I55Sp4TRBIIuaNbBTJH4FI9qHU_zNPGhQqD5J0UVjMvA3ltDUt9RVqHZasmt64l1sUjqg
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(w5XLz9KUKzppsf7h6I55Sp4TRBIIuaNbBTJH4FI9qHU_zNPGhQqD5J0UVjMvA3ltDUt9RVqHZasmt64l1sUjqg)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(w5XLz9KUKzppsf7h6I55Sp4TRBIIuaNbBTJH4FI9qHU_zNPGhQqD5J0UVjMvA3ltDUt9RVqHZasmt64l1sUjqg)
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
  Output: Document ID = UYpXad-QyOwOgh242Fd_UBE3FrNURCVIJvPGsOupWk7xfPkmXL5fu3HmORYYb8Jh4mStmzs5qjwmcnEUBf82lA
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(UYpXad-QyOwOgh242Fd_UBE3FrNURCVIJvPGsOupWk7xfPkmXL5fu3HmORYYb8Jh4mStmzs5qjwmcnEUBf82lA)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(UYpXad-QyOwOgh242Fd_UBE3FrNURCVIJvPGsOupWk7xfPkmXL5fu3HmORYYb8Jh4mStmzs5qjwmcnEUBf82lA)
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

> @ekodb/ekodb-client@0.30.0 prepare
> npm run build


> @ekodb/ekodb-client@0.30.0 build
> tsc


up to date, audited 44 packages in 542ms

12 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.30.0 build
> tsc

✅ TypeScript client built!

added 1 package, removed 1 package, and audited 9 packages in 431ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities

added 1 package, removed 1 package, and audited 13 packages in 406ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities

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
✓ Created session: jtvZnpbTN4ybjXOVWSnRpPnbi3jvDGL_Ros721XYqeGezgV1c3CTnNridYkk_NPjnTodwc0hQmUuhUu3ftg52A

=== Sending Initial Message ===
✓ Message sent
  Response: There are currently no products available in the database based on the specified criteria. If you need further assistance or wish to check other filters, please let me know!

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
✓ Created second session: DrJ4ZIDt2xwZa45L_IpU6J29uYaaPEDgofrxfxcYcTzKKKQ7kCsg4KoEoqfzugdqy-NFAFTTJQ93eQqyx1i1mw
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
✓ Created session: 2G8zI5Qec947apAxSMahTPvNROQmLlmErC14jKw5yoMJlmvqFfsWEUEE_XVochXDxKO_mS_QS7Dy1GzcR02rCw

=== Sending Chat Message ===
Message ID: FY2wP7Ew9QO9G5D63FbhN__ZrSvdtHJVWiLCsXrp6Kfb61Kqvpoc7Byu1VolgCmhlfue5w9G98bZ5TYf_t_5Gw

=== AI Response ===
Here are the products available along with their prices:

1. **ekoDB Cloud**: $499
   - Description: Fully managed cloud database service product

2. **ekoDB**: $99
   - Description: A high-performance database product with AI capabilities

3. **ekoDB Pro**: $299
   - Description: Enterprise edition product with advanced features

If you would like to know more about any specific product, feel free to ask!

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    price: 499,
    description: 'Fully managed cloud database service product',
    id: 'Ug9dxlk51XkuJqAyFvcVcqN9QXes-_u2VNjl5RGhmbCUiPB_VpEnhap9aCSTpLROT0lCpeTahOqwFAAQ7Vls4w',
    name: 'ekoDB Cloud'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    id: 'aH7L1YGNo82RA4AXBgcwQmQVVlzwo2hycageaRCOndnl43DZ21DNVVYG1nSopYXQELY7OAh9oJjxztKSzCrnaQ',
    description: 'A high-performance database product with AI capabilities',
    name: 'ekoDB',
    price: 99
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    description: 'Enterprise edition product with advanced features',
    name: 'ekoDB Pro',
    price: 299,
    id: 'c0Ta9InZGFo6NV-Df8y8D0Y64GvckkGU34WbXlDpOTeXnM9zfvqPESFkusP5KPZJg-h68-0b8_fF7dJPxyXdRw'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2382ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 102
Total tokens: 3515

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
    - gpt-5.4-mini
    - o1-pro
    - gpt-5.1-codex
    - tts-1
    - gpt-5-mini
    - gpt-5-mini-2025-08-07
    - gpt-4-turbo-2024-04-09
    - gpt-realtime-2.1-mini
    - gpt-realtime-2025-08-28
    - gpt-4
    - tts-1-hd
    - gpt-realtime-1.5
    - gpt-realtime
    - gpt-5.2-pro
    - gpt-5-chat-latest
    - gpt-5.5-pro
    - gpt-audio-2025-08-28
    - gpt-4o-mini-transcribe-2025-03-20
    - omni-moderation-2024-09-26
    - gpt-live-transcribe
    - gpt-5.6-luna
    - omni-moderation-latest
    - o4-mini-2025-04-16
    - gpt-4o-2024-11-20
    - gpt-5.1
    - whisper-1
    - gpt-5-pro
    - gpt-realtime-whisper
    - gpt-5-2025-08-07
    - gpt-5.4-mini-2026-03-17
    - babbage-002
    - gpt-4o-2024-05-13
    - text-embedding-3-large
    - chat-latest
    - gpt-realtime-translate
    - gpt-5.4-pro-2026-03-05
    - gpt-realtime-2
    - gpt-5-search-api
    - gpt-5.5-2026-04-23
    - tts-1-1106
    - gpt-5.5-pro-2026-04-23
    - gpt-audio-mini-2025-10-06
    - gpt-5.2-pro-2025-12-11
    - o3-mini-2025-01-31
    - gpt-audio-1.5
    - gpt-image-1.5
    - gpt-4-turbo
    - gpt-4o-2024-08-06
    - o3
    - gpt-4o-mini-search-preview-2025-03-11
    - gpt-4o-mini-transcribe-2025-12-15
    - gpt-4.1-mini
    - gpt-4o-mini
    - gpt-4.1-nano
    - gpt-image-2
    - gpt-3.5-turbo-16k
    - text-embedding-ada-002
    - gpt-realtime-2.1
    - gpt-4.1
    - gpt-5.5
    - gpt-4o-transcribe-diarize
    - gpt-5-codex
    - gpt-4-0613
    - gpt-5.1-2025-11-13
    - gpt-4o-search-preview
    - o1-2024-12-17
    - gpt-5-pro-2025-10-06
    - gpt-5.6-sol
    - gpt-4o-mini-tts-2025-12-15
    - gpt-image-1
    - gpt-audio
    - gpt-5.3-codex
    - gpt-5-search-api-2025-10-14
    - gpt-realtime-mini
    - gpt-image-1-mini
    - gpt-5.6-terra
    - gpt-3.5-turbo-instruct-0914
    - sora-2
    - gpt-5.4-pro
    - gpt-4o-mini-tts
    - gpt-realtime-mini-2025-12-15
    - gpt-5.1-codex-max
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
    - gpt-5.1-chat-latest
    - tts-1-hd-1106
    - gpt-5.2-codex
    - gpt-4o
    - o4-mini-deep-research
    - gpt-5.4
    - gpt-4o-search-preview-2025-03-11
    - text-embedding-3-small
    - gpt-5-nano-2025-08-07
    - gpt-4o-transcribe
    - o4-mini
    - gpt-5.1-codex-mini
    - gpt-image-2.5-sunburst-2026-09-08
    - gpt-5.3-chat-latest
    - o3-2025-04-16
    - gpt-5.2-2025-12-11
    - chatgpt-image-latest
    - gpt-3.5-turbo
    - gpt-3.5-turbo-1106
    - gpt-4o-mini-tts-2025-03-20
    - gpt-5.2
    - gpt-5.4-nano-2026-03-17
    - o1
    - davinci-002
    - gpt-5.4-nano
    - o3-mini
    - o4-mini-deep-research-2025-06-26
    - gpt-audio-mini-2025-12-15
    - o1-pro-2025-03-19
    - gpt-6-luna
    - gpt-image-2.5-flare
    - gpt-6-astra
    - gpt-audio-mini
    - gpt-4.1-mini-2025-04-14
    - gpt-image-2.5-sunburst
    - gpt-image-2.5-flare-2026-09-08
    - gpt-4o-mini-search-preview
    - gpt-6-sol
    - gpt-live-1
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
    - veo-3.1-generate-preview
    - veo-3.1-fast-generate-preview
    - veo-3.1-lite-generate-preview
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
  gemini: ok 63 models
  openai: ok 135 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get Specific Provider Models ===
OpenAI models: gpt-3.5-turbo-0125, gpt-5.4-2026-03-05, gpt-5-nano, gpt-5.4-mini, o1-pro, gpt-5.1-codex, tts-1, gpt-5-mini, gpt-5-mini-2025-08-07, gpt-4-turbo-2024-04-09, gpt-realtime-2.1-mini, gpt-realtime-2025-08-28, gpt-4, tts-1-hd, gpt-realtime-1.5, gpt-realtime, gpt-5.2-pro, gpt-5-chat-latest, gpt-5.5-pro, gpt-audio-2025-08-28, gpt-4o-mini-transcribe-2025-03-20, omni-moderation-2024-09-26, gpt-live-transcribe, gpt-5.6-luna, omni-moderation-latest, o4-mini-2025-04-16, gpt-4o-2024-11-20, gpt-5.1, whisper-1, gpt-5-pro, gpt-realtime-whisper, gpt-5-2025-08-07, gpt-5.4-mini-2026-03-17, babbage-002, gpt-4o-2024-05-13, text-embedding-3-large, chat-latest, gpt-realtime-translate, gpt-5.4-pro-2026-03-05, gpt-realtime-2, gpt-5-search-api, gpt-5.5-2026-04-23, tts-1-1106, gpt-5.5-pro-2026-04-23, gpt-audio-mini-2025-10-06, gpt-5.2-pro-2025-12-11, o3-mini-2025-01-31, gpt-audio-1.5, gpt-image-1.5, gpt-4-turbo, gpt-4o-2024-08-06, o3, gpt-4o-mini-search-preview-2025-03-11, gpt-4o-mini-transcribe-2025-12-15, gpt-4.1-mini, gpt-4o-mini, gpt-4.1-nano, gpt-image-2, gpt-3.5-turbo-16k, text-embedding-ada-002, gpt-realtime-2.1, gpt-4.1, gpt-5.5, gpt-4o-transcribe-diarize, gpt-5-codex, gpt-4-0613, gpt-5.1-2025-11-13, gpt-4o-search-preview, o1-2024-12-17, gpt-5-pro-2025-10-06, gpt-5.6-sol, gpt-4o-mini-tts-2025-12-15, gpt-image-1, gpt-audio, gpt-5.3-codex, gpt-5-search-api-2025-10-14, gpt-realtime-mini, gpt-image-1-mini, gpt-5.6-terra, gpt-3.5-turbo-instruct-0914, sora-2, gpt-5.4-pro, gpt-4o-mini-tts, gpt-realtime-mini-2025-12-15, gpt-5.1-codex-max, gpt-4o-mini-2024-07-18, gpt-4.1-2025-04-14, gpt-5, gpt-4o-mini-transcribe, gpt-image-2-2026-04-21, gpt-5.2-chat-latest, sora-2-pro, gpt-3.5-turbo-instruct, gpt-transcribe, gpt-4.1-nano-2025-04-14, gpt-5.1-chat-latest, tts-1-hd-1106, gpt-5.2-codex, gpt-4o, o4-mini-deep-research, gpt-5.4, gpt-4o-search-preview-2025-03-11, text-embedding-3-small, gpt-5-nano-2025-08-07, gpt-4o-transcribe, o4-mini, gpt-5.1-codex-mini, gpt-image-2.5-sunburst-2026-09-08, gpt-5.3-chat-latest, o3-2025-04-16, gpt-5.2-2025-12-11, chatgpt-image-latest, gpt-3.5-turbo, gpt-3.5-turbo-1106, gpt-4o-mini-tts-2025-03-20, gpt-5.2, gpt-5.4-nano-2026-03-17, o1, davinci-002, gpt-5.4-nano, o3-mini, o4-mini-deep-research-2025-06-26, gpt-audio-mini-2025-12-15, o1-pro-2025-03-19, gpt-6-luna, gpt-image-2.5-flare, gpt-6-astra, gpt-audio-mini, gpt-4.1-mini-2025-04-14, gpt-image-2.5-sunburst, gpt-image-2.5-flare-2026-09-08, gpt-4o-mini-search-preview, gpt-6-sol, gpt-live-1, gpt-6.1-sol

=== Get Anthropic Models ===
Anthropic models: claude-haiku-5-5, claude-sonnet-5-5, claude-opus-5-5, claude-fable-5-1, claude-opus-5, claude-sonnet-5, claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 404: {"error":"Unknown provider 'nonexistent_provider_xyz'","error_kind":"unknown_provider"}

✓ Chat Models example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: rrB0uUthp4hO7dVeXzsOPcPY_U0K6pXmVnRPhC6mYOGpB8Bm366Ho3W760pdAOG1G-45MjIyP9MuC4igoooWnw

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** A high-performance database product
- **Price:** $99

If you need more information or have any specific questions about ekoDB, feel free to ask!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: w81Pm8v8IdOsPM-gtTxpIiaEjZ_kwihC4k5PDYxasPjM_wlK8k7XgRjTI786aRgtDKzYyeRG10OshbEvjrjGBA
  Parent: rrB0uUthp4hO7dVeXzsOPcPY_U0K6pXmVnRPhC6mYOGpB8Bm366Ho3W760pdAOG1G-45MjIyP9MuC4igoooWnw

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: w81Pm8v8IdOsPM-gtTxpIiaEjZ_kwihC4k5PDYxasPjM_wlK8k7XgRjTI786aRgtDKzYyeRG10OshbEvjrjGBA (Untitled)
  Session 2: rrB0uUthp4hO7dVeXzsOPcPY_U0K6pXmVnRPhC6mYOGpB8Bm366Ho3W760pdAOG1G-45MjIyP9MuC4igoooWnw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: w81Pm8v8IdOsPM-gtTxpIiaEjZ_kwihC4k5PDYxasPjM_wlK8k7XgRjTI786aRgtDKzYyeRG10OshbEvjrjGBA

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: aMooFFIlmZ2H5sMALY6GSkgwMPT_SmE5HLk7OpXZ8eGdP-jRflWt9hQSrSu1iUqLICzgBhaoiKkFNnVlvfy30w

=== List Collections ===
Total collections: 23
Sample collections: chat_goals__ek0_testing,test_accounts,chat_messages__ek0_testing,schedules__ek0_testing,schema_documents_client_go

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: sg6Pc1r_bcAbXFwPHOxRjF3ICSgcfJJGcnpaq5GHVrqgEmJCtdoOwbPgy8XaMLaWYmkZU8tbyldT-13ZnXtJow

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 4z_edSERniBKfmsfPJzw0apOsp6t6ZZgFCSq9D4cM9zimB51iMPSnAnLkPBza5WxQdIl0MKvMbACfDcf_-rrNw

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
✓ Edge cache script created: qZFKzs_o-iVoqwGH95fQxZ0YEufAClANojKKDWe5eCeCA6FLv3oLTm7ycHa_fQzj184_uoD51g8SbuhRnQxqSA

Call 1: Cache miss (fetches from API)
Response time: 463ms
Result: {
  "records": [
    {
      "value": {
        "value": {
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "°C",
            "time": "iso8601"
          },
          "latitude": 40.710335,
          "utc_offset_seconds": 0,
          "elevation": 32,
          "generationtime_ms": 0.02646446228027344,
          "timezone_abbreviation": "GMT",
          "longitude": -73.99308,
          "current": {
            "temperature_2m": 11.4,
            "time": "2026-10-17T08:30",
            "interval": 900
          },
          "timezone": "GMT"
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
Response time: 12ms (38.6x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "°C",
            "time": "iso8601"
          },
          "latitude": 40.710335,
          "utc_offset_seconds": 0,
          "elevation": 32,
          "generationtime_ms": 0.02646446228027344,
          "timezone_abbreviation": "GMT",
          "longitude": -73.99308,
          "current": {
            "temperature_2m": 11.4,
            "time": "2026-10-17T08:30",
            "interval": 900
          },
          "timezone": "GMT"
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
   ⏱️  Duration: 59ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "id": 1,
      "company": {
        "catchPhrase": "Multi-layered client-server neural-net",
        "name": "Romaguera-Crona",
        "bs": ...

Second call (cache hit - from cache):
   ⏱️  Duration: 2ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "id": 1,
      "company": {
        "catchPhrase": "Multi-layered client-server neural-net",
        "name": "Romaguera-Crona",
        "bs": "harness real-time e-m...
   🚀 Cache speedup: 29.5x faster!

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

✅ Function saved: iLIhjtdM3L3cVo2HGbx8fWS4K9co887nScKM-t-3ZYTJZ2o5biTQxwk_uGM3TZcqqoR3FszPWv2BWqzy7gH9Bg
📊 Found 5 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

✅ Function saved
📊 Found 3 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 2 groups
   {"avg_score":{"value":50,"type":"Float"},"status":{"type":"String","value":"inactive"},"count":{"value":5,"type":"Integer"}}
   {"avg_score":{"value":60,"type":"Float"},"status":{"type":"String","value":"active"},"count":{"type":"Integer","value":5}}
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
   1. Wireless Mouse - $29 (⭐4.5)
   2. Standing Desk - $599 (⭐4.7)
   3. Keyboard - $89 (⭐4.4)
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
   1. Precise: Vector data is more precise since it uses specific coordinates for representing locations.

2. Scalability: Vector data can be easily scaled without losing its quality which promotes clear visibility at all zoom levels.

3. Less Disk Space: Vector data typically requires less disk space as compared to raster data.

4. Accurate Geographic Data: They provide an accurate representation of geographic features such as roads, buildings, etc.

5. Advanced Analysis: They facilitate complex spatial and network analysis including shortest path, buffer generation, etc.

6. Rich Attributes: Vector data can hold a large number of attribute data such as name, category, status, etc. for each individual feature.

7. Data Limit: There is no limit on the level of detail that can be represented in vector data.

8. Flexible: It's versatile for creating different types of maps as vector data can be segregated by attribute to create specific views.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding function saved
📊 Generated embeddings for 3 articles
   1. "Advanced Query Patterns" - 1536D vector
   2. "Getting Started with ekoDB" - 1536D vector
   3. "Draft Article" - 1536D vector
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

✅ Function saved: uMqgbx91IlAP4X-oc-Vfg0jik4E_Z_DPg-3GV5jYhiAfTPMvv2_7Ut2iBHe5OCOGzrABKZSV_gC3ok_cfRTovw
📊 Found 2 product groups
   {"count":{"value":3,"type":"Integer"},"avg_price":{"value":575.6666666666666,"type":"Float"},"category":{"type":"String","value":"Electronics"}}
   {"category":{"value":"Furniture","type":"String"},"avg_price":{"value":474,"type":"Float"},"count":{"value":2,"type":"Integer"}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"category":{"type":"String","value":"Electronics"},"count":{"type":"Integer","value":3}}
   {"category":{"type":"String","value":"Furniture"},"count":{"type":"Integer","value":2}}
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
   2. User 9 - Score: 90
   3. User 7 - Score: 70
   4. User 10 - Score: 100
   5. User 5 - Score: 50
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

✅ Inserted order: BVh8IDdGVosticU4VBtZRQFnuu_d26WpHPCC-MnN5DFlYioMbeiI0K-gCVklVZc_uEP_ZOvvhe6RVAn-NTLwbQ
✅ Inserted 2 products with wrapped types

📝 Example 2: Function with Wrapped Type Parameters

✅ Function saved: l2du5C3E8XHWr-tqmJJ3okJY4SC7N0IHn6TegZ5z5k4smG5SqSY6TGzEjaQUml4GbQKlJCSQO1SINCulWxpmvA
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"value":{"userId":"user_abc","role":"admin"},"type":"Object"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: xngoGzePQwLt9-o1oi3-k7FHwSNvJljg5rORzvDqvS_-_JBVaUTPk25D8H4MxFPoTABwoZE4D7-8-L3mtPyLpw
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: 1ICBQGfagWtYmQQ2tqC4qi80RykdNrSqy34DK-6WUl8bjQpDHScWvfhfYrY64CB-c_SaAgScU2t86HSeOcEfrA
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
   1. Introduction to Machine Learning (AI)
   2. Getting Started with ekoDB (Database)
   3. Vector Databases Explained (Database)
   4. Database Design Principles (Database)
   5. Natural Language Processing (AI)
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
   1. Introduction to Machine Learning
   2. Getting Started with ekoDB
   3. Vector Databases Explained
   4. Database Design Principles
   5. Natural Language Processing
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Function saved
📊 Projected documents (showing first 3):
   1. Introduction to Machine Learning
   2. Getting Started with ekoDB
   3. Vector Databases Explained
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Function saved
📊 All documents (5 total, showing first 2):
   1. Introduction to Machine Learning (AI)
   2. Getting Started with ekoDB (Database)
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
Retrieved value: { value: { username: 'john_doe', userId: 123 }, type: 'Object' }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { type: 'Object', value: { price: 29.99, name: 'Product 1' } }
cache:product:2: {
  type: 'Object',
  value: { name: 'Product 2', price: 39.989999999999995 }
}
cache:product:3: {
  type: 'Object',
  value: { name: 'Product 3', price: 49.989999999999995 }
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
  1. Score: 26.400, Matched: bio, title
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
  2. Score: 0.736, Matched:
  3. Score: 0.725, Matched:

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.494, Matched: title, content
  2. Score: 0.890, Matched: content, title
  3. Score: 0.298, Matched:

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: bio, skills

=== Cleanup ===
✅ Deleted test collections

✅ Search examples completed!
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'YVXKqWxig6q1Cm9IWFxh1C9k27qe2MAxLc56MpjahbbIUg4vGRogPBJgaH5d12X5Az8qhaD2mCjP-wUy9PoTow'
}

=== Find by ID ===
Found: {
  active: { value: true, type: 'Boolean' },
  id: 'YVXKqWxig6q1Cm9IWFxh1C9k27qe2MAxLc56MpjahbbIUg4vGRogPBJgaH5d12X5Az8qhaD2mCjP-wUy9PoTow',
  name: { type: 'String', value: 'Test Record' },
  value: { type: 'Integer', value: 42 }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  active: { value: true, type: 'Boolean' },
  id: 'YVXKqWxig6q1Cm9IWFxh1C9k27qe2MAxLc56MpjahbbIUg4vGRogPBJgaH5d12X5Az8qhaD2mCjP-wUy9PoTow',
  value: { type: 'Integer', value: 100 },
  name: { value: 'Updated Record', type: 'String' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: oS7WOaieT1HfpcYapy1d9i7VNMQ0kzL2Clmx7EQ04A5F1pAMqPzQJ8SPECWHDJSGkuOV2QFDd4iHJBTzug5LFA

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
✓ Created native SWR script: github_user_native_js (KCKbifRVERglpmVHkIF24hnNW-ZIKHgwj5u2E3vI3MSoGmG0MkhALoHeIVYvDYj2jOzu24Hr9vwKr05L9H9fyQ)

First call (cache miss - will fetch from GitHub API):
  Response time: 126ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 3ms
  Speedup: 42.0x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_js (OATU45jcHVumYNAfvgq1LBNYMfi1tivnI12LnBXn3qo4Y3M1QBKeYtGGbtagH3aqxD9kwcScdX_sL_rL7cEaig)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_js' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_js (KTJn4sSpTAhKIGdsmpFfe2a-g4I6hLTB4g91P3ni4LHDKfZHHEvYBpycHV3BGMtvfYDlazI4mAAhq95pK1cjkQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_js' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_js (9UTi_liZsUhP4dF_9ztcqWaftTvzDFa5QW-tHLs4I88hAWg0ZSIE8VPQSYd8NF_9gDrjlcw90lWDz7z71FdmPw)
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
✓ Created SWR script: fetch_api_user_js_1706_1792226030269 (eTSmwGYcM9yrs_z2abxga3mObpqSgbbQuFYal7YldgfmsTTm9L2OCCk_QafX4lN5UHDWVFqUkXpqSc97C6iluw)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "value": {
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "name": "Leanne Graham",
          "address": {
            "suite": "Apt. 556",
            "street": "Kulas Light",
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "zipcode": "92998-3874"
          },
          "website": "hildegard.org",
          "id": 1,
          "email": "Sincere@april.biz",
          "phone": "1-770-736-8031 x56442",
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
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 3ms (served from cache)
✓ Lightning fast cache hit

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
🧹 Cleaning up...
✓ Cleanup complete

✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: e8uDn_1uAwDINS4nc8gF6doZc_Lbko2rDhqKmKmpJYrOoE4DJifjpFzv4AHfQzy0sfTfuSMnoIbvawChXvy8ZA
Created Bob: $500 - ID: J-KxIL_STMGj5cIKLPp3FLhK51CmcP6MGAV8Ff4xnZv2dvEUvo28ZPdfW07hydVxADFxg5Z48OQG4IUaC7N7WA

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): d0d1ada4-549c-42ec-a48f-4ced958385be

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
New transaction: aaeeed08-df14-4f19-9f8b-e6b622439cda
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: KTv4e7RFtOIrq4uI6B7aaERIJGKRWK8nm29m47nyKaIsCf9q7ezvA8Vzs3b7uJ31eCEmjJku8tcFKFUWer9syA

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
=== ekoDB Advanced CRUD Example (TypeScript) ===

--- Inserting base record ---
Inserted: YgwLK3JT0q0fCMb1yst3zOTe__nCm0w4cpbYGLgKB8z9LcMQlER4RWVn7YEPNw2hrM51Ix14-nbPIBQXVrekTQ

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
✓ Created session: ypDQNxKMgFe0DF-vKR6w0LbhOruirR_Mkw7A5z6Tqt8TSez9cMjKjD_-hrEAxcVnqMp0KA5cPjPq_3nc8I_8Eg

=== Sending Initial Message ===
✓ Message sent
  Response: It seems that there are currently no available products listed in the database. The only entry present regarding a product is about ekoDB, but it didn't return any records when queried. If there's anything else you'd like to explore or if you have specific products in mind, feel free to let me know!

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
✓ Created second session: J3LFByvpp_bNBZVUmtnkWs7f-wu-qaC79zNig60bnzrd_bIYZVDPUY2rZp0JduAeGXxdAZf5xWu4vDa202tXoQ
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: J3LFByvpp_bNBZVUmtnkWs7f-wu-qaC79zNig60bnzrd_bIYZVDPUY2rZp0JduAeGXxdAZf5xWu4vDa202tXoQ
✓ Deleted chat session: ypDQNxKMgFe0DF-vKR6w0LbhOruirR_Mkw7A5z6Tqt8TSez9cMjKjD_-hrEAxcVnqMp0KA5cPjPq_3nc8I_8Eg
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: vYD2wDa11PCpOtNLxHPSR4FXNiv2HqVR3l0qyyb_2i7UPtyDfILw_rLNgWKNnu7okod3GRNca1WlGHRDvYcU1A

=== Sending Chat Message ===
Message ID: xeaDWxaE-CL0vLIO7JI-PmKHDfDONpMyL1vrjgjYglmx9UVFF_qwGa-us5duF9-ev35IdNl1u_zxNsvPaPU_iA

=== AI Response ===
The available products and their prices are as follows:

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
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'EQMfCwMLFMyrjaJq2nFypOSl32T2QYETMXVKSw4a1xOq-5ZDBJcoM0RROf_SCv2t7Pvvpp1t3MP10cdpFYvk1w',
    price: 299,
    name: 'ekoDB Pro',
    description: 'Enterprise edition product with advanced features'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 499,
    name: 'ekoDB Cloud',
    id: 'WQh1ApAtpmwHP4uv6PnON8WlQg4Wr3lUpzXaI3cEW2dlBHFLGnZst0rY0G96e93yLsfOFIBuBUY8JR8t7vmjWg',
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 99,
    name: 'ekoDB',
    description: 'A high-performance database product with AI capabilities',
    id: 'xwX7G1-HnsI-iYl67JdqvA4uLuoVFwGGVyGKjj0xURwE_fVUsUAmuB3QX0EbYICzGhpKvrQvn5SXUCr8PXZc_Q'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 1802ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 90
Total tokens: 3503

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Message Stream (SSE) Example (TypeScript) ===

Created session: I1080J8ap9GX988MpSyadW5z4yY2RusDx5iLn9JuBq1PYuiVA1AdFgwC4QexhmBSbzFzDIcsLlY3ll-Y-P8thg

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}**ekoDB** is a comprehensive, cloud-based knowledge platform focused on environmental, conservation, and biodiversity data. Designed to facilitate research, collaboration, and decision-making in ecology and related fields, ekoDB offers centralized access to a wide range of datasets—including species observations, habitat information, geospatial data, and environmental variables.

**Key features of ekoDB include:**
- **Data Aggregation:** Gathers and organizes biodiversity, environmental, and ecological data from various sources (e.g., scientific studies, public databases, citizen science initiatives).
- **Search and Visualization:** Provides user-friendly tools for searching datasets and visualizing complex ecological relationships.
- **Collaboration Tools:** Supports data sharing, project collaboration, and community engagement among researchers, conservation practitioners, and policymakers.
- **APIs and Integration:** Allows integration with other platforms and tools, facilitating advanced analysis and reporting.

**Applications:**
- Academic research in ecology, conservation, and environmental sciences
- Conservation policy and habitat management decisions
- Citizen science initiatives and data sharing

**Note:** If you’re referring to a specific project, platform, or database with a similar name but different focus, please provide more context, as there may be multiple uses of the term "ekoDB" in various regions or sectors.

--- Stream complete ---
Message ID: CpNNEYa23tZyGg-MG11qbFf4heReCFQTgd4ySPOEl3BxWRGMOhJeCSGXi63ejjvK2MVfh_VLmU9uIOPq-tgqGg
Execution time: 1814ms
Context window: 1000000 tokens

✓ Chat message stream example completed
✓ Client created

=== List Chat Models ===
Available chat models by provider:
  openai:
    - gpt-3.5-turbo-0125
    - gpt-5.4-2026-03-05
    - gpt-5-nano
    - gpt-5.4-mini
    - o1-pro
    - gpt-5.1-codex
    - tts-1
    - gpt-5-mini
    - gpt-5-mini-2025-08-07
    - gpt-4-turbo-2024-04-09
    - gpt-realtime-2.1-mini
    - gpt-realtime-2025-08-28
    - gpt-4
    - tts-1-hd
    - gpt-realtime-1.5
    - gpt-realtime
    - gpt-5.2-pro
    - gpt-5-chat-latest
    - gpt-5.5-pro
    - gpt-audio-2025-08-28
    - gpt-4o-mini-transcribe-2025-03-20
    - omni-moderation-2024-09-26
    - gpt-live-transcribe
    - gpt-5.6-luna
    - omni-moderation-latest
    - o4-mini-2025-04-16
    - gpt-4o-2024-11-20
    - gpt-5.1
    - whisper-1
    - gpt-5-pro
    - gpt-realtime-whisper
    - gpt-5-2025-08-07
    - gpt-5.4-mini-2026-03-17
    - babbage-002
    - gpt-4o-2024-05-13
    - text-embedding-3-large
    - chat-latest
    - gpt-realtime-translate
    - gpt-5.4-pro-2026-03-05
    - gpt-realtime-2
    - gpt-5-search-api
    - gpt-5.5-2026-04-23
    - tts-1-1106
    - gpt-5.5-pro-2026-04-23
    - gpt-audio-mini-2025-10-06
    - gpt-5.2-pro-2025-12-11
    - o3-mini-2025-01-31
    - gpt-audio-1.5
    - gpt-image-1.5
    - gpt-4-turbo
    - gpt-4o-2024-08-06
    - o3
    - gpt-4o-mini-search-preview-2025-03-11
    - gpt-4o-mini-transcribe-2025-12-15
    - gpt-4.1-mini
    - gpt-4o-mini
    - gpt-4.1-nano
    - gpt-image-2
    - gpt-3.5-turbo-16k
    - text-embedding-ada-002
    - gpt-realtime-2.1
    - gpt-4.1
    - gpt-5.5
    - gpt-4o-transcribe-diarize
    - gpt-5-codex
    - gpt-4-0613
    - gpt-5.1-2025-11-13
    - gpt-4o-search-preview
    - o1-2024-12-17
    - gpt-5-pro-2025-10-06
    - gpt-5.6-sol
    - gpt-4o-mini-tts-2025-12-15
    - gpt-image-1
    - gpt-audio
    - gpt-5.3-codex
    - gpt-5-search-api-2025-10-14
    - gpt-realtime-mini
    - gpt-image-1-mini
    - gpt-5.6-terra
    - gpt-3.5-turbo-instruct-0914
    - sora-2
    - gpt-5.4-pro
    - gpt-4o-mini-tts
    - gpt-realtime-mini-2025-12-15
    - gpt-5.1-codex-max
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
    - gpt-5.1-chat-latest
    - tts-1-hd-1106
    - gpt-5.2-codex
    - gpt-4o
    - o4-mini-deep-research
    - gpt-5.4
    - gpt-4o-search-preview-2025-03-11
    - text-embedding-3-small
    - gpt-5-nano-2025-08-07
    - gpt-4o-transcribe
    - o4-mini
    - gpt-5.1-codex-mini
    - gpt-image-2.5-sunburst-2026-09-08
    - gpt-5.3-chat-latest
    - o3-2025-04-16
    - gpt-5.2-2025-12-11
    - chatgpt-image-latest
    - gpt-3.5-turbo
    - gpt-3.5-turbo-1106
    - gpt-4o-mini-tts-2025-03-20
    - gpt-5.2
    - gpt-5.4-nano-2026-03-17
    - o1
    - davinci-002
    - gpt-5.4-nano
    - o3-mini
    - o4-mini-deep-research-2025-06-26
    - gpt-audio-mini-2025-12-15
    - o1-pro-2025-03-19
    - gpt-6-luna
    - gpt-image-2.5-flare
    - gpt-6-astra
    - gpt-audio-mini
    - gpt-4.1-mini-2025-04-14
    - gpt-image-2.5-sunburst
    - gpt-image-2.5-flare-2026-09-08
    - gpt-4o-mini-search-preview
    - gpt-6-sol
    - gpt-live-1
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
    - veo-3.1-generate-preview
    - veo-3.1-fast-generate-preview
    - veo-3.1-lite-generate-preview
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
  gemini: ok 63 models
  openai: ok 135 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get Specific Provider Models ===
OpenAI models: gpt-3.5-turbo-0125, gpt-5.4-2026-03-05, gpt-5-nano, gpt-5.4-mini, o1-pro, gpt-5.1-codex, tts-1, gpt-5-mini, gpt-5-mini-2025-08-07, gpt-4-turbo-2024-04-09, gpt-realtime-2.1-mini, gpt-realtime-2025-08-28, gpt-4, tts-1-hd, gpt-realtime-1.5, gpt-realtime, gpt-5.2-pro, gpt-5-chat-latest, gpt-5.5-pro, gpt-audio-2025-08-28, gpt-4o-mini-transcribe-2025-03-20, omni-moderation-2024-09-26, gpt-live-transcribe, gpt-5.6-luna, omni-moderation-latest, o4-mini-2025-04-16, gpt-4o-2024-11-20, gpt-5.1, whisper-1, gpt-5-pro, gpt-realtime-whisper, gpt-5-2025-08-07, gpt-5.4-mini-2026-03-17, babbage-002, gpt-4o-2024-05-13, text-embedding-3-large, chat-latest, gpt-realtime-translate, gpt-5.4-pro-2026-03-05, gpt-realtime-2, gpt-5-search-api, gpt-5.5-2026-04-23, tts-1-1106, gpt-5.5-pro-2026-04-23, gpt-audio-mini-2025-10-06, gpt-5.2-pro-2025-12-11, o3-mini-2025-01-31, gpt-audio-1.5, gpt-image-1.5, gpt-4-turbo, gpt-4o-2024-08-06, o3, gpt-4o-mini-search-preview-2025-03-11, gpt-4o-mini-transcribe-2025-12-15, gpt-4.1-mini, gpt-4o-mini, gpt-4.1-nano, gpt-image-2, gpt-3.5-turbo-16k, text-embedding-ada-002, gpt-realtime-2.1, gpt-4.1, gpt-5.5, gpt-4o-transcribe-diarize, gpt-5-codex, gpt-4-0613, gpt-5.1-2025-11-13, gpt-4o-search-preview, o1-2024-12-17, gpt-5-pro-2025-10-06, gpt-5.6-sol, gpt-4o-mini-tts-2025-12-15, gpt-image-1, gpt-audio, gpt-5.3-codex, gpt-5-search-api-2025-10-14, gpt-realtime-mini, gpt-image-1-mini, gpt-5.6-terra, gpt-3.5-turbo-instruct-0914, sora-2, gpt-5.4-pro, gpt-4o-mini-tts, gpt-realtime-mini-2025-12-15, gpt-5.1-codex-max, gpt-4o-mini-2024-07-18, gpt-4.1-2025-04-14, gpt-5, gpt-4o-mini-transcribe, gpt-image-2-2026-04-21, gpt-5.2-chat-latest, sora-2-pro, gpt-3.5-turbo-instruct, gpt-transcribe, gpt-4.1-nano-2025-04-14, gpt-5.1-chat-latest, tts-1-hd-1106, gpt-5.2-codex, gpt-4o, o4-mini-deep-research, gpt-5.4, gpt-4o-search-preview-2025-03-11, text-embedding-3-small, gpt-5-nano-2025-08-07, gpt-4o-transcribe, o4-mini, gpt-5.1-codex-mini, gpt-image-2.5-sunburst-2026-09-08, gpt-5.3-chat-latest, o3-2025-04-16, gpt-5.2-2025-12-11, chatgpt-image-latest, gpt-3.5-turbo, gpt-3.5-turbo-1106, gpt-4o-mini-tts-2025-03-20, gpt-5.2, gpt-5.4-nano-2026-03-17, o1, davinci-002, gpt-5.4-nano, o3-mini, o4-mini-deep-research-2025-06-26, gpt-audio-mini-2025-12-15, o1-pro-2025-03-19, gpt-6-luna, gpt-image-2.5-flare, gpt-6-astra, gpt-audio-mini, gpt-4.1-mini-2025-04-14, gpt-image-2.5-sunburst, gpt-image-2.5-flare-2026-09-08, gpt-4o-mini-search-preview, gpt-6-sol, gpt-live-1, gpt-6.1-sol

=== Get Anthropic Models ===
Anthropic models: claude-haiku-5-5, claude-sonnet-5-5, claude-opus-5-5, claude-fable-5-1, claude-opus-5, claude-sonnet-5, claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 404: {"error":"Unknown provider 'nonexistent_provider_xyz'","error_kind":"unknown_provider"}

✓ Chat Models example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: OO1PnQRPIj4lYeywr1qYN2lmtNH4FHmPMwMUQkJKPDURVHIvd1zYHaSRAJQKoAfyVX2meuK3FoVjKk1SjQF39g

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99

If you need more information or have further questions, let me know!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: LUe4T1KcGJH_sAfc9vnAmvuE8LG3KcuFQ8vNclTmEtr5mUOMRYCBQDlPwfhhgsoPH8EDZk7zl1-VbXr_7iFqng
  Parent: OO1PnQRPIj4lYeywr1qYN2lmtNH4FHmPMwMUQkJKPDURVHIvd1zYHaSRAJQKoAfyVX2meuK3FoVjKk1SjQF39g

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: LUe4T1KcGJH_sAfc9vnAmvuE8LG3KcuFQ8vNclTmEtr5mUOMRYCBQDlPwfhhgsoPH8EDZk7zl1-VbXr_7iFqng (Untitled)
  Session 2: OO1PnQRPIj4lYeywr1qYN2lmtNH4FHmPMwMUQkJKPDURVHIvd1zYHaSRAJQKoAfyVX2meuK3FoVjKk1SjQF39g (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: LUe4T1KcGJH_sAfc9vnAmvuE8LG3KcuFQ8vNclTmEtr5mUOMRYCBQDlPwfhhgsoPH8EDZk7zl1-VbXr_7iFqng

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: ju_iBYolqE8sSq6gBm0Nmv-kg9qZqo5Pi75m_dGAJlV-LJVeK6qWG0TgmrAprwe81HV01DIkZTs-d5iDylwQkQ

=== List Collections ===
Total collections: 27
Sample collections: chat_goals__ek0_testing,test_accounts,chat_messages__ek0_testing,schedules__ek0_testing,schema_documents_client_go

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
  POST /api/functions/conc_demo_pay_ts_3150_1792226049044 { "idempotency_key": "...", "amount": 100 }
  POST /api/functions/conc_demo_rl_fail_ts_3150_1792226049044 { "user_id": 42 }
  POST /api/functions/conc_demo_rl_skip_ts_3150_1792226049044 { "user_id": 42 }
  POST /api/functions/conc_demo_lock_ts_3150_1792226049044 { "resource": "queue:drain" }

✓ Cleaned up demo functions
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'wmIMeLnlY_1bdMP2om7WlXBiEaLlmaeHo9Z72TIPDyoBU4bhlSYF3S1zsRCvsoXzrlgwhe7kKwXZSwYoLlaD1g'
}

=== Upsert Operation ===
✓ First upsert (update): {
  age: { type: 'Integer', value: 29 },
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  name: { type: 'String', value: 'Alice Johnson' },
  id: 'wmIMeLnlY_1bdMP2om7WlXBiEaLlmaeHo9Z72TIPDyoBU4bhlSYF3S1zsRCvsoXzrlgwhe7kKwXZSwYoLlaD1g',
  active: { value: true, type: 'Boolean' }
}
✓ Second upsert (insert): { id: 'new-user-id' }

=== Find One Operation ===
✓ Found user by email: {
  id: 'wmIMeLnlY_1bdMP2om7WlXBiEaLlmaeHo9Z72TIPDyoBU4bhlSYF3S1zsRCvsoXzrlgwhe7kKwXZSwYoLlaD1g',
  active: { type: 'Boolean', value: true },
  name: { type: 'String', value: 'Alice Johnson' },
  age: { value: 29, type: 'Integer' },
  email: { type: 'String', value: 'alice.j@newdomain.com' }
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
✓ Inserted document: eWihjHvWri4uUmvCptDIurdKZZ0QS_zwTJ4nJDhIJU8df1QvzJsn2n_czNW6xpt-ZYEGMWmVEpk-D1zRPaa1HQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: blgQjefTM9FzAkH0XY5CtOVK-FerCCDenZlwPE0I5FS3UYRGaSNc1z9jhrgT3Dv-UF5wgGU1M1LfUjuAOyyLiQ

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
✓ Edge cache script created: PH9hXh7UJUUcf6hjSGymUqos9mMWe34ljChfIIj3dmqopywJkxzfUVceFJbA0dwbULhWv8r3fDzVc3T6stM6gA

Call 1: Cache miss (fetches from API)
Response time: 53ms
Result: {
  "records": [
    {
      "value": {
        "value": {
          "address": {
            "zipcode": "92998-3874",
            "city": "Gwenborough",
            "street": "Kulas Light",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "suite": "Apt. 556"
          },
          "email": "Sincere@april.biz",
          "phone": "1-770-736-8031 x56442",
          "id": 1,
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets"
          },
          "website": "hildegard.org",
          "username": "Bret",
          "name": "Leanne Graham"
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
Response time: 3ms (17.7x faster!)
Result: {
  "records": [
    {
      "value": {
        "value": {
          "address": {
            "zipcode": "92998-3874",
            "city": "Gwenborough",
            "street": "Kulas Light",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "suite": "Apt. 556"
          },
          "email": "Sincere@april.biz",
          "phone": "1-770-736-8031 x56442",
          "id": 1,
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets"
          },
          "website": "hildegard.org",
          "username": "Bret",
          "name": "Leanne Graham"
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
   ⏱️  Duration: 59ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "phone": "1-770-736-8031 x56442",
      "website": "hildegard.org",
      "email": "Sincere@april.biz",
      "address": {
        "street": "...

Second call (cache hit - from cache):
   ⏱️  Duration: 3ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "phone": "1-770-736-8031 x56442",
      "website": "hildegard.org",
      "email": "Sincere@april.biz",
      "address": {
        "street": "...
   🚀 Cache speedup: 19.7x faster!

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

✅ Function saved: 6ZCtiUfLcow72XRDQbLGKsaJ6xnM6noLxnh9Gmty2PU5bgSoLW_B2ONWdBSelWbAVYhbzofc2of9P89eC4XRiQ
📊 Found 5 active users

📝 Example 2: Parameterized Function

✅ Function saved: BqTuFrfWlVWXhPgfZnNXYpKllO_OBIi7nprBog8oi1dwCqnMyZzwoFhv-vz9hhovljxPp6ocsfA2zsizLlHCig
📊 Found 3 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: jgatsWHPAXKj-l0XogMoKTypXEtAWfXSfsGydsU8HSO6Cqt6qZPWdv1-8SeRmsAKyLcf54hXVRVv7KgCZJcsDA
📊 Statistics: 2 groups
   {"count":{"type":"Integer","value":5},"status":{"value":"inactive","type":"String"},"avg_score":{"value":50,"type":"Float"}}
   {"status":{"type":"String","value":"active"},"avg_score":{"value":60,"type":"Float"},"count":{"value":5,"type":"Integer"}}

📝 Example 4: UserFunction Management

📋 Total scripts: 8
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
   {"category":{"value":"Furniture","type":"String"},"avg_price":{"type":"Float","value":365.6666666666667},"count":{"type":"Integer","value":3}}
   {"category":{"value":"Electronics","type":"String"},"avg_price":{"value":367,"type":"Float"},"count":{"type":"Integer","value":5}}
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

1. **Efficient Similarity Search**: They enable rapid retrieval of similar items based on vector representations, ideal for applications like recommendations, search, and image recognition.

2. **High-dimensional Data Handling**: Vector databases can manage complex data types, such as text embeddings and image features, effectively.

3. **Scalability**: They can scale to handle large datasets, maintaining performance even with millions or billions of vectors.

4. **Machine Learning Integration**: Vector databases facilitate integrating with ML models, allowing for seamless operations on embeddings and prediction tasks.

5. **Real-time Processing**: Many vector databases support real-time querying and updates, making them suitable for dynamic applications.

6. **Flexible Indexing**: They often provide advanced indexing techniques (like approximate nearest neighbor search) for faster query responses.

7. **Enhanced Analytics**: Vector databases can support advanced analytics and insights generation, leveraging the semantic relationships within vector data.

8. **Multi-modal Capabilities**: They can handle various data types simultaneously, such as text, images, and audio, enabling comprehensive analysis across different modalities.
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

✅ Function saved: A4gNZoddbD9Gh-HVE63HWTK6eFLAHERuTNMnASdL5xSiKRI7Bk977Qzq8zDYJ6CqbSMEBubZ_WmrOwJq487a6Q
📊 Found 2 product groups
   {"count":{"value":2,"type":"Integer"},"avg_price":{"value":474,"type":"Float"},"category":{"type":"String","value":"Furniture"}}
   {"avg_price":{"type":"Float","value":575.6666666666666},"count":{"value":3,"type":"Integer"},"category":{"type":"String","value":"Electronics"}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"count":{"type":"Integer","value":3},"category":{"value":"Electronics","type":"String"}}
   {"count":{"value":2,"type":"Integer"},"category":{"type":"String","value":"Furniture"}}
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
   {"role":{"value":"user","type":"String"},"avg_score":{"type":"Float","value":70},"count":{"type":"Integer","value":7}}
   {"role":{"value":"admin","type":"String"},"avg_score":{"type":"Float","value":20},"count":{"value":3,"type":"Integer"}}
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

✅ Inserted order: hI1QBVZz9hjbbNO8iHMPVifmqDCy03MmBBi9mEFczL3RP7Qo5IWJUxV7PRFk7zGZ3Ok9Gfu3BnfollMRucAIDA
✅ Inserted 2 products with wrapped types

📝 Example 2: UserFunction with Wrapped Type Parameters

✅ Function saved: s7PLI-srLW1zszgYvsDPEE1AA1-cMLF_Q4E2-0Damj-VCUfIm1l5lQQ5ZDWWzN4yzNydJl18abezgU8nqh3Ymg
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"value":{"role":"admin","userId":"user_abc"},"type":"Object"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: tO_CEGkYVwOK2xN1WQhnOzOWpDRVnxbqrgH22uP7cX-KNGxaKWs7Ynxy7xU3xWVXJQYbXqP4n4PIKIJHrGHTtg
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: zmtpQuexZWmunW7VdRD8DIXMwDF4UkpUSLSOkwYS2aUt6rnBCqTUiPiH8IQ2sALYB6fpQqDWvBovXqXxe5GOyA
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
   1. Introduction to Machine Learning (AI)
   2. Natural Language Processing (AI)
   3. Vector Databases Explained (Database)
   4. Database Design Principles (Database)
   5. Getting Started with ekoDB (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"count":{"value":2,"type":"Integer"},"category":{"type":"String","value":"AI"}}
   {"category":{"type":"String","value":"Database"},"count":{"value":3,"type":"Integer"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Goal Template CRUD Example (TypeScript) ===

--- Creating goal template ---
Created template: Data Migration (id: E_ooDu5Old1mMPDH5FiUcKsA0-ViGSrGVuSAdIlmcQgIFxR1sov7Akk6vOuEYU64RnIi6s6dVjKyvDvXcGDUNw)

--- Listing templates ---
Templates: {
  count: 1,
  items: [
    {
      description: [Object],
      id: 'E_ooDu5Old1mMPDH5FiUcKsA0-ViGSrGVuSAdIlmcQgIFxR1sov7Akk6vOuEYU64RnIi6s6dVjKyvDvXcGDUNw',
      steps: [Object],
      title: [Object]
    }
  ]
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
Created goal: Deploy v2.0 (id: Z0Yp7GcLBnMFm0MxNKZQO1FybHCIVrYAYrJP8RZLbFYbn63z-G7OIoNVFxDzYM_D7zU4xvpF0cqEatRm5UN-Nw)

--- Listing goals ---
Goals: {
  "count": 1,
  "goals": [
    {
      "created_at": "2026-10-17T08:34:14.186395+00:00",
      "description": "Ship version 2.0 to production",
      "id": "Z0Yp7GcLBnMFm0MxNKZQO1FybHCIVrYAYrJP8RZLbFYbn63z-G7OIoNVFxDzYM_D7zU4xvpF0cqEatRm5UN-Nw",
      "status": "pending",
      "steps": "[{\"description\":\"Run test suite\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]",
      "title": "Deploy v2.0",
      "updated_at": "2026-10-17T08:34:14.186395+00:00"
    }
  ]
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
        "value": "2026-10-17T08:34:14.186395+00:00"
      },
      "description": {
        "type": "String",
        "value": "Ship version 2.0 with hot-fix patches"
      },
      "id": "Z0Yp7GcLBnMFm0MxNKZQO1FybHCIVrYAYrJP8RZLbFYbn63z-G7OIoNVFxDzYM_D7zU4xvpF0cqEatRm5UN-Nw",
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
        "value": "2026-10-17T08:34:14.200361+00:00"
      }
    }
  ]
}

--- Goal step: start step 0 ---
Step 0 started on goal Z0Yp7GcLBnMFm0MxNKZQO1FybHCIVrYAYrJP8RZLbFYbn63z-G7OIoNVFxDzYM_D7zU4xvpF0cqEatRm5UN-Nw
--- Goal step: complete step 0 ---
Step 0 completed on goal Z0Yp7GcLBnMFm0MxNKZQO1FybHCIVrYAYrJP8RZLbFYbn63z-G7OIoNVFxDzYM_D7zU4xvpF0cqEatRm5UN-Nw
--- Goal step: fail step 1 ---
Step 1 failed on goal Z0Yp7GcLBnMFm0MxNKZQO1FybHCIVrYAYrJP8RZLbFYbn63z-G7OIoNVFxDzYM_D7zU4xvpF0cqEatRm5UN-Nw

--- Completing goal ---
Goal status: pending_review
--- Approving goal ---
Goal status after approve: in_progress

--- Creating goal to reject ---
--- Rejecting goal ---
Goal status after reject: failed


--- Creating task ---
Created task: Hourly Health Check (id: -Zf1wiDq4rHjxP0Kf19X1cY6oxNroEO0YzTvtmqY7pEHmJeQpdyT2fcjHlWHAMpWVZDSQ8_10LlryBYQ9Ow_kg)

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
      "id": "-Zf1wiDq4rHjxP0Kf19X1cY6oxNroEO0YzTvtmqY7pEHmJeQpdyT2fcjHlWHAMpWVZDSQ8_10LlryBYQ9Ow_kg",
      "name": {
        "type": "String",
        "value": "Hourly Health Check"
      }
    }
  ]
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
Created agent: SupportBot (id: NNIO-thhBbd4dzsYcahza3Kx5MoXVnipomxvWLZlB92En-DGxAuK7dE1WmAiFiTUR6-RmtPFpjtWyMTO-wg_uA)

--- Listing agents ---
Agents: {
  "count": 1,
  "items": [
    {
      "deployment_id": {
        "type": "String",
        "value": "deploy_prod_1"
      },
      "id": "NNIO-thhBbd4dzsYcahza3Kx5MoXVnipomxvWLZlB92En-DGxAuK7dE1WmAiFiTUR6-RmtPFpjtWyMTO-wg_uA",
      "llm_model": {
        "type": "String",
        "value": "gpt-4"
      },
      "name": {
        "type": "String",
        "value": "SupportBot"
      },
      "system_prompt": {
        "type": "String",
        "value": "You are a helpful customer support agent."
      }
    }
  ]
}

--- Getting agent by ID ---
Fetched: SupportBot

--- Getting agent by name ---
By name: SupportBot (id: NNIO-thhBbd4dzsYcahza3Kx5MoXVnipomxvWLZlB92En-DGxAuK7dE1WmAiFiTUR6-RmtPFpjtWyMTO-wg_uA)

--- Updating agent ---
Updated agent: SupportBot

--- Getting agents by deployment ---
Agents in deployment: {
  "count": 0,
  "items": []
}
WARNING: agents-by-deployment omitted created agent NNIO-thhBbd4dzsYcahza3Kx5MoXVnipomxvWLZlB92En-DGxAuK7dE1WmAiFiTUR6-RmtPFpjtWyMTO-wg_uA; TODO: check/fix the server-side deployment lookup

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
  POST /api/functions/jwt_register_ts_3990_1792226054705 { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/jwt_login_ts_3990_1792226054705 { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/jwt_verify_ts_3990_1792226054705 { "token": "<jwt>" }

Set JWT_SECRET in ekoDB's environment_vars whitelist before invoking.

✓ Cleaned up demo functions
=== ekoDB KV Document Linking Example (TypeScript) ===

--- Setup: creating KV key and documents ---
Set KV key: session:admin
Inserted document 1: 0089KlSNHg0qegVrc2pZzoNFCwDFva3EVNz82nzy_mhCLGP6ttQATWSDEN0-UQgZs8uTT7L8NCnXsF9Oe266yw
Inserted document 2: ZlpMHC06c9l4ZKBh8YndaWyqIFvU8Cu6Taktq0NFUoK2PXkaBeuihNbTE4f8_WGpgqf_1kaCB-NCQn98j8C0fg

--- Linking documents to KV key ---
Linked doc 0089KlSNHg0qegVrc2pZzoNFCwDFva3EVNz82nzy_mhCLGP6ttQATWSDEN0-UQgZs8uTT7L8NCnXsF9Oe266yw: null
Linked doc ZlpMHC06c9l4ZKBh8YndaWyqIFvU8Cu6Taktq0NFUoK2PXkaBeuihNbTE4f8_WGpgqf_1kaCB-NCQn98j8C0fg: null

--- Getting links for KV key ---
Links: [
  {
    "collection": "kv_links_example_ts_3992_1792226054787",
    "document_id": "ZlpMHC06c9l4ZKBh8YndaWyqIFvU8Cu6Taktq0NFUoK2PXkaBeuihNbTE4f8_WGpgqf_1kaCB-NCQn98j8C0fg",
    "field_path": null,
    "created_at": "2026-10-17T08:34:14.867922Z",
    "last_accessed": "2026-10-17T08:34:14.869300Z",
    "metadata": {}
  },
  {
    "collection": "kv_links_example_ts_3992_1792226054787",
    "document_id": "0089KlSNHg0qegVrc2pZzoNFCwDFva3EVNz82nzy_mhCLGP6ttQATWSDEN0-UQgZs8uTT7L8NCnXsF9Oe266yw",
    "field_path": null,
    "created_at": "2026-10-17T08:34:14.866526Z",
    "last_accessed": "2026-10-17T08:34:14.869300Z",
    "metadata": {}
  }
]

--- Unlinking document ---
Unlinked doc ZlpMHC06c9l4ZKBh8YndaWyqIFvU8Cu6Taktq0NFUoK2PXkaBeuihNbTE4f8_WGpgqf_1kaCB-NCQn98j8C0fg: null

--- Verifying remaining links ---
Remaining links: [
  {
    "collection": "kv_links_example_ts_3992_1792226054787",
    "document_id": "0089KlSNHg0qegVrc2pZzoNFCwDFva3EVNz82nzy_mhCLGP6ttQATWSDEN0-UQgZs8uTT7L8NCnXsF9Oe266yw",
    "field_path": null,
    "created_at": "2026-10-17T08:34:14.866526Z",
    "last_accessed": "2026-10-17T08:34:14.871293Z",
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
  kv_ops_ts_3994_1792226054957:cache:product:1: success
  kv_ops_ts_3994_1792226054957:cache:product:2: success
  kv_ops_ts_3994_1792226054957:cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  kv_ops_ts_3994_1792226054957:cache:product:1: { price: 29.99, name: 'Product 1' }
  kv_ops_ts_3994_1792226054957:cache:product:2: { price: 39.99, name: 'Product 2' }
  kv_ops_ts_3994_1792226054957:cache:product:3: { price: 49.99, name: 'Product 3' }

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
  kv_ops_ts_3994_1792226054957:cache:product:1: deleted
  kv_ops_ts_3994_1792226054957:cache:product:2: deleted
  kv_ops_ts_3994_1792226054957:cache:product:3: deleted

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
Response: The three primary colors are red, blue, and yellow.

--- Structured Output via SSE ---
JSON response: [
  {
    "name": "Mercury",
    "diameter_km": 4879
  },
  {
    "name": "Jupiter",
    "diameter_km": 139820
  },
  {
    "name": "Neptune",
    "diameter_km": 49244
  }
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello! I hope you're having a wonderful day.

=== Done ===
=== ekoDB Schedule Management Example (TypeScript) ===

--- Creating schedule ---
Created schedule: Nightly Database Backup (id: e62be03b-1cc3-4c6a-8e9d-798d72a43e4c, cron: 0 0 2 * * *)

--- Listing schedules ---
Schedules: {
  "count": 1,
  "schedules": [
    {
      "created_at": "2026-10-17T08:34:17.434286Z",
      "cron_expression": "0 0 2 * * *",
      "description": null,
      "enabled": true,
      "function_label": "schedule_noop_typescript_4443_1792226057378",
      "id": "e62be03b-1cc3-4c6a-8e9d-798d72a43e4c",
      "last_execution": null,
      "name": "Nightly Database Backup",
      "next_execution": "2026-10-18T02:00:00Z",
      "parameters": {},
      "stats": {
        "avg_execution_time_ms": 0,
        "failed_executions": 0,
        "last_error": null,
        "successful_executions": 0,
        "total_executions": 0
      },
      "timezone": "UTC",
      "updated_at": "2026-10-17T08:34:17.434286Z"
    }
  ]
}

--- Getting schedule ---
Fetched: Nightly Database Backup (cron: 0 0 2 * * *)

--- Updating schedule ---
Updated: Nightly Full Backup (new cron: 0 0 3 * * *)

--- Triggering schedule ---
Trigger response: {
  "schedule_id": "e62be03b-1cc3-4c6a-8e9d-798d72a43e4c",
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
  1. Score: 12.870, Matched: name, email
  2. Score: 6.270, Matched: name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 13.200, Matched: bio, title
  2. Score: 13.200, Matched: title, bio
  3. Score: 13.200, Matched: bio, title
  4. Score: 13.200, Matched: bio, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400, Matched: title, bio
  2. Score: 26.400, Matched: title, bio
  3. Score: 26.400, Matched: bio, title
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
  1. Score: 0.758, Matched:
  2. Score: 0.748, Matched:
  3. Score: 0.747, Matched:

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.499, Matched: content, title
  2. Score: 0.899, Matched: content, title
  3. Score: 0.303, Matched:

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
  id: 'oWcCINsGqyc0QW36MfO6zD0mX0BUvfpAlx970uECMp5VgeEHbb5UUO0RYfa6NmaiLwz7HX2RhhKweJaJcws0NA'
}

=== Find by ID ===
Found: {
  name: { value: 'Test Record', type: 'String' },
  metadata: { type: 'Object', value: { key: 'value', nested: [Object] } },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  price: { value: 99.99, type: 'Float' },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  value: { type: 'Integer', value: 42 },
  active: { type: 'Boolean', value: true },
  created_at: { type: 'DateTime', value: '2026-10-17T08:34:18.034+00:00' },
  id: 'oWcCINsGqyc0QW36MfO6zD0mX0BUvfpAlx970uECMp5VgeEHbb5UUO0RYfa6NmaiLwz7HX2RhhKweJaJcws0NA'
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-10-17T08:34:18.034Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { key: 'value', nested: { deep: true } }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  name: 'Test Record',
  metadata: { key: 'value', nested: { deep: true } },
  categories: [ 'electronics', 'computers' ],
  data: 'aGVsbG8gd29ybGQ=',
  price: 99.99,
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  tags: [ 'tag1', 'tag2', 'tag3' ],
  value: 42,
  active: true,
  created_at: '2026-10-17T08:34:18.034+00:00',
  id: 'oWcCINsGqyc0QW36MfO6zD0mX0BUvfpAlx970uECMp5VgeEHbb5UUO0RYfa6NmaiLwz7HX2RhhKweJaJcws0NA'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  metadata: { type: 'Object', value: { key: 'value', nested: [Object] } },
  price: { type: 'Float', value: 99.99 },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  active: { value: true, type: 'Boolean' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  id: 'oWcCINsGqyc0QW36MfO6zD0mX0BUvfpAlx970uECMp5VgeEHbb5UUO0RYfa6NmaiLwz7HX2RhhKweJaJcws0NA',
  name: { value: 'Updated Record', type: 'String' },
  value: { type: 'Integer', value: 100 },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  created_at: { type: 'DateTime', value: '2026-10-17T08:34:18.034+00:00' },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: 6V1rekcgpiLYz7zF2VMs7RlacqdG1y_uRYZcPulbsqq-oxzOpPNkVlUoI6goNQF_ebbb6Gk6U_nlYOEyAP7FIg

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
✓ Created native SWR script: github_user_native_ts (_eE6WP9XHARDd5RCxQsOq-en6ehNZ9yMxx6iXilzJo3LKeEQFwcCVtO9lyYY1DWh-6oHVSCs1ZjRci59Sfj6Cg)

First call (cache miss - will fetch from GitHub API):
  Response time: 54ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 2ms
  Speedup: 27.0x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_ts (yNuv7nl6HKUnBBcCrLXuvRbH949LcsANQHqhLsYB1gQzP5mWl3Do4MGE4OA99-AVdn-GidV5dXmWPuQVrR0dCg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_ts' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_ts (j3E4xGMtg03cWzr3EYL2tXdrBVDFRLes2euzsS8RZYW0hHn9nOJpjDvX8dKWbZPYru54QPQoBd5le0KV5wtJyA)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_ts' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_ts (rIhJaYQcLz2nBw4jUYHjr2GodQnhbmDraVrSE5ak-6eSFLOLM34tlRD7EGnbf4LPdygzIR3POaFSDYBvwtT7Mg)
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
✓ Created SWR script: fetch_api_user_ts_4706_1792226059130 (96PcNn5f-14QV0LnHhdIFTqN5KBxZ5s5DtH68sASFv84oYU5E4OBciuJ_RLPTeuOlbPa-Ip2usc_2D6p9GqSuw)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "id": 1,
          "address": {
            "street": "Kulas Light",
            "city": "Gwenborough",
            "suite": "Apt. 556",
            "zipcode": "92998-3874",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            }
          },
          "name": "Leanne Graham",
          "email": "Sincere@april.biz",
          "username": "Bret",
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "phone": "1-770-736-8031 x56442",
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
          "id": 1,
          "address": {
            "street": "Kulas Light",
            "city": "Gwenborough",
            "suite": "Apt. 556",
            "zipcode": "92998-3874",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            }
          },
          "name": "Leanne Graham",
          "email": "Sincere@april.biz",
          "username": "Bret",
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "phone": "1-770-736-8031 x56442",
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
✓ Created enrichment script: fetch_product_reviews_ts_4706_1792226059130 (Xqwv-VEkr_1eNbTay76b9DjOYfzuCUR4IwK6EGQB5Z6WWedaC__Jyhv6dQ5A_TcSK-p1QeOAinl3BuYJhfrMvQ)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "value": {
          "shippingInformation": "Ships in 3-5 business days",
          "id": 1,
          "title": "Essence Mascara Lash Princess",
          "meta": {
            "updatedAt": "2026-05-23T11:27:41.868Z",
            "barcode": "5784719087687",
            "createdAt": "2025-10-09T14:47:01.588Z",
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png"
          },
          "reviews": [
            {
              "date": "2025-04-30T09:41:02.053Z",
              "comment": "Would not recommend!",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "reviewerName": "Eleanor Collins",
              "rating": 3
            },
            {
              "reviewerName": "Lucas Gordon",
              "rating": 4,
              "date": "2025-04-30T09:41:02.053Z",
              "comment": "Very satisfied!",
              "reviewerEmail": "lucas.gordon@x.dummyjson.com"
            },
            {
              "date": "2025-04-30T09:41:02.053Z",
              "comment": "Highly impressed!",
              "rating": 5,
              "reviewerName": "Eleanor Collins",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com"
            }
          ],
          "sku": "BEA-ESS-ESS-001",
          "dimensions": {
            "depth": 22.99,
            "height": 13.08,
            "width": 15.14
          },
          "stock": 99,
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "rating": 2.56,
          "brand": "Essence",
          "minimumOrderQuantity": 48,
          "availabilityStatus": "In Stock",
          "price": 9.99,
          "discountPercentage": 10.48,
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "returnPolicy": "No return policy",
          "tags": [
            "beauty",
            "mascara"
          ],
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "warrantyInformation": "1 week warranty",
          "weight": 4,
          "category": "beauty"
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

✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: M0N-8QWIg1Fy3ag6Y3pyI86aqwcjTYiMk9B227VmalA66rK0b4jbd1-HSi83UVJ4j4P0Z7qJhNhcijVI72BCJw
Created Bob: $500 - ID: EQbkJ5PUu5OAClPtP22a9Gf5ZltEF7N-o7JSLpOyQbmu7jF0C9k33QbNjaOUXUkBckr_dYhY_WdmygGyyeWOeA

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): dc622af5-411b-4ad9-9b7a-1a1cd7212267

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
New transaction: bcfbc3b1-ff7d-4590-8fd4-efa36d49683e
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Create User Function ===
Created user function with ID: 7k_V6JEACKUxt1_gRrXGFi1B5f5qIdDM1wPP2vsrJDOOARC9jK1RHDni3NdHeGUEUYl_3rWE-7k6FTgLftO9vA

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 8 user functions:
  - get_active_users_ts: Get Active Users
  - get_active_users_client_js: Get Active Users (Updated)
  - get_active_users_client_ts_updated: Get Active Users (Updated)
  - get_active_users_client_ts_updated: Get Active Users (Updated)
  - fetch_product_reviews_ts_4706_1792226059130: Fetch Product with Reviews (Multi-API)
  - fetch_product_reviews_ts_97810_1792225951079: Fetch Product with Reviews (Multi-API)
  - conc_demo_rl_fail_ts_95985_1792225931695: Rate-limit (fail mode)
  - conc_demo_rl_skip_ts_95985_1792225931695: Rate-limit (skip mode)

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
=== WebSocket Chat Streaming Example (TypeScript) ===

Created chat session: X0-ZBVpVDEsV2Zs-mn1kt-IX8DFNhNlbu6sVU2byJYGaQcpWTq-jDEthsIJ6UO125RHBw2ZPd-GWQeGNAG_aKg

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: ttWd2cZWDj2SRdxYaNnc1C51UFySrFIQKSoj5GXsH_CS8wxIuDzDClvUDkx2vYDKrHqKtwvzTg9EqsVIuj6Z5w
Execution time: 619ms
Token usage: {"completion_tokens":8,"prompt_tokens":15,"total_tokens":23}

Full response: {"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris....
=== WebSocket Subscription Example ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_ts' ===
✓ Subscribed (subscription_id: sub_39cf5aae5d7f4c2fb53fcf1ab6068d2a)

=== Performing mutations to trigger notifications ===
Inserting a record...
✓ Inserted record: Q1YZXtU4k8XPlX1QuYjTsPgi35IP70UkYXsTtF_9HUSoG6ktJP6EMb0jNjhl9BWV5C6OYf5m1gz5gNtKp4wYUw
  📡 Notification received for Q1YZXtU4k8XPlX1QuYjTsPgi35IP70UkYXsTtF_9HUSoG6ktJP6EMb0jNjhl9BWV5C6OYf5m1gz5gNtKp4wYUw

Inserting another record...
✓ Inserted record: HW0_bb_SivpoCpNvRC_lZ9QiQ3a5FXcWNFJpPC-KOHHrKBbiFfENUD0mV5O2EajWfpyqrTBr5Vajn3XVGBLIjw
  📡 Notification received for HW0_bb_SivpoCpNvRC_lZ9QiQ3a5FXcWNFJpPC-KOHHrKBbiFfENUD0mV5O2EajWfpyqrTBr5Vajn3XVGBLIjw

=== Unsubscribing ===
✓ Unsubscribed: {"collection":"ws_subscribe_example_ts","found":true,"unsubscribed":true}

✓ WebSocket subscription example completed successfully
✓ Deleted collection 'ws_subscribe_example_ts'
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: 8IRn0SpXNUpXXk480UnlzV-0tb5rTDvhNF_Ibz-oKBshW2EdLa3uTP4PMqSVHEuGAHiRdauvquxtXE8PquxBjg

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
   Inserted with ripple: {"id":"vbjI9AaqG4L18dp5uDqqtg-03b90_JnSJqOr-oePq4C0vesbYLR8H3euDyE6IcXh1imjYJktWeq9U6stAy9Ozg"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"gMuPt2mEqfdyXH4jMP2HFr8a4dsuZnmaNfO0ls1Bk001fJFj18oLpgndGR8GF0hi05gyzlNXzFhMwMm6Ge2LIA"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"price":{"value":150,"type":"Integer"},"id":"vbjI9AaqG4L18dp5uDqqtg-03b90_JnSJqOr-oePq4C0vesbYLR8H3euDyE6IcXh1imjYJktWeq9U6stAy9Ozg","name":{"type":"String","value":"Product 1"}}

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
  First user: Dave Brown <dave@example.com>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["created_at","id","email","user_role","bio","status","age","name","avatar_url"]

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
    - Fields: ["name","api_key","age","created_at","bio","status","email","avatar_url","user_role","id","secret_token","password"]
  Projected query:
    - 3 fields per record
    - Fields: ["email","name","id"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
✅ All JavaScript integration tests complete!
🟣 Building Kotlin client library...
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.8.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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

BUILD SUCCESSFUL in 9s
11 actionable tasks: 11 up-to-date
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
✅ Kotlin client built!
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
Inserted: sAxKHEOPwqxKq7ANKTZkSt2cdE3ZfhnQaGX3Bptj8i6dAOFE0_LSotgFZn8aZ9OCrGVxB_efUrHGxIwEw-2-iA

--- updateWithAction: increment views ---
After increment: EkoRecord(fields={views=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=15)}), score=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=100.0)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Counter Record)}), id=StringValue(value=sAxKHEOPwqxKq7ANKTZkSt2cdE3ZfhnQaGX3Bptj8i6dAOFE0_LSotgFZn8aZ9OCrGVxB_efUrHGxIwEw-2-iA), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=kotlin), StringValue(value=example)])})})

--- updateWithAction: push to tags ---
After push: EkoRecord(fields={name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Counter Record)}), views=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=15)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=kotlin), StringValue(value=example), StringValue(value=advanced)])}), score=ObjectValue(value={value=FloatValue(value=100.0), type=StringValue(value=Float)}), id=StringValue(value=sAxKHEOPwqxKq7ANKTZkSt2cdE3ZfhnQaGX3Bptj8i6dAOFE0_LSotgFZn8aZ9OCrGVxB_efUrHGxIwEw-2-iA)})

--- updateWithActionSequence: multiple operations ---
After sequence: EkoRecord(fields={tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=kotlin), StringValue(value=example), StringValue(value=advanced), StringValue(value=sequenced)])}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Counter Record)}), id=StringValue(value=sAxKHEOPwqxKq7ANKTZkSt2cdE3ZfhnQaGX3Bptj8i6dAOFE0_LSotgFZn8aZ9OCrGVxB_efUrHGxIwEw-2-iA), views=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=115)}), score=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=75.0)})})

--- Cleanup ---
Deleted collection: kotlin_advanced_crud_example

=== Example Complete ===

BUILD SUCCESSFUL in 19s
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
  IDs: b2zEpInmR3Wwmz2dP0V3UTVAAKNyemJnTpeM8ek-cF6GWQ-J_tqVp81kfSnCQg76M7X6AyrQkgT-8Xc70cTY2Q, siYKclsA8UpKnlKV2yjZ27mgKpm7N43v_A2sqhRLltOTr7Jp_9idjQm8wRKwoQiCWfAbCIclZkp3-a-tn6mj0g, O8Oh7HE3UNULxL-iv6ckQ8JwfmiucFZhFsWnc99dJ3N3DsvkesmzN-t54H7YB-Ml_tQ2KbdqMTzPkmNNzgHoXw...

=== Batch Update ===
✓ Updated 3 records; 0 failed

=== Batch Delete ===
✓ Deleted 2 records

=== Cleanup ===
✓ Deleted collection: kotlin_batch_example

=== Example Complete ===

BUILD SUCCESSFUL in 8s
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
✓ Created session: gwh_CJSnGXHb9YYPB7Xv9fQG_RL3uvnsFBWF3XIltIyDhQsQzLREyKwlzJXEZZ3kOPaLJuCfI1y1VlyTRP7FWg

=== Sending Initial Message ===
✓ Message sent
  Responses: ["It appears that there are currently no products available in the database. If you need more information or if there is anything else you'd like to explore, please let me know!"]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["The price of ekoDB is $99. If you have any other questions or need further assistance, feel free to ask!"]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: JW2PyNk3V2J4ikbx_N9qfwY77JK1n1hIw6IJ_A8C515GI7pLPbFGj78b0w_wCvmpLtiMiqaFE2en-WADyQq39g

=== Merging Sessions ===
✓ Merged sessions
  Total messages in merged session: 5

=== Deleting Message ===
✓ Deleted message

=== Cleanup ===
✓ Deleted chat session: JW2PyNk3V2J4ikbx_N9qfwY77JK1n1hIw6IJ_A8C515GI7pLPbFGj78b0w_wCvmpLtiMiqaFE2en-WADyQq39g
✓ Deleted chat session: gwh_CJSnGXHb9YYPB7Xv9fQG_RL3uvnsFBWF3XIltIyDhQsQzLREyKwlzJXEZZ3kOPaLJuCfI1y1VlyTRP7FWg
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
✓ Created session: 115M1GCaojauI5kLKXJQyKSQ6mEPfT6WGQdapADUKmilq0yU56-gh9PU-85ny5X4WUZYi1PVKv2p_SRXXUTayQ

=== Sending Chat Message ===
✓ Chat response:
  Message ID: "yTfdWMoM8jJw1A59qhB62yXJISwe6n6f8b60Rcago83_KJR-6GnikkqwuwzjlDTFKZTWPVXa1asSXZGT12XzPg"
  Responses: ["ekoDB is a high-performance database designed to integrate seamlessly with AI capabilities and provide intelligent caching and real-time functionalities. \n\n### Key Features of ekoDB:\n1. **AI Chat Integration**: The chat feature allows users to query the database using natural language, receiving AI-powered responses relevant to their context.\n2. **Search Capabilities**: ekoDB supports various searching techniques:\n   - **Full-text Search**: Efficiently find content based on keyword matches.\n   - **Vector Search**: Enables semantic searching based on meaning rather than just keywords.\n   - **Hybrid Search**: Combines full-text and vector search capabilities.\n3. **Caching**: ekoDB offers intelligent caching mechanisms to boost performance and response time.\n\nFor more specifics, you can refer to the documentation provided in the database or ask anything specific about its capabilities!"]

=== Cleanup ===
✓ Deleted chat session
✓ Deleted collection: kotlin_chat_basic_example

✓ Basic chat example completed successfully

BUILD SUCCESSFUL in 10s
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
Created session: 3uCUDgghO0YkpN21b8Moap9Qpq0vvYCqimzFWT1pBwujVES3rHThszQ3Kd_PlHyJvt6J1SSsliCIj9njB5FXkw

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}EkoDB is a database management system that is designed to provide a fast, scalable, and efficient solution for managing structured and semi-structured data. While there isn't a single widely-known product named "ekoDB," it is associated with different contexts or implementations depending on the specific technology or organization referring to it.

In many cases, specialized databases, like EkoDB, might focus on use cases that involve real-time data processing, high availability, and performance optimization, catering to developers and data engineers in various industries.

Do you have a specific context or application in mind regarding EkoDB? If you provide more details, I can give a more precise explanation.

--- Stream complete ---
Message ID: EvvN1TESAnvKbaYtMiYseARAtq6tceShah-AL79xdtaK4H3onkVY2eW1iF_c12DO8WYpOMw5JnGtwPWEDQUkZw
Execution time: 1437ms
Context window: 128000 tokens

✓ Chat message stream example completed

BUILD SUCCESSFUL in 9s
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
    - gpt-5.4-mini
    - o1-pro
    - gpt-5.1-codex
    - tts-1
    - gpt-5-mini
    - gpt-5-mini-2025-08-07
    - gpt-4-turbo-2024-04-09
    - gpt-realtime-2.1-mini
    - gpt-realtime-2025-08-28
    - gpt-4
    - tts-1-hd
    - gpt-realtime-1.5
    - gpt-realtime
    - gpt-5.2-pro
    - gpt-5-chat-latest
    - gpt-5.5-pro
    - gpt-audio-2025-08-28
    - gpt-4o-mini-transcribe-2025-03-20
    - omni-moderation-2024-09-26
    - gpt-live-transcribe
    - gpt-5.6-luna
    - omni-moderation-latest
    - o4-mini-2025-04-16
    - gpt-4o-2024-11-20
    - gpt-5.1
    - whisper-1
    - gpt-5-pro
    - gpt-realtime-whisper
    - gpt-5-2025-08-07
    - gpt-5.4-mini-2026-03-17
    - babbage-002
    - gpt-4o-2024-05-13
    - text-embedding-3-large
    - chat-latest
    - gpt-realtime-translate
    - gpt-5.4-pro-2026-03-05
    - gpt-realtime-2
    - gpt-5-search-api
    - gpt-5.5-2026-04-23
    - tts-1-1106
    - gpt-5.5-pro-2026-04-23
    - gpt-audio-mini-2025-10-06
    - gpt-5.2-pro-2025-12-11
    - o3-mini-2025-01-31
    - gpt-audio-1.5
    - gpt-image-1.5
    - gpt-4-turbo
    - gpt-4o-2024-08-06
    - o3
    - gpt-4o-mini-search-preview-2025-03-11
    - gpt-4o-mini-transcribe-2025-12-15
    - gpt-4.1-mini
    - gpt-4o-mini
    - gpt-4.1-nano
    - gpt-image-2
    - gpt-3.5-turbo-16k
    - text-embedding-ada-002
    - gpt-realtime-2.1
    - gpt-4.1
    - gpt-5.5
    - gpt-4o-transcribe-diarize
    - gpt-5-codex
    - gpt-4-0613
    - gpt-5.1-2025-11-13
    - gpt-4o-search-preview
    - o1-2024-12-17
    - gpt-5-pro-2025-10-06
    - gpt-5.6-sol
    - gpt-4o-mini-tts-2025-12-15
    - gpt-image-1
    - gpt-audio
    - gpt-5.3-codex
    - gpt-5-search-api-2025-10-14
    - gpt-realtime-mini
    - gpt-image-1-mini
    - gpt-5.6-terra
    - gpt-3.5-turbo-instruct-0914
    - sora-2
    - gpt-5.4-pro
    - gpt-4o-mini-tts
    - gpt-realtime-mini-2025-12-15
    - gpt-5.1-codex-max
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
    - gpt-5.1-chat-latest
    - tts-1-hd-1106
    - gpt-5.2-codex
    - gpt-4o
    - o4-mini-deep-research
    - gpt-5.4
    - gpt-4o-search-preview-2025-03-11
    - text-embedding-3-small
    - gpt-5-nano-2025-08-07
    - gpt-4o-transcribe
    - o4-mini
    - gpt-5.1-codex-mini
    - gpt-image-2.5-sunburst-2026-09-08
    - gpt-5.3-chat-latest
    - o3-2025-04-16
    - gpt-5.2-2025-12-11
    - chatgpt-image-latest
    - gpt-3.5-turbo
    - gpt-3.5-turbo-1106
    - gpt-4o-mini-tts-2025-03-20
    - gpt-5.2
    - gpt-5.4-nano-2026-03-17
    - o1
    - davinci-002
    - gpt-5.4-nano
    - o3-mini
    - o4-mini-deep-research-2025-06-26
    - gpt-audio-mini-2025-12-15
    - o1-pro-2025-03-19
    - gpt-6-luna
    - gpt-image-2.5-flare
    - gpt-6-astra
    - gpt-audio-mini
    - gpt-4.1-mini-2025-04-14
    - gpt-image-2.5-sunburst
    - gpt-image-2.5-flare-2026-09-08
    - gpt-4o-mini-search-preview
    - gpt-6-sol
    - gpt-live-1
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
    - veo-3.1-generate-preview
    - veo-3.1-fast-generate-preview
    - veo-3.1-lite-generate-preview
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
  gemini: ok 63 models
  openai: ok 135 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get OpenAI Models ===
OpenAI models: gpt-3.5-turbo-0125, gpt-5.4-2026-03-05, gpt-5-nano, gpt-5.4-mini, o1-pro, gpt-5.1-codex, tts-1, gpt-5-mini, gpt-5-mini-2025-08-07, gpt-4-turbo-2024-04-09, gpt-realtime-2.1-mini, gpt-realtime-2025-08-28, gpt-4, tts-1-hd, gpt-realtime-1.5, gpt-realtime, gpt-5.2-pro, gpt-5-chat-latest, gpt-5.5-pro, gpt-audio-2025-08-28, gpt-4o-mini-transcribe-2025-03-20, omni-moderation-2024-09-26, gpt-live-transcribe, gpt-5.6-luna, omni-moderation-latest, o4-mini-2025-04-16, gpt-4o-2024-11-20, gpt-5.1, whisper-1, gpt-5-pro, gpt-realtime-whisper, gpt-5-2025-08-07, gpt-5.4-mini-2026-03-17, babbage-002, gpt-4o-2024-05-13, text-embedding-3-large, chat-latest, gpt-realtime-translate, gpt-5.4-pro-2026-03-05, gpt-realtime-2, gpt-5-search-api, gpt-5.5-2026-04-23, tts-1-1106, gpt-5.5-pro-2026-04-23, gpt-audio-mini-2025-10-06, gpt-5.2-pro-2025-12-11, o3-mini-2025-01-31, gpt-audio-1.5, gpt-image-1.5, gpt-4-turbo, gpt-4o-2024-08-06, o3, gpt-4o-mini-search-preview-2025-03-11, gpt-4o-mini-transcribe-2025-12-15, gpt-4.1-mini, gpt-4o-mini, gpt-4.1-nano, gpt-image-2, gpt-3.5-turbo-16k, text-embedding-ada-002, gpt-realtime-2.1, gpt-4.1, gpt-5.5, gpt-4o-transcribe-diarize, gpt-5-codex, gpt-4-0613, gpt-5.1-2025-11-13, gpt-4o-search-preview, o1-2024-12-17, gpt-5-pro-2025-10-06, gpt-5.6-sol, gpt-4o-mini-tts-2025-12-15, gpt-image-1, gpt-audio, gpt-5.3-codex, gpt-5-search-api-2025-10-14, gpt-realtime-mini, gpt-image-1-mini, gpt-5.6-terra, gpt-3.5-turbo-instruct-0914, sora-2, gpt-5.4-pro, gpt-4o-mini-tts, gpt-realtime-mini-2025-12-15, gpt-5.1-codex-max, gpt-4o-mini-2024-07-18, gpt-4.1-2025-04-14, gpt-5, gpt-4o-mini-transcribe, gpt-image-2-2026-04-21, gpt-5.2-chat-latest, sora-2-pro, gpt-3.5-turbo-instruct, gpt-transcribe, gpt-4.1-nano-2025-04-14, gpt-5.1-chat-latest, tts-1-hd-1106, gpt-5.2-codex, gpt-4o, o4-mini-deep-research, gpt-5.4, gpt-4o-search-preview-2025-03-11, text-embedding-3-small, gpt-5-nano-2025-08-07, gpt-4o-transcribe, o4-mini, gpt-5.1-codex-mini, gpt-image-2.5-sunburst-2026-09-08, gpt-5.3-chat-latest, o3-2025-04-16, gpt-5.2-2025-12-11, chatgpt-image-latest, gpt-3.5-turbo, gpt-3.5-turbo-1106, gpt-4o-mini-tts-2025-03-20, gpt-5.2, gpt-5.4-nano-2026-03-17, o1, davinci-002, gpt-5.4-nano, o3-mini, o4-mini-deep-research-2025-06-26, gpt-audio-mini-2025-12-15, o1-pro-2025-03-19, gpt-6-luna, gpt-image-2.5-flare, gpt-6-astra, gpt-audio-mini, gpt-4.1-mini-2025-04-14, gpt-image-2.5-sunburst, gpt-image-2.5-flare-2026-09-08, gpt-4o-mini-search-preview, gpt-6-sol, gpt-live-1, gpt-6.1-sol

=== Get Anthropic Models ===
Anthropic models: claude-haiku-5-5, claude-sonnet-5-5, claude-opus-5-5, claude-fable-5-1, claude-opus-5, claude-sonnet-5, claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Request failed with status 404: {"error":"Unknown provider 'nonexistent_provider_xyz'","error_kind":"unknown_provider"}

=== Chat Models Example Complete ===

BUILD SUCCESSFUL in 7s
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
✓ Created session: adFsdZahxd4-ZgN8GIc4W6tpwsELsOgwNlPGL1mCj0xpazhykj-eYKJKNQbWWqpJRjwhw3YpCehRvHUpxMp-MQ

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["The available product is:\n\n- **Product**: ekoDB\n- **Description**: A high-performance database product with AI capabilities\n- **Price**: $99\n\nIf you need more information or have further questions, feel free to ask!"]

✓ Message 2 sent
  Responses: ["The price of the product **ekoDB** is **$99**."]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"chat_id":{"type":"String","value":"adFsdZahxd4-ZgN8GIc4W6tpwsELsOgwNlPGL1mCj0xpazhykj-eYKJKNQbWWqpJRjwhw3YpCehRvHUpxMp-MQ"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"EHeNIWHrOGmqcvDcchqSacYGnWaUFkI_oXCtXdu_NZoFavfEfg_OuRUj36NKHGUKMT1ViGaEgWnDn6s_4U5ykg","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-10-17T08:35:52.495431+00:00"},"id":"WDIGQpidv_GEh45MqAPPa-ILO5fRQo-oWws0TCzKl1JggMPlNjRWtPplkOtXyCfQKn3kggp0wl2AkV0NV4B5Ig","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":77,"prompt_tokens":6740,"total_tokens":6817}},"updated_at":{"type":"DateTime","value":"2026-10-17T08:35:52.495431+00:00"}},{"chat_id":{"type":"String","value":"adFsdZahxd4-ZgN8GIc4W6tpwsELsOgwNlPGL1mCj0xpazhykj-eYKJKNQbWWqpJRjwhw3YpCehRvHUpxMp-MQ"},"content":{"type":"String","value":"The available product is:\n\n- **Product**: ekoDB\n- **Description**: A high-performance database product with AI capabilities\n- **Price**: $99\n\nIf you need more information or have further questions, feel free to ask!"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"EHeNIWHrOGmqcvDcchqSacYGnWaUFkI_oXCtXdu_NZoFavfEfg_OuRUj36NKHGUKMT1ViGaEgWnDn6s_4U5ykg","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-10-17T08:35:52.512392+00:00"},"id":"fIqGmpjgK1uYS9L6zsYuYfjluNsnOe9xgQbF4Mn7x3OlYdPYJw11UBA1rjVHqSO7dSgYkRKtoqLPLn4M5IvfgA","llm_model":{"type":"String","value":"gpt-4o-mini"},"llm_provider":{"type":"String","value":"openai"},"role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":77,"prompt_tokens":6740,"total_tokens":6817}},"tool_call_count":{"type":"Number","value":2},"tool_call_history":{"type":"Object","value":{"iterations":2,"tool_calls":[{"arguments":{"collection":"kotlin_chat_sessions_example","filter":{},"sort":[{"ascending":true,"field":"id"}]},"id":"call_fqFgO2PYCTDPKZ09MxxZH2ER","name":"query_collection"}],"tool_results":[{"error":null,"result":{"count":1,"records":[{"description":"A high-performance database product with AI capabilities","id":"EHeNIWHrOGmqcvDcchqSacYGnWaUFkI_oXCtXdu_NZoFavfEfg_OuRUj36NKHGUKMT1ViGaEgWnDn6s_4U5ykg","price":99,"product":"ekoDB"}]},"success":true,"tool_call_id":"call_fqFgO2PYCTDPKZ09MxxZH2ER","tool_name":"query_collection"}]}},"updated_at":{"type":"DateTime","value":"2026-10-17T08:35:52.512392+00:00"}},{"chat_id":{"type":"String","value":"adFsdZahxd4-ZgN8GIc4W6tpwsELsOgwNlPGL1mCj0xpazhykj-eYKJKNQbWWqpJRjwhw3YpCehRvHUpxMp-MQ"},"content":{"type":"String","value":"What is the price?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description","product","price"],"record":{"description":"A high-performance database product with AI capabilities","id":"EHeNIWHrOGmqcvDcchqSacYGnWaUFkI_oXCtXdu_NZoFavfEfg_OuRUj36NKHGUKMT1ViGaEgWnDn6s_4U5ykg","price":99,"product":"ekoDB"},"score":2.095}]},"created_at":{"type":"DateTime","value":"2026-10-17T08:35:53.561437+00:00"},"id":"PN_W2OOVydDd-c2Z0b6uZTFzo4TxBfNjZ9pNxJ0VYeawN58AEqoXtkuoGLvy0d6SrUJbam4EiFOHQC9bbazgXQ","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":11,"prompt_tokens":3385,"total_tokens":3396}},"updated_at":{"type":"DateTime","value":"2026-10-17T08:35:53.561437+00:00"}},{"chat_id":{"type":"String","value":"adFsdZahxd4-ZgN8GIc4W6tpwsELsOgwNlPGL1mCj0xpazhykj-eYKJKNQbWWqpJRjwhw3YpCehRvHUpxMp-MQ"},"content":{"type":"String","value":"The price of the product **ekoDB** is **$99**."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description","product","price"],"record":{"description":"A high-performance database product with AI capabilities","id":"EHeNIWHrOGmqcvDcchqSacYGnWaUFkI_oXCtXdu_NZoFavfEfg_OuRUj36NKHGUKMT1ViGaEgWnDn6s_4U5ykg","price":99,"product":"ekoDB"},"score":2.095}]},"created_at":{"type":"DateTime","value":"2026-10-17T08:35:53.577745+00:00"},"id":"QS87fOLSxY7R4rZZT2jigRUiFe_HFb6AOWPEs0DPvkjxQ6nIJB3GVkoZgOImyTlTh6SK-lJscq2pI4Zf4TPOdQ","llm_model":{"type":"String","value":"gpt-4o-mini"},"llm_provider":{"type":"String","value":"openai"},"role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":11,"prompt_tokens":3385,"total_tokens":3396}},"updated_at":{"type":"DateTime","value":"2026-10-17T08:35:53.577745+00:00"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 1

=== Branching Session ===
✓ Created branched session: NErTZs8sPao8JE0pxvyE1YjK6DM98pBneHA5v7EHjtTJyvjgqS5NiWrJwS1Sa304yffBcUC8DnB4obUMsH0e-g

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_sessions_example

✓ Chat session management example completed successfully

BUILD SUCCESSFUL in 11s
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
  - chat_goals__ek0_testing
  - test_accounts
  - chat_messages__ek0_testing
  - schedules__ek0_testing
  - schema_documents_client_go

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
✓ Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
✓ Schema: {"fields":{"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2026-10-17T08:36:02.117871Z","last_modified":"2026-10-17T08:36:02.117871Z","bypass_ripple":false,"primary_key_alias":"id"}

=== Cleanup ===
✓ Deleted collection: kotlin_collection_example

=== Example Complete ===

BUILD SUCCESSFUL in 7s
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
Collection metadata: {"analytics":["collection_utils_test_kt",{"manifest_backed":false,"record_count":5,"resident_record_ids":5,"total_size":675}],"collection":{"bypass_ripple":false,"created_at":"2026-10-17T08:36:11.216847Z","fields":{"index":{"default":null,"enums":[],"field_type":"Integer","max":null,"min":null,"regex":null,"required":false,"unique":false},"name":{"default":null,"enums":[],"field_type":"String","max":null,"min":null,"regex":null,"required":false,"unique":false}},"last_modified":"2026-10-17T08:36:11.217180Z","primary_key_alias":"id","version":1}}

=== List Collections ===
All collections (27):
  - chat_goals__ek0_testing
  - test_accounts
  - chat_messages__ek0_testing
  - schedules__ek0_testing
  - schema_documents_client_go
  - schema_users_client_ts
  - schema_documents_client_ts
  - schema_users_client_js
  - chat_raw_completions__ek0_testing
  - collection_utils_test_kt
  - schema_users_client_go
  - chat_agent_configs__ek0_testing
  - audit__ek0_testing
  - schema_products_client_go
  - test_collection
  - chat_goal_templates__ek0_testing
  - schema_employees_client_ts
  - schema_products_client_ts
  - functions__ek0_testing
  - chat_configurations__ek0_testing
  - chat_tasks__ek0_testing
  - chat_turns__ek0_testing
  - schema_employees_client_go
  - schema_documents_client_js
  - schema_products_client_js
  - agent_function_versions__ek0_testing
  - schema_employees_client_js

=== Check Non-Existent Collection ===
Collection 'nonexistent_collection_xyz_1848180187425375' exists: false

=== Cleanup ===
Deleted collection 'collection_utils_test_kt'

=== Collection Utilities Example Complete ===

BUILD SUCCESSFUL in 8s
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

BUILD SUCCESSFUL in 8s
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
✓ Created record: EkoRecord(fields={id=StringValue(value=3NBt2qe6hTMCpoaYv8viD-woL3UIz2K1ZONmMbOV6YCtot-fYEyeCzk1V00IlWzDWUHSLz6kAksSxgvEu1b1CQ)})

=== Upsert Operation ===
✓ First upsert (update): EkoRecord(fields={active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), id=StringValue(value=3NBt2qe6hTMCpoaYv8viD-woL3UIz2K1ZONmMbOV6YCtot-fYEyeCzk1V00IlWzDWUHSLz6kAksSxgvEu1b1CQ), email=ObjectValue(value={value=StringValue(value=alice.j@newdomain.com), type=StringValue(value=String)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)})})
✓ Second upsert (insert): EkoRecord(fields={id=StringValue(value=new-user-id)})

=== Find One Operation ===
✓ Found user by email: EkoRecord(fields={email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice.j@newdomain.com)}), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), id=StringValue(value=3NBt2qe6hTMCpoaYv8viD-woL3UIz2K1ZONmMbOV6YCtot-fYEyeCzk1V00IlWzDWUHSLz6kAksSxgvEu1b1CQ), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)})})
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

BUILD SUCCESSFUL in 8s
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

BUILD SUCCESSFUL in 8s
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

BUILD SUCCESSFUL in 7s
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
  Document ID: tkU928imfd7Y5rAgCo7o3BaQgxZy9sulEVvoXsIQoEKKPEM9Ej7_NHBvEXdTnN2P-fsOpuQto-I1XNRyS2dvrQ

=== Verify Document Exists ===
✓ Document found: session_id, created_at, id, user_id, ttl

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: 5-BpjyAABKtxkm-CrwVG4UtBFItKUtCyrjcfDTC59cI-lijq2NDdMHAP5hMRoW_Xp9dBZhRt-h_0KAcdgRxNyA

=== TTL Expiration ===
✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document ===
✓ Long TTL document still exists: value, id, cache_key, ttl

=== Delete Document ===
✓ Deleted document

=== Cleanup ===
✓ Deleted collection: kotlin_ttl_example

=== Example Complete ===

BUILD SUCCESSFUL in 7s
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
✓ Edge cache function created: FoK_g1euf4i3jQP4JNc1JlxZ3G_7Gmzb0lSpvSFunvCpGwd9CM-IaBTNfALxXUhrFYc_4s4kSn_Aw1UGgm2SzQ

Call 1: Cache lookup
Found 1 cached entries
Response time: 16ms

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

BUILD SUCCESSFUL in 8s
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
   Name: {"type":"String","value":"User 1"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 71ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 5ms
   📊 Records: 1
   🚀 Cache speedup: 14.2x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: get_verified_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: {"type":"String","value":"User 1"}
   Department: {"value":"engineering","type":"String"}

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - get_verified_user: Specific workflow

🧹 Cleaning up...
✅ Cleanup complete

✅ All composition examples completed!

BUILD SUCCESSFUL in 8s
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

BUILD SUCCESSFUL in 7s
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

✅ Function saved: TJJGi_g7eOnj0GAtUEho4z2lNNjerBc_9VIAJZTmpJQ5lXRh67ERafX0F6AxUE9bO_8CwbV0Q0mRfwivLc4Ayg
📊 Found 5 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

✅ Function saved
📊 Found 3 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 2 groups
   {"avg_score":{"value":50.0,"type":"Float"},"status":{"type":"String","value":"inactive"},"count":{"type":"Integer","value":5}}
   {"status":{"type":"String","value":"active"},"avg_score":{"type":"Float","value":60.0},"count":{"type":"Integer","value":5}}
⏱️  Execution time: 0ms

📝 Example 4: function Management

📋 Total functions: 10
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

BUILD SUCCESSFUL in 9s
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
   {"count":{"type":"Integer","value":5},"category":{"value":"Electronics","type":"String"},"avg_price":{"value":367.0,"type":"Float"}}
   {"count":{"value":3,"type":"Integer"},"avg_price":{"value":365.6666666666667,"type":"Float"},"category":{"type":"String","value":"Furniture"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced function examples finished!

BUILD SUCCESSFUL in 8s
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
   Vector databases offer several benefits, including:

1. **Efficient Similarity Search**: They enable fast retrieval of similar items based on vector representations, useful for AI applications like image and text search.

2. **High-Dimensional Data Handling**: Designed to manage high-dimensional data efficiently, which is essential for machine learning and deep learning models.

3. **Scalability**: Vector databases can scale to handle large datasets without significant performance degradation.

4. **Advanced Query Capabilities**: Support for complex queries using distance metrics, allowing for nuanced searching and filtering.

5. **Integration with Machine Learning**: Seamless integration with ML pipelines, enabling easy storage and retrieval of embeddings.

6. **Real-time Processing**: Capable of supporting real-time data processing and analytics, beneficial for dynamic applications.

7. **Improved User Experience**: Enhances user interactions through personalized content recommendations and fast response times.

8. **Multi-modal Support**: Can handle various data types (text, images, audio) using unified vector representations.

These advantages make vector databases particularly suited for applications in AI-driven domains such as recommendation systems, search engines, and natural language processing.
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

BUILD SUCCESSFUL in 11s
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

✅ Function saved: l62k_hcfefRN-xieAxmtlo98V1qbQYzpyHquPZPyp55_m_rJxCPVgYMMrLwhdS-isQrp4gjVvgLylZDqsOi5FA
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

BUILD SUCCESSFUL in 9s
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
   {"status":{"type":"String","value":"active"},"count":{"type":"Integer","value":7}}
   {"count":{"type":"Integer","value":3},"status":{"value":"inactive","type":"String"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD function examples finished!

BUILD SUCCESSFUL in 8s
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

✅ Inserted order: StringValue(value=GanWG1YqBlSEs0Ue2IpradfNRt8tdwrdYx1_7vbkf2FZBXSjg2eGulAg0qn35dmnh8FISphfhaGsyIcZiGGqog)
✅ Inserted 2 products

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: x2quC6ZVPJzuUxnTYM7NtTu0inq5jalFNiE1w46JOzxYGmNNq4nJTMVT3uv1pbxmanY3XcWkCX72NOfma9q4Jg
📊 Created order via function
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"value":{"userId":"user_abc","role":"admin"},"type":"Object"}
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: Hg5pWl7lT2JrEv5m2fImXgruI7wAXjac7C2qEgaEBEioJW__OQK0UfdmHf6ikdyit3VVmjqylNPvIhNTRGV7dA
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: Ni5EV4-ip83mWDJnnynimueELeBtdRtMoTg9_ExphX3ZdmoVX4FdqdW5Kjb73I1VqXXkBoH8846A2PJC98o4IA
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

BUILD SUCCESSFUL in 17s
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
   1. {"value":"Vector Databases Explained","type":"String"} ({"type":"String","value":"Database"})
   2. {"type":"String","value":"Getting Started with ekoDB"} ({"value":"Database","type":"String"})
   3. {"value":"Introduction to Machine Learning","type":"String"} ({"value":"AI","type":"String"})
   4. {"value":"Natural Language Processing","type":"String"} ({"type":"String","value":"AI"})
   5. {"type":"String","value":"Database Design Principles"} ({"value":"Database","type":"String"})
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"count":{"value":2,"type":"Integer"},"category":{"type":"String","value":"AI"}}
   {"category":{"value":"Database","type":"String"},"count":{"type":"Integer","value":3}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search function examples finished!

BUILD SUCCESSFUL in 13s
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
Created template: Data Migration (id: hef9P7E2EMc8CWuMv1ro9WYkkiAzoNzIQpGg2PONQGx8SaVV5ia-ppNzWP1kUl9aLjyoPKLy60aCEC2B8GBYEQ)

--- Listing templates ---
Templates: {"count":1,"items":[{"description":{"type":"String","value":"Template for migrating data between schemas"},"id":"hef9P7E2EMc8CWuMv1ro9WYkkiAzoNzIQpGg2PONQGx8SaVV5ia-ppNzWP1kUl9aLjyoPKLy60aCEC2B8GBYEQ","steps":{"type":"Array","value":[{"description":"Analyze source schema"},{"description":"Create target schema"},{"description":"Migrate records"},{"description":"Validate results"}]},"title":{"type":"String","value":"Data Migration"}}]}

--- Getting template ---
Fetched: {"type":"String","value":"Data Migration"}

--- Updating template ---
Updated description: {"type":"String","value":"Updated: comprehensive data migration workflow"}

--- Deleting template ---
Template deleted successfully

✓ Goal template CRUD example completed

BUILD SUCCESSFUL in 8s
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
Created goal: 1t0oKR-szZGupIiEa8J-3O0lQFmyBUiJiAYJT5beb-OvueoBaF7ozDfZj-EOANuZYie6iI0OOfqyMV-FlJO3XA

--- Listing goals ---
Goals: {"count":1,"goals":[{"created_at":"2026-10-17T08:39:00.265639+00:00","description":"Ship the next major release","id":"1t0oKR-szZGupIiEa8J-3O0lQFmyBUiJiAYJT5beb-OvueoBaF7ozDfZj-EOANuZYie6iI0OOfqyMV-FlJO3XA","status":"pending","steps":"[{\"description\":\"Run test suite\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to production\"}]","title":"Deploy v2.0","updated_at":"2026-10-17T08:39:00.265639+00:00"}]}

--- Getting goal ---
Fetched: {"type":"String","value":"Deploy v2.0"}

--- Updating goal ---
Updated description: {"type":"String","value":"Ship v2.0 with full test coverage"}

--- Searching goals ---
Search results: {"count":1,"items":[{"_score":12.870000000000001,"created_at":{"type":"DateTime","value":"2026-10-17T08:39:00.265639+00:00"},"description":{"type":"String","value":"Ship v2.0 with full test coverage"},"id":"1t0oKR-szZGupIiEa8J-3O0lQFmyBUiJiAYJT5beb-OvueoBaF7ozDfZj-EOANuZYie6iI0OOfqyMV-FlJO3XA","status":{"type":"String","value":"pending"},"steps":{"type":"String","value":"[{\"description\":\"Run test suite\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0"},"updated_at":{"type":"DateTime","value":"2026-10-17T08:39:00.291733+00:00"}}]}

--- Goal step lifecycle ---
Step 0 started: "1t0oKR-szZGupIiEa8J-3O0lQFmyBUiJiAYJT5beb-OvueoBaF7ozDfZj-EOANuZYie6iI0OOfqyMV-FlJO3XA"
Step 0 completed: "1t0oKR-szZGupIiEa8J-3O0lQFmyBUiJiAYJT5beb-OvueoBaF7ozDfZj-EOANuZYie6iI0OOfqyMV-FlJO3XA"
Step 1 started: "1t0oKR-szZGupIiEa8J-3O0lQFmyBUiJiAYJT5beb-OvueoBaF7ozDfZj-EOANuZYie6iI0OOfqyMV-FlJO3XA"
Step 1 failed: "1t0oKR-szZGupIiEa8J-3O0lQFmyBUiJiAYJT5beb-OvueoBaF7ozDfZj-EOANuZYie6iI0OOfqyMV-FlJO3XA"

--- Completing goal ---
Goal status after complete: {"type":"String","value":"pending_review"}

--- Approving goal ---
Goal status after approve: {"type":"String","value":"in_progress"}

--- Creating goal for rejection ---
Goal status after reject: {"type":"String","value":"failed"}

--- Creating task ---
Created task: GyICa9y2LzNUr5U-rZaSN5M8U3hhAm6GcLxmGpqBhpox-p7jGC0D8wUUCT36RMGOac9u-YBoc6dlhGlT1BWBjw

--- Listing tasks ---
Tasks: {"count":1,"items":[{"cron":{"type":"String","value":"0 2 * * *"},"description":{"type":"String","value":"Full database backup every night at 2 AM"},"id":"GyICa9y2LzNUr5U-rZaSN5M8U3hhAm6GcLxmGpqBhpox-p7jGC0D8wUUCT36RMGOac9u-YBoc6dlhGlT1BWBjw","name":{"type":"String","value":"Nightly Backup"}}]}

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
Created agent: YyTejUkxzKp3t0IrJIMH-5sqBTFJ3ndA6WVOdFtVeghHMNzqxstgGPlI9ExlPmfojtQMepcANxTPgD39hLJ5bw — null

--- Listing agents ---
Agents: {"count":1,"items":[{"deployment_id":{"type":"String","value":"deploy_kt_example"},"id":"YyTejUkxzKp3t0IrJIMH-5sqBTFJ3ndA6WVOdFtVeghHMNzqxstgGPlI9ExlPmfojtQMepcANxTPgD39hLJ5bw","llm_model":{"type":"String","value":"gpt-4.1"},"name":{"type":"String","value":"DataBot"},"system_prompt":{"type":"String","value":"You are a data analysis assistant."}}]}

--- Getting agent by ID ---
Agent: {"type":"String","value":"DataBot"}

--- Getting agent by name ---
By name: {"type":"String","value":"DataBot"}

--- Updating agent ---
Updated agent system_prompt

--- Agents by deployment ---
Agents for deployment: {"count":0,"items":[]}
WARNING: agents-by-deployment omitted created agent YyTejUkxzKp3t0IrJIMH-5sqBTFJ3ndA6WVOdFtVeghHMNzqxstgGPlI9ExlPmfojtQMepcANxTPgD39hLJ5bw; TODO: check/fix the server-side deployment lookup

--- Deleting agent ---
Agent deleted

--- Cleanup: deleting goals ---
Goals deleted

=== Example Complete ===

BUILD SUCCESSFUL in 8s
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

BUILD SUCCESSFUL in 8s
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

BUILD SUCCESSFUL in 7s
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
Links for user:alice: [{"collection":"invoices","document_id":"inv_100","field_path":null,"created_at":"2026-10-17T08:39:25.906843Z","last_accessed":"2026-10-17T08:39:25.910862Z","metadata":{}},{"collection":"orders","document_id":"order_002","field_path":null,"created_at":"2026-10-17T08:39:25.903498Z","last_accessed":"2026-10-17T08:39:25.910862Z","metadata":{}},{"collection":"orders","document_id":"order_001","field_path":null,"created_at":"2026-10-17T08:39:25.900017Z","last_accessed":"2026-10-17T08:39:25.910862Z","metadata":{}}]

--- Unlinking document ---
Unlinked order_002: null

--- Verifying remaining links ---
Remaining links: [{"collection":"invoices","document_id":"inv_100","field_path":null,"created_at":"2026-10-17T08:39:25.906843Z","last_accessed":"2026-10-17T08:39:25.921151Z","metadata":{}},{"collection":"orders","document_id":"order_001","field_path":null,"created_at":"2026-10-17T08:39:25.900017Z","last_accessed":"2026-10-17T08:39:25.921151Z","metadata":{}}]

--- Cleanup ---
Unlinked remaining documents
Deleted key: user:alice

=== Example Complete ===

BUILD SUCCESSFUL in 7s
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
✓ Retrieved value: {"value":{"email":"alice@example.com","name":"Alice","role":"admin"},"type":"Object"}

=== KV Set with TTL ===
✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key ===
✓ Session value: {"value":{"created_at":1792226374310,"user_id":"123"},"type":"Object"}
  (Will expire in 10 seconds)

=== KV Batch Set ===
✓ Batch set 3 keys
  kv_ops_kt_1792226374196:config:db: success
  kv_ops_kt_1792226374196:config:cache: success
  kv_ops_kt_1792226374196:config:api: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  kv_ops_kt_1792226374196:config:db: {"port":5432,"host":"localhost"}
  kv_ops_kt_1792226374196:config:cache: {"ttl":3600,"enabled":true}
  kv_ops_kt_1792226374196:config:api: {"timeout":30,"retries":3}

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
  kv_ops_kt_1792226374196:config:db: deleted
  kv_ops_kt_1792226374196:config:cache: deleted
  kv_ops_kt_1792226374196:config:api: deleted

=== Example Complete ===

BUILD SUCCESSFUL in 7s
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

BUILD SUCCESSFUL in 8s
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

BUILD SUCCESSFUL in 7s
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

BUILD SUCCESSFUL in 9s
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
  - Score: ObjectValue(value={value=IntegerValue(value=95), type=StringValue(value=Integer)})
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

BUILD SUCCESSFUL in 8s
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
Response: The three primary colors are red, blue, and yellow.

--- Structured Output via SSE ---
JSON response: [
  {
    "name": "Mercury",
    "diameter_km": 4879
  },
  {
    "name": "Venus",
    "diameter_km": 12104
  },
  {
    "name": "Earth",
    "diameter_km": 12742
  }
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello! I hope you're having a wonderful day.

=== Done ===

BUILD SUCCESSFUL in 9s
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
Created schedule: 68d22b9a-e0c6-4b94-ac65-39ebc2a9fa37 — "Hourly Health Check"

--- Listing schedules ---
Schedules: {"count":1,"schedules":[{"created_at":"2026-10-17T08:40:28.473343Z","cron_expression":"0 0 * * * *","description":"Ping all services every hour","enabled":true,"function_label":"schedule_noop_kotlin_36627_1792226428340","id":"68d22b9a-e0c6-4b94-ac65-39ebc2a9fa37","last_execution":null,"name":"Hourly Health Check","next_execution":"2026-10-17T09:00:00Z","parameters":{},"stats":{"avg_execution_time_ms":0.0,"failed_executions":0,"last_error":null,"successful_executions":0,"total_executions":0},"timezone":"UTC","updated_at":"2026-10-17T08:40:28.473343Z"}]}

--- Getting schedule ---
Fetched: "Hourly Health Check" (cron: "0 0 * * * *")

--- Updating schedule ---
Updated cron: "0 */30 * * * *"

--- Triggering schedule ---
Trigger response: {"schedule_id":"68d22b9a-e0c6-4b94-ac65-39ebc2a9fa37","status":"triggered"}

--- Pausing schedule ---
Enabled after pause: false

--- Resuming schedule ---
Enabled after resume: true

--- Deleting schedule ---
Schedule deleted successfully

=== Example Complete ===

BUILD SUCCESSFUL in 8s
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
✓ Inserted user 1: StringValue(value=QsAg1enHYrwEnq5vMRePZ6e49VP0_rdbvvHoOdJFrtocU5ojrmblMv0ZQZRvYYqL_VUzBeHnBtDHGikyXY9gEQ)
✓ Inserted user 2: StringValue(value=Q7QNYcC-GJvcLG_JoaBZSzIcogONtMmWIQF223ewQY_O6VVvLwVxSINQtriVE5QKEl4sHU-gZ0uQkTY7O6R1yg)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections ===
✓ Total collections: 27
  Sample: [kotlin_schema_example, chat_goals__ek0_testing, test_accounts, chat_messages__ek0_testing, schedules__ek0_testing]

=== Cleanup ===
✓ Deleted collection: kotlin_schema_example

✓ All schema management operations completed successfully

BUILD SUCCESSFUL in 7s
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
{"value":"Rust Programming Answer","type":"String"}: score=0.9162907318741551, matched=[title]
{"type":"String","value":"Rust Programming Question"}: score=0.9162907318741551, matched=[title]
Paired vector search: document=Rust Programming Answer, query=Rust Programming Question
Filtered vector results: 2
Custom-weight hybrid results: 3
Raw search: {"results":[{"record":{"id":"blByol9Qku1A2OZl6T2T1eAH6NlozvKGAfyFyfTSZvW2sghjy3sxBERAPGMbaElGCEOIMvduvdVtL6KN8aan0A","title":{"type":"String","value":"Rust Programming Answer"},"query_embedding":{"type":"Vector","value":[0.0,1.0,0.0]},"category":{"value":"programming","type":"String"},"document_embedding":{"value":[1.0,0.0,0.0],"type":"Vector"}},"score":1.0,"matched_fields":[]},{"record":{"document_embedding":{"type":"Vector","value":[0.0,1.0,0.0]},"query_embedding":{"value":[1.0,0.0,0.0],"type":"Vector"},"id":"zrVLHylGTY8606KF7vrmNIRVinbBGedL3ZYlEfiI4dttdviAeIoLbbLjznCkNkIcmBqOkSLX0gRGXu-F4XYJ0A","category":{"type":"String","value":"programming"},"title":{"type":"String","value":"Rust Programming Question"}},"score":1.0,"matched_fields":[]}],"total":2,"execution_time_ms":0}

BUILD SUCCESSFUL in 8s
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
✓ Inserted user: EkoRecord(fields={id=StringValue(value=N9lfx47LoSptQ9jVpebMN_fKDePNcnIL8NZdPcV-sNqNhXiaarpQ5iYcrzd6mnA-Blxtqi4k0e1Zf0unyRd3-Q)})
  User ID: N9lfx47LoSptQ9jVpebMN_fKDePNcnIL8NZdPcV-sNqNhXiaarpQ5iYcrzd6mnA-Blxtqi4k0e1Zf0unyRd3-Q

=== Read ===
✓ Found user by ID: EkoRecord(fields={user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), age=ObjectValue(value={value=IntegerValue(value=28), type=StringValue(value=Integer)}), embedding=VectorValue(value=[0.1, 0.2, 0.3, 0.4, 0.5]), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])}), data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)]), type=StringValue(value=Array)}), created_at=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Sat Oct 17 04:40:54 EDT 2026)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), id=StringValue(value=N9lfx47LoSptQ9jVpebMN_fKDePNcnIL8NZdPcV-sNqNhXiaarpQ5iYcrzd6mnA-Blxtqi4k0e1Zf0unyRd3-Q), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Alice Johnson
  email (String): alice@example.com
  age (Integer): 28
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): Sat Oct 17 04:40:54 EDT 2026
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1, tag2, tag3]
  metadata (Object): {key=value, nested={deep=true}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): [electronics, computers]
  data (Bytes): 11 bytes
Record fields: user_id, email, age, embedding, price, categories, data, metadata, tags, created_at, active, id, name

=== Update ===
✓ Updated user: EkoRecord(fields={created_at=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Sat Oct 17 04:40:54 EDT 2026)}), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)}), categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), embedding=VectorValue(value=[0.1, 0.2, 0.3, 0.4, 0.5]), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), id=StringValue(value=N9lfx47LoSptQ9jVpebMN_fKDePNcnIL8NZdPcV-sNqNhXiaarpQ5iYcrzd6mnA-Blxtqi4k0e1Zf0unyRd3-Q), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)})})

=== Query ===
✓ Found 1 users matching query
  - EkoRecord(fields={categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), embedding=VectorValue(value=[0.1, 0.2, 0.3, 0.4, 0.5]), created_at=ObjectValue(value={value=StringValue(value=Sat Oct 17 04:40:54 EDT 2026), type=StringValue(value=String)}), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), metadata=ObjectValue(value={value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})}), type=StringValue(value=Object)}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), city=ObjectValue(value={value=StringValue(value=San Francisco), type=StringValue(value=String)}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)]), type=StringValue(value=Array)}), id=StringValue(value=N9lfx47LoSptQ9jVpebMN_fKDePNcnIL8NZdPcV-sNqNhXiaarpQ5iYcrzd6mnA-Blxtqi4k0e1Zf0unyRd3-Q), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)})})

=== Delete ===
✓ Deleted user with ID: N9lfx47LoSptQ9jVpebMN_fKDePNcnIL8NZdPcV-sNqNhXiaarpQ5iYcrzd6mnA-Blxtqi4k0e1Zf0unyRd3-Q

✓ Confirmed user was deleted

=== Cleanup ===
✓ Deleted collection: kotlin_users_example

=== Example Complete ===

BUILD SUCCESSFUL in 8s
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
  {"data":[{"status":{"value":"active","type":"String"},"name":{"type":"String","value":"Test User"},"id":"IuSbgRLtJgP6H9EtxawgIP6P6HhQkcq0POUdvlaxcL6poZ1gU_C7sMBDpPuN6oQgsIhpiro1DWVf5g_-5ckkxA"}]}

=== Close WebSocket ===
✓ WebSocket closed

=== Cleanup ===
✓ Deleted collection: kotlin_websocket_example

=== Example Complete ===

BUILD SUCCESSFUL in 7s
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
✓ Created native SWR function: github_user_native_kt (m7UAkEKf9ebppGCM3J3k9VC6NNBg5Kuvj6M1wLGUYvkdMeZ8kKYH0wdBhjG6ey7TwamP4pzHqRuT2MxApJvxDA)

First call (cache miss - will fetch from GitHub API):
  Response time: 141ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 5ms
  Speedup: 28.2x faster 🚀


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR function with audit trail: product_swr_audit_kt (zA1tHnztdhmuAj-e4Iz30LwdEys_9D17d9-DD7sHSnwJGz0KS9cGF8lNuBB9dkVnrcfF06TH5dMKrDyqQ7y3jw)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_kt' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_kt (U9K9vZFqzwGIBM69CArfBMHCcPk0Eeuvbq_XgbztbODr2OH1wk7PgVRhhyCqqGIqQv3pJQ_ZrQRZz29UnsLuhg)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_kt' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL function: flexible_cache_kt (cdTcvoLYzlw1TS0NrUBeOMO0-O2MVKHjKFRzh0k80PsYO33ruvMDxKRkmrknIapLQCadt9PZEImB9p6OBU4MOA)
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

BUILD SUCCESSFUL in 9s
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
✓ Created SWR function: swr_cache_lookup_kt_1792226481237 (DSqgLSDvjtCYUeyFUhHh2opB6frOX4Js9o2AkOKI1q9KCNB2dfABVDx64rIH4pU5RvmJ1K7uGTKcGqcaf0z6Aw)

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


BUILD SUCCESSFUL in 8s
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
Created Alice: $1000 - ID: zgHRkTMw-cVg_YYjFsPmGwry8-SB8Uh0kaWfZpGn_gqlhvPnrsC57p-xmdB6beMaLpL8shHU6mFvOzV1VGDhyw
Created Bob: $500 - ID: MFa5pntQimgSBOVwHNT9EEm6JznIgApNI-VLrXz28zDBFRvMLbMbyJOSv-YRNVC8cxvA5BCv49hKNE_PLYDzXg

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): eab10b2b-919b-448a-aee1-82a9694765ea

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
New transaction: 7a1a9132-ecd9-4d3e-91bf-a104046a7ba9
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed

BUILD SUCCESSFUL in 8s
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
Created user function with ID: 7i9XgyX1c6LTSKo-XwUZ0o954ph4gfzW3VG-w9c3msNktcz7dfqzQGIam6fJzsRVQIGideALq8Pn_pgRaSAN7A

=== Get User Function ===
Retrieved: "get_active_users_kt" - "Get Active Users (Kotlin)"
Description: "Fetches all users and filters by active status"

=== List All User Functions ===
Found 9 user functions:
  - "get_active_users_kt": "Get Active Users (Kotlin)"
  - "get_active_users_client_js": "Get Active Users (Updated)"
  - "get_active_users_client_ts_updated": "Get Active Users (Updated)"
  - "get_active_users_client_ts_updated": "Get Active Users (Updated)"
  - "fetch_product_reviews_ts_4706_1792226059130": "Fetch Product with Reviews (Multi-API)"

=== List User Functions by Tag ===
Found 1 user functions with 'kotlin' tag:
  - "get_active_users_kt"

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

=== User Functions Example Complete ===

BUILD SUCCESSFUL in 8s
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

Created chat session: d1Q9i-8EC5lY2Ilaa-jvyUHVEB7_gGNBbegb7uttgGwhxacANjSIXy0dq0Jdhg-N9nr0rMEFWFVjdP-x62lMTg

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: EXK0V1EeOmsT4TXms7FUMWQjMOhlJfD2j0DgxLSqWWGj5RBdEIKV8yzDSCDjLxeuU4elFlqu0GdUURTGmljZ-w
Execution time: 629ms
Token usage: {"completion_tokens":8,"prompt_tokens":15,"total_tokens":23}

Full response: {"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}The capital of France is Paris....

BUILD SUCCESSFUL in 8s
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
✓ Subscribed (subscription_id: sub_891e117f312c4d1fba6b8fcaa36bc7bc)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: "d6Wq-xIMVzcuc54BwqL1VJdGsvu6oy_inJ0_jmHRycrS0HZzczkhOtau2oKxpYDTe90WekyctL-c9LVV12vfqg"

  📡 Notification received:
     Event:      "insert"
     Collection: "ws_subscribe_example_kt"
     Record IDs: ["d6Wq-xIMVzcuc54BwqL1VJdGsvu6oy_inJ0_jmHRycrS0HZzczkhOtau2oKxpYDTe90WekyctL-c9LVV12vfqg"]
     Timestamp:  "2026-10-17T08:41:58.445646+00:00"

Inserting record 2...
✓ Inserted: "f9kEEgIEVOEwz3f1_MlP_t8iyVuSWDXpg1HJiLWHrOlLmSC72TjXJecgwh4q_QsQMhkK4Wb3exAZiGahyIfFUQ"

  📡 Notification received:
     Event:      "insert"
     Record IDs: ["f9kEEgIEVOEwz3f1_MlP_t8iyVuSWDXpg1HJiLWHrOlLmSC72TjXJecgwh4q_QsQMhkK4Wb3exAZiGahyIfFUQ"]

=== Unsubscribing ===
✓ Unsubscribed: {"payload":{"data":{"collection":"ws_subscribe_example_kt","found":true,"unsubscribed":true}},"type":"Success"}

✓ WebSocket subscription example completed successfully
✓ Deleted collection 'ws_subscribe_example_kt'

BUILD SUCCESSFUL in 9s
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
✓ Inserted document with TTL: StringValue(value=di1BfRUdP95eu1--EtUOtLnAedTibs7Wlz2705DttwuqIlYuHxYH_VemtDDlLXbBY4HblkdtGq9pThR8PPZ0MA)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"data":[{"created_at":{"type":"Integer","value":1792226526826},"id":"di1BfRUdP95eu1--EtUOtLnAedTibs7Wlz2705DttwuqIlYuHxYH_VemtDDlLXbBY4HblkdtGq9pThR8PPZ0MA","value":{"type":"Integer","value":42},"name":{"type":"String","value":"WebSocket TTL Test"},"ttl":"2026-10-17T09:42:06.939062Z"}]}

✓ WebSocket closed

=== Cleanup ===
✓ Deleted collection: kotlin_websocket_ttl_example

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration

BUILD SUCCESSFUL in 8s
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
   Inserted with ripple: EkoRecord(fields={id=StringValue(value=J3zGWonQJWsnQFDNhyY8Fja8bjf7MPHmV9Z0IzLQ-hzcyWKTwteqovMVp2VEDpb9vHDmAfL0vG8bjaZeO49NgA)})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: EkoRecord(fields={id=StringValue(value=1UaXzaOYcdJq0VC1nHJdThkf9w7OCkipSOZZJM58mC22v-u7PE4mA8ApVelJPa2rfrvxDOGhBiRirHIxN0FGiQ)})

3. Update with bypass_ripple:
   Updated with bypass_ripple: EkoRecord(fields={name=ObjectValue(value={value=StringValue(value=Product 1), type=StringValue(value=String)}), price=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=150)}), id=StringValue(value=J3zGWonQJWsnQFDNhyY8Fja8bjf7MPHmV9Z0IzLQ-hzcyWKTwteqovMVp2VEDpb9vHDmAfL0vG8bjaZeO49NgA)})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: EkoRecord(fields={name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Upsert Product)}), price=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=500)}), id=StringValue(value=J3zGWonQJWsnQFDNhyY8Fja8bjf7MPHmV9Z0IzLQ-hzcyWKTwteqovMVp2VEDpb9vHDmAfL0vG8bjaZeO49NgA)})

✅ All bypass_ripple operations completed successfully!

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
✅ All Kotlin integration tests complete!
🐍 Building Python client package...
🔧 Ensuring maturin is available in .venv...
🔨 Building wheel...
🍹 Building a mixed python/rust project
🐍 Found CPython 3.11 at /Library/Frameworks/Python.framework/Versions/3.11/bin/python3
🔗 Found pyo3 bindings with abi3-py3.8 support
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.18s
📦 Built wheel for abi3 Python ≥ 3.8 to ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.30.0-cp38-abi3-macosx_11_0_arm64.whl
📦 Installing Python wheel into .venv...
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.30.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.30.0
    Uninstalling ekodb_client-0.30.0:
      Successfully uninstalled ekodb_client-0.30.0
Successfully installed ekodb-client-0.30.0
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
    ✓ Generated embedding: 1536 dimensions in 319.243875ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 243.936125ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 296.556333ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 271.980083ms
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 231.980375ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 509.222583ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 341.754708ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 469.106459ms
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 252.178208ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 308.690834ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 246.308125ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 395.259541ms
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
    ✓ Generated embedding: 1536 dimensions in 296.547ms
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages_rust
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 64.977875ms

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

Writing memory-safe, high-performance database code involves several practices that ensure both safety from memory leaks or corruption and efficiency in executing database operations. Here are some key guidelines to follow:

### 1. Use High-Level Abstractions
- **ORMs and Query Builders**: Using Object-Relational Mappers (ORMs) or query builders can abstract many complexities of direct database interactions, providing built-in protection against SQL injection and type errors.
- **Type Safety**: Languages with strong typing (e.g., Rust, TypeScript) can help ensure type safety at compile time.

### 2. Optimize Database Interactions
- **Batch Queries**: Instead of performing multiple individual database calls, batch your queries when possible to reduce network overhead and improve performance.
- **Connection Pooling**: Use a connection pool to maintain a set of open connections and reuse them for multiple requests, reducing the overhead of establishing a new connection for every operation.
- **Prepared Statements**: Use prepared statements to optimize query execution and enhance security against SQL injection attacks.

### 3. Ensure Memory Safety
- **Automatic Memory Management**: Utilize languages and frameworks that provide automatic garbage collection (like Python, Java) or use safe pointer logic (like Rust) to manage memory automatically.
- **Keep Data Structures Simple**: Use simple data structures that minimize the overhead incurred from memory allocations and deallocations, which can lead to fragmentation and inefficiency.

### 4. Use Efficient Data Structures
- **Indexes**: Create appropriate indexes on your database to speed up query performance. Be sure to tailor them to your specific query patterns.
- **Caching**: Implement caching strategies (like in-memory caches) for frequently accessed data to minimize database load and access times.

### 5. Transaction Management
- **ACID Transactions**: Ensure that your database operations are atomic, consistent, isolated, and durable through proper transaction management. This ensures data integrity.
- **Error Handling**: Implement robust error handling to recover gracefully from database failures, ensuring that resources are cleaned up, and the application state is maintained.

### 6. Monitoring and Profiling
- **Performance Monitoring**: Use monitoring tools to track database performance metrics such as query execution time, cache hit rates, and connection counts.
- **Query Optimization**: Regularly profile your database queries using tools to identify slow queries, and optimize them via indexing or rewriting them.

### 7. Security Considerations
- **Access Controls**: Implement fine-grained access control to protect sensitive data and prevent unauthorized access.
- **Use of TLS**: Ensure secure connections to the database to protect data in transit against eavesdropping or tampering.

### Example Code Structure
```python
# Example in Python using SQLAlchemy
from sqlalchemy import create_engine, Column, Integer, String
from sqlalchemy.orm import sessionmaker
from sqlalchemy.ext.declarative import declarative_base

# Set up the database engine and session
engine = create_engine('sqlite:///example.db')
Session = sessionmaker(bind=engine)

Base = declarative_base()

class User(Base):
    __tablename__ = 'users'
    id = Column(Integer, primary_key=True)
    name = Column(String)

# Memory-safe & high-performance data insertion
def add_user(name):
    session = Session()
    try:
        new_user = User(name=name)
        session.add(new_user)
        session.commit()  # Transaction management for safety
    except Exception as e:
        session.rollback()  # Ensure cleanup on error
        print(f"Error: {e}")
    finally:
        session.close()  # Clean up session

# Remember to configure indexes and optimize your database as needed!
```

### Conclusion
By following these principles and continually assessing and refining your approach, you can write database code that ensures memory safety while maintaining high performance. Regular profiling and adaptation to changing workloads are essential for ongoing optimization.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 261.568666ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 4144 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 235.608792ms
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
  ✓ Text search completed in 38.463334ms

✓ Found 3 messages mentioning ownership:
  1. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  2. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

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
=== ekoDB RAG Conversation System ===

This example shows how ekoDB can power a self-improving AI system
that learns from its own conversation history.

=== Step 1: Building Conversation History ===
Storing previous conversations with embeddings...

  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 34 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.263s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.284s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.225s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.228s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.260s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.251s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.278s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.312s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.232s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.217s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.303s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.235s
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
    ✓ Generated embedding: 1536 dimensions in 0.231s
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages_py
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.054s

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

Writing memory-safe, high-performance database code involves several best practices. Here’s a comprehensive approach combining programming principles, database management, and software design:

### 1. Use Parameterized Queries
- **Why**: Prevents SQL injection attacks and optimizes query performance by allowing the database to cache execution plans.
- **How**: Always use prepared statements when executing SQL queries to ensure that user inputs are treated as parameters, not executable code.

### 2. Connection Pooling
- **Why**: Reduces the overhead of opening and closing database connections, which can be costly in terms of time and resources.
- **How**: Use a connection pool to manage multiple connections to the database in a reusable way, allowing for efficient handling of database requests.

### 3. Efficient Query Design
- **Why**: Poorly designed queries can lead to slow performance, excessive resource usage, and increased memory consumption.
- **How**:
  - Use **indexes** wisely to speed up data retrieval.
  - Avoid **SELECT \***. Instead, select only the columns you need.
  - Limit the use of complex joins and subqueries where simpler queries would suffice.

### 4. Regularly Update Database Statistics
- **Why**: Databases use statistics to create execution plans. Outdated stats can lead to inefficient query execution plans.
- **How**: Ensure your database statistics are fresh by using the appropriate update commands (e.g., `ANALYZE` in PostgreSQL).

### 5. Employ Transaction Management
- **Why**: Transactions ensure data integrity and allow for rollback in case of errors, which is crucial for maintaining a consistent state.
- **How**: Use transactions appropriately using `BEGIN`, `COMMIT`, and `ROLLBACK` commands to ensure atomic operations.

### 6. Monitor and Optimize Performance
- **Why**: Continuous performance monitoring helps identify slow queries and resource bottlenecks.
- **How**:
  - Use query analysis tools (like `EXPLAIN` in SQL databases) to understand query performance.
  - Regularly review slow query logs and optimize them as needed.

### 7. Manage Memory Usage
- **Why**: High-memory usage can lead to performance degradation and application crashes.
- **How**:
  - Set limits for query result sizes to prevent overwhelming memory (e.g., with pagination).
  - Release unused resources (e.g., close unused connections and free memory in applications).

### 8. Server-Side Caching
- **Why**: Caching frequently accessed data reduces database load and improves performance.
- **How**: Utilize caching mechanisms (like Redis or in-memory caches) to store results of frequent queries temporarily.

### 9. Secure Your Database
- **Why**: Ensuring that your database is secure helps prevent data breaches and unauthorized access.
- **How**:
  - Implement strict access controls. Use roles and permissions to limit what users can do.
  - Encrypt sensitive data both in transit and at rest.
  - Regularly audit database access and query logs for suspicious activities.

### 10. Read the Documentation & Community Best Practices
- **Why**: Understanding the database system you are using helps you make the best choices for your use case.
- **How**: Follow best practices and recommendations specific to the database engine you are using (MySQL, PostgreSQL, MongoDB, etc.).

### Conclusion
By following these principles, you can ensure that your database code is not only high-performing but also memory-safe. Regularly reviewing and refining your approach based on performance metrics and evolving needs is essential for long-term success.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.234s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 3590 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.256s
    • Function auto-cleaned up by client
✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...

✓ Found 3 messages mentioning ownership:
  1. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  2. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  3. From conv_new_question: Writing memory-safe, high-performance database code involves several best practices. Here’s a comprehensive approach combining programming principles, database management, and software design:

### 1. Use Parameterized Queries
- **Why**: Prevents SQL injection attacks and optimizes query performance by allowing the database to cache execution plans.
- **How**: Always use prepared statements when executing SQL queries to ensure that user inputs are treated as parameters, not executable code.

### 2. Connection Pooling
- **Why**: Reduces the overhead of opening and closing database connections, which can be costly in terms of time and resources.
- **How**: Use a connection pool to manage multiple connections to the database in a reusable way, allowing for efficient handling of database requests.

### 3. Efficient Query Design
- **Why**: Poorly designed queries can lead to slow performance, excessive resource usage, and increased memory consumption.
- **How**:
  - Use **indexes** wisely to speed up data retrieval.
  - Avoid **SELECT \***. Instead, select only the columns you need.
  - Limit the use of complex joins and subqueries where simpler queries would suffice.

### 4. Regularly Update Database Statistics
- **Why**: Databases use statistics to create execution plans. Outdated stats can lead to inefficient query execution plans.
- **How**: Ensure your database statistics are fresh by using the appropriate update commands (e.g., `ANALYZE` in PostgreSQL).

### 5. Employ Transaction Management
- **Why**: Transactions ensure data integrity and allow for rollback in case of errors, which is crucial for maintaining a consistent state.
- **How**: Use transactions appropriately using `BEGIN`, `COMMIT`, and `ROLLBACK` commands to ensure atomic operations.

### 6. Monitor and Optimize Performance
- **Why**: Continuous performance monitoring helps identify slow queries and resource bottlenecks.
- **How**:
  - Use query analysis tools (like `EXPLAIN` in SQL databases) to understand query performance.
  - Regularly review slow query logs and optimize them as needed.

### 7. Manage Memory Usage
- **Why**: High-memory usage can lead to performance degradation and application crashes.
- **How**:
  - Set limits for query result sizes to prevent overwhelming memory (e.g., with pagination).
  - Release unused resources (e.g., close unused connections and free memory in applications).

### 8. Server-Side Caching
- **Why**: Caching frequently accessed data reduces database load and improves performance.
- **How**: Utilize caching mechanisms (like Redis or in-memory caches) to store results of frequent queries temporarily.

### 9. Secure Your Database
- **Why**: Ensuring that your database is secure helps prevent data breaches and unauthorized access.
- **How**:
  - Implement strict access controls. Use roles and permissions to limit what users can do.
  - Encrypt sensitive data both in transit and at rest.
  - Regularly audit database access and query logs for suspicious activities.

### 10. Read the Documentation & Community Best Practices
- **Why**: Understanding the database system you are using helps you make the best choices for your use case.
- **How**: Follow best practices and recommendations specific to the database engine you are using (MySQL, PostgreSQL, MongoDB, etc.).

### Conclusion
By following these principles, you can ensure that your database code is not only high-performing but also memory-safe. Regularly reviewing and refining your approach based on performance metrics and evolving needs is essential for long-term success.

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
    ✓ Generated embedding: 1536 dimensions in 0.266s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.251s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.204s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.237s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.266s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.294s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.230s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.245s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.270s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.212s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.269s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.271s
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
    ✓ Generated embedding: 1536 dimensions in 0.262s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages_ts
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.053s
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

Writing memory-safe, high-performance database code involves a combination of best practices in database design, efficient querying, and programming techniques. Here are some key strategies:

### 1. **Use Appropriate Data Types**
   - Choose the right data types for your database fields based on the expected values, as this can reduce storage requirements and improve performance.

### 2. **Normalize Data**
   - Normalize your database schema to eliminate redundancy. However, strike a balance, as over-normalization can lead to performance issues with complex joins.

### 3. **Write Efficient Queries**
   - Use indexes judiciously to speed up query performance, especially on columns frequently involved in WHERE, JOIN, and ORDER BY clauses.
   - Avoid SELECT *; instead, specify only the columns you need to minimize memory usage.

### 4. **Batch Operations**
   - Whenever possible, perform batch inserts, updates, and deletes. This reduces the number of database round-trips and can significantly improve performance.

### 5. **Connection Pooling**
   - Use connection pooling to minimize the overhead of opening and closing database connections. This helps manage resources effectively.

### 6. **Prepared Statements**
   - Use prepared statements to help prevent SQL injection attacks. They also improve performance, especially for repeated queries, because the database can cache the parsed query plan.

### 7. **Proper Transaction Management**
   - Use transactions to ensure data integrity. Keep transactions as short as possible to avoid locking issues that can degrade performance.

### 8. **Handle Errors Gracefully**
   - Implement error handling to gracefully manage database errors without leaking resources. Close connections and handle exceptions to prevent memory leaks.

### 9. **Caching Strategies**
   - Implement caching strategies to reduce the load on the database for frequently accessed data. This can be done at the application level or using relevant database caching mechanisms.

### 10. **Use Asynchronous Query Execution**
   - In applications that support it, consider using asynchronous query execution to avoid blocking operations that can impact overall application performance.

### 11. **Profile and Optimize**
   - Regularly profile your database queries and application performance. Use tools to monitor slow queries and analyze execution plans to identify bottlenecks.

### 12. **Leverage NoSQL When Appropriate**
   - If your application involves unstructured data, high write throughput, or requires horizontal scaling, consider using NoSQL databases. They often offer better performance for specific use cases.

### Example Code Snippet
Here’s a simple example using prepared statements in a hypothetical database interaction (using a pseudo code):

```python
# Pseudo code for using prepared statements
connection = get_database_connection()
try:
    with connection.cursor(prepared=True) as cursor:
        # Using a prepared statement for inserting data
        cursor.execute("INSERT INTO users (username, email) VALUES (?, ?)", (username, email))
    connection.commit()
except Exception as e:
    print(f"Error: {e}")
finally:
    connection.close()
```

### Conclusion
By combining these strategies and practices, you can write efficient, memory-safe database code that enhances application performance and maintainability while safeguarding against common pitfalls like SQL injection and resource leakage. Always tailor your approach based on the specific requirements and context of your application.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.269s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 3556 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.247s
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
    ✓ Generated embedding: 1536 dimensions in 0.254s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.271s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.224s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.245s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.269s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.265s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.215s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.226s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.269s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.225s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.225s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.277s
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
    ✓ Generated embedding: 1536 dimensions in 0.227s
    • Function auto-cleaned up by client

→ Executing HybridSearch()...
  • Collection: rag_messages_go
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.052s

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

Writing memory-safe, high-performance database code involves several key practices that encompass both your programming techniques and the database interactions. Here's a comprehensive approach to achieve this:

### 1. Choose the Right Database and Model
- **Select Based on Use Case**: Determine whether a SQL or NoSQL database suits your application needs. SQL databases are great for complex queries and relationships, while NoSQL databases offer flexibility with unstructured data.
- **Use Proper Models**: Design your database schema carefully, applying normalization principles in SQL to avoid redundancy and ensure data integrity. In NoSQL, structure your documents to prevent excessive nesting.

### 2. Efficient Query Design
- **Optimize Queries**: Write efficient queries by minimizing the number of database hits. Use joins wisely in SQL and consider aggregation frameworks in NoSQL.
- **Selective Fields**: Retrieve only the necessary fields instead of using `SELECT *`. This reduces the data transferred and processed.
- **Indexing**: Utilize indexing on commonly queried fields to speed up searches, while being mindful of the index overhead on write operations. Regularly analyze query performance with tools like `query_explain`.

### 3. Connection Management
- **Connection Pooling**: Use connection pooling to manage database connections. This helps to reduce the overhead of creating and destroying connections every time a query is made.
- **Handle Timeouts and Errors Gracefully**: Implement proper error handling and retry logic to manage database connectivity issues without leaking resources.

### 4. Memory Management
- **Avoid Memory Leaks**: Ensure that all database connections are closed properly. Use languages and frameworks that have built-in memory management features (like Rust or Go with their garbage collectors) or ensure manual cleanup in languages like C/C++.
- **Batch Operations**: Use batch inserts/updates to reduce the number of transactions. This minimizes the overhead of multiple database round trips and optimizes memory usage in both your application and database.

### 5. Secure Coding Practices
- **Prepared Statements**: Always use prepared statements or parameterized queries to prevent SQL injection attacks. This enhances not only security but also performance through execution plan reuse.
- **Data Validation**: Validate input at both the application and database levels to prevent the insertion of invalid or malicious data.

### 6. Testing and Monitoring
- **Load Testing**: Simulate high-load scenarios to observe performance and adjust configurations as necessary.
- **Monitoring Tools**: Use monitoring and tracing tools to analyze query performance, response times, and resource consumption.

### 7. Use of Stored Procedures and Functions
- **Encapsulate Logic**: Use stored procedures/functions to encapsulate business logic within the database. This can improve performance by reducing data transfer and allowing the database to optimize execution plans.

### 8. Optimize Data Retrieval Patterns
- **Pagination**: Implement pagination for large data sets to avoid overwhelming memory resources.
- **Caching**: Employ caching strategies to store frequently accessed data in memory, reducing the number of database queries required.

### 9. Database Configuration
- **Tweak Settings**: Tune your database’s configuration settings based on the workload (e.g., memory allocation, buffer sizes).
- **Backup and Recovery**: Design robust backup and recovery strategies that ensure data durability without compromising performance during write-heavy operations.

### Example (Using ekoDB)
If you were using ekoDB as your backend, here’s how you might apply these principles programmatically:

1. **Batch Records**: Use `batch_insert` or `upsert_many` for efficient data handling.
2. **Connection Management**: Leverage ekoDB's built-in features for managing access.
3. **Stored Functions**: Encapsulate complex logic with `create_function` to optimize execution within the database, reducing data transfer needs.

By adhering to these principles and practices, you can develop memory-safe, high-performance database applications that are efficient, secure, and scalable.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.220s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 4233 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.263s
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
  ✓ Text search completed in 0.040s

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
    ✓ Generated embedding: 1536 dimensions in 0.25s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.249s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.273s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.316s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.234s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.265s
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
    ✓ Generated embedding: 1536 dimensions in 0.227s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.234s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.269s
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
    ✓ Generated embedding: 1536 dimensions in 0.244s
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
    ✓ Generated embedding: 1536 dimensions in 0.245s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages_kt
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.074s

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
    ✓ Generated embedding: 1536 dimensions in 0.282s
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
  ✓ Text search completed in 0.031s

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

> @ekodb/ekodb-client@0.30.0 prepare
> npm run build


> @ekodb/ekodb-client@0.30.0 build
> tsc


up to date, audited 44 packages in 562ms

12 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.30.0 build
> tsc

✅ TypeScript client built!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Running TypeScript SWR Examples...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_ts_55318_1792226653562 (zLeM6B2C5avRqk_A9Vvew1q_ALpc18dzWKvLf8IwiKDVo7zN5V9krPJvRvUCDsew6GpFQD3bIZKKK94dR3aF2w)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "value": {
          "username": "Bret",
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "name": "Leanne Graham",
          "address": {
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "zipcode": "92998-3874",
            "city": "Gwenborough",
            "suite": "Apt. 556",
            "street": "Kulas Light"
          },
          "email": "Sincere@april.biz",
          "phone": "1-770-736-8031 x56442",
          "website": "hildegard.org",
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
Response time: 3ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "value": {
          "username": "Bret",
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "name": "Leanne Graham",
          "address": {
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "zipcode": "92998-3874",
            "city": "Gwenborough",
            "suite": "Apt. 556",
            "street": "Kulas Light"
          },
          "email": "Sincere@april.biz",
          "phone": "1-770-736-8031 x56442",
          "website": "hildegard.org",
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
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_reviews_ts_55318_1792226653562 (E9tmJK4VzkAn3zIU-50fY8LNHXDhbUe8NE2E_svVqdHr8SNUEbkIu6ZUYXH5wC4jZBi9G13lhZKYO8a7gwMRyQ)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "value": {
          "weight": 4,
          "rating": 2.56,
          "discountPercentage": 10.48,
          "price": 9.99,
          "stock": 99,
          "shippingInformation": "Ships in 3-5 business days",
          "category": "beauty",
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "meta": {
            "createdAt": "2025-10-09T14:47:01.588Z",
            "updatedAt": "2026-05-23T11:27:41.868Z",
            "barcode": "5784719087687",
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png"
          },
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "title": "Essence Mascara Lash Princess",
          "returnPolicy": "No return policy",
          "warrantyInformation": "1 week warranty",
          "availabilityStatus": "In Stock",
          "id": 1,
          "brand": "Essence",
          "dimensions": {
            "height": 13.08,
            "depth": 22.99,
            "width": 15.14
          },
          "sku": "BEA-ESS-ESS-001",
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "minimumOrderQuantity": 48,
          "reviews": [
            {
              "reviewerName": "Eleanor Collins",
              "rating": 3,
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "comment": "Would not recommend!"
            },
            {
              "reviewerEmail": "lucas.gordon@x.dummyjson.com",
              "rating": 4,
              "reviewerName": "Lucas Gordon",
              "comment": "Very satisfied!",
              "date": "2025-04-30T09:41:02.053Z"
            },
            {
              "rating": 5,
              "date": "2025-04-30T09:41:02.053Z",
              "comment": "Highly impressed!",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "reviewerName": "Eleanor Collins"
            }
          ],
          "tags": [
            "beauty",
            "mascara"
          ]
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
✓ Edge cache script created: z7yryq8hN7I8-MQ1XtaYzN9N07Qlneau_Ps3ymF5w65I1-rf24Oeubi7f2rhhWPjMrJXkHb-6mbEVBSUDAJkTg

Call 1: Cache miss (fetches from API)
Response time: 53ms
Result: {
  "records": [
    {
      "value": {
        "value": {
          "website": "hildegard.org",
          "name": "Leanne Graham",
          "id": 1,
          "username": "Bret",
          "email": "Sincere@april.biz",
          "address": {
            "zipcode": "92998-3874",
            "street": "Kulas Light",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "city": "Gwenborough",
            "suite": "Apt. 556"
          },
          "company": {
            "name": "Romaguera-Crona",
            "catchPhrase": "Multi-layered client-server neural-net",
            "bs": "harness real-time e-markets"
          },
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
Response time: 2ms (26.5x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "website": "hildegard.org",
          "name": "Leanne Graham",
          "id": 1,
          "username": "Bret",
          "email": "Sincere@april.biz",
          "address": {
            "zipcode": "92998-3874",
            "street": "Kulas Light",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "city": "Gwenborough",
            "suite": "Apt. 556"
          },
          "company": {
            "name": "Romaguera-Crona",
            "catchPhrase": "Multi-layered client-server neural-net",
            "bs": "harness real-time e-markets"
          },
          "phone": "1-770-736-8031 x56442"
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

✅ TypeScript SWR examples complete!
🐍 Building Python client package...
🔧 Ensuring maturin is available in .venv...
🔨 Building wheel...
🍹 Building a mixed python/rust project
🐍 Found CPython 3.11 at /Library/Frameworks/Python.framework/Versions/3.11/bin/python3
🔗 Found pyo3 bindings with abi3-py3.8 support
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.10s
📦 Built wheel for abi3 Python ≥ 3.8 to ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.30.0-cp38-abi3-macosx_11_0_arm64.whl
📦 Installing Python wheel into .venv...
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.30.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.30.0
    Uninstalling ekodb_client-0.30.0:
      Successfully uninstalled ekodb_client-0.30.0
Successfully installed ekodb-client-0.30.0
🧪 Ensuring test dependencies (pytest) in .venv...
✅ Python client package built and installed!
📦 Ensuring Python example dependencies in .venv...

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Running Python SWR Examples...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user_swr_py (N-h5C4Ocsx6pO-qI4U1m1i0Fy9y95F5OBMX7s69Mnm7wv1Eq_dioPf2SDaIAsxT4vZmgGs5m0kPokpan64s0RQ)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 171ms
Result: [
  {
    "cached_at": {
      "type": "String",
      "value": "1792226657"
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
        "followers": 328075,
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
        "updated_at": "2026-10-16T15:33:53Z",
        "url": "https://api.github.com/users/torvalds",
        "user_view_type": "public"
      }
    },
    "id": "torvalds"
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 10ms (17.6x faster!)
Result: [
  {
    "cached_at": {
      "type": "String",
      "value": "1792226657"
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
        "followers": 328075,
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
        "updated_at": "2026-10-16T15:33:53Z",
        "url": "https://api.github.com/users/torvalds",
        "user_view_type": "public"
      }
    }
  }
]
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched_swr_py (7U67JOotKw80mpNH7b3ZcOUVirlzjMEd33NQ9KeFPLiTO6mB6_GFLfIEL0MF1_3_EQqEwOpVFxJ4ewUZnoUSNQ)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "enriched_at": {
      "type": "String",
      "value": "1792226657"
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
✓ Created SWR script: fetch_github_user (lFHC_fIPwCFCRf5qXMOhTKBpqh6T1-myTV5fvishS7qfgsPLcuqIYfOsLKGX6DBxIRUKMoJf7R6UVtxp4U-GZw)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 79.80425ms
Result: [
  {
    "cached_at": {
      "type": "DateTime",
      "value": "2026-10-17T08:44:18+00:00"
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
        "followers": 328075,
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
        "updated_at": "2026-10-16T15:33:53Z",
        "url": "https://api.github.com/users/torvalds",
        "user_view_type": "public"
      }
    },
    "id": "torvalds"
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 9.036708ms (8.8x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (uXnuPYlrSXljDD0F8ias3wfSs5FIKYvElNSCUYEwPHzp8KKUK7HKKcPy-7OAwXDKDTtMNL67Ub62uITda2OG1Q)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "enriched_at": {
      "type": "DateTime",
      "value": "2026-10-17T08:44:18+00:00"
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.28s
✅ Client build complete!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Running Rust SWR Examples...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.67s
     Running `target/debug/examples/swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (5WYLsBGjBILwOp4FOwh1W9GDaqVfJkADCKDwmhQSL7F8SeRkVsTjlGAtKqmOONjNlHict7PNOvBGxKrTH4Pu_Q)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 149ms
Result: {
  "data": {
    "value": {
      "gravatar_id": "",
      "location": "Portland, OR",
      "blog": "",
      "node_id": "MDQ6VXNlcjEwMjQwMjU=",
      "created_at": "2011-09-03T15:26:22Z",
      "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
      "login": "torvalds",
      "followers_url": "https://api.github.com/users/torvalds/followers",
      "updated_at": "2026-10-16T15:33:53Z",
      "user_view_type": "public",
      "site_admin": false,
      "public_gists": 1,
      "name": "Linus Torvalds",
      "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
      "url": "https://api.github.com/users/torvalds",
      "bio": null,
      "organizations_url": "https://api.github.com/users/torvalds/orgs",
      "id": 1024025,
      "twitter_username": null,
      "repos_url": "https://api.github.com/users/torvalds/repos",
      "email": null,
      "events_url": "https://api.github.com/users/torvalds/events{/privacy}",
      "type": "User",
      "company": "Linux Foundation",
      "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
      "html_url": "https://github.com/torvalds",
      "received_events_url": "https://api.github.com/users/torvalds/received_events",
      "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
      "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
      "followers": 328075,
      "hireable": null,
      "following": 0,
      "public_repos": 12
    },
    "type": "Object"
  }
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 5ms (29.8x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (TrUjWVrhKR2nmebux5kbFLXN843xr6qPnUCTinPzD9jvz4x5H3K4kYkt41XrRD8xZ_7BkEvyJdS6wl1pYGIx4Q)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: {
  "enriched_data": {
    "type": "Object",
    "value": {
      "images": [
        "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
      ],
      "dimensions": {
        "depth": 22.99,
        "width": 15.14,
        "height": 13.08
      },
      "returnPolicy": "No return policy",
      "title": "Essence Mascara Lash Princess",
      "discountPercentage": 10.48,
      "availabilityStatus": "In Stock",
      "price": 9.99,
      "shippingInformation": "Ships in 3-5 business days",
      "brand": "Essence",
      "sku": "BEA-ESS-ESS-001",
      "rating": 2.56,
      "stock": 99,
      "weight": 4,
      "meta": {
        "barcode": "5784719087687",
        "createdAt": "2025-10-09T14:47:01.588Z",
        "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
        "updatedAt": "2026-05-23T11:27:41.868Z"
      },
      "reviews": [
        {
          "reviewerEmail": "eleanor.collins@x.dummyjson.com",
          "rating": 3,
          "reviewerName": "Eleanor Collins",
          "comment": "Would not recommend!",
          "date": "2025-04-30T09:41:02.053Z"
        },
        {
          "reviewerEmail": "lucas.gordon@x.dummyjson.com",
          "rating": 4,
          "date": "2025-04-30T09:41:02.053Z",
          "comment": "Very satisfied!",
          "reviewerName": "Lucas Gordon"
        },
        {
          "comment": "Highly impressed!",
          "reviewerEmail": "eleanor.collins@x.dummyjson.com",
          "reviewerName": "Eleanor Collins",
          "date": "2025-04-30T09:41:02.053Z",
          "rating": 5
        }
      ],
      "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
      "id": 1,
      "category": "beauty",
      "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
      "tags": [
        "beauty",
        "mascara"
      ],
      "minimumOrderQuantity": 48,
      "warrantyInformation": "1 week warranty"
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
✓ Created SWR function: swr_fetch_github_user_kt_1792226705654 (a6CvEVcbzXYz_PQIAp9pcSVLlUidayWjasG_OV4upREw0-AXPVeN6Nb8PLQ641ReZEvAqt75QH9Bnwg5V-elBA)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 98ms
Result: [{"cached_at":{"type":"String","value":"1792226705665"},"data":{"value":{"email":null,"followers_url":"https://api.github.com/users/torvalds/followers","blog":"","site_admin":false,"html_url":"https://github.com/torvalds","public_gists":1,"gists_url":"https://api.github.com/users/torvalds/gists{/gist_id}","starred_url":"https://api.github.com/users/torvalds/starred{/owner}{/repo}","twitter_username":null,"user_view_type":"public","following_url":"https://api.github.com/users/torvalds/following{/other_user}","company":"Linux Foundation","bio":null,"received_events_url":"https://api.github.com/users/torvalds/received_events","name":"Linus Torvalds","node_id":"MDQ6VXNlcjEwMjQwMjU=","avatar_url":"https://avatars.githubusercontent.com/u/1024025?v=4","created_at":"2011-09-03T15:26:22Z","following":0,"hireable":null,"organizations_url":"https://api.github.com/users/torvalds/orgs","gravatar_id":"","url":"https://api.github.com/users/torvalds","location":"Portland, OR","id":1024025,"type":"User","subscriptions_url":"https://api.github.com/users/torvalds/subscriptions","events_url":"https://api.github.com/users/torvalds/events{/privacy}","followers":328075,"login":"torvalds","public_repos":12,"updated_at":"2026-10-16T15:33:53Z","repos_url":"https://api.github.com/users/torvalds/repos"},"type":"Object"},"id":"torvalds"}]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 6ms
Cache hit was 16.3x faster!

✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment function: swr_fetch_product_kt_1792226705654 (xEyehXn1BpEdmktaqVgDKfLgSnvjUlpEi506wEQ8H592y27xdNE76TENd5Vo0rRF1yWyjQHxCeQcy6_5WRHCyQ)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [{"enriched_at":{"type":"String","value":"1792226705931"},"enriched_data":{"value":{"returnPolicy":"No return policy","availabilityStatus":"In Stock","price":9.99,"thumbnail":"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp","sku":"BEA-ESS-ESS-001","dimensions":{"width":15.14,"depth":22.99,"height":13.08},"stock":99,"category":"beauty","meta":{"createdAt":"2025-10-09T14:47:01.588Z","barcode":"5784719087687","qrCode":"https://cdn.dummyjson.com/public/qr-code.png","updatedAt":"2026-05-23T11:27:41.868Z"},"images":["https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"],"tags":["beauty","mascara"],"id":1,"title":"Essence Mascara Lash Princess","brand":"Essence","minimumOrderQuantity":48,"reviews":[{"rating":3,"date":"2025-04-30T09:41:02.053Z","reviewerEmail":"eleanor.collins@x.dummyjson.com","reviewerName":"Eleanor Collins","comment":"Would not recommend!"},{"reviewerName":"Lucas Gordon","comment":"Very satisfied!","date":"2025-04-30T09:41:02.053Z","reviewerEmail":"lucas.gordon@x.dummyjson.com","rating":4},{"reviewerName":"Eleanor Collins","rating":5,"date":"2025-04-30T09:41:02.053Z","comment":"Highly impressed!","reviewerEmail":"eleanor.collins@x.dummyjson.com"}],"shippingInformation":"Ships in 3-5 business days","description":"The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.","rating":2.56,"warrantyInformation":"1 week warranty","weight":4,"discountPercentage":10.48},"type":"Object"},"id":"1"}]
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


BUILD SUCCESSFUL in 7s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
✅ Client build complete!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Running Rust Function Composition Examples...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
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
   ⏱️  Duration: 324.126917ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "phone": "1-770-736-8031 x56442",
      "website": "hildegard.org",
      "id": 1,
      "company": {
        "catchPhrase": "Multi-layered cl...

Second call (cache hit - from cache):
   ⏱️  Duration: 18.536458ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "username": "Bret",
      "address": {
        "zipcode": "92998-3874",
        "geo": {
          "lng": "81.1496",
          "lat": "-37.315...
   🚀 Cache speedup: 17.5x faster!

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

> @ekodb/ekodb-client@0.30.0 prepare
> npm run build


> @ekodb/ekodb-client@0.30.0 build
> tsc


up to date, audited 44 packages in 724ms

12 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.30.0 build
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
   Name: {"type":"String","value":"User 1"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 61ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "name": "Leanne Graham",
      "email": "Sincere@april.biz",
      "username": "Bret",
      "address": {
        "suite": "Apt. 556",
        "zipcode": "92998-387...

Second call (cache hit - from cache):
   ⏱️  Duration: 3ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "name": "Leanne Graham",
      "email": "Sincere@april.biz",
      "username": "Bret",
      "address": {
        "suite": "Apt. 556",
       ...
   🚀 Cache speedup: 20.3x faster!

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
🐍 Found CPython 3.11 at /Library/Frameworks/Python.framework/Versions/3.11/bin/python3
🔗 Found pyo3 bindings with abi3-py3.8 support
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.10s
📦 Built wheel for abi3 Python ≥ 3.8 to ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.30.0-cp38-abi3-macosx_11_0_arm64.whl
📦 Installing Python wheel into .venv...
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.30.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.30.0
    Uninstalling ekodb_client-0.30.0:
      Successfully uninstalled ekodb_client-0.30.0
Successfully installed ekodb-client-0.30.0
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
   ⏱️  Duration: 59.4ms
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
   ⏱️  Duration: 4.9ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "type": "Object",
            "value": {
                  "address": {
                        "city": "Gwenborough",
                        "geo": {
                 ...
   🚀 Cache speedup: 12.2x faster!

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
   ⏱️  Duration: 67.004542ms
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
   ⏱️  Duration: 5.155458ms
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
   🚀 Cache speedup: 13.4x faster!

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
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 65ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "email": "Sincere@april.biz",
      "phone": "1-770-736-8031 x56442",
      "address": {
        "city": "Gwenborough",
        "suite": "Apt. 556",
        "street...

Second call (cache hit - from cache):
   ⏱️  Duration: 2ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "email": "Sincere@april.biz",
      "phone": "1-770-736-8031 x56442",
      "address": {
        "city": "Gwenborough",
        "suite": "Apt....
   🚀 Cache speedup: 32.5x faster!

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
✅ Client build complete!

🦀 Rust WebSocket Subscription Test...
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_websocket_subscribe`
✓ Authentication successful

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_rs' ===
✓ Subscribed (subscription_id: sub_2effe890a93e4f51b3394484cb313320)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: yHutiIKsougiGdRIwZOtiZvoBCmBHG8c51xsX_N0vbPa_DIBSctBkJkP2uC5Xqnol4e7WE5Awz8gVnee88hPrA

  📡 Notification received:
     Event:      "insert"
     Collection: "ws_subscribe_example_rs"
     Record IDs: ["yHutiIKsougiGdRIwZOtiZvoBCmBHG8c51xsX_N0vbPa_DIBSctBkJkP2uC5Xqnol4e7WE5Awz8gVnee88hPrA"]
     Timestamp:  "2026-10-17T08:48:41.019229+00:00"

Inserting record 2...
✓ Inserted: cJM0ao_Vh1ee5kAQELL4sNkxm63WvJwSCSbAityxhLDVq1ICCg-HU_0MtJRnOOzgCnG-foLvLXn-XJ6Qc5s9Qg

  📡 Notification received:
     Event:      "insert"
     Record IDs: ["cJM0ao_Vh1ee5kAQELL4sNkxm63WvJwSCSbAityxhLDVq1ICCg-HU_0MtJRnOOzgCnG-foLvLXn-XJ6Qc5s9Qg"]

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
✓ Subscribed (subscription_id: sub_ebe2a22118aa48bdafa7798544c9e582)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: 42yp0-EivjGDWqJvIeLovmx5S1ItmXFFDh1_Kga1Pu_1N9NudE84RGnXnpou16j0Y6eNibTE0JWlCK8LzXYyIg

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_go
     Record IDs: [42yp0-EivjGDWqJvIeLovmx5S1ItmXFFDh1_Kga1Pu_1N9NudE84RGnXnpou16j0Y6eNibTE0JWlCK8LzXYyIg]
     Timestamp:  2026-10-17T08:48:41.271344+00:00

Inserting record 2...
✓ Inserted: 3NAc_HjYczdobEe8TRM1nneJBVxm0uCKNCcdTABUpC1i9aLkM5MkKyH-FnXGnXpNAsMIkhG4HP9z4GtLlwh7VA

  📡 Notification received:
     Event:      insert
     Record IDs: [3NAc_HjYczdobEe8TRM1nneJBVxm0uCKNCcdTABUpC1i9aLkM5MkKyH-FnXGnXpNAsMIkhG4HP9z4GtLlwh7VA]

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
✓ Subscribed (subscription_id: sub_e44f5cdaa2f249108726b58e9313c64e)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: WdVe2u6TIn-oBNPJ6bO57_dMfO8LarCSvSACm29sHAWgRqYb6PEdPKmTv8juQGaqmpXeJGe-l_4x6JZomrrBOg

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_py
     Record IDs: WdVe2u6TIn-oBNPJ6bO57_dMfO8LarCSvSACm29sHAWgRqYb6PEdPKmTv8juQGaqmpXeJGe-l_4x6JZomrrBOg
     Timestamp:  2026-10-17T08:48:41.847115+00:00

Inserting record 2...
✓ Inserted: f1Q5F8OS5J5Q2-66p2QW8YvdExJlkSeVUxrftqzh7qntW86BJ5dF3xgqfZpGVqN4kv4pnGpnU_kMt7RPjK-QDw

  📡 Notification received:
     Event:      insert
     Record IDs: f1Q5F8OS5J5Q2-66p2QW8YvdExJlkSeVUxrftqzh7qntW86BJ5dF3xgqfZpGVqN4kv4pnGpnU_kMt7RPjK-QDw

=== Unsubscribing ===
✓ Unsubscribed: {'collection': 'ws_subscribe_example_py', 'found': True, 'unsubscribed': True}

=== Cleanup ===
✓ Deleted collection 'ws_subscribe_example_py'

✓ WebSocket subscription example completed successfully
✅ Python subscription test complete!
📦 Building TypeScript client library...

> @ekodb/ekodb-client@0.30.0 prepare
> npm run build


> @ekodb/ekodb-client@0.30.0 build
> tsc


up to date, audited 44 packages in 586ms

12 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.30.0 build
> tsc

✅ TypeScript client built!

📘 TypeScript WebSocket Subscription Test...
=== WebSocket Subscription Example ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_ts' ===
✓ Subscribed (subscription_id: sub_f49feeeee1884b0db2a826e52324fe43)

=== Performing mutations to trigger notifications ===
Inserting a record...
✓ Inserted record: 79GgGYCcYGwizO8Y9Nyz6F-YAfuSPGdHr1_MTS_VoBP6wEcTe2kiuvXQwtOA-m2QlJwobCZzlpWeEyLGTsJalQ
  📡 Notification received for 79GgGYCcYGwizO8Y9Nyz6F-YAfuSPGdHr1_MTS_VoBP6wEcTe2kiuvXQwtOA-m2QlJwobCZzlpWeEyLGTsJalQ

Inserting another record...
✓ Inserted record: Mg8fnhMoeAV9zz8Gz1uwJy_IXI0_2VtOl3NJ518Q_RiIPnjHyg_SwMv_khCXYXuA5kOxnciiDNX7tK92uJHATw
  📡 Notification received for Mg8fnhMoeAV9zz8Gz1uwJy_IXI0_2VtOl3NJ518Q_RiIPnjHyg_SwMv_khCXYXuA5kOxnciiDNX7tK92uJHATw

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
✓ Subscribed (subscription_id: sub_72900579c2a542f7bd9730d024e545ff)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: "6zFCOaQB9YCvVy1mmdz7w4NPG6mVWNJD0S8LLrn52US14T-rSy0BXpeOfDDINtp8z5d0jIoBYrSBrnW-X9r-Vg"

  📡 Notification received:
     Event:      "insert"
     Collection: "ws_subscribe_example_kt"
     Record IDs: ["6zFCOaQB9YCvVy1mmdz7w4NPG6mVWNJD0S8LLrn52US14T-rSy0BXpeOfDDINtp8z5d0jIoBYrSBrnW-X9r-Vg"]
     Timestamp:  "2026-10-17T08:48:49.841175+00:00"

Inserting record 2...
✓ Inserted: "GN9_cQhiIyWeXg7VoNTfa_a79I0ozpBQ3GBgf30rkboZEtqL0vUfgBloyz91jsXv6HqQimOcbUxoIontp6F8sQ"

  📡 Notification received:
     Event:      "insert"
     Record IDs: ["GN9_cQhiIyWeXg7VoNTfa_a79I0ozpBQ3GBgf30rkboZEtqL0vUfgBloyz91jsXv6HqQimOcbUxoIontp6F8sQ"]

=== Unsubscribing ===
✓ Unsubscribed: {"payload":{"data":{"collection":"ws_subscribe_example_kt","found":true,"unsubscribed":true}},"type":"Success"}

✓ WebSocket subscription example completed successfully
✓ Deleted collection 'ws_subscribe_example_kt'

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
✅ Kotlin subscription test complete!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ All WebSocket Subscription Tests Passed!
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
