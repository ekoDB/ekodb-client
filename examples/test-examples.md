make test-examples
🧪 Running Rust examples (direct HTTP/WebSocket)...
   Compiling ekodb_client v0.32.0 (ekoDB/ekodb-client/ekodb_client)
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.88s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("JrZ1OJUwvhFOmRH4HQ4Sr9Aul0jDHVdyetgzHOmuMsnleL8GbEyFd7l2YT4A8AyZ_geAUJg3BqwsQq88hEG4_w")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("JrZ1OJUwvhFOmRH4HQ4Sr9Aul0jDHVdyetgzHOmuMsnleL8GbEyFd7l2YT4A8AyZ_geAUJg3BqwsQq88hEG4_w"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("JrZ1OJUwvhFOmRH4HQ4Sr9Aul0jDHVdyetgzHOmuMsnleL8GbEyFd7l2YT4A8AyZ_geAUJg3BqwsQq88hEG4_w"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("JrZ1OJUwvhFOmRH4HQ4Sr9Aul0jDHVdyetgzHOmuMsnleL8GbEyFd7l2YT4A8AyZ_geAUJg3BqwsQq88hEG4_w"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.93s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: 4me3LRqHhZT1ydSTLvxve2wl4EUR8ZjpslLUPo0up6w-UiZHop7LNa8PQ0X8_PEAyQWgbf8Jyoxaq3FpY4G4yg

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1794155917639235000",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "4me3LRqHhZT1ydSTLvxve2wl4EUR8ZjpslLUPo0up6w-UiZHop7LNa8PQ0X8_PEAyQWgbf8Jyoxaq3FpY4G4yg",
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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.66s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.62s
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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.60s
     Running `target/debug/examples/collection_management`
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: "vYv-7Ag3dRVKz66Vyhg2KC1FEMrowVJ1WdHBT0yfgPZYHxDbxSAA3icdrMiOO5UDMre1BkQozzasGY3pEnp-hw"

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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.62s
     Running `target/debug/examples/document_ttl`
=== ekoDB Document TTL Example (Rust) ===
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: j_PF0QskTlCd1PBBXvjhwNfLAzLwQ3Wp5nKkSR33AtR1CI-GAHwDDDde12Itji5p_4khHnYMqYXSuj9rrFYPsQ

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: FXKc-2GBHOUsPdHA9-12uR7NT0g0rGSPVPgg9EaLae7rt8JBN41tq5RMwWuq6aHN0WnDyG8krms0x_s7LJvLMg

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: JD1OK6-7S2yA0GfqzlAmvx2yT-UT6bNzUmV4ti8c4JtXQ4txJ9BxvhkDQI6aVnLC0mjijXk48mCJI5YOOZcFkA

=== Query Documents ===
✓ Found 3 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.92s
     Running `target/debug/examples/websocket_ttl`
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: String("GIpqDOQYXUvDvWqgbO1Ibz_s-0q7UahXR_jmggJ2kHfv18uNnKSyVhQpNnHkr0SCDE0RuK3qQjWu5gk8T7k9CA")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record via WebSocket

Record 1:
  id: "GIpqDOQYXUvDvWqgbO1Ibz_s-0q7UahXR_jmggJ2kHfv18uNnKSyVhQpNnHkr0SCDE0RuK3qQjWu5gk8T7k9CA"
  name: {"type":"String","value":"WebSocket TTL Test"}
  ttl: "2026-11-08T17:39:34.425935Z"
  value: {"type":"Integer","value":42}

✓ WebSocket TTL example completed successfully
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.72s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Functions Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: iRfdgtMzX-pJmY2z5yxl6PEoxDeuBAtlVMtV4YbY6BH-4ME1rTpVv9P8sxRBmWTpyqAJw2h2bYeg6YImRBVNjA
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: T2Q6S6NG3z_t9BoFaqLFMPaNrHQF51hC0ZSSa1Gma3f33b7sbROh7MwRDzM_UTlolLXwQHYZpB6qrjmqDWTUOQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Function saved: KG6-2GVFXXEFnWmyd42YUML9a3i2mOH6ySHcFeuJ-48DBRLZ5FR_Ubvk92lfkYGxWkXRtZe_lfzALUwrZYw2tA
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: NnQpKhNHtffJto_TPMuSZGVjzF2qcsbbv61sKx4rt0gQ3gBpvhSA5yEexwwT158I8G56ADyRwfGvq3G1ydRNMQ
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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.60s
     Running `target/debug/examples/transactions`
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: ynf4SK056yKFNiVQ8JQpNWb8jPlI38gw5EU_mTKh6eAZ-Gv7XRAGNwa_RxBJzujrqHegqYclpIybCDHKNZSaCQ
Created Bob: $500 - ID: UdivaiP19Su_LtaJ0ubuoUI_oBsg7ucJ5CS2BQCOaunNTtyRwOs2Bl1TrnLtGggv_b7o7Dv3eHDDzqizrunA6A

=== Example 1: Begin Transaction ===
Transaction ID: 4aff9a00-4d50-4139-96e5-5cbdbda23663

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
New transaction: 2148e872-7609-4691-bc2f-067d6df9a963
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"type":"Integer","value":700}

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✅ Rust direct examples complete!
🛠️  Building client library...
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.19s
✅ Client build complete!
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.86s
     Running `target/debug/examples/client_admin`
Admin permissions read successfully
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.92s
     Running `target/debug/examples/client_advanced_crud`
=== ekoDB Advanced CRUD Example (Rust) ===

--- Inserting base document ---
Inserted: Widget Counter (id: xuLpyH7bJO5TUBsydbBMyW6wxwo4F032RtlXdgCgi2Fv_AXx0rAPzAZNi6bE_H-6Be1QKtsss5jP1KZK9sGfZA)

--- update_with_action: increment views by 10 ---
views after increment: Some(Object({"type": String("Integer"), "value": Integer(110)}))

--- update_with_action: decrement views by 3 ---
views after decrement: Some(Object({"type": String("Integer"), "value": Integer(107)}))

--- update_with_action: multiply score by 2 ---
score after multiply: Some(Object({"value": Float(9.0), "type": String("Float")}))

--- update_with_action: push 'ekodb' to tags ---
tags after push: Some(Object({"value": Array([String("rust"), String("database"), String("ekodb")]), "type": String("Array")}))

--- update_with_action: append '-suffix' to label ---
label after append: Some(Object({"type": String("String"), "value": String("prefix-suffix")}))

--- update_with_action: pop last element from tags ---
tags after pop: Some(Object({"type": String("Array"), "value": Array([String("rust"), String("database")])}))

--- update_with_action: remove 'rust' from tags ---
tags after remove: Some(Object({"value": Array([String("database")]), "type": String("Array")}))

--- update_with_action_sequence: increment views + push tag + append label ---
views after sequence: Some(Object({"value": Integer(157), "type": String("Integer")}))
tags after sequence: Some(Object({"type": String("Array"), "value": Array([String("database"), String("batch")])}))
label after sequence: Some(Object({"type": String("String"), "value": String("prefix-suffix-v2")}))

--- Cleanup ---
Deleted collection

=== All advanced CRUD operations completed ===
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.91s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.04s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 0BxoMrXTzmkmmC3Ch98AqFmd9TY6Xcmcpny8bW-YFYLdQHKP0_Rp3kqG9TAX8UFcEMYTyo2CE6zhnX6IQzKo-w

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
✓ Created second session: M-Fzm8LhraYTnsk4iYELUrcuQaZPanUgPflXmWedHSN7Fj208VaKxalYfDaD8thXmciP6_zywopbeabWdkY5ew
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: M-Fzm8LhraYTnsk4iYELUrcuQaZPanUgPflXmWedHSN7Fj208VaKxalYfDaD8thXmciP6_zywopbeabWdkY5ew
✓ Deleted chat session: 0BxoMrXTzmkmmC3Ch98AqFmd9TY6Xcmcpny8bW-YFYLdQHKP0_Rp3kqG9TAX8UFcEMYTyo2CE6zhnX6IQzKo-w
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.93s
     Running `target/debug/examples/client_chat_basic`
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: lFVuhRCuSsY1dJlHaMtMCHkht26HKxhOaWEt-DMI265pmUtOzFamfL2kRBfWVwxEY4mkpfWvy_4e-yH59N1jag

=== Sending Chat Message ===
Message ID: VDBP58VOiqVJhUsyGdELUpeDFjxVjWGC11PcTU7_0GjHyaG6ZZTk0OH9c1-ouvFvy4g3iFWG1xuB40gSIKlGGQ

=== AI Response ===
Response 1: ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration. Its features include full-text, vector, and hybrid search with automatic context retrieval, plus AI-powered chat that lets you query your database in natural language.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["category", "content", "title"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("f_fqVHV8b-B0BAgXLawBhlLdriYYQr7AzQdD5WU86gS70YAcnyY94lTYcwhI6-sdUIyaEzywotaCULx614d5Rg"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "title"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("MsuzCHJNfpdHsreKl2H9lo1woHLhBLNv2Tzjy0FXCCMVc76WKKJdG0zBOCR3DsEbbV5E2cQzgN9E_I6NZtJbAQ"), "title": String("Introduction to ekoDB")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["category", "content", "title"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("T3GTL3uzBv7_KDpc48eYpoFHC-EoDbwT6j3prXc_K3dRLvE_f52-mq5UxpKxrfF8wu1YbZ154v-rCr73Levzgw"), "title": String("Search Features")}

Execution Time: 4099ms

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.82s
     Running `target/debug/examples/client_chat_message_stream`
=== ekoDB Chat Message Stream (SSE) Example (Rust) ===

Created session: H2MbGNBX88N5aDeDNHk08Uod_XryXe90E-wl7rYVzIh6BxjuY_OLsRBqbqM-33Dmyv7RjeCZgjAI36jzYhfq0Q

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"gpt-6-luna","provider":"openai"}“ekoDB” isn’t a universally recognized database term, and the name may refer to different software projects. If you share the link or context where you saw it, I can tell you which one it is and what it does.

--- Stream complete ---
Message ID: DYrwW6VpAZJuUDhpeg0DgOsV7vSRxaKf3YWsf2GYLPrgmCW9tlYKgXK9QYrlRep0BrlVcSHe_pTkvbF3keNS0g
Execution time: 14314ms
Context window: 128000 tokens

✓ Chat message stream example completed
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.71s
     Running `target/debug/examples/client_chat_models`
✓ Client created

=== Get All Chat Models ===
Available chat models by provider:

OpenAI models (96):
  - tts-1
  - gpt-5-mini
  - gpt-5-mini-2025-08-07
  - gpt-5.4-2026-03-05
  - gpt-5-nano
  - gpt-5.4-mini
  - omni-moderation-2024-09-26
  - gpt-live-transcribe
  - gpt-5.6-luna
  - gpt-4o-2024-11-20
  - gpt-5.1
  - whisper-1
  - gpt-5-pro
  - gpt-realtime-whisper
  - omni-moderation-latest
  - gpt-realtime-2
  - gpt-5-search-api
  - gpt-5.5-2026-04-23
  - gpt-5.5-pro
  - gpt-audio-2025-08-28
  - gpt-4o-mini-transcribe-2025-03-20
  - chat-latest
  - gpt-realtime-translate
  - gpt-5.4-pro-2026-03-05
  - gpt-4o-mini-transcribe-2025-12-15
  - gpt-5-pro-2025-10-06
  - gpt-5.6-sol
  - gpt-5.4-mini-2026-03-17
  - text-embedding-3-large
  - gpt-4o-2024-08-06
  - o3
  - gpt-5.4-pro
  - gpt-4o-mini-tts
  - gpt-realtime-mini-2025-12-15
  - gpt-realtime-2.1-mini
  - gpt-realtime-2025-08-28
  - tts-1-hd-1106
  - tts-1-hd
  - gpt-realtime-1.5
  - gpt-realtime
  - gpt-5.2-pro
  - gpt-4o-transcribe-diarize
  - gpt-5.1-2025-11-13
  - gpt-5.4
  - text-embedding-3-small
  - gpt-5-2025-08-07
  - o3-2025-04-16
  - gpt-5.2-2025-12-11
  - chatgpt-image-latest
  - tts-1-1106
  - gpt-5.5-pro-2026-04-23
  - gpt-5.2-pro-2025-12-11
  - gpt-5.4-nano
  - gpt-audio-mini-2025-12-15
  - gpt-audio-mini
  - gpt-4.1-mini-2025-04-14
  - gpt-5
  - gpt-4o-mini-transcribe
  - gpt-image-2-2026-04-21
  - gpt-transcribe
  - gpt-image-2
  - text-embedding-ada-002
  - gpt-realtime-2.1
  - gpt-4.1
  - gpt-5.5
  - gpt-audio-1.5
  - gpt-image-1.5
  - gpt-6.1-sol
  - gpt-6-sol
  - gpt-realtime-mini
  - gpt-image-1-mini
  - gpt-5.6-terra
  - gpt-5-nano-2025-08-07
  - gpt-4o-transcribe
  - gpt-4.1-mini
  - gpt-4o-mini
  - gpt-6.1-terra
  - gpt-6-luna
  - gpt-4o-mini-tts-2025-12-15
  - gpt-audio
  - gpt-5.3-codex
  - gpt-5-search-api-2025-10-14
  - gpt-4o
  - gpt-image-2.6-flare
  - gpt-4o-mini-2024-07-18
  - gpt-4.1-2025-04-14
  - gpt-image-2.5-flare
  - gpt-4o-mini-tts-2025-03-20
  - gpt-5.2
  - gpt-5.4-nano-2026-03-17
  - gpt-6-astra
  - gpt-image-2.5-sunburst-2026-09-08
  - gpt-image-2.5-sunburst
  - gpt-image-2.5-flare-2026-09-08
  - gpt-image-2.6-sunburst
  - gpt-live-1

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

Gemini models (61):
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
  - gemini-omni-1.2-flash
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
  gemini: ok 61 models
  openai: ok 96 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get Models for Specific Provider ===

openai models (96):
  - tts-1
  - gpt-5-mini
  - gpt-5-mini-2025-08-07
  - gpt-5.4-2026-03-05
  - gpt-5-nano
  ... and 91 more

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.08s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 5OqUkymDz4p07-Jd4NqLQGxQYA63zPQryqDnTI1ZoFNN7G3Fc0G7JyHbsKxmOEs7bF88_XrDAioYyECm5cwbRw
=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is **ekoDB**, priced at **$99**. It’s described as a high-performance database with AI capabilities.

✓ Message 2 sent
  Response: ekoDB is priced at **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: rGHXQJElJsEP-SXcGo9DdwH8WjA-jB5juLAuDWGtPatGiHxQARxzjoz0m-sKYAoE0SwFrC96bq5f-x7ydpqx6Q
  Parent: 5OqUkymDz4p07-Jd4NqLQGxQYA63zPQryqDnTI1ZoFNN7G3Fc0G7JyHbsKxmOEs7bF88_XrDAioYyECm5cwbRw

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: rGHXQJElJsEP-SXcGo9DdwH8WjA-jB5juLAuDWGtPatGiHxQARxzjoz0m-sKYAoE0SwFrC96bq5f-x7ydpqx6Q (Untitled)
  Session 2: 5OqUkymDz4p07-Jd4NqLQGxQYA63zPQryqDnTI1ZoFNN7G3Fc0G7JyHbsKxmOEs7bF88_XrDAioYyECm5cwbRw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: rGHXQJElJsEP-SXcGo9DdwH8WjA-jB5juLAuDWGtPatGiHxQARxzjoz0m-sKYAoE0SwFrC96bq5f-x7ydpqx6Q

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.98s
     Running `target/debug/examples/client_collection_management`
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "PosnDB4ydefz7GzSGwqcQRmDeWv2s1QjT25NKFoE2eQkoM6fA7Ej1PgricuDhGsV8jaXWZ3D4Q1lto0UEvIshQ"

=== List Collections ===
Total collections: 7
Sample collections: ["functions__ek0_testing", "client_collection_management_rust", "audit__ek0_testing", "chat_configurations__ek0_testing", "agent_function_versions__ek0_testing"]

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.95s
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
  - functions__ek0_testing
  - audit__ek0_testing
  - chat_configurations__ek0_testing
  - agent_function_versions__ek0_testing
  - collection_utils_test_rust <-- our test
  - chat_turns__ek0_testing
  - chat_messages__ek0_testing

=== Check Non-Existent Collection ===
Collection 'nonexistent_collection_xyz' exists: false

=== Cleanup ===
Deleted collection 'collection_utils_test_rust'

✓ Collection Utilities example complete
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.84s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.94s
     Running `target/debug/examples/client_convenience_methods`
=== ekoDB Convenience Methods Example ===

=== Record Builder Pattern ===
✓ Created record with fluent builder: Record({"id": String("QPe5sGGNQ5iWfAlvO79f3sQDmf-MGEkI6W1zsqqLmT9OKzocs1d3_WnfzGqByVqI2sbRKc-IG0vxetFkKjVlWg")})

=== Upsert Operation ===
✓ First upsert (insert): Record({"id": String("bob-user-id")})
✓ Second upsert (update): Record({"email": Object({"type": String("String"), "value": String("bob.smith@newdomain.com")}), "name": Object({"value": String("Bob Smith"), "type": String("String")}), "id": String("bob-user-id"), "age": Object({"type": String("Integer"), "value": Integer(36)})})

=== Find One Operation ===
✓ Found user by email: Record({"age": Object({"value": Integer(28), "type": String("Integer")}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "email": Object({"type": String("String"), "value": String("alice@example.com")}), "name": Object({"value": String("Alice Johnson"), "type": String("String")}), "id": String("QPe5sGGNQ5iWfAlvO79f3sQDmf-MGEkI6W1zsqqLmT9OKzocs1d3_WnfzGqByVqI2sbRKc-IG0vxetFkKjVlWg")})
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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.86s
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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.92s
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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.96s
     Running `target/debug/examples/client_document_ttl`
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "BP-kAELrANoOOdKvys3pB82lcXN9ecuV3Z5v0qv7WSN1M4YAKk24N1pvxjFfX3sefXS_LvDN7an8fc0_wzeASw"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("bTEHxPBiijj22Q8HsXW5gZ0GvTUKb28hcJy2C3yMA0NnuWt357dKF8I_01lbOBvy8xffmyETIO227B0T78wFeA"))

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.42s
     Running `target/debug/examples/client_edge_cache`
=== ekoDB as Edge Cache - Simple Example ===

Setting up edge cache collection...
✓ Cache entry created

Creating edge cache lookup script...
✓ Edge cache script created: 6dS7JnplRS22F3eaX7AZUpbmvk5193WXsaFqTU-zxXfF67gsdED3TCk4nR5TqrOfxPBmLR1-CUZodrkUIKP8EQ

Call 1: Cache lookup
Response time: 0ms
Found 1 cached entries

Call 2: Cache lookup (connection warm)
Response time: 0ms
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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.65s
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
   ⏱️  Duration: 117.714125ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "id": 1,
      "phone": "1-770-736-8031 x56442",
      "email": "Sincere@april.biz",
      "name": "Leanne Graham",
      "website": "hildegard.org",
      "address...

Second call (cache hit - from cache):
   ⏱️  Duration: 3.340666ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "email": "Sincere@april.biz",
      "username": "Bret",
      "address": {
        "street": "Kulas Light",
        "suite": "Apt. 556",
        "city": "Gwenboroug...
   🚀 Cache speedup: 35.2x faster!

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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.50s
     Running `target/debug/examples/client_function_contract`
client_function_contract: ok
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.44s
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
   Record({"avg_price": Object({"type": String("Float"), "value": Float(367.0)}), "count": Object({"type": String("Integer"), "value": Integer(5)}), "category": Object({"value": String("Electronics"), "type": String("String")})})
   Record({"category": Object({"value": String("Furniture"), "type": String("String")}), "avg_price": Object({"value": Float(365.6666666666667), "type": String("Float")}), "count": Object({"value": Integer(3), "type": String("Integer")})})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.49s
     Running `target/debug/examples/client_functions_ai`
🚀 ekoDB Rust AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases are designed to store and search **embeddings**—numerical representations of text, images, audio, and other data. Their main benefits include:

- **Similarity search:** Find items that are conceptually or visually similar, even when they don’t share exact keywords.
- **Better semantic search:** Retrieve results based on meaning and context, rather than just matching words.
- **Efficient search at scale:** Specialized indexes can make nearest-neighbor searches across large collections fast.
- **AI application support:** They’re commonly used for retrieval-augmented generation (RAG), recommendations, image search, and duplicate detection.
- **Metadata filtering:** Narrow similarity results by attributes such as date, category, or access permissions.
- **Flexible data types:** Many can work with embeddings for text, images, audio, and other modalities.

They’re most useful when similarity-based retrieval is central to an application. For small datasets or exact-match queries, a conventional database may be simpler and sufficient.
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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.52s
     Running `target/debug/examples/client_functions_complete`
🚀 ekoDB Rust Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: qZm6-OQry3BKZXYZnPXAWZGdPHFoKvuUvYRzeV8UvX94Eu_K3QJT9CdXL7frX7h35HVz256BdDQs7iPymzVPGg
📊 Found 2 product groups
   Record({"count": Object({"value": Integer(2), "type": String("Integer")}), "avg_price": Object({"type": String("Float"), "value": Float(474.0)}), "category": Object({"value": String("Furniture"), "type": String("String")})})
   Record({"category": Object({"type": String("String"), "value": String("Electronics")}), "avg_price": Object({"value": Float(575.6666666666666), "type": String("Float")}), "count": Object({"value": Integer(3), "type": String("Integer")})})
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   Record({"category": Object({"value": String("Electronics"), "type": String("String")}), "count": Object({"value": Integer(3), "type": String("Integer")})})
   Record({"category": Object({"type": String("String"), "value": String("Furniture")}), "count": Object({"value": Integer(2), "type": String("Integer")})})
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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.49s
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
   Record({"count": Object({"type": String("Integer"), "value": Integer(7)}), "status": Object({"type": String("String"), "value": String("active")})})
   Record({"status": Object({"type": String("String"), "value": String("inactive")}), "count": Object({"value": Integer(3), "type": String("Integer")})})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.59s
     Running `target/debug/examples/client_functions_kv_wrapped`
🚀 ekoDB Rust KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: Some(String("o106YHKHj7NgdN2K4fnoy817OL-VqBkE61SD0NXoecj3D_U6u5zp6fNfdwuAwGeg0s9VxRDjZQbfKbdexY_EMQ"))
✅ Inserted 2 products with wrapped types

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: e1tgihfqcNC65-YSbg-K7eNm8rtQpv6J7J86QR8E35Xp8TSQHNiTTT3BbIabanfrpdG2cbBGBMV-4mPVQyZJxw
📊 function executed
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: Some(Object {"type": String("Object"), "value": Object {"role": String("admin"), "userId": String("user_abc")}})
🔍 Key exists: true
✅ Set cached data
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: hqH0zmzTW5oM4hdg9Y6Qy4nWHkRMgaDLMZ4e_Eg7fIZhjbeoj1FehlqGZDt0G2YGpY6FHfpsZWwAU-CHkxXPHA
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: 3fi0Ven6qojsZImge6C9v7P_n-R2_F1Be6rekk2JMcGCCxGcYl0DSw0fc6xhFAIulj0cAoXCrRj31r4b6o6V2g
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.50s
     Running `target/debug/examples/client_functions_search`
🚀 ekoDB Rust Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Getting Started with ekoDB (Database)
   2. Vector Databases Explained (Database)
   3. Introduction to Machine Learning (AI)
   4. Database Design Principles (Database)
   5. Natural Language Processing (AI)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   Record({"category": Object({"type": String("String"), "value": String("AI")}), "count": Object({"type": String("Integer"), "value": Integer(2)})})
   Record({"category": Object({"value": String("Database"), "type": String("String")}), "count": Object({"type": String("Integer"), "value": Integer(3)})})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.45s
     Running `target/debug/examples/client_functions`
=== ekoDB Rust Client - Functions Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: p-A1wEMnVKLd8fj6bebxZOLf5WgSvXG4dHkAfXGgC2zhIH3vPl047LeOyTAOw9dCsRURh3qThA5JjNo1YOF5vQ
📊 Found 5 active users

📝 Example 2: Parameterized Function

✅ Function saved: Qi0xDucL03LC57tyGPK8Di_KymKc8cSxscF0qQmV8NnAL7F4o-AHybMW7lqU0o99hkX1Ne0AsVh13Rz9DxIs5A
📊 Found 3 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: freOjzTlt_G-7j7-W_qxCK_fZ7Hkv7icuV6XHiFkjgqCPn5kyw5Q2nMkRr7nslAAYIsMWjoKhv_nfAcxeUryGg
📊 Statistics: 2 groups

📝 Example 4: Function Management

📋 Total functions: 3
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.


✅ All examples completed!
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.74s
     Running `target/debug/examples/client_goal_templates`
=== ekoDB Goal Template CRUD Example (Rust) ===

--- Creating goal template ---
Created template: Data Migration (id: _tBE72DZQpoEueSAu-fMPkpzUReRP3YbohWuVns40hzoSbhPoyGL2cQy04EnsqnH1C_tHT0F4sJ0tZZ0j5k05w)

--- Listing templates ---
Templates: {"count":1,"items":[{"description":{"type":"String","value":"Template for migrating data between schemas"},"id":"_tBE72DZQpoEueSAu-fMPkpzUReRP3YbohWuVns40hzoSbhPoyGL2cQy04EnsqnH1C_tHT0F4sJ0tZZ0j5k05w","owner":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"steps":{"type":"Array","value":[{"description":"Analyze source schema"},{"description":"Create target schema"},{"description":"Migrate records"},{"description":"Validate results"}]},"title":{"type":"String","value":"Data Migration"}}],"total":1}

--- Getting template ---
Fetched: {"type":"String","value":"Data Migration"}

--- Updating template ---
Updated description: {"type":"String","value":"Updated: comprehensive data migration workflow"}

--- Deleting template ---
Template deleted successfully

✓ Goal template CRUD example completed
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.94s
     Running `target/debug/examples/client_goals_tasks_agents`
=== ekoDB Goals, Tasks & Agents Integration Example (Rust) ===

--- Goal: create ---
Created goal: "Deploy v2.0 to production" (id: 4DKjZcWQz7-gtFMgAoJkC-xMcBNLob-UY4Xj0TE0lqCHiBnNjFJ39Mv3xjqUWvFn162LlW6tP3kfdaWpRj9elQ)

--- Goal: list ---
Goals: {"count":1,"goals":[{"created_at":{"type":"DateTime","value":"2026-11-08T16:46:18.796622+00:00"},"description":{"type":"String","value":"Full release cycle for version 2.0"},"id":"4DKjZcWQz7-gtFMgAoJkC-xMcBNLob-UY4Xj0TE0lqCHiBnNjFJ39Mv3xjqUWvFn162LlW6tP3kfdaWpRj9elQ","owner":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"status":{"type":"String","value":"pending"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-11-08T16:46:18.796622+00:00"}}],"total":1}

--- Goal: get ---
Fetched goal: {"type":"String","value":"Deploy v2.0 to production"}

--- Goal: update ---
Updated description: {"type":"String","value":"Updated: full v2.0 release with rollback plan"}

--- Goal: search ---
Search results: {"count":1,"items":[{"_score":12.87,"created_at":{"type":"DateTime","value":"2026-11-08T16:46:18.796622+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"4DKjZcWQz7-gtFMgAoJkC-xMcBNLob-UY4Xj0TE0lqCHiBnNjFJ39Mv3xjqUWvFn162LlW6tP3kfdaWpRj9elQ","owner":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"status":{"type":"String","value":"pending"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-11-08T16:46:18.829622+00:00"}}]}

--- Goal step: start (step 0) ---
Step 0 started: {"created_at":{"type":"DateTime","value":"2026-11-08T16:46:18.796622+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"4DKjZcWQz7-gtFMgAoJkC-xMcBNLob-UY4Xj0TE0lqCHiBnNjFJ39Mv3xjqUWvFn162LlW6tP3kfdaWpRj9elQ","owner":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"status":{"type":"String","value":"in_progress"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\",\"status\":\"InProgress\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-11-08T16:46:18.837775+00:00"}}

--- Goal step: complete (step 0) ---
Step 0 completed: {"created_at":{"type":"DateTime","value":"2026-11-08T16:46:18.796622+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"4DKjZcWQz7-gtFMgAoJkC-xMcBNLob-UY4Xj0TE0lqCHiBnNjFJ39Mv3xjqUWvFn162LlW6tP3kfdaWpRj9elQ","owner":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"status":{"type":"String","value":"in_progress"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\",\"result\":\"All 247 tests passed\",\"status\":\"Completed\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-11-08T16:46:18.843463+00:00"}}

--- Goal step: start (step 1) ---
Step 1 started

--- Goal step: fail (step 1) ---
Step 1 failed: {"created_at":{"type":"DateTime","value":"2026-11-08T16:46:18.796622+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"4DKjZcWQz7-gtFMgAoJkC-xMcBNLob-UY4Xj0TE0lqCHiBnNjFJ39Mv3xjqUWvFn162LlW6tP3kfdaWpRj9elQ","owner":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"status":{"type":"String","value":"in_progress"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\",\"result\":\"All 247 tests passed\",\"status\":\"Completed\"},{\"description\":\"Build release artifacts\",\"error\":\"Build timeout after 300s\",\"status\":\"Failed\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-11-08T16:46:18.856960+00:00"}}

--- Goal: complete ---
Goal completed (pending review): {"completed_at":{"type":"DateTime","value":"2026-11-08T16:46:18.863213+00:00"},"created_at":{"type":"DateTime","value":"2026-11-08T16:46:18.796622+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"4DKjZcWQz7-gtFMgAoJkC-xMcBNLob-UY4Xj0TE0lqCHiBnNjFJ39Mv3xjqUWvFn162LlW6tP3kfdaWpRj9elQ","owner":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"status":{"type":"String","value":"pending_review"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\",\"result\":\"All 247 tests passed\",\"status\":\"Completed\"},{\"description\":\"Build release artifacts\",\"error\":\"Build timeout after 300s\",\"status\":\"Failed\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"summary":{"type":"String","value":"Partial completion — step 1 failed"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-11-08T16:46:18.863213+00:00"}}

--- Goal: approve ---
Goal approved: {"completed_at":{"type":"DateTime","value":"2026-11-08T16:46:18.863213+00:00"},"created_at":{"type":"DateTime","value":"2026-11-08T16:46:18.796622+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"4DKjZcWQz7-gtFMgAoJkC-xMcBNLob-UY4Xj0TE0lqCHiBnNjFJ39Mv3xjqUWvFn162LlW6tP3kfdaWpRj9elQ","owner":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"status":{"type":"String","value":"in_progress"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\",\"result\":\"All 247 tests passed\",\"status\":\"Completed\"},{\"description\":\"Build release artifacts\",\"error\":\"Build timeout after 300s\",\"status\":\"Failed\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"summary":{"type":"String","value":"Partial completion — step 1 failed"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-11-08T16:46:18.869745+00:00"}}

--- Goal: create (for rejection) ---
Created goal2: "Refactor auth module" (id: vZo3SuTcTdJ2x6gW38_un7_jAaolUJ6XZ0Vgye4yMyMlticpfsRtYopCsk3JQfm8VCsIk8Yy83EHEwgTxqyMsA)

--- Goal: complete (goal2) ---

--- Goal: reject ---
Goal rejected: {"completed_at":{"type":"DateTime","value":"2026-11-08T16:46:18.888951+00:00"},"created_at":{"type":"DateTime","value":"2026-11-08T16:46:18.876025+00:00"},"description":{"type":"String","value":"Rewrite JWT handling"},"id":"vZo3SuTcTdJ2x6gW38_un7_jAaolUJ6XZ0Vgye4yMyMlticpfsRtYopCsk3JQfm8VCsIk8Yy83EHEwgTxqyMsA","owner":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"reason":{"type":"String","value":"Breaks backward compatibility"},"status":{"type":"String","value":"failed"},"steps":{"type":"String","value":"[]"},"summary":{"type":"String","value":"Completed refactor"},"title":{"type":"String","value":"Refactor auth module"},"updated_at":{"type":"DateTime","value":"2026-11-08T16:46:18.888951+00:00"}}

--- Task: create ---
Created task: Nightly backup (id: hBdIH_f_Xd8lpvD8s0_1f5h8qGfXTwc0oGMUeSMO9638BdwUc3y2x5CEpy-suumASS68aU03XmSxOJa2UEg5gw)

--- Task: list ---
Tasks: {"count":1,"items":[{"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"hBdIH_f_Xd8lpvD8s0_1f5h8qGfXTwc0oGMUeSMO9638BdwUc3y2x5CEpy-suumASS68aU03XmSxOJa2UEg5gw","max_retries":{"type":"Integer","value":3},"owner":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"schedule":{"type":"String","value":"0 2 * * *"},"title":{"type":"String","value":"Nightly backup"}}],"total":1}

--- Task: get ---
Fetched task: {"type":"String","value":"Nightly backup"}

--- Task: start ---
Task started: {"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"hBdIH_f_Xd8lpvD8s0_1f5h8qGfXTwc0oGMUeSMO9638BdwUc3y2x5CEpy-suumASS68aU03XmSxOJa2UEg5gw","max_retries":{"type":"Integer","value":3},"owner":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"schedule":{"type":"String","value":"0 2 * * *"},"status":{"type":"String","value":"running"},"title":{"type":"String","value":"Nightly backup"},"updated_at":{"type":"DateTime","value":"2026-11-08T16:46:18.935045+00:00"}}

--- Task: succeed ---
Task succeeded: {"consecutive_failures":{"type":"Integer","value":0},"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"hBdIH_f_Xd8lpvD8s0_1f5h8qGfXTwc0oGMUeSMO9638BdwUc3y2x5CEpy-suumASS68aU03XmSxOJa2UEg5gw","last_error":{"type":"Null","value":null},"last_run":{"type":"DateTime","value":"2026-11-08T16:46:18.940909+00:00"},"max_retries":{"type":"Integer","value":3},"owner":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"run_count":{"type":"Integer","value":1},"schedule":{"type":"String","value":"0 2 * * *"},"status":{"type":"String","value":"active"},"title":{"type":"String","value":"Nightly backup"},"updated_at":{"type":"DateTime","value":"2026-11-08T16:46:18.940909+00:00"}}

--- Task: start (round 2) ---

--- Task: pause ---
Task paused: {"consecutive_failures":{"type":"Integer","value":0},"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"hBdIH_f_Xd8lpvD8s0_1f5h8qGfXTwc0oGMUeSMO9638BdwUc3y2x5CEpy-suumASS68aU03XmSxOJa2UEg5gw","last_error":{"type":"Null","value":null},"last_run":{"type":"DateTime","value":"2026-11-08T16:46:18.940909+00:00"},"max_retries":{"type":"Integer","value":3},"owner":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"run_count":{"type":"Integer","value":1},"schedule":{"type":"String","value":"0 2 * * *"},"status":{"type":"String","value":"paused"},"title":{"type":"String","value":"Nightly backup"},"updated_at":{"type":"DateTime","value":"2026-11-08T16:46:18.952373+00:00"}}

--- Task: resume ---
Task resumed: {"consecutive_failures":{"type":"Integer","value":0},"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"hBdIH_f_Xd8lpvD8s0_1f5h8qGfXTwc0oGMUeSMO9638BdwUc3y2x5CEpy-suumASS68aU03XmSxOJa2UEg5gw","last_error":{"type":"Null","value":null},"last_run":{"type":"DateTime","value":"2026-11-08T16:46:18.940909+00:00"},"max_retries":{"type":"Integer","value":3},"owner":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"run_count":{"type":"Integer","value":1},"schedule":{"type":"String","value":"0 2 * * *"},"status":{"type":"String","value":"active"},"title":{"type":"String","value":"Nightly backup"},"updated_at":{"type":"DateTime","value":"2026-11-08T16:46:18.958367+00:00"}}

--- Task: fail ---
Task failed: {"consecutive_failures":{"type":"Integer","value":1},"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"hBdIH_f_Xd8lpvD8s0_1f5h8qGfXTwc0oGMUeSMO9638BdwUc3y2x5CEpy-suumASS68aU03XmSxOJa2UEg5gw","last_error":{"type":"String","value":"S3 bucket access denied"},"last_run":{"type":"DateTime","value":"2026-11-08T16:46:18.964687+00:00"},"max_retries":{"type":"Integer","value":3},"owner":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"run_count":{"type":"Integer","value":1},"schedule":{"type":"String","value":"0 2 * * *"},"status":{"type":"String","value":"active"},"title":{"type":"String","value":"Nightly backup"},"updated_at":{"type":"DateTime","value":"2026-11-08T16:46:18.964687+00:00"}}

--- Task: due ---
Due tasks: {"count":0,"items":[]}

--- Task: delete ---
Task deleted

--- Agent: create ---
Created agent: backup-agent (id: gqcNt56Jd0AVcZCBjCKJ1XbBrnb7VFBemPIyb__4PEv4BtfQLWvCF1TZD0OrY6L8kLOmdJaNbfkfXnCdmgmLBg)

--- Agent: list ---
Agents: {"count":1,"items":[{"capabilities":{"type":"Array","value":["backup","restore","verify"]},"deployment_id":{"type":"String","value":"deploy-abc-123"},"description":{"type":"String","value":"Handles nightly backups and restores"},"id":"gqcNt56Jd0AVcZCBjCKJ1XbBrnb7VFBemPIyb__4PEv4BtfQLWvCF1TZD0OrY6L8kLOmdJaNbfkfXnCdmgmLBg","llm_model":{"type":"String","value":"gpt-6-luna"},"name":{"type":"String","value":"backup-agent"},"owner":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"run_policy":{"type":"String","value":"{\"trigger\":{\"allow_write_operations\":false,\"write_tools\":[]},\"eval\":{\"allow_write_operations\":false,\"write_tools\":[]},\"customer\":{\"allow_write_operations\":false,\"write_tools\":[]}}"}}],"total":1}

--- Agent: get ---
Fetched agent: {"type":"String","value":"backup-agent"}

--- Agent: get_by_name ---
Found by name: {"type":"String","value":"backup-agent"}

--- Agent: update ---
Updated agent description: {"type":"String","value":"Handles backups, restores, and disaster recovery"}

--- Agent: agents_by_deployment ---
Agents in deployment: {"count":0,"items":[]}
WARNING: agents_by_deployment omitted created agent gqcNt56Jd0AVcZCBjCKJ1XbBrnb7VFBemPIyb__4PEv4BtfQLWvCF1TZD0OrY6L8kLOmdJaNbfkfXnCdmgmLBg; TODO: check/fix the server-side deployment lookup

--- Agent: delete ---
Agent deleted

--- Goal: delete ---
Goal 1 deleted
Goal 2 deleted

=== All goals, tasks & agents operations completed ===
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.94s
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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.85s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.92s
     Running `target/debug/examples/client_kv_links`
=== ekoDB KV Document Linking Example (Rust) ===

--- Inserting documents ---
Inserted doc1: Project Alpha (id: wOoMUK-_NUD5Wakzik2mYphG-ML-W9MYflHhmBJQJpHev2jtiJMCHSF2kJs7q-25ynW2XHsW8u9lrgq0J0XG_Q)
Inserted doc2: Project Beta (id: dEf8r6FGi7H-42MYBPKFK_qNsFf2Pmg2iyP-aRDMCuxKCx2s5DjHE5zIChBQolEuDvEG8AZTr1dEBMYfUwxHDw)

--- Setting KV key ---
Set key: kv_links:rs:user:alice:projects

--- Linking documents ---
Linked doc1: null
Linked doc2: null

--- Getting links ---
Links for kv_links:rs:user:alice:projects: [{"collection":"kv_links_example_rs","created_at":"2026-11-08T16:46:55.217032Z","document_id":"dEf8r6FGi7H-42MYBPKFK_qNsFf2Pmg2iyP-aRDMCuxKCx2s5DjHE5zIChBQolEuDvEG8AZTr1dEBMYfUwxHDw","field_path":null,"last_accessed":"2026-11-08T16:46:55.217476Z","metadata":{}},{"collection":"kv_links_example_rs","created_at":"2026-11-08T16:46:55.216575Z","document_id":"wOoMUK-_NUD5Wakzik2mYphG-ML-W9MYflHhmBJQJpHev2jtiJMCHSF2kJs7q-25ynW2XHsW8u9lrgq0J0XG_Q","field_path":null,"last_accessed":"2026-11-08T16:46:55.217476Z","metadata":{}}]

--- Unlinking doc1 ---
Unlinked doc1: null

--- Verifying remaining links ---
Remaining links: [{"collection":"kv_links_example_rs","created_at":"2026-11-08T16:46:55.217032Z","document_id":"dEf8r6FGi7H-42MYBPKFK_qNsFf2Pmg2iyP-aRDMCuxKCx2s5DjHE5zIChBQolEuDvEG8AZTr1dEBMYfUwxHDw","field_path":null,"last_accessed":"2026-11-08T16:46:55.218376Z","metadata":{}}]

--- Cleanup ---
Unlinked remaining document
Deleted KV key
Deleted documents
Deleted collection

=== All KV linking operations completed ===
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.88s
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
  kv_operations:rs:cache:product:2: Record({"name": String("Product 2"), "price": Float(39.989999999999995)})
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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.82s
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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.84s
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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.90s
     Running `target/debug/examples/client_query_builder`
=== ekoDB Query Builder Example ===

=== Inserting Sample Data ===
✓ Inserted 5 users

=== Simple Equality Query ===
✓ Found 3 active users
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"value": String("David"), "type": String("String")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"value": String("Eve"), "type": String("String")}))
  - Some(Object({"value": String("David"), "type": String("String")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"type": String("String"), "value": String("Eve")}))
  - Some(Object({"value": String("David"), "type": String("String")}))

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
  - Some(Object({"value": String("David"), "type": String("String")}))

=== OR Query ===
✓ Found 2 users with age < 28 OR age > 32
  - Some(Object({"type": String("String"), "value": String("Alice")}))
  - Some(Object({"value": String("Charlie"), "type": String("String")}))

=== Sorted Query (by score descending) ===
✓ Top 3 users by score:
  1. Some(Object({"type": String("String"), "value": String("Bob")}))
  2. Some(Object({"type": String("String"), "value": String("David")}))
  3. Some(Object({"value": String("Alice"), "type": String("String")}))

=== Pagination (page 2, size 2) ===
✓ Page 2 results:
  - Some(Object({"value": String("Charlie"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("David")}))

=== NOT Operator ===
✓ Found 4 users NOT inactive

=== Complex Chained Query ===
✓ Found 3 users with all conditions

=== Cleanup ===
✓ Deleted collection

✓ All query builder operations completed successfully
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.76s
     Running `target/debug/examples/client_raw_completion_stream`
=== ekoDB Raw Completion Stream (SSE) Example ===

--- Basic SSE Raw Completion ---
Response: For traditional art, the primary colors are **red, yellow, and blue**. For light, they’re **red, green, and blue**.

--- Structured Output via SSE ---
JSON response: [
  {"name": "Earth", "diameter_km": 12742},
  {"name": "Mars", "diameter_km": 6779},
  {"name": "Jupiter", "diameter_km": 139820}
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello! How can I help you today?

=== Done ===
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.92s
     Running `target/debug/examples/client_schedules`
=== ekoDB Schedule Management Example (Rust) ===

--- Creating schedule ---
Created schedule: "nightly-cleanup" (id: 760bf402-0463-4ddb-a072-f346e97aab2a)

--- Listing schedules ---
Schedules: {"count":1,"schedules":[{"created_at":"2026-11-08T16:48:11.086946Z","created_by":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"},"cron_expression":"0 0 3 * * *","description":"Remove expired sessions and temp files","enabled":true,"function_label":"schedule_noop_rust_62610","id":"760bf402-0463-4ddb-a072-f346e97aab2a","last_execution":null,"name":"nightly-cleanup","next_execution":"2026-11-09T03:00:00Z","parameters":{},"stats":{"avg_execution_time_ms":0.0,"failed_executions":0,"last_error":null,"successful_executions":0,"total_executions":0},"timezone":"UTC","updated_at":"2026-11-08T16:48:11.086946Z"}]}

--- Getting schedule ---
Fetched schedule: "nightly-cleanup"

--- Updating schedule ---
Updated cron: "0 0 4 * * *"
Updated description: "Remove expired sessions, temp files, and orphaned uploads"

--- Triggering schedule ---
Trigger response: {"schedule_id":"760bf402-0463-4ddb-a072-f346e97aab2a","status":"triggered"}

--- Pausing schedule ---
Schedule paused: {"created_at":"2026-11-08T16:48:11.086946Z","created_by":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"},"cron_expression":"0 0 4 * * *","description":"Remove expired sessions, temp files, and orphaned uploads","enabled":false,"function_label":"schedule_noop_rust_62610","id":"760bf402-0463-4ddb-a072-f346e97aab2a","last_execution":"2026-11-08T16:48:11.109802Z","name":"nightly-cleanup","next_execution":null,"parameters":{"type":"Object","value":{}},"stats":{"avg_execution_time_ms":0.0,"failed_executions":0,"last_error":null,"successful_executions":0,"total_executions":0},"timezone":"UTC","updated_at":"2026-11-08T16:48:11.110482Z"}

--- Resuming schedule ---
Schedule resumed: {"created_at":"2026-11-08T16:48:11.086946Z","created_by":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"},"cron_expression":"0 0 4 * * *","description":"Remove expired sessions, temp files, and orphaned uploads","enabled":true,"function_label":"schedule_noop_rust_62610","id":"760bf402-0463-4ddb-a072-f346e97aab2a","last_execution":"2026-11-08T16:48:11.109802Z","name":"nightly-cleanup","next_execution":"2026-11-09T04:00:00Z","parameters":{"type":"Object","value":{}},"stats":{"avg_execution_time_ms":0.0,"failed_executions":0,"last_error":null,"successful_executions":0,"total_executions":0},"timezone":"UTC","updated_at":"2026-11-08T16:48:11.111492Z"}

--- Deleting schedule ---
Schedule deleted successfully

=== All schedule operations completed ===
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.93s
     Running `target/debug/examples/client_schema`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("m0fJ64XcfSMSwn2p3ja1xCok_TkhN7TEbR_Nw16PI-3NIKAQd4oU0_JaAL9jC_kUvWK2w99qk1HyMTVxhmRcQA"))
✓ Inserted user 2: Some(String("4GPui-G-JuKK83zAtpaIlhLXus1hUmIQ2ueC9YynzYf2EfEpAlnwNN7F4HW_QIrpJBY0WP90tPYHyxWfieETCg"))

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
  Sample: ["chat_raw_completions__ek0_testing", "chat_agent_configs__ek0_testing", "schedules__ek0_testing", "functions__ek0_testing", "audit__ek0_testing"]

=== Cleanup ===
✓ Deleted collection

✓ All schema management operations completed successfully
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.98s
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

Execution time: 0ms
Paired vector search: document=Some("Rust Programming Answer"), query=Some("Rust Programming Question")
=== Cleanup ===
✓ Deleted collection

✓ All search operations completed successfully
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.92s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record({"id": String("nqSqZZYf70d9-_rwjyNuKaeuaEL4YyjUfTFxDMgWaEpXoBeCff9VRMpBq387IjxcwhQSd0dtwFuNjWJ8hlhzTQ")})

=== Find by ID ===
Found: Record({"name": Object({"value": String("Test Record"), "type": String("String")}), "id": String("nqSqZZYf70d9-_rwjyNuKaeuaEL4YyjUfTFxDMgWaEpXoBeCff9VRMpBq387IjxcwhQSd0dtwFuNjWJ8hlhzTQ"), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "created_at": Object({"value": String("2026-11-08T16:48:47.043956+00:00"), "type": String("DateTime")}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "embedding": Vector([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "metadata": Object({"type": String("Object"), "value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})})}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "value": Object({"type": String("Integer"), "value": Integer(42)}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-11-08T16:48:47.043956+00:00")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 5 dims
  categories (Set): 2 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record({"name": Object({"type": String("String"), "value": String("Test Record")}), "embedding": Vector([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "price": Object({"value": Float(99.99), "type": String("Float")}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "metadata": Object({"value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})}), "type": String("Object")}), "created_at": Object({"type": String("DateTime"), "value": String("2026-11-08T16:48:47.043956+00:00")}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "value": Object({"type": String("Integer"), "value": Integer(42)}), "id": String("nqSqZZYf70d9-_rwjyNuKaeuaEL4YyjUfTFxDMgWaEpXoBeCff9VRMpBq387IjxcwhQSd0dtwFuNjWJ8hlhzTQ"), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])})})]

=== Update Document ===
Updated: Record({"metadata": Object({"type": String("Object"), "value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})})}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "embedding": Vector([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "value": Object({"type": String("Integer"), "value": Integer(100)}), "id": String("nqSqZZYf70d9-_rwjyNuKaeuaEL4YyjUfTFxDMgWaEpXoBeCff9VRMpBq387IjxcwhQSd0dtwFuNjWJ8hlhzTQ"), "name": Object({"type": String("String"), "value": String("Updated Record")}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "created_at": Object({"value": String("2026-11-08T16:48:47.043956+00:00"), "type": String("DateTime")}), "price": Object({"value": Float(99.99), "type": String("Float")}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)})})

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.20s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: kAYXBNT7fbZxL06NYl-AglZyYUod_ia2NizZhmgn7AxWZ8WsDj17twj2TMjo8FY_87ohhYSdWf2BSd5hJL5ung

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  active: true
  id: "kAYXBNT7fbZxL06NYl-AglZyYUod_ia2NizZhmgn7AxWZ8WsDj17twj2TMjo8FY_87ohhYSdWf2BSd5hJL5ung"
  name: "WebSocket Test Record"
  value: 42

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.51s
     Running `target/debug/examples/client_swr_native`
=== ekoDB Native SWR Function ===

This example shows the new simplified SWR function that replaces
the manual FindById → If → HttpRequest → Insert pattern.

Example 1: Basic GitHub User Cache with Native SWR
─────────────────────────────────────────────────────

✓ Created native SWR script: github_user_native_rs (O4QRv9KVsfHXjr0hxeH6MWMfcpxn37HjZpssuAOkZ3yICfNEym3QkldLLOqbCN1Jd6XOwLfZWlv1k6zcyceIvQ)

First call (cache miss - will fetch from GitHub API):
  Response time: 208ms
  Records returned: 1
  ✓ Data fetched from API and cached with 15m TTL

Second call (cache hit - instant from KV store):
  Response time: 2ms
  Speedup: 104.0x faster
  ✓ Lightning fast cache hit


Example 2: SWR with Audit Trail Collection
─────────────────────────────────────────────────────

✓ Created SWR script with audit trail: product_swr_audit_rs (83QVb_2BXiS9OWKzx5EvQ9-WyQgsf58WSX3657VttWDL7ZFFUYx4DrpCVnI7dZpdnZPxVkPQ2ByDyG_ZxC-JNQ)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_rs' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
─────────────────────────────────────────────────────

Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_rs (qKtgm2vXJpj5A-GKrefZo-14PFRouPX4N7TUozAZU2Nw0zTWbYEUWxP8QkoI8AmGT7x0Zy_oz9wHQQcgHWbVOA)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_rs' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
─────────────────────────────────────────────────────

✓ Created dynamic TTL script: flexible_cache_rs (RLQAX_0aXDvB-J0YYOXZUy1EgO5_vupq_c0g4Y5G2HRJZHXVQQgVGo4-THengHGR3LcUGCOCEaiOtkNgr38ONg)
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.50s
     Running `target/debug/examples/client_swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_rs (I_Ry9fzyh_WUn0fhRs8V2DAE7dypLlNxCKXKYzwuwYJ1l6iKLW10YPqyoLBy0aWtr73JUy7ufR0OTKz5HJ79Vg)

Step 2: First call - Cache miss, fetches from API
Result: FunctionStats { input_count: 0, output_count: 1, execution_time_ms: 0, stages_executed: 2, stage_stats: [] }
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 1ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.99s
     Running `target/debug/examples/client_transactions`
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 2heAxackTOuwTb9_KTW58_VhIHt3IY_4SXrsG6OD5Jbi38iHeZkanWCDCyRv1WdFdIvXhIywWLvxOtlrJsZGbw
Created Bob: $500 - ID: F2g6h8_Anv_pr2pm6MNls41GT6CXk7GCb5rbVSBldxpJErRsVaC6nSI6_PfE04-1tgtQ4JWKjjWFOTBlZF2HrA

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 96e08787-a205-4fbd-b47f-0ded6b534f39

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
New transaction: 5dfd6612-191a-42ec-b0dc-3ac72207e801
Updated Bob: $700 → $600 (in transaction)
Status before rollback: "Active"
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.41s
     Running `target/debug/examples/client_user_functions`
✓ Client created

=== Create User Function ===
Created user function with ID: 3g0qIglf2Y4YL1H4x7ZS4ouxjxCTnVGOGj0eX9MCGhxyJcon3pzTC6GvAd138g2bYRy3tEHBvgBFGF8X3-WTcg

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
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.00s
     Running `target/debug/examples/client_websocket_chat_stream`
=== WebSocket Chat Streaming Example (Rust) ===

✓ Authentication successful
✓ Created chat session: 9ecfa0U6mvWVdfo8_iJZr-upaUyNEuC2JkNSK9aAB3-jr6nX4KPOto8Wckuc50lGBAJKP-eKTsAUTGGSj-vJIQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Sending message: 'What is the capital of France?' ===
✓ Message accepted, streaming...

{"__progress":"received"}{"__progress":"generating","model":"gpt-5.6-luna","provider":"openai"}The capital of France is **Paris**.

--- Stream ended ---
Message ID: "ZJHiZ9cbM5liW7q-EEEAsoNtU1ZfOPjJGTPhspEPQNsPoRI9ZDB_RmzMXuM4FekggBdpUF_AMDMWrW64aPDl7g"
Execution time: 785ms
Token usage: {"cached_input_tokens":0,"completion_tokens":13,"prompt_tokens":23,"thinking_tokens":0,"total_tokens":36}
Context window: 128000 tokens

Full response: {"__progress":"received"}{"__progress":"generating","model":"gpt-5.6-luna","provider":"openai"}The capital of France is **Paris**....

✓ WebSocket chat streaming example completed successfully
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.91s
     Running `target/debug/examples/client_websocket_subscribe`
✓ Authentication successful

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_rs' ===
✓ Subscribed (subscription_id: sub_5ce84d25d49a4f2ebe6994ee2efde106)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: _dmHYfTSsO9xRjiCfbOJ5NRY1KqSIcroLo4P-RsD51HplnyPpprY0BfxLp_dKNBbZnhySsNAqal7CiV2GBJhDA

  📡 Notification received:
     Event:      "insert"
     Collection: "ws_subscribe_example_rs"
     Record IDs: ["_dmHYfTSsO9xRjiCfbOJ5NRY1KqSIcroLo4P-RsD51HplnyPpprY0BfxLp_dKNBbZnhySsNAqal7CiV2GBJhDA"]
     Timestamp:  "2026-11-08T16:50:17.753880+00:00"

Inserting record 2...
✓ Inserted: t8_R_LrDpYmgLrXcd5F1tEZpgTBB1sXeSt7Tpkrvf_uJQuiFpr0SG3CIHidGpUHp8RvZ1a4EbHgJDJFD4EYx9Q

  📡 Notification received:
     Event:      "insert"
     Record IDs: ["t8_R_LrDpYmgLrXcd5F1tEZpgTBB1sXeSt7Tpkrvf_uJQuiFpr0SG3CIHidGpUHp8RvZ1a4EbHgJDJFD4EYx9Q"]

=== Unsubscribing ===
✓ Unsubscribed

=== Cleanup ===
✓ Deleted collection 'ws_subscribe_example_rs'

✓ WebSocket subscription example completed successfully
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.19s
     Running `target/debug/examples/client_websocket_ttl`
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Some(String("H168ywdE8bir2cSfFsvtbTyWN2_KBtGFmKN-3NLrnblTucslu6w_mr8CiblOrkQkQ14O_RsoNhs6cSGYTpqHfw"))

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.97s
     Running `target/debug/examples/bypass_ripple_example`
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: Record({"id": String("PXkAHjQhaW0SLCcNcmiREv2OrEVsMFOWe5jwXfWv_3hshVCMCUcgHo8omyi7tdxcYzq2bvpiAkXKcR1mK2xqhg")})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: Record({"id": String("lWrSd1DCebAxR5bqC-FAaoTEyhoKcFxRmqTqb_IkW7reevn3z1Ilf_1RfqZYI0MqRM_Nr87e0wdjG0cUozOEbQ")})

3. Update with bypass_ripple:
   Updated with bypass_ripple: Record({"id": String("PXkAHjQhaW0SLCcNcmiREv2OrEVsMFOWe5jwXfWv_3hshVCMCUcgHo8omyi7tdxcYzq2bvpiAkXKcR1mK2xqhg"), "price": Object({"value": Integer(150), "type": String("Integer")}), "name": Object({"type": String("String"), "value": String("Product 1")})})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: Record({"id": String("custom-id")})

✅ All bypass_ripple operations completed successfully!
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.90s
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
  Fields returned: ["created_at", "user_role", "status", "id", "avatar_url", "name", "bio", "age", "email"]

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)

Example 4: Query inactive users with profile fields
  Found 1 inactive users

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ["id", "password", "secret_token", "status", "email", "name", "age", "user_role", "created_at", "bio", "avatar_url", "api_key"]
  Projected query:
    - 3 fields per record
    - Fields: ["name", "id", "email"]
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
Inserted: {'id': 'OELepit3Lgzj45HpBv_tsDerl9T_URMlDk63UPr-XxgHZTWqwqvRratrA1xXb88xlDO__UNoJrRMgqBk7Enzkg'}

=== Find by ID ===
Found: {'name': {'value': 'Test Record', 'type': 'String'}, 'active': {'type': 'Boolean', 'value': True}, 'id': 'OELepit3Lgzj45HpBv_tsDerl9T_URMlDk63UPr-XxgHZTWqwqvRratrA1xXb88xlDO__UNoJrRMgqBk7Enzkg', 'value': {'value': 42, 'type': 'Integer'}}

=== Find with Query ===
Found documents: [{'value': {'type': 'Integer', 'value': 42}, 'name': {'type': 'String', 'value': 'Test Record'}, 'id': 'OELepit3Lgzj45HpBv_tsDerl9T_URMlDk63UPr-XxgHZTWqwqvRratrA1xXb88xlDO__UNoJrRMgqBk7Enzkg', 'active': {'value': True, 'type': 'Boolean'}}]

=== Update Document ===
Updated: {'active': {'type': 'Boolean', 'value': True}, 'value': {'type': 'Integer', 'value': 100}, 'id': 'OELepit3Lgzj45HpBv_tsDerl9T_URMlDk63UPr-XxgHZTWqwqvRratrA1xXb88xlDO__UNoJrRMgqBk7Enzkg', 'name': {'type': 'String', 'value': 'Updated Record'}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: PMwDVoTXyzsbBfUOXD9YkJgWAPJ_GYDGEZaMtm2SX0kS94io2_aAx8opoI-GkqCD_Uf8Hm6fVWGkGn858T3shw

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
          "value": "WebSocket Test Record",
          "type": "String"
        },
        "id": "PMwDVoTXyzsbBfUOXD9YkJgWAPJ_GYDGEZaMtm2SX0kS94io2_aAx8opoI-GkqCD_Uf8Hm6fVWGkGn858T3shw",
        "active": {
          "value": true,
          "type": "Boolean"
        }
      }
    ]
  },
  "messageId": "229620708"
}
✓ Retrieved 1 record via WebSocket

✓ WebSocket example completed successfully
🚀 ekoDB Functions Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: LBoNX32cBSmZjwRwpbr4sr2F0lY6RAJVr_TIVAI8qMU7YyEan4hyU2GswQRLvyO1SCxLxrMENN_iwZtaxxFraQ
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: LMek5zyg4OfSrCrU-2ylqTDE8stq5rZ7pFJ6kZ9pA-gWGfvOeLjnXNcvMiygjN0AvgkjZII97UQDikRKhDT0iA
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: AcUsoNfUcXIdBmsPgubcHCuq9gvqJPrXi1byDDjxExvvzm6bskGvgzZs27r3Q6SJBqqYJ4Gea9TcR-lUf7DgfQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'avg_score': {'value': 50.0, 'type': 'Float'}, 'count': {'value': 5, 'type': 'Integer'}, 'status': {'value': 'inactive', 'type': 'String'}, 'max_score': {'type': 'Integer', 'value': 90}}
   {'max_score': {'type': 'Integer', 'value': 100}, 'avg_score': {'type': 'Float', 'value': 60.0}, 'count': {'type': 'Integer', 'value': 5}, 'status': {'type': 'String', 'value': 'active'}}

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
Retrieved value: {'value': {'username': 'john_doe', 'userId': 123}, 'type': 'Object'}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
kv_operations:direct:py:cache:product:1: {'value': {'price': 29.99, 'name': 'Product 1'}, 'type': 'Object'}
kv_operations:direct:py:cache:product:2: {'value': {'price': 39.989999999999995, 'name': 'Product 2'}, 'type': 'Object'}
kv_operations:direct:py:cache:product:3: {'value': {'name': 'Product 3', 'price': 49.989999999999995}, 'type': 'Object'}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: JiKSKO-sKIa_Pga8xPIcc5RmsN9IVl6uzY4bVRJKZoR3c-03SOSrBmnbrEXS5d30TqKAlBlG-SJ9KhjO4HnDIQ

=== List Collections ===
Total collections: 13
Sample collections: ['chat_raw_completions__ek0_testing', 'chat_agent_configs__ek0_testing', 'schedules__ek0_testing', 'functions__ek0_testing', 'audit__ek0_testing']

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
Created Alice: $1000 - ID: R7XdyMgIGe3iVeZZIz_1XGolkHSYSZ1pfNPT3_ZaulN9JjsK9Hm-nNvjHlQdCc2KMAiTbwJsEDNHvA0LZbm_vg
Created Bob: $500 - ID: vinnfCdvnx4YNYj02pKondcXRwzsAv1W6RA214d05UlzueumotgHydJXeexdvFU0aWwjuZDtbbRH7_WLqoLj0A

=== Example 1: Begin Transaction ===
Transaction ID: 2e723d36-195e-4ccb-8076-9918e047de94

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
Bob: ${'value': 700, 'type': 'Integer'}

=== Example 5: Rollback ===
New transaction: 34596cfa-947b-4733-a9f2-512200e803cf
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: FV8UhBRQMiD5O5DhuDTuOEULg4Gjo8HSi0K3Zn0MJ3kCk94-_bYebA8P_gqm1vHQXmim9EWlO4fOOZzUcQQLEA
Created Bob: $500 - ID: iNgrLOLYsEMWd8wpSbKC7dpJloDZxHdg7ke6CQROG-n8gpZD_ELNfx2ul6e1Gr2_R0PhH1C6lZB2GESOo5dbcg

=== Example 1: Begin Transaction ===
Transaction ID: cc800e30-caa7-4070-a447-0c87f28f456d

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
Created Alice: $1000 - ID: Or44EmU_6FYJr0E8yVWfHhCXZZFh1vlItbDk2vpAt13Pyr9t_Iec6YETZvIBU3HZ14iVwPdour6msdwmOLeaCg
Created Bob: $500 - ID: rAskWtvq-Fd7k0sN6hJxY-zmvprDcMCLirZiGC5v6cyQ-h1uDkNYM_UECOoFVHWWd2qzLUZ6VLHyHft7pR3PtQ

=== Example 1: Begin Transaction ===
Transaction ID: 850a9aab-0834-4538-86c4-e64e1dee9fa7

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
New transaction: ebf95359-3f13-4515-98ed-79a8cfc4514d
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: iU76r7oQ_2sa2FrfOrlC4f_EXQbvU4riYIkKzlAcXFhcdUcqXvTiI-OmVXjrzEK6fQob_l1T2h4_iGSN0I5CeQ
Created Bob: $500 - ID: xoiMvw_2SrPuPOXarUXg47i3ijtE9-gbLsfgDLIHl7ruiC9jDi-wsxOEtwPEVzj0G3MQnpA3ZsPsdr_2r1P2Zg

=== Example 1: Begin Transaction ===
Transaction ID: 273c19fc-2f27-4a9b-8121-f7499a732ccb

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'type': 'Integer', 'value': 800}
Bob: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ Deleted test accounts


🔄 Switching to DURABLE mode...
   ✓ Config updated: durable_operations=true


============================================================
🔥 TEST 3: Durable Mode (durable=true)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: j27Fy7egSEpVak7n9xJ3ZDjCpU1G4GaE-SllOk_FIJghjsZePFOhm_2bSuOIC1Few2PMj4JroiytIPHrVJQArg
Created Bob: $500 - ID: o8BKwxdK2FubKnv2J0TXUOqV4NHJztKYk0QKqo6g_YBYQsIZmAneT91vKMDK9XsCNMhRUef6E6fMkbpqzS43WA

=== Example 1: Begin Transaction ===
Transaction ID: e0785a45-89e5-4846-816b-509ca0d29403

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
New transaction: ed3c7e9e-5d6f-4841-bd59-03b1f95723fb
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: MCL2bAgU23RdS9Ke2-e9f2lMcfeGqWyFN5RYdq7xyevRAjH4GGzDGIR8rRl1Dm7NIWKXrqSmubP-5CD8v49pLw
Created Bob: $500 - ID: E2vFLrK5J-imsTJzy3j9b-TuKA8EFEqlkXF7eZMv5ugSxfqICdEr21tnS33uQW1uryPAMcqlHl7qj2QSBQz3hA

=== Example 1: Begin Transaction ===
Transaction ID: 05d570d9-6cbc-48ce-b620-ffc9ab00907b

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
   ✅ Function saved: SC4dHLlFHj-lScnCbqApNvRCvr74FAXaDi4nJKvDieX1r11bzp9xyUHvS9hBqBckw0ZOISqsUpujdjzpoPwwww

2️⃣ Calling function (Insert + Verify)...
   ✅ function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: 8AxfXhMTQgNfX3d42Oa5k6og7z8IRU_fuJMYpFavwjZOl9XjWBjFuxLXiWvFjj_-Ov8nMVNug1m3dVTVXesJSg
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'type': 'String', 'value': 'pending'}
   📋 Credits: {'type': 'Integer', 'value': 0}

============================================================
📝 function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: 9FT-bkhPNQ3naWwNRw2Zimmm9ljJXdzzHWnkP-B9kNUhE7QqFeUbsYGVcJx5-rR22sCriUZONbqBv58DH_oANg

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
   ✅ Function saved: HhIAcaH9bH-IbuioRrFy9NonswwqAuNiLMHHvynzLfCK39LwIK3W5gfKZsJG4BgtTHFuej-bu_anva1U9xa4Pw

2️⃣ Calling function (Query + Update Credits + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'value': 100, 'type': 'Integer'}
   📋 Status: {'type': 'String', 'value': 'active'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: 8QvM3vAkaY4XCjEzGnB_Woz37rhtdeUA5Cziu84k3Y1b6HecsgZ_wJ4r_Q-kQvrHMy3kno3XTzFMPk1ZdHoy3Q

2️⃣ Calling function (Query + Delete + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: SC4dHLlFHj-lScnCbqAp...
   ✅ Deleted script: 9FT-bkhPNQ3naWwNRw2Z...
   ✅ Deleted script: HhIAcaH9bH-IbuioRrFy...
   ✅ Deleted script: 8QvM3vAkaY4XCjEzGnB_...
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
  Output: Document ID = wxsAi_MqhN_-GTEgvrbD8EKZuxSIkpZahcrAZInFrFCWhLd6mPsFIU33XIIwNuksfgh_s0Q8YJGlV9F6Q6hFKg
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(wxsAi_MqhN_-GTEgvrbD8EKZuxSIkpZahcrAZInFrFCWhLd6mPsFIU33XIIwNuksfgh_s0Q8YJGlV9F6Q6hFKg)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(wxsAi_MqhN_-GTEgvrbD8EKZuxSIkpZahcrAZInFrFCWhLd6mPsFIU33XIIwNuksfgh_s0Q8YJGlV9F6Q6hFKg)
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
  Output: Document ID = T8Dnoux2iwX8ljnnh4dvWo3UZwV3-d3RGJApNngR7zHxlWw98LMy5AXUYFRgjGo6F17tCz37FKfiIPHq55zqlg
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(T8Dnoux2iwX8ljnnh4dvWo3UZwV3-d3RGJApNngR7zHxlWw98LMy5AXUYFRgjGo6F17tCz37FKfiIPHq55zqlg)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(T8Dnoux2iwX8ljnnh4dvWo3UZwV3-d3RGJApNngR7zHxlWw98LMy5AXUYFRgjGo6F17tCz37FKfiIPHq55zqlg)
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
    Finished `release` profile [optimized] target(s) in 0.16s
📦 Built wheel for abi3 Python ≥ 3.8 to ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.32.0-cp38-abi3-macosx_11_0_arm64.whl
📦 Installing Python wheel into .venv...
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.32.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.32.0
    Uninstalling ekodb_client-0.32.0:
      Successfully uninstalled ekodb_client-0.32.0
Successfully installed ekodb-client-0.32.0
🧪 Ensuring test dependencies (pytest) in .venv...
✅ Python client package built and installed!
Admin permissions read successfully
=== ekoDB Advanced CRUD Example (Python) ===

--- insert ---
Inserted: r0YMs22Jh-dPVO2g08wOTfexG_qxWAF5krbVQ6tBTRW3tqYdPmIQVS6PJ6XA3aTWyj1RGo4YE2MYUNQtzZ6-NA

--- update_with_action (increment) ---
After increment count by 5: {'score': {'value': 50.0, 'type': 'Float'}, 'count': {'type': 'Integer', 'value': 15}, 'id': 'r0YMs22Jh-dPVO2g08wOTfexG_qxWAF5krbVQ6tBTRW3tqYdPmIQVS6PJ6XA3aTWyj1RGo4YE2MYUNQtzZ6-NA', 'tags': {'value': ['initial'], 'type': 'Array'}, 'name': {'value': 'Counter Record', 'type': 'String'}}

--- update_with_action (decrement) ---
After decrement score by 10: {'name': {'type': 'String', 'value': 'Counter Record'}, 'count': {'value': 15, 'type': 'Integer'}, 'id': 'r0YMs22Jh-dPVO2g08wOTfexG_qxWAF5krbVQ6tBTRW3tqYdPmIQVS6PJ6XA3aTWyj1RGo4YE2MYUNQtzZ6-NA', 'tags': {'value': ['initial'], 'type': 'Array'}, 'score': {'type': 'Float', 'value': 40.0}}

--- update_with_action_sequence ---
After action sequence: {'name': {'value': 'Counter Record', 'type': 'String'}, 'count': {'type': 'Integer', 'value': 115}, 'tags': {'value': ['initial', 'sequenced'], 'type': 'Array'}, 'score': {'type': 'Float', 'value': 40.0}, 'id': 'r0YMs22Jh-dPVO2g08wOTfexG_qxWAF5krbVQ6tBTRW3tqYdPmIQVS6PJ6XA3aTWyj1RGo4YE2MYUNQtzZ6-NA'}

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
✓ Created session: QtPLxQJYy2M0cRiArnUGNoM8j9h9bIpfLXdSvHAJe8OqK2L3sSY_zMASmkHZpdicAC4vop8W-pcL-OV9sWoJfg

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is **ekoDB**, a high-performance database product priced at **$99**.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['llm_provider', 'content', 'chat_id', 'llm_model', 'context_snippets', 'updated_at', 'token_usage', 'created_at', 'role', 'id', 'author'])
Debug: First message role: {'type': 'String', 'value': 'assistant'}
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: ekoDB costs **$99**.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: 1W_q_bIMG38iwwd-q4M57r7mI3f4eo2HTpmc1vZmTIAfAkIAJDbflEocLOoVmRP3wjkez2LMngNpBRonrC9MmQ
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: 1W_q_bIMG38iwwd-q4M57r7mI3f4eo2HTpmc1vZmTIAfAkIAJDbflEocLOoVmRP3wjkez2LMngNpBRonrC9MmQ
✓ Deleted chat session: QtPLxQJYy2M0cRiArnUGNoM8j9h9bIpfLXdSvHAJe8OqK2L3sSY_zMASmkHZpdicAC4vop8W-pcL-OV9sWoJfg
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: Nuggvdpkd_XPSgCOKunOMmAUj8Z5eFf1mlCEA6o8tlzKIB2bRPZOIDhpJLPo1d2U9cxNQEA6_aHhugdBfOnNyw

=== Sending Chat Message ===
Message ID: _ECIIFUCuaaP02ne_A-As0_KEhj-jlbUWHZWH4l63X394qIw1IPl0e78lgXttTPCrR5HnFbSsRNMW5EB86JIrw

=== AI Response ===
Available products and prices:

- **ekoDB** — 99
- **ekoDB Pro** — 299
- **ekoDB Cloud** — 499

Execution Time: 6161ms

=== Token Usage ===
Prompt tokens: 55699
Completion tokens: 253
Total tokens: 55952

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Message Stream (SSE) Example (Python) ===

Created session: Sl_qhnJmRYr-xxeRIVU_tLaF-A-byMgkPleiRGT5ZjQGGQUbFDeB123ejhwR6DqyLXJ1g3iJ9zzvjAwDP66XlA

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"gpt-6-luna","provider":"openai"}**ekoDB** is a database platform for building applications. The name can refer to different projects, so its exact features depend on which one you mean. If you share a link or where you saw it, I can give you a more precise explanation.

--- Stream complete ---
Message ID: 21F_EQv5Xkba86e8nF_ZtPYW2tV8oSz6VlLRHAggt_jzsNKVjMZM_6xfvRHb0LHTWHqsjK4FOYAvwzstuuORDQ
Execution time: 6338ms
Context window: 128000 tokens

✓ Chat message stream example completed
✓ Client created

=== Get All Chat Models ===
OpenAI models: ['tts-1', 'gpt-5-mini', 'gpt-5-mini-2025-08-07', 'gpt-5.4-2026-03-05', 'gpt-5-nano', 'gpt-5.4-mini', 'omni-moderation-2024-09-26', 'gpt-live-transcribe', 'gpt-5.6-luna', 'gpt-4o-2024-11-20', 'gpt-5.1', 'whisper-1', 'gpt-5-pro', 'gpt-realtime-whisper', 'omni-moderation-latest', 'gpt-realtime-2', 'gpt-5-search-api', 'gpt-5.5-2026-04-23', 'gpt-5.5-pro', 'gpt-audio-2025-08-28', 'gpt-4o-mini-transcribe-2025-03-20', 'chat-latest', 'gpt-realtime-translate', 'gpt-5.4-pro-2026-03-05', 'gpt-4o-mini-transcribe-2025-12-15', 'gpt-5-pro-2025-10-06', 'gpt-5.6-sol', 'gpt-5.4-mini-2026-03-17', 'text-embedding-3-large', 'gpt-4o-2024-08-06', 'o3', 'gpt-5.4-pro', 'gpt-4o-mini-tts', 'gpt-realtime-mini-2025-12-15', 'gpt-realtime-2.1-mini', 'gpt-realtime-2025-08-28', 'tts-1-hd-1106', 'tts-1-hd', 'gpt-realtime-1.5', 'gpt-realtime', 'gpt-5.2-pro', 'gpt-4o-transcribe-diarize', 'gpt-5.1-2025-11-13', 'gpt-5.4', 'text-embedding-3-small', 'gpt-5-2025-08-07', 'o3-2025-04-16', 'gpt-5.2-2025-12-11', 'chatgpt-image-latest', 'tts-1-1106', 'gpt-5.5-pro-2026-04-23', 'gpt-5.2-pro-2025-12-11', 'gpt-5.4-nano', 'gpt-audio-mini-2025-12-15', 'gpt-audio-mini', 'gpt-4.1-mini-2025-04-14', 'gpt-5', 'gpt-4o-mini-transcribe', 'gpt-image-2-2026-04-21', 'gpt-transcribe', 'gpt-image-2', 'text-embedding-ada-002', 'gpt-realtime-2.1', 'gpt-4.1', 'gpt-5.5', 'gpt-audio-1.5', 'gpt-image-1.5', 'gpt-6.1-sol', 'gpt-6-sol', 'gpt-realtime-mini', 'gpt-image-1-mini', 'gpt-5.6-terra', 'gpt-5-nano-2025-08-07', 'gpt-4o-transcribe', 'gpt-4.1-mini', 'gpt-4o-mini', 'gpt-6.1-terra', 'gpt-6-luna', 'gpt-4o-mini-tts-2025-12-15', 'gpt-audio', 'gpt-5.3-codex', 'gpt-5-search-api-2025-10-14', 'gpt-4o', 'gpt-image-2.6-flare', 'gpt-4o-mini-2024-07-18', 'gpt-4.1-2025-04-14', 'gpt-image-2.5-flare', 'gpt-4o-mini-tts-2025-03-20', 'gpt-5.2', 'gpt-5.4-nano-2026-03-17', 'gpt-6-astra', 'gpt-image-2.5-sunburst-2026-09-08', 'gpt-image-2.5-sunburst', 'gpt-image-2.5-flare-2026-09-08', 'gpt-image-2.6-sunburst', 'gpt-live-1']
Anthropic models: ['claude-haiku-5-5', 'claude-sonnet-5-5', 'claude-opus-5-5', 'claude-fable-5-1', 'claude-opus-5', 'claude-sonnet-5', 'claude-fable-5', 'claude-opus-4-8', 'claude-opus-4-7', 'claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929']
Perplexity models: []
Gemini models: []

=== Get OpenAI Models ===
OpenAI models: ['tts-1', 'gpt-5-mini', 'gpt-5-mini-2025-08-07', 'gpt-5.4-2026-03-05', 'gpt-5-nano', 'gpt-5.4-mini', 'omni-moderation-2024-09-26', 'gpt-live-transcribe', 'gpt-5.6-luna', 'gpt-4o-2024-11-20', 'gpt-5.1', 'whisper-1', 'gpt-5-pro', 'gpt-realtime-whisper', 'omni-moderation-latest', 'gpt-realtime-2', 'gpt-5-search-api', 'gpt-5.5-2026-04-23', 'gpt-5.5-pro', 'gpt-audio-2025-08-28', 'gpt-4o-mini-transcribe-2025-03-20', 'chat-latest', 'gpt-realtime-translate', 'gpt-5.4-pro-2026-03-05', 'gpt-4o-mini-transcribe-2025-12-15', 'gpt-5-pro-2025-10-06', 'gpt-5.6-sol', 'gpt-5.4-mini-2026-03-17', 'text-embedding-3-large', 'gpt-4o-2024-08-06', 'o3', 'gpt-5.4-pro', 'gpt-4o-mini-tts', 'gpt-realtime-mini-2025-12-15', 'gpt-realtime-2.1-mini', 'gpt-realtime-2025-08-28', 'tts-1-hd-1106', 'tts-1-hd', 'gpt-realtime-1.5', 'gpt-realtime', 'gpt-5.2-pro', 'gpt-4o-transcribe-diarize', 'gpt-5.1-2025-11-13', 'gpt-5.4', 'text-embedding-3-small', 'gpt-5-2025-08-07', 'o3-2025-04-16', 'gpt-5.2-2025-12-11', 'chatgpt-image-latest', 'tts-1-1106', 'gpt-5.5-pro-2026-04-23', 'gpt-5.2-pro-2025-12-11', 'gpt-5.4-nano', 'gpt-audio-mini-2025-12-15', 'gpt-audio-mini', 'gpt-4.1-mini-2025-04-14', 'gpt-5', 'gpt-4o-mini-transcribe', 'gpt-image-2-2026-04-21', 'gpt-transcribe', 'gpt-image-2', 'text-embedding-ada-002', 'gpt-realtime-2.1', 'gpt-4.1', 'gpt-5.5', 'gpt-audio-1.5', 'gpt-image-1.5', 'gpt-6.1-sol', 'gpt-6-sol', 'gpt-realtime-mini', 'gpt-image-1-mini', 'gpt-5.6-terra', 'gpt-5-nano-2025-08-07', 'gpt-4o-transcribe', 'gpt-4.1-mini', 'gpt-4o-mini', 'gpt-6.1-terra', 'gpt-6-luna', 'gpt-4o-mini-tts-2025-12-15', 'gpt-audio', 'gpt-5.3-codex', 'gpt-5-search-api-2025-10-14', 'gpt-4o', 'gpt-image-2.6-flare', 'gpt-4o-mini-2024-07-18', 'gpt-4.1-2025-04-14', 'gpt-image-2.5-flare', 'gpt-4o-mini-tts-2025-03-20', 'gpt-5.2', 'gpt-5.4-nano-2026-03-17', 'gpt-6-astra', 'gpt-image-2.5-sunburst-2026-09-08', 'gpt-image-2.5-sunburst', 'gpt-image-2.5-flare-2026-09-08', 'gpt-image-2.6-sunburst', 'gpt-live-1']

=== Get Anthropic Models ===
Anthropic models: ['claude-haiku-5-5', 'claude-sonnet-5-5', 'claude-opus-5-5', 'claude-fable-5-1', 'claude-opus-5', 'claude-sonnet-5', 'claude-fable-5', 'claude-opus-4-8', 'claude-opus-4-7', 'claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929']

✓ Chat Models API example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: XYLVXt4wvzFPOWkgsymxmrWYASThgu0hDFXVmAwgT-AUQJRJsHI-Fhdp_RSoY-Yepw4YMeRK4UAYCN_EUACwtg

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is **ekoDB**, a high-performance database product priced at **$99**.

✓ Message 2 sent
  Response: ekoDB costs **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: XGcalUqGkvWMGHrAq1pTP4NQNQZJbx0kjRgrfhMrKtXv_cLiaI847rJf4mliS5Df7y0DmygttogL_Tq53GoenQ
  Parent: XYLVXt4wvzFPOWkgsymxmrWYASThgu0hDFXVmAwgT-AUQJRJsHI-Fhdp_RSoY-Yepw4YMeRK4UAYCN_EUACwtg

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: XGcalUqGkvWMGHrAq1pTP4NQNQZJbx0kjRgrfhMrKtXv_cLiaI847rJf4mliS5Df7y0DmygttogL_Tq53GoenQ (Untitled)
  Session 2: XYLVXt4wvzFPOWkgsymxmrWYASThgu0hDFXVmAwgT-AUQJRJsHI-Fhdp_RSoY-Yepw4YMeRK4UAYCN_EUACwtg (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: XGcalUqGkvWMGHrAq1pTP4NQNQZJbx0kjRgrfhMrKtXv_cLiaI847rJf4mliS5Df7y0DmygttogL_Tq53GoenQ

=== Cleanup ===
✓ Deleted sessions and collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "3aEP3fEASx2H9RjqdOnRfcVSQU2iQmwKo1CzvxqWO3agUVVW6hM1wBevgpzi2o973R-yhXBisu0g9y6wZZ1t1w"

=== List Collections ===
Total collections: 13
Sample collections: ['chat_raw_completions__ek0_testing', 'chat_agent_configs__ek0_testing', 'schedules__ek0_testing', 'functions__ek0_testing', 'audit__ek0_testing']

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
✓ Created record with plain dict: {'id': 'VGmvuet4gkfu_YKwp9qsdwy1n2qI2LjQjQmxgjPQUJO5leppTG0hDP5r2zh2kYpkq6MQxz5FR7D6x1eiAkEOeQ'}

=== Upsert Operation ===
✓ Upsert (update existing record): VGmvuet4gkfu_YKwp9qsdwy1n2qI2LjQjQmxgjPQUJO5leppTG0hDP5r2zh2kYpkq6MQxz5FR7D6x1eiAkEOeQ
✓ Inserted second record: ZtZzvpRdfPK6c61p4GVo5EKjtwdhKwxHRFF1cqx6iaC7aVvDkBQjdb605w2UEIQchDwW6muMZGC9xfRXBqihyA
✓ Upsert (update second record): ZtZzvpRdfPK6c61p4GVo5EKjtwdhKwxHRFF1cqx6iaC7aVvDkBQjdb605w2UEIQchDwW6muMZGC9xfRXBqihyA

=== Find One Operation ===
✓ Found user by email: {'id': 'VGmvuet4gkfu_YKwp9qsdwy1n2qI2LjQjQmxgjPQUJO5leppTG0hDP5r2zh2kYpkq6MQxz5FR7D6x1eiAkEOeQ', 'active': {'type': 'Boolean', 'value': True}, 'age': {'value': 29, 'type': 'Integer'}, 'name': {'value': 'Alice Johnson', 'type': 'String'}, 'email': {'type': 'String', 'value': 'alice.j@newdomain.com'}}
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
✓ Inserted document: Edbzx8vQHtnWxXZlnoQwjxw6YEyS0-tlZvVTBFRPzUwQDhdE4s_8i5ysM1vwo8Rf1bVJkUIOoF4vTnmuFYUI9Q

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: myL3fI6ikcANwGWBo7BeA4lYy2hDwKnIP6lLp2wgspytty1ybS1CSMVXaKWmKY5whlmc6P-_AoHNc1ahDPdDKw

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
✓ Edge cache script created: w-OqoF_52h-kSLe8bBNZiOi8jtnScdnwnKw0L-RGL7c5Agl9Usk1yxrlcuXB97Rs_tXZR92jZ9e2kVNtnzp9fQ

Call 1: Cache miss (fetches from API)
Response time: 157ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "current": {
            "interval": 900,
            "temperature_2m": 14.1,
            "time": "2026-11-08T16:45"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "\u00b0C",
            "time": "iso8601"
          },
          "elevation": 32.0,
          "generationtime_ms": 0.02658367156982422,
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
Response time: 1ms (198.1x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "current": {
            "interval": 900,
            "temperature_2m": 14.1,
            "time": "2026-11-08T16:45"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "\u00b0C",
            "time": "iso8601"
          },
          "elevation": 32.0,
          "generationtime_ms": 0.02658367156982422,
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
   ⏱️  Duration: 68.6ms
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
   ⏱️  Duration: 0.7ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "type": "Object",
            "value": {
                  "address": {
                        "city": "Gwenborough",
                        "geo": {
                 ...
   🚀 Cache speedup: 95.2x faster!

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
   Vector databases store embeddings—numeric representations of text, images, audio, and other data—and make it efficient to find items that are *similar in meaning or content*, even when they don’t share exact keywords.

Key benefits include:

- **Semantic search:** Finds relevant results by meaning, not just keyword matches.
- **Fast similarity search at scale:** Uses specialized indexes to search large collections of vectors efficiently.
- **Better AI applications:** Helps retrieval-augmented generation (RAG) systems find relevant context for language models.
- **Support for multiple data types:** Can power search across text, images, audio, and other content represented as vectors.
- **Filtering and hybrid search:** Many systems combine vector similarity with metadata filters or traditional keyword search.

They’re usually best used alongside a traditional database: vectors help find similar content, while relational or document databases remain useful for structured records and transactions.
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

✅ Function saved: SwGXuG_FRhl99Y8i-kX1qRZvQZXgW_6wHD24okoWtU5t9R7sIV4POlyiAPL3V3h_UXPohvmayomPKOF12xvdJA
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

✅ Inserted order: qfWIEhqCaNmtqAYlgVzeXPUcQnP3UaeKzAt2Ow9GoEDs5Njv7Yw90x9Zg384NU8Qgig6ChmgJlNiIfWVOiR5hw
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

✅ Inserted order: oeQJMpXxsloaK5bewoa16WcRZXll1qVaNtIX6lFfVsRGG8w2_HYEMHbuzRJOw1zqmN9c4bHZ-XDbLAJIIc_B2g
✅ Cached order status
📊 Quick status lookup: {'value': '{"type":"Object","value":{"status":"processing","updated_at":"2026-11-08T16:52:08.584701+00:00"}}'}

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
   1. Introduction to Machine Learning (AI)
   2. Natural Language Processing (AI)
   3. Getting Started with ekoDB (Database)
   4. Database Design Principles (Database)
   5. Vector Databases Explained (Database)
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
🚀 ekoDB Functions Example (Python)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: rvX6NZLg5dKHqoDRuG3eburGh9YvMN5Cdw7fHJo3_CkZWgAViSwStb_kTorICnEaOvNIzpoKdyOwVfTEFk3E2w
📊 Found 5 active users

📝 Example 2: Parameterized Function

✅ Function saved: 9bdZUjxHsN9Rt4kNagzZnFsA_LhNLPcBy2QCVxthzNEQtmOPiIgJmJxjBTfGhZ9AYQ1ps2a5CZG-YDwu9GPsaA
📊 Found 3 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: JITdgoQIrsF30ONVESVcN0DNKbtBeckWSUbuz8GpQjOF6dYAH1H3tpKrhpv5EXHfEXKaMwrqClVaNKE4XbmgxA
📊 Statistics: 2 groups
   {'avg_score': {'type': 'Float', 'value': 60.0}, 'count': {'type': 'Integer', 'value': 5}, 'status': {'type': 'String', 'value': 'active'}}

   {'avg_score': {'type': 'Float', 'value': 50.0}, 'count': {'type': 'Integer', 'value': 5}, 'status': {'type': 'String', 'value': 'inactive'}}

📝 Example 4: Function Management

📋 Total functions: 3
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed!
=== ekoDB Goal Template CRUD Example (Python) ===

--- Creating goal template ---
Created template: Data Migration (id: tqyAr_mLTBT5tcVfY82p3_Ivl_uLJHkGerre7zhAIPeOxfH-X-IAYzai3lnn_dE4cduIme6yxd9gty_CeL6lZg)

--- Listing templates ---
Templates: {'count': 1, 'items': [{'description': {'type': 'String', 'value': 'Template for migrating data between schemas'}, 'id': 'tqyAr_mLTBT5tcVfY82p3_Ivl_uLJHkGerre7zhAIPeOxfH-X-IAYzai3lnn_dE4cduIme6yxd9gty_CeL6lZg', 'owner': {'type': 'Object', 'value': {'key_id': '3a0cdba0da7c060610042783cc758f1f', 'key_label': 'default_admin'}}, 'steps': {'type': 'Array', 'value': [{'description': 'Analyze source schema'}, {'description': 'Create target schema'}, {'description': 'Migrate records'}, {'description': 'Validate results'}]}, 'title': {'type': 'String', 'value': 'Data Migration'}}], 'total': 1}

--- Getting template ---
Fetched: {'type': 'String', 'value': 'Data Migration'}

--- Updating template ---
Updated description: {'type': 'String', 'value': 'Updated: comprehensive data migration workflow'}

--- Deleting template ---
Template deleted successfully

✓ Goal template CRUD example completed
=== ekoDB Goals / Tasks / Agents Integration Example (Python) ===

--- goal_create ---
Created goal: Deploy v3 (id: Oyj4iOD8ANUnHqd8Q7Opy0e06iplzJL9d8GAGm15KIgkToIBvpyyLeWWXn-JfbUzfJwOK-2cFslKaHHLsATLiw)

--- goal_list ---
Goals: {'count': 1, 'goals': [{'created_at': {'type': 'DateTime', 'value': '2026-11-08T16:52:09.201391+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production'}, 'id': 'Oyj4iOD8ANUnHqd8Q7Opy0e06iplzJL9d8GAGm15KIgkToIBvpyyLeWWXn-JfbUzfJwOK-2cFslKaHHLsATLiw', 'owner': {'type': 'Object', 'value': {'key_id': '3a0cdba0da7c060610042783cc758f1f', 'key_label': 'default_admin'}}, 'status': {'type': 'String', 'value': 'pending'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","status":"pending"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-11-08T16:52:09.201391+00:00'}}], 'total': 1}

--- goal_get ---
Fetched: {'type': 'String', 'value': 'Deploy v3'}

--- goal_update ---
Updated description: {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}

--- goal_search ---
Search results: {'count': 1, 'items': [{'_score': 13.2, 'created_at': {'type': 'DateTime', 'value': '2026-11-08T16:52:09.201391+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'Oyj4iOD8ANUnHqd8Q7Opy0e06iplzJL9d8GAGm15KIgkToIBvpyyLeWWXn-JfbUzfJwOK-2cFslKaHHLsATLiw', 'owner': {'type': 'Object', 'value': {'key_id': '3a0cdba0da7c060610042783cc758f1f', 'key_label': 'default_admin'}}, 'status': {'type': 'String', 'value': 'pending'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","status":"pending"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-11-08T16:52:09.205389+00:00'}}]}

--- goal_step_start (step 0) ---
Step 0 started: {'created_at': {'type': 'DateTime', 'value': '2026-11-08T16:52:09.201391+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'Oyj4iOD8ANUnHqd8Q7Opy0e06iplzJL9d8GAGm15KIgkToIBvpyyLeWWXn-JfbUzfJwOK-2cFslKaHHLsATLiw', 'owner': {'type': 'Object', 'value': {'key_id': '3a0cdba0da7c060610042783cc758f1f', 'key_label': 'default_admin'}}, 'status': {'type': 'String', 'value': 'in_progress'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","status":"InProgress"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-11-08T16:52:09.208900+00:00'}}

--- goal_step_complete (step 0) ---
Step 0 completed: {'created_at': {'type': 'DateTime', 'value': '2026-11-08T16:52:09.201391+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'Oyj4iOD8ANUnHqd8Q7Opy0e06iplzJL9d8GAGm15KIgkToIBvpyyLeWWXn-JfbUzfJwOK-2cFslKaHHLsATLiw', 'owner': {'type': 'Object', 'value': {'key_id': '3a0cdba0da7c060610042783cc758f1f', 'key_label': 'default_admin'}}, 'status': {'type': 'String', 'value': 'in_progress'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","result":"Docker image built: sha256:abc123","status":"Completed"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-11-08T16:52:09.213226+00:00'}}

--- goal_step_fail (step 1) ---
Step 1 failed: {'created_at': {'type': 'DateTime', 'value': '2026-11-08T16:52:09.201391+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'Oyj4iOD8ANUnHqd8Q7Opy0e06iplzJL9d8GAGm15KIgkToIBvpyyLeWWXn-JfbUzfJwOK-2cFslKaHHLsATLiw', 'owner': {'type': 'Object', 'value': {'key_id': '3a0cdba0da7c060610042783cc758f1f', 'key_label': 'default_admin'}}, 'status': {'type': 'String', 'value': 'in_progress'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","result":"Docker image built: sha256:abc123","status":"Completed"},{"description":"Run migrations","error":"Migration failed: column already exists","status":"Failed"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-11-08T16:52:09.217408+00:00'}}

--- goal_complete ---
Goal completed: {'type': 'String', 'value': 'pending_review'}

--- goal_approve ---
Goal approved: {'type': 'String', 'value': 'in_progress'}

--- goal_reject ---
Goal rejected: {'type': 'String', 'value': 'failed'}

--- task_create ---
Created task: Nightly Backup (id: CA8F6MpxlvgIm8m1LtaMyVM9lqmWTXlzIAAfh7fZtXS_lLJQHR8K3bNdndxBYfNuprumEpNvy2su1QKkt18ObQ)

--- task_list ---
Tasks: {'count': 1, 'items': [{'cron': {'type': 'String', 'value': '0 2 * * *'}, 'description': {'type': 'String', 'value': 'Backup all databases nightly'}, 'id': 'CA8F6MpxlvgIm8m1LtaMyVM9lqmWTXlzIAAfh7fZtXS_lLJQHR8K3bNdndxBYfNuprumEpNvy2su1QKkt18ObQ', 'max_consecutive_failures': {'type': 'Integer', 'value': 3}, 'name': {'type': 'String', 'value': 'Nightly Backup'}, 'owner': {'type': 'Object', 'value': {'key_id': '3a0cdba0da7c060610042783cc758f1f', 'key_label': 'default_admin'}}}], 'total': 1}

--- task_get ---
Fetched: {'type': 'String', 'value': 'Nightly Backup'}

--- task_start ---
Task started: {'type': 'String', 'value': 'running'}

--- task_succeed ---
Task succeeded: {'consecutive_failures': {'type': 'Integer', 'value': 0}, 'cron': {'type': 'String', 'value': '0 2 * * *'}, 'description': {'type': 'String', 'value': 'Backup all databases nightly'}, 'id': 'CA8F6MpxlvgIm8m1LtaMyVM9lqmWTXlzIAAfh7fZtXS_lLJQHR8K3bNdndxBYfNuprumEpNvy2su1QKkt18ObQ', 'last_error': {'type': 'Null', 'value': None}, 'last_run': {'type': 'DateTime', 'value': '2026-11-08T16:52:09.260646+00:00'}, 'max_consecutive_failures': {'type': 'Integer', 'value': 3}, 'name': {'type': 'String', 'value': 'Nightly Backup'}, 'owner': {'type': 'Object', 'value': {'key_id': '3a0cdba0da7c060610042783cc758f1f', 'key_label': 'default_admin'}}, 'run_count': {'type': 'Integer', 'value': 1}, 'status': {'type': 'String', 'value': 'active'}, 'updated_at': {'type': 'DateTime', 'value': '2026-11-08T16:52:09.260646+00:00'}}

--- task_pause ---
Task paused: {'type': 'String', 'value': 'paused'}

--- task_resume ---
Task resumed: {'type': 'String', 'value': 'active'}

--- task_fail ---
Task failed: {'consecutive_failures': {'type': 'Integer', 'value': 1}, 'cron': {'type': 'String', 'value': '0 2 * * *'}, 'description': {'type': 'String', 'value': 'Backup all databases nightly'}, 'id': 'CA8F6MpxlvgIm8m1LtaMyVM9lqmWTXlzIAAfh7fZtXS_lLJQHR8K3bNdndxBYfNuprumEpNvy2su1QKkt18ObQ', 'last_error': {'type': 'String', 'value': 'Disk full'}, 'last_run': {'type': 'DateTime', 'value': '2026-11-08T16:52:09.282687+00:00'}, 'max_consecutive_failures': {'type': 'Integer', 'value': 3}, 'name': {'type': 'String', 'value': 'Nightly Backup'}, 'next_run': {'type': 'DateTime', 'value': '2026-03-22T02:00:00+00:00'}, 'owner': {'type': 'Object', 'value': {'key_id': '3a0cdba0da7c060610042783cc758f1f', 'key_label': 'default_admin'}}, 'run_count': {'type': 'Integer', 'value': 1}, 'status': {'type': 'String', 'value': 'active'}, 'updated_at': {'type': 'DateTime', 'value': '2026-11-08T16:52:09.282687+00:00'}}

--- task_due ---
Due tasks: {'count': 0, 'items': []}

--- task_delete ---
Task deleted

--- agent_create ---
Created agent: CodeReviewer (id: QDMvjCRQU2qqfdW5SXu1hg_9HgcBEiNXmzZ7ChwQJUm8nQx3VESW6xruka____6XqsxcbRYvlV1K4izA2n2oWw)

--- agent_list ---
Agents: {'count': 1, 'items': [{'deployment_id': {'type': 'String', 'value': 'deploy_test'}, 'id': 'QDMvjCRQU2qqfdW5SXu1hg_9HgcBEiNXmzZ7ChwQJUm8nQx3VESW6xruka____6XqsxcbRYvlV1K4izA2n2oWw', 'llm_model': {'type': 'String', 'value': 'gpt-6-luna'}, 'name': {'type': 'String', 'value': 'CodeReviewer'}, 'owner': {'type': 'Object', 'value': {'key_id': '3a0cdba0da7c060610042783cc758f1f', 'key_label': 'default_admin'}}, 'run_policy': {'type': 'String', 'value': '{"trigger":{"allow_write_operations":false,"write_tools":[]},"eval":{"allow_write_operations":false,"write_tools":[]},"customer":{"allow_write_operations":false,"write_tools":[]}}'}, 'system_prompt': {'type': 'String', 'value': 'You review code for correctness and style.'}, 'tools': {'type': 'Array', 'value': ['web_search', 'file_read']}}], 'total': 1}

--- agent_get ---
Fetched: {'type': 'String', 'value': 'CodeReviewer'}

--- agent_get_by_name ---
By name: {'type': 'String', 'value': 'CodeReviewer'}

--- agent_update ---
Updated agent prompt: {'type': 'String', 'value': 'You review code. Be concise.'}

--- agents_by_deployment ---
Agents in deploy_test: {'count': 0, 'items': []}
WARNING: agents_by_deployment omitted created agent QDMvjCRQU2qqfdW5SXu1hg_9HgcBEiNXmzZ7ChwQJUm8nQx3VESW6xruka____6XqsxcbRYvlV1K4izA2n2oWw; TODO: check/fix the server-side deployment lookup

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
Inserted document: VWVSn-K7zvEbKwf-xmlRWCUotzD1Be2BTqZ1c0iXCqY_M5ErFDPktUnEB5FifiibIDiQt1wzXD9tZpS8MyZZZA

--- kv_link ---
Linked: None

--- kv_get_links ---
Links for app:config: [{'collection': 'kv_links_example_py', 'created_at': '2026-11-08T16:52:09.871995Z', 'document_id': 'VWVSn-K7zvEbKwf-xmlRWCUotzD1Be2BTqZ1c0iXCqY_M5ErFDPktUnEB5FifiibIDiQt1wzXD9tZpS8MyZZZA', 'field_path': None, 'last_accessed': '2026-11-08T16:52:09.872348Z', 'metadata': {}}]

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
Response: The traditional primary colors are red, yellow, and blue. For light and screens, they’re red, green, and blue (RGB).

--- Structured Output via SSE ---
JSON response: [
  {"name": "Earth", "diameter_km": 12742},
  {"name": "Mars", "diameter_km": 6779},
  {"name": "Jupiter", "diameter_km": 139820}
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello!

=== Done ===
=== ekoDB Schedule Management Example (Python) ===

--- create_schedule ---
Created schedule: Nightly Report (id: 4677e629-13e4-4134-82b4-a0cedec81bf6)

--- list_schedules ---
Schedules: {'count': 1, 'schedules': [{'created_at': '2026-11-08T16:52:14.514613Z', 'created_by': {'key_id': '3a0cdba0da7c060610042783cc758f1f', 'key_label': 'default_admin'}, 'cron_expression': '0 0 0 * * *', 'description': 'Generate and email nightly analytics report', 'enabled': True, 'function_label': 'schedule_noop_python_70851', 'id': '4677e629-13e4-4134-82b4-a0cedec81bf6', 'last_execution': None, 'name': 'Nightly Report', 'next_execution': '2026-11-09T00:00:00Z', 'parameters': {}, 'stats': {'avg_execution_time_ms': 0.0, 'failed_executions': 0, 'last_error': None, 'successful_executions': 0, 'total_executions': 0}, 'timezone': 'UTC', 'updated_at': '2026-11-08T16:52:14.514613Z'}]}

--- get_schedule ---
Fetched: Nightly Report cron=0 0 0 * * *

--- update_schedule ---
Updated cron: 0 30 1 * * *

--- trigger_schedule ---
Triggered: {'schedule_id': '4677e629-13e4-4134-82b4-a0cedec81bf6', 'status': 'triggered'}

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
  1. Score: 13.200, Matched: title, bio
  2. Score: 13.200, Matched: title, bio
  3. Score: 13.200, Matched: bio, title
  4. Score: 13.200, Matched: bio, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400, Matched: bio, title
  2. Score: 26.400, Matched: title, bio
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
  1. Score: 0.750
  2. Score: 0.747
  3. Score: 0.726

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.375, Matched: content, title
  2. Score: 0.863, Matched: content, title
  3. Score: 0.373, Matched:

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
Inserted: {'id': '71D2VJXPu4o9zPW9E5fsuJnzdC73ENiC0HhgqgmnG8fxffnu4rMLvSv56HExStAy25rSS133_mDriwwcD8iz5g'}

=== Find by ID ===
Found: {'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}, 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}, 'name': {'value': 'Test Record', 'type': 'String'}, 'value': {'type': 'Integer', 'value': 42}, 'metadata': {'type': 'Object', 'value': {'key': 'value', 'nested': {'deep': True}}}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'created_at': {'type': 'String', 'value': '2026-11-08T11:52:15.057201'}, 'price': {'value': 99.99, 'type': 'Float'}, 'active': {'type': 'Boolean', 'value': True}, 'id': '71D2VJXPu4o9zPW9E5fsuJnzdC73ENiC0HhgqgmnG8fxffnu4rMLvSv56HExStAy25rSS133_mDriwwcD8iz5g', 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-11-08 11:52:15.057201
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'key': 'value', 'nested': {'deep': True}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'data': 'aGVsbG8gd29ybGQ=', 'tags': ['tag1', 'tag2', 'tag3'], 'name': 'Test Record', 'value': 42, 'metadata': {'key': 'value', 'nested': {'deep': True}}, 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'created_at': '2026-11-08T11:52:15.057201', 'price': 99.99, 'active': True, 'id': '71D2VJXPu4o9zPW9E5fsuJnzdC73ENiC0HhgqgmnG8fxffnu4rMLvSv56HExStAy25rSS133_mDriwwcD8iz5g', 'categories': ['electronics', 'computers'], 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5]}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'price': {'type': 'Float', 'value': 99.99}, 'active': {'value': True, 'type': 'Boolean'}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}, 'metadata': {'type': 'Object', 'value': {'key': 'value', 'nested': {'deep': True}}}, 'id': '71D2VJXPu4o9zPW9E5fsuJnzdC73ENiC0HhgqgmnG8fxffnu4rMLvSv56HExStAy25rSS133_mDriwwcD8iz5g', 'created_at': {'type': 'String', 'value': '2026-11-08T11:52:15.057201'}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'name': {'value': 'Updated Record', 'type': 'String'}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'value': {'type': 'Integer', 'value': 100}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: neQyneCRJYDN2VjWFQmZ7nZt1ABlPNVTiLW6M_OdYoLh2yQmjSDfLktsiYDdjsFzuUJvvPiOLwcGPwcGmf3alA

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
✓ Created native SWR script: github_user_native_py (i_sfk6wLDtBRPJV4qkhYaWsyz2UK3-H7R-Q_dw0BkZnrwbezOer1c-SO61S_eCmBDVW_5heXgVL6VFkbs2lN_A)

First call (cache miss - will fetch from GitHub API):
  Response time: 125ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 1ms
  Speedup: 130.2x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_py (VnoMZ088Ea0IZqYZV67io7gkceSgyz5H_JVwFsPR-iotxq0O9bMX_-QrgNPqD4ueDd7tva6_ioB9Bwu5ldy4rQ)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_py' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_py (NGgSmZmk5ofi6aBlsTQWL9YjVagr0FZbU5NJhuh4eyw_gdGYARktTJwxk2_XkL2yIbj1-r7GjGB-sM1vdxZlGQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_py' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_py (L0TS23vHPJCHiIGxpQ7oJqYK2d1j2yiAn9mf_ytS4JhXrbVTBV6jJgPqr31WTVp0UNAlqV5Xjhg4nu9vfCt-Lg)
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
✓ Created SWR script: fetch_api_user_py (rV2yQQObZrmF-w6aJu3m4DJl2epXb2eUZXOUwwNySiSX1ca9MNLStZxqFy7PeyoaswxMrB70sOXPslD1ZNWiUQ)

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
Response time: 1ms (served from cache)
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
Created Alice: $1000 - ID: ihNgMMgrXBz1t5_4p49L-vu0pkdPYFO-TKKWUFVRvTdmnOqkw2WzewqAkk22fIa_yKS2nzOIAUzIUBvhVxm-TQ
Created Bob: $500 - ID: 8HtFCRVhup_ObZ921dAVHRRUNh49R2XHm2ai6KRKPx5o2alHJWv01gi4ip0VDamXNpST5ndsXdSNU2BYgdVEvw

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 418bc902-a563-4931-aec7-cf5e78a8571c

=== Example 2: Operations within Transaction ===
Alice in transaction: {'id': 'ihNgMMgrXBz1t5_4p49L-vu0pkdPYFO-TKKWUFVRvTdmnOqkw2WzewqAkk22fIa_yKS2nzOIAUzIUBvhVxm-TQ', 'balance': {'value': 800, 'type': 'Integer'}, 'name': {'type': 'String', 'value': 'Alice'}, 'account_id': {'value': 'ACC001', 'type': 'String'}}
Bob in transaction: {'id': '8HtFCRVhup_ObZ921dAVHRRUNh49R2XHm2ai6KRKPx5o2alHJWv01gi4ip0VDamXNpST5ndsXdSNU2BYgdVEvw', 'balance': {'type': 'Integer', 'value': 700}, 'account_id': {'value': 'ACC002', 'type': 'String'}, 'name': {'type': 'String', 'value': 'Bob'}}
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

✓ Verified committed balances: Alice=$800, Bob=$700

=== Example 5: Rollback Demo ===
New transaction: 3269fccd-5b21-4a43-9c4c-cfc739023f08
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Create User Function ===
Created user function with ID: xKd8whtwSTHLr7qLUYqAoQ9kzUbBk3FzrcSe_azwwQYzylt2dA_J0fFCQfd5h0BlNLZhDc4RJSrlBEDvR9om2Q

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

Created chat session: 8cAiwHDvss_xnMjUTdzMihaLFl5gKjU72wDW5AiEUVON2VYYI_JTTF_rikSb08AnkgprBzQtAsPJ08h6lpzHEQ

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"gpt-5.6-luna","provider":"openai"}The capital of France is **Paris**.

--- Stream ended ---
Message ID: G3ZcST0mNCuEANG94p-RIAVcJpbyMrYdMhkncraENxObRFmNmbWC38RR-Oghifpt5mEPZHuquIbdy46dNuYTSQ
Execution time: 678ms
Token usage: {'cached_input_tokens': 0, 'completion_tokens': 13, 'prompt_tokens': 23, 'thinking_tokens': 0, 'total_tokens': 36}

Full response: {"__progress":"received"}{"__progress":"generating","model":"gpt-5.6-luna","provider":"openai"}The capital of France is **Paris**....
=== WebSocket Subscription Example (Python) ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_py' ===
✓ Subscribed (subscription_id: sub_7a4d9ce268c64cd5b9659d7bb9c7d275)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: 3Jt6E-wg7mJa_2LIr_-MpWaevdHXnK3gps8nFkPJSyXXn3ZMfdD1huLNK8vOGF7PxKO3HkKHW5lkNswOSNFCMQ

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_py
     Record IDs: 3Jt6E-wg7mJa_2LIr_-MpWaevdHXnK3gps8nFkPJSyXXn3ZMfdD1huLNK8vOGF7PxKO3HkKHW5lkNswOSNFCMQ
     Timestamp:  2026-11-08T16:52:17.631117+00:00

Inserting record 2...
✓ Inserted: CdoH6jw5WIHeRooSEladDEG3Rr-CX0J84bVy1SwVF-Q7qQ1SMO_Cn7ClzP0_JdGYJQYUcBFERCBNwgpmZjvukg

  📡 Notification received:
     Event:      insert
     Record IDs: CdoH6jw5WIHeRooSEladDEG3Rr-CX0J84bVy1SwVF-Q7qQ1SMO_Cn7ClzP0_JdGYJQYUcBFERCBNwgpmZjvukg

=== Unsubscribing ===
✓ Unsubscribed: {'collection': 'ws_subscribe_example_py', 'found': True, 'unsubscribed': True}

=== Cleanup ===
✓ Deleted collection 'ws_subscribe_example_py'

✓ WebSocket subscription example completed successfully
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: nSJ8IIUWOPA9ntogFHN613BLnwgpeX_wPDoz9yXy9Ph2DA1i6ErVdatjs5WuquUJIpQtdfkSC5m9KV1MnDMvYQ

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
   Inserted with ripple: {'id': 'IgvBAtAUSt7cM9KAuXrvjoWCoI6iO1j3tT_vsmGQCQ3QNMDrIjrfyMXQBDwDLxbgx0FppwuPRaO6R6YHekPB0g'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'Gqd3ROH6Zp__RJYnP_xIiGwGOQBMeQw2UM9WSjVnAU-jD8lrq-MsDuFPtJ5V2LQ86J7PyLWy8fkbUPzKj71eyw'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'price': {'value': 150, 'type': 'Integer'}, 'id': 'IgvBAtAUSt7cM9KAuXrvjoWCoI6iO1j3tT_vsmGQCQ3QNMDrIjrfyMXQBDwDLxbgx0FppwuPRaO6R6YHekPB0g', 'name': {'type': 'String', 'value': 'Product 1'}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'name': {'value': 'Upsert Product', 'type': 'String'}, 'price': {'value': 500, 'type': 'Integer'}, 'id': 'IgvBAtAUSt7cM9KAuXrvjoWCoI6iO1j3tT_vsmGQCQ3QNMDrIjrfyMXQBDwDLxbgx0FppwuPRaO6R6YHekPB0g'}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ['email', 'id', 'name']
  First user: {'type': 'String', 'value': 'Alice Johnson'} <{'type': 'String', 'value': 'alice@example.com'}>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ['user_role', 'created_at', 'bio', 'id', 'avatar_url', 'age', 'name', 'status', 'email']

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - {'type': 'String', 'value': 'Dave Brown'} (age {'type': 'Integer', 'value': 45})
    - {'type': 'String', 'value': 'Alice Johnson'} (age {'type': 'Integer', 'value': 30})
    - {'type': 'String', 'value': 'Bob Smith'} (age {'value': 25, 'type': 'Integer'})

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - {'value': 'Carol White', 'type': 'String'}: {'type': 'String', 'value': 'Manager'}

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ['api_key', 'created_at', 'password', 'email', 'bio', 'user_role', 'id', 'name', 'age', 'avatar_url', 'status', 'secret_token']
  Projected query:
    - 3 fields per record
    - Fields: ['email', 'name', 'id']
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
Inserted: map[id:vNR5hJm30oaRmrqltKwQXrjkjavkO72W-WWE32Tk-iz2Nt9QYKTCdkrR7-ErILMB63eVEXvAHGTYz2kKCo69MQ]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:vNR5hJm30oaRmrqltKwQXrjkjavkO72W-WWE32Tk-iz2Nt9QYKTCdkrR7-ErILMB63eVEXvAHGTYz2kKCo69MQ name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found documents: [map[active:map[type:Boolean value:true] id:vNR5hJm30oaRmrqltKwQXrjkjavkO72W-WWE32Tk-iz2Nt9QYKTCdkrR7-ErILMB63eVEXvAHGTYz2kKCo69MQ name:map[type:String value:Test Record] value:map[type:Integer value:42]]]

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:vNR5hJm30oaRmrqltKwQXrjkjavkO72W-WWE32Tk-iz2Nt9QYKTCdkrR7-ErILMB63eVEXvAHGTYz2kKCo69MQ name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ simple_crud.go completed successfully

=== Running simple_websocket.go ===
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: B6-XLiG64BG-oOIQwNQmVY9D8cSD_kpCK1BilIqbBVPKxQODE3Cl1gAfIlKLFZkMPUVjDw1FzcJApIz6Rpcg_g

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1794156742001645000",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "B6-XLiG64BG-oOIQwNQmVY9D8cSD_kpCK1BilIqbBVPKxQODE3Cl1gAfIlKLFZkMPUVjDw1FzcJApIz6Rpcg_g",
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

✅ Function saved: WV_0iKJQBinxxYm3-h4GCjMOkuGs93qcigqnE3EYh63po3caZFCpesR0fwD3_YhCcre0wn88icNXZ1ixpyad5w
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: c3TGjbnzZU8tTcai_rSslrbIfwlRhWGYn1gXX4tySNaNSlnJViR3wFmyRT011qy3aN7GuRfGnbo9wI4rbK6u6g
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: K04YzVXovJl7eEiMCy7XNYbHJosJUuOJWJKJCB_j9G5jT9FVxS6jS-AwlKNAmX_sdOa0LHxB01jGcBXSFbd9Jw
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
Collection created with first record: O3o1Jbq2JhH7mqD-AXPZP4gox2LneFZfXVZIixWA2nJvJ-KPjPXRJqYIZ0-9JweiWhdsc6oErgNFiSRHsjHVnw

=== List Collections ===
Total collections: 13
Sample collections: [chat_raw_completions__ek0_testing chat_agent_configs__ek0_testing schedules__ek0_testing functions__ek0_testing collection_management_go]

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ collection_management.go completed successfully

=== Running transactions.go ===
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 94v9CTAxyPln576Z_YLjnAtnkc4aZ_-ZCV40T8lGyhap28RCcAZ9SOH7RFPqCZt-wXtEtJSqqNqTt7mNeqbozA
Created Bob: $500 - ID: t-N1s-_m7E3FXMDWm0ZjIx4jFsgqwg8uzFQjcTKAKqenKG0IedV8CYxx3R8zGWroBnNFrKf0ia2x1AOgXGks7g

=== Example 1: Begin Transaction ===
Transaction ID: ec67c54e-78d3-4779-8071-b51f5d8a0290

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
New transaction: 0f6f1e31-967a-4463-948a-513089409d14
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
   ✅ Function saved: oY6RGEqaqzdZ2Gr0A3ree-hjl1JLci24j3_CY9zguK3s6NfhGEggC8M6Cu98TJRRNb6SndsNO-xmy_lXIDPX2Q

2️⃣ Calling function (Insert + Verify)...
   ✅ function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: uyQwbxpmuUug1wVHhXla2AGbmfguoLbVfw4iR5Io4WbbkdfFf47_A8jOiGNCfoQqBROD8CMJf7OEy-6MmvICoQ
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: _bkEL0sAhf9Ag2CU0CFu3pZOP101BoPkD-5_EBIUNa0m7AXxQFFb6pm5q8k1K2y6gkuZa-jL3m3Sd_ui26j9Hw

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
   ✅ Function saved: ab_oyo85ljmwUE65P6rLDDQRWzxcdNuAdhuCaumPBjyjiRmhOJ_FKa-CEzA4uDopg6RZ7x4FID1lhtj_8eCM8w

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
   ✅ Function saved: 5G5OnwmAwyTJdljv-oL42PXVLeuEp0PNJEpJyqLDwbF7b5VQusqCiPDCbj9Kt_PSNZSYayRBi5ZA4EaDpPp5Tw

2️⃣ Calling function (Query + Delete + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: oY6RGEqaqzdZ2Gr0A3re...
   ✅ Deleted script: _bkEL0sAhf9Ag2CU0CFu...
   ✅ Deleted script: ab_oyo85ljmwUE65P6rL...
   ✅ Deleted script: 5G5OnwmAwyTJdljv-oL4...
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
  Output: Document ID = a8wnRPrKYJLmiB5ZogjoQclARPah2oPwarwv2-oy8RTI_tzztkLFBilKNNP_zrtZgfZFzqAt6OfMzkhF-Sz7Dw
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: FindByID(a8wnRPrKYJLmiB5ZogjoQclARPah2oPwarwv2-oy8RTI_tzztkLFBilKNNP_zrtZgfZFzqAt6OfMzkhF-Sz7Dw)
  Output: Found document with name = map[type:String value:TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: FindByID(a8wnRPrKYJLmiB5ZogjoQclARPah2oPwarwv2-oy8RTI_tzztkLFBilKNNP_zrtZgfZFzqAt6OfMzkhF-Sz7Dw)
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
  Output: Document ID = CuO6xvsSTr5I9gxkxZfsP1M-C8fLlqlYPUVAZp4iEbhyTq5-JT0JRiH-1K8DJGb5cxxpe7ByefTUYLfY-7f9EQ
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: FindByID(CuO6xvsSTr5I9gxkxZfsP1M-C8fLlqlYPUVAZp4iEbhyTq5-JT0JRiH-1K8DJGb5cxxpe7ByefTUYLfY-7f9EQ)
  Output: Found document with name = map[type:String value:WS TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: FindByID(CuO6xvsSTr5I9gxkxZfsP1M-C8fLlqlYPUVAZp4iEbhyTq5-JT0JRiH-1K8DJGb5cxxpe7ByefTUYLfY-7f9EQ)
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
Inserted record: kIUOYvJJPVFlCbds2hAX1zFrYR3zbfmkgB_QMzCSDQuQlFBCGPqs4cC3JRpGex718lUwM2OhcSpKcVG1MNuIQw

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
✓ Created session: vri3UvzseN42IYNTkB-k49uE39LlX2bQeXiAK64626u25vwaWWLJ0EPjMTsTun-xeF0TS6rdlvQkjiMjLf-VAQ

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is **ekoDB** — a high-performance database product, priced at **$99**.

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
✓ Created second session: lhg1LQF7wq_dt5vVyveekmr3xP2FU_ymR3nrH2EzK-VF2uq5mif2w-uqdUNXIsf_qg1F1l7yLHlN9gzsqp1wMA
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: lhg1LQF7wq_dt5vVyveekmr3xP2FU_ymR3nrH2EzK-VF2uq5mif2w-uqdUNXIsf_qg1F1l7yLHlN9gzsqp1wMA
✓ Deleted chat session: vri3UvzseN42IYNTkB-k49uE39LlX2bQeXiAK64626u25vwaWWLJ0EPjMTsTun-xeF0TS6rdlvQkjiMjLf-VAQ
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: otoNgYUGhdpGPYxWtODeNY9pS9T-as_2C9nTVPIHtMDJJVQhhFNnB6UNARzy6nysfkseLZczE_4dyyxOs2E5bQ

=== Sending Chat Message ===
Message ID: SQdqrVE8oAZknNUEnhzRFz4bMu9jfYxkroPBUQ2HG6lHCjj0yQnJpUVgibQLolQpwtvzKpY9PJBDrymxSI-nqw

=== AI Response ===
Available products and prices:

- **ekoDB** — 99
- **ekoDB Pro** — 299
- **ekoDB Cloud** — 499

The currency isn’t specified.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:6GiDG5Iq047xwAazqU327YSLWIMyfx1aySljRImU4BBrno-1t5XjOZd7vlP6v4LFJI0AC9zjNa7jW4cawxMIfQ name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:D2lyjs7MEvh7_uVAA-REN3yngo6NLs0S1eRk2wH10MK1dhM6cxOHN8QvfDiwep9Fz-7eiO5KTH_82K7NcsiPhw name:ekoDB price:99] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:kyUiMcAYBJpX5iK42sbwMiVoaQ-YdCxfX2dufujGbLbROGU14VyFnxRe8In-YAtLk5Wf_syfcjps0ZOGExofOA name:ekoDB Cloud price:499] score:0.1111111111111111]

Execution Time: 5613ms

=== Token Usage ===
Prompt tokens: 41698
Completion tokens: 282
Total tokens: 41980

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Message Stream (SSE) Example (Go) ===

Created session: fJ6wryuhKUZ6jbOnmE5UbXEg3I3YkgCPFYhiii5TYGubnI-I31p9j0nRZI9bEsFzhhAvQ8H_icOoxKogIWwUVw

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"gpt-6-luna","provider":"openai"}**EkoDB** is the name of a database project, but it can refer to different products or libraries. If you share where you saw it—such as a website, package, or article—I can identify the specific one and explain what it does.

--- Stream complete ---
Message ID: XaiZ6OyqfQDg-_NiKj8qbINJyfX3PYDaMRLqkQOggG5BeZpPo-ObtE4FkBPoCNXSixn6dERAmteJYG2ZLBtI3Q
Execution time: 6872ms
Context window: 128000 tokens

✓ Chat message stream example completed
✓ Client created

=== Get All Chat Models ===
OpenAI models: [tts-1 gpt-5-mini gpt-5-mini-2025-08-07 gpt-5.4-2026-03-05 gpt-5-nano gpt-5.4-mini omni-moderation-2024-09-26 gpt-live-transcribe gpt-5.6-luna gpt-4o-2024-11-20 gpt-5.1 whisper-1 gpt-5-pro gpt-realtime-whisper omni-moderation-latest gpt-realtime-2 gpt-5-search-api gpt-5.5-2026-04-23 gpt-5.5-pro gpt-audio-2025-08-28 gpt-4o-mini-transcribe-2025-03-20 chat-latest gpt-realtime-translate gpt-5.4-pro-2026-03-05 gpt-4o-mini-transcribe-2025-12-15 gpt-5-pro-2025-10-06 gpt-5.6-sol gpt-5.4-mini-2026-03-17 text-embedding-3-large gpt-4o-2024-08-06 o3 gpt-5.4-pro gpt-4o-mini-tts gpt-realtime-mini-2025-12-15 gpt-realtime-2.1-mini gpt-realtime-2025-08-28 tts-1-hd-1106 tts-1-hd gpt-realtime-1.5 gpt-realtime gpt-5.2-pro gpt-4o-transcribe-diarize gpt-5.1-2025-11-13 gpt-5.4 text-embedding-3-small gpt-5-2025-08-07 o3-2025-04-16 gpt-5.2-2025-12-11 chatgpt-image-latest tts-1-1106 gpt-5.5-pro-2026-04-23 gpt-5.2-pro-2025-12-11 gpt-5.4-nano gpt-audio-mini-2025-12-15 gpt-audio-mini gpt-4.1-mini-2025-04-14 gpt-5 gpt-4o-mini-transcribe gpt-image-2-2026-04-21 gpt-transcribe gpt-image-2 text-embedding-ada-002 gpt-realtime-2.1 gpt-4.1 gpt-5.5 gpt-audio-1.5 gpt-image-1.5 gpt-6.1-sol gpt-6-sol gpt-realtime-mini gpt-image-1-mini gpt-5.6-terra gpt-5-nano-2025-08-07 gpt-4o-transcribe gpt-4.1-mini gpt-4o-mini gpt-6.1-terra gpt-6-luna gpt-4o-mini-tts-2025-12-15 gpt-audio gpt-5.3-codex gpt-5-search-api-2025-10-14 gpt-4o gpt-image-2.6-flare gpt-4o-mini-2024-07-18 gpt-4.1-2025-04-14 gpt-image-2.5-flare gpt-4o-mini-tts-2025-03-20 gpt-5.2 gpt-5.4-nano-2026-03-17 gpt-6-astra gpt-image-2.5-sunburst-2026-09-08 gpt-image-2.5-sunburst gpt-image-2.5-flare-2026-09-08 gpt-image-2.6-sunburst gpt-live-1]
Anthropic models: [claude-haiku-5-5 claude-sonnet-5-5 claude-opus-5-5 claude-fable-5-1 claude-opus-5 claude-sonnet-5 claude-fable-5 claude-opus-4-8 claude-opus-4-7 claude-sonnet-4-6 claude-opus-4-6 claude-opus-4-5-20251101 claude-haiku-4-5-20251001 claude-sonnet-4-5-20250929]
Perplexity models: []
Gemini models: [gemini-2.5-flash gemini-2.5-pro gemini-2.5-flash-preview-tts gemini-2.5-pro-preview-tts gemma-4-26b-a4b-it gemma-4-31b-it gemini-flash-latest gemini-flash-lite-latest gemini-pro-latest gemini-2.5-flash-lite gemini-2.5-flash-image gemini-3-flash-preview gemini-3.1-pro-preview gemini-3.1-pro-preview-customtools gemini-3.1-flash-lite-preview gemini-3.1-flash-lite gemini-3-pro-image-preview gemini-3-pro-image nano-banana-pro-preview gemini-3.1-flash-image-preview gemini-3.1-flash-image gemini-3.1-flash-lite-image gemini-nano-banana-2.1 gemini-3.5-flash gemini-3.5-flash-lite gemini-omni-flash-preview gemini-omni-1.1-flash gemini-omni-1.2-flash gemini-3.5-transcribe gemini-3.6-flash gemini-3.7-flash gemini-3.8-flash gemini-4-argon lyria-3-clip-preview lyria-3-pro-preview lyria-3.5 gemini-3.1-flash-tts-preview gemini-3.8-flash-tts gemini-3.8-flash-lite-tts gemini-robotics-er-2-preview gemini-2.5-computer-use-preview-10-2025 antigravity-preview-05-2026 antigravity-preview-09-2026 antigravity-preview-latest deep-research-max-preview-04-2026 deep-research-preview-04-2026 deep-research-pro-preview-12-2025 gemini-embedding-001 gemini-embedding-2-preview gemini-embedding-2 aqa gemini-3.5-transcribe-live gemini-2.5-flash-native-audio-latest gemini-2.5-flash-native-audio-preview-09-2025 gemini-2.5-flash-native-audio-preview-12-2025 gemini-3.1-flash-live-preview gemini-3.8-live gemini-3.8-live-extended-thinking gemini-robotics-er-2-streaming-preview gemini-3.5-live-translate-preview lyria-realtime-exp]
  anthropic: ok 14 models
  gemini: ok 61 models
  openai: ok 96 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get OpenAI Models ===
OpenAI models: [tts-1 gpt-5-mini gpt-5-mini-2025-08-07 gpt-5.4-2026-03-05 gpt-5-nano gpt-5.4-mini omni-moderation-2024-09-26 gpt-live-transcribe gpt-5.6-luna gpt-4o-2024-11-20 gpt-5.1 whisper-1 gpt-5-pro gpt-realtime-whisper omni-moderation-latest gpt-realtime-2 gpt-5-search-api gpt-5.5-2026-04-23 gpt-5.5-pro gpt-audio-2025-08-28 gpt-4o-mini-transcribe-2025-03-20 chat-latest gpt-realtime-translate gpt-5.4-pro-2026-03-05 gpt-4o-mini-transcribe-2025-12-15 gpt-5-pro-2025-10-06 gpt-5.6-sol gpt-5.4-mini-2026-03-17 text-embedding-3-large gpt-4o-2024-08-06 o3 gpt-5.4-pro gpt-4o-mini-tts gpt-realtime-mini-2025-12-15 gpt-realtime-2.1-mini gpt-realtime-2025-08-28 tts-1-hd-1106 tts-1-hd gpt-realtime-1.5 gpt-realtime gpt-5.2-pro gpt-4o-transcribe-diarize gpt-5.1-2025-11-13 gpt-5.4 text-embedding-3-small gpt-5-2025-08-07 o3-2025-04-16 gpt-5.2-2025-12-11 chatgpt-image-latest tts-1-1106 gpt-5.5-pro-2026-04-23 gpt-5.2-pro-2025-12-11 gpt-5.4-nano gpt-audio-mini-2025-12-15 gpt-audio-mini gpt-4.1-mini-2025-04-14 gpt-5 gpt-4o-mini-transcribe gpt-image-2-2026-04-21 gpt-transcribe gpt-image-2 text-embedding-ada-002 gpt-realtime-2.1 gpt-4.1 gpt-5.5 gpt-audio-1.5 gpt-image-1.5 gpt-6.1-sol gpt-6-sol gpt-realtime-mini gpt-image-1-mini gpt-5.6-terra gpt-5-nano-2025-08-07 gpt-4o-transcribe gpt-4.1-mini gpt-4o-mini gpt-6.1-terra gpt-6-luna gpt-4o-mini-tts-2025-12-15 gpt-audio gpt-5.3-codex gpt-5-search-api-2025-10-14 gpt-4o gpt-image-2.6-flare gpt-4o-mini-2024-07-18 gpt-4.1-2025-04-14 gpt-image-2.5-flare gpt-4o-mini-tts-2025-03-20 gpt-5.2 gpt-5.4-nano-2026-03-17 gpt-6-astra gpt-image-2.5-sunburst-2026-09-08 gpt-image-2.5-sunburst gpt-image-2.5-flare-2026-09-08 gpt-image-2.6-sunburst gpt-live-1]

=== Get Anthropic Models ===
Anthropic models: [claude-haiku-5-5 claude-sonnet-5-5 claude-opus-5-5 claude-fable-5-1 claude-opus-5 claude-sonnet-5 claude-fable-5 claude-opus-4-8 claude-opus-4-7 claude-sonnet-4-6 claude-opus-4-6 claude-opus-4-5-20251101 claude-haiku-4-5-20251001 claude-sonnet-4-5-20250929]

✓ Chat Models API example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: VWK1YDgSjY9W56aV0ioMitQRNMavyYpgpgazhg6siwp7279Q-xcU5O60QzU0mDARk8WYKpSSZIqj5P2XP-kCgw

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is **ekoDB**, described as a high-performance database product, priced at **$99**.

✓ Message 2 sent
  Response: ekoDB costs **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: pUK-I9qOnonQRf_8WhY5poEsV9MFTn6mCCyQcZRy-VenQeBef2LhGISU3n0mUYJAqkx9iPLZKGWeMTw-ToSCVg
  Parent: VWK1YDgSjY9W56aV0ioMitQRNMavyYpgpgazhg6siwp7279Q-xcU5O60QzU0mDARk8WYKpSSZIqj5P2XP-kCgw

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: pUK-I9qOnonQRf_8WhY5poEsV9MFTn6mCCyQcZRy-VenQeBef2LhGISU3n0mUYJAqkx9iPLZKGWeMTw-ToSCVg (Untitled)
  Session 2: VWK1YDgSjY9W56aV0ioMitQRNMavyYpgpgazhg6siwp7279Q-xcU5O60QzU0mDARk8WYKpSSZIqj5P2XP-kCgw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: pUK-I9qOnonQRf_8WhY5poEsV9MFTn6mCCyQcZRy-VenQeBef2LhGISU3n0mUYJAqkx9iPLZKGWeMTw-ToSCVg

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: ac9wUVSuTYMk3Rxbe1N09ESw1iNYzX_4_4DcaSmi4x4IuSnm1zJOCVsNFTT1fbu0cKZRwSeEVD_Z_j577P_OgA

=== List Collections ===
Total collections: 13
Sample collections: [chat_raw_completions__ek0_testing chat_agent_configs__ek0_testing schedules__ek0_testing functions__ek0_testing client_collection_management_go]

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
✓ Created record with native map: map[id:UtJCzySFFreBFp0IV3e-lfn6LZO52ODvYEUPnJBOIy7gWXFqTSnjXinjcnyIdX088arJ8eIH33VHnyIx4LZFNA]

=== Upsert Operation ===
✓ First upsert (update): map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:UtJCzySFFreBFp0IV3e-lfn6LZO52ODvYEUPnJBOIy7gWXFqTSnjXinjcnyIdX088arJ8eIH33VHnyIx4LZFNA name:map[type:String value:Alice Johnson]]
✓ Second upsert (insert): map[id:new-user-1794156808348011000]

=== Find One Operation ===
✓ Found user by email: map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:UtJCzySFFreBFp0IV3e-lfn6LZO52ODvYEUPnJBOIy7gWXFqTSnjXinjcnyIdX088arJ8eIH33VHnyIx4LZFNA name:map[type:String value:Alice Johnson]]
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
✓ Inserted document: N9K65yqgH6a5DfQAekFcPi_J_8dpcJol-9qigWqPGOuNzIdlaqBCaHAnAZ1ARy33osIYx6nBp7vBaDyE_LdCdA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: WRgsgFShqvQdB0zUEww-s-UF0El7iZUXgxqr8gmlVzrwLsvowRx-WZ44TenkdgtN7EnaerzUtmg0ZamptdTf6Q

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
✓ Edge cache script created: mLoDDLfwshBmhsaFQpkOycXRoycLlf0F34F6tKIzq53AphsvraaQ4EXII-Cti8OagsS7v9I1c_aH0_MgTLUHXg

Call 1: Cache lookup
Response time: 0ms
Found 1 cached entries

Call 2: Cache lookup (connection warm)
Response time: 0ms
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
   ⏱️  Duration: 49.467291ms
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
   ⏱️  Duration: 479µs
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
🤖 AI Response:
   Vector databases are designed to store and search **embeddings**—numerical representations of text, images, audio, or other data. Their main benefits include:

- **Semantic search:** Find items by meaning or similarity, not just exact keywords.
- **Fast similarity matching:** Efficiently search large collections for the nearest vectors.
- **Multimodal search:** Compare different data types, such as matching an image to a text description, when they share an embedding space.
- **Metadata filtering:** Combine similarity search with filters such as date, category, or access permissions.
- **AI application support:** Provide relevant context for retrieval-augmented generation (RAG), recommendations, deduplication, and anomaly detection.
- **Scalability:** Many are built to index and search large embedding collections efficiently.

They’re most useful when similarity matters; for exact lookups, transactions, or ordinary structured queries, a traditional database may be a better fit.
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

✅ Function saved: NRQ2kuuIsIrjl73Fak_0oGDb06UeIsQpqxa669vPMtgrZ9y0n96huTGnKcy4bqlQsek2Z7_o3XE-Z6_Kh3INBA
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

✅ Inserted order: _8IP4sDiaxAHfEE1YDpS0NdXtuMh8zemTYba_hvNdMyq9rnMNTeSAR3vGpP3Qp5gOa7JvFf1r_74Ls_xI06ggw
✅ Inserted 2 products with wrapped types

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: uYoFI2y0nl-w3Qhm-aIHbFAWFxj7akwweFHlH_KNyaAS6Lpg2oVylKgKWLBXCXAlFkp-KN-ws87IxkhTgokNmQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: map[type:Object value:map[role:admin userId:user_abc]]
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session
📝 Example 4: KV Operations in Functions

✅ Function saved: FNQuRRDdkt_IT74MTYBFrvgHU1yW5FSddFkBFRmMdHPzOlGMbPm7odPvm2zmrhTxouS2kejrt00d_giMDQnJAg
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: _6j_uZVMQCWTMIfcXn_PKvBYRDOA9j2Mzyl4n7ovcBsBp3jTtHKrVyH_RDfXZxunOrIQAMbGs5t3_Xv7ElKz8w
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
   1. map[type:String value:Natural Language Processing] (map[type:String value:AI])
   2. map[type:String value:Getting Started with ekoDB] (map[type:String value:Database])
   3. map[type:String value:Database Design Principles] (map[type:String value:Database])
   4. map[type:String value:Vector Databases Explained] (map[type:String value:Database])
   5. map[type:String value:Introduction to Machine Learning] (map[type:String value:AI])
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   map[category:map[type:String value:Database] count:map[type:Integer value:3]]
   map[category:map[type:String value:AI] count:map[type:Integer value:2]]
⏱️  Execution time: 0ms

✅ All search script examples finished!
🧹 Cleaning up...
✅ Cleanup complete
🚀 ekoDB Functions Example (Go Client)

✅ Client initialized

🧹 Cleaning up...
✅ Deleted collection
✅ Deleted test scripts

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: 2QKLkGP9WV9beI5YSFogMpfVpC7QFSwgJowX7jMUhIUwXlS69AwVZAgu_TVsA9FjC4f4a0Kzt91YHOTmv7HIXw
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
=== ekoDB Goal Template CRUD Example (Go) ===

--- Creating goal template ---
Created template: Data Migration (id: QoNMKOxtSi_vti2P8jukyWmjqTqC-DJa7_FLO4Zu3fhWSW4RPCZOuaxVKLuB5H-zBbiIe6DlA-r9c_e46o6BbQ)

--- Listing templates ---
Templates: map[count:1 items:[map[description:map[type:String value:Template for migrating data between schemas] id:QoNMKOxtSi_vti2P8jukyWmjqTqC-DJa7_FLO4Zu3fhWSW4RPCZOuaxVKLuB5H-zBbiIe6DlA-r9c_e46o6BbQ owner:map[type:Object value:map[key_id:3a0cdba0da7c060610042783cc758f1f key_label:default_admin]] steps:map[type:Array value:[map[description:Analyze source schema] map[description:Create target schema] map[description:Migrate records] map[description:Validate results]]] title:map[type:String value:Data Migration]]] total:1]

--- Getting template ---
Fetched: Data Migration

--- Updating template ---
Updated description: Updated: comprehensive data migration workflow

--- Deleting template ---
Template deleted successfully

✓ Goal template CRUD example completed
=== ekoDB Goals, Tasks & Agents Integration Example (Go) ===

--- Creating goal ---
Created goal: Deploy v2.0 (id: jyRga1f6EB4UVgr2WY88T6wdybw1xx3ieCsTBg4uRNbbfuurdV5pN5bEvtqUWUo8qxIbjeNPTfFHo60rQlwHGQ)

--- Listing goals ---
Goals response: map[count:1 goals:[map[created_at:map[type:DateTime value:2026-11-08T16:53:39.412859+00:00] description:map[type:String value:Ship the v2.0 release to production] id:jyRga1f6EB4UVgr2WY88T6wdybw1xx3ieCsTBg4uRNbbfuurdV5pN5bEvtqUWUo8qxIbjeNPTfFHo60rQlwHGQ owner:map[type:Object value:map[key_id:3a0cdba0da7c060610042783cc758f1f key_label:default_admin]] status:map[type:String value:pending] steps:map[type:String value:[{"description":"Run integration tests"},{"description":"Build release artifacts"},{"description":"Deploy to staging"}]] title:map[type:String value:Deploy v2.0] updated_at:map[type:DateTime value:2026-11-08T16:53:39.412859+00:00]]] total:1]

--- Getting goal ---
Fetched goal: Deploy v2.0

--- Updating goal ---
Updated description: Ship the v2.0 release to production (with hotfix)

--- Searching goals ---
Search result: map[count:1 items:[map[_score:12.870000000000001 created_at:map[type:DateTime value:2026-11-08T16:53:39.412859+00:00] description:map[type:String value:Ship the v2.0 release to production (with hotfix)] id:jyRga1f6EB4UVgr2WY88T6wdybw1xx3ieCsTBg4uRNbbfuurdV5pN5bEvtqUWUo8qxIbjeNPTfFHo60rQlwHGQ owner:map[type:Object value:map[key_id:3a0cdba0da7c060610042783cc758f1f key_label:default_admin]] status:map[type:String value:pending] steps:map[type:String value:[{"description":"Run integration tests"},{"description":"Build release artifacts"},{"description":"Deploy to staging"}]] title:map[type:String value:Deploy v2.0] updated_at:map[type:DateTime value:2026-11-08T16:53:39.418592+00:00]]]]

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
Created task: Run benchmarks (id: 6wh7k7WKaptbc05tn2yUa0S38t_q-XRvlzbL-xDqwIa8YZoUlZ4B9D5CsHR75mx5KD_BuyQIQnKg0Cs2fI_1UQ)

--- Listing tasks ---
Tasks: map[count:1 items:[map[description:map[type:String value:Execute YCSB benchmarks against staging] due_at:map[type:DateTime value:2026-11-09T16:53:39+00:00] id:6wh7k7WKaptbc05tn2yUa0S38t_q-XRvlzbL-xDqwIa8YZoUlZ4B9D5CsHR75mx5KD_BuyQIQnKg0Cs2fI_1UQ name:map[type:String value:Run benchmarks] owner:map[type:Object value:map[key_id:3a0cdba0da7c060610042783cc758f1f key_label:default_admin]]]] total:1]

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
Created agent: benchmark-runner (id: sGtVYrY7owfVZxs9zrDDJQ16_WvBZqbIdiBK9XU7bnPmeQ8thBzTIyRTgJfyWWSoI5U2Wh1AV-bfVhrOLf4tJQ)

--- Listing agents ---
Agents: map[count:1 items:[map[deployment_id:map[type:String value:deploy_prod_1] description:map[type:String value:Runs periodic benchmarks] id:sGtVYrY7owfVZxs9zrDDJQ16_WvBZqbIdiBK9XU7bnPmeQ8thBzTIyRTgJfyWWSoI5U2Wh1AV-bfVhrOLf4tJQ llm_model:map[type:String value:gpt-6-luna] name:map[type:String value:benchmark-runner] owner:map[type:Object value:map[key_id:3a0cdba0da7c060610042783cc758f1f key_label:default_admin]] run_policy:map[type:String value:{"trigger":{"allow_write_operations":false,"write_tools":[]},"eval":{"allow_write_operations":false,"write_tools":[]},"customer":{"allow_write_operations":false,"write_tools":[]}}]]] total:1]

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
Inserted doc1: goT-z5XyS7-EgxIN9Nc4gI8ZERGoJfmWsZLDN41dd8YQHk3IroXZ4iSYelPngn8APD06WZA8XdOw66eyGvlm9Q
Inserted doc2: I2jE-Y3GH4nU1IxyvWuGPVZpG0P9zdC9uc-DzbtgDTixwtthUNYg8EeGEenLeRzcew1wcba4qoAPBc_8ZzG4cA

--- Setting KV entry ---
KV entry set: team:engineering:go

--- Linking KV key to documents ---
Linked doc1: map[]
Linked doc2: map[]

--- Getting links ---
Links for team:engineering:go: [map[collection:kv_links_example_go created_at:2026-11-08T16:53:41.095796Z document_id:goT-z5XyS7-EgxIN9Nc4gI8ZERGoJfmWsZLDN41dd8YQHk3IroXZ4iSYelPngn8APD06WZA8XdOw66eyGvlm9Q field_path:<nil> last_accessed:2026-11-08T16:53:41.096132Z metadata:map[]] map[collection:kv_links_example_go created_at:2026-11-08T16:53:41.095980Z document_id:I2jE-Y3GH4nU1IxyvWuGPVZpG0P9zdC9uc-DzbtgDTixwtthUNYg8EeGEenLeRzcew1wcba4qoAPBc_8ZzG4cA field_path:<nil> last_accessed:2026-11-08T16:53:41.096132Z metadata:map[]]]

--- Unlinking doc2 ---
Unlink result: map[]

--- Verifying links after unlink ---
Links after unlink: [map[collection:kv_links_example_go created_at:2026-11-08T16:53:41.095796Z document_id:goT-z5XyS7-EgxIN9Nc4gI8ZERGoJfmWsZLDN41dd8YQHk3IroXZ4iSYelPngn8APD06WZA8XdOw66eyGvlm9Q field_path:<nil> last_accessed:2026-11-08T16:53:41.096466Z metadata:map[]]]

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
Response: In traditional art, the primary colors are **red, yellow, and blue**. For light, they’re **red, green, and blue**.

--- Structured Output via SSE ---
JSON response: [
  {"name": "Earth", "diameter_km": 12742},
  {"name": "Mars", "diameter_km": 6779},
  {"name": "Jupiter", "diameter_km": 139820}
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello! How can I help you today?

=== Done ===
=== ekoDB Schedules Integration Example (Go) ===

--- Creating schedule ---
Created schedule: nightly-backup (id: ce18e8fe-774b-4975-bf5e-68ccb4c956d9)

--- Listing schedules ---
Schedules: map[count:1 schedules:[map[created_at:2026-11-08T16:53:50.102466Z created_by:map[key_id:3a0cdba0da7c060610042783cc758f1f key_label:default_admin] cron_expression:0 0 0 * * * description:Runs a database backup every night at midnight enabled:true function_label:schedule_noop_go_71802 id:ce18e8fe-774b-4975-bf5e-68ccb4c956d9 last_execution:<nil> name:nightly-backup next_execution:2026-11-09T00:00:00Z parameters:map[] stats:map[avg_execution_time_ms:0 failed_executions:0 last_error:<nil> successful_executions:0 total_executions:0] timezone:UTC updated_at:2026-11-08T16:53:50.102466Z]]]

--- Getting schedule ---
Fetched schedule: nightly-backup (cron: 0 0 0 * * *)

--- Updating schedule ---
Updated cron: 0 0 2 * * *

--- Triggering schedule ---
Trigger response: map[schedule_id:ce18e8fe-774b-4975-bf5e-68ccb4c956d9 status:triggered]

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
  1. Score: 0.764
  2. Score: 0.752
  3. Score: 0.741

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.496
  2. Score: 0.906
  3. Score: 0.301

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
Inserted: map[id:LURdF_UrP7H9yzM-LboEEJsMB1Yy5kVtNpZcGI8ewREFxupWRlkw5-6TbU_Q1eprwryaR1D0JPsmj3fvnFs8Vg]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-11-08T16:53:51+00:00] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:LURdF_UrP7H9yzM-LboEEJsMB1Yy5kVtNpZcGI8ewREFxupWRlkw5-6TbU_Q1eprwryaR1D0JPsmj3fvnFs8Vg metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Test Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:42]]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-11-08 16:53:51 +0000 +0000
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-11-08T16:53:51+00:00 data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:LURdF_UrP7H9yzM-LboEEJsMB1Yy5kVtNpZcGI8ewREFxupWRlkw5-6TbU_Q1eprwryaR1D0JPsmj3fvnFs8Vg metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-11-08T16:53:51+00:00] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:LURdF_UrP7H9yzM-LboEEJsMB1Yy5kVtNpZcGI8ewREFxupWRlkw5-6TbU_Q1eprwryaR1D0JPsmj3fvnFs8Vg metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: iUQ_amLXopOJJ0_W6YZPhIzhaQvwXqnB_OU0wpcA7v8-AjmHUZamb2JHh2HTRFJpDAdFEebF5rvICA-Bk5FkYQ

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
✓ Created native SWR script: github_user_native_go (V0Oz9OsC43PceP4eVIJ692yQYpxMI9s9VdNp4aE7mbun6aFjXRd7qWEYtQJ54plDwsX1BsR1QnIQ0g93MHHgpw)

First call (cache miss - will fetch from GitHub API):
  Response time: 128ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 2ms
  Speedup: 64.0x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_go (GA2kteCrVLSIAAMWFnX9OTrmc3QVotvFHSEH_QdvWTS5Ev5IS7hbpRRmDbFPLGcr2_rP4z8xTbracGyG9dghiQ)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_go' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_go (4YTdf_BgDD1MDIjUn5gRrpXI9NVsJRiLrXxkoiWuWkyFAhAY_VceY6G7s1yImuouA2rkowG7kPVTx5yhtPg9bg)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_go' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_go (3a7j2-uPvnHChtySFpyn0Wq_aRHpIM4WcOLCQkO9PP_HB1Hp33i1pgEVG3L3Ln2m2OKOvBLvuugy5aZ9Ffs5Iw)
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
✓ Created SWR script: fetch_api_user_go (n1FI_7QxW8JgLCy0xhdxPJhSL0_Ddu5rmPS_wRZA8X8eGS_R8mypgh4ISrnZ3dN-ua5Z4FxjpwvBsDUnuzjzkw)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "cached_at": {
        "type": "DateTime",
        "value": "2026-11-08T16:53:53+00:00"
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
Response time: 0ms (served from cache)
✓ Lightning fast cache hit

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
🧹 Cleaning up...
✓ Cleanup complete

✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: mTSW5Lw42kZKlZii5rSCK0NWgbBC4jbAUf8rEcebhr3C-daC2fEzSjoadeA4LoFRTDTaqVXt1pJKeMLdg-jGGg
Created Bob: $500 - ID: cDR0C6V98DSDCvZR2LczzVMP27GcRLtkI-f4Xvi-QFa_4k5NWp2cfbfnsp7uG5ENRyBGQZjKSJGuCi9JW5VRtg

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 9f3d29ac-cbb8-43ff-af6c-a827e7e7f517

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
New transaction: 1c64a0f5-fe47-4e08-9835-784cfdb1f298
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Create User Function ===
Created user function with ID: ZB079MujhBqeonNIRN0ti5nBFZ2IRlNUta_S01H8BXO8c11vV4SIUPMVBss2e7mX0Iugg1xOZCwfDbPeIFYVtw

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

Created chat session: hXsoO-mu44496eaMKWm5tBK-Xmyxrio0ZQyLS9nnCEWdjog67_5CbrxHBTp5jdJkIJEtsk3lYo6v02tM8L4Xfg

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"gpt-5.6-luna","provider":"openai"}The capital of France is **Paris**.

--- Stream ended ---
Message ID: tkKiHJoRqmUhy5AE40HB62A6F3ynHgICsgQYCY9jJPMIPdaNPMnRX-QBF0FhoNT0XbqcOhLXVzuuLcyxrCnT_A
Execution time: 698ms
Token usage: {"cached_input_tokens":0,"completion_tokens":13,"prompt_tokens":23,"thinking_tokens":0,"total_tokens":36}

Full response: {"__progress":"received"}{"__progress":"generating","model":"gpt-5.6-luna","provider":"openai"}The capital of France is **Paris**....
=== WebSocket Subscription Example (Go) ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_go' ===
✓ Subscribed (subscription_id: sub_22e49d81b427402cafb4859b198429ac)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: HoR6DIZtB1DOO1Z1Knul-zcy2nH_JelsUmmiHT90lwPflKiBAVC78nbieYpanBMQSOjLV-wTfgM8phK2VOa8NQ

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_go
     Record IDs: [HoR6DIZtB1DOO1Z1Knul-zcy2nH_JelsUmmiHT90lwPflKiBAVC78nbieYpanBMQSOjLV-wTfgM8phK2VOa8NQ]
     Timestamp:  2026-11-08T16:53:56.567468+00:00

Inserting record 2...
✓ Inserted: Kxk7QRT1LRBG-lvEITg0Vb2ybDjUtXxCtMZ8QZYPxr3D7LY4dFrMAFwwF2uydr-m92WgY1g010KoV2TgjajGcA

  📡 Notification received:
     Event:      insert
     Record IDs: [Kxk7QRT1LRBG-lvEITg0Vb2ybDjUtXxCtMZ8QZYPxr3D7LY4dFrMAFwwF2uydr-m92WgY1g010KoV2TgjajGcA]

=== Unsubscribing ===
✓ Unsubscribed

✓ WebSocket subscription example completed successfully
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: RBfdR91gtsFu8LeB186xIFqjdNVqhgsL6DBekItXycYBQKJVUcrhHkBaDtTvyCEl2LwzLCG_Ofo5hl7qhy1h8g

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
Inserted with ripple: map[id:CNJz70ZxXbSPTo_Joy7NeD1MkGxGXIff7GHvxDXT2f3i_TG2W4Fh_0YNJJiDciftagZWuUbxXJsNvoAln-RikA]
Inserted with bypass_ripple: map[id:EOx31lnZDB4aUViW7_aVo4TeXuQevSNIE0nDwl2oTUvbqgL3PMF9B9-ExX9NcE7VCovjlmAGoWT281pMuwd85Q]
Inserted with TTL and bypass_ripple: map[id:qT_DKIYN8QB3-1N1EkHDf83Udjv-Af_iwbYhRVPmZpW9SVnsNz_X0kK8HIA497E8zACT5GCqZCasq-13SOHreA]
Updated with bypass_ripple: map[id:CNJz70ZxXbSPTo_Joy7NeD1MkGxGXIff7GHvxDXT2f3i_TG2W4Fh_0YNJJiDciftagZWuUbxXJsNvoAln-RikA name:map[type:String value:Product 1] price:map[type:Integer value:150]]
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

> @ekodb/ekodb-client@0.32.0 prepare
> npm run build


> @ekodb/ekodb-client@0.32.0 build
> tsc


up to date, audited 44 packages in 921ms

12 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
npm warn install-scripts 1 package has install scripts not yet covered by allowScripts:
npm warn install-scripts   fsevents@2.3.3 (install: (install scripts present))
npm warn install-scripts
npm warn install-scripts Run `npm install-scripts ls` to review, or `npm install-scripts approve <pkg>` to allow.

> @ekodb/ekodb-client@0.32.0 build
> tsc

✅ TypeScript client built!

added 1 package, removed 1 package, and audited 13 packages in 452ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
npm warn install-scripts 3 packages have install scripts not yet covered by allowScripts:
npm warn install-scripts   ekodb-client-ts@0.32.0 (prepare: npm run build)
npm warn install-scripts   esbuild@0.28.1 (postinstall: node install.js)
npm warn install-scripts   fsevents@2.3.3 (install: (install scripts present))
npm warn install-scripts
npm warn install-scripts Run `npm install-scripts ls` to review, or `npm install-scripts approve <pkg>` to allow.
Admin permissions read successfully
=== ekoDB Advanced CRUD Example (TypeScript) ===

--- Inserting base record ---
Inserted: Ww91CocKTstsTIbOwCxhVpTrVNNLh4imGS5EWChmuhDtR-HFmbKgJ9P7bUFeKEw0zUD4sKU1efwYXt6FIDfa3g

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
✓ Created session: WQXdkWX05JnnqeZl8hdKxyYlPi6Bb7BW90S2W5odU0spROssJNBx8Bx5qKhuCCTfH7u722Mtc7pSpCgSKG3EXg

=== Sending Initial Message ===
✓ Message sent
  Response: The available product listed is **ekoDB**, described as a high-performance database product, priced at **99**. The currency wasn’t specified.

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
✓ Created second session: ATADWYb6BSWW5QQsdDr-kZKkSyXvly4QVhHzDfTk8Spr6kSgY44lvCaOJiVTJmOouvYo-2UcY261txfmcG4GTA
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: ATADWYb6BSWW5QQsdDr-kZKkSyXvly4QVhHzDfTk8Spr6kSgY44lvCaOJiVTJmOouvYo-2UcY261txfmcG4GTA
✓ Deleted chat session: WQXdkWX05JnnqeZl8hdKxyYlPi6Bb7BW90S2W5odU0spROssJNBx8Bx5qKhuCCTfH7u722Mtc7pSpCgSKG3EXg
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: 0tDVOfiJLYZohMnIUHo47907BZj2_2I7Ko8G4SbFX9ct6L6Zaf2GeO8Bfsof7T51xWgPu_tHHbwbQ1VRfeSqqQ

=== Sending Chat Message ===
Message ID: cN5pmdYhThNqLqzdcJEWCTCiQ3t4YS-wm3sUXe9EJ4jgSKHzoEnIB7WE4Bk7tRr5EiovqtTGUKKk24A2o0z_Fw

=== AI Response ===
Available products and their listed prices:

- **ekoDB** — 99
- **ekoDB Pro** — 299
- **ekoDB Cloud** — 499

The currency isn’t specified in the provided data.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Fully managed cloud database service product',
    name: 'ekoDB Cloud',
    price: 499,
    id: '4M1Waecu9uv0xLsI9QL7oJAORzHL27qrP-IhZHrci2jekOD7RYpvFZ2ZnyYSc863DI_rG5RzXzIQKnuYmjol2Q'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Enterprise edition product with advanced features',
    name: 'ekoDB Pro',
    price: 299,
    id: 'QrPFSnw9UF6uyV7THtF1IVEeESWvrYJTgvc3HtD3m8p8IzMMv1bcNRcBXYqV3MgSda9wRmtpuyO3tlMfbrMaWA'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'hPQ2sBk1YmSxRxslkRG5bmpGbWX8P9X8NQ8qKR99fZl0Uuz7800pq6LVBFEQiK4J5PsSf5RWJpOI_25rKCx0rg',
    name: 'ekoDB',
    description: 'A high-performance database product with AI capabilities',
    price: 99
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 1853ms

=== Token Usage ===
Prompt tokens: 13711
Completion tokens: 80
Total tokens: 13791

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Message Stream (SSE) Example (TypeScript) ===

Created session: QkMluyZfaloTSCGJPcP6X-W7nJH-KtYG4eb8P97dYY9-5lhPpU4ryxzqQVu-zE8C1Z9vTkmV6dieKUE4RAyctQ

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"gpt-6-luna","provider":"openai"}**ekoDB** is a database platform for application developers. It provides a way for apps to store and access data—typically through an API or SDK—without having to build all the database infrastructure themselves. The name may refer to more than one project, so share a link if you mean a specific ekoDB.

--- Stream complete ---
Message ID: _yhCRa8ecxPuE9ABKpTxqPV4S_YI7NlVGOVElvAKFAka1bpBSCgNACu16zaWsz_GApSCXCNvXwMQwri0hYPc7w
Execution time: 7312ms
Context window: 128000 tokens

✓ Chat message stream example completed
✓ Client created

=== List Chat Models ===
Available chat models by provider:
  openai:
    - tts-1
    - gpt-5-mini
    - gpt-5-mini-2025-08-07
    - gpt-5.4-2026-03-05
    - gpt-5-nano
    - gpt-5.4-mini
    - omni-moderation-2024-09-26
    - gpt-live-transcribe
    - gpt-5.6-luna
    - gpt-4o-2024-11-20
    - gpt-5.1
    - whisper-1
    - gpt-5-pro
    - gpt-realtime-whisper
    - omni-moderation-latest
    - gpt-realtime-2
    - gpt-5-search-api
    - gpt-5.5-2026-04-23
    - gpt-5.5-pro
    - gpt-audio-2025-08-28
    - gpt-4o-mini-transcribe-2025-03-20
    - chat-latest
    - gpt-realtime-translate
    - gpt-5.4-pro-2026-03-05
    - gpt-4o-mini-transcribe-2025-12-15
    - gpt-5-pro-2025-10-06
    - gpt-5.6-sol
    - gpt-5.4-mini-2026-03-17
    - text-embedding-3-large
    - gpt-4o-2024-08-06
    - o3
    - gpt-5.4-pro
    - gpt-4o-mini-tts
    - gpt-realtime-mini-2025-12-15
    - gpt-realtime-2.1-mini
    - gpt-realtime-2025-08-28
    - tts-1-hd-1106
    - tts-1-hd
    - gpt-realtime-1.5
    - gpt-realtime
    - gpt-5.2-pro
    - gpt-4o-transcribe-diarize
    - gpt-5.1-2025-11-13
    - gpt-5.4
    - text-embedding-3-small
    - gpt-5-2025-08-07
    - o3-2025-04-16
    - gpt-5.2-2025-12-11
    - chatgpt-image-latest
    - tts-1-1106
    - gpt-5.5-pro-2026-04-23
    - gpt-5.2-pro-2025-12-11
    - gpt-5.4-nano
    - gpt-audio-mini-2025-12-15
    - gpt-audio-mini
    - gpt-4.1-mini-2025-04-14
    - gpt-5
    - gpt-4o-mini-transcribe
    - gpt-image-2-2026-04-21
    - gpt-transcribe
    - gpt-image-2
    - text-embedding-ada-002
    - gpt-realtime-2.1
    - gpt-4.1
    - gpt-5.5
    - gpt-audio-1.5
    - gpt-image-1.5
    - gpt-6.1-sol
    - gpt-6-sol
    - gpt-realtime-mini
    - gpt-image-1-mini
    - gpt-5.6-terra
    - gpt-5-nano-2025-08-07
    - gpt-4o-transcribe
    - gpt-4.1-mini
    - gpt-4o-mini
    - gpt-6.1-terra
    - gpt-6-luna
    - gpt-4o-mini-tts-2025-12-15
    - gpt-audio
    - gpt-5.3-codex
    - gpt-5-search-api-2025-10-14
    - gpt-4o
    - gpt-image-2.6-flare
    - gpt-4o-mini-2024-07-18
    - gpt-4.1-2025-04-14
    - gpt-image-2.5-flare
    - gpt-4o-mini-tts-2025-03-20
    - gpt-5.2
    - gpt-5.4-nano-2026-03-17
    - gpt-6-astra
    - gpt-image-2.5-sunburst-2026-09-08
    - gpt-image-2.5-sunburst
    - gpt-image-2.5-flare-2026-09-08
    - gpt-image-2.6-sunburst
    - gpt-live-1
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
    - gemini-omni-1.2-flash
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
  gemini: ok 61 models
  openai: ok 96 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get Specific Provider Models ===
OpenAI models: tts-1, gpt-5-mini, gpt-5-mini-2025-08-07, gpt-5.4-2026-03-05, gpt-5-nano, gpt-5.4-mini, omni-moderation-2024-09-26, gpt-live-transcribe, gpt-5.6-luna, gpt-4o-2024-11-20, gpt-5.1, whisper-1, gpt-5-pro, gpt-realtime-whisper, omni-moderation-latest, gpt-realtime-2, gpt-5-search-api, gpt-5.5-2026-04-23, gpt-5.5-pro, gpt-audio-2025-08-28, gpt-4o-mini-transcribe-2025-03-20, chat-latest, gpt-realtime-translate, gpt-5.4-pro-2026-03-05, gpt-4o-mini-transcribe-2025-12-15, gpt-5-pro-2025-10-06, gpt-5.6-sol, gpt-5.4-mini-2026-03-17, text-embedding-3-large, gpt-4o-2024-08-06, o3, gpt-5.4-pro, gpt-4o-mini-tts, gpt-realtime-mini-2025-12-15, gpt-realtime-2.1-mini, gpt-realtime-2025-08-28, tts-1-hd-1106, tts-1-hd, gpt-realtime-1.5, gpt-realtime, gpt-5.2-pro, gpt-4o-transcribe-diarize, gpt-5.1-2025-11-13, gpt-5.4, text-embedding-3-small, gpt-5-2025-08-07, o3-2025-04-16, gpt-5.2-2025-12-11, chatgpt-image-latest, tts-1-1106, gpt-5.5-pro-2026-04-23, gpt-5.2-pro-2025-12-11, gpt-5.4-nano, gpt-audio-mini-2025-12-15, gpt-audio-mini, gpt-4.1-mini-2025-04-14, gpt-5, gpt-4o-mini-transcribe, gpt-image-2-2026-04-21, gpt-transcribe, gpt-image-2, text-embedding-ada-002, gpt-realtime-2.1, gpt-4.1, gpt-5.5, gpt-audio-1.5, gpt-image-1.5, gpt-6.1-sol, gpt-6-sol, gpt-realtime-mini, gpt-image-1-mini, gpt-5.6-terra, gpt-5-nano-2025-08-07, gpt-4o-transcribe, gpt-4.1-mini, gpt-4o-mini, gpt-6.1-terra, gpt-6-luna, gpt-4o-mini-tts-2025-12-15, gpt-audio, gpt-5.3-codex, gpt-5-search-api-2025-10-14, gpt-4o, gpt-image-2.6-flare, gpt-4o-mini-2024-07-18, gpt-4.1-2025-04-14, gpt-image-2.5-flare, gpt-4o-mini-tts-2025-03-20, gpt-5.2, gpt-5.4-nano-2026-03-17, gpt-6-astra, gpt-image-2.5-sunburst-2026-09-08, gpt-image-2.5-sunburst, gpt-image-2.5-flare-2026-09-08, gpt-image-2.6-sunburst, gpt-live-1

=== Get Anthropic Models ===
Anthropic models: claude-haiku-5-5, claude-sonnet-5-5, claude-opus-5-5, claude-fable-5-1, claude-opus-5, claude-sonnet-5, claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 404: {"error":"Unknown provider 'nonexistent_provider_xyz'","error_kind":"unknown_provider"}

✓ Chat Models example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: -j8xdgg2upC78ULhsHwJp7uSXhwIgdqKkwvvw4591VbHmC9c645ZCTeXasxjkuxDvauFxImxbqbocynfp78NXw

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
✓ Created branch: w9aP6mVntdUGxMKvqU_ZdTXYD3KtluoksASd1Ixo6uP2I3RYmHqMKtyRsN22HQllM5zj3s46PMAuh5W-hDHVQQ
  Parent: -j8xdgg2upC78ULhsHwJp7uSXhwIgdqKkwvvw4591VbHmC9c645ZCTeXasxjkuxDvauFxImxbqbocynfp78NXw

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: w9aP6mVntdUGxMKvqU_ZdTXYD3KtluoksASd1Ixo6uP2I3RYmHqMKtyRsN22HQllM5zj3s46PMAuh5W-hDHVQQ (Untitled)
  Session 2: -j8xdgg2upC78ULhsHwJp7uSXhwIgdqKkwvvw4591VbHmC9c645ZCTeXasxjkuxDvauFxImxbqbocynfp78NXw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: w9aP6mVntdUGxMKvqU_ZdTXYD3KtluoksASd1Ixo6uP2I3RYmHqMKtyRsN22HQllM5zj3s46PMAuh5W-hDHVQQ

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: WYCAt1MNx0tPMG9Y2017Lm3d2ulT9HjSed-MXJLwYDLE1MsQeRFKLZuOP93rL4lxyoGxVcyXGAN2EFL_zTDyPw

=== List Collections ===
Total collections: 17
Sample collections: schema_products_client_go,chat_raw_completions__ek0_testing,chat_agent_configs__ek0_testing,schedules__ek0_testing,functions__ek0_testing

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
  POST /api/functions/conc_demo_pay_ts_72807_1794156869177 { "idempotency_key": "...", "amount": 100 }
  POST /api/functions/conc_demo_rl_fail_ts_72807_1794156869177 { "user_id": 42 }
  POST /api/functions/conc_demo_rl_skip_ts_72807_1794156869177 { "user_id": 42 }
  POST /api/functions/conc_demo_lock_ts_72807_1794156869177 { "resource": "queue:drain" }

✓ Cleaned up demo functions
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'ABz_QZRdhe7JI4bTOtVp1PNl5Ok_Ue5Wl5L0J-KZPKpsoXJpVg7i18f07x09Vh_27UDGzSbXa0UIO3WA6E7Rdw'
}

=== Upsert Operation ===
✓ First upsert (update): {
  id: 'ABz_QZRdhe7JI4bTOtVp1PNl5Ok_Ue5Wl5L0J-KZPKpsoXJpVg7i18f07x09Vh_27UDGzSbXa0UIO3WA6E7Rdw',
  age: { type: 'Integer', value: 29 },
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  name: { type: 'String', value: 'Alice Johnson' },
  active: { value: true, type: 'Boolean' }
}
✓ Second upsert (insert): { id: 'new-user-id' }

=== Find One Operation ===
✓ Found user by email: {
  active: { value: true, type: 'Boolean' },
  id: 'ABz_QZRdhe7JI4bTOtVp1PNl5Ok_Ue5Wl5L0J-KZPKpsoXJpVg7i18f07x09Vh_27UDGzSbXa0UIO3WA6E7Rdw',
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  name: { value: 'Alice Johnson', type: 'String' },
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
✓ Inserted document: B9ccbRfk435s2WR0GSVA0se2fUqyGASxNIurvMyOjLn-QcWd8KGI6kJEQgR0o_KcXDIX_9C6SvDwCTrpjEuMPQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: xMbMmBPk6DPZeCAlg4Nss7BT9r_ZgHompPjFmQsxsySEexSGPMR80sJPyEX57hgYa-lvzymc6Q-lRmLiWWRRRg

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
✓ Edge cache script created: zXwUury1OnlTukeVqfvZRzqeAc1QE8pK61uK6sg7i4UeUIX27K7EAZR4sW5B4lK7YgBy97AE4ot5OMqLOmErZw

Call 1: Cache miss (fetches from API)
Response time: 52ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "username": "Bret",
          "id": 1,
          "phone": "1-770-736-8031 x56442",
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "bs": "harness real-time e-markets",
            "name": "Romaguera-Crona"
          },
          "website": "hildegard.org",
          "email": "Sincere@april.biz",
          "name": "Leanne Graham",
          "address": {
            "zipcode": "92998-3874",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "suite": "Apt. 556",
            "city": "Gwenborough",
            "street": "Kulas Light"
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
Response time: 2ms (26x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "username": "Bret",
          "id": 1,
          "phone": "1-770-736-8031 x56442",
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "bs": "harness real-time e-markets",
            "name": "Romaguera-Crona"
          },
          "website": "hildegard.org",
          "email": "Sincere@april.biz",
          "name": "Leanne Graham",
          "address": {
            "zipcode": "92998-3874",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "suite": "Apt. 556",
            "city": "Gwenborough",
            "street": "Kulas Light"
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
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 53ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "company": {
        "bs": "harness real-time e-markets",
        "name": "Romaguera-Crona",
        "catchPhrase": "Multi-layered client-serv...

Second call (cache hit - from cache):
   ⏱️  Duration: 1ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "company": {
        "bs": "harness real-time e-markets",
        "name": "Romaguera-Crona",
        "catchPhrase": "Multi-layered client-server neural-net"
      }...
   🚀 Cache speedup: 53.0x faster!

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
   {"count":{"value":3,"type":"Integer"},"avg_price":{"value":365.6666666666667,"type":"Float"},"category":{"type":"String","value":"Furniture"}}
   {"category":{"type":"String","value":"Electronics"},"avg_price":{"value":367,"type":"Float"},"count":{"value":5,"type":"Integer"}}
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
   Vector databases are designed to store and search **embeddings**—numeric representations of text, images, audio, or other data. Their main benefits include:

- **Semantic search:** Find items that are similar in meaning, even when they don’t share exact keywords.
- **Fast similarity search at scale:** Specialized indexes can efficiently search large collections of high-dimensional vectors.
- **Multimodal retrieval:** Search across content such as text, images, and audio when they’re represented as vectors.
- **Metadata filtering:** Combine similarity search with filters like date, category, or user ID.
- **Support for AI applications:** They’re commonly used for retrieval-augmented generation (RAG), recommendations, deduplication, and personalization.

They’re usually best alongside a traditional database: use a vector database for similarity retrieval and a relational or document database for structured records and transactions.
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

✅ Function saved: QFj1MNEkj96TqaVxomkQoR3uDkzMVHc3MISAx_F7fyiBsnAn0w4Slw1AJzziYkQDXlvztIpXTJ0E-yy8KI_Kkg
📊 Found 2 product groups
   {"avg_price":{"value":575.6666666666666,"type":"Float"},"count":{"value":3,"type":"Integer"},"category":{"type":"String","value":"Electronics"}}
   {"category":{"value":"Furniture","type":"String"},"avg_price":{"type":"Float","value":474},"count":{"value":2,"type":"Integer"}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"category":{"type":"String","value":"Electronics"},"count":{"type":"Integer","value":3}}
   {"category":{"type":"String","value":"Furniture"},"count":{"value":2,"type":"Integer"}}
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
   {"role":{"type":"String","value":"user"},"count":{"type":"Integer","value":7},"avg_score":{"type":"Float","value":70}}
   {"count":{"type":"Integer","value":3},"avg_score":{"type":"Float","value":20},"role":{"type":"String","value":"admin"}}
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

✅ Inserted order: ZMSxgHxYhWjOwdMOIkEPXJB6pFpiLrZRxY9ZrBUNw9-bKBPEfwNsW3HIo87izeyF8xT5SgK5jnhN4hD0nmHhFA
✅ Inserted 2 products with wrapped types

📝 Example 2: UserFunction with Wrapped Type Parameters

✅ Function saved: zcspIaw5KhGLN92UGuI7pvP5eAnkZGf4ED8YlX2bi6wcxvBpEOz8BgfiPpVAXGRfe6eSjxHUXBhP4tOxPY4tTw
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"value":{"userId":"user_abc","role":"admin"},"type":"Object"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: mpo8Jq0dqIyaL6BtjS8hrkcvYCDtIu6ULQoZJOwd9nAmyeNlsQLrTQBfgcXkeZgCce2-rKipCGg19Cv62GbqXQ
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: lPcvTRJSb6ZTjYbmIf2p4QiK8wvjiV7QEc2KHs5ucou86rdheUNISd8rVsKUUpTDzX1M_o2vSlMUXmNOzT6Mag
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
   2. Vector Databases Explained (Database)
   3. Natural Language Processing (AI)
   4. Getting Started with ekoDB (Database)
   5. Introduction to Machine Learning (AI)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"category":{"type":"String","value":"Database"},"count":{"type":"Integer","value":3}}
   {"category":{"type":"String","value":"AI"},"count":{"type":"Integer","value":2}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
🚀 ekoDB Functions Example (TypeScript)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: C1LR-gzetmXl8y0G73uSf5qM2fSabG9XLtZxUIunH8AaeBwN6X_7e5uiAHimCmU5CP9si6RGpc46-VDiVv48Ew
📊 Found 5 active users

📝 Example 2: Parameterized Function

✅ Function saved: _RG4Ozksn6zTNdFL0HLOnv4mlaXsxHxe-ws1sLOWNXw7__MrCZu7_a2CN9djqRjnMTWwA5bzXeUpAdgi9VDEKg
📊 Found 3 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: VAw1xRXONbitCIkz-rpxRV9GWVfMsF-8RgstwJlK4RjWfdI5zFtUV4Iejmc-ibasfH5DAMEnereQMOJNY2V4UQ
📊 Statistics: 2 groups
   {"avg_score":{"type":"Float","value":60},"status":{"type":"String","value":"active"},"count":{"type":"Integer","value":5}}
   {"count":{"type":"Integer","value":5},"avg_score":{"type":"Float","value":50},"status":{"type":"String","value":"inactive"}}

📝 Example 4: UserFunction Management

📋 Total scripts: 5
🔍 Retrieved script: Get Active Users
✏️  function updated
🗑️  function deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed!
=== ekoDB Goal Template CRUD Example (TypeScript) ===

--- Creating goal template ---
Created template: Data Migration (id: vn486V67QjY-H2pN6lkZFUqfVQIUnkTnmKD6Y7jE3YWf2nW3xqdNX3gFUhdjmPobdv3xep80jDfgn4LaagJ9vA)

--- Listing templates ---
Templates: {
  count: 1,
  items: [
    {
      description: [Object],
      id: 'vn486V67QjY-H2pN6lkZFUqfVQIUnkTnmKD6Y7jE3YWf2nW3xqdNX3gFUhdjmPobdv3xep80jDfgn4LaagJ9vA',
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
Created goal: Deploy v2.0 (id: EX8VWT7sRGg9TnJyDvT3-OZLAxHZDLmyaC5y3u0aLlm1_rGuN72-2VgmWAIpnxsdOECS2jFvoxHBL9rCMGA3vw)

--- Listing goals ---
Goals: {
  "count": 1,
  "goals": [
    {
      "created_at": {
        "type": "DateTime",
        "value": "2026-11-08T16:54:40.542830+00:00"
      },
      "description": {
        "type": "String",
        "value": "Ship version 2.0 to production"
      },
      "id": "EX8VWT7sRGg9TnJyDvT3-OZLAxHZDLmyaC5y3u0aLlm1_rGuN72-2VgmWAIpnxsdOECS2jFvoxHBL9rCMGA3vw",
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
        "value": "2026-11-08T16:54:40.542830+00:00"
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
        "value": "2026-11-08T16:54:40.542830+00:00"
      },
      "description": {
        "type": "String",
        "value": "Ship version 2.0 with hot-fix patches"
      },
      "id": "EX8VWT7sRGg9TnJyDvT3-OZLAxHZDLmyaC5y3u0aLlm1_rGuN72-2VgmWAIpnxsdOECS2jFvoxHBL9rCMGA3vw",
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
        "value": "2026-11-08T16:54:40.550890+00:00"
      }
    }
  ]
}

--- Goal step: start step 0 ---
Step 0 started on goal EX8VWT7sRGg9TnJyDvT3-OZLAxHZDLmyaC5y3u0aLlm1_rGuN72-2VgmWAIpnxsdOECS2jFvoxHBL9rCMGA3vw
--- Goal step: complete step 0 ---
Step 0 completed on goal EX8VWT7sRGg9TnJyDvT3-OZLAxHZDLmyaC5y3u0aLlm1_rGuN72-2VgmWAIpnxsdOECS2jFvoxHBL9rCMGA3vw
--- Goal step: fail step 1 ---
Step 1 failed on goal EX8VWT7sRGg9TnJyDvT3-OZLAxHZDLmyaC5y3u0aLlm1_rGuN72-2VgmWAIpnxsdOECS2jFvoxHBL9rCMGA3vw

--- Completing goal ---
Goal status: pending_review
--- Approving goal ---
Goal status after approve: in_progress

--- Creating goal to reject ---
--- Rejecting goal ---
Goal status after reject: failed


--- Creating task ---
Created task: Hourly Health Check (id: ays47243PZal4h6FMyhoQwN3RHcsetKNsxkCAKmE54A310jroOxM4MG8n7AFrJ2Yux-PmTax4JzYzfpiKGa6wQ)

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
      "id": "ays47243PZal4h6FMyhoQwN3RHcsetKNsxkCAKmE54A310jroOxM4MG8n7AFrJ2Yux-PmTax4JzYzfpiKGa6wQ",
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
Created agent: SupportBot (id: y82NGV9dHmPhZBZHOlNThYUCXFP1h2tdJsPTsEbmoulb0b2qIHHnOhFifkleFjBux_pp4pdDkFmYUY4U2HGy2Q)

--- Listing agents ---
Agents: {
  "count": 1,
  "items": [
    {
      "deployment_id": {
        "type": "String",
        "value": "deploy_prod_1"
      },
      "id": "y82NGV9dHmPhZBZHOlNThYUCXFP1h2tdJsPTsEbmoulb0b2qIHHnOhFifkleFjBux_pp4pdDkFmYUY4U2HGy2Q",
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
By name: SupportBot (id: y82NGV9dHmPhZBZHOlNThYUCXFP1h2tdJsPTsEbmoulb0b2qIHHnOhFifkleFjBux_pp4pdDkFmYUY4U2HGy2Q)

--- Updating agent ---
Updated agent: SupportBot

--- Getting agents by deployment ---
Agents in deployment: {
  "count": 0,
  "items": []
}
WARNING: agents-by-deployment omitted created agent y82NGV9dHmPhZBZHOlNThYUCXFP1h2tdJsPTsEbmoulb0b2qIHHnOhFifkleFjBux_pp4pdDkFmYUY4U2HGy2Q; TODO: check/fix the server-side deployment lookup

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
  POST /api/functions/jwt_register_ts_73353_1794156881678 { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/jwt_login_ts_73353_1794156881678 { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/jwt_verify_ts_73353_1794156881678 { "token": "<jwt>" }

Set JWT_SECRET in ekoDB's environment_vars whitelist before invoking.

✓ Cleaned up demo functions
=== ekoDB KV Document Linking Example (TypeScript) ===

--- Setup: creating KV key and documents ---
Set KV key: session:admin
Inserted document 1: 0kgJufq1foXMmjk1b4KhKbFOXcG9_eGTgQzoaREZRX7KmokPri45tzf8SSZLIy5J8QHUPucv1OWfiREVMvYHvw
Inserted document 2: r5KXTx5uroRGEplvHfR9AhbwPGMxOBp6eh_Pu36eb6Z6YLVL-A2nyuLe2sIXP40fx1FpDnte651ayHKug165GQ

--- Linking documents to KV key ---
Linked doc 0kgJufq1foXMmjk1b4KhKbFOXcG9_eGTgQzoaREZRX7KmokPri45tzf8SSZLIy5J8QHUPucv1OWfiREVMvYHvw: null
Linked doc r5KXTx5uroRGEplvHfR9AhbwPGMxOBp6eh_Pu36eb6Z6YLVL-A2nyuLe2sIXP40fx1FpDnte651ayHKug165GQ: null

--- Getting links for KV key ---
Links: [
  {
    "collection": "kv_links_example_ts_73383_1794156882088",
    "document_id": "r5KXTx5uroRGEplvHfR9AhbwPGMxOBp6eh_Pu36eb6Z6YLVL-A2nyuLe2sIXP40fx1FpDnte651ayHKug165GQ",
    "field_path": null,
    "created_at": "2026-11-08T16:54:42.161280Z",
    "last_accessed": "2026-11-08T16:54:42.162297Z",
    "metadata": {}
  },
  {
    "collection": "kv_links_example_ts_73383_1794156882088",
    "document_id": "0kgJufq1foXMmjk1b4KhKbFOXcG9_eGTgQzoaREZRX7KmokPri45tzf8SSZLIy5J8QHUPucv1OWfiREVMvYHvw",
    "field_path": null,
    "created_at": "2026-11-08T16:54:42.160059Z",
    "last_accessed": "2026-11-08T16:54:42.162297Z",
    "metadata": {}
  }
]

--- Unlinking document ---
Unlinked doc r5KXTx5uroRGEplvHfR9AhbwPGMxOBp6eh_Pu36eb6Z6YLVL-A2nyuLe2sIXP40fx1FpDnte651ayHKug165GQ: null

--- Verifying remaining links ---
Remaining links: [
  {
    "collection": "kv_links_example_ts_73383_1794156882088",
    "document_id": "0kgJufq1foXMmjk1b4KhKbFOXcG9_eGTgQzoaREZRX7KmokPri45tzf8SSZLIy5J8QHUPucv1OWfiREVMvYHvw",
    "field_path": null,
    "created_at": "2026-11-08T16:54:42.160059Z",
    "last_accessed": "2026-11-08T16:54:42.163724Z",
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
  kv_ops_ts_73415_1794156882585:cache:product:1: success
  kv_ops_ts_73415_1794156882585:cache:product:2: success
  kv_ops_ts_73415_1794156882585:cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  kv_ops_ts_73415_1794156882585:cache:product:1: { name: 'Product 1', price: 29.99 }
  kv_ops_ts_73415_1794156882585:cache:product:2: { price: 39.99, name: 'Product 2' }
  kv_ops_ts_73415_1794156882585:cache:product:3: { name: 'Product 3', price: 49.99 }

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
  kv_ops_ts_73415_1794156882585:cache:product:1: deleted
  kv_ops_ts_73415_1794156882585:cache:product:2: deleted
  kv_ops_ts_73415_1794156882585:cache:product:3: deleted

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
Response: It depends on the context: **red, yellow, and blue** are the traditional primary colors in art; **red, green, and blue** are primary colors of light.

--- Structured Output via SSE ---
JSON response: [
  {"name": "Earth", "diameter_km": 12742},
  {"name": "Mars", "diameter_km": 6779},
  {"name": "Jupiter", "diameter_km": 139820}
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello! I hope you’re having a great day.

=== Done ===
=== ekoDB Schedule Management Example (TypeScript) ===

--- Creating schedule ---
Created schedule: Nightly Database Backup (id: c90b3bad-a43b-45a0-b535-638c48c9cd7d, cron: 0 0 2 * * *)

--- Listing schedules ---
Schedules: {
  "count": 1,
  "schedules": [
    {
      "created_at": "2026-11-08T16:54:49.363487Z",
      "created_by": {
        "key_id": "3a0cdba0da7c060610042783cc758f1f",
        "key_label": "default_admin"
      },
      "cron_expression": "0 0 2 * * *",
      "description": null,
      "enabled": true,
      "function_label": "schedule_noop_typescript_73568_1794156889329",
      "id": "c90b3bad-a43b-45a0-b535-638c48c9cd7d",
      "last_execution": null,
      "name": "Nightly Database Backup",
      "next_execution": "2026-11-09T02:00:00Z",
      "parameters": {},
      "stats": {
        "avg_execution_time_ms": 0,
        "failed_executions": 0,
        "last_error": null,
        "successful_executions": 0,
        "total_executions": 0
      },
      "timezone": "UTC",
      "updated_at": "2026-11-08T16:54:49.363487Z"
    }
  ]
}

--- Getting schedule ---
Fetched: Nightly Database Backup (cron: 0 0 2 * * *)

--- Updating schedule ---
Updated: Nightly Full Backup (new cron: 0 0 3 * * *)

--- Triggering schedule ---
Trigger response: {
  "schedule_id": "c90b3bad-a43b-45a0-b535-638c48c9cd7d",
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
  3. Score: 13.200, Matched: title, bio
  4. Score: 13.200, Matched: bio, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400, Matched: title, bio
  2. Score: 26.400, Matched: bio, title
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
  1. Score: 0.762, Matched:
  2. Score: 0.755, Matched:
  3. Score: 0.735, Matched:

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.502, Matched: title, content
  2. Score: 0.894, Matched: content, title
  3. Score: 0.305, Matched:

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
  id: 'Gnez-xvJSRYgNgFSp1HtHBZW4gZ3WpfOhp4pixFP8Q1sLpwhE0O6Kn9iD-krj9UWEjpnzlRboBn4Q6ck5DdOnA'
}

=== Find by ID ===
Found: {
  name: { value: 'Test Record', type: 'String' },
  created_at: { value: '2026-11-08T16:54:50.895+00:00', type: 'DateTime' },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  value: { type: 'Integer', value: 42 },
  price: { type: 'Float', value: 99.99 },
  metadata: { value: { key: 'value', nested: [Object] }, type: 'Object' },
  active: { value: true, type: 'Boolean' },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  id: 'Gnez-xvJSRYgNgFSp1HtHBZW4gZ3WpfOhp4pixFP8Q1sLpwhE0O6Kn9iD-krj9UWEjpnzlRboBn4Q6ck5DdOnA',
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-11-08T16:54:50.895Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { key: 'value', nested: { deep: true } }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  name: 'Test Record',
  created_at: '2026-11-08T16:54:50.895+00:00',
  tags: [ 'tag1', 'tag2', 'tag3' ],
  value: 42,
  price: 99.99,
  metadata: { key: 'value', nested: { deep: true } },
  active: true,
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  categories: [ 'electronics', 'computers' ],
  data: 'aGVsbG8gd29ybGQ=',
  id: 'Gnez-xvJSRYgNgFSp1HtHBZW4gZ3WpfOhp4pixFP8Q1sLpwhE0O6Kn9iD-krj9UWEjpnzlRboBn4Q6ck5DdOnA',
  user_id: '550e8400-e29b-41d4-a716-446655440000'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  value: { type: 'Integer', value: 100 },
  metadata: { value: { key: 'value', nested: [Object] }, type: 'Object' },
  active: { value: true, type: 'Boolean' },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  name: { value: 'Updated Record', type: 'String' },
  created_at: { type: 'DateTime', value: '2026-11-08T16:54:50.895+00:00' },
  id: 'Gnez-xvJSRYgNgFSp1HtHBZW4gZ3WpfOhp4pixFP8Q1sLpwhE0O6Kn9iD-krj9UWEjpnzlRboBn4Q6ck5DdOnA',
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  price: { type: 'Float', value: 99.99 },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: F-7UIRdVuNTX9Fe3gGYPHN8x3D1cs61PbVNo1ZSC9uoQfptCoYHsCEJYKk73be36gOFgG97vBGERQHddcdSc1g

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
✓ Created native SWR script: github_user_native_ts (xRjWD_usj0uTtRZsxpcIi54Xv6fv3JHGSm9MY9GvM2OYMmz9imHfW_ArVXz9HkplhEsIj0vgpgCvlTpJkXTDSw)

First call (cache miss - will fetch from GitHub API):
  Response time: 135ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 2ms
  Speedup: 67.5x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_ts (z8qjm8hbo6nmh7hOXy2SOYViqDz-Z_tqCo594npghbIAqTlMpnM1AF2ZPmENInN0qxP4R82JQCEJz5AEBP6x7A)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_ts' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_ts (B5kYMGWb6HKduF1FfXv-757DYrYVthvUvh8I3V8Vga6pD1B_GzeZpJWV6zQ786hg4T-3LPeIuqBEsb_omI6ukA)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_ts' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_ts (YnGvUy9vMuyfURNTaxIR-3lPRCAeK8lJUqzsBNUizehZmVGYpL_CDakSdDpiuZGsjM2ZTbx3MAvoheAatTWDNg)
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
✓ Created SWR script: fetch_api_user_ts_73755_1794156892739 (LpJECKFtczLI6sdxOv9T6rYbBwJMwJnkIqHLfZFySgfuUhSV7le4C0nWYCpv2Q7e4UDKguK-1ObzitwW3ig6bQ)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "value": {
          "address": {
            "street": "Kulas Light",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "city": "Gwenborough",
            "suite": "Apt. 556",
            "zipcode": "92998-3874"
          },
          "name": "Leanne Graham",
          "phone": "1-770-736-8031 x56442",
          "email": "Sincere@april.biz",
          "username": "Bret",
          "id": 1,
          "company": {
            "bs": "harness real-time e-markets",
            "name": "Romaguera-Crona",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
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
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 2ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "value": {
          "address": {
            "street": "Kulas Light",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "city": "Gwenborough",
            "suite": "Apt. 556",
            "zipcode": "92998-3874"
          },
          "name": "Leanne Graham",
          "phone": "1-770-736-8031 x56442",
          "email": "Sincere@april.biz",
          "username": "Bret",
          "id": 1,
          "company": {
            "bs": "harness real-time e-markets",
            "name": "Romaguera-Crona",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
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
✓ Created enrichment script: fetch_product_reviews_ts_73755_1794156892739 (YnTU5eaNtAU8ll82D7l9TKurfta-waYalDoG540YCYlieWZKH1bzKJj9D6yTiBC5lnLnb2dUkAOb8Dv7vxqYHA)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "value": {
          "weight": 4,
          "shippingInformation": "Ships in 3-5 business days",
          "brand": "Essence",
          "title": "Essence Mascara Lash Princess",
          "meta": {
            "createdAt": "2025-10-09T14:47:01.588Z",
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
            "barcode": "5784719087687",
            "updatedAt": "2026-05-23T11:27:41.868Z"
          },
          "tags": [
            "beauty",
            "mascara"
          ],
          "availabilityStatus": "In Stock",
          "price": 9.99,
          "returnPolicy": "No return policy",
          "warrantyInformation": "1 week warranty",
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "category": "beauty",
          "discountPercentage": 10.48,
          "stock": 99,
          "rating": 2.56,
          "minimumOrderQuantity": 48,
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
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerEmail": "lucas.gordon@x.dummyjson.com",
              "comment": "Very satisfied!"
            },
            {
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "reviewerName": "Eleanor Collins",
              "rating": 5,
              "date": "2025-04-30T09:41:02.053Z",
              "comment": "Highly impressed!"
            }
          ],
          "dimensions": {
            "width": 15.14,
            "height": 13.08,
            "depth": 22.99
          },
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "sku": "BEA-ESS-ESS-001",
          "id": 1,
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula."
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
Created Alice: $1000 - ID: GbgC8vEehzy3wzeZxhGUBuDrBo2vzIOoB4jgJ2QTcGMmapS2yd2RhLQFAoglznT5S3399HXBh1EM3xspGI6Iag
Created Bob: $500 - ID: _avzY0Zph8EO-uR9rcyanm7T-2xReQu7RIH0jyFoKL4EA8mQUYAf45onqBGfwYVTNd5A6RvEjeGkKYd5v_WcoA

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 9ea4d75c-00b6-4acd-8edc-37c94502cadc

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
New transaction: 5b873bd6-a379-4a23-834a-3d696728b5d6
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Create User Function ===
Created user function with ID: nuo66esntrr9TGdcxsYm87JrqSNXhLD5AyFr_JS81nbkSf54LcPEXejzE68hRHzo0A60yKjnH3M0t-dQvuwj3A

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 5 user functions:
  - conc_demo_rl_fail_ts_72807_1794156869177: Rate-limit (fail mode)
  - get_active_users_client_ts: Get Active Users (Updated)
  - fetch_product_reviews_ts_73755_1794156892739: Fetch Product with Reviews (Multi-API)
  - conc_demo_rl_skip_ts_72807_1794156869177: Rate-limit (skip mode)
  - get_active_users_ts: Get Active Users

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
=== WebSocket Chat Streaming Example (TypeScript) ===

Created chat session: TWst7du_U3JctA7H6b8kyHmYfhkJPhCa8CjxM90S9SrWruaOhUbVjg1QoJYBIvMMru24PnVM2xYY12BglEDwoA

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"gpt-5.6-luna","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: -X98AKVC5Imq_gfADhaVWwH2cKWMryNXO5ApRBTrcywYWgQLuwvZBiZG9et1vyWJljK_oWuWu3ISfR2TbPcr8w
Execution time: 640ms
Token usage: {"cached_input_tokens":0,"completion_tokens":11,"prompt_tokens":23,"thinking_tokens":0,"total_tokens":34}

Full response: {"__progress":"received"}{"__progress":"generating","model":"gpt-5.6-luna","provider":"openai"}The capital of France is Paris....
=== WebSocket Subscription Example ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_ts' ===
✓ Subscribed (subscription_id: sub_d4915ba652024e568835dabbf8b1ee48)

=== Performing mutations to trigger notifications ===
Inserting a record...
✓ Inserted record: Xs631W0ml8MSEI5hTTjMQnL7k0aA2tEAmodNMGCX95TIUPpbQbpmF1zKVpQwxjf-BZHNZ92gp6y00gP6XK8Ocw
  📡 Notification received for Xs631W0ml8MSEI5hTTjMQnL7k0aA2tEAmodNMGCX95TIUPpbQbpmF1zKVpQwxjf-BZHNZ92gp6y00gP6XK8Ocw

Inserting another record...
✓ Inserted record: yWFLtq9YafJzBgxdOuSF6sNThgnGUvjkKSRZvvA2n-GexokdMnVaJMGqXvIIkkMTWGZUDxKf2mBkvz9ObsIngw
  📡 Notification received for yWFLtq9YafJzBgxdOuSF6sNThgnGUvjkKSRZvvA2n-GexokdMnVaJMGqXvIIkkMTWGZUDxKf2mBkvz9ObsIngw

=== Unsubscribing ===
✓ Unsubscribed: {"collection":"ws_subscribe_example_ts","found":true,"unsubscribed":true}

✓ WebSocket subscription example completed successfully
✓ Deleted collection 'ws_subscribe_example_ts'
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: PuMMrLlYwt2kkR3qTcvF_RzaTh73W_tVRhOqDu9LKJOlazALy0PE5nm49NL8iBSsp6NGMLb9QWDtDbdBW60Z2Q

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
   Inserted with ripple: {"id":"R4qjX_6j5cnl9T73XrIM_KICEJCGISdazjvXMdESZGQCtMCuclXqS2VtzvCoguCZpNB8afXzWhqX2wp9wpXKGQ"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"2kknmXrjZrJntINF5enmAkZNHPNFcGvDwbQwaLhg9hGcI8whEQ6SiD_vTnBgR6N8nLryIhx2Yz5oe-iKqKQrtg"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"price":{"type":"Integer","value":150},"name":{"type":"String","value":"Product 1"},"id":"R4qjX_6j5cnl9T73XrIM_KICEJCGISdazjvXMdESZGQCtMCuclXqS2VtzvCoguCZpNB8afXzWhqX2wp9wpXKGQ"}

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
  First user: Alice Johnson <alice@example.com>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["id","avatar_url","created_at","user_role","age","name","status","email","bio"]

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
    - Fields: ["id","email","user_role","bio","avatar_url","name","age","status","api_key","created_at","secret_token","password"]
  Projected query:
    - 3 fields per record
    - Fields: ["name","email","id"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
✅ All TypeScript integration tests complete!
🧪 Running JavaScript examples (direct HTTP/WebSocket)...

added 1 package, removed 1 package, and audited 9 packages in 457ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
npm warn install-scripts 1 package has install scripts not yet covered by allowScripts:
npm warn install-scripts   ekodb-client-ts@0.32.0 (prepare: npm run build)
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
  id: '7HXLGVIaRA5rYgL9BZMa71WFAOqs9CbZekHQYg0bH8OeIiyj8oUyrpantD3C0ySsvCIR4bEfA9rg3cGaKE-OIg'
}

=== Find by ID ===
Found: {
  value: { type: 'Integer', value: 42 },
  name: { value: 'Test Record', type: 'String' },
  id: '7HXLGVIaRA5rYgL9BZMa71WFAOqs9CbZekHQYg0bH8OeIiyj8oUyrpantD3C0ySsvCIR4bEfA9rg3cGaKE-OIg',
  active: { value: true, type: 'Boolean' }
}

=== Find with Query ===
Found documents: [
  {
    value: { type: 'Integer', value: 42 },
    active: { value: true, type: 'Boolean' },
    name: { value: 'Test Record', type: 'String' },
    id: '7HXLGVIaRA5rYgL9BZMa71WFAOqs9CbZekHQYg0bH8OeIiyj8oUyrpantD3C0ySsvCIR4bEfA9rg3cGaKE-OIg'
  }
]

=== Update Document ===
Updated: {
  name: { type: 'String', value: 'Updated Record' },
  value: { value: 100, type: 'Integer' },
  active: { type: 'Boolean', value: true },
  id: '7HXLGVIaRA5rYgL9BZMa71WFAOqs9CbZekHQYg0bH8OeIiyj8oUyrpantD3C0ySsvCIR4bEfA9rg3cGaKE-OIg'
}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ simple_crud.js completed successfully

=== Running ekoDB/ekodb-client/examples/javascript/simple_websocket.js ===
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: VLIh4AG7OtIYv7zCL_FBvllB89Wh8fRtjPYg6nbwi9CW4ZBRkBOIVP5tbbVi8w74VlV9BefQvx9xEr5LpM90KA

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
        "id": "VLIh4AG7OtIYv7zCL_FBvllB89Wh8fRtjPYg6nbwi9CW4ZBRkBOIVP5tbbVi8w74VlV9BefQvx9xEr5LpM90KA",
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
  "messageId": "1794156899851"
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

✅ Function saved: CJ_vdzBhoRz8d45dCvJIheZZytcyTnskLuf9OWN-0t2Dr8QwUVt-m571HfEStD7XfsKan31VEsnoyOpSxG6m_w
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: 1L5kpJ7S7j3PhDC2x2D5AW69F6fY3zP9fzMX9sCHFdYoevTbdLv_4wHb9YP-r1Y5wgAcQHzWVhp0e52VTXZuOQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: gp1YdHIqutRtcHF_n43nNtL1vNOTOmQlnPR8yxFHyt-mzE2jadtOzFhl6BZddocvHEBR6r872WVZAq7RzR1Mtg
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":{"value":50,"type":"Float"},"count":{"value":5,"type":"Integer"},"status":{"type":"String","value":"inactive"},"max_score":{"type":"Integer","value":90}}
   {"count":{"type":"Integer","value":5},"avg_score":{"value":60,"type":"Float"},"status":{"value":"active","type":"String"},"max_score":{"type":"Integer","value":100}}

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
  type: 'Object',
  value: { name: 'Product 2', price: 39.989999999999995 }
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
✓ kv_operations.js completed successfully

=== Running ekoDB/ekodb-client/examples/javascript/collection_management.js ===
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: A3keD60H-V-gT1w37Nj1NMf0veWMEKGmkTdM-MACUoj_zWnxIwWioQKDje3ics24S5Ed_tfz9CiQu4nFAMRFtA

=== List Collections ===
Total collections: 22
Sample collections: [
  'schema_products_client_go',
  'schema_documents_client_ts',
  'chat_raw_completions__ek0_testing',
  'chat_agent_configs__ek0_testing',
  'schedules__ek0_testing'
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
Created Alice: $1000 - ID: R7vLrpPKPQIVXlX0Gb45zhNJSqDt2NCAbkxb705Y1WQ0_dc2-kgxydqKt36tvsACvkaH0GLSb1SPZbgVKDmN1A
Created Bob: $500 - ID: wrz7T6KGz-FU6OuDPcQXtPYwxQ72zRueJQEuvGjSIw4xxfN2PwP3C94Z3D6L-kqwn_yVyQOrI_D9Q-qn6AGzmA

=== Example 1: Begin Transaction ===
Transaction ID: e57ea19a-934c-49f2-b6e5-4b2b8dc01e06

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
New transaction: b29a50b5-8df0-437f-acc8-afc815078e37
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
   ✅ Function saved: w56Yu1a4hJOAhyvk3PdUXsfo_-a4Rld6726Mu9PasxTGgGElS6CA96V1ZJIb6vYKKjyYYg8oMDKta76_wJW8FA

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
   ✅ Function saved: brHP6cTZU7VtVv0R9TzDiEov1cxSjTXZVr5zzmZBsX3E2qUMZq44wvgCUFYl3Ri1aL4AKaLT_dPI8lqdxztDLw

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
   ✅ Function saved: S9-cGjCWeCjZu1AVDu_BuDsO5nWSechsQMzUO1Q6WH1kcOn9Br8Y3jj0jrkTK5TCSERVTSTr5iHmrhl82jzNng

2️⃣ Calling Function (Query + Update Credits + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"value":100,"type":"Integer"}
   📋 Status: {"value":"active","type":"String"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: JeLSfb01y76VnYZ2GRZPheJ-aRKltLVQDEsQEmFVfCg3vLmKX-Wq9uR-c04GMNRujyaNKIDbPsQciutsCW4tJg

2️⃣ Calling Function (Query + Delete + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted function: w56Yu1a4hJOAhyvk3PdU...
   ✅ Deleted function: brHP6cTZU7VtVv0R9TzD...
   ✅ Deleted function: S9-cGjCWeCjZu1AVDu_B...
   ✅ Deleted function: JeLSfb01y76VnYZ2GRZP...
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
  Output: Document ID = 4TVn8_b9Ba5pnS_EsEESJnR0dp62aJpCNUwsQuXuZtncZBSW0P9s1GmVoMhDVLT1u-SPt-7lXbu5ZU9_9xXz5w
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(4TVn8_b9Ba5pnS_EsEESJnR0dp62aJpCNUwsQuXuZtncZBSW0P9s1GmVoMhDVLT1u-SPt-7lXbu5ZU9_9xXz5w)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(4TVn8_b9Ba5pnS_EsEESJnR0dp62aJpCNUwsQuXuZtncZBSW0P9s1GmVoMhDVLT1u-SPt-7lXbu5ZU9_9xXz5w)
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
  Output: Document ID = PgtizoSMOrxceuLaKVVM462tWZFMDMYDUsh2-H8uMFnCWncegVfGjLEi9-EDyj4YNFfm_wum6-q9tNiPtHDO3Q
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(PgtizoSMOrxceuLaKVVM462tWZFMDMYDUsh2-H8uMFnCWncegVfGjLEi9-EDyj4YNFfm_wum6-q9tNiPtHDO3Q)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(PgtizoSMOrxceuLaKVVM462tWZFMDMYDUsh2-H8uMFnCWncegVfGjLEi9-EDyj4YNFfm_wum6-q9tNiPtHDO3Q)
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

> @ekodb/ekodb-client@0.32.0 prepare
> npm run build


> @ekodb/ekodb-client@0.32.0 build
> tsc


up to date, audited 44 packages in 615ms

12 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
npm warn install-scripts 1 package has install scripts not yet covered by allowScripts:
npm warn install-scripts   fsevents@2.3.3 (install: (install scripts present))
npm warn install-scripts
npm warn install-scripts Run `npm install-scripts ls` to review, or `npm install-scripts approve <pkg>` to allow.

> @ekodb/ekodb-client@0.32.0 build
> tsc

✅ TypeScript client built!

added 1 package, removed 1 package, and audited 9 packages in 442ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
npm warn install-scripts 1 package has install scripts not yet covered by allowScripts:
npm warn install-scripts   ekodb-client-ts@0.32.0 (prepare: npm run build)
npm warn install-scripts
npm warn install-scripts Run `npm install-scripts ls` to review, or `npm install-scripts approve <pkg>` to allow.

added 1 package, removed 1 package, and audited 13 packages in 441ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
npm warn install-scripts 3 packages have install scripts not yet covered by allowScripts:
npm warn install-scripts   ekodb-client-ts@0.32.0 (prepare: npm run build)
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
✓ Created session: yXXGXYQW9PUz8QAMkq9RAUYuJG90NXB9tV-2T2zTbHPnj-eJWv2KOaAW-JI7x3qxdmUhXjT9H-mgL4-3JHUfOw

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is **ekoDB**, a high-performance database product priced at **$99**.

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
✓ Created second session: lUeW_k4iBXLL5j414uVTSrzImDDKxJPpqQs3dLoJeYqX8ygixGol9leVAvpX93P9_HoVyftZXOB5yHXE83L8Zw
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
✓ Created session: rEmxX2vSKzutF0Cno1VZdJDRX8IoY42SQqEjsYrNiRiS1duJoFB_jz0eQEMHRfEPKJqJJIMK6UVFBIwf3D7jhg

=== Sending Chat Message ===
Message ID: XbkEHwl8D6EvDL4i3YOeGAByZRR2CTmsy9Wt3XG4oPpDms2jMdX746H8WB98MiXt7vbj5ougYEoGsskRjfc_HA

=== AI Response ===
- **ekoDB** — $99
- **ekoDB Pro** — $299
- **ekoDB Cloud** — $499

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    description: 'A high-performance database product with AI capabilities',
    id: '1x4O65dguiglYqvPoqumtiuBvah9L99-UOqXi5HzKMR4QNvtYC_rWkIB7pMDIgTr6CEoRsQDkU70AXfI_2dTKg',
    price: 99,
    name: 'ekoDB'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    id: 'DVabTHICOVfS8KdR8R4amVXosxW9WxdRzKmG1_XoGoXH3KyJGXltyMcBvwBNlWATRarRnkk2miE_Nr1zxzm-FA',
    name: 'ekoDB Pro',
    description: 'Enterprise edition product with advanced features',
    price: 299
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB Cloud',
    price: 499,
    description: 'Fully managed cloud database service product',
    id: 'LTeTdD5sKzL03iJqJA0vhsYYjKFcFVcY3ExaM6oICc90yv3BTLCciTgiC-zL75ouq2Itm_7woOcYvNP2ouZAew'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 3760ms

=== Token Usage ===
Prompt tokens: 41732
Completion tokens: 117
Total tokens: 41849

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
✓ Client created

=== List Chat Models ===
Available chat models by provider:
  openai:
    - tts-1
    - gpt-5-mini
    - gpt-5-mini-2025-08-07
    - gpt-5.4-2026-03-05
    - gpt-5-nano
    - gpt-5.4-mini
    - omni-moderation-2024-09-26
    - gpt-live-transcribe
    - gpt-5.6-luna
    - gpt-4o-2024-11-20
    - gpt-5.1
    - whisper-1
    - gpt-5-pro
    - gpt-realtime-whisper
    - omni-moderation-latest
    - gpt-realtime-2
    - gpt-5-search-api
    - gpt-5.5-2026-04-23
    - gpt-5.5-pro
    - gpt-audio-2025-08-28
    - gpt-4o-mini-transcribe-2025-03-20
    - chat-latest
    - gpt-realtime-translate
    - gpt-5.4-pro-2026-03-05
    - gpt-4o-mini-transcribe-2025-12-15
    - gpt-5-pro-2025-10-06
    - gpt-5.6-sol
    - gpt-5.4-mini-2026-03-17
    - text-embedding-3-large
    - gpt-4o-2024-08-06
    - o3
    - gpt-5.4-pro
    - gpt-4o-mini-tts
    - gpt-realtime-mini-2025-12-15
    - gpt-realtime-2.1-mini
    - gpt-realtime-2025-08-28
    - tts-1-hd-1106
    - tts-1-hd
    - gpt-realtime-1.5
    - gpt-realtime
    - gpt-5.2-pro
    - gpt-4o-transcribe-diarize
    - gpt-5.1-2025-11-13
    - gpt-5.4
    - text-embedding-3-small
    - gpt-5-2025-08-07
    - o3-2025-04-16
    - gpt-5.2-2025-12-11
    - chatgpt-image-latest
    - tts-1-1106
    - gpt-5.5-pro-2026-04-23
    - gpt-5.2-pro-2025-12-11
    - gpt-5.4-nano
    - gpt-audio-mini-2025-12-15
    - gpt-audio-mini
    - gpt-4.1-mini-2025-04-14
    - gpt-5
    - gpt-4o-mini-transcribe
    - gpt-image-2-2026-04-21
    - gpt-transcribe
    - gpt-image-2
    - text-embedding-ada-002
    - gpt-realtime-2.1
    - gpt-4.1
    - gpt-5.5
    - gpt-audio-1.5
    - gpt-image-1.5
    - gpt-6.1-sol
    - gpt-6-sol
    - gpt-realtime-mini
    - gpt-image-1-mini
    - gpt-5.6-terra
    - gpt-5-nano-2025-08-07
    - gpt-4o-transcribe
    - gpt-4.1-mini
    - gpt-4o-mini
    - gpt-6.1-terra
    - gpt-6-luna
    - gpt-4o-mini-tts-2025-12-15
    - gpt-audio
    - gpt-5.3-codex
    - gpt-5-search-api-2025-10-14
    - gpt-4o
    - gpt-image-2.6-flare
    - gpt-4o-mini-2024-07-18
    - gpt-4.1-2025-04-14
    - gpt-image-2.5-flare
    - gpt-4o-mini-tts-2025-03-20
    - gpt-5.2
    - gpt-5.4-nano-2026-03-17
    - gpt-6-astra
    - gpt-image-2.5-sunburst-2026-09-08
    - gpt-image-2.5-sunburst
    - gpt-image-2.5-flare-2026-09-08
    - gpt-image-2.6-sunburst
    - gpt-live-1
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
    - gemini-omni-1.2-flash
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
  gemini: ok 61 models
  openai: ok 96 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get Specific Provider Models ===
OpenAI models: tts-1, gpt-5-mini, gpt-5-mini-2025-08-07, gpt-5.4-2026-03-05, gpt-5-nano, gpt-5.4-mini, omni-moderation-2024-09-26, gpt-live-transcribe, gpt-5.6-luna, gpt-4o-2024-11-20, gpt-5.1, whisper-1, gpt-5-pro, gpt-realtime-whisper, omni-moderation-latest, gpt-realtime-2, gpt-5-search-api, gpt-5.5-2026-04-23, gpt-5.5-pro, gpt-audio-2025-08-28, gpt-4o-mini-transcribe-2025-03-20, chat-latest, gpt-realtime-translate, gpt-5.4-pro-2026-03-05, gpt-4o-mini-transcribe-2025-12-15, gpt-5-pro-2025-10-06, gpt-5.6-sol, gpt-5.4-mini-2026-03-17, text-embedding-3-large, gpt-4o-2024-08-06, o3, gpt-5.4-pro, gpt-4o-mini-tts, gpt-realtime-mini-2025-12-15, gpt-realtime-2.1-mini, gpt-realtime-2025-08-28, tts-1-hd-1106, tts-1-hd, gpt-realtime-1.5, gpt-realtime, gpt-5.2-pro, gpt-4o-transcribe-diarize, gpt-5.1-2025-11-13, gpt-5.4, text-embedding-3-small, gpt-5-2025-08-07, o3-2025-04-16, gpt-5.2-2025-12-11, chatgpt-image-latest, tts-1-1106, gpt-5.5-pro-2026-04-23, gpt-5.2-pro-2025-12-11, gpt-5.4-nano, gpt-audio-mini-2025-12-15, gpt-audio-mini, gpt-4.1-mini-2025-04-14, gpt-5, gpt-4o-mini-transcribe, gpt-image-2-2026-04-21, gpt-transcribe, gpt-image-2, text-embedding-ada-002, gpt-realtime-2.1, gpt-4.1, gpt-5.5, gpt-audio-1.5, gpt-image-1.5, gpt-6.1-sol, gpt-6-sol, gpt-realtime-mini, gpt-image-1-mini, gpt-5.6-terra, gpt-5-nano-2025-08-07, gpt-4o-transcribe, gpt-4.1-mini, gpt-4o-mini, gpt-6.1-terra, gpt-6-luna, gpt-4o-mini-tts-2025-12-15, gpt-audio, gpt-5.3-codex, gpt-5-search-api-2025-10-14, gpt-4o, gpt-image-2.6-flare, gpt-4o-mini-2024-07-18, gpt-4.1-2025-04-14, gpt-image-2.5-flare, gpt-4o-mini-tts-2025-03-20, gpt-5.2, gpt-5.4-nano-2026-03-17, gpt-6-astra, gpt-image-2.5-sunburst-2026-09-08, gpt-image-2.5-sunburst, gpt-image-2.5-flare-2026-09-08, gpt-image-2.6-sunburst, gpt-live-1

=== Get Anthropic Models ===
Anthropic models: claude-haiku-5-5, claude-sonnet-5-5, claude-opus-5-5, claude-fable-5-1, claude-opus-5, claude-sonnet-5, claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 404: {"error":"Unknown provider 'nonexistent_provider_xyz'","error_kind":"unknown_provider"}

✓ Chat Models example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: xi-uP9IZ-PQ5AVptI_c7rtll-GxBwYyoWeusYcjExJNSvqFlB_6xAZRBOjVtXRthabQxebx9R2Gn3aGncfBxZQ

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is **ekoDB**, a high-performance database product, priced at **$99**.

✓ Message 2 sent
  Response: ekoDB costs **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: tn2MkWyQjnaioHYDy1UHH5q2C8bAF46ZpvVAJ3W9wBNBd7eMgu15DDVh0FG5MeFy5lYwI3nc0VJq5XcXbToIxw
  Parent: xi-uP9IZ-PQ5AVptI_c7rtll-GxBwYyoWeusYcjExJNSvqFlB_6xAZRBOjVtXRthabQxebx9R2Gn3aGncfBxZQ

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: tn2MkWyQjnaioHYDy1UHH5q2C8bAF46ZpvVAJ3W9wBNBd7eMgu15DDVh0FG5MeFy5lYwI3nc0VJq5XcXbToIxw (Untitled)
  Session 2: xi-uP9IZ-PQ5AVptI_c7rtll-GxBwYyoWeusYcjExJNSvqFlB_6xAZRBOjVtXRthabQxebx9R2Gn3aGncfBxZQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: tn2MkWyQjnaioHYDy1UHH5q2C8bAF46ZpvVAJ3W9wBNBd7eMgu15DDVh0FG5MeFy5lYwI3nc0VJq5XcXbToIxw

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: NYE1HvXJMJDhzizIvnOJuSYWOqbvv94Ngha6tIAmonpAhVBWZ5SKsTEhWX4SY9ruJsnsvlvSfGcNXPCwUBaLkg

=== List Collections ===
Total collections: 23
Sample collections: schema_products_client_go,schema_documents_client_ts,chat_raw_completions__ek0_testing,chat_agent_configs__ek0_testing,schedules__ek0_testing

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: ZTROywyyKat4gFpSV0TDkpclN5NAf6eeGDRz2H0H7pry_c8s2hiWEcZCRcgh1cCLiTHHM5c4nWwUYKahPp7Bhg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: EFpQ-r_A_e7FHJyvBkGnI8PEKcMS0WaxZy2JbUEKa40qRD9d5r5WZv20w3PTd_PCQdUQuTQLoc-uf1IYUVhLqA

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
✓ Edge cache script created: ruBXlgsTROBp7oy4OdpTd9UHL9YjlO3J6rZ0Pjfb7UAItrAh1CD0KXuaj9W4HhyAkPqI_fq4vkeONLa5wk_KOQ

Call 1: Cache miss (fetches from API)
Response time: 151ms
Result: {
  "records": [
    {
      "value": {
        "value": {
          "elevation": 32,
          "timezone": "GMT",
          "timezone_abbreviation": "GMT",
          "longitude": -73.99308,
          "current": {
            "interval": 900,
            "temperature_2m": 14.1,
            "time": "2026-11-08T16:45"
          },
          "utc_offset_seconds": 0,
          "latitude": 40.710335,
          "generationtime_ms": 0.030875205993652344,
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "°C",
            "time": "iso8601"
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

Call 2: Cache hit (served from ekoDB)
Response time: 2ms (75.5x faster!)
Result: {
  "records": [
    {
      "value": {
        "value": {
          "elevation": 32,
          "timezone": "GMT",
          "timezone_abbreviation": "GMT",
          "longitude": -73.99308,
          "current": {
            "interval": 900,
            "temperature_2m": 14.1,
            "time": "2026-11-08T16:45"
          },
          "utc_offset_seconds": 0,
          "latitude": 40.710335,
          "generationtime_ms": 0.030875205993652344,
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "°C",
            "time": "iso8601"
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
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 60ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "address": {
        "street": "Kulas Light",
        "city": "Gwenborough",
        "geo": {
          "lat": "-37.3159",
          "lng": "81.1496"
        },
   ...

Second call (cache hit - from cache):
   ⏱️  Duration: 1ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "address": {
        "street": "Kulas Light",
        "city": "Gwenborough",
        "geo": {
          "lat": "-37.3159",
          "lng": "81.1496"
        },
   ...
   🚀 Cache speedup: 60.0x faster!

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
   2. Laptop Pro - $1299 (⭐4.8)
   3. Bookshelf - $149 (⭐4.1)
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
   Vector databases store and search high-dimensional embeddings, which makes them useful when “similar in meaning” matters more than an exact text or field match.

Key benefits include:

- **Semantic search:** Find relevant text, images, audio, or other content even when queries use different wording.
- **Efficient nearest-neighbor search:** Specialized indexes can retrieve similar vectors quickly across large collections.
- **RAG support:** Retrieve relevant documents to provide context for language models, improving grounded answers.
- **Multimodal retrieval:** Search across embeddings from different data types, depending on the embedding model and database.
- **Metadata filtering and hybrid search:** Combine similarity results with filters or keyword search to improve relevance.
- **Scalable operations:** Many systems support indexing, updates, and distributed storage as collections grow.

They’re most useful when similarity-based retrieval is central; for exact lookups or structured queries, a conventional database may be simpler.
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
🚀 ekoDB JavaScript Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: ZUiR_9PLM-RseOGMd49KqvhrUTNcIg5u9hKfvxS4woVRv2Oq7HvvXzZwnxSOxn_kEOVSojlCJCLALuFYAJaqGw
📊 Found 2 product groups
   {"count":{"value":2,"type":"Integer"},"category":{"type":"String","value":"Furniture"},"avg_price":{"type":"Float","value":474}}
   {"avg_price":{"type":"Float","value":575.6666666666666},"category":{"value":"Electronics","type":"String"},"count":{"type":"Integer","value":3}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"category":{"type":"String","value":"Electronics"},"count":{"value":3,"type":"Integer"}}
   {"count":{"type":"Integer","value":2},"category":{"value":"Furniture","type":"String"}}
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
   1. User 6 - Score: 60
   2. User 10 - Score: 100
   3. User 9 - Score: 90
   4. User 2 - Score: 20
   5. User 8 - Score: 80
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

✅ Inserted order: IUEM1B6Kl1gSUJBxHCSM1HpUUAoSfvN_VCIkOigJ7kyKxqkAH8M2N6p8B_sOsiWM9e4VYhDXmTZHk7jRFG_DTw
✅ Inserted 2 products with wrapped types

📝 Example 2: Function with Wrapped Type Parameters

✅ Function saved: CUUdFP1Eq5BKqDtdHcaTVwVi3IEhpRr3YBKKV55rxpbi-H9qW47OJYcn0dr1EprkDsYIPqcE2O8puzigRoms2A
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"value":{"role":"admin","userId":"user_abc"},"type":"Object"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: y2YP6Xa8mISPXgj6EAuwTF1l4AIxMP3JJXW3Bj_yLHEFU98O2vbbwD8B5Qan-F3w7dNNL961ktfv-mSx-kMNng
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: JPhiqEErxez9BLn_7AxYZ0OjV8m1ojBKC7BfehTFUBW10d-0vBeEYMgApYoYXfzvjOgo_Z_gphRkeUYH_aUSQw
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
   2. Database Design Principles (Database)
   3. Getting Started with ekoDB (Database)
   4. Natural Language Processing (AI)
   5. Vector Databases Explained (Database)
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
   2. Database Design Principles
   3. Getting Started with ekoDB
   4. Natural Language Processing
   5. Vector Databases Explained
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Function saved
📊 Projected documents (showing first 3):
   1. Introduction to Machine Learning
   2. Database Design Principles
   3. Getting Started with ekoDB
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Function saved
📊 All documents (5 total, showing first 2):
   1. Introduction to Machine Learning (AI)
   2. Database Design Principles (Database)
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search function examples completed!
🚀 ekoDB Functions Example (JavaScript Client)

✅ Client initialized (token exchange automatic)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: JXEetr5XEJX41IJePSHXZMbET8o0gY5YM4AxuOhgFwgOBBiORC5XDhVSBVf1YD8zKHGlV-33cAfoDDyHA9FIBA
📊 Found 5 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

✅ Function saved
📊 Found 3 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 2 groups
   {"avg_score":{"type":"Float","value":60},"status":{"type":"String","value":"active"},"count":{"type":"Integer","value":5}}
   {"status":{"type":"String","value":"inactive"},"count":{"type":"Integer","value":5},"avg_score":{"value":50,"type":"Float"}}
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

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: { type: 'Object', value: { username: 'john_doe', userId: 123 } }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { type: 'Object', value: { name: 'Product 1', price: 29.99 } }
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
  1. Score: 13.200, Matched: bio, title
  2. Score: 13.200, Matched: bio, title
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
Found 1 results (matches: work, working, worked)
  1. Score: 6.600, Matched: bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.744, Matched:
  2. Score: 0.736, Matched:
  3. Score: 0.726, Matched:

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.497, Matched: content, title
  2. Score: 0.891, Matched: title, content
  3. Score: 0.294, Matched:

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: bio, skills

=== Cleanup ===
✅ Deleted test collections

✅ Search examples completed!
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'dwnhACa-rVk4Jr7B9vJO1rujJj6586ePZB80J5tVs2sOg4poRrNpQUqqfU29U0grJr8Qj-6ol6KLMP4kbMsrCw'
}

=== Find by ID ===
Found: {
  id: 'dwnhACa-rVk4Jr7B9vJO1rujJj6586ePZB80J5tVs2sOg4poRrNpQUqqfU29U0grJr8Qj-6ol6KLMP4kbMsrCw',
  name: { type: 'String', value: 'Test Record' },
  active: { value: true, type: 'Boolean' },
  value: { value: 42, type: 'Integer' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  name: { type: 'String', value: 'Updated Record' },
  active: { value: true, type: 'Boolean' },
  value: { type: 'Integer', value: 100 },
  id: 'dwnhACa-rVk4Jr7B9vJO1rujJj6586ePZB80J5tVs2sOg4poRrNpQUqqfU29U0grJr8Qj-6ol6KLMP4kbMsrCw'
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: Tlt8kSs_OIUnUUVddObdPm91SgldzmOOJDLWw4nvGLVqyRvd5ytrE5eJEE7JC2SmZiRpV-0lI-gsmiNY9XJdAA

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
✓ Created native SWR script: github_user_native_js (BMRrFi4W1Cy-3SQt9Z53h3vFfiEhfRcIOEMA3AwYlFziA0W7R2uGbs2twr41njwHmP9GxBY-Bsc_gezHxZ0hLA)

First call (cache miss - will fetch from GitHub API):
  Response time: 128ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 2ms
  Speedup: 64.0x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_js (9bDBP2LkYupDOBH1N8vuJ2ErVmDbXRcY-qCa1G6zjfBpm1vTkb3pD1iymFDuDGOS4U9yDJQyxDwPfEJxOIU-JA)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_js' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_js (HgRYwttZX2_HXV5h7XHs88hZT-Pc3jhqqPXUe2hNN1YcBNGSYQPc7SD8Ud8jPkMi0XM0G_wObpBAunt0Ua3Erg)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_js' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_js (NAtvhQKXanEak6JJOvh73MIjTRg48doD_V4ed4Q4FjDhtuRJNKOWo7qFzb0tXxw26GLJW75FenVCU0mSk7GjVA)
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
✓ Created SWR script: fetch_api_user_js_75830_1794156971445 (vlV3EckrZcS8Z4HSBb1AWyt4SjqUisl8qq-vrnE2n7esXqrLhSm3rnJub4Cjzg0zQ0UeKMy2IFI73L6PjMuuZA)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "username": "Bret",
          "phone": "1-770-736-8031 x56442",
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "id": 1,
          "website": "hildegard.org",
          "name": "Leanne Graham",
          "address": {
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "street": "Kulas Light",
            "city": "Gwenborough",
            "suite": "Apt. 556",
            "zipcode": "92998-3874"
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
Created Alice: $1000 - ID: Xrgl4S8tBaHP7LqsaJUGrYCmMyfJ42-aG0Y2WvWyj2Fzqrl-qZYSAlpDD0CbkdiiPYvLXN0Oi8x-WHbDMxob8g
Created Bob: $500 - ID: gyNr8dYC50cehJTI1QnJVJBSnJg3ot9yShCF73Nfwq-KzDIpCmpJ6HwxY-swfEFYIIDz-8WC6hMnHg8Uogb9_Q

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 076daf08-be15-4c79-a0a5-aff1da6f1593

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
New transaction: 419e30d4-4b48-4279-8415-64572d42d923
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: igRKCgVyDlxgOPRa-aryEvq3Jf60l-q1a8InKkN5oEHtSSFV9B77-V6iT3R2Vscx_qE5a1hfGOTf6jLK2VWDbg

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
Inserted: HJpkvL-0F4OL_ZODK1KSfunKBCyhr-_SaJjNoiz2CfMv4bOdaOw9ul1oYWLzHdXYqlxCMmy-qBORrgp7nBPp7g

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
✓ Created session: UWngJK028EGWraZLGSftwLDsP2zL7SYHrjn_Gmy4iR_UOhsJCpau54dfKMtkc07-XYIRdkUEGVDF9ViaYK35YA

=== Sending Initial Message ===
✓ Message sent
  Response: The available product listed is **ekoDB**, described as a high-performance database product, priced at **$99**.

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
✓ Created second session: oPGr_QJtuIow8MVvR0R1Fg7tk9aSoGFQh1feI35Ps7SiUSNcCgrezKC-XIJrqT9upGR2QXLBPkfq3dFvYzGZMw
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: oPGr_QJtuIow8MVvR0R1Fg7tk9aSoGFQh1feI35Ps7SiUSNcCgrezKC-XIJrqT9upGR2QXLBPkfq3dFvYzGZMw
✓ Deleted chat session: UWngJK028EGWraZLGSftwLDsP2zL7SYHrjn_Gmy4iR_UOhsJCpau54dfKMtkc07-XYIRdkUEGVDF9ViaYK35YA
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: 13IdId8HKSLdpc4-JF99Jl9DJS92hQ4IIT0Oo1t7_u9T7Apt5W0-kxvMF1gIRu8pBmYkXooPd5BQFAOzglkgqw

=== Sending Chat Message ===
Message ID: lRboxAgvoeTSBAuX40xxte60lgfAbUjf52w9z5htAcYCAxG3yFM5tLA8g_IVxR3x6yNrTGYz2I43zz1Ovg932g

=== AI Response ===
- **ekoDB** — 99
- **ekoDB Pro** — 299
- **ekoDB Cloud** — 499

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'C1fy9c3WEr3CvqUDGMeknVSFn4OZKLX7ACPuyATX9raWR3e-4Kdt83Cv-Oh8woiiGzV9vzxlnhX2g3WV0fhLnw',
    price: 99,
    name: 'ekoDB',
    description: 'A high-performance database product with AI capabilities'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Enterprise edition product with advanced features',
    price: 299,
    name: 'ekoDB Pro',
    id: 'qWBTpUJpl9xRZmmfGnU2qh0G8hd_1aPM9En-_nmcMToRzPMnCNQsdNXJ-rye6A2uzKE13uCz_CL3pLwqlgJ6sQ'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 499,
    name: 'ekoDB Cloud',
    id: 'rga7z6D-lGPB_M6WpiVKdqjdIcixu6kQ1asQoq_pH_A5LE64zdc2tL3YTm0pXwhJmYfdiNJMoWT_fJVj6Ve94g',
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 1976ms

=== Token Usage ===
Prompt tokens: 13718
Completion tokens: 64
Total tokens: 13782

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Message Stream (SSE) Example (TypeScript) ===

Created session: Gxr5IlHlncmrbPlqlaetLy54_fecxluPczj8hBgpnAnowo2vgEszBjrEvuboyfwjQL-v0FVBxO_sjQVQjVuTuw

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"gpt-6-luna","provider":"openai"}**EkoDB** usually refers to a decentralized database platform: instead of keeping application data on one centrally managed server, it aims to distribute storage across a network. It’s often presented for Web3 applications and data-ownership use cases.

The name may refer to different projects, so if you share a link or where you saw it, I can identify the specific one.

--- Stream complete ---
Message ID: UrgRksUkZL96S2KtIr29Ft8MJ3zeH4Qr0bU2blFc7Vug6pFFsMMv-4ziUXu8k2gu6_wp8crrLINYY8WFKhmuhg
Execution time: 6438ms
Context window: 128000 tokens

✓ Chat message stream example completed
✓ Client created

=== List Chat Models ===
Available chat models by provider:
  openai:
    - tts-1
    - gpt-5-mini
    - gpt-5-mini-2025-08-07
    - gpt-5.4-2026-03-05
    - gpt-5-nano
    - gpt-5.4-mini
    - omni-moderation-2024-09-26
    - gpt-live-transcribe
    - gpt-5.6-luna
    - gpt-4o-2024-11-20
    - gpt-5.1
    - whisper-1
    - gpt-5-pro
    - gpt-realtime-whisper
    - omni-moderation-latest
    - gpt-realtime-2
    - gpt-5-search-api
    - gpt-5.5-2026-04-23
    - gpt-5.5-pro
    - gpt-audio-2025-08-28
    - gpt-4o-mini-transcribe-2025-03-20
    - chat-latest
    - gpt-realtime-translate
    - gpt-5.4-pro-2026-03-05
    - gpt-4o-mini-transcribe-2025-12-15
    - gpt-5-pro-2025-10-06
    - gpt-5.6-sol
    - gpt-5.4-mini-2026-03-17
    - text-embedding-3-large
    - gpt-4o-2024-08-06
    - o3
    - gpt-5.4-pro
    - gpt-4o-mini-tts
    - gpt-realtime-mini-2025-12-15
    - gpt-realtime-2.1-mini
    - gpt-realtime-2025-08-28
    - tts-1-hd-1106
    - tts-1-hd
    - gpt-realtime-1.5
    - gpt-realtime
    - gpt-5.2-pro
    - gpt-4o-transcribe-diarize
    - gpt-5.1-2025-11-13
    - gpt-5.4
    - text-embedding-3-small
    - gpt-5-2025-08-07
    - o3-2025-04-16
    - gpt-5.2-2025-12-11
    - chatgpt-image-latest
    - tts-1-1106
    - gpt-5.5-pro-2026-04-23
    - gpt-5.2-pro-2025-12-11
    - gpt-5.4-nano
    - gpt-audio-mini-2025-12-15
    - gpt-audio-mini
    - gpt-4.1-mini-2025-04-14
    - gpt-5
    - gpt-4o-mini-transcribe
    - gpt-image-2-2026-04-21
    - gpt-transcribe
    - gpt-image-2
    - text-embedding-ada-002
    - gpt-realtime-2.1
    - gpt-4.1
    - gpt-5.5
    - gpt-audio-1.5
    - gpt-image-1.5
    - gpt-6.1-sol
    - gpt-6-sol
    - gpt-realtime-mini
    - gpt-image-1-mini
    - gpt-5.6-terra
    - gpt-5-nano-2025-08-07
    - gpt-4o-transcribe
    - gpt-4.1-mini
    - gpt-4o-mini
    - gpt-6.1-terra
    - gpt-6-luna
    - gpt-4o-mini-tts-2025-12-15
    - gpt-audio
    - gpt-5.3-codex
    - gpt-5-search-api-2025-10-14
    - gpt-4o
    - gpt-image-2.6-flare
    - gpt-4o-mini-2024-07-18
    - gpt-4.1-2025-04-14
    - gpt-image-2.5-flare
    - gpt-4o-mini-tts-2025-03-20
    - gpt-5.2
    - gpt-5.4-nano-2026-03-17
    - gpt-6-astra
    - gpt-image-2.5-sunburst-2026-09-08
    - gpt-image-2.5-sunburst
    - gpt-image-2.5-flare-2026-09-08
    - gpt-image-2.6-sunburst
    - gpt-live-1
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
    - gemini-omni-1.2-flash
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
  gemini: ok 61 models
  openai: ok 96 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get Specific Provider Models ===
OpenAI models: tts-1, gpt-5-mini, gpt-5-mini-2025-08-07, gpt-5.4-2026-03-05, gpt-5-nano, gpt-5.4-mini, omni-moderation-2024-09-26, gpt-live-transcribe, gpt-5.6-luna, gpt-4o-2024-11-20, gpt-5.1, whisper-1, gpt-5-pro, gpt-realtime-whisper, omni-moderation-latest, gpt-realtime-2, gpt-5-search-api, gpt-5.5-2026-04-23, gpt-5.5-pro, gpt-audio-2025-08-28, gpt-4o-mini-transcribe-2025-03-20, chat-latest, gpt-realtime-translate, gpt-5.4-pro-2026-03-05, gpt-4o-mini-transcribe-2025-12-15, gpt-5-pro-2025-10-06, gpt-5.6-sol, gpt-5.4-mini-2026-03-17, text-embedding-3-large, gpt-4o-2024-08-06, o3, gpt-5.4-pro, gpt-4o-mini-tts, gpt-realtime-mini-2025-12-15, gpt-realtime-2.1-mini, gpt-realtime-2025-08-28, tts-1-hd-1106, tts-1-hd, gpt-realtime-1.5, gpt-realtime, gpt-5.2-pro, gpt-4o-transcribe-diarize, gpt-5.1-2025-11-13, gpt-5.4, text-embedding-3-small, gpt-5-2025-08-07, o3-2025-04-16, gpt-5.2-2025-12-11, chatgpt-image-latest, tts-1-1106, gpt-5.5-pro-2026-04-23, gpt-5.2-pro-2025-12-11, gpt-5.4-nano, gpt-audio-mini-2025-12-15, gpt-audio-mini, gpt-4.1-mini-2025-04-14, gpt-5, gpt-4o-mini-transcribe, gpt-image-2-2026-04-21, gpt-transcribe, gpt-image-2, text-embedding-ada-002, gpt-realtime-2.1, gpt-4.1, gpt-5.5, gpt-audio-1.5, gpt-image-1.5, gpt-6.1-sol, gpt-6-sol, gpt-realtime-mini, gpt-image-1-mini, gpt-5.6-terra, gpt-5-nano-2025-08-07, gpt-4o-transcribe, gpt-4.1-mini, gpt-4o-mini, gpt-6.1-terra, gpt-6-luna, gpt-4o-mini-tts-2025-12-15, gpt-audio, gpt-5.3-codex, gpt-5-search-api-2025-10-14, gpt-4o, gpt-image-2.6-flare, gpt-4o-mini-2024-07-18, gpt-4.1-2025-04-14, gpt-image-2.5-flare, gpt-4o-mini-tts-2025-03-20, gpt-5.2, gpt-5.4-nano-2026-03-17, gpt-6-astra, gpt-image-2.5-sunburst-2026-09-08, gpt-image-2.5-sunburst, gpt-image-2.5-flare-2026-09-08, gpt-image-2.6-sunburst, gpt-live-1

=== Get Anthropic Models ===
Anthropic models: claude-haiku-5-5, claude-sonnet-5-5, claude-opus-5-5, claude-fable-5-1, claude-opus-5, claude-sonnet-5, claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 404: {"error":"Unknown provider 'nonexistent_provider_xyz'","error_kind":"unknown_provider"}

✓ Chat Models example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 1vKB38VkkJtWBjE0EIjhzYMIxxgJvd3sb3oHKBeTtBjtSR-1qk9RnGkNH1f5JGDuFPpLoEKoNnJheMOhI3SLyw

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is **ekoDB**, a high-performance database product, priced at **99**.

✓ Message 2 sent
  Response: ekoDB is priced at **99**. The currency isn’t specified.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: obOWxwYrzyRmDdwtsDhIoT0YNtvt2EVexVTB-g9G8AisoX-CvW45PoHIFzvdXRctkZ75bPf0J36yHhCNIXVQcQ
  Parent: 1vKB38VkkJtWBjE0EIjhzYMIxxgJvd3sb3oHKBeTtBjtSR-1qk9RnGkNH1f5JGDuFPpLoEKoNnJheMOhI3SLyw

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: obOWxwYrzyRmDdwtsDhIoT0YNtvt2EVexVTB-g9G8AisoX-CvW45PoHIFzvdXRctkZ75bPf0J36yHhCNIXVQcQ (Untitled)
  Session 2: 1vKB38VkkJtWBjE0EIjhzYMIxxgJvd3sb3oHKBeTtBjtSR-1qk9RnGkNH1f5JGDuFPpLoEKoNnJheMOhI3SLyw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: obOWxwYrzyRmDdwtsDhIoT0YNtvt2EVexVTB-g9G8AisoX-CvW45PoHIFzvdXRctkZ75bPf0J36yHhCNIXVQcQ

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: lHxVzFZ3XfsVB1iQzlRUAKxJ3rGlj394IoU6UC4QfsFYZB7sDXKDG2FC-S4XMFOl46vWGTKXSCrejiybW2Aqvg

=== List Collections ===
Total collections: 27
Sample collections: schema_products_client_go,schema_documents_client_ts,chat_raw_completions__ek0_testing,chat_agent_configs__ek0_testing,schedules__ek0_testing

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
  POST /api/functions/conc_demo_pay_ts_75979_1794156993426 { "idempotency_key": "...", "amount": 100 }
  POST /api/functions/conc_demo_rl_fail_ts_75979_1794156993426 { "user_id": 42 }
  POST /api/functions/conc_demo_rl_skip_ts_75979_1794156993426 { "user_id": 42 }
  POST /api/functions/conc_demo_lock_ts_75979_1794156993426 { "resource": "queue:drain" }

✓ Cleaned up demo functions
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'mvRUM_qhZE4YCrbrFqgk19SbyJoVYOG2Y9eYitJtTn1zc3khGCFuI5TJNgIk6jZ0tAjG_EOmnsVYLov046lZyw'
}

=== Upsert Operation ===
✓ First upsert (update): {
  id: 'mvRUM_qhZE4YCrbrFqgk19SbyJoVYOG2Y9eYitJtTn1zc3khGCFuI5TJNgIk6jZ0tAjG_EOmnsVYLov046lZyw',
  active: { type: 'Boolean', value: true },
  name: { type: 'String', value: 'Alice Johnson' },
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  age: { type: 'Integer', value: 29 }
}
✓ Second upsert (insert): { id: 'new-user-id' }

=== Find One Operation ===
✓ Found user by email: {
  name: { type: 'String', value: 'Alice Johnson' },
  email: { value: 'alice.j@newdomain.com', type: 'String' },
  active: { type: 'Boolean', value: true },
  age: { type: 'Integer', value: 29 },
  id: 'mvRUM_qhZE4YCrbrFqgk19SbyJoVYOG2Y9eYitJtTn1zc3khGCFuI5TJNgIk6jZ0tAjG_EOmnsVYLov046lZyw'
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
✓ Inserted document: E-Y-XkOVEWkQ2bUuGQTZxJJqQRw3CGvbT_vVI3fuzvvWuFmlgyRBUyMeRBdG2rZ87beBleXYHlHXhUzJU4apqA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Qaoly7BJhd71kTOScRq9UAL-NmBMDvTx3UTCrLabVA5bX4CTWfDBvQaIeHpi5AudIwujV4NxIfoQ83jhBoZhjA

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
✓ Edge cache script created: _pjD8gRK8zq4LVd0X-NK5BMCusWzyYuhlVj6dC-XpNgtHgBWOl2vVtDoVd-ZvPRet4hBZ3RWNNH8fQ82eKujYA

Call 1: Cache miss (fetches from API)
Response time: 54ms
Result: {
  "records": [
    {
      "value": {
        "value": {
          "id": 1,
          "email": "Sincere@april.biz",
          "phone": "1-770-736-8031 x56442",
          "website": "hildegard.org",
          "username": "Bret",
          "address": {
            "zipcode": "92998-3874",
            "suite": "Apt. 556",
            "city": "Gwenborough",
            "street": "Kulas Light",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            }
          },
          "name": "Leanne Graham",
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
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

Call 2: Cache hit (served from ekoDB)
Response time: 2ms (27x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "id": 1,
          "email": "Sincere@april.biz",
          "phone": "1-770-736-8031 x56442",
          "website": "hildegard.org",
          "username": "Bret",
          "address": {
            "zipcode": "92998-3874",
            "suite": "Apt. 556",
            "city": "Gwenborough",
            "street": "Kulas Light",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            }
          },
          "name": "Leanne Graham",
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
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
   Name: {"type":"String","value":"User 1"}
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
    "type": "Object",
    "value": {
      "phone": "1-770-736-8031 x56442",
      "username": "Bret",
      "company": {
        "name": "Romaguera-Crona",
        "bs": "harness real-...

Second call (cache hit - from cache):
   ⏱️  Duration: 1ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "phone": "1-770-736-8031 x56442",
      "username": "Bret",
      "company": {
        "name": "Romaguera-Crona",
        "bs": "harness real-time e-markets",
     ...
   🚀 Cache speedup: 55.0x faster!

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
   {"count":{"value":5,"type":"Integer"},"category":{"type":"String","value":"Electronics"},"avg_price":{"type":"Float","value":367}}
   {"category":{"type":"String","value":"Furniture"},"count":{"type":"Integer","value":3},"avg_price":{"value":365.6666666666667,"type":"Float"}}
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
   Vector databases are designed to store and search **vector embeddings**—numerical representations of text, images, audio, or other data. Their main benefits include:

- **Semantic search:** Find items by meaning or similarity, not just exact keywords.
- **Fast similarity lookup at scale:** Specialized indexes can search large collections of vectors efficiently.
- **Richer retrieval:** Combine vector similarity with metadata filters, such as date, category, or access permissions.
- **AI application support:** They’re commonly used for retrieval-augmented generation (RAG), recommendations, image search, and deduplication.
- **Operational features:** Many provide tools for updating data, managing indexes, and scaling storage and search.

They’re most useful when similarity matters; for exact-match or simple structured queries, a traditional database may be a better fit.
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

✅ Function saved: mB8DhI8H21WfW_s1WFA79deo7FQpV7g_q_PbbLWjtj7AS09OGU91jF0uJIVpZKlWl1z4dCxbVI3k4hONPZ7d1w
📊 Found 2 product groups
   {"avg_price":{"value":575.6666666666666,"type":"Float"},"category":{"value":"Electronics","type":"String"},"count":{"value":3,"type":"Integer"}}
   {"avg_price":{"type":"Float","value":474},"category":{"value":"Furniture","type":"String"},"count":{"type":"Integer","value":2}}
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
   {"role":{"type":"String","value":"admin"},"avg_score":{"type":"Float","value":20},"count":{"value":3,"type":"Integer"}}
   {"count":{"type":"Integer","value":7},"avg_score":{"type":"Float","value":70},"role":{"type":"String","value":"user"}}
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

✅ Inserted order: vuVloonmM1RSRn4ROtMD9QQWAiszEA13iXpP7OEw5JVhei0OnkS3j9Ycp-VBdL14tABs1OgtQV3vpMN9BPsuIw
✅ Inserted 2 products with wrapped types

📝 Example 2: UserFunction with Wrapped Type Parameters

✅ Function saved: uYgcfi6nIANJzhofgJPJsI0MEuI7dswPclOhuBRH0BPY19687_o6z0L-Yt0XW8FC9EyRshOVv8wmydoi-JZwWA
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"type":"Object","value":{"role":"admin","userId":"user_abc"}}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: _120e6CRxMsNaCzF6ryTCL7NSLg2rQ_1m4Haa5lfSerAqkCfrd0jM7_882SbTT_VCPQvIzj5cb4I_-mU9DnEHw
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: 5dS0UEhs5k4bOU-fQflL9n04emmNpExtpozcFaHURqCNd2Fi-EM_dD4jtzpQakgtIE5Weasae9u0Sawx1ny32g
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
   1. Natural Language Processing (AI)
   2. Introduction to Machine Learning (AI)
   3. Database Design Principles (Database)
   4. Getting Started with ekoDB (Database)
   5. Vector Databases Explained (Database)
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
🚀 ekoDB Functions Example (TypeScript)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

ℹ️  Function 'get_active_users_client_ts' already existed — updated instead
✅ Function saved: 1pdngGQZ-kjO_tpMvWyBJCM1Hkx1wcWStb4bLZRtEjmsiZige2_JrxbjQcfjd340sUpnwRX9hyilREBGb13pVQ
📊 Found 5 active users

📝 Example 2: Parameterized Function

✅ Function saved: pZz6TtBdnqugn_2jTX_Zofn_s3N6J5bKZd_7Ap2VHmYMiPDgH4b1mw4GtaK1C4E8ky2Wfw1Cv_tpcaG-ORdi4Q
📊 Found 3 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: ShhEm8NoxaDrEdSDqZBdiZ74WGxtMpTI6aBnX2K8fDCNhO_mNweIjAVK4ZihzFJZ076a7RfzwVEYj6_QGmMcXA
📊 Statistics: 2 groups
   {"status":{"type":"String","value":"active"},"count":{"value":5,"type":"Integer"},"avg_score":{"value":60,"type":"Float"}}
   {"count":{"type":"Integer","value":5},"avg_score":{"type":"Float","value":50},"status":{"type":"String","value":"inactive"}}

📝 Example 4: UserFunction Management

📋 Total scripts: 9
🔍 Retrieved script: Get Active Users
✏️  function updated
🗑️  function deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed!
=== ekoDB Goal Template CRUD Example (TypeScript) ===

--- Creating goal template ---
Created template: Data Migration (id: 3eIW1oE54EkbuzxN7akoZzIockhcqYGcf6Ot8KXVp__ZPwaAJal4fY4BkReaJC5zCs1lZQYAmAFFlbb-1H-IMw)

--- Listing templates ---
Templates: {
  count: 1,
  items: [
    {
      description: [Object],
      id: '3eIW1oE54EkbuzxN7akoZzIockhcqYGcf6Ot8KXVp__ZPwaAJal4fY4BkReaJC5zCs1lZQYAmAFFlbb-1H-IMw',
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
Created goal: Deploy v2.0 (id: LQa6LqB0gQEpLZ0-4CkjnYQ6lr7vEb4P1tk9JqWxEgiZ4Ou1NxW7ngzSK0PFSVL3X1cdUtJnwDTbpNmbYrNXfQ)

--- Listing goals ---
Goals: {
  "count": 1,
  "goals": [
    {
      "created_at": {
        "type": "DateTime",
        "value": "2026-11-08T16:56:39.441132+00:00"
      },
      "description": {
        "type": "String",
        "value": "Ship version 2.0 to production"
      },
      "id": "LQa6LqB0gQEpLZ0-4CkjnYQ6lr7vEb4P1tk9JqWxEgiZ4Ou1NxW7ngzSK0PFSVL3X1cdUtJnwDTbpNmbYrNXfQ",
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
        "value": "2026-11-08T16:56:39.441132+00:00"
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
        "value": "2026-11-08T16:56:39.441132+00:00"
      },
      "description": {
        "type": "String",
        "value": "Ship version 2.0 with hot-fix patches"
      },
      "id": "LQa6LqB0gQEpLZ0-4CkjnYQ6lr7vEb4P1tk9JqWxEgiZ4Ou1NxW7ngzSK0PFSVL3X1cdUtJnwDTbpNmbYrNXfQ",
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
        "value": "2026-11-08T16:56:39.448694+00:00"
      }
    }
  ]
}

--- Goal step: start step 0 ---
Step 0 started on goal LQa6LqB0gQEpLZ0-4CkjnYQ6lr7vEb4P1tk9JqWxEgiZ4Ou1NxW7ngzSK0PFSVL3X1cdUtJnwDTbpNmbYrNXfQ
--- Goal step: complete step 0 ---
Step 0 completed on goal LQa6LqB0gQEpLZ0-4CkjnYQ6lr7vEb4P1tk9JqWxEgiZ4Ou1NxW7ngzSK0PFSVL3X1cdUtJnwDTbpNmbYrNXfQ
--- Goal step: fail step 1 ---
Step 1 failed on goal LQa6LqB0gQEpLZ0-4CkjnYQ6lr7vEb4P1tk9JqWxEgiZ4Ou1NxW7ngzSK0PFSVL3X1cdUtJnwDTbpNmbYrNXfQ

--- Completing goal ---
Goal status: pending_review
--- Approving goal ---
Goal status after approve: in_progress

--- Creating goal to reject ---
--- Rejecting goal ---
Goal status after reject: failed


--- Creating task ---
Created task: Hourly Health Check (id: -5-T84AZ1pu2vOuKA6A4E79ooS4H1-cdpn4oOHO4uaKqkPZ14hGPwJ0PAOxMSfb14zFnVz8W9BD4UczrMQGKQQ)

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
      "id": "-5-T84AZ1pu2vOuKA6A4E79ooS4H1-cdpn4oOHO4uaKqkPZ14hGPwJ0PAOxMSfb14zFnVz8W9BD4UczrMQGKQQ",
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
Created agent: SupportBot (id: BWrGPDRocUdSBqNqWqzVNWJBOZZH7LjpJr6zANtc077zIibFC9VhGyas6WbB8FzGkQpuTye4SxvZ_p2Wyrmfig)

--- Listing agents ---
Agents: {
  "count": 1,
  "items": [
    {
      "deployment_id": {
        "type": "String",
        "value": "deploy_prod_1"
      },
      "id": "BWrGPDRocUdSBqNqWqzVNWJBOZZH7LjpJr6zANtc077zIibFC9VhGyas6WbB8FzGkQpuTye4SxvZ_p2Wyrmfig",
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
By name: SupportBot (id: BWrGPDRocUdSBqNqWqzVNWJBOZZH7LjpJr6zANtc077zIibFC9VhGyas6WbB8FzGkQpuTye4SxvZ_p2Wyrmfig)

--- Updating agent ---
Updated agent: SupportBot

--- Getting agents by deployment ---
Agents in deployment: {
  "count": 0,
  "items": []
}
WARNING: agents-by-deployment omitted created agent BWrGPDRocUdSBqNqWqzVNWJBOZZH7LjpJr6zANtc077zIibFC9VhGyas6WbB8FzGkQpuTye4SxvZ_p2Wyrmfig; TODO: check/fix the server-side deployment lookup

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
  POST /api/functions/jwt_register_ts_76042_1794156999982 { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/jwt_login_ts_76042_1794156999982 { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/jwt_verify_ts_76042_1794156999982 { "token": "<jwt>" }

Set JWT_SECRET in ekoDB's environment_vars whitelist before invoking.

✓ Cleaned up demo functions
=== ekoDB KV Document Linking Example (TypeScript) ===

--- Setup: creating KV key and documents ---
Set KV key: session:admin
Inserted document 1: U6aXHFp9rSUT4SFFeeJtF77SauoWzk1Z_P25JfNNlk6kXRFI7Z-_Mn20k84fbbBPghM3B-1qLav9HfukFuKQEw
Inserted document 2: lE2cGG8nbRmBcjU9sVDuS2tVS-jYfVXiWeuMykGLe3DccloBNvdUkiZZFXvjtwcmaqFWJMdTI8QeR2wERsztDQ

--- Linking documents to KV key ---
Linked doc U6aXHFp9rSUT4SFFeeJtF77SauoWzk1Z_P25JfNNlk6kXRFI7Z-_Mn20k84fbbBPghM3B-1qLav9HfukFuKQEw: null
Linked doc lE2cGG8nbRmBcjU9sVDuS2tVS-jYfVXiWeuMykGLe3DccloBNvdUkiZZFXvjtwcmaqFWJMdTI8QeR2wERsztDQ: null

--- Getting links for KV key ---
Links: [
  {
    "collection": "kv_links_example_ts_76044_1794157000065",
    "document_id": "U6aXHFp9rSUT4SFFeeJtF77SauoWzk1Z_P25JfNNlk6kXRFI7Z-_Mn20k84fbbBPghM3B-1qLav9HfukFuKQEw",
    "field_path": null,
    "created_at": "2026-11-08T16:56:40.136893Z",
    "last_accessed": "2026-11-08T16:56:40.138463Z",
    "metadata": {}
  },
  {
    "collection": "kv_links_example_ts_76044_1794157000065",
    "document_id": "lE2cGG8nbRmBcjU9sVDuS2tVS-jYfVXiWeuMykGLe3DccloBNvdUkiZZFXvjtwcmaqFWJMdTI8QeR2wERsztDQ",
    "field_path": null,
    "created_at": "2026-11-08T16:56:40.137835Z",
    "last_accessed": "2026-11-08T16:56:40.138463Z",
    "metadata": {}
  }
]

--- Unlinking document ---
Unlinked doc lE2cGG8nbRmBcjU9sVDuS2tVS-jYfVXiWeuMykGLe3DccloBNvdUkiZZFXvjtwcmaqFWJMdTI8QeR2wERsztDQ: null

--- Verifying remaining links ---
Remaining links: [
  {
    "collection": "kv_links_example_ts_76044_1794157000065",
    "document_id": "U6aXHFp9rSUT4SFFeeJtF77SauoWzk1Z_P25JfNNlk6kXRFI7Z-_Mn20k84fbbBPghM3B-1qLav9HfukFuKQEw",
    "field_path": null,
    "created_at": "2026-11-08T16:56:40.136893Z",
    "last_accessed": "2026-11-08T16:56:40.139569Z",
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
Retrieved value: { type: 'Object', value: { username: 'john_doe', userId: 123 } }

=== KV Batch Set ===
✓ Batch set 3 keys
  kv_ops_ts_76046_1794157000231:cache:product:1: success
  kv_ops_ts_76046_1794157000231:cache:product:2: success
  kv_ops_ts_76046_1794157000231:cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  kv_ops_ts_76046_1794157000231:cache:product:1: { name: 'Product 1', price: 29.99 }
  kv_ops_ts_76046_1794157000231:cache:product:2: { price: 39.99, name: 'Product 2' }
  kv_ops_ts_76046_1794157000231:cache:product:3: { price: 49.99, name: 'Product 3' }

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
  kv_ops_ts_76046_1794157000231:cache:product:1: deleted
  kv_ops_ts_76046_1794157000231:cache:product:2: deleted
  kv_ops_ts_76046_1794157000231:cache:product:3: deleted

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
Response: In traditional art, the three primary colors are **red, yellow, and blue**. For light and screens, they’re **red, green, and blue (RGB)**.

--- Structured Output via SSE ---
JSON response: [
  {"name":"Earth","diameter_km":12742},
  {"name":"Mars","diameter_km":6779},
  {"name":"Jupiter","diameter_km":139820}
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello!

=== Done ===
=== ekoDB Schedule Management Example (TypeScript) ===

--- Creating schedule ---
Created schedule: Nightly Database Backup (id: 45bf27e0-9347-49ae-874b-aa55adb5a199, cron: 0 0 2 * * *)

--- Listing schedules ---
Schedules: {
  "count": 1,
  "schedules": [
    {
      "created_at": "2026-11-08T16:56:47.212294Z",
      "created_by": {
        "key_id": "3a0cdba0da7c060610042783cc758f1f",
        "key_label": "default_admin"
      },
      "cron_expression": "0 0 2 * * *",
      "description": null,
      "enabled": true,
      "function_label": "schedule_noop_typescript_76072_1794157007180",
      "id": "45bf27e0-9347-49ae-874b-aa55adb5a199",
      "last_execution": null,
      "name": "Nightly Database Backup",
      "next_execution": "2026-11-09T02:00:00Z",
      "parameters": {},
      "stats": {
        "avg_execution_time_ms": 0,
        "failed_executions": 0,
        "last_error": null,
        "successful_executions": 0,
        "total_executions": 0
      },
      "timezone": "UTC",
      "updated_at": "2026-11-08T16:56:47.212294Z"
    }
  ]
}

--- Getting schedule ---
Fetched: Nightly Database Backup (cron: 0 0 2 * * *)

--- Updating schedule ---
Updated: Nightly Full Backup (new cron: 0 0 3 * * *)

--- Triggering schedule ---
Trigger response: {
  "schedule_id": "45bf27e0-9347-49ae-874b-aa55adb5a199",
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
  2. Score: 13.200, Matched: bio, title
  3. Score: 13.200, Matched: title, bio
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
  1. Score: 0.772, Matched:
  2. Score: 0.757, Matched:
  3. Score: 0.755, Matched:

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.502, Matched: content, title
  2. Score: 0.903, Matched: content, title
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
  id: 'tePisxITjXDEegCkvLZdopR5FmgMYBKeEWpNsZ1JrlwwSsc9p_3Pr-Aw6x-MbtfG1qri4MJJW8-vXfoPLTffOQ'
}

=== Find by ID ===
Found: {
  metadata: { type: 'Object', value: { nested: [Object], key: 'value' } },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  name: { value: 'Test Record', type: 'String' },
  value: { type: 'Integer', value: 42 },
  active: { value: true, type: 'Boolean' },
  created_at: { value: '2026-11-08T16:56:47.734+00:00', type: 'DateTime' },
  id: 'tePisxITjXDEegCkvLZdopR5FmgMYBKeEWpNsZ1JrlwwSsc9p_3Pr-Aw6x-MbtfG1qri4MJJW8-vXfoPLTffOQ',
  price: { type: 'Float', value: 99.99 },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-11-08T16:56:47.734Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { nested: { deep: true }, key: 'value' }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  metadata: { nested: { deep: true }, key: 'value' },
  categories: [ 'electronics', 'computers' ],
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  tags: [ 'tag1', 'tag2', 'tag3' ],
  data: 'aGVsbG8gd29ybGQ=',
  name: 'Test Record',
  value: 42,
  active: true,
  created_at: '2026-11-08T16:56:47.734+00:00',
  id: 'tePisxITjXDEegCkvLZdopR5FmgMYBKeEWpNsZ1JrlwwSsc9p_3Pr-Aw6x-MbtfG1qri4MJJW8-vXfoPLTffOQ',
  price: 99.99,
  user_id: '550e8400-e29b-41d4-a716-446655440000'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  price: { type: 'Float', value: 99.99 },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  value: { value: 100, type: 'Integer' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  active: { type: 'Boolean', value: true },
  metadata: { type: 'Object', value: { nested: [Object], key: 'value' } },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  name: { type: 'String', value: 'Updated Record' },
  created_at: { type: 'DateTime', value: '2026-11-08T16:56:47.734+00:00' },
  id: 'tePisxITjXDEegCkvLZdopR5FmgMYBKeEWpNsZ1JrlwwSsc9p_3Pr-Aw6x-MbtfG1qri4MJJW8-vXfoPLTffOQ',
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: Q0HfOcuodhOdhbORk-COVxX7oLdpfNDqe4JU6kIcvIyeLH1DaaNmAavoYzlB9t8RVg-UiXXxNtPSP3nX7XURWw

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
✓ Created native SWR script: github_user_native_ts (ghW1ixw8HQjbBiBvGCJZsp1Ixtmwcn3NZHq6NM7b1dttr2yvDr3cR2rlTTLrIBjj9DwcSo4xfMOcCOqM2gOdIA)

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
✓ Created SWR script with audit trail: product_swr_audit_ts (RFevTUUnpd0fcO3QuDqSmnNURZYc283cGTzp350uixCVEOG3h-eq2OIZBoU-U2meCGzSgLDUWGqk5jHkKql1Iw)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_ts' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_ts (1IpGw9ZTEk-HEyQWOoq3KZ3HpJ4qauicYtFNUNBlbDg-EFQqornZxxBfFTOS3eOzbW-fRRjhag4o8nA8z2CVug)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_ts' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_ts (N3meY34C_4chiLGbDH1g2jiFpHV9o_IKUJn2UDoVkXx50A_YpOHWaOCtB8Gj6AlyhadX6riJe5dEdxsO3dXhpw)
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
✓ Created SWR script: fetch_api_user_ts_76092_1794157008856 (QUn8O5evddyT7axEggnzyxelUmQsv-4UsRLju7d5RPrZiKnKdJw6UUTMoslbr7ViuTbAyowAMMVDD-8IWCSujQ)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "value": {
          "phone": "1-770-736-8031 x56442",
          "username": "Bret",
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "name": "Leanne Graham",
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
          "website": "hildegard.org",
          "id": 1,
          "email": "Sincere@april.biz"
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
        "value": {
          "phone": "1-770-736-8031 x56442",
          "username": "Bret",
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "name": "Leanne Graham",
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
          "website": "hildegard.org",
          "id": 1,
          "email": "Sincere@april.biz"
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
✓ Created enrichment script: fetch_product_reviews_ts_76092_1794157008856 (TSW5xCPvTD1X29lJB-4HLCHcmOLW59_3C8-H1nyp8eWDLWLq1MlSnSogd3sDbEOYpQfmDpLxazHufx-vxv22JA)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "value": {
          "stock": 99,
          "warrantyInformation": "1 week warranty",
          "weight": 4,
          "tags": [
            "beauty",
            "mascara"
          ],
          "id": 1,
          "meta": {
            "createdAt": "2025-10-09T14:47:01.588Z",
            "barcode": "5784719087687",
            "updatedAt": "2026-05-23T11:27:41.868Z",
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png"
          },
          "discountPercentage": 10.48,
          "category": "beauty",
          "dimensions": {
            "width": 15.14,
            "height": 13.08,
            "depth": 22.99
          },
          "minimumOrderQuantity": 48,
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "shippingInformation": "Ships in 3-5 business days",
          "title": "Essence Mascara Lash Princess",
          "sku": "BEA-ESS-ESS-001",
          "availabilityStatus": "In Stock",
          "brand": "Essence",
          "price": 9.99,
          "returnPolicy": "No return policy",
          "reviews": [
            {
              "rating": 3,
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "reviewerName": "Eleanor Collins",
              "comment": "Would not recommend!"
            },
            {
              "reviewerEmail": "lucas.gordon@x.dummyjson.com",
              "comment": "Very satisfied!",
              "rating": 4,
              "reviewerName": "Lucas Gordon",
              "date": "2025-04-30T09:41:02.053Z"
            },
            {
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerName": "Eleanor Collins",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "comment": "Highly impressed!",
              "rating": 5
            }
          ],
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "rating": 2.56
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
Created Alice: $1000 - ID: ZOAGxbW4jRB3OzCn12sKnujflzw9dPHCTyovq4NC9j9q9bQCIQ_fdcPmmt97G9r7OBTU3100gv7NvX4VNKmAVw
Created Bob: $500 - ID: 8rQnU1fKNoOljxcyJsmyxh19IwOlDFxW2A4ImDatZQONAPjR38UoHvlE4WCsU485_hcMVevPZ2iPBAW2LuCOMw

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 46753eed-06fd-45e4-9c0e-c0e8f13e53ab

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
New transaction: 5b1b0a40-0ed3-4825-b9a9-2d4fe055c89f
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Create User Function ===
Created user function with ID: 7up90GWJ5FxKSahMqTz_nnyyhJFZZPw8XENgRF_6Az0mBULqwTMlw1VxKLpx2jxQixfsZIj19IIAvbDulG3SRA

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 9 user functions:
  - conc_demo_rl_fail_ts_72807_1794156869177: Rate-limit (fail mode)
  - get_active_users_ts: Get Active Users
  - get_active_users_client_ts: Get Active Users (Updated)
  - get_active_users_client_js: Get Active Users (Updated)
  - conc_demo_rl_skip_ts_75979_1794156993426: Rate-limit (skip mode)
  - fetch_product_reviews_ts_76092_1794157008856: Fetch Product with Reviews (Multi-API)
  - conc_demo_rl_fail_ts_75979_1794156993426: Rate-limit (fail mode)
  - fetch_product_reviews_ts_73755_1794156892739: Fetch Product with Reviews (Multi-API)
  - conc_demo_rl_skip_ts_72807_1794156869177: Rate-limit (skip mode)

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
=== WebSocket Chat Streaming Example (TypeScript) ===

Created chat session: XDzhK4MfhcFgJqRUjamWFzdF0uep1fiNqzUXmstUb28krGt1uLW8RhXr1AmAmp62HluuZLE_7ucisTlR11VJ6Q

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"gpt-5.6-luna","provider":"openai"}The capital of France is **Paris**.

--- Stream ended ---
Message ID: 18luysfN7sdCpSDNoaeHwlU_6D5cIWrVhVp6mUULq28YoFmswWMqCrx92X6v2UPD0aTrcbPtj9Mpy46W_8Vtpw
Execution time: 657ms
Token usage: {"cached_input_tokens":0,"completion_tokens":13,"prompt_tokens":23,"thinking_tokens":0,"total_tokens":36}

Full response: {"__progress":"received"}{"__progress":"generating","model":"gpt-5.6-luna","provider":"openai"}The capital of France is **Paris**....
=== WebSocket Subscription Example ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_ts' ===
✓ Subscribed (subscription_id: sub_19023f527cef42daa3f4d01e7f9c1828)

=== Performing mutations to trigger notifications ===
Inserting a record...
✓ Inserted record: RhUvQzy5bNDOa8WUBcFh-2cS8-N_3b-2i9tzcJCmMUSIurTeQaA-bYp83-CO9ZzFw8oMvb3glGnULTPzBBPgKA
  📡 Notification received for RhUvQzy5bNDOa8WUBcFh-2cS8-N_3b-2i9tzcJCmMUSIurTeQaA-bYp83-CO9ZzFw8oMvb3glGnULTPzBBPgKA

Inserting another record...
✓ Inserted record: ZCk58Q-l_x9pMllAoWr-6eYP1X8_ZTiQSa8GUb7Hg7JnrKgqK1xpldnS5UsWguTlzRT1SyJjjZanyyN1bEnNyA
  📡 Notification received for ZCk58Q-l_x9pMllAoWr-6eYP1X8_ZTiQSa8GUb7Hg7JnrKgqK1xpldnS5UsWguTlzRT1SyJjjZanyyN1bEnNyA

=== Unsubscribing ===
✓ Unsubscribed: {"collection":"ws_subscribe_example_ts","found":true,"unsubscribed":true}

✓ WebSocket subscription example completed successfully
✓ Deleted collection 'ws_subscribe_example_ts'
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: umeJwAVWYrNqs27DyL2hddP6jPhlTHa5bOlsnmQRbO8c84PCQBQIYs6qS50Oj47go8b-6s40PKUPQWJWt66yyQ

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
   Inserted with ripple: {"id":"WRnFlihQH3ll-acxbHQCmP0mTHUayJEAfxLTGZcshLcFLnGRTxhPges5Y7peMw24Q1TE9KxKEByjSe4Uvw98aw"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"VNBOvqUW7jNsynx5YqROPeKqPEsEbpsyvTlPrWivkFLarL84t3NFZ5qyrJJVm3pusS2ibiVEGwFDhxYtakkKVQ"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"id":"WRnFlihQH3ll-acxbHQCmP0mTHUayJEAfxLTGZcshLcFLnGRTxhPges5Y7peMw24Q1TE9KxKEByjSe4Uvw98aw","name":{"value":"Product 1","type":"String"},"price":{"type":"Integer","value":150}}

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
  Fields returned: ["name","id","email"]
  First user: Dave Brown <dave@example.com>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["user_role","id","status","created_at","bio","avatar_url","age","email","name"]

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
    - Fields: ["bio","age","secret_token","id","created_at","email","avatar_url","name","status","api_key","user_role","password"]
  Projected query:
    - 3 fields per record
    - Fields: ["email","id","name"]
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

BUILD SUCCESSFUL in 5s
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

BUILD SUCCESSFUL in 8s
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
Inserted: G5Sb1nrS4Fajcqe844O3TuCq58IOlg5i847yZWQGdo4KFVdBThwoOXpYM-4AJsq4AXBmwgZxT9HATKUxpZIGJA

--- updateWithAction: increment views ---
After increment: EkoRecord(fields={score=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=100.0)}), id=StringValue(value=G5Sb1nrS4Fajcqe844O3TuCq58IOlg5i847yZWQGdo4KFVdBThwoOXpYM-4AJsq4AXBmwgZxT9HATKUxpZIGJA), views=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=15)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Counter Record)}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=kotlin), StringValue(value=example)]), type=StringValue(value=Array)})})

--- updateWithAction: push to tags ---
After push: EkoRecord(fields={id=StringValue(value=G5Sb1nrS4Fajcqe844O3TuCq58IOlg5i847yZWQGdo4KFVdBThwoOXpYM-4AJsq4AXBmwgZxT9HATKUxpZIGJA), score=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=100.0)}), views=ObjectValue(value={value=IntegerValue(value=15), type=StringValue(value=Integer)}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=kotlin), StringValue(value=example), StringValue(value=advanced)]), type=StringValue(value=Array)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Counter Record)})})

--- updateWithActionSequence: multiple operations ---
After sequence: EkoRecord(fields={views=ObjectValue(value={value=IntegerValue(value=115), type=StringValue(value=Integer)}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=kotlin), StringValue(value=example), StringValue(value=advanced), StringValue(value=sequenced)]), type=StringValue(value=Array)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Counter Record)}), score=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=75.0)}), id=StringValue(value=G5Sb1nrS4Fajcqe844O3TuCq58IOlg5i847yZWQGdo4KFVdBThwoOXpYM-4AJsq4AXBmwgZxT9HATKUxpZIGJA)})

--- Cleanup ---
Deleted collection: kotlin_advanced_crud_example

=== Example Complete ===

BUILD SUCCESSFUL in 7s
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
  IDs: ZSJ0gFKZ3l9FNcYt7Str0pT-EXNcWqzERFMasji1Keq60DtRsGDeaAi6X5qXmibjXnCNkmCo2TmGviBa9tx7yA, q18QA8UcSomHX_q6CfeRZzP8FhcrmWU8y-7DGTkLd9iFXsRVV-LQUB8VyNtbVwoox5Ta3gLxMliYi_J8fusouQ, Czf-0I3xoduziCuk-UH-bxZqD9P7q6AUC7zzH9ubN2gTUF_H099CtXZrUr9WhQB6iO0IKGbKqJsuIxohRD2Ttw...

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
✓ Created session: QT0RY2y5x4e8evsst8EMdaIx9q4YJf4MgHvlK5DSJYFxo236WAkiTkiH018o2t26av-BTUUmwdLRrenDnA0uSw

=== Sending Initial Message ===
✓ Message sent
  Responses: ["The available product is **ekoDB** — a high-performance database product, priced at **$99**."]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["ekoDB costs **$99**."]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: ArsDTZ50kZIXmly-xW6EZx1TMJr1yqSxnagVafIH4fWHdRuWaj7kMRSeQiMv_RLghyJrhVIkDPDV93MYFX-9og

=== Merging Sessions ===
✓ Merged sessions
  Total messages in merged session: 5

=== Deleting Message ===
✓ Deleted message

=== Cleanup ===
✓ Deleted chat session: ArsDTZ50kZIXmly-xW6EZx1TMJr1yqSxnagVafIH4fWHdRuWaj7kMRSeQiMv_RLghyJrhVIkDPDV93MYFX-9og
✓ Deleted chat session: QT0RY2y5x4e8evsst8EMdaIx9q4YJf4MgHvlK5DSJYFxo236WAkiTkiH018o2t26av-BTUUmwdLRrenDnA0uSw
✓ Deleted collection: kotlin_chat_advanced_example

✓ Advanced chat features example completed successfully

BUILD SUCCESSFUL in 14s
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
✓ Created session: dNF322hSdoLFYcdFzx2wNtnv9Rj5LJaHM0nlcuH1UMjuKUwyCOZCMGWFXlCOAIzH2qAj_Gl551dutjGjGgLncg

=== Sending Chat Message ===
✓ Chat response:
  Message ID: "0Vx6NREq7QEDTmh1ni_kX11zBh-GT3UTuPgHz8kwHMXFC2qZ-q8SGow6Z0OoE-tWWf3LbvT1VXRvho9F2nl0vg"
  Responses: ["ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration.\n\nIts features include:\n- **AI chat integration:** Ask questions about your database in natural language and get AI-powered responses with relevant context.\n- **Search:** Full-text, vector, and hybrid search, with automatic context retrieval."]

=== Cleanup ===
✓ Deleted chat session
✓ Deleted collection: kotlin_chat_basic_example

✓ Basic chat example completed successfully

BUILD SUCCESSFUL in 12s
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
Created session: jjQjFZ1corpiK8vyNp8ZudWL9Dkv1Ntrv-mlkPvyu0y7bEejk8IvR5SSGMrPsBgW6ItMeI8yuy1PlIAW-Z5sHA

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"gpt-6-luna","provider":"openai"}**ekoDB** is a database platform designed to keep application data synchronized in real time across clients or services. The name can refer to different projects, so its exact features depend on which ekoDB you mean—share a link if you’d like details about a specific one.

--- Stream complete ---
Message ID: WvV9dpfAUg5Ot4R41MTHgKDQv8Q2DWwo46oafufQt7HhK3PBztXSECq2hiKiFzhvzAdlEANp0A-2DddW3mPdWA
Execution time: 6933ms
Context window: 128000 tokens

✓ Chat message stream example completed

BUILD SUCCESSFUL in 14s
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
    - tts-1
    - gpt-5-mini
    - gpt-5-mini-2025-08-07
    - gpt-5.4-2026-03-05
    - gpt-5-nano
    - gpt-5.4-mini
    - omni-moderation-2024-09-26
    - gpt-live-transcribe
    - gpt-5.6-luna
    - gpt-4o-2024-11-20
    - gpt-5.1
    - whisper-1
    - gpt-5-pro
    - gpt-realtime-whisper
    - omni-moderation-latest
    - gpt-realtime-2
    - gpt-5-search-api
    - gpt-5.5-2026-04-23
    - gpt-5.5-pro
    - gpt-audio-2025-08-28
    - gpt-4o-mini-transcribe-2025-03-20
    - chat-latest
    - gpt-realtime-translate
    - gpt-5.4-pro-2026-03-05
    - gpt-4o-mini-transcribe-2025-12-15
    - gpt-5-pro-2025-10-06
    - gpt-5.6-sol
    - gpt-5.4-mini-2026-03-17
    - text-embedding-3-large
    - gpt-4o-2024-08-06
    - o3
    - gpt-5.4-pro
    - gpt-4o-mini-tts
    - gpt-realtime-mini-2025-12-15
    - gpt-realtime-2.1-mini
    - gpt-realtime-2025-08-28
    - tts-1-hd-1106
    - tts-1-hd
    - gpt-realtime-1.5
    - gpt-realtime
    - gpt-5.2-pro
    - gpt-4o-transcribe-diarize
    - gpt-5.1-2025-11-13
    - gpt-5.4
    - text-embedding-3-small
    - gpt-5-2025-08-07
    - o3-2025-04-16
    - gpt-5.2-2025-12-11
    - chatgpt-image-latest
    - tts-1-1106
    - gpt-5.5-pro-2026-04-23
    - gpt-5.2-pro-2025-12-11
    - gpt-5.4-nano
    - gpt-audio-mini-2025-12-15
    - gpt-audio-mini
    - gpt-4.1-mini-2025-04-14
    - gpt-5
    - gpt-4o-mini-transcribe
    - gpt-image-2-2026-04-21
    - gpt-transcribe
    - gpt-image-2
    - text-embedding-ada-002
    - gpt-realtime-2.1
    - gpt-4.1
    - gpt-5.5
    - gpt-audio-1.5
    - gpt-image-1.5
    - gpt-6.1-sol
    - gpt-6-sol
    - gpt-realtime-mini
    - gpt-image-1-mini
    - gpt-5.6-terra
    - gpt-5-nano-2025-08-07
    - gpt-4o-transcribe
    - gpt-4.1-mini
    - gpt-4o-mini
    - gpt-6.1-terra
    - gpt-6-luna
    - gpt-4o-mini-tts-2025-12-15
    - gpt-audio
    - gpt-5.3-codex
    - gpt-5-search-api-2025-10-14
    - gpt-4o
    - gpt-image-2.6-flare
    - gpt-4o-mini-2024-07-18
    - gpt-4.1-2025-04-14
    - gpt-image-2.5-flare
    - gpt-4o-mini-tts-2025-03-20
    - gpt-5.2
    - gpt-5.4-nano-2026-03-17
    - gpt-6-astra
    - gpt-image-2.5-sunburst-2026-09-08
    - gpt-image-2.5-sunburst
    - gpt-image-2.5-flare-2026-09-08
    - gpt-image-2.6-sunburst
    - gpt-live-1
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
    - gemini-omni-1.2-flash
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
  gemini: ok 61 models
  openai: ok 96 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get OpenAI Models ===
OpenAI models: tts-1, gpt-5-mini, gpt-5-mini-2025-08-07, gpt-5.4-2026-03-05, gpt-5-nano, gpt-5.4-mini, omni-moderation-2024-09-26, gpt-live-transcribe, gpt-5.6-luna, gpt-4o-2024-11-20, gpt-5.1, whisper-1, gpt-5-pro, gpt-realtime-whisper, omni-moderation-latest, gpt-realtime-2, gpt-5-search-api, gpt-5.5-2026-04-23, gpt-5.5-pro, gpt-audio-2025-08-28, gpt-4o-mini-transcribe-2025-03-20, chat-latest, gpt-realtime-translate, gpt-5.4-pro-2026-03-05, gpt-4o-mini-transcribe-2025-12-15, gpt-5-pro-2025-10-06, gpt-5.6-sol, gpt-5.4-mini-2026-03-17, text-embedding-3-large, gpt-4o-2024-08-06, o3, gpt-5.4-pro, gpt-4o-mini-tts, gpt-realtime-mini-2025-12-15, gpt-realtime-2.1-mini, gpt-realtime-2025-08-28, tts-1-hd-1106, tts-1-hd, gpt-realtime-1.5, gpt-realtime, gpt-5.2-pro, gpt-4o-transcribe-diarize, gpt-5.1-2025-11-13, gpt-5.4, text-embedding-3-small, gpt-5-2025-08-07, o3-2025-04-16, gpt-5.2-2025-12-11, chatgpt-image-latest, tts-1-1106, gpt-5.5-pro-2026-04-23, gpt-5.2-pro-2025-12-11, gpt-5.4-nano, gpt-audio-mini-2025-12-15, gpt-audio-mini, gpt-4.1-mini-2025-04-14, gpt-5, gpt-4o-mini-transcribe, gpt-image-2-2026-04-21, gpt-transcribe, gpt-image-2, text-embedding-ada-002, gpt-realtime-2.1, gpt-4.1, gpt-5.5, gpt-audio-1.5, gpt-image-1.5, gpt-6.1-sol, gpt-6-sol, gpt-realtime-mini, gpt-image-1-mini, gpt-5.6-terra, gpt-5-nano-2025-08-07, gpt-4o-transcribe, gpt-4.1-mini, gpt-4o-mini, gpt-6.1-terra, gpt-6-luna, gpt-4o-mini-tts-2025-12-15, gpt-audio, gpt-5.3-codex, gpt-5-search-api-2025-10-14, gpt-4o, gpt-image-2.6-flare, gpt-4o-mini-2024-07-18, gpt-4.1-2025-04-14, gpt-image-2.5-flare, gpt-4o-mini-tts-2025-03-20, gpt-5.2, gpt-5.4-nano-2026-03-17, gpt-6-astra, gpt-image-2.5-sunburst-2026-09-08, gpt-image-2.5-sunburst, gpt-image-2.5-flare-2026-09-08, gpt-image-2.6-sunburst, gpt-live-1

=== Get Anthropic Models ===
Anthropic models: claude-haiku-5-5, claude-sonnet-5-5, claude-opus-5-5, claude-fable-5-1, claude-opus-5, claude-sonnet-5, claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Request failed with status 404: {"error":"Unknown provider 'nonexistent_provider_xyz'","error_kind":"unknown_provider"}

=== Chat Models Example Complete ===

BUILD SUCCESSFUL in 8s
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
✓ Created session: fGmS6pWxaJdyUH38F2mBWMBz7FBCxK_hgoI-7I_G5sdaAUFOnxt8suulVVQGBztyOImI-7wns8iebnKMzdD2ow

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["The available product is **ekoDB**, described as a high-performance database with AI capabilities. It costs **$99**."]

✓ Message 2 sent
  Responses: ["ekoDB costs **$99**."]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"author":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"chat_id":{"type":"String","value":"fGmS6pWxaJdyUH38F2mBWMBz7FBCxK_hgoI-7I_G5sdaAUFOnxt8suulVVQGBztyOImI-7wns8iebnKMzdD2ow"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"bCvdrewL-Wiy_gERK71v012w9nXgDrJjf144iitB8vglW2zvD06AOhVDAsQJxX25n7r7LCd2WCqxBeR0kwQ2EA","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-11-08T16:58:24.544370+00:00"},"id":"qnpIWt9WF0agoUlcs2PGXhTxWnG4yQfLT2tWFgTUwGOIMHUaaftZhBtiDCdTCDf_kbelyKF3wHIuks0O6mmfhw","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":29,"prompt_tokens":13558,"total_tokens":13587}},"updated_at":{"type":"DateTime","value":"2026-11-08T16:58:24.544370+00:00"}},{"author":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"chat_id":{"type":"String","value":"fGmS6pWxaJdyUH38F2mBWMBz7FBCxK_hgoI-7I_G5sdaAUFOnxt8suulVVQGBztyOImI-7wns8iebnKMzdD2ow"},"content":{"type":"String","value":"The available product is **ekoDB**, described as a high-performance database with AI capabilities. It costs **$99**."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"bCvdrewL-Wiy_gERK71v012w9nXgDrJjf144iitB8vglW2zvD06AOhVDAsQJxX25n7r7LCd2WCqxBeR0kwQ2EA","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-11-08T16:58:24.557181+00:00"},"id":"B8_f20Qr7PHAzyRAZiGXk9qtNWIH8SrwpG5ON428mcgjz1EkY0nYCrfyb30DTxwxK7mFnb-M_pOtS8TLsmGxdg","llm_model":{"type":"String","value":"gpt-6-luna"},"llm_provider":{"type":"String","value":"openai"},"role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":29,"prompt_tokens":13558,"total_tokens":13587}},"updated_at":{"type":"DateTime","value":"2026-11-08T16:58:24.557181+00:00"}},{"author":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"chat_id":{"type":"String","value":"fGmS6pWxaJdyUH38F2mBWMBz7FBCxK_hgoI-7I_G5sdaAUFOnxt8suulVVQGBztyOImI-7wns8iebnKMzdD2ow"},"content":{"type":"String","value":"What is the price?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description","product","price"],"record":{"description":"A high-performance database product with AI capabilities","id":"bCvdrewL-Wiy_gERK71v012w9nXgDrJjf144iitB8vglW2zvD06AOhVDAsQJxX25n7r7LCd2WCqxBeR0kwQ2EA","price":99,"product":"ekoDB"},"score":0.9916666666666666}]},"created_at":{"type":"DateTime","value":"2026-11-08T16:58:26.240839+00:00"},"id":"ElWjZ0g9lKbjfKC7E70uUqRuDCw4KSBuYuGnp0JiePPs0P_3_9y7kwh3EKhwfiHsyOl4ohzCnzlQe86GQq-SyQ","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":12,"prompt_tokens":13602,"total_tokens":13614}},"updated_at":{"type":"DateTime","value":"2026-11-08T16:58:26.240839+00:00"}},{"author":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"chat_id":{"type":"String","value":"fGmS6pWxaJdyUH38F2mBWMBz7FBCxK_hgoI-7I_G5sdaAUFOnxt8suulVVQGBztyOImI-7wns8iebnKMzdD2ow"},"content":{"type":"String","value":"ekoDB costs **$99**."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description","product","price"],"record":{"description":"A high-performance database product with AI capabilities","id":"bCvdrewL-Wiy_gERK71v012w9nXgDrJjf144iitB8vglW2zvD06AOhVDAsQJxX25n7r7LCd2WCqxBeR0kwQ2EA","price":99,"product":"ekoDB"},"score":0.9916666666666666}]},"created_at":{"type":"DateTime","value":"2026-11-08T16:58:26.254004+00:00"},"id":"5wbLzMvzPGrGHecw6Btz18T48wgrfXC9lUKfhKuX6Z1u4w_Dp9fY_llGFPgfSmYUv4uGnYV7HxEbQhleGT2YOg","llm_model":{"type":"String","value":"gpt-6-luna"},"llm_provider":{"type":"String","value":"openai"},"role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":12,"prompt_tokens":13602,"total_tokens":13614}},"updated_at":{"type":"DateTime","value":"2026-11-08T16:58:26.254004+00:00"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 1

=== Branching Session ===
✓ Created branched session: 7OJ0cdydJO7z1Aq6qb1TVInkE-QRxWi1bA_FfTtNi4UPiJGC3FK4AEnQWO5pc3i3TYUBzr0N9Fs1PEjDzCGogw

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_sessions_example

✓ Chat session management example completed successfully

BUILD SUCCESSFUL in 12s
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
  - schema_products_client_go
  - schema_documents_client_ts
  - chat_raw_completions__ek0_testing
  - chat_agent_configs__ek0_testing
  - schedules__ek0_testing

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
✓ Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
✓ Schema: {"fields":{"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2026-11-08T16:58:35.605737Z","last_modified":"2026-11-08T16:58:35.605737Z","bypass_ripple":false,"primary_key_alias":"id"}

=== Cleanup ===
✓ Deleted collection: kotlin_collection_example

=== Example Complete ===

BUILD SUCCESSFUL in 8s
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
Collection metadata: {"analytics":["collection_utils_test_kt",{"manifest_backed":false,"record_count":5,"resident_record_ids":5,"total_size":675}],"collection":{"bypass_ripple":false,"created_at":"2026-11-08T16:58:44.636959Z","fields":{"index":{"default":null,"enums":[],"field_type":"Integer","max":null,"min":null,"regex":null,"required":false,"unique":false},"name":{"default":null,"enums":[],"field_type":"String","max":null,"min":null,"regex":null,"required":false,"unique":false}},"last_modified":"2026-11-08T16:58:44.637224Z","primary_key_alias":"id","version":1}}

=== List Collections ===
All collections (27):
  - schema_products_client_go
  - schema_documents_client_ts
  - chat_raw_completions__ek0_testing
  - chat_agent_configs__ek0_testing
  - schedules__ek0_testing
  - functions__ek0_testing
  - schema_documents_client_go
  - audit__ek0_testing
  - schema_employees_client_go
  - chat_configurations__ek0_testing
  - chat_goals__ek0_testing
  - test_accounts
  - agent_function_versions__ek0_testing
  - test_collection
  - chat_tasks__ek0_testing
  - chat_goal_templates__ek0_testing
  - schema_employees_client_js
  - collection_utils_test_kt
  - schema_documents_client_js
  - schema_employees_client_ts
  - schema_users_client_js
  - schema_products_client_js
  - schema_products_client_ts
  - schema_users_client_go
  - schema_users_client_ts
  - chat_turns__ek0_testing
  - chat_messages__ek0_testing

=== Check Non-Existent Collection ===
Collection 'nonexistent_collection_xyz_230086870539625' exists: false

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
✓ Created record: EkoRecord(fields={id=StringValue(value=bJk0RucG30AxJ4D_vrb1mveoTJPVZXMeJ3HncQHwLBBTWX1lckqziNjpljxT8MU5kEWi25e30I0qtowlNCzNzw)})

=== Upsert Operation ===
✓ First upsert (update): EkoRecord(fields={age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), id=StringValue(value=bJk0RucG30AxJ4D_vrb1mveoTJPVZXMeJ3HncQHwLBBTWX1lckqziNjpljxT8MU5kEWi25e30I0qtowlNCzNzw), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice.j@newdomain.com)})})
✓ Second upsert (insert): EkoRecord(fields={id=StringValue(value=new-user-id)})

=== Find One Operation ===
✓ Found user by email: EkoRecord(fields={name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), id=StringValue(value=bJk0RucG30AxJ4D_vrb1mveoTJPVZXMeJ3HncQHwLBBTWX1lckqziNjpljxT8MU5kEWi25e30I0qtowlNCzNzw), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice.j@newdomain.com)})})
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
  Document ID: Lqhki-h8j9nHcLeOMV2GTlXLeBgdYRw5IV99p2X-kNOnXv-huMkujQ6j5aVQ7uwjE2K3zc-qDPyccKez7VhIyg

=== Verify Document Exists ===
✓ Document found: session_id, created_at, id, user_id, ttl

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: BRIRgcJutaY9C5XEdeAUXO8TMYFQmAomZxDc4zEzd-qo3UsZ0pigAa35NrHviZf032QkPztDY-eXPHYGo-0B9A

=== TTL Expiration ===
✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document ===
✓ Long TTL document still exists: id, cache_key, value, ttl

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
✓ Edge cache function created: jjZ4-7BFtJgxRN4A2GU_KosJRzrs1PGQqk6ayI1dkyrS3-LW-4ftpbnpjXETTAiOdxfIsDbeWb6h9khJqnqz4w

Call 1: Cache lookup
Found 1 cached entries
Response time: 15ms

Call 2: Cache lookup (connection warm)
Found 1 cached entries
Response time: 3ms

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

BUILD SUCCESSFUL in 9s
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
   Department: {"value":"engineering","type":"String"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 56ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 3ms
   📊 Records: 1
   🚀 Cache speedup: 18.7x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: get_verified_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: {"value":"User 1","type":"String"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - get_verified_user: Specific workflow

🧹 Cleaning up...
✅ Cleanup complete

✅ All composition examples completed!

BUILD SUCCESSFUL in 9s
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

BUILD SUCCESSFUL in 8s
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

✅ Function saved: NZZar-dfLR592igeFKj-Qo1eeFSBFcptOfR0MO_9e5nYWFA-aBaCOBJ-mRb76RyADEyCN4spKquUE9AA_-wFFQ
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
   {"avg_score":{"type":"Float","value":60.0},"count":{"type":"Integer","value":5},"status":{"value":"active","type":"String"}}
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
📊 Total user count: {"value":10,"type":"Integer"}
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
   {"category":{"type":"String","value":"Furniture"},"avg_price":{"value":365.6666666666667,"type":"Float"},"count":{"value":3,"type":"Integer"}}
   {"category":{"type":"String","value":"Electronics"},"avg_price":{"type":"Float","value":367.0},"count":{"value":5,"type":"Integer"}}
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
   Vector databases store and search **embeddings**—numerical representations of text, images, audio, or other data. Their main benefits are:

- **Semantic search:** Find items with similar meaning or content, even when they don’t share exact keywords.
- **Fast similarity search at scale:** Specialized indexing helps retrieve nearby vectors efficiently across large datasets.
- **AI application support:** They make it easier to build retrieval-augmented generation (RAG), recommendations, image search, and deduplication systems.
- **Metadata filtering:** Combine similarity search with filters such as date, category, or access permissions.
- **Practical data management:** Many support updating, deleting, and managing vectors alongside their source data.

They’re useful when similarity is central to the task, but they generally complement—not replace—traditional databases for structured data and exact queries.
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

BUILD SUCCESSFUL in 13s
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

✅ Function saved: sti-aLqhAG_P5lrTcmDlEemQoIJOshTJpp9o2z6UPYFtP-z1tRUbVk90CB1l32P634H_RZaXrvQSkSHfG5IMrQ
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

BUILD SUCCESSFUL in 8s
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
   {"status":{"type":"String","value":"inactive"},"count":{"type":"Integer","value":3}}
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

✅ Inserted order: StringValue(value=japy39ZrLWO73eJHp8GfMLhP2zPS8eaXw1ttAtxdprZNm47m7ZNDNOhyx_xFYRT8Zqcgf93eJjm0wU0MzsWjog)
✅ Inserted 2 products

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: d_xStaPKZVRsSZE0sc9x2ZsLZGcHd9aSS7YDQCH-NUN6VTD1qYRAjdYh4qb7tRRZqGilggEJ3F0mhlNjAbgAAw
📊 Created order via function
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"type":"Object","value":{"userId":"user_abc","role":"admin"}}
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: HvECsc5GbVS3aJRxI940UpGySwF3AqsaJzy-L-o0OIaUOX23-euEN0Hr9QMTL9Fo1gbjDh-Li0CqAuuuoMW92A
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: _Uvp5KCRDAEOPo_wvBpA6r6gvMyeQdU3XuKSWeAAQoKR5UhR7VDk_OacDSEtIt44rhxSosLF-F6zKQ7xorwbXA
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

BUILD SUCCESSFUL in 8s
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
   1. {"value":"Database Design Principles","type":"String"} ({"type":"String","value":"Database"})
   2. {"type":"String","value":"Introduction to Machine Learning"} ({"value":"AI","type":"String"})
   3. {"value":"Vector Databases Explained","type":"String"} ({"type":"String","value":"Database"})
   4. {"value":"Natural Language Processing","type":"String"} ({"type":"String","value":"AI"})
   5. {"type":"String","value":"Getting Started with ekoDB"} ({"type":"String","value":"Database"})
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"count":{"value":3,"type":"Integer"},"category":{"value":"Database","type":"String"}}
   {"count":{"type":"Integer","value":2},"category":{"type":"String","value":"AI"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search function examples finished!

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
Created goal: eNtwSb8cjsAIGXgRfRnM79S4UCHlTkNCRstbZ3nbUH8wOu4W3FL_5j5pZmaDyl_XkVMao_wXyC2ZlYAjZkqsEA

--- Listing goals ---
Goals: {"count":1,"goals":[{"created_at":{"type":"DateTime","value":"2026-11-08T17:01:18.188476+00:00"},"description":{"type":"String","value":"Ship the next major release"},"id":"eNtwSb8cjsAIGXgRfRnM79S4UCHlTkNCRstbZ3nbUH8wOu4W3FL_5j5pZmaDyl_XkVMao_wXyC2ZlYAjZkqsEA","owner":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"status":{"type":"String","value":"pending"},"steps":{"type":"String","value":"[{\"description\":\"Run test suite\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0"},"updated_at":{"type":"DateTime","value":"2026-11-08T17:01:18.188476+00:00"}}],"total":1}

--- Getting goal ---
Fetched: {"type":"String","value":"Deploy v2.0"}

--- Updating goal ---
Updated description: {"type":"String","value":"Ship v2.0 with full test coverage"}

--- Searching goals ---
Search results: {"count":1,"items":[{"_score":12.870000000000001,"created_at":{"type":"DateTime","value":"2026-11-08T17:01:18.188476+00:00"},"description":{"type":"String","value":"Ship v2.0 with full test coverage"},"id":"eNtwSb8cjsAIGXgRfRnM79S4UCHlTkNCRstbZ3nbUH8wOu4W3FL_5j5pZmaDyl_XkVMao_wXyC2ZlYAjZkqsEA","owner":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"status":{"type":"String","value":"pending"},"steps":{"type":"String","value":"[{\"description\":\"Run test suite\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0"},"updated_at":{"type":"DateTime","value":"2026-11-08T17:01:18.211645+00:00"}}]}

--- Goal step lifecycle ---
Step 0 started: "eNtwSb8cjsAIGXgRfRnM79S4UCHlTkNCRstbZ3nbUH8wOu4W3FL_5j5pZmaDyl_XkVMao_wXyC2ZlYAjZkqsEA"
Step 0 completed: "eNtwSb8cjsAIGXgRfRnM79S4UCHlTkNCRstbZ3nbUH8wOu4W3FL_5j5pZmaDyl_XkVMao_wXyC2ZlYAjZkqsEA"
Step 1 started: "eNtwSb8cjsAIGXgRfRnM79S4UCHlTkNCRstbZ3nbUH8wOu4W3FL_5j5pZmaDyl_XkVMao_wXyC2ZlYAjZkqsEA"
Step 1 failed: "eNtwSb8cjsAIGXgRfRnM79S4UCHlTkNCRstbZ3nbUH8wOu4W3FL_5j5pZmaDyl_XkVMao_wXyC2ZlYAjZkqsEA"

--- Completing goal ---
Goal status after complete: {"type":"String","value":"pending_review"}

--- Approving goal ---
Goal status after approve: {"type":"String","value":"in_progress"}

--- Creating goal for rejection ---
Goal status after reject: {"type":"String","value":"failed"}

--- Creating task ---
Created task: DIKu9mVwUAmSQNeEoQ9ondTaLfimBIlsMOX1fSwwt9i3PuRc3-_rPKZEI1gZav_0zCaGtxKzcfog9N9gjHD_qA

--- Listing tasks ---
Tasks: {"count":1,"items":[{"cron":{"type":"String","value":"0 2 * * *"},"description":{"type":"String","value":"Full database backup every night at 2 AM"},"id":"DIKu9mVwUAmSQNeEoQ9ondTaLfimBIlsMOX1fSwwt9i3PuRc3-_rPKZEI1gZav_0zCaGtxKzcfog9N9gjHD_qA","name":{"type":"String","value":"Nightly Backup"},"owner":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}}}],"total":1}

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
Created agent: AQlbKbzXwYxrUY82kEkDjUlqU020LrYVlHXpZN91y-ZTyKOjkuhtAeSqtZ8dRKjufIZsZtarkGESdG5tYdpGgQ — null

--- Listing agents ---
Agents: {"count":1,"items":[{"deployment_id":{"type":"String","value":"deploy_kt_example"},"id":"AQlbKbzXwYxrUY82kEkDjUlqU020LrYVlHXpZN91y-ZTyKOjkuhtAeSqtZ8dRKjufIZsZtarkGESdG5tYdpGgQ","llm_model":{"type":"String","value":"gpt-6-luna"},"name":{"type":"String","value":"DataBot"},"owner":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"run_policy":{"type":"String","value":"{\"trigger\":{\"allow_write_operations\":false,\"write_tools\":[]},\"eval\":{\"allow_write_operations\":false,\"write_tools\":[]},\"customer\":{\"allow_write_operations\":false,\"write_tools\":[]}}"},"system_prompt":{"type":"String","value":"You are a data analysis assistant."}}],"total":1}

--- Getting agent by ID ---
Agent: {"type":"String","value":"DataBot"}

--- Getting agent by name ---
By name: {"type":"String","value":"DataBot"}

--- Updating agent ---
Updated agent system_prompt

--- Agents by deployment ---
Agents for deployment: {"count":0,"items":[]}
WARNING: agents-by-deployment omitted created agent AQlbKbzXwYxrUY82kEkDjUlqU020LrYVlHXpZN91y-ZTyKOjkuhtAeSqtZ8dRKjufIZsZtarkGESdG5tYdpGgQ; TODO: check/fix the server-side deployment lookup

--- Deleting agent ---
Agent deleted

--- Cleanup: deleting goals ---
Goals deleted

=== Example Complete ===

BUILD SUCCESSFUL in 9s
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
Created template: Data Migration (id: zQpT6St5gQDMuo09VydFHFYe1XXoSncm714V4LQnE9g47XirSqFkOJM-o41i7jvYdSr22eAfvIBvMKp3fssimg)

--- Listing templates ---
Templates: {"count":1,"items":[{"description":{"type":"String","value":"Template for migrating data between schemas"},"id":"zQpT6St5gQDMuo09VydFHFYe1XXoSncm714V4LQnE9g47XirSqFkOJM-o41i7jvYdSr22eAfvIBvMKp3fssimg","owner":{"type":"Object","value":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"}},"steps":{"type":"Array","value":[{"description":"Analyze source schema"},{"description":"Create target schema"},{"description":"Migrate records"},{"description":"Validate results"}]},"title":{"type":"String","value":"Data Migration"}}],"total":1}

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

BUILD SUCCESSFUL in 9s
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

BUILD SUCCESSFUL in 8s
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
Links for user:alice: [{"collection":"orders","document_id":"order_002","field_path":null,"created_at":"2026-11-08T17:01:55.642442Z","last_accessed":"2026-11-08T17:01:55.649619Z","metadata":{}},{"collection":"invoices","document_id":"inv_100","field_path":null,"created_at":"2026-11-08T17:01:55.645781Z","last_accessed":"2026-11-08T17:01:55.649619Z","metadata":{}},{"collection":"orders","document_id":"order_001","field_path":null,"created_at":"2026-11-08T17:01:55.639137Z","last_accessed":"2026-11-08T17:01:55.649619Z","metadata":{}}]

--- Unlinking document ---
Unlinked order_002: null

--- Verifying remaining links ---
Remaining links: [{"collection":"orders","document_id":"order_001","field_path":null,"created_at":"2026-11-08T17:01:55.639137Z","last_accessed":"2026-11-08T17:01:55.657315Z","metadata":{}},{"collection":"invoices","document_id":"inv_100","field_path":null,"created_at":"2026-11-08T17:01:55.645781Z","last_accessed":"2026-11-08T17:01:55.657315Z","metadata":{}}]

--- Cleanup ---
Unlinked remaining documents
Deleted key: user:alice

=== Example Complete ===

BUILD SUCCESSFUL in 8s
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
✓ Retrieved value: {"type":"Object","value":{"role":"admin","email":"alice@example.com","name":"Alice"}}

=== KV Set with TTL ===
✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key ===
✓ Session value: {"value":{"created_at":1794157324753,"user_id":"123"},"type":"Object"}
  (Will expire in 10 seconds)

=== KV Batch Set ===
✓ Batch set 3 keys
  kv_ops_kt_1794157324633:config:db: success
  kv_ops_kt_1794157324633:config:cache: success
  kv_ops_kt_1794157324633:config:api: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  kv_ops_kt_1794157324633:config:db: {"port":5432,"host":"localhost"}
  kv_ops_kt_1794157324633:config:cache: {"ttl":3600,"enabled":true}
  kv_ops_kt_1794157324633:config:api: {"timeout":30,"retries":3}

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
  kv_ops_kt_1794157324633:config:db: deleted
  kv_ops_kt_1794157324633:config:cache: deleted
  kv_ops_kt_1794157324633:config:api: deleted

=== Example Complete ===

BUILD SUCCESSFUL in 8s
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

BUILD SUCCESSFUL in 8s
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

BUILD SUCCESSFUL in 8s
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
  - Score: ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=88)})

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
Response: It depends on the color model: **red, yellow, and blue** are the traditional primary colors for painting; **red, green, and blue** are primary colors of light.

--- Structured Output via SSE ---
JSON response: [
  {"name": "Earth", "diameter_km": 12742},
  {"name": "Mars", "diameter_km": 6779},
  {"name": "Jupiter", "diameter_km": 139820}
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello!

=== Done ===

BUILD SUCCESSFUL in 12s
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
Created schedule: c8c21314-1ddf-4cc5-bdd9-ad30441d12a9 — "Hourly Health Check"

--- Listing schedules ---
Schedules: {"count":1,"schedules":[{"created_at":"2026-11-08T17:03:03.468699Z","created_by":{"key_id":"3a0cdba0da7c060610042783cc758f1f","key_label":"default_admin"},"cron_expression":"0 0 * * * *","description":"Ping all services every hour","enabled":true,"function_label":"schedule_noop_kotlin_60271_1794157383328","id":"c8c21314-1ddf-4cc5-bdd9-ad30441d12a9","last_execution":null,"name":"Hourly Health Check","next_execution":"2026-11-08T18:00:00Z","parameters":{},"stats":{"avg_execution_time_ms":0.0,"failed_executions":0,"last_error":null,"successful_executions":0,"total_executions":0},"timezone":"UTC","updated_at":"2026-11-08T17:03:03.468699Z"}]}

--- Getting schedule ---
Fetched: "Hourly Health Check" (cron: "0 0 * * * *")

--- Updating schedule ---
Updated cron: "0 */30 * * * *"

--- Triggering schedule ---
Trigger response: {"schedule_id":"c8c21314-1ddf-4cc5-bdd9-ad30441d12a9","status":"triggered"}

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
✓ Inserted user 1: StringValue(value=LtctnnpPAEtuu_0vqYx3zDgUiiwH3AyRpKt9oauKhWz18jKILT_8vuUX-u7xzoY1bObgMEYN7xfWcscRucGNww)
✓ Inserted user 2: StringValue(value=rzktGNbBcxuS1q2ch47ZSGrN8gYvAR5ag7Lcr0iyVlwOQ5n1bfWU3O0HheT6q-_dPLnpZNSDo0VzyVUmLyg5Dw)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections ===
✓ Total collections: 27
  Sample: [schema_products_client_go, schema_documents_client_ts, chat_raw_completions__ek0_testing, chat_agent_configs__ek0_testing, schedules__ek0_testing]

=== Cleanup ===
✓ Deleted collection: kotlin_schema_example

✓ All schema management operations completed successfully

BUILD SUCCESSFUL in 8s
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
{"type":"String","value":"Rust Programming Answer"}: score=0.9162907318741551, matched=[title]
{"value":"Rust Programming Question","type":"String"}: score=0.9162907318741551, matched=[title]
Paired vector search: document=Rust Programming Answer, query=Rust Programming Question
Filtered vector results: 2
Custom-weight hybrid results: 3
Raw search: {"results":[{"record":{"title":{"value":"Rust Programming Answer","type":"String"},"category":{"value":"programming","type":"String"},"id":"2nBYXpy2rnuWvikYB4mXWj24za_o5umKiN5MAsjbTffBT2ONUnZKbggRTPAuzLntveS-rgd6IZDPq63PULbmQQ","document_embedding":{"value":[1.0,0.0,0.0],"type":"Vector"},"query_embedding":{"type":"Vector","value":[0.0,1.0,0.0]}},"score":1.0,"matched_fields":[]},{"record":{"query_embedding":{"value":[1.0,0.0,0.0],"type":"Vector"},"id":"u5duQ6KLjuArTYUmwPyBoy6lgYNwoWFU6HevjY8K5s3_T4kXJiuWUnxaaSMhi5wRPm6d07qhe5exUyyMdzgKOg","category":{"type":"String","value":"programming"},"document_embedding":{"type":"Vector","value":[0.0,1.0,0.0]},"title":{"value":"Rust Programming Question","type":"String"}},"score":1.0,"matched_fields":[]}],"total":2,"execution_time_ms":0}

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
✓ Inserted user: EkoRecord(fields={id=StringValue(value=hUkr-xAuZ7DaypuPZSSlZA_xNcwf3jXjQc1EqcZ98rQUD2i80vkhixqplD9yq34HPfyiHR6d9B4iGDSC0i3uXw)})
  User ID: hUkr-xAuZ7DaypuPZSSlZA_xNcwf3jXjQc1EqcZ98rQUD2i80vkhixqplD9yq34HPfyiHR6d9B4iGDSC0i3uXw

=== Read ===
✓ Found user by ID: EkoRecord(fields={email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), created_at=ObjectValue(value={value=StringValue(value=Sun Nov 08 12:03:30 EST 2026), type=StringValue(value=String)}), age=ObjectValue(value={value=IntegerValue(value=28), type=StringValue(value=Integer)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])}), id=StringValue(value=hUkr-xAuZ7DaypuPZSSlZA_xNcwf3jXjQc1EqcZ98rQUD2i80vkhixqplD9yq34HPfyiHR6d9B4iGDSC0i3uXw), embedding=VectorValue(value=[0.1, 0.2, 0.3, 0.4, 0.5]), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Alice Johnson
  email (String): alice@example.com
  age (Integer): 28
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): Sun Nov 08 12:03:30 EST 2026
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1, tag2, tag3]
  metadata (Object): {key=value, nested={deep=true}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): [electronics, computers]
  data (Bytes): 11 bytes
Record fields: email, created_at, age, metadata, data, user_id, active, categories, id, embedding, tags, price, name

=== Update ===
✓ Updated user: EkoRecord(fields={city=ObjectValue(value={value=StringValue(value=San Francisco), type=StringValue(value=String)}), embedding=VectorValue(value=[0.1, 0.2, 0.3, 0.4, 0.5]), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), metadata=ObjectValue(value={value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})}), type=StringValue(value=Object)}), id=StringValue(value=hUkr-xAuZ7DaypuPZSSlZA_xNcwf3jXjQc1EqcZ98rQUD2i80vkhixqplD9yq34HPfyiHR6d9B4iGDSC0i3uXw), created_at=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Sun Nov 08 12:03:30 EST 2026)}), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)})})

=== Query ===
✓ Found 1 users matching query
  - EkoRecord(fields={email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), embedding=VectorValue(value=[0.1, 0.2, 0.3, 0.4, 0.5]), city=ObjectValue(value={value=StringValue(value=San Francisco), type=StringValue(value=String)}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), id=StringValue(value=hUkr-xAuZ7DaypuPZSSlZA_xNcwf3jXjQc1EqcZ98rQUD2i80vkhixqplD9yq34HPfyiHR6d9B4iGDSC0i3uXw), data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), created_at=ObjectValue(value={value=StringValue(value=Sun Nov 08 12:03:30 EST 2026), type=StringValue(value=String)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])})})

=== Delete ===
✓ Deleted user with ID: hUkr-xAuZ7DaypuPZSSlZA_xNcwf3jXjQc1EqcZ98rQUD2i80vkhixqplD9yq34HPfyiHR6d9B4iGDSC0i3uXw

✓ Confirmed user was deleted

=== Cleanup ===
✓ Deleted collection: kotlin_users_example

=== Example Complete ===

BUILD SUCCESSFUL in 9s
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
  {"data":[{"status":{"type":"String","value":"active"},"id":"MCZ7l8_tWML4NAJ_4sTojHq7flsx1DxkaPvmCF2hQWbnbiWHVMm8-51wg2Ah0HPb-EqrdethI8qk1h6Ww2Sazg","name":{"value":"Test User","type":"String"}}]}

=== Close WebSocket ===
✓ WebSocket closed

=== Cleanup ===
✓ Deleted collection: kotlin_websocket_example

=== Example Complete ===

BUILD SUCCESSFUL in 8s
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
✓ Created native SWR function: github_user_native_kt (SbgUie6NRexixy_XWF_PyR1JRPdv45Bo392qZBZX_99vEtpm3UYCquAb2GYuqtmxhgkwuC3FvO_XJhS8aH7c1Q)

First call (cache miss - will fetch from GitHub API):
  Response time: 170ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 2ms
  Speedup: 85.0x faster 🚀


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR function with audit trail: product_swr_audit_kt (5z4sdfTA8yhIuO-JVsw0cYk0ssJDvZbpxvIFX8YjgyCfnhUOKnJ9jW2Ibq4b2vzB1SbwvEkgcIUT6NZnQ0fnwQ)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_kt' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_kt (FuhK9Y3nUXGaYu4cLUwRSrGg2Jlz6vZEbam9c1wfKl1vCjaXI1MtFLaHRJSaXYnYIVyQ5S4zW7Uhy_BWgijfyg)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_kt' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL function: flexible_cache_kt (d5-nEygKh9jrJDVkzShh6CcIreWu7qfv9SpJTeCgGuEcr0KrlWLhb2S5GsJGR1e66qn5jxznoki8qca82cdxFQ)
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

BUILD SUCCESSFUL in 10s
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
✓ Created SWR function: swr_cache_lookup_kt_1794157439364 (ZbgzEr9YR5SyuKF39AQh6xexKBEYQDyWr24I-fp5AYdWD9BmmH6hYeoy48fg1QzxzESIp-6r9J-fgci6jWpB5w)

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
Created Alice: $1000 - ID: z4jLEW2asFekde4_kImU4HRBk5hMvdET28WMNkasutLMleS4srtqPMdlHeEJdaMSvzv2MCg35k-Uc-7MjF0S4w
Created Bob: $500 - ID: eGiIkU2bPY99QGSsiOLB9MCqgBsdmkXKmjBlIWn__1Nn6YpJ5R8PGm6PTabnGMAh-Koko6urVqWu3Nl0crmDSA

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 715dd645-7551-4596-a171-349b04bd18d6

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
New transaction: acac8e70-942e-4355-ba7f-c1dab967c5a2
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
Created user function with ID: YYJ55b1YGUSrBuQJulw5cK07_u-TfyQIGO9DhBUPNblWrAxHDJ3vcsryLIsS7kAlKoOsyGSfT86ihi5ODXMDbQ

=== Get User Function ===
Retrieved: "get_active_users_kt" - "Get Active Users (Kotlin)"
Description: "Fetches all users and filters by active status"

=== List All User Functions ===
Found 10 user functions:
  - "conc_demo_rl_fail_ts_72807_1794156869177": "Rate-limit (fail mode)"
  - "client_functions_get_active_users_kt": "Get Active Users (Updated)"
  - "get_active_users_client_ts": "Get Active Users (Updated)"
  - "get_active_users_client_js": "Get Active Users (Updated)"
  - "conc_demo_rl_skip_ts_75979_1794156993426": "Rate-limit (skip mode)"

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

Created chat session: cW0453pxn3gEc7btY7fpPOV7FEqPCy_8meXjhDupDbaipDrnmOuQ0dfjBFttyM8Cj2flmN3Z-GI2vbRobbh_Ew

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"gpt-5.6-luna","provider":"openai"}The capital of France is **Paris**.

--- Stream ended ---
Message ID: tX58WCfN-GUx2mr5l3N6u5LCp74ypzqmNmMVHZEE2uPSjqPUgs9LGfvIKwskoAYB_30woorMUFYeYH_J1EkYwA
Execution time: 858ms
Token usage: {"cached_input_tokens":0,"completion_tokens":13,"prompt_tokens":23,"thinking_tokens":0,"total_tokens":36}

Full response: {"__progress":"received"}{"__progress":"generating","model":"gpt-5.6-luna","provider":"openai"}The capital of France is **Paris**....

BUILD SUCCESSFUL in 9s
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
✓ Subscribed (subscription_id: sub_ef6d389199e14048bb8a731495f38b6b)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: "PX16xd3ao_AjQ4mnibP9vwX674Oe4GeKlSpQtB6IEc-QVXJ-PtjDotBPWPMPK1PL1vP_kO4plB6X8JImO8NRCg"

  📡 Notification received:
     Event:      "insert"
     Collection: "ws_subscribe_example_kt"
     Record IDs: ["PX16xd3ao_AjQ4mnibP9vwX674Oe4GeKlSpQtB6IEc-QVXJ-PtjDotBPWPMPK1PL1vP_kO4plB6X8JImO8NRCg"]
     Timestamp:  "2026-11-08T17:04:38.068446+00:00"

Inserting record 2...
✓ Inserted: "puyuDdTy75INFvahV6DDbjm-QKhww9VVEPUcQ9WD7qvLJy3eIG0s0FYLhyeb60rQe9_lt_oIMP7VPJUs7ck_wQ"

  📡 Notification received:
     Event:      "insert"
     Record IDs: ["puyuDdTy75INFvahV6DDbjm-QKhww9VVEPUcQ9WD7qvLJy3eIG0s0FYLhyeb60rQe9_lt_oIMP7VPJUs7ck_wQ"]

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
✓ Inserted document with TTL: StringValue(value=vRP4J3ku5yRR2miWjsvw04EgGg8eKGC-zzQfJQo6jpjL4wRcIxixb0eTM8iqdJ7n2v7ZE75K4nm_yqFaUYhUhA)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"data":[{"id":"vRP4J3ku5yRR2miWjsvw04EgGg8eKGC-zzQfJQo6jpjL4wRcIxixb0eTM8iqdJ7n2v7ZE75K4nm_yqFaUYhUhA","created_at":{"type":"Integer","value":1794157486696},"value":{"type":"Integer","value":42},"name":{"type":"String","value":"WebSocket TTL Test"},"ttl":"2026-11-08T18:04:46.807885Z"}]}

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
   Inserted with ripple: EkoRecord(fields={id=StringValue(value=iCpkWzpHzZ9OVByQooBrl1bYr0UrrG5aZ5wyiojLa911LfFVrhbdRnO5rrhKQsFyUbkzXBCymqhDsxiWzpz_9Q)})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: EkoRecord(fields={id=StringValue(value=gv3xjWkVDcwWdaBXPEib9BepDhnPFCcAL82F4K6krhXPHSTavyZTyPzI0vszua6UgdUPMIXX1q2PKO0sCwTnqQ)})

3. Update with bypass_ripple:
   Updated with bypass_ripple: EkoRecord(fields={price=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=150)}), id=StringValue(value=iCpkWzpHzZ9OVByQooBrl1bYr0UrrG5aZ5wyiojLa911LfFVrhbdRnO5rrhKQsFyUbkzXBCymqhDsxiWzpz_9Q), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Product 1)})})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: EkoRecord(fields={price=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=500)}), id=StringValue(value=iCpkWzpHzZ9OVByQooBrl1bYr0UrrG5aZ5wyiojLa911LfFVrhbdRnO5rrhKQsFyUbkzXBCymqhDsxiWzpz_9Q), name=ObjectValue(value={value=StringValue(value=Upsert Product), type=StringValue(value=String)})})

✅ All bypass_ripple operations completed successfully!

BUILD SUCCESSFUL in 8s
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
    Finished `release` profile [optimized] target(s) in 0.10s
📦 Built wheel for abi3 Python ≥ 3.8 to ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.32.0-cp38-abi3-macosx_11_0_arm64.whl
📦 Installing Python wheel into .venv...
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.32.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.32.0
    Uninstalling ekodb_client-0.32.0:
      Successfully uninstalled ekodb_client-0.32.0
Successfully installed ekodb-client-0.32.0
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
    ✓ Generated embedding: 1536 dimensions in 460.804792ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 271.738ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 234.168667ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 271.472625ms
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 308.596834ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 278.503625ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 277.214333ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 310.740666ms
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 272.524167ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 285.147875ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 249.947292ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 274.533959ms
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
    ✓ Generated embedding: 1536 dimensions in 249.498667ms
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages_rust
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 41.656917ms

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

Write database code so its **memory use is bounded by the work in progress**, then optimize the queries that dominate real workloads. “Memory-safe” can mean two things: preventing bugs such as use-after-free and data races, and avoiding application or database memory exhaustion. Good design addresses both.

### Keep memory bounded
- **Paginate or stream results.** Don’t load an entire large collection or result set into memory. Fetch a limited page, process it, then continue.
- **Project only needed fields.** Large blobs and unused columns add allocation and transfer costs.
- **Batch writes** in a measured, bounded size. Oversized batches can spike memory and increase latency; tiny batches can waste round trips.
- **Put limits on inputs and work:** page size, payload size, concurrency, and queued jobs. Apply backpressure instead of letting work accumulate in unbounded queues.
- **Release resources promptly.** Close cursors, transactions, connections, and file/blob handles using your language’s scope-based cleanup (`defer`, `using`, RAII, etc.). Don’t keep references to large records longer than necessary.

### Use safe concurrency and ownership
Prefer a memory-safe language and its standard ownership or garbage-collection mechanisms. Avoid raw pointers and shared mutable state unless profiling shows a real need. If multiple tasks share a connection pool or cache, use synchronization primitives correctly—and bound the number of concurrent tasks. For example, 1,000 concurrent queries can exhaust memory even if each query is individually small.

### Make queries do less work
- Avoid N+1 patterns: fetch related data in batches or through an appropriate join/API operation.
- Filter early and use indexes that match common filters and sort orders.
- Use query-plan tools and representative data to check whether a query scans too much.
- Prefer keyset pagination for large, changing datasets when offset-based pagination becomes expensive.
- Avoid repeated conversions, copies, and serialization of large values in hot paths.

### Handle failures safely
Use parameterized query APIs rather than building query strings from input. Keep transactions short, roll them back on error, and make retries safe—especially for writes. Set timeouts and define what happens when a database or downstream service is slow.

### Measure, then tune
Profile memory allocations, peak resident memory, query latency, and throughput under realistic load. Test large result sets, slow queries, retries, and concurrent requests. Change batch sizes, pool sizes, or indexes based on those measurements rather than guesses.

A useful rule of thumb: **each request should have a predictable upper bound on rows, bytes, and concurrent work**. In ekoDB, the same principles translate to using query limits and field projection, explicit schemas, indexes for frequent filters, and bounded batches or stored functions for repeatable operations.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 220.183916ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2951 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 230.177875ms
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
  ✓ Text search completed in 26.555292ms

✓ Found 3 messages mentioning ownership:
  1. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  2. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  3. From conv_new_question: Write database code so its **memory use is bounded by the work in progress**, then optimize the queries that dominate real workloads. “Memory-safe” can mean two things: preventing bugs such as use-after-free and data races, and avoiding application or database memory exhaustion. Good design addresses both.

### Keep memory bounded
- **Paginate or stream results.** Don’t load an entire large collection or result set into memory. Fetch a limited page, process it, then continue.
- **Project only needed fields.** Large blobs and unused columns add allocation and transfer costs.
- **Batch writes** in a measured, bounded size. Oversized batches can spike memory and increase latency; tiny batches can waste round trips.
- **Put limits on inputs and work:** page size, payload size, concurrency, and queued jobs. Apply backpressure instead of letting work accumulate in unbounded queues.
- **Release resources promptly.** Close cursors, transactions, connections, and file/blob handles using your language’s scope-based cleanup (`defer`, `using`, RAII, etc.). Don’t keep references to large records longer than necessary.

### Use safe concurrency and ownership
Prefer a memory-safe language and its standard ownership or garbage-collection mechanisms. Avoid raw pointers and shared mutable state unless profiling shows a real need. If multiple tasks share a connection pool or cache, use synchronization primitives correctly—and bound the number of concurrent tasks. For example, 1,000 concurrent queries can exhaust memory even if each query is individually small.

### Make queries do less work
- Avoid N+1 patterns: fetch related data in batches or through an appropriate join/API operation.
- Filter early and use indexes that match common filters and sort orders.
- Use query-plan tools and representative data to check whether a query scans too much.
- Prefer keyset pagination for large, changing datasets when offset-based pagination becomes expensive.
- Avoid repeated conversions, copies, and serialization of large values in hot paths.

### Handle failures safely
Use parameterized query APIs rather than building query strings from input. Keep transactions short, roll them back on error, and make retries safe—especially for writes. Set timeouts and define what happens when a database or downstream service is slow.

### Measure, then tune
Profile memory allocations, peak resident memory, query latency, and throughput under realistic load. Test large result sets, slow queries, retries, and concurrent requests. Change batch sizes, pool sizes, or indexes based on those measurements rather than guesses.

A useful rule of thumb: **each request should have a predictable upper bound on rows, bytes, and concurrent work**. In ekoDB, the same principles translate to using query limits and field projection, explicit schemas, indexes for frequent filters, and bounded batches or stored functions for repeatable operations.

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
    ✓ Generated embedding: 1536 dimensions in 0.247s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.233s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.296s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.262s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.265s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.215s
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
    ✓ Generated embedding: 1536 dimensions in 0.243s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.324s
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
    ✓ Generated embedding: 1536 dimensions in 0.272s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.220s
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
    ✓ Generated embedding: 1536 dimensions in 0.228s
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages_py
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.029s

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

Write database code so memory use is **bounded and predictable**, then optimize based on measurements.

### Keep memory use safe

- **Prefer managed or ownership-safe APIs.** In Rust, use owned values and borrowing carefully; in C/C++, use RAII, smart pointers, and containers rather than manual `malloc`/`free` where possible.
- **Bound every result set.** Use limits and keyset pagination instead of loading an entire table into memory. For large exports or scans, process rows as a stream or in fixed-size batches.
- **Avoid unnecessary copies.** Select only needed columns, reuse buffers when appropriate, and pass references or slices where the API permits.
- **Make resource lifetimes explicit.** Close cursors and transactions promptly; use context managers, RAII, or `defer`-style cleanup. Don’t retain database rows or large buffers longer than needed.
- **Validate input sizes.** Cap page sizes, batch sizes, query lengths, and payload sizes to prevent accidental or malicious memory spikes.

### Make it fast without wasting resources

- **Use parameterized queries** and reuse prepared statements where supported.
- **Fetch only what you need.** Avoid `SELECT *` when a few fields suffice.
- **Index for actual query patterns.** Index columns used frequently in filters, joins, and ordering—but avoid excessive indexes, which increase write and storage costs.
- **Avoid N+1 queries.** Fetch related records in batches or use an appropriate join.
- **Batch writes** when safe, but keep batches bounded; very large batches can increase memory use, lock time, and recovery costs.
- **Use transactions deliberately.** Group related changes for consistency, and keep transactions short to reduce lock contention.
- **Use connection pooling** for services, and set sensible pool limits and timeouts.

### Measure, then tune

Profile both the application and database. Track query latency, rows scanned versus returned, allocation rate, peak memory, and connection-pool waits. Inspect query plans, add or adjust indexes based on evidence, and test with realistic data volumes and concurrency.

A useful rule of thumb: **stream large reads, batch bounded writes, keep transactions short, and never optimize a query without checking its plan and measurements.** The best implementation details depend on your language and database; if you share those, I can give a concrete example.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.252s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2385 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.196s
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
    ✓ Generated embedding: 1536 dimensions in 0.275s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.265s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.243s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.259s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.273s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.270s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.213s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.314s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.243s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.200s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.266s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.258s
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
    ✓ Generated embedding: 1536 dimensions in 0.253s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages_ts
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.030s
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

Write database code so **memory use, work per request, and concurrency are all bounded**—then measure and optimize the actual bottleneck.

- **Fetch only what you need.** Select specific fields, filter on the server, and set a limit. Avoid loading whole collections or large blobs just to use a few fields. In ekoDB, use `select_fields` and a bounded `limit` with `query_collection`.
- **Process large result sets in pages.** Keep one page in memory, process it, then fetch the next. Prefer keyset pagination—“records after the last seen ID or timestamp”—over large offsets when your query supports it.
- **Avoid N+1 queries.** Don’t issue a query for every row in a result. Fetch related data in batches or redesign the query/API so the server does more of the work.
- **Index the access path, not every field.** Index fields used frequently for filters and sorting. Check the query plan first; in ekoDB, `query_explain` shows whether a query scans or uses an index and may recommend indexes.
- **Keep batches and concurrency bounded.** Use modest batch sizes, cap simultaneous queries, and apply backpressure when the database or downstream service is slow. Unbounded queues and “fetch everything, then process” designs can exhaust memory under load.
- **Make resource ownership explicit.** In C++, prefer RAII and smart pointers over unmanaged ownership; in Rust, let ownership and borrowing govern lifetimes. Release cursors, connections, statements, and temporary buffers promptly. Don’t retain references into buffers after their owner is gone.
- **Keep transactions short.** Do work that doesn’t require the transaction before or after it. Long transactions can hold locks and resources and increase contention.
- **Use parameterized queries and validate bounds.** This avoids injection and lets you enforce maximum page sizes, batch sizes, and payload sizes.
- **Measure before tuning.** Track query latency, rows and bytes returned, allocation rate, peak memory, and contention. Benchmark with realistic data and concurrent load; a query that is fast on a tiny dataset may not scale.

A useful request pattern is: **validate the request → query a bounded, projected page → process it without accumulating every page → return or persist a bounded result**. That usually improves both memory safety and throughput without relying on fragile micro-optimizations.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.253s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2368 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.210s
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
  ✓ Text search completed in 0.023s
✓ Found 3 messages mentioning ownership:
  1. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  2. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

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
    ✓ Generated embedding: 1536 dimensions in 0.256s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.273s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.255s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.254s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.385s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.273s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.233s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.302s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.290s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.235s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.253s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.303s
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
    ✓ Generated embedding: 1536 dimensions in 0.230s
    • Function auto-cleaned up by client

→ Executing HybridSearch()...
  • Collection: rag_messages_go
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.023s

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

Memory safety and database performance are related but separate concerns: keep both **application memory use** and **database work** bounded.

### In application code
- Prefer a memory-safe language and its standard database driver. Use safe abstractions instead of raw pointers or manual allocation.
- Scope connections, cursors, statements, and transactions so they’re reliably closed—even on errors. Use RAII, `defer`, `using`, or equivalent.
- Bound inputs and outputs: cap request sizes, batch sizes, result counts, and concurrency. Don’t load an entire large table into memory.
- Stream large results or process them in bounded batches. Avoid building many intermediate copies of large records.
- Use parameterized queries. This prevents injection and often lets the database reuse query plans.
- Set timeouts and cancellation; avoid unbounded queues of work when the database slows down.

### In database access
- Fetch only the columns and rows you need; use pagination. For large or changing datasets, prefer **keyset pagination** over large `OFFSET`s.
- Avoid N+1 queries. Fetch related data in a join or a deliberate batch where appropriate.
- Batch writes when possible, but keep batches bounded and transactions short. Don’t hold a transaction open while doing network calls or other slow work.
- Use a connection pool sized for your workload; more connections are not automatically faster.
- Add indexes for real filter, join, and sort patterns. Indexes speed reads but cost memory, disk, and write time—measure before adding them.
- Check query plans and benchmark with representative data. Fix slow queries based on evidence, not intuition.

### A practical loop
1. Set memory, concurrency, and query-size limits.
2. Measure latency, allocation rate, peak memory, and database load.
3. Inspect slow queries and their execution plans.
4. Optimize the largest measured cost.
5. Re-test under realistic concurrency and failure conditions.

If you’re using ekoDB, `query_explain` can show whether a query scans or uses an index; `collection_info` shows existing indexes. For language-specific code, tell me the language and database driver.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.231s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2164 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.286s
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
  ✓ Text search completed in 0.021s

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
    ✓ Generated embedding: 1536 dimensions in 0.31s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.222s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.229s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.229s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.235s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.236s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.246s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.221s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.279s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.263s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.228s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.252s
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
    ✓ Generated embedding: 1536 dimensions in 0.233s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages_kt
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.035s

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
    ✓ Generated embedding: 1536 dimensions in 0.206s
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
  ✓ Text search completed in 0.009s

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

> @ekodb/ekodb-client@0.32.0 prepare
> npm run build


> @ekodb/ekodb-client@0.32.0 build
> tsc


up to date, audited 44 packages in 629ms

12 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
npm warn install-scripts 1 package has install scripts not yet covered by allowScripts:
npm warn install-scripts   fsevents@2.3.3 (install: (install scripts present))
npm warn install-scripts
npm warn install-scripts Run `npm install-scripts ls` to review, or `npm install-scripts approve <pkg>` to allow.

> @ekodb/ekodb-client@0.32.0 build
> tsc

✅ TypeScript client built!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Running TypeScript SWR Examples...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_ts_9467_1794157605379 (4kA5TvFeBCGTK1eS1iCwbiRy-zOmOd3mSreKdWVX-gC36RI2eGkWuFAp8j4TqAcELOYKh1xce-BRnVg8KLDMxQ)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "value": {
          "address": {
            "zipcode": "92998-3874",
            "city": "Gwenborough",
            "street": "Kulas Light",
            "suite": "Apt. 556",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            }
          },
          "website": "hildegard.org",
          "id": 1,
          "email": "Sincere@april.biz",
          "company": {
            "bs": "harness real-time e-markets",
            "name": "Romaguera-Crona",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "phone": "1-770-736-8031 x56442",
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
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 1ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "address": {
            "zipcode": "92998-3874",
            "city": "Gwenborough",
            "street": "Kulas Light",
            "suite": "Apt. 556",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            }
          },
          "website": "hildegard.org",
          "id": 1,
          "email": "Sincere@april.biz",
          "company": {
            "bs": "harness real-time e-markets",
            "name": "Romaguera-Crona",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "phone": "1-770-736-8031 x56442",
          "username": "Bret",
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
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_reviews_ts_9467_1794157605379 (lHcrpcPDoRl9S5bGOh3AaizjUM15ARbLSiVV6qHmTTF5hGHGXLv4p7Zt4uuTcX7xuFC3VrykyMvxDQ5q1FJ-FQ)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "meta": {
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
            "barcode": "5784719087687",
            "updatedAt": "2026-05-23T11:27:41.868Z",
            "createdAt": "2025-10-09T14:47:01.588Z"
          },
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "dimensions": {
            "height": 13.08,
            "depth": 22.99,
            "width": 15.14
          },
          "minimumOrderQuantity": 48,
          "discountPercentage": 10.48,
          "price": 9.99,
          "returnPolicy": "No return policy",
          "shippingInformation": "Ships in 3-5 business days",
          "stock": 99,
          "reviews": [
            {
              "comment": "Would not recommend!",
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "reviewerName": "Eleanor Collins",
              "rating": 3
            },
            {
              "reviewerName": "Lucas Gordon",
              "date": "2025-04-30T09:41:02.053Z",
              "comment": "Very satisfied!",
              "rating": 4,
              "reviewerEmail": "lucas.gordon@x.dummyjson.com"
            },
            {
              "comment": "Highly impressed!",
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "rating": 5,
              "reviewerName": "Eleanor Collins"
            }
          ],
          "rating": 2.56,
          "id": 1,
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "title": "Essence Mascara Lash Princess",
          "weight": 4,
          "availabilityStatus": "In Stock",
          "category": "beauty",
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "tags": [
            "beauty",
            "mascara"
          ],
          "brand": "Essence",
          "sku": "BEA-ESS-ESS-001",
          "warrantyInformation": "1 week warranty"
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
✓ Edge cache script created: Jz_6cEpKRlftUm7Vfy5KuE7hxj9H1xEY3qJW0NUQ0mA6lrgoGlioPM2QaJegFmSZC31_UoVio_Ix-V_-68c91g

Call 1: Cache miss (fetches from API)
Response time: 50ms
Result: {
  "records": [
    {
      "value": {
        "value": {
          "username": "Bret",
          "address": {
            "zipcode": "92998-3874",
            "street": "Kulas Light",
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "suite": "Apt. 556"
          },
          "id": 1,
          "name": "Leanne Graham",
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "bs": "harness real-time e-markets",
            "name": "Romaguera-Crona"
          },
          "email": "Sincere@april.biz",
          "website": "hildegard.org",
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
Response time: 2ms (25x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "username": "Bret",
          "address": {
            "zipcode": "92998-3874",
            "street": "Kulas Light",
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "suite": "Apt. 556"
          },
          "id": 1,
          "name": "Leanne Graham",
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "bs": "harness real-time e-markets",
            "name": "Romaguera-Crona"
          },
          "email": "Sincere@april.biz",
          "website": "hildegard.org",
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
🐍 Found CPython 3.15 at /Users/tek/.local/bin/python3
🔗 Found pyo3 bindings with abi3-py3.8 support
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.09s
📦 Built wheel for abi3 Python ≥ 3.8 to ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.32.0-cp38-abi3-macosx_11_0_arm64.whl
📦 Installing Python wheel into .venv...
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.32.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.32.0
    Uninstalling ekodb_client-0.32.0:
      Successfully uninstalled ekodb_client-0.32.0
Successfully installed ekodb-client-0.32.0
🧪 Ensuring test dependencies (pytest) in .venv...
✅ Python client package built and installed!
📦 Ensuring Python example dependencies in .venv...

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Running Python SWR Examples...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user_swr_py (AYB7GVGDlii5FcxFTc7eVXl1okMo7hAnArmP8zsasd3pfy8-YKsGUmp6E4GV62tQKFd0C3S0oyWcwbZ9DuI3qA)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 183ms
Result: [
  {
    "cached_at": {
      "type": "String",
      "value": "1794157608"
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
        "followers": 331157,
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
Response time: 2ms (89.8x faster!)
Result: [
  {
    "cached_at": {
      "type": "String",
      "value": "1794157608"
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
        "followers": 331157,
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
✓ Created enrichment script: fetch_product_enriched_swr_py (EI-ubNRDtV7QvkBZ-KMevKBH27ZiJc4QQPgiDEx5gJch7TfXVfvE76OPBTwGdpIfY6u7Y6v73feUh1T7-evuyg)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "enriched_at": {
      "type": "String",
      "value": "1794157608"
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
✓ Created SWR script: fetch_github_user (FyC_U73bTNfq_gNl2NJmcakbxqK4hqQ_7CuwWHbg1It5Y70nl74RizPfZphMJ8DD-0YuOGTOBKCHTNfKo-_sKg)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 92.814917ms
Result: [
  {
    "cached_at": {
      "type": "DateTime",
      "value": "2026-11-08T17:06:49+00:00"
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
        "followers": 331157,
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
Response time: 630.667µs (+Infx faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (kQO_uoA39MvWXmb9FcrmVBtDu_uz_9ZOBIVE9_e6IFrubAwuuPhzRFCmzUeRH8R-RDmyUOSOnMnYfXm3rBuLQQ)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "enriched_at": {
      "type": "DateTime",
      "value": "2026-11-08T17:06:49+00:00"
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.21s
✅ Client build complete!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Running Rust SWR Examples...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
   Compiling ekodb-examples v0.1.0 (ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.43s
     Running `target/debug/examples/swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (nRxT8wtPCn2ra0nE6SUqPe3qr0oEkTU1y_3VxZaaamEAbHq4DhNcPwxkAdPuKwaYPRSKhvIhGRu7D0AbnKe9xg)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 71ms
Result: {
  "data": {
    "type": "Object",
    "value": {
      "organizations_url": "https://api.github.com/users/torvalds/orgs",
      "blog": "",
      "id": 1024025,
      "url": "https://api.github.com/users/torvalds",
      "user_view_type": "public",
      "twitter_username": null,
      "followers": 331157,
      "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
      "followers_url": "https://api.github.com/users/torvalds/followers",
      "events_url": "https://api.github.com/users/torvalds/events{/privacy}",
      "hireable": null,
      "public_repos": 12,
      "node_id": "MDQ6VXNlcjEwMjQwMjU=",
      "type": "User",
      "updated_at": "2026-11-01T20:12:14Z",
      "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
      "html_url": "https://github.com/torvalds",
      "login": "torvalds",
      "site_admin": false,
      "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
      "company": "Linux Foundation",
      "public_gists": 1,
      "received_events_url": "https://api.github.com/users/torvalds/received_events",
      "bio": null,
      "created_at": "2011-09-03T15:26:22Z",
      "email": null,
      "following": 0,
      "gravatar_id": "",
      "name": "Linus Torvalds",
      "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
      "location": "Portland, OR",
      "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
      "repos_url": "https://api.github.com/users/torvalds/repos"
    }
  }
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 2ms (35.5x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (IDuXKNCtLmht3ksZ4h2dihx4fCm6390_hzaRqNij-UGOq5TgOjGHnuU9LKNIFYsn2cTqtWs3R-jjm7tUDw4Ypw)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: {
  "enriched_data": {
    "type": "Object",
    "value": {
      "warrantyInformation": "1 week warranty",
      "dimensions": {
        "depth": 22.99,
        "height": 13.08,
        "width": 15.14
      },
      "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
      "images": [
        "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
      ],
      "id": 1,
      "category": "beauty",
      "minimumOrderQuantity": 48,
      "brand": "Essence",
      "price": 9.99,
      "stock": 99,
      "meta": {
        "barcode": "5784719087687",
        "updatedAt": "2026-05-23T11:27:41.868Z",
        "createdAt": "2025-10-09T14:47:01.588Z",
        "qrCode": "https://cdn.dummyjson.com/public/qr-code.png"
      },
      "discountPercentage": 10.48,
      "shippingInformation": "Ships in 3-5 business days",
      "title": "Essence Mascara Lash Princess",
      "weight": 4,
      "reviews": [
        {
          "reviewerName": "Eleanor Collins",
          "date": "2025-04-30T09:41:02.053Z",
          "comment": "Would not recommend!",
          "rating": 3,
          "reviewerEmail": "eleanor.collins@x.dummyjson.com"
        },
        {
          "reviewerName": "Lucas Gordon",
          "rating": 4,
          "date": "2025-04-30T09:41:02.053Z",
          "comment": "Very satisfied!",
          "reviewerEmail": "lucas.gordon@x.dummyjson.com"
        },
        {
          "reviewerEmail": "eleanor.collins@x.dummyjson.com",
          "comment": "Highly impressed!",
          "rating": 5,
          "date": "2025-04-30T09:41:02.053Z",
          "reviewerName": "Eleanor Collins"
        }
      ],
      "rating": 2.56,
      "returnPolicy": "No return policy",
      "sku": "BEA-ESS-ESS-001",
      "availabilityStatus": "In Stock",
      "tags": [
        "beauty",
        "mascara"
      ],
      "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp"
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
✓ Created SWR function: swr_fetch_github_user_kt_1794157628248 (MxPcFEkKakUyryER7Z12h4olZl_54VP5Hs9zwabkMJg5z5pKUycRBpy0sf9gKbt9pJ4u9oA3PNkK2j52MDSYSA)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 117ms
Result: [{"data":{"value":{"following":0,"company":"Linux Foundation","organizations_url":"https://api.github.com/users/torvalds/orgs","public_gists":1,"avatar_url":"https://avatars.githubusercontent.com/u/1024025?v=4","followers":331157,"location":"Portland, OR","node_id":"MDQ6VXNlcjEwMjQwMjU=","subscriptions_url":"https://api.github.com/users/torvalds/subscriptions","id":1024025,"url":"https://api.github.com/users/torvalds","email":null,"gists_url":"https://api.github.com/users/torvalds/gists{/gist_id}","events_url":"https://api.github.com/users/torvalds/events{/privacy}","following_url":"https://api.github.com/users/torvalds/following{/other_user}","type":"User","hireable":null,"gravatar_id":"","twitter_username":null,"starred_url":"https://api.github.com/users/torvalds/starred{/owner}{/repo}","user_view_type":"public","followers_url":"https://api.github.com/users/torvalds/followers","html_url":"https://github.com/torvalds","login":"torvalds","created_at":"2011-09-03T15:26:22Z","blog":"","received_events_url":"https://api.github.com/users/torvalds/received_events","site_admin":false,"updated_at":"2026-11-01T20:12:14Z","public_repos":12,"repos_url":"https://api.github.com/users/torvalds/repos","bio":null,"name":"Linus Torvalds"},"type":"Object"},"id":"torvalds","cached_at":{"value":"1794157628257","type":"String"}}]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 5ms
Cache hit was 23.4x faster!

✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment function: swr_fetch_product_kt_1794157628248 (Fdg6ZYm7pZ_wslwkxeNERxZaxniG3-b_NFspayy_dGD4xAJ0jv-oSFPZ90damRq7UKbtU6sXCeV-HyaCASYWhA)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [{"enriched_at":{"value":"1794157628504","type":"String"},"id":"1","enriched_data":{"type":"Object","value":{"minimumOrderQuantity":48,"title":"Essence Mascara Lash Princess","rating":2.56,"warrantyInformation":"1 week warranty","id":1,"description":"The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.","reviews":[{"reviewerEmail":"eleanor.collins@x.dummyjson.com","rating":3,"date":"2025-04-30T09:41:02.053Z","comment":"Would not recommend!","reviewerName":"Eleanor Collins"},{"rating":4,"comment":"Very satisfied!","reviewerEmail":"lucas.gordon@x.dummyjson.com","reviewerName":"Lucas Gordon","date":"2025-04-30T09:41:02.053Z"},{"date":"2025-04-30T09:41:02.053Z","reviewerName":"Eleanor Collins","comment":"Highly impressed!","rating":5,"reviewerEmail":"eleanor.collins@x.dummyjson.com"}],"thumbnail":"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp","weight":4,"availabilityStatus":"In Stock","meta":{"barcode":"5784719087687","qrCode":"https://cdn.dummyjson.com/public/qr-code.png","createdAt":"2025-10-09T14:47:01.588Z","updatedAt":"2026-05-23T11:27:41.868Z"},"shippingInformation":"Ships in 3-5 business days","discountPercentage":10.48,"category":"beauty","dimensions":{"height":13.08,"width":15.14,"depth":22.99},"returnPolicy":"No return policy","stock":99,"sku":"BEA-ESS-ESS-001","brand":"Essence","tags":["beauty","mascara"],"images":["https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"],"price":9.99}}}]
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
✅ Client build complete!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Running Rust Function Composition Examples...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
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
   ⏱️  Duration: 69.750209ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "company": {
        "catchPhrase": "Multi-layered client-server neural-net",
        "bs": "harness real-time e-markets",
        "name": "Romaguera-Crona"
      }...

Second call (cache hit - from cache):
   ⏱️  Duration: 6.107ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "id": 1,
      "phone": "1-770-736-8031 x56442",
      "company": {
        "bs": "harness real-time e-markets",
        "catchPhrase": "Multi-layered client-server...
   🚀 Cache speedup: 11.4x faster!

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

> @ekodb/ekodb-client@0.32.0 prepare
> npm run build


> @ekodb/ekodb-client@0.32.0 build
> tsc


up to date, audited 44 packages in 600ms

12 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
npm warn install-scripts 1 package has install scripts not yet covered by allowScripts:
npm warn install-scripts   fsevents@2.3.3 (install: (install scripts present))
npm warn install-scripts
npm warn install-scripts Run `npm install-scripts ls` to review, or `npm install-scripts approve <pkg>` to allow.

> @ekodb/ekodb-client@0.32.0 build
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
   ⏱️  Duration: 56ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "username": "Bret",
      "id": 1,
      "phone": "1-770-736-8031 x56442",
      "website": "hildegard.org",
      "email": "Sincere@april.biz...

Second call (cache hit - from cache):
   ⏱️  Duration: 1ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "username": "Bret",
      "id": 1,
      "phone": "1-770-736-8031 x56442",
      "website": "hildegard.org",
      "email": "Sincere@april.biz...
   🚀 Cache speedup: 56.0x faster!

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
    Finished `release` profile [optimized] target(s) in 0.09s
📦 Built wheel for abi3 Python ≥ 3.8 to ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.32.0-cp38-abi3-macosx_11_0_arm64.whl
📦 Installing Python wheel into .venv...
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.32.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.32.0
    Uninstalling ekodb_client-0.32.0:
      Successfully uninstalled ekodb_client-0.32.0
Successfully installed ekodb-client-0.32.0
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
   ⏱️  Duration: 52.4ms
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
   ⏱️  Duration: 1.4ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "type": "Object",
            "value": {
                  "address": {
                        "city": "Gwenborough",
                        "geo": {
                 ...
   🚀 Cache speedup: 37.4x faster!

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
   ⏱️  Duration: 59.203667ms
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
   ⏱️  Duration: 607µs
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
   Name: {"type":"String","value":"User 1"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 52ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "username": "Bret",
      "company": {
        "name": "Romaguera-Crona",
        "bs": "harness real-time e-markets",
        "catchPhrase": ...

Second call (cache hit - from cache):
   ⏱️  Duration: 1ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "username": "Bret",
      "company": {
        "name": "Romaguera-Crona",
        "bs": "harness real-time e-markets",
        "catchPhrase": "Multi-layered client-...
   🚀 Cache speedup: 52.0x faster!

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
✅ Client build complete!

🦀 Rust WebSocket Subscription Test...
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_websocket_subscribe`
✓ Authentication successful

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_rs' ===
✓ Subscribed (subscription_id: sub_9f08c1522791464d820bdd4dfd170b62)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: rWiY8cp-m-Qj8jZGmzeVvg83ARsuUHV2ci5zQ0tbV-Io-p0Z3r1Fu302UJIRo5FbcMjlXa0v1ZmT8rTrGt1AIw

  📡 Notification received:
     Event:      "insert"
     Collection: "ws_subscribe_example_rs"
     Record IDs: ["rWiY8cp-m-Qj8jZGmzeVvg83ARsuUHV2ci5zQ0tbV-Io-p0Z3r1Fu302UJIRo5FbcMjlXa0v1ZmT8rTrGt1AIw"]
     Timestamp:  "2026-11-08T17:07:43.751228+00:00"

Inserting record 2...
✓ Inserted: tDrT1kur4YmmM9gLSQsFahZVMQYhBchTxouzw2w9V3SyEPYU5dNIxggazncu_umLqQ4oUtiQT5LWw0rN6jmJlA

  📡 Notification received:
     Event:      "insert"
     Record IDs: ["tDrT1kur4YmmM9gLSQsFahZVMQYhBchTxouzw2w9V3SyEPYU5dNIxggazncu_umLqQ4oUtiQT5LWw0rN6jmJlA"]

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
✓ Subscribed (subscription_id: sub_d18bc86ea8da45faa6e5f52ca016ffd4)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: 4qVGurFIHF1UcjoAufFF0QdVZLlkicaY73YzXVN6yaW76tPYj3uj8X6lEsV9YY4v425Xw_B1IQS9Jg2z_GM22w

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_go
     Record IDs: [4qVGurFIHF1UcjoAufFF0QdVZLlkicaY73YzXVN6yaW76tPYj3uj8X6lEsV9YY4v425Xw_B1IQS9Jg2z_GM22w]
     Timestamp:  2026-11-08T17:07:44.098139+00:00

Inserting record 2...
✓ Inserted: JpQmtcBTVMDEfM18TNpN4cqzR9kX5vVDc1GubSc0JXxghP4gC26a_vGjOVrMMUCJhcC_aJcZ6SSBcsPnZeFcpg

  📡 Notification received:
     Event:      insert
     Record IDs: [JpQmtcBTVMDEfM18TNpN4cqzR9kX5vVDc1GubSc0JXxghP4gC26a_vGjOVrMMUCJhcC_aJcZ6SSBcsPnZeFcpg]

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
✓ Subscribed (subscription_id: sub_db8948e8b4d244aa911594d3bb5d8cea)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: c0Dem0UYBjj3AAKQSdgwgHzRDX-DlfdaLGpPb9gUrptUM2bZVx-KkL9d0FU0_E9p1catn4EVLR88y9zaI259ZQ

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_py
     Record IDs: c0Dem0UYBjj3AAKQSdgwgHzRDX-DlfdaLGpPb9gUrptUM2bZVx-KkL9d0FU0_E9p1catn4EVLR88y9zaI259ZQ
     Timestamp:  2026-11-08T17:07:44.764073+00:00

Inserting record 2...
✓ Inserted: dgc76vMrbFWQJSJZgPqOehKPUOH4sBRvNoaFb85NfIer_zEQc7ROmVrgWrId7UKNYudateihlfX3lfk6KxzJLg

  📡 Notification received:
     Event:      insert
     Record IDs: dgc76vMrbFWQJSJZgPqOehKPUOH4sBRvNoaFb85NfIer_zEQc7ROmVrgWrId7UKNYudateihlfX3lfk6KxzJLg

=== Unsubscribing ===
✓ Unsubscribed: {'collection': 'ws_subscribe_example_py', 'found': True, 'unsubscribed': True}

=== Cleanup ===
✓ Deleted collection 'ws_subscribe_example_py'

✓ WebSocket subscription example completed successfully
✅ Python subscription test complete!
📦 Building TypeScript client library...

> @ekodb/ekodb-client@0.32.0 prepare
> npm run build


> @ekodb/ekodb-client@0.32.0 build
> tsc


up to date, audited 44 packages in 585ms

12 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
npm warn install-scripts 1 package has install scripts not yet covered by allowScripts:
npm warn install-scripts   fsevents@2.3.3 (install: (install scripts present))
npm warn install-scripts
npm warn install-scripts Run `npm install-scripts ls` to review, or `npm install-scripts approve <pkg>` to allow.

> @ekodb/ekodb-client@0.32.0 build
> tsc

✅ TypeScript client built!

📘 TypeScript WebSocket Subscription Test...
=== WebSocket Subscription Example ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_ts' ===
✓ Subscribed (subscription_id: sub_5db000abced547a5ab8aec526f5ed0ea)

=== Performing mutations to trigger notifications ===
Inserting a record...
✓ Inserted record: Y_TNfmleLyDfEx06q58VW_Urrja4OaMFaWlUJcFAHhoqvkrUdaP8hAUGsOQbuZwA6Li0u5N2-fd0zPYvZsbXUw
  📡 Notification received for Y_TNfmleLyDfEx06q58VW_Urrja4OaMFaWlUJcFAHhoqvkrUdaP8hAUGsOQbuZwA6Li0u5N2-fd0zPYvZsbXUw

Inserting another record...
✓ Inserted record: tquBLMA4421px353dO7IbYAgd0zvxlpw9LK3MwQZ833p0BiFX7WFVTrqt2hVQgb_F0a_OP0R3O-gBi80BEC9_w
  📡 Notification received for tquBLMA4421px353dO7IbYAgd0zvxlpw9LK3MwQZ833p0BiFX7WFVTrqt2hVQgb_F0a_OP0R3O-gBi80BEC9_w

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
✓ Subscribed (subscription_id: sub_d437e8604bc8417f8b8511219745da69)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: "erkZkRXVIQL4cqPdaacChOkzFy8XTgmUjj50jE9FdEjkTBsikkwnMhopyLUP45PO1EzFE6u-wsx65sVZUEyFcw"

  📡 Notification received:
     Event:      "insert"
     Collection: "ws_subscribe_example_kt"
     Record IDs: ["erkZkRXVIQL4cqPdaacChOkzFy8XTgmUjj50jE9FdEjkTBsikkwnMhopyLUP45PO1EzFE6u-wsx65sVZUEyFcw"]
     Timestamp:  "2026-11-08T17:07:51.661667+00:00"

Inserting record 2...
✓ Inserted: "H2eo-G59GaM_FUYmR5KQEw8srucSlsFjJEZFf7T6mBe2r9zV0EWWKy3RT5BuQ-ZBvO-Z0_To-4KnY0yUS69FzA"

  📡 Notification received:
     Event:      "insert"
     Record IDs: ["H2eo-G59GaM_FUYmR5KQEw8srucSlsFjJEZFf7T6mBe2r9zV0EWWKy3RT5BuQ-ZBvO-Z0_To-4KnY0yUS69FzA"]

=== Unsubscribing ===
✓ Unsubscribed: {"payload":{"data":{"collection":"ws_subscribe_example_kt","found":true,"unsubscribed":true}},"type":"Success"}

✓ WebSocket subscription example completed successfully
✓ Deleted collection 'ws_subscribe_example_kt'

BUILD SUCCESSFUL in 6s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.8.1/userguide/configuration_cache_enabling.html
✅ Kotlin subscription test complete!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ All WebSocket Subscription Tests Passed!
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
