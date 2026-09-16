make test-examples
Composed from the latest per-language reports after focused reruns; no all-language rerun was performed.

🧪 Running Rust examples (direct HTTP/WebSocket)...
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.33s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("Z0h1T97ETrikMhyfrWmHW8CGIlga_7zb1AoCAb-IZxfIDLtFyUrYE-yMojia-QX81j2TbZYGK4C9gv6GQAij4A")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("Z0h1T97ETrikMhyfrWmHW8CGIlga_7zb1AoCAb-IZxfIDLtFyUrYE-yMojia-QX81j2TbZYGK4C9gv6GQAij4A"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("Z0h1T97ETrikMhyfrWmHW8CGIlga_7zb1AoCAb-IZxfIDLtFyUrYE-yMojia-QX81j2TbZYGK4C9gv6GQAij4A"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("Z0h1T97ETrikMhyfrWmHW8CGIlga_7zb1AoCAb-IZxfIDLtFyUrYE-yMojia-QX81j2TbZYGK4C9gv6GQAij4A"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: BuJyoP-A5m_zxZfcrNSPehBg5psmQyacvEQ9RYr-DyEzrJnDC4Rq0o-CIG9JMcIEzIgr2lOmIKiXZmhlV8nPRA

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1789563129210903000",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "BuJyoP-A5m_zxZfcrNSPehBg5psmQyacvEQ9RYr-DyEzrJnDC4Rq0o-CIG9JMcIEzIgr2lOmIKiXZmhlV8nPRA",
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/collection_management`
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: "s8PLj482_CytEmrNSk0Jhd0q5DjzX4GjBKncwS78rKtnvt4MIax1MoWUsKWS4xQ0q919Pxqd53KsMNirGEdPzw"

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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/document_ttl`
=== ekoDB Document TTL Example (Rust) ===
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: 0bITwwl4DvHD_-g-uTNg7IsNozmnwx7XCXEpKbNG2sJ1zP41N6vL-kGJh7kP-zB3RlObGguD1kEviC6082Bxzw

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: 5nDApqj6-uzFa3XZFVbOzzlTuw-6rO9zJb_aDCfJT9-_q3YIeP752xSGTusU9ZvDL4IVTF1TVXGEdntnIfzGBQ

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: b58e_ckLprMvLjNKXN7-xjQQjClijzdx_2yWGm2PSCli6QstLHEekacZTZcrsFVjKkrf2nhxnhdhjmkgycfTRw

=== Query Documents ===
✓ Found 3 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/websocket_ttl`
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: String("zDu__FXDajqDs532gfO9Kg7FwAs_5-QjAh4VfED83oMhr_cF5u_RIQew-mOJUE0wZ9CeM_-RxyNj7nrY_y3qkg")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record via WebSocket

Record 1:
  id: "zDu__FXDajqDs532gfO9Kg7FwAs_5-QjAh4VfED83oMhr_cF5u_RIQew-mOJUE0wZ9CeM_-RxyNj7nrY_y3qkg"
  name: {"type":"String","value":"WebSocket TTL Test"}
  ttl: "2026-09-16T13:56:53.289744Z"
  value: {"type":"Integer","value":42}

✓ WebSocket TTL example completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Functions Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: 6wquNjJM3_pMc66OQ0xt4MEdyF51oL10tMCAita4BHTYrxQTPYFKn7i0tCikvs2Hjdne0OOkrdQCPiwTs9PevA
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: DECwFOpsFqy4UyFjtPpeeXDKcGCkxLTuSxBi1dED9bJ9O3N_JiF_AyclBoqnxNNbeEl1xrq6YwTAJ9C7Uyv8dQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Function saved: eQCXq-sihZIS5lBS5zbH4jMOkTV0thINY4_YGK8gnBqwBq-bPsmIvQApPruG6WDbuS7MqX780AXQsxE5brkOYg
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: ySOzgrPnI6lVmsMpfUiFSiBcSYhbpZapQwn3vAGzZBDnQaJIb7ssnJK_OHI_WmxeemRpT7PwNh39TMnF8c28WQ
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/transactions`
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: gyYAeRRzaJ4678zHhc42jlmQKLNzZexCP9ZzrONE-9MmnmIz2RVP1ZD3lBmw2cgu21M-Y_kfrZysSfgR_r09jQ
Created Bob: $500 - ID: 2-YfTFXkA7CujBaxJU2CaaBYPYUkngjfei8laQIhIcNsYk82RY2Isq7nSqr3mLsKwFfo4AZSJhiqAT6y43ixBw

=== Example 1: Begin Transaction ===
Transaction ID: 4ce9a49a-eb18-4e5a-a86e-fc63b61c0102

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
New transaction: 83d7dcce-80a8-415f-b98a-6e0b997248c0
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"type":"Integer","value":700}

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✅ Rust direct examples complete!
🛠️  Building client library...
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.26s
✅ Client build complete!
bash: warning: setlocale: LC_ALL: cannot change locale (C.UTF-8): No such file or directory
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_advanced_crud`
=== ekoDB Advanced CRUD Example (Rust) ===

--- Inserting base document ---
Inserted: Widget Counter (id: S7eanqhlZAh9ei-K8vORXUHRv9s1UiT-pHzpDHA5MV0e9xh2IUIQvHQe5GtrABIDEzJ6tHBALZk6Lq4fY2FSEA)

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
tags after sequence: Some(Object({"type": String("Array"), "value": Array([String("database"), String("batch")])}))
label after sequence: Some(Object({"type": String("String"), "value": String("prefix-suffix-v2")}))

--- Cleanup ---
Deleted collection

=== All advanced CRUD operations completed ===
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: CJFDxZ7ySXKr2xupDkPpSbSRYCDuSrznhhHBEOINRPtnDwqtPJzrqLE9f9aqDiS4bAz0WuMoJrvgCT6dqgucUA

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you need more details or additional products, let me know!

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
✓ Created second session: x94-ex96XM6G9sItMBgu-Pjb2NywpJkANmMeY2K6Tpq-gQb3GbiVNUZoBhJZVzbFhvuj7j-wiNwVfFMQX6bHgA
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: x94-ex96XM6G9sItMBgu-Pjb2NywpJkANmMeY2K6Tpq-gQb3GbiVNUZoBhJZVzbFhvuj7j-wiNwVfFMQX6bHgA
✓ Deleted chat session: CJFDxZ7ySXKr2xupDkPpSbSRYCDuSrznhhHBEOINRPtnDwqtPJzrqLE9f9aqDiS4bAz0WuMoJrvgCT6dqgucUA
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_chat_basic`
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: ziikrS8SSoT8XnnY80XN31mOw9x_br3cXewzlYilC6aEZ4c1tim5bMT_bik9hZo0nWExqMmInxjfaW2MSvQ1wQ

=== Sending Chat Message ===
Message ID: i5slv-rqzJCkUz41uzrW6cObLiMsKJjSPOvPOHCvAHQP4t1qU0W0gAXN5mKVfuZSTj8MICWugo8y7rdlqDkKIg

=== AI Response ===
Response 1: ekoDB is a high-performance database designed with intelligent caching and real-time capabilities, along with AI integration.

### Key Features:
1. **AI Chat Integration**: The chat feature allows you to query your database using natural language, providing AI-powered responses with relevant context.

2. **Search Capabilities**:
   - **Full-text Search**: Enables querying of text data efficiently.
   - **Vector Search**: Allows for semantic searches using vector embeddings.
   - **Hybrid Search**: Combines both full-text and vector searching with automatic context retrieval.

These features make ekoDB a versatile and powerful tool for managing and querying data effectively.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["title", "category", "content"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("Uc5yGDp_U06VPAhiEg3yMiWtjiO54uT3nX9VRX9IsRgb3cVtdJ-G39BNEj3_GEezHIAATUu87ahen_cjUVFKSQ"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["category", "title", "content"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("lBEkOWr3s40PvtyTuxBXRoNgfzfe3JbYOcj_YLAuy6rhoKS6YYJbkXJxJ3QV4R0tIl64V-luACGAPJ5JDLiGpw"), "title": String("Search Features")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "content"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("-41IQXXmuqHyIFDphWjnE6RgU-9pXZVwH8ZWaFsTZ9gP5ezgxJzucB4rX4MruG2k03ec1wojjOi0GVUQmWFZkA"), "title": String("Introduction to ekoDB")}

Execution Time: 3470ms

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_chat_message_stream`
=== ekoDB Chat Message Stream (SSE) Example (Rust) ===

Created session: Yml_AosYcMSL_uitCWAmtTLFICgyIyBvbapxMNZ4YIs7zapOQQw-8XOKtAdv3-GEYCdL-y0XoTGEPw9Bcujuug

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}**ekoDB** is a publicly accessible, comprehensive database that integrates and organizes ecological knowledge, particularly data on how species interact with their environment (eco-evolutionary responses), traits, and biotic interactions. Its main features are:

- **Purpose**: To facilitate research in ecology and evolutionary biology by providing a centralized source for ecological response data, trait information, and species interactions.
- **Content**: Includes published and curated data on ecological traits (like growth rate, diet, or thermal tolerance), how those traits respond to environmental factors, and how species interact with one another (e.g., predation, mutualism).
- **Utility**: Helps researchers analyze ecological patterns, develop and test ecological models, and perform meta-analyses, especially in the context of global change or biodiversity studies.

ekoDB is similar in concept to other knowledge bases like TRY (for plant traits) and the Global Biotic Interactions database, but aims for broader integration, harmonizing different kinds of ecological data in one place.

**References:**
- Online documentation and database: [https://www.ekodb.org/](https://www.ekodb.org/)
- See also: Brodersen, J., et al. (2023). ekoDB: Ecological knowledge database for eco‐evolutionary analysis. *Ecology and Evolution*, 13(2), e9926. [https://doi.org/10.1002/ece3.9926](https://doi.org/10.1002/ece3.9926)

--- Stream complete ---
Message ID: 0yDQXyQxMRzbAvNUI9Madzn2KD7mVWFLIqV4GMM3uWS5rs1FiRoiR79g6ha2fUVJZEPSPgDHJpVPxyH9kTOWHQ
Execution time: 3743ms
Context window: 1000000 tokens

✓ Chat message stream example completed
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_chat_models`
✓ Client created

=== Get All Chat Models ===
Available chat models by provider:

OpenAI models (132):
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
  - gpt-image-2.5-flare
  - gpt-image-2.5-sunburst
  - gpt-image-2.5-flare-2026-09-08
  - gpt-image-2.5-sunburst-2026-09-08
  - gpt-live-1

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
  openai: ok 132 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get Models for Specific Provider ===

openai models (132):
  - text-embedding-ada-002
  - whisper-1
  - gpt-3.5-turbo
  - tts-1
  - gpt-3.5-turbo-16k
  ... and 127 more

anthropic models (11):
  - claude-fable-5-1
  - claude-opus-5
  - claude-sonnet-5
  - claude-fable-5
  - claude-opus-4-8
  ... and 6 more
GetChatModel(perplexity) error: Record not found

✓ Chat Models API example complete
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 6cWoPtMLf7xz-iAQKNNfe3zmkUQXC22ZTumfXp6Q8O9cDhtl66SOo_ap5Z8N9n7w3R9k64vMYa-8jKrFNlLqBw
=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Product Name:** ekoDB
- **Description:** A high-performance database product with AI capabilities
- **Price:** $99

If you need more details or additional products, let me know!

✓ Message 2 sent
  Response: The price of the product ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: nKqdZso8cfgChZmV3w8z57mPSc8cy4BFxf_sAun42DlayyjnUTqaj46SAC-drdJg72QKrM-dFkaOAibBOXuVUw
  Parent: 6cWoPtMLf7xz-iAQKNNfe3zmkUQXC22ZTumfXp6Q8O9cDhtl66SOo_ap5Z8N9n7w3R9k64vMYa-8jKrFNlLqBw

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: nKqdZso8cfgChZmV3w8z57mPSc8cy4BFxf_sAun42DlayyjnUTqaj46SAC-drdJg72QKrM-dFkaOAibBOXuVUw (Untitled)
  Session 2: 6cWoPtMLf7xz-iAQKNNfe3zmkUQXC22ZTumfXp6Q8O9cDhtl66SOo_ap5Z8N9n7w3R9k64vMYa-8jKrFNlLqBw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: nKqdZso8cfgChZmV3w8z57mPSc8cy4BFxf_sAun42DlayyjnUTqaj46SAC-drdJg72QKrM-dFkaOAibBOXuVUw

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_collection_management`
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "w48Ns5c78s8OAnW3jbjuNtUHCi9TPTJTpECeaey8Qn-3uxX2FxEJOvu0C_65fiTDRoLOYdSAhiKy9ZibXx4SDQ"

=== List Collections ===
Total collections: 7
Sample collections: ["agent_function_versions__ek0_testing", "functions__ek0_testing", "chat_configurations__ek0_testing", "client_collection_management_rust", "audit__ek0_testing"]

=== Count Documents ===
Document count: 1

=== Check Collection Exists ===
Collection exists: true

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
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
  - agent_function_versions__ek0_testing
  - functions__ek0_testing
  - chat_configurations__ek0_testing
  - audit__ek0_testing
  - collection_utils_test_rust <-- our test
  - chat_messages__ek0_testing
  - chat_turns__ek0_testing

=== Check Non-Existent Collection ===
Collection 'nonexistent_collection_xyz' exists: false

=== Cleanup ===
Deleted collection 'collection_utils_test_rust'

✓ Collection Utilities example complete
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_convenience_methods`
=== ekoDB Convenience Methods Example ===

=== Record Builder Pattern ===
✓ Created record with fluent builder: Record({"id": String("xHHO_SZO9GSvmZUbN-MSxUaMk8B6IEL9JmrEU3ID1qdAycZbY4cIr7BnzNSvHqJ9jWjnKqL5spB8XwBkGVHvGw")})

=== Upsert Operation ===
✓ First upsert (insert): Record({"id": String("JOS5Tj5n2ueYUtHz0JVW2gdLAvZqTdCOLGr9x0LHY-yoUKzk9fnHsm8QJnnrWmvPI5RCistOSyPlHpaeIK4_TQ")})
✓ Second upsert (update): Record({"id": String("JOS5Tj5n2ueYUtHz0JVW2gdLAvZqTdCOLGr9x0LHY-yoUKzk9fnHsm8QJnnrWmvPI5RCistOSyPlHpaeIK4_TQ"), "email": Object({"value": String("bob.smith@newdomain.com"), "type": String("String")}), "name": Object({"type": String("String"), "value": String("Bob Smith")}), "age": Object({"type": String("Integer"), "value": Integer(36)})})

=== Find One Operation ===
✓ Found user by email: Record({"name": Object({"value": String("Alice Johnson"), "type": String("String")}), "email": Object({"type": String("String"), "value": String("alice@example.com")}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "age": Object({"type": String("Integer"), "value": Integer(28)}), "id": String("xHHO_SZO9GSvmZUbN-MSxUaMk8B6IEL9JmrEU3ID1qdAycZbY4cIr7BnzNSvHqJ9jWjnKqL5spB8XwBkGVHvGw")})
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/examples/client_document_ttl`
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "Ppb-0tgxNy3qEKcTbd4CimSY4UN170faXhlEBm_yA7nyBNQhCB22kRgA_Kx5LE7zzeguqZ2Yk1yShmUsxNGjPQ"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("hk04mrw6-pAq4Zu33LOsbUNvrYNLGasZxWiIANhZDqSKxjhbJQfJa8o0zZNY_bJ5II-5uIRaTi_Dq5k9lIHCAg"))

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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_edge_cache`
=== ekoDB as Edge Cache - Simple Example ===

Setting up edge cache collection...
✓ Cache entry created

Creating edge cache lookup script...
✓ Edge cache script created: qIcr8gwEutVxrA-6MXDRU9T183eeC0GCmOA53_mIxc3IVoqFQlQlCzhcEm6a1aspbQ51lqnQj4o8BSxnU02fBQ

Call 1: Cache lookup
Response time: 7ms
Found 1 cached entries

Call 2: Cache lookup (connection warm)
Response time: 5ms
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
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
   ⏱️  Duration: 91.863875ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "email": "Sincere@april.biz",
      "name": "Leanne Graham",
      "id": 1,
      "address": {
        "zipcode": "92998-3874",
        "city"...

Second call (cache hit - from cache):
   ⏱️  Duration: 5.721584ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "email": "Sincere@april.biz",
      "phone": "1-770-736-8031 x56442",
      "name": "Leanne Graham",
      "username": "Bret",
      "address": {
        "geo": {
 ...
   🚀 Cache speedup: 16.1x faster!

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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_function_contract`
client_function_contract: ok
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_functions`
=== ekoDB Rust Client - Functions Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: PJ2MKyrqMIXnN0VzFBS18JByfCmKLkoBzdwyL6vmKUeW1Wk0buQG71iZj-D-a8W2VHtlVqS3TJ9xDkc9QFoO5g
📊 Found 5 active users

📝 Example 2: Parameterized Function

✅ Function saved: TVA2xrfxRMgG9MItEEnTiqbFrTRQYMVnG8aPuNrEtm-vBW3267aPK_TCbdzV5yzRYjW4ZuLnP7S46-IBH-DflQ
📊 Found 3 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: TsqE2h6QOwFDcmfmgLfXfUPi5QP4QafGXAMdDymkD_GfXe7vvVm0YXjmDWEkRMVwnFXteq2qUQMJxfgY63jugQ
📊 Statistics: 2 groups

📝 Example 4: Function Management

📋 Total functions: 3
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.


✅ All examples completed!
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
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
   Record({"category": Object({"type": String("String"), "value": String("Electronics")}), "count": Object({"type": String("Integer"), "value": Integer(5)}), "avg_price": Object({"type": String("Float"), "value": Float(367.0)})})
   Record({"avg_price": Object({"value": Float(365.6666666666667), "type": String("Float")}), "category": Object({"value": String("Furniture"), "type": String("String")}), "count": Object({"type": String("Integer"), "value": Integer(3)})})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_functions_ai`
🚀 ekoDB Rust AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer several benefits:

1. **Efficient Similarity Search**: They excel at finding similar items quickly using vector embeddings, ideal for tasks like recommendation and image retrieval.

2. **Scalability**: Designed to handle large datasets with high dimensionality, vector databases can scale horizontally with distributed computing.

3. **Real-time Performance**: They provide fast indexing and querying, making real-time applications feasible.

4. **Supports Complex Data Types**: They can efficiently manage unstructured data like images, text, and audio by converting them into vector representations.

5. **Advanced Algorithms**: Vector databases utilize sophisticated algorithms like Approximate Nearest Neighbors (ANN) to enhance search performance.

6. **Integration with Machine Learning**: They work well with machine learning workflows, allowing for easy storage and retrieval of feature vectors.

7. **Flexibility**: Many vector databases support various distance metrics, enabling tailored similarity searches for different applications.

8. **Reduced Latency**: They minimize the time taken to fetch related data, improving user experience in applications like chatbots or personalized content delivery.
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_functions_complete`
🚀 ekoDB Rust Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: NvSepuMRVP9sdxRUgTs98MWWz6zhycomTszJfyc4wvhjFdR5wl4Yd59s4rXFpFMVLmmoDtryDFJUWvTUXGPJag
📊 Found 2 product groups
   Record({"category": Object({"type": String("String"), "value": String("Electronics")}), "count": Object({"value": Integer(3), "type": String("Integer")}), "avg_price": Object({"value": Float(575.6666666666666), "type": String("Float")})})
   Record({"avg_price": Object({"type": String("Float"), "value": Float(474.0)}), "count": Object({"value": Integer(2), "type": String("Integer")}), "category": Object({"type": String("String"), "value": String("Furniture")})})
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   Record({"category": Object({"type": String("String"), "value": String("Furniture")}), "count": Object({"value": Integer(2), "type": String("Integer")})})
   Record({"count": Object({"value": Integer(3), "type": String("Integer")}), "category": Object({"value": String("Electronics"), "type": String("String")})})
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
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
   Record({"count": Object({"type": String("Integer"), "value": Integer(3)}), "status": Object({"value": String("inactive"), "type": String("String")})})
   Record({"status": Object({"value": String("active"), "type": String("String")}), "count": Object({"value": Integer(7), "type": String("Integer")})})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_functions_kv_wrapped`
🚀 ekoDB Rust KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: Some(String("r_PrrGWlWLADT630OIrkEM6wxT_GgGInaTrJSMfSxpc5v3YyQqXARxsotoHAcKBla-6azgU2cn9NwQE9R6F0AQ"))
✅ Inserted 2 products with wrapped types

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: MEjNoK5vk3ZVbNkgC3bg-ZbwegJ0BKsPhnYFJiICJzSrWjYEyQ-UIr9WiNk7_Sg6L8wqhrSW-o9P8Y8bpG_69g
📊 function executed
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: Some(Object {"type": String("Object"), "value": Object {"role": String("admin"), "userId": String("user_abc")}})
🔍 Key exists: true
✅ Set cached data
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: H_kB-HdrYTE5d-Mxs7oHs14rJYZiZrrdpgbrWGMgiG7GQ7LDbCtdkjeRadYdlH2sjWo2uBStgs8XXnNIvtJTBQ
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: utbbMaZPGQ7DazQE_dk7p0GilYkThmbLKGUKjdvtVMPkhCb-ZOz56U5GJ8Bspd2OKqONWHMQCqfz_x5w2KC7PA
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_functions_search`
🚀 ekoDB Rust Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Getting Started with ekoDB (Database)
   2. Introduction to Machine Learning (AI)
   3. Database Design Principles (Database)
   4. Vector Databases Explained (Database)
   5. Natural Language Processing (AI)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   Record({"category": Object({"value": String("AI"), "type": String("String")}), "count": Object({"value": Integer(2), "type": String("Integer")})})
   Record({"count": Object({"value": Integer(3), "type": String("Integer")}), "category": Object({"type": String("String"), "value": String("Database")})})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_goal_templates`
=== ekoDB Goal Template CRUD Example (Rust) ===

--- Creating goal template ---
Created template: Data Migration (id: y4AGokfGbjULCU7zCEaatvOk-vl6gkHI-03nqXcwloOum7a-8pPCiPGAZjR_Gsm3Z_3f6Mq5EVSMeMBkw61SvA)

--- Listing templates ---
Templates: {"count":1,"items":[{"description":{"type":"String","value":"Template for migrating data between schemas"},"id":"y4AGokfGbjULCU7zCEaatvOk-vl6gkHI-03nqXcwloOum7a-8pPCiPGAZjR_Gsm3Z_3f6Mq5EVSMeMBkw61SvA","steps":{"type":"Array","value":[{"description":"Analyze source schema"},{"description":"Create target schema"},{"description":"Migrate records"},{"description":"Validate results"}]},"title":{"type":"String","value":"Data Migration"}}]}

--- Getting template ---
Fetched: {"type":"String","value":"Data Migration"}

--- Updating template ---
Updated description: {"type":"String","value":"Updated: comprehensive data migration workflow"}

--- Deleting template ---
Template deleted successfully

✓ Goal template CRUD example completed
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/examples/client_goals_tasks_agents`
=== ekoDB Goals, Tasks & Agents Integration Example (Rust) ===

--- Goal: create ---
Created goal: "Deploy v2.0 to production" (id: D5NVcTWVmpDKFpRUDMnntMipZF_BHTuoPFbIS290sLmjib-KkGyQ9JQSy1m3jnaCunctTyHJ5fa_i60PSXN7hw)

--- Goal: list ---
Goals: {"count":1,"goals":[{"created_at":"2026-09-16T13:24:14.983121+00:00","description":"Full release cycle for version 2.0","id":"D5NVcTWVmpDKFpRUDMnntMipZF_BHTuoPFbIS290sLmjib-KkGyQ9JQSy1m3jnaCunctTyHJ5fa_i60PSXN7hw","status":"pending","steps":"[{\"description\":\"Run integration tests\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]","title":"Deploy v2.0 to production","updated_at":"2026-09-16T13:24:14.983121+00:00"}]}

--- Goal: get ---
Fetched goal: {"type":"String","value":"Deploy v2.0 to production"}

--- Goal: update ---
Updated description: {"type":"String","value":"Updated: full v2.0 release with rollback plan"}

--- Goal: search ---
Search results: {"count":1,"items":[{"_score":12.87,"created_at":{"type":"DateTime","value":"2026-09-16T13:24:14.983121+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"D5NVcTWVmpDKFpRUDMnntMipZF_BHTuoPFbIS290sLmjib-KkGyQ9JQSy1m3jnaCunctTyHJ5fa_i60PSXN7hw","status":{"type":"String","value":"pending"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-09-16T13:24:15.035462+00:00"}}]}

--- Goal step: start (step 0) ---
Step 0 started: {"created_at":{"type":"DateTime","value":"2026-09-16T13:24:14.983121+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"D5NVcTWVmpDKFpRUDMnntMipZF_BHTuoPFbIS290sLmjib-KkGyQ9JQSy1m3jnaCunctTyHJ5fa_i60PSXN7hw","status":{"type":"String","value":"in_progress"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\",\"status\":\"InProgress\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-09-16T13:24:15.052739+00:00"}}

--- Goal step: complete (step 0) ---
Step 0 completed: {"created_at":{"type":"DateTime","value":"2026-09-16T13:24:14.983121+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"D5NVcTWVmpDKFpRUDMnntMipZF_BHTuoPFbIS290sLmjib-KkGyQ9JQSy1m3jnaCunctTyHJ5fa_i60PSXN7hw","status":{"type":"String","value":"in_progress"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\",\"result\":\"All 247 tests passed\",\"status\":\"Completed\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-09-16T13:24:15.065422+00:00"}}

--- Goal step: start (step 1) ---
Step 1 started

--- Goal step: fail (step 1) ---
Step 1 failed: {"created_at":{"type":"DateTime","value":"2026-09-16T13:24:14.983121+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"D5NVcTWVmpDKFpRUDMnntMipZF_BHTuoPFbIS290sLmjib-KkGyQ9JQSy1m3jnaCunctTyHJ5fa_i60PSXN7hw","status":{"type":"String","value":"in_progress"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\",\"result\":\"All 247 tests passed\",\"status\":\"Completed\"},{\"description\":\"Build release artifacts\",\"error\":\"Build timeout after 300s\",\"status\":\"Failed\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-09-16T13:24:15.086183+00:00"}}

--- Goal: complete ---
Goal completed (pending review): {"completed_at":{"type":"DateTime","value":"2026-09-16T13:24:15.095281+00:00"},"created_at":{"type":"DateTime","value":"2026-09-16T13:24:14.983121+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"D5NVcTWVmpDKFpRUDMnntMipZF_BHTuoPFbIS290sLmjib-KkGyQ9JQSy1m3jnaCunctTyHJ5fa_i60PSXN7hw","status":{"type":"String","value":"pending_review"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\",\"result\":\"All 247 tests passed\",\"status\":\"Completed\"},{\"description\":\"Build release artifacts\",\"error\":\"Build timeout after 300s\",\"status\":\"Failed\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"summary":{"type":"String","value":"Partial completion — step 1 failed"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-09-16T13:24:15.095281+00:00"}}

--- Goal: approve ---
Goal approved: {"completed_at":{"type":"DateTime","value":"2026-09-16T13:24:15.095281+00:00"},"created_at":{"type":"DateTime","value":"2026-09-16T13:24:14.983121+00:00"},"description":{"type":"String","value":"Updated: full v2.0 release with rollback plan"},"id":"D5NVcTWVmpDKFpRUDMnntMipZF_BHTuoPFbIS290sLmjib-KkGyQ9JQSy1m3jnaCunctTyHJ5fa_i60PSXN7hw","status":{"type":"String","value":"in_progress"},"steps":{"type":"String","value":"[{\"description\":\"Run integration tests\",\"result\":\"All 247 tests passed\",\"status\":\"Completed\"},{\"description\":\"Build release artifacts\",\"error\":\"Build timeout after 300s\",\"status\":\"Failed\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"},"summary":{"type":"String","value":"Partial completion — step 1 failed"},"title":{"type":"String","value":"Deploy v2.0 to production"},"updated_at":{"type":"DateTime","value":"2026-09-16T13:24:15.104471+00:00"}}

--- Goal: create (for rejection) ---
Created goal2: "Refactor auth module" (id: 4zzaCO6TqzWTWWclqpm9XGpgRcY9h9BI3cEViwxH_J5oimpnB00pMxVZ1ixltgioQFWwSmm3BrIPHKosV1mTHg)

--- Goal: complete (goal2) ---

--- Goal: reject ---
Goal rejected: {"completed_at":{"type":"DateTime","value":"2026-09-16T13:24:15.130807+00:00"},"created_at":{"type":"DateTime","value":"2026-09-16T13:24:15.113443+00:00"},"description":{"type":"String","value":"Rewrite JWT handling"},"id":"4zzaCO6TqzWTWWclqpm9XGpgRcY9h9BI3cEViwxH_J5oimpnB00pMxVZ1ixltgioQFWwSmm3BrIPHKosV1mTHg","reason":{"type":"String","value":"Breaks backward compatibility"},"status":{"type":"String","value":"failed"},"steps":{"type":"String","value":"[]"},"summary":{"type":"String","value":"Completed refactor"},"title":{"type":"String","value":"Refactor auth module"},"updated_at":{"type":"DateTime","value":"2026-09-16T13:24:15.130807+00:00"}}

--- Task: create ---
Created task: Nightly backup (id: 9EzC-4nX2_-60iWre_YoqrVpUKIBOyCbBhGH-i1reOP6LTCjeygW0ReFCen5WjDB5knMujsznVByL0-hqHxjFA)

--- Task: list ---
Tasks: {"count":1,"items":[{"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"9EzC-4nX2_-60iWre_YoqrVpUKIBOyCbBhGH-i1reOP6LTCjeygW0ReFCen5WjDB5knMujsznVByL0-hqHxjFA","max_retries":{"type":"Integer","value":3},"schedule":{"type":"String","value":"0 2 * * *"},"title":{"type":"String","value":"Nightly backup"}}]}

--- Task: get ---
Fetched task: {"type":"String","value":"Nightly backup"}

--- Task: start ---
Task started: {"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"9EzC-4nX2_-60iWre_YoqrVpUKIBOyCbBhGH-i1reOP6LTCjeygW0ReFCen5WjDB5knMujsznVByL0-hqHxjFA","max_retries":{"type":"Integer","value":3},"schedule":{"type":"String","value":"0 2 * * *"},"status":{"type":"String","value":"running"},"title":{"type":"String","value":"Nightly backup"},"updated_at":{"type":"DateTime","value":"2026-09-16T13:24:15.178359+00:00"}}

--- Task: succeed ---
Task succeeded: {"consecutive_failures":{"type":"Integer","value":0},"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"9EzC-4nX2_-60iWre_YoqrVpUKIBOyCbBhGH-i1reOP6LTCjeygW0ReFCen5WjDB5knMujsznVByL0-hqHxjFA","last_error":{"type":"Null","value":null},"last_run":{"type":"DateTime","value":"2026-09-16T13:24:15.187065+00:00"},"max_retries":{"type":"Integer","value":3},"run_count":{"type":"Integer","value":1},"schedule":{"type":"String","value":"0 2 * * *"},"status":{"type":"String","value":"active"},"title":{"type":"String","value":"Nightly backup"},"updated_at":{"type":"DateTime","value":"2026-09-16T13:24:15.187065+00:00"}}

--- Task: start (round 2) ---

--- Task: pause ---
Task paused: {"consecutive_failures":{"type":"Integer","value":0},"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"9EzC-4nX2_-60iWre_YoqrVpUKIBOyCbBhGH-i1reOP6LTCjeygW0ReFCen5WjDB5knMujsznVByL0-hqHxjFA","last_error":{"type":"Null","value":null},"last_run":{"type":"DateTime","value":"2026-09-16T13:24:15.187065+00:00"},"max_retries":{"type":"Integer","value":3},"run_count":{"type":"Integer","value":1},"schedule":{"type":"String","value":"0 2 * * *"},"status":{"type":"String","value":"paused"},"title":{"type":"String","value":"Nightly backup"},"updated_at":{"type":"DateTime","value":"2026-09-16T13:24:15.205785+00:00"}}

--- Task: resume ---
Task resumed: {"consecutive_failures":{"type":"Integer","value":0},"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"9EzC-4nX2_-60iWre_YoqrVpUKIBOyCbBhGH-i1reOP6LTCjeygW0ReFCen5WjDB5knMujsznVByL0-hqHxjFA","last_error":{"type":"Null","value":null},"last_run":{"type":"DateTime","value":"2026-09-16T13:24:15.187065+00:00"},"max_retries":{"type":"Integer","value":3},"run_count":{"type":"Integer","value":1},"schedule":{"type":"String","value":"0 2 * * *"},"status":{"type":"String","value":"active"},"title":{"type":"String","value":"Nightly backup"},"updated_at":{"type":"DateTime","value":"2026-09-16T13:24:15.214914+00:00"}}

--- Task: fail ---
Task failed: {"consecutive_failures":{"type":"Integer","value":1},"description":{"type":"String","value":"Full database backup to S3"},"due_at":{"type":"DateTime","value":"2026-03-22T02:00:00+00:00"},"id":"9EzC-4nX2_-60iWre_YoqrVpUKIBOyCbBhGH-i1reOP6LTCjeygW0ReFCen5WjDB5knMujsznVByL0-hqHxjFA","last_error":{"type":"String","value":"S3 bucket access denied"},"last_run":{"type":"DateTime","value":"2026-09-16T13:24:15.226393+00:00"},"max_retries":{"type":"Integer","value":3},"run_count":{"type":"Integer","value":1},"schedule":{"type":"String","value":"0 2 * * *"},"status":{"type":"String","value":"active"},"title":{"type":"String","value":"Nightly backup"},"updated_at":{"type":"DateTime","value":"2026-09-16T13:24:15.226393+00:00"}}

--- Task: due ---
Due tasks: {"count":0,"items":[]}

--- Task: delete ---
Task deleted

--- Agent: create ---
Created agent: backup-agent (id: qG3hbvNQv9bXzex89VZzptJy9uIQBHNSS-SdpZlMDIDaCO-STg0Tc0uOpvsV2QrfsABazhvIlmn8SP-By-YFWg)

--- Agent: list ---
Agents: {"count":1,"items":[{"capabilities":{"type":"Array","value":["backup","restore","verify"]},"deployment_id":{"type":"String","value":"deploy-abc-123"},"description":{"type":"String","value":"Handles nightly backups and restores"},"id":"qG3hbvNQv9bXzex89VZzptJy9uIQBHNSS-SdpZlMDIDaCO-STg0Tc0uOpvsV2QrfsABazhvIlmn8SP-By-YFWg","llm_model":{"type":"String","value":"gpt-4.1"},"name":{"type":"String","value":"backup-agent"}}]}

--- Agent: get ---
Fetched agent: {"type":"String","value":"backup-agent"}

--- Agent: get_by_name ---
Found by name: {"type":"String","value":"backup-agent"}

--- Agent: update ---
Updated agent description: {"type":"String","value":"Handles backups, restores, and disaster recovery"}

--- Agent: agents_by_deployment ---
Agents in deployment: {"count":0,"items":[]}
WARNING: agents_by_deployment omitted created agent qG3hbvNQv9bXzex89VZzptJy9uIQBHNSS-SdpZlMDIDaCO-STg0Tc0uOpvsV2QrfsABazhvIlmn8SP-By-YFWg; TODO: check/fix the server-side deployment lookup

--- Agent: delete ---
Agent deleted

--- Goal: delete ---
Goal 1 deleted
Goal 2 deleted

=== All goals, tasks & agents operations completed ===
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_kv_links`
=== ekoDB KV Document Linking Example (Rust) ===

--- Inserting documents ---
Inserted doc1: Project Alpha (id: j4Opi_fn1na5xnhLFMxxiCJ8ga3ZLd6E18QO_11cpP5h32jRPxQYrTS2lcFJgG23T35Ha3kq7WfX8W3b85G6AQ)
Inserted doc2: Project Beta (id: f9T04Plq9D0m2_5RTNV1owy9E4LSRnQS_ldIW9oJTFPiok6aw1wr7G-WTl2EldfGCIpJwX606CEgAkUp8nNniQ)

--- Setting KV key ---
Set key: kv_links:rs:user:alice:projects

--- Linking documents ---
Linked doc1: null
Linked doc2: null

--- Getting links ---
Links for kv_links:rs:user:alice:projects: [{"collection":"kv_links_example_rs","created_at":"2026-09-16T13:27:01.718838Z","document_id":"j4Opi_fn1na5xnhLFMxxiCJ8ga3ZLd6E18QO_11cpP5h32jRPxQYrTS2lcFJgG23T35Ha3kq7WfX8W3b85G6AQ","field_path":null,"last_accessed":"2026-09-16T13:27:01.722294Z","metadata":{}},{"collection":"kv_links_example_rs","created_at":"2026-09-16T13:27:01.720586Z","document_id":"f9T04Plq9D0m2_5RTNV1owy9E4LSRnQS_ldIW9oJTFPiok6aw1wr7G-WTl2EldfGCIpJwX606CEgAkUp8nNniQ","field_path":null,"last_accessed":"2026-09-16T13:27:01.722294Z","metadata":{}}]

--- Unlinking doc1 ---
Unlinked doc1: null

--- Verifying remaining links ---
Remaining links: [{"collection":"kv_links_example_rs","created_at":"2026-09-16T13:27:01.720586Z","document_id":"f9T04Plq9D0m2_5RTNV1owy9E4LSRnQS_ldIW9oJTFPiok6aw1wr7G-WTl2EldfGCIpJwX606CEgAkUp8nNniQ","field_path":null,"last_accessed":"2026-09-16T13:27:01.725649Z","metadata":{}}]

--- Cleanup ---
Unlinked remaining document
Deleted KV key
Deleted documents
Deleted collection

=== All KV linking operations completed ===
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_query_builder`
=== ekoDB Query Builder Example ===

=== Inserting Sample Data ===
✓ Inserted 5 users

=== Simple Equality Query ===
✓ Found 3 active users
  - Some(Object({"type": String("String"), "value": String("Alice")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"value": String("David"), "type": String("String")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Eve")}))
  - Some(Object({"value": String("David"), "type": String("String")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))
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
  - Some(Object({"type": String("String"), "value": String("Charlie")}))
  - Some(Object({"type": String("String"), "value": String("Alice")}))

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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_raw_completion_stream`
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
    "name": "Mars",
    "diameter_km": 6779
  }
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello! I hope you're having a great day.

=== Done ===
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_schedules`
=== ekoDB Schedule Management Example (Rust) ===

--- Creating schedule ---
Created schedule: "nightly-cleanup" (id: 1192c86d-2c5a-4634-8c82-7bfe045782fe)

--- Listing schedules ---
Schedules: {"count":1,"schedules":[{"created_at":"2026-09-16T13:33:12.904710Z","cron_expression":"0 0 3 * * *","description":"Remove expired sessions and temp files","enabled":true,"function_label":"schedule_noop_rust_97860","id":"1192c86d-2c5a-4634-8c82-7bfe045782fe","last_execution":null,"name":"nightly-cleanup","next_execution":"2026-09-17T03:00:00Z","parameters":{},"stats":{"avg_execution_time_ms":0.0,"failed_executions":0,"last_error":null,"successful_executions":0,"total_executions":0},"timezone":"UTC","updated_at":"2026-09-16T13:33:12.904710Z"}]}

--- Getting schedule ---
Fetched schedule: "nightly-cleanup"

--- Updating schedule ---
Updated cron: "0 0 4 * * *"
Updated description: "Remove expired sessions, temp files, and orphaned uploads"

--- Triggering schedule ---
Trigger response: {"schedule_id":"1192c86d-2c5a-4634-8c82-7bfe045782fe","status":"triggered"}

--- Pausing schedule ---
Schedule paused: {"created_at":"2026-09-16T13:33:12.904710Z","cron_expression":"0 0 4 * * *","description":"Remove expired sessions, temp files, and orphaned uploads","enabled":false,"function_label":"schedule_noop_rust_97860","id":"1192c86d-2c5a-4634-8c82-7bfe045782fe","last_execution":"2026-09-16T13:33:12.962552Z","name":"nightly-cleanup","next_execution":null,"parameters":{"type":"Object","value":{}},"stats":{"avg_execution_time_ms":0.0,"failed_executions":0,"last_error":null,"successful_executions":0,"total_executions":0},"timezone":"UTC","updated_at":"2026-09-16T13:33:12.966516Z"}

--- Resuming schedule ---
Schedule resumed: {"created_at":"2026-09-16T13:33:12.904710Z","cron_expression":"0 0 4 * * *","description":"Remove expired sessions, temp files, and orphaned uploads","enabled":true,"function_label":"schedule_noop_rust_97860","id":"1192c86d-2c5a-4634-8c82-7bfe045782fe","last_execution":"2026-09-16T13:33:12.962552Z","name":"nightly-cleanup","next_execution":"2026-09-17T04:00:00Z","parameters":{"type":"Object","value":{}},"stats":{"avg_execution_time_ms":0.0,"failed_executions":0,"last_error":null,"successful_executions":0,"total_executions":0},"timezone":"UTC","updated_at":"2026-09-16T13:33:12.970539Z"}

--- Deleting schedule ---
Schedule deleted successfully

=== All schedule operations completed ===
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/examples/client_schema`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("M6WEiYR3obpIKwNG_1X1ODOfoJcx4PJAWGRM46DJ-T-q53TuCDRmnPpZMa88wCEBKyQ_QvLw8oe67AvkC1tFrw"))
✓ Inserted user 2: Some(String("GR6XiSZRVTdKxYMmoa9cSm8x5whDCLn-uap6wm6pI8vaR_Mrp132BphCuDweeMgz_1Z-wHy2E5Pie8Wey5ri0w"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - title: String
    (required)
  - email: String
    (required)
  - status: String
  - age: Integer

=== Listing Collections ===
✓ Total collections: 13
  Sample: ["chat_agent_configs__ek0_testing", "agent_function_versions__ek0_testing", "schedules__ek0_testing", "functions__ek0_testing", "chat_configurations__ek0_testing"]

=== Cleanup ===
✓ Deleted collection

✓ All schema management operations completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
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
     Matched: ["title", "description"]
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

Execution time: 7ms
=== Cleanup ===
✓ Deleted collection

✓ All search operations completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record({"id": String("GO79zzr3Ba7Sd-VIVQDt8C9muk_HihLXrRrSfuaeAzOvBWUwWCiE45iZYtz4T0msTmaJbadjaryfOmHdFqjr1A")})

=== Find by ID ===
Found: Record({"price": Object({"type": String("Float"), "value": Float(99.99)}), "name": Object({"value": String("Test Record"), "type": String("String")}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "id": String("GO79zzr3Ba7Sd-VIVQDt8C9muk_HihLXrRrSfuaeAzOvBWUwWCiE45iZYtz4T0msTmaJbadjaryfOmHdFqjr1A"), "created_at": Object({"type": String("DateTime"), "value": String("2026-09-16T13:36:20.355088+00:00")}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "embedding": Vector([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "value": Object({"value": Integer(42), "type": String("Integer")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "metadata": Object({"value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})}), "type": String("Object")})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-09-16T13:36:20.355088+00:00")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 5 dims
  categories (Set): 2 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record({"created_at": Object({"type": String("DateTime"), "value": String("2026-09-16T13:36:20.355088+00:00")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "value": Object({"type": String("Integer"), "value": Integer(42)}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "name": Object({"type": String("String"), "value": String("Test Record")}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "embedding": Vector([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "id": String("GO79zzr3Ba7Sd-VIVQDt8C9muk_HihLXrRrSfuaeAzOvBWUwWCiE45iZYtz4T0msTmaJbadjaryfOmHdFqjr1A"), "metadata": Object({"value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})}), "type": String("Object")})})]

=== Update Document ===
Updated: Record({"embedding": Vector([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "price": Object({"type": String("Float"), "value": Float(99.99)}), "metadata": Object({"type": String("Object"), "value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")})}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "name": Object({"value": String("Updated Record"), "type": String("String")}), "created_at": Object({"type": String("DateTime"), "value": String("2026-09-16T13:36:20.355088+00:00")}), "id": String("GO79zzr3Ba7Sd-VIVQDt8C9muk_HihLXrRrSfuaeAzOvBWUwWCiE45iZYtz4T0msTmaJbadjaryfOmHdFqjr1A"), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "value": Object({"type": String("Integer"), "value": Integer(100)})})

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: zh5nrq4AzUj1_YXq-BTd5-drVdAJEsUyO9ErsqtB9XHu8CY7KrxdCN-xK8rpJ1gMSwD7Dh27tbXCz_pIGX5-vw

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  active: true
  value: 42
  name: "WebSocket Test Record"
  id: "zh5nrq4AzUj1_YXq-BTd5-drVdAJEsUyO9ErsqtB9XHu8CY7KrxdCN-xK8rpJ1gMSwD7Dh27tbXCz_pIGX5-vw"

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_swr_native`
=== ekoDB Native SWR Function ===

This example shows the new simplified SWR function that replaces
the manual FindById → If → HttpRequest → Insert pattern.

Example 1: Basic GitHub User Cache with Native SWR
─────────────────────────────────────────────────────

✓ Created native SWR script: github_user_native_rs (XU85XY54vl1JBrJ3OoQWrlHO7P1tApr_69ukUwSEZlJxDxPJxI3n6BQQJOAiHgb9-fCHuFXRv4oaASabclryzg)

First call (cache miss - will fetch from GitHub API):
  Response time: 174ms
  Records returned: 1
  ✓ Data fetched from API and cached with 15m TTL

Second call (cache hit - instant from KV store):
  Response time: 3ms
  Speedup: 58.0x faster
  ✓ Lightning fast cache hit


Example 2: SWR with Audit Trail Collection
─────────────────────────────────────────────────────

✓ Created SWR script with audit trail: product_swr_audit_rs (h-ldBqy5rCwuioQLXEMQsQzGgNQD2jZ5bdMt_74nKevpJW4-lLAF1seRRxmaRDLQ2S7bU2KWN-EruHBcwwhW7w)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_rs' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
─────────────────────────────────────────────────────

Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_rs (B7x5Vd0s0GvEXK6pSrEkQIbl2gkZFM_p2ADkFrK2_PhQ5pJM7vJhogzUUg2D7DgTs7Mm59DICSD0ELLD_EhbiQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_rs' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
─────────────────────────────────────────────────────

✓ Created dynamic TTL script: flexible_cache_rs (zEVZfSxI6iy14B8xq2mHyjV8uiGbEx77_4gA1VLL8Rc4Qnt5fEMkAvLGiCggUWGDuvMakvbBxnjWIgNcWk-Liw)
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

perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_rs (ZEMbIx8l47DWHbfAiEB0w7twFXE53THoWbwOaGiSKmfzoceXtST5yaLC8LMArYxGzanG4pWjSRXaaXFF3C0zaQ)

Step 2: First call - Cache miss, fetches from API
Result: FunctionStats { input_count: 0, output_count: 1, execution_time_ms: 0, stages_executed: 2, stage_stats: [] }
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 13ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_transactions`
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 0PdbEi5cW3p2l9T1v1D4cIS5EA4bZrLQyPSY9E3P4nWVAaTK8dNCms9drZe4G0axfaO-TO0Vg7IdXI5ttekYxw
Created Bob: $500 - ID: CZZA2uw4vrUkbWw7a3kKGwQy6f-T6kxhpRvjLsIYPJ8YGKt3zlDR4zz1xbJWiNYUo_fbtIayAvTgwcNWsgo3xg

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): e7f2b28f-ba2b-4126-9fc2-6360a546a6af

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
New transaction: 7d799d00-ab45-4f26-aef2-d6ad9610628f
Updated Bob: $700 → $600 (in transaction)
Status before rollback: "Active"
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_user_functions`
✓ Client created

=== Create User Function ===
Created user function with ID: Ya5jiKsb6sbJO2CvfmHjKNgh3v_jUPAVpxEtDJniPJkQz_hfC-dfcadQu8Bfe4B4chVbqD37GudED8gH9Hf1ow

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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_websocket_chat_stream`
=== WebSocket Chat Streaming Example (Rust) ===

✓ Authentication successful
✓ Created chat session: 7QFgeFN96a9NlN3pppQPL_xuzwUrB4B-1x4W6m82BLFkBqkffP5ZOX9qfAF93iTpIsrbPh-eJkyuzm1S2kayAg

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Sending message: 'What is the capital of France?' ===
✓ Message accepted, streaming...

{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: "VHeqDJpq1Asg2WpK62faqwLvoqnsT4GW4Emy3HylrmVD0gPMSgfzC4gVksYG520dC3traVTUhf5syc1XDcOUQA"
Execution time: 1020ms
Token usage: {"completion_tokens":8,"prompt_tokens":15,"total_tokens":23}
Context window: 1000000 tokens

Full response: {"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris....

✓ WebSocket chat streaming example completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_websocket_subscribe`
✓ Authentication successful

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_rs' ===
✓ Subscribed (subscription_id: sub_9a52760246f94cb389395a5bfadd802d)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: 3iGCRbRRPP8pyOKcdXOzyhrsf2l1FxA-FzR4WdWztZNRZtQW47Mi_e7NbqM03GBhJebZKrRsrmmT6mTecEJPHQ

  📡 Notification received:
     Event:      "insert"
     Collection: "ws_subscribe_example_rs"
     Record IDs: ["3iGCRbRRPP8pyOKcdXOzyhrsf2l1FxA-FzR4WdWztZNRZtQW47Mi_e7NbqM03GBhJebZKrRsrmmT6mTecEJPHQ"]
     Timestamp:  "2026-09-16T13:42:44.956886+00:00"

Inserting record 2...
✓ Inserted: iidsP1x4Hz1KWR_wvOfnS-kuVufcaHgNe35fNpaquy5xSsjEYdtvyb7IFwYIHSVKOGKe2kOeuB25Hd3MPcRFkQ

  📡 Notification received:
     Event:      "insert"
     Record IDs: ["iidsP1x4Hz1KWR_wvOfnS-kuVufcaHgNe35fNpaquy5xSsjEYdtvyb7IFwYIHSVKOGKe2kOeuB25Hd3MPcRFkQ"]

=== Unsubscribing ===
✓ Unsubscribed

=== Cleanup ===
✓ Deleted collection 'ws_subscribe_example_rs'

✓ WebSocket subscription example completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_websocket_ttl`
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Some(String("baYROqS7A06La9OjSiBMMbkqQaAP9oF3i7ZE6ZgndSHC3mvl11eZgfE_ynn81-EPEfbmECG_YbiKmpKweT0Blw"))

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/bypass_ripple_example`
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: Record({"id": String("psB7ZwNMa5QlcSJtX3XQ4ulAU78axuVddiYaSaTdMeT4aYyjF4OVfqt4EK8XfjIG6sy03SI6wj8F3I6fO1vwuA")})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: Record({"id": String("s7I2Br5j72DCD1aGcuIH7KsYxeChGxfNz9LaE7sE1wKMEpMWzb4ZNiucBhTEDObQQmJOG_B61uAYFtI4n6I0ZQ")})

3. Update with bypass_ripple:
   Updated with bypass_ripple: Record({"name": Object({"value": String("Product 1"), "type": String("String")}), "id": String("psB7ZwNMa5QlcSJtX3XQ4ulAU78axuVddiYaSaTdMeT4aYyjF4OVfqt4EK8XfjIG6sy03SI6wj8F3I6fO1vwuA"), "price": Object({"type": String("Integer"), "value": Integer(150)})})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: Record({"id": String("h3Ciw3daU62OJA2Vy9_TTIWHXsop-n2l2vd8zIatbanW4YEpM6WRvFCjugZkxDOKauVYfb3MDUUHUbU2MKD9Pw")})

✅ All bypass_ripple operations completed successfully!
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/projection_example`
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ["id", "email", "name"]

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["avatar_url", "name", "id", "bio", "created_at", "status", "email", "age", "user_role"]

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)

Example 4: Query inactive users with profile fields
  Found 1 inactive users

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ["user_role", "email", "name", "status", "age", "created_at", "api_key", "bio", "password", "secret_token", "id", "avatar_url"]
  Projected query:
    - 3 fields per record
    - Fields: ["email", "name", "id"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!

📦 Ensuring Python example dependencies in .venv...
🧪 Running Python examples (direct HTTP/WebSocket)...
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'TM-QdUR_6x6IslYUQbtugSFgLeifarLw6N7SHSUWkUQCVjEURCmVJjcLHIDAknO-ciup0ntYpzj50LRl9v9Mxw'}

=== Find by ID ===
Found: {'name': {'type': 'String', 'value': 'Test Record'}, 'active': {'type': 'Boolean', 'value': True}, 'id': 'TM-QdUR_6x6IslYUQbtugSFgLeifarLw6N7SHSUWkUQCVjEURCmVJjcLHIDAknO-ciup0ntYpzj50LRl9v9Mxw', 'value': {'value': 42, 'type': 'Integer'}}

=== Find with Query ===
Found documents: [{'name': {'type': 'String', 'value': 'Test Record'}, 'value': {'value': 42, 'type': 'Integer'}, 'active': {'type': 'Boolean', 'value': True}, 'id': 'TM-QdUR_6x6IslYUQbtugSFgLeifarLw6N7SHSUWkUQCVjEURCmVJjcLHIDAknO-ciup0ntYpzj50LRl9v9Mxw'}]

=== Update Document ===
Updated: {'name': {'type': 'String', 'value': 'Updated Record'}, 'active': {'value': True, 'type': 'Boolean'}, 'id': 'TM-QdUR_6x6IslYUQbtugSFgLeifarLw6N7SHSUWkUQCVjEURCmVJjcLHIDAknO-ciup0ntYpzj50LRl9v9Mxw', 'value': {'type': 'Integer', 'value': 100}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: vnEyQd5QIipWYXaesy5-e3jkSI5lF5Nntzy0yNsk8QDvZJ2Out94Lb4Kkh2lJWzCu3oJXVOvVhdHrebBeMgJ7A

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
        "value": {
          "value": 42,
          "type": "Integer"
        },
        "id": "vnEyQd5QIipWYXaesy5-e3jkSI5lF5Nntzy0yNsk8QDvZJ2Out94Lb4Kkh2lJWzCu3oJXVOvVhdHrebBeMgJ7A"
      }
    ]
  },
  "messageId": "372394558"
}
✓ Retrieved 1 record via WebSocket

✓ WebSocket example completed successfully
🚀 ekoDB Functions Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: sM68v2lTjXweuS5N0EaQyBcF1223pit5d-J-kFSzAsf9EQYOHOEGj5MJIfXAQPhh5bawnYwTSfm4qKdkkWiigA
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: RuE08wdmkE9MkXv9EMiJSxyM_YfPmSbKG3koydGlUfkBGtPm4Kl7-Cwzue59fLWMrXXdBIY80KYMHQ05z1j_ig
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: U1c1tCezuP_dW6DxqPJolTkcKFbvP2nRie6l9P8R66I-hdkQ47u8ZwHwRhaUC4DOV2XrWU-MC8XS62LpC9qttw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'count': {'type': 'Integer', 'value': 5}, 'max_score': {'type': 'Integer', 'value': 100}, 'status': {'value': 'active', 'type': 'String'}, 'avg_score': {'value': 60.0, 'type': 'Float'}}
   {'max_score': {'value': 90, 'type': 'Integer'}, 'avg_score': {'value': 50.0, 'type': 'Float'}, 'status': {'type': 'String', 'value': 'inactive'}, 'count': {'value': 5, 'type': 'Integer'}}

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
Retrieved value: {'type': 'Object', 'value': {'userId': 123, 'username': 'john_doe'}}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
kv_operations:direct:py:cache:product:1: {'type': 'Object', 'value': {'name': 'Product 1', 'price': 29.99}}
kv_operations:direct:py:cache:product:2: {'type': 'Object', 'value': {'name': 'Product 2', 'price': 39.989999999999995}}
kv_operations:direct:py:cache:product:3: {'type': 'Object', 'value': {'price': 49.989999999999995, 'name': 'Product 3'}}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: 8WIytDUW7n2w13lz4Qb1Dr99fiP2CzYXfQLFavlVyyd9178oo_TnFnj064ZvCDJUcW1mSc4bFRzg4TwWXI_Rvg

=== List Collections ===
Total collections: 13
Sample collections: ['chat_agent_configs__ek0_testing', 'agent_function_versions__ek0_testing', 'schedules__ek0_testing', 'functions__ek0_testing', 'chat_configurations__ek0_testing']

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
Created Alice: $1000 - ID: VLImFh2NTxDJx6Aip5SHAQFUg1IOTO6ELQyQidOaazkGZ6rTYzDuxBGLI5dPk291XEKfBWPeTBQmzH_8RxkfPw
Created Bob: $500 - ID: mZZbicGtnRYDnYqSgv_oCKC0GTVWGyNCc3irZzTCcK8FjZ3RhF6dDb6jU2YuVgP2FnxLcIXg-VndRBMJtQgswA

=== Example 1: Begin Transaction ===
Transaction ID: ff04e3d1-50e3-4a55-9f93-b28754427a11

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
New transaction: c7e8547f-738c-4246-a7e5-aa5e5ebde828
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: RDmu5MWZdrTO-PFMdZdANPVSCeU5XtMYEjQJOUTwoX0d_mmHKuOI9hFW6polhu93HitdIUJ8hFqfyFByZtx7Ew
Created Bob: $500 - ID: rKnVOtTCZv8cur7YIPwI63yDKHnCLRLRpAKhuzYdCme3jbqWf_xyJO4-3jLz2h0tl415Jv7ceI97DeNRgjPwRA

=== Example 1: Begin Transaction ===
Transaction ID: ce4bf37f-aecf-4d19-9f78-06d69ce67523

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
Created Alice: $1000 - ID: 2YgIbBifibp8lDGVf0hzL8sgn7KaXtm_d4512WIBb0f6hL-dLvA9yCW7oAlwI_faYrbb12YA4prcqnBo4cHeyA
Created Bob: $500 - ID: sKrbpVGUav4XAXXpiyaSvRknnJCFnf84qKZOIiWSZs_IKCuEsj1xuoyLk_RRsPuhNlX2LUxUtfsyIqqQ306S0w

=== Example 1: Begin Transaction ===
Transaction ID: 7600bc9c-180d-4aec-ba1c-00fd9eb7e916

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
New transaction: 30eb8f0d-dc08-46cc-832c-3b5c7b6c7a4e
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: IvhYLSywsp88rDwhpMKXGNo_OqB8YHbP1U0ODUfhjUJ64tQGH1iCY024VudY3aGzUauYRcj_rFwDAidepyTE4A
Created Bob: $500 - ID: 6LEfITW0a04fCDUF5H7EgLwGrIbkUXiWGqpD3kDs-4_g8sf_6XtLE1McnqHJyEChuPAMkEnFOgwTmazj1cudhA

=== Example 1: Begin Transaction ===
Transaction ID: a35c94d5-a3bf-4a31-be1d-a3931946c1ef

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


🔄 Switching to DURABLE mode...
   ✓ Config updated: durable_operations=true


============================================================
🔥 TEST 3: Durable Mode (durable=true)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: L4FLEcQPSjzLkTk6xZr6ZcJLc1fbFB34INXVcD7D0y4YcfVzd43ZSGKS4vW-A712_AJakReNBaKqMvbekUFFQQ
Created Bob: $500 - ID: ZGPJeClHYKpMCi3kRZ4rlbSsyDRTpM-Q-BCQTCPQTAzheRP4hUJC5s_xiDLXD1bdS9KRMK6onSrisnRkzL0Oqg

=== Example 1: Begin Transaction ===
Transaction ID: 4def801c-e469-4c61-bd91-ac0d314a28de

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
New transaction: 5ab1d015-ab97-4838-80a8-11054ca1c48a
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: mqw3yKMtTb-KfuFe3jglw_ERh-3JkObr9L7lpNFeWe8fYN_qU2Zt6mzz4B8_qZk2qtwFLo9-g8nr95dwrE1GJA
Created Bob: $500 - ID: hxGdhy7SIFjUovyrJp_Eev92UkYm5qGuwaseFdzl9oCIXc6CUBcWd71k8eoknvGNocBVKCAhsrVe8DfaeLnI3w

=== Example 1: Begin Transaction ===
Transaction ID: 37aab40f-f612-48eb-9d1b-648e0962c746

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
   ✅ Function saved: cEvB-vC2XGDlxOtAxl7_C-CEjJtG1vOT_v-I6ie23hrAZVnbevpnO2UaJ854spKW6b4wHtAVuyD_-vozZB2b8A

2️⃣ Calling function (Insert + Verify)...
   ✅ function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: EP55Hdi3uQCblEzQCISkkYD5u7z1aU3kD_DxDEJZtwskUn2Rcv72yxIDRQnS9dG5rEX7rj2YGn_JTGtVaUvFJA
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'value': 'pending', 'type': 'String'}
   📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================
📝 function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: 1gNXWpQ-JcyL9mHbaqM-YxR9pT4c0AVWLm-wmvpuKaElJq6AGPqrgCmUhK118aIdQKOFpjrhEyc8y3czmOCxSw

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
   ✅ Function saved: _ixl6NC74KZeIQ9mtt9s30unMakTLGhhRr4sByQ-S2eaP5ZqJQRYURT_URBYSALRSwRRty4C_mBVs_dK99eAow

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
   ✅ Function saved: hBoRVgjU2Wu7jIjO8VbUiJ9ljWZe9XMcOXOdgYNFEpQc6NwZRddoUsyp_EqD04VcstYp_qDmGHPDYMs7zB8fYQ

2️⃣ Calling function (Query + Delete + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: cEvB-vC2XGDlxOtAxl7_...
   ✅ Deleted script: 1gNXWpQ-JcyL9mHbaqM-...
   ✅ Deleted script: _ixl6NC74KZeIQ9mtt9s...
   ✅ Deleted script: hBoRVgjU2Wu7jIjO8VbU...
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
  Output: Document ID = vNkdU5uF5FZ0y-VqoEUtKMowZL0CrFWQsvY1ALkNwMwsjaj_eWvLwgC_vwDbVuuh4MGXSbmLp4Gdx9jenSu5Zg
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(vNkdU5uF5FZ0y-VqoEUtKMowZL0CrFWQsvY1ALkNwMwsjaj_eWvLwgC_vwDbVuuh4MGXSbmLp4Gdx9jenSu5Zg)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(vNkdU5uF5FZ0y-VqoEUtKMowZL0CrFWQsvY1ALkNwMwsjaj_eWvLwgC_vwDbVuuh4MGXSbmLp4Gdx9jenSu5Zg)
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
  Output: Document ID = 5PzrkBB84GZ3Rw-oEBp9uUbX8nby1cuihKBfnrzHA-ms9gw_oQTBu-jlIFJSSgDkoyhWLERdkB3DocJTaD9Jpw
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(5PzrkBB84GZ3Rw-oEBp9uUbX8nby1cuihKBfnrzHA-ms9gw_oQTBu-jlIFJSSgDkoyhWLERdkB3DocJTaD9Jpw)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(5PzrkBB84GZ3Rw-oEBp9uUbX8nby1cuihKBfnrzHA-ms9gw_oQTBu-jlIFJSSgDkoyhWLERdkB3DocJTaD9Jpw)
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
    Finished `release` profile [optimized] target(s) in 0.19s
📦 Built wheel for abi3 Python ≥ 3.8 to ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.26.4-cp38-abi3-macosx_11_0_arm64.whl
📦 Installing Python wheel into .venv...
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.26.4-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.26.4
    Uninstalling ekodb_client-0.26.4:
      Successfully uninstalled ekodb_client-0.26.4
Successfully installed ekodb-client-0.26.4
🧪 Ensuring test dependencies (pytest) in .venv...
✅ Python client package built and installed!
bash: warning: setlocale: LC_ALL: cannot change locale (C.UTF-8): No such file or directory
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Advanced CRUD Example (Python) ===

--- insert ---
Inserted: aqM86JhjkPW7sw4LwSnTbLZeBgdIJRUQlFE6Aj74WW40KoIESgVk9aVwU3NWjP5F7G8Vf2jAHwjspAjPcv5ndw

--- update_with_action (increment) ---
After increment count by 5: {'id': 'aqM86JhjkPW7sw4LwSnTbLZeBgdIJRUQlFE6Aj74WW40KoIESgVk9aVwU3NWjP5F7G8Vf2jAHwjspAjPcv5ndw', 'name': {'type': 'String', 'value': 'Counter Record'}, 'tags': {'type': 'Array', 'value': ['initial']}, 'count': {'type': 'Integer', 'value': 15}, 'score': {'value': 50.0, 'type': 'Float'}}

--- update_with_action (decrement) ---
After decrement score by 10: {'name': {'type': 'String', 'value': 'Counter Record'}, 'id': 'aqM86JhjkPW7sw4LwSnTbLZeBgdIJRUQlFE6Aj74WW40KoIESgVk9aVwU3NWjP5F7G8Vf2jAHwjspAjPcv5ndw', 'score': {'value': 40.0, 'type': 'Float'}, 'tags': {'type': 'Array', 'value': ['initial']}, 'count': {'type': 'Integer', 'value': 15}}

--- update_with_action_sequence ---
After action sequence: {'id': 'aqM86JhjkPW7sw4LwSnTbLZeBgdIJRUQlFE6Aj74WW40KoIESgVk9aVwU3NWjP5F7G8Vf2jAHwjspAjPcv5ndw', 'name': {'type': 'String', 'value': 'Counter Record'}, 'tags': {'value': ['initial', 'sequenced'], 'type': 'Array'}, 'score': {'value': 40.0, 'type': 'Float'}, 'count': {'type': 'Integer', 'value': 115}}

--- cleanup ---
Cleaned up collection

=== Advanced CRUD example completed ===
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: LnBuzLJPwASGqnCQlUmhO-1c-qo1rcC7OuzGVFe0vmhLkxk_HNpdR-SGcLd6nLT15m11-fTUmBM2R0N8KXOWqQ

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you need more information or have any other questions, feel free to ask!

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['llm_provider', 'token_usage', 'chat_id', 'id', 'updated_at', 'content', 'created_at', 'context_snippets', 'llm_model', 'role'])
Debug: First message role: {'value': 'assistant', 'type': 'String'}
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you have any other questions or need further assistance, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: PblUNwUt_wNcVUJhMbyomvEWuBtNCEiXbReOivh-2e6abH23PqdXpOJNTJg6RIJOZTcjgq9Z3CTFDkPoORg7kQ
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: PblUNwUt_wNcVUJhMbyomvEWuBtNCEiXbReOivh-2e6abH23PqdXpOJNTJg6RIJOZTcjgq9Z3CTFDkPoORg7kQ
✓ Deleted chat session: LnBuzLJPwASGqnCQlUmhO-1c-qo1rcC7OuzGVFe0vmhLkxk_HNpdR-SGcLd6nLT15m11-fTUmBM2R0N8KXOWqQ
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: d8LtJHpYC_tm702s4o4sgae4mXMsGk5ATuVD8XQxHZV3cprhe4-YQYp61TL-c-AopjE2vl8RqAjOO3r1n20l8g

=== Sending Chat Message ===
Message ID: 9IjPNKJ5vZnKlE2rpTcWxrPpTO8udDzuThHvRDt4KQrovnYYluhDlay2I5ejh644jlH9PeknDfvSieDBX5uvdA

=== AI Response ===
The available products and their prices are as follows:

1. **ekoDB Cloud**: $499
   - Description: Fully managed cloud database service product

2. **ekoDB**: $99
   - Description: A high-performance database product with AI capabilities

3. **ekoDB Pro**: $299
   - Description: Enterprise edition product with advanced features

Execution Time: 2239ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 82
Total tokens: 3495

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Chat Message Stream (SSE) Example (Python) ===

Created session: d2jbEsqufBGFUSSPpH8BB2fd4CXpZNhqpCEmA8abcXgB3Jzg8z05ZsjBxJVgwmVlaHmqYy8ifqrTSWa0RYTX8A

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}EkoDB is a high-performance, embedded database specifically designed for use with the Elixir programming language. It focuses on providing fast access to data while being lightweight and easy to integrate into Elixir applications. EkoDB supports transactional capabilities and is optimized for concurrent access, making it suitable for applications that require efficient data storage and retrieval.

EkoDB is particularly suitable for scenarios where you want a database that can be embedded within an application without the need for a separate database server, thus reducing the overhead of managing a standalone database system. Its design leverages the strengths of the BEAM virtual machine, which Elixir and Erlang run on, providing excellent performance under concurrent workloads.

If you need more specific details or have particular questions about EkoDB, feel free to ask!

--- Stream complete ---
Message ID: 7oTL2uYA-skM95Dt4TndZ-L42OhbZucqA1W3GC2dFVnRbBVfVe5aMGgqKGI598Iz6xztDo7GfzTMv5JVIZSmtg
Execution time: 2307ms
Context window: 128000 tokens

✓ Chat message stream example completed
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

=== Get All Chat Models ===
OpenAI models: ['text-embedding-ada-002', 'whisper-1', 'gpt-3.5-turbo', 'tts-1', 'gpt-3.5-turbo-16k', 'gpt-4-0613', 'gpt-4', 'davinci-002', 'babbage-002', 'gpt-3.5-turbo-instruct', 'gpt-3.5-turbo-instruct-0914', 'gpt-3.5-turbo-1106', 'tts-1-hd', 'tts-1-1106', 'tts-1-hd-1106', 'text-embedding-3-small', 'text-embedding-3-large', 'gpt-3.5-turbo-0125', 'gpt-4-turbo', 'gpt-4-turbo-2024-04-09', 'gpt-4o', 'gpt-4o-2024-05-13', 'gpt-4o-mini-2024-07-18', 'gpt-4o-mini', 'gpt-4o-2024-08-06', 'omni-moderation-latest', 'omni-moderation-2024-09-26', 'o1-2024-12-17', 'o1', 'o3-mini', 'o3-mini-2025-01-31', 'gpt-4o-2024-11-20', 'gpt-4o-mini-search-preview-2025-03-11', 'gpt-4o-mini-search-preview', 'gpt-4o-transcribe', 'gpt-4o-mini-transcribe', 'o1-pro-2025-03-19', 'o1-pro', 'gpt-4o-mini-tts', 'o3-2025-04-16', 'o4-mini-2025-04-16', 'o3', 'o4-mini', 'gpt-4.1-2025-04-14', 'gpt-4.1', 'gpt-4.1-mini-2025-04-14', 'gpt-4.1-mini', 'gpt-4.1-nano-2025-04-14', 'gpt-4.1-nano', 'gpt-image-1', 'o4-mini-deep-research', 'gpt-4o-transcribe-diarize', 'o4-mini-deep-research-2025-06-26', 'gpt-5-chat-latest', 'gpt-5-2025-08-07', 'gpt-5', 'gpt-5-mini-2025-08-07', 'gpt-5-mini', 'gpt-5-nano-2025-08-07', 'gpt-5-nano', 'gpt-audio-2025-08-28', 'gpt-realtime', 'gpt-realtime-2025-08-28', 'gpt-audio', 'gpt-5-codex', 'gpt-image-1-mini', 'gpt-5-pro-2025-10-06', 'gpt-5-pro', 'gpt-audio-mini', 'gpt-audio-mini-2025-10-06', 'gpt-5-search-api', 'gpt-realtime-mini', 'sora-2', 'sora-2-pro', 'gpt-5-search-api-2025-10-14', 'gpt-5.1-chat-latest', 'gpt-5.1-2025-11-13', 'gpt-5.1', 'gpt-5.1-codex', 'gpt-5.1-codex-mini', 'gpt-5.1-codex-max', 'gpt-image-1.5', 'gpt-5.2-2025-12-11', 'gpt-5.2', 'gpt-5.2-pro-2025-12-11', 'gpt-5.2-pro', 'gpt-5.2-chat-latest', 'gpt-4o-mini-transcribe-2025-12-15', 'gpt-4o-mini-transcribe-2025-03-20', 'gpt-4o-mini-tts-2025-03-20', 'gpt-4o-mini-tts-2025-12-15', 'gpt-realtime-mini-2025-12-15', 'gpt-audio-mini-2025-12-15', 'chatgpt-image-latest', 'gpt-5.2-codex', 'gpt-5.3-codex', 'gpt-realtime-1.5', 'gpt-audio-1.5', 'gpt-4o-search-preview', 'gpt-4o-search-preview-2025-03-11', 'gpt-5.3-chat-latest', 'gpt-5.4-2026-03-05', 'gpt-5.4-pro', 'gpt-5.4-pro-2026-03-05', 'gpt-5.4', 'gpt-5.4-nano-2026-03-17', 'gpt-5.4-nano', 'gpt-5.4-mini-2026-03-17', 'gpt-5.4-mini', 'gpt-image-2', 'gpt-image-2-2026-04-21', 'gpt-5.5', 'gpt-5.5-2026-04-23', 'gpt-5.5-pro', 'gpt-5.5-pro-2026-04-23', 'chat-latest', 'gpt-realtime-translate', 'gpt-realtime-2', 'gpt-realtime-whisper', 'gpt-5.6-sol', 'gpt-5.6-terra', 'gpt-5.6-luna', 'gpt-realtime-2.1', 'gpt-realtime-2.1-mini', 'gpt-transcribe', 'gpt-live-transcribe', 'gpt-6-astra', 'gpt-image-2.5-flare', 'gpt-image-2.5-sunburst', 'gpt-image-2.5-flare-2026-09-08', 'gpt-image-2.5-sunburst-2026-09-08', 'gpt-live-1']
Anthropic models: ['claude-fable-5-1', 'claude-opus-5', 'claude-sonnet-5', 'claude-fable-5', 'claude-opus-4-8', 'claude-opus-4-7', 'claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929']
Perplexity models: []
Gemini models: []

=== Get OpenAI Models ===
OpenAI models: ['text-embedding-ada-002', 'whisper-1', 'gpt-3.5-turbo', 'tts-1', 'gpt-3.5-turbo-16k', 'gpt-4-0613', 'gpt-4', 'davinci-002', 'babbage-002', 'gpt-3.5-turbo-instruct', 'gpt-3.5-turbo-instruct-0914', 'gpt-3.5-turbo-1106', 'tts-1-hd', 'tts-1-1106', 'tts-1-hd-1106', 'text-embedding-3-small', 'text-embedding-3-large', 'gpt-3.5-turbo-0125', 'gpt-4-turbo', 'gpt-4-turbo-2024-04-09', 'gpt-4o', 'gpt-4o-2024-05-13', 'gpt-4o-mini-2024-07-18', 'gpt-4o-mini', 'gpt-4o-2024-08-06', 'omni-moderation-latest', 'omni-moderation-2024-09-26', 'o1-2024-12-17', 'o1', 'o3-mini', 'o3-mini-2025-01-31', 'gpt-4o-2024-11-20', 'gpt-4o-mini-search-preview-2025-03-11', 'gpt-4o-mini-search-preview', 'gpt-4o-transcribe', 'gpt-4o-mini-transcribe', 'o1-pro-2025-03-19', 'o1-pro', 'gpt-4o-mini-tts', 'o3-2025-04-16', 'o4-mini-2025-04-16', 'o3', 'o4-mini', 'gpt-4.1-2025-04-14', 'gpt-4.1', 'gpt-4.1-mini-2025-04-14', 'gpt-4.1-mini', 'gpt-4.1-nano-2025-04-14', 'gpt-4.1-nano', 'gpt-image-1', 'o4-mini-deep-research', 'gpt-4o-transcribe-diarize', 'o4-mini-deep-research-2025-06-26', 'gpt-5-chat-latest', 'gpt-5-2025-08-07', 'gpt-5', 'gpt-5-mini-2025-08-07', 'gpt-5-mini', 'gpt-5-nano-2025-08-07', 'gpt-5-nano', 'gpt-audio-2025-08-28', 'gpt-realtime', 'gpt-realtime-2025-08-28', 'gpt-audio', 'gpt-5-codex', 'gpt-image-1-mini', 'gpt-5-pro-2025-10-06', 'gpt-5-pro', 'gpt-audio-mini', 'gpt-audio-mini-2025-10-06', 'gpt-5-search-api', 'gpt-realtime-mini', 'sora-2', 'sora-2-pro', 'gpt-5-search-api-2025-10-14', 'gpt-5.1-chat-latest', 'gpt-5.1-2025-11-13', 'gpt-5.1', 'gpt-5.1-codex', 'gpt-5.1-codex-mini', 'gpt-5.1-codex-max', 'gpt-image-1.5', 'gpt-5.2-2025-12-11', 'gpt-5.2', 'gpt-5.2-pro-2025-12-11', 'gpt-5.2-pro', 'gpt-5.2-chat-latest', 'gpt-4o-mini-transcribe-2025-12-15', 'gpt-4o-mini-transcribe-2025-03-20', 'gpt-4o-mini-tts-2025-03-20', 'gpt-4o-mini-tts-2025-12-15', 'gpt-realtime-mini-2025-12-15', 'gpt-audio-mini-2025-12-15', 'chatgpt-image-latest', 'gpt-5.2-codex', 'gpt-5.3-codex', 'gpt-realtime-1.5', 'gpt-audio-1.5', 'gpt-4o-search-preview', 'gpt-4o-search-preview-2025-03-11', 'gpt-5.3-chat-latest', 'gpt-5.4-2026-03-05', 'gpt-5.4-pro', 'gpt-5.4-pro-2026-03-05', 'gpt-5.4', 'gpt-5.4-nano-2026-03-17', 'gpt-5.4-nano', 'gpt-5.4-mini-2026-03-17', 'gpt-5.4-mini', 'gpt-image-2', 'gpt-image-2-2026-04-21', 'gpt-5.5', 'gpt-5.5-2026-04-23', 'gpt-5.5-pro', 'gpt-5.5-pro-2026-04-23', 'chat-latest', 'gpt-realtime-translate', 'gpt-realtime-2', 'gpt-realtime-whisper', 'gpt-5.6-sol', 'gpt-5.6-terra', 'gpt-5.6-luna', 'gpt-realtime-2.1', 'gpt-realtime-2.1-mini', 'gpt-transcribe', 'gpt-live-transcribe', 'gpt-6-astra', 'gpt-image-2.5-flare', 'gpt-image-2.5-sunburst', 'gpt-image-2.5-flare-2026-09-08', 'gpt-image-2.5-sunburst-2026-09-08', 'gpt-live-1']

=== Get Anthropic Models ===
Anthropic models: ['claude-fable-5-1', 'claude-opus-5', 'claude-sonnet-5', 'claude-fable-5', 'claude-opus-4-8', 'claude-opus-4-7', 'claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929']

✓ Chat Models API example complete
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: oU2biI16LkKeN5i_IyBuQxxuujQp6RtWRlBHUnF17Bkgv3dcM20VRUwVz578P3nj7KmMhfGg9ilOR_iq5jI-UA

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99

If you need more details or have other questions, feel free to ask!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: D5-jZ63H9q4tt_Rv8EoZUi7qV_LxdHn0Fthwil8JBmSVd30GcPAM4b2zer9Q6nOP-7mTnrkumglcFawMr8nb3g
  Parent: oU2biI16LkKeN5i_IyBuQxxuujQp6RtWRlBHUnF17Bkgv3dcM20VRUwVz578P3nj7KmMhfGg9ilOR_iq5jI-UA

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: D5-jZ63H9q4tt_Rv8EoZUi7qV_LxdHn0Fthwil8JBmSVd30GcPAM4b2zer9Q6nOP-7mTnrkumglcFawMr8nb3g (Untitled)
  Session 2: oU2biI16LkKeN5i_IyBuQxxuujQp6RtWRlBHUnF17Bkgv3dcM20VRUwVz578P3nj7KmMhfGg9ilOR_iq5jI-UA (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: D5-jZ63H9q4tt_Rv8EoZUi7qV_LxdHn0Fthwil8JBmSVd30GcPAM4b2zer9Q6nOP-7mTnrkumglcFawMr8nb3g

=== Cleanup ===
✓ Deleted sessions and collection

✓ All session management operations completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "_880851ZpRaOlHOpj6IL3C_3AeBEaD0VYnKnMDK_iL5Eq85tK_Cci6eO90jToLPKMWcCLVofcjJIDc-RAjufVw"

=== List Collections ===
Total collections: 13
Sample collections: ['chat_agent_configs__ek0_testing', 'agent_function_versions__ek0_testing', 'client_collection_management_python', 'schedules__ek0_testing', 'functions__ek0_testing']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': 'UuZqbQLFX9209NKfyFPQRaMjGyC82kaMangHnRkiat83hVKIZFihD5Z04YWjpLJARCTS59-I7f7soG3ffincAA'}

=== Upsert Operation ===
✓ Upsert (update existing record): UuZqbQLFX9209NKfyFPQRaMjGyC82kaMangHnRkiat83hVKIZFihD5Z04YWjpLJARCTS59-I7f7soG3ffincAA
✓ Inserted second record: 4imi8hLOz-1OJRE-OsPEy4f_rSuAC_MWElFRrZW6jfE--Ct3gZosPXykkkk5zd_MZR2OrZci2GSJFegMHG9sdw
✓ Upsert (update second record): 4imi8hLOz-1OJRE-OsPEy4f_rSuAC_MWElFRrZW6jfE--Ct3gZosPXykkkk5zd_MZR2OrZci2GSJFegMHG9sdw

=== Find One Operation ===
✓ Found user by email: {'active': {'type': 'Boolean', 'value': True}, 'age': {'type': 'Integer', 'value': 29}, 'id': 'UuZqbQLFX9209NKfyFPQRaMjGyC82kaMangHnRkiat83hVKIZFihD5Z04YWjpLJARCTS59-I7f7soG3ffincAA', 'email': {'type': 'String', 'value': 'alice.j@newdomain.com'}, 'name': {'type': 'String', 'value': 'Alice Johnson'}}
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: ama0yiTT93xI7mRlE5SzKYmmEPx7Xf-58ooldS9drtPa9pgW216FbV4uELq1W6kWydtVg-kBopFzaDAWNECWHg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: lpPdHomV0qnCX-N1NFzEyAluq6LDjxupYRSpT-L75-oPy36pCX67ItRTk9Bcfi6621iPrO6frzZi2gzNsbz79Q

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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB as Edge Cache - Simple Example ===

Creating edge cache function...
✓ Edge cache script created: 04t0Hil8MnDnr6wDBmJFm3wfgbGFPDjBWZ4qlR618H9sqSGVJQMWEpZFE9dnuDUtUOnkswEs49DbT4yzRSzf-w

Call 1: Cache miss (fetches from API)
Response time: 439ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "current": {
            "interval": 900,
            "temperature_2m": 20.0,
            "time": "2026-09-16T13:45"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "\u00b0C",
            "time": "iso8601"
          },
          "elevation": 32.0,
          "generationtime_ms": 0.03254413604736328,
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
Response time: 12ms (37.0x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "current": {
            "interval": 900,
            "temperature_2m": 20.0,
            "time": "2026-09-16T13:45"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "\u00b0C",
            "time": "iso8601"
          },
          "elevation": 32.0,
          "generationtime_ms": 0.03254413604736328,
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

perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
   ⏱️  Duration: 102.7ms
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
   ⏱️  Duration: 10.2ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "type": "Object",
            "value": {
                  "address": {
                        "city": "Gwenborough",
                        "geo": {
                 ...
   🚀 Cache speedup: 10.0x faster!

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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
client_function_contract: ok
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
🚀 ekoDB Functions Example (Python)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: uBTrqc9g8xgrY3yfXCvwj0RjoBpGV8eABjKx6Pt9NN8qPNBN2argQTmPhaUdABFHNzn6vPr08kI-8lGYKZin9g
📊 Found 5 active users

📝 Example 2: Parameterized Function

✅ Function saved: Q7vNTMfUB0R7p18d8r-3LzRJQmv5__c5jaK3Vm9WSM_IBXeS88RDw9aoxnIiCNngKSF_sctZhxTtqTy0wwoXWA
📊 Found 3 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: K-SpJpHmEcyordqYtWLwxiadH5RjiQBBQvZzhU-ejDxtwb17Q_WfYEAEPLeBQVLHp7SUPwb81flo-VJONZesgg
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
🚀 ekoDB Python AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer several benefits:

1. **Efficient Similarity Search**: They enable fast retrieval of similar items using vector embeddings, which is essential for applications like image and text similarity.

2. **High Dimensional Data Handling**: They can efficiently manage and query high-dimensional data, making them suitable for machine learning and AI applications.

3. **Scalability**: Vector databases are designed to scale with large datasets, providing robust performance as data grows.

4. **Real-Time Processing**: They support real-time data processing and querying, which is critical for dynamic applications.

5. **Advanced Indexing Techniques**: Use of specialized indexing techniques (e.g., Annoy, HNSW) to improve search speed and accuracy.

6. **Integration with ML Models**: They easily integrate with machine learning frameworks, enabling seamless workflows from model training to deployment.

7. **Diverse Data Types**: Ability to store and query various data types (text, images, audio) in a unified manner.

8. **Enhanced Recommendation Systems**: Improve recommendation systems through better user-item similarity calculations.

These advantages make vector databases particularly valuable in fields like natural language processing, computer vision, and recommendation systems.
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
🚀 ekoDB Python Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: 1Az68NmOcpikvGyF8TGvTr5s4JQQGJ5eMkg_7NjHKI7EGJwjjijYgpSjCG2m8OtzogGgfqLHxrNle1YUA0KIng
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
   {'category': {'type': 'String', 'value': 'Electronics'}, 'count': {'type': 'Integer', 'value': 3}}
   {'category': {'type': 'String', 'value': 'Furniture'}, 'count': {'type': 'Integer', 'value': 2}}
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

perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
🚀 ekoDB Python KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: cO6ej5ddtcabksB-r6i6VrV3j_CQKfb6hjSXIg_28tMnD6knMeUHFRd2AZ08Em7QQgMuZHRqnipjf0zAJNQuaA
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

✅ Inserted order: _Nc7sXcocnckJf_QLGbzlvmbjs9LxTIAVeFHg6cXTKZaCk8qqSsP2PSY4ExzDJet_5QLL1zKCyc2SZMLhO1q2w
✅ Cached order status
📊 Quick status lookup: {'value': '{"type":"Object","value":{"status":"processing","updated_at":"2026-09-16T13:46:38.975619+00:00"}}'}

🧹 Cleaning up...
✅ Cleanup complete

✅ All KV & Wrapped Types examples completed!

💡 Key takeaways:
   ✅ Use field_* helpers for type-safe wrapped values
   ✅ field_decimal() preserves precision (no floating point errors)
   ✅ KV store is great for caching and quick lookups
   ✅ Combine KV caching with collection inserts for real workflows
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
🚀 ekoDB Python Search Functions Example

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
   {'category': {'type': 'String', 'value': 'Database'}, 'count': {'type': 'Integer', 'value': 3}}
   {'category': {'type': 'String', 'value': 'AI'}, 'count': {'type': 'Integer', 'value': 2}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Goal Template CRUD Example (Python) ===

--- Creating goal template ---
Created template: Data Migration (id: AuHv6TdHjDt1-RbjlghEBYgJHaYo3sDUs2iwKOUsn5ZIBT6rYedHCuJvhQAM6JIGkx9uEcWhiGnuHLKdCCKEJg)

--- Listing templates ---
Templates: {'count': 1, 'items': [{'description': {'type': 'String', 'value': 'Template for migrating data between schemas'}, 'id': 'AuHv6TdHjDt1-RbjlghEBYgJHaYo3sDUs2iwKOUsn5ZIBT6rYedHCuJvhQAM6JIGkx9uEcWhiGnuHLKdCCKEJg', 'steps': {'type': 'Array', 'value': [{'description': 'Analyze source schema'}, {'description': 'Create target schema'}, {'description': 'Migrate records'}, {'description': 'Validate results'}]}, 'title': {'type': 'String', 'value': 'Data Migration'}}]}

--- Getting template ---
Fetched: {'type': 'String', 'value': 'Data Migration'}

--- Updating template ---
Updated description: {'type': 'String', 'value': 'Updated: comprehensive data migration workflow'}

--- Deleting template ---
Template deleted successfully

✓ Goal template CRUD example completed
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Goals / Tasks / Agents Integration Example (Python) ===

--- goal_create ---
Created goal: Deploy v3 (id: bDL5pHK0PGrg0tSGOgbzgYc4LEyEdawMZEjYIagxXGNxm2lsEEwePDGcjH1GA6mfDguiQxzmCfnV-95Bzk8Neg)

--- goal_list ---
Goals: {'count': 1, 'goals': [{'created_at': '2026-09-16T13:46:39.430861+00:00', 'description': 'Deploy version 3 to production', 'id': 'bDL5pHK0PGrg0tSGOgbzgYc4LEyEdawMZEjYIagxXGNxm2lsEEwePDGcjH1GA6mfDguiQxzmCfnV-95Bzk8Neg', 'status': 'pending', 'steps': '[{"description":"Build Docker image","status":"pending"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]', 'title': 'Deploy v3', 'updated_at': '2026-09-16T13:46:39.430861+00:00'}]}

--- goal_get ---
Fetched: {'type': 'String', 'value': 'Deploy v3'}

--- goal_update ---
Updated description: {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}

--- goal_search ---
Search results: {'count': 1, 'items': [{'_score': 13.2, 'created_at': {'type': 'DateTime', 'value': '2026-09-16T13:46:39.430861+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'bDL5pHK0PGrg0tSGOgbzgYc4LEyEdawMZEjYIagxXGNxm2lsEEwePDGcjH1GA6mfDguiQxzmCfnV-95Bzk8Neg', 'status': {'type': 'String', 'value': 'pending'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","status":"pending"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-09-16T13:46:39.451770+00:00'}}]}

--- goal_step_start (step 0) ---
Step 0 started: {'created_at': {'type': 'DateTime', 'value': '2026-09-16T13:46:39.430861+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'bDL5pHK0PGrg0tSGOgbzgYc4LEyEdawMZEjYIagxXGNxm2lsEEwePDGcjH1GA6mfDguiQxzmCfnV-95Bzk8Neg', 'status': {'type': 'String', 'value': 'in_progress'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","status":"InProgress"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-09-16T13:46:39.462377+00:00'}}

--- goal_step_complete (step 0) ---
Step 0 completed: {'created_at': {'type': 'DateTime', 'value': '2026-09-16T13:46:39.430861+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'bDL5pHK0PGrg0tSGOgbzgYc4LEyEdawMZEjYIagxXGNxm2lsEEwePDGcjH1GA6mfDguiQxzmCfnV-95Bzk8Neg', 'status': {'type': 'String', 'value': 'in_progress'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","result":"Docker image built: sha256:abc123","status":"Completed"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-09-16T13:46:39.468647+00:00'}}

--- goal_step_fail (step 1) ---
Step 1 failed: {'created_at': {'type': 'DateTime', 'value': '2026-09-16T13:46:39.430861+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'bDL5pHK0PGrg0tSGOgbzgYc4LEyEdawMZEjYIagxXGNxm2lsEEwePDGcjH1GA6mfDguiQxzmCfnV-95Bzk8Neg', 'status': {'type': 'String', 'value': 'in_progress'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","result":"Docker image built: sha256:abc123","status":"Completed"},{"description":"Run migrations","error":"Migration failed: column already exists","status":"Failed"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-09-16T13:46:39.475090+00:00'}}

--- goal_complete ---
Goal completed: {'type': 'String', 'value': 'pending_review'}

--- goal_approve ---
Goal approved: {'type': 'String', 'value': 'in_progress'}

--- goal_reject ---
Goal rejected: {'type': 'String', 'value': 'failed'}

--- task_create ---
Created task: Nightly Backup (id: STMTtV3OhmZfvVi1jECkojYI8Poh8Y0RKDuGLpjKsOX7ga0WiwMGYmSnk0Ybz7pQ7XNoaBUxUQtldeLQKndG1w)

--- task_list ---
Tasks: {'count': 1, 'items': [{'cron': {'type': 'String', 'value': '0 2 * * *'}, 'description': {'type': 'String', 'value': 'Backup all databases nightly'}, 'id': 'STMTtV3OhmZfvVi1jECkojYI8Poh8Y0RKDuGLpjKsOX7ga0WiwMGYmSnk0Ybz7pQ7XNoaBUxUQtldeLQKndG1w', 'max_consecutive_failures': {'type': 'Integer', 'value': 3}, 'name': {'type': 'String', 'value': 'Nightly Backup'}}]}

--- task_get ---
Fetched: {'type': 'String', 'value': 'Nightly Backup'}

--- task_start ---
Task started: {'type': 'String', 'value': 'running'}

--- task_succeed ---
Task succeeded: {'consecutive_failures': {'type': 'Integer', 'value': 0}, 'cron': {'type': 'String', 'value': '0 2 * * *'}, 'description': {'type': 'String', 'value': 'Backup all databases nightly'}, 'id': 'STMTtV3OhmZfvVi1jECkojYI8Poh8Y0RKDuGLpjKsOX7ga0WiwMGYmSnk0Ybz7pQ7XNoaBUxUQtldeLQKndG1w', 'last_error': {'type': 'Null', 'value': None}, 'last_run': {'type': 'DateTime', 'value': '2026-09-16T13:46:39.546563+00:00'}, 'max_consecutive_failures': {'type': 'Integer', 'value': 3}, 'name': {'type': 'String', 'value': 'Nightly Backup'}, 'run_count': {'type': 'Integer', 'value': 1}, 'status': {'type': 'String', 'value': 'active'}, 'updated_at': {'type': 'DateTime', 'value': '2026-09-16T13:46:39.546563+00:00'}}

--- task_pause ---
Task paused: {'type': 'String', 'value': 'paused'}

--- task_resume ---
Task resumed: {'type': 'String', 'value': 'active'}

--- task_fail ---
Task failed: {'consecutive_failures': {'type': 'Integer', 'value': 1}, 'cron': {'type': 'String', 'value': '0 2 * * *'}, 'description': {'type': 'String', 'value': 'Backup all databases nightly'}, 'id': 'STMTtV3OhmZfvVi1jECkojYI8Poh8Y0RKDuGLpjKsOX7ga0WiwMGYmSnk0Ybz7pQ7XNoaBUxUQtldeLQKndG1w', 'last_error': {'type': 'String', 'value': 'Disk full'}, 'last_run': {'type': 'DateTime', 'value': '2026-09-16T13:46:39.580430+00:00'}, 'max_consecutive_failures': {'type': 'Integer', 'value': 3}, 'name': {'type': 'String', 'value': 'Nightly Backup'}, 'next_run': {'type': 'DateTime', 'value': '2026-03-22T02:00:00+00:00'}, 'run_count': {'type': 'Integer', 'value': 1}, 'status': {'type': 'String', 'value': 'active'}, 'updated_at': {'type': 'DateTime', 'value': '2026-09-16T13:46:39.580430+00:00'}}

--- task_due ---
Due tasks: {'count': 0, 'items': []}

--- task_delete ---
Task deleted

--- agent_create ---
Created agent: CodeReviewer (id: XIDwOsl1zc59N9RYvoqX_7GkCvQ2Fs7LqAmJHrTh7MmEeXtB4E94coLPiTS9mJ57mBiviiyxAzEpehW_L8PYIA)

--- agent_list ---
Agents: {'count': 1, 'items': [{'deployment_id': {'type': 'String', 'value': 'deploy_test'}, 'id': 'XIDwOsl1zc59N9RYvoqX_7GkCvQ2Fs7LqAmJHrTh7MmEeXtB4E94coLPiTS9mJ57mBiviiyxAzEpehW_L8PYIA', 'llm_model': {'type': 'String', 'value': 'gpt-4o'}, 'name': {'type': 'String', 'value': 'CodeReviewer'}, 'system_prompt': {'type': 'String', 'value': 'You review code for correctness and style.'}, 'tools': {'type': 'Array', 'value': ['web_search', 'file_read']}}]}

--- agent_get ---
Fetched: {'type': 'String', 'value': 'CodeReviewer'}

--- agent_get_by_name ---
By name: {'type': 'String', 'value': 'CodeReviewer'}

--- agent_update ---
Updated agent prompt: {'type': 'String', 'value': 'You review code. Be concise.'}

--- agents_by_deployment ---
Agents in deploy_test: {'count': 0, 'items': []}
WARNING: agents_by_deployment omitted created agent XIDwOsl1zc59N9RYvoqX_7GkCvQ2Fs7LqAmJHrTh7MmEeXtB4E94coLPiTS9mJ57mBiviiyxAzEpehW_L8PYIA; TODO: check/fix the server-side deployment lookup

--- agent_delete ---
Agent deleted

--- cleanup ---
Goals cleaned up

=== All goals/tasks/agents operations completed successfully ===
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB KV Links Example (Python) ===

--- kv_set ---
Set KV key: app:config

--- insert document ---
Inserted document: U4z5TpCZJQfXc3du_-UM0WXrbXOUya1mezFtSPlfIN5tJx3hrTcRKHMeyENao2L_hgkjuwN9LXJzKbECH2Xwig

--- kv_link ---
Linked: None

--- kv_get_links ---
Links for app:config: [{'collection': 'kv_links_example_py', 'created_at': '2026-09-16T13:46:40.281713Z', 'document_id': 'U4z5TpCZJQfXc3du_-UM0WXrbXOUya1mezFtSPlfIN5tJx3hrTcRKHMeyENao2L_hgkjuwN9LXJzKbECH2Xwig', 'field_path': None, 'last_accessed': '2026-09-16T13:46:40.282939Z', 'metadata': {}}]

--- kv_unlink ---
Unlinked: None

--- kv_get_links (verify empty) ---
Links after unlink: []

--- cleanup ---
Cleaned up KV key and collection

=== KV links example completed ===
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Schedule Management Example (Python) ===

--- create_schedule ---
Created schedule: Nightly Report (id: cf536bf3-55eb-48f8-9fcd-eb9f3e5ee824)

--- list_schedules ---
Schedules: {'count': 1, 'schedules': [{'created_at': '2026-09-16T13:46:43.864278Z', 'cron_expression': '0 0 0 * * *', 'description': 'Generate and email nightly analytics report', 'enabled': True, 'function_label': 'schedule_noop_python_15331', 'id': 'cf536bf3-55eb-48f8-9fcd-eb9f3e5ee824', 'last_execution': None, 'name': 'Nightly Report', 'next_execution': '2026-09-17T00:00:00Z', 'parameters': {}, 'stats': {'avg_execution_time_ms': 0.0, 'failed_executions': 0, 'last_error': None, 'successful_executions': 0, 'total_executions': 0}, 'timezone': 'UTC', 'updated_at': '2026-09-16T13:46:43.864278Z'}]}

--- get_schedule ---
Fetched: Nightly Report cron=0 0 0 * * *

--- update_schedule ---
Updated cron: 0 30 1 * * *

--- trigger_schedule ---
Triggered: {'schedule_id': 'cf536bf3-55eb-48f8-9fcd-eb9f3e5ee824', 'status': 'triggered'}

--- pause_schedule ---
Paused: enabled=False

--- resume_schedule ---
Resumed: enabled=True
Schedule deleted successfully

=== Schedule management example completed ===
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
  1. Score: 0.767
  2. Score: 0.762
  3. Score: 0.739

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.381, Matched: title, content
  2. Score: 0.884, Matched: title, content
  3. Score: 0.370, Matched:

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 6.600, Matched: title

9. Vector search with a metadata pre-filter (category = ml):
Found 2 documents in category "ml" (NLP excluded)
  1. Deep Learning Fundamentals (category: ml)
  2. Introduction to Machine Learning (category: ml)

=== Cleanup ===
✅ Deleted test collections

✅ Search examples completed!
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': 'N-jyqJKt4chumKeEwGLoDr_Mt1BzZ3w98K7EC8gbnD7fQMu9DWBrtly37pGLR-l8474E1rE_4Ij6midLbfqj6A'}

=== Find by ID ===
Found: {'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}, 'name': {'type': 'String', 'value': 'Test Record'}, 'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'created_at': {'type': 'String', 'value': '2026-09-16T09:46:44.552501'}, 'metadata': {'type': 'Object', 'value': {'nested': {'deep': True}, 'key': 'value'}}, 'price': {'type': 'Float', 'value': 99.99}, 'value': {'value': 42, 'type': 'Integer'}, 'id': 'N-jyqJKt4chumKeEwGLoDr_Mt1BzZ3w98K7EC8gbnD7fQMu9DWBrtly37pGLR-l8474E1rE_4Ij6midLbfqj6A', 'active': {'value': True, 'type': 'Boolean'}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-09-16 09:46:44.552501
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'nested': {'deep': True}, 'key': 'value'}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'tags': ['tag1', 'tag2', 'tag3'], 'name': 'Test Record', 'data': 'aGVsbG8gd29ybGQ=', 'categories': ['electronics', 'computers'], 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'created_at': '2026-09-16T09:46:44.552501', 'metadata': {'nested': {'deep': True}, 'key': 'value'}, 'price': 99.99, 'value': 42, 'id': 'N-jyqJKt4chumKeEwGLoDr_Mt1BzZ3w98K7EC8gbnD7fQMu9DWBrtly37pGLR-l8474E1rE_4Ij6midLbfqj6A', 'active': True}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'created_at': {'type': 'String', 'value': '2026-09-16T09:46:44.552501'}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'metadata': {'value': {'key': 'value', 'nested': {'deep': True}}, 'type': 'Object'}, 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'name': {'value': 'Updated Record', 'type': 'String'}, 'id': 'N-jyqJKt4chumKeEwGLoDr_Mt1BzZ3w98K7EC8gbnD7fQMu9DWBrtly37pGLR-l8474E1rE_4Ij6midLbfqj6A', 'value': {'type': 'Integer', 'value': 100}, 'price': {'type': 'Float', 'value': 99.99}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'active': {'type': 'Boolean', 'value': True}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: d60DCX9GnxIKQsDpagOUzX2xmIKZtyU89-K6jIyshEpRUocVoMj1xBKKVYKM3c-v21kOSTSV7VpsAFvJDTEBpA

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
✓ Created native SWR script: github_user_native_py (xxZRSrRhnZaSTLnQLruEdF-V8vOpgMgyn1PIEGKNjV86T6ht_OICPgXbtae5MSo1XfNeJQkUzeaV0PrWbT09-w)

First call (cache miss - will fetch from GitHub API):
  Response time: 160ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 4ms
  Speedup: 38.9x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_py (BbAIc7ghYyGfeNUFbYU5NIDFyHSQwe-LvKn5MKfTpPm6fl6FhpjykrRq1tybVbbDw_geCee3myS11qfGpkr_eg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_py' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_py (XHbikHblkYdNoShLSTDacMgtjXxgoxnSmKTjL0FAB7Ebb13KjeK3Q3AUVuRxdkWHjn0HDNnaGbcBUCeFdXX-wQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_py' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_py (Zq_uMGdi__HW7v131_fyoYI4PXaKs9RpAhjNZQsCNdSwpCLn9Mh9cr1hJYTp4uyCmgJrKMPQ-WsraWM6LlinuA)
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_py (4xgBQBp-RoILrO5Ipdr09WufQuaQbTVoJmelROSED7wAtpL783yPsSUQiEzkduVVmlH0yFoSfjQ5IAE5nupzJA)

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
Response time: 6ms (served from cache)
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: qzE2Ohr-Pn04Lm5t2PLqd1TaF06MQf_SSrUY94v1hf1p2qK0T4JSBa9wxUJhZjgqgtzIl12HL8FxHaOp0KO5Ww
Created Bob: $500 - ID: a2-ixUxm5rJsDPFl8jimqo7DSN5Oc-yBvKr1GNTZ1W2Cfo1t01goYIOJd3AuPaH4dI400ADV0KAWnnXov2OJNg

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 255cf96b-c6cf-4b94-b8bc-2b2541f1f723

=== Example 2: Operations within Transaction ===
Alice in transaction: {'id': 'qzE2Ohr-Pn04Lm5t2PLqd1TaF06MQf_SSrUY94v1hf1p2qK0T4JSBa9wxUJhZjgqgtzIl12HL8FxHaOp0KO5Ww', 'name': {'type': 'String', 'value': 'Alice'}, 'balance': {'value': 800, 'type': 'Integer'}, 'account_id': {'type': 'String', 'value': 'ACC001'}}
Bob in transaction: {'id': 'a2-ixUxm5rJsDPFl8jimqo7DSN5Oc-yBvKr1GNTZ1W2Cfo1t01goYIOJd3AuPaH4dI400ADV0KAWnnXov2OJNg', 'balance': {'type': 'Integer', 'value': 700}, 'name': {'type': 'String', 'value': 'Bob'}, 'account_id': {'value': 'ACC002', 'type': 'String'}}
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

✓ Verified committed balances: Alice=$800, Bob=$700

=== Example 5: Rollback Demo ===
New transaction: 38cbd076-acca-4a4e-9140-90fd8c3c7b49
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

=== Create User Function ===
Created user function with ID: cikQHqX29yz_g8VdcDYS8SZs3o8EhOOxEmzCyC7yIRKdfVcdHpmVoajnngCj3gUDievScrqsrp3Fq32ZJAdXYA

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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== WebSocket Chat Streaming Example (Python) ===

Created chat session: 8X7E13QPW5EJp8uVTxi94mr0Whg69J-rK-gEXlsvfga6DNhMNarxtcG4t1DlPUR8ynq-PO4xqfzvts9WN85UrA

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: tosx-JmwfYh2ujqkvdKMxTo9TDgcnYODkw708jvG1CXe0fOqg5TbJXmYmRESh0OvaNgI_OYvoccXHCZi_z4vyA
Execution time: 870ms
Token usage: {'completion_tokens': 8, 'prompt_tokens': 15, 'total_tokens': 23}

Full response: {"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}The capital of France is Paris....
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== WebSocket Subscription Example (Python) ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_py' ===
✓ Subscribed (subscription_id: sub_f2670ef209344356a9e118212bb4ef18)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: JWGtopGtQlVlfAwgxYqbMrD7d-mspY4Jf-lAINnj6MujzSb__AkyjCq9wn9X7h4H-QUHOXAMDSDNQrdCiXcYlg

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_py
     Record IDs: JWGtopGtQlVlfAwgxYqbMrD7d-mspY4Jf-lAINnj6MujzSb__AkyjCq9wn9X7h4H-QUHOXAMDSDNQrdCiXcYlg
     Timestamp:  2026-09-16T13:46:47.501309+00:00

Inserting record 2...
✓ Inserted: Q2IixamaWeZb8UZTF3wllCBzQHEXUelak2R0Ok35JmVxhQvsUveUAbyslsTwfsozjaSSDBiuBTKjgAh4kLZnuw

  📡 Notification received:
     Event:      insert
     Record IDs: Q2IixamaWeZb8UZTF3wllCBzQHEXUelak2R0Ok35JmVxhQvsUveUAbyslsTwfsozjaSSDBiuBTKjgAh4kLZnuw

=== Unsubscribing ===
✓ Unsubscribed: {'collection': 'ws_subscribe_example_py', 'found': True, 'unsubscribed': True}

=== Cleanup ===
✓ Deleted collection 'ws_subscribe_example_py'

✓ WebSocket subscription example completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: 09XHycUGX8_ITYQH0fmkijqqoXnPs_jtI6ULyUdu5XvHF1uvQSrLc_q0bSFAC8id4PLD75UwwqSAr0Dnf2Tc-Q

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: {'id': 'SzVyw2VOoq18eU2wQBBXOlNN8JbFY3EPnUsEw7oi0mznOWDr7MTNl2K9k9R2F6IvqIqHfTc3a3n6DuSOB268XQ'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'z4fZDtrI9ZHsdgRuWjXeuv4zW5PeJDVLnlyc1VVV_z6bbcQBiEThc6DWF9PpYCYN3hHBbUqKa3IHBCsDDqSTFA'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'name': {'value': 'Product 1', 'type': 'String'}, 'price': {'value': 150, 'type': 'Integer'}, 'id': 'SzVyw2VOoq18eU2wQBBXOlNN8JbFY3EPnUsEw7oi0mznOWDr7MTNl2K9k9R2F6IvqIqHfTc3a3n6DuSOB268XQ'}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'price': {'type': 'Integer', 'value': 500}, 'id': 'SzVyw2VOoq18eU2wQBBXOlNN8JbFY3EPnUsEw7oi0mznOWDr7MTNl2K9k9R2F6IvqIqHfTc3a3n6DuSOB268XQ', 'name': {'type': 'String', 'value': 'Upsert Product'}}

✅ All bypass_ripple operations completed successfully!
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ['id', 'email', 'name']
  First user: {'type': 'String', 'value': 'Dave Brown'} <{'value': 'dave@example.com', 'type': 'String'}>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ['user_role', 'bio', 'created_at', 'name', 'avatar_url', 'email', 'id', 'status', 'age']

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - {'value': 'Dave Brown', 'type': 'String'} (age {'type': 'Integer', 'value': 45})
    - {'type': 'String', 'value': 'Alice Johnson'} (age {'type': 'Integer', 'value': 30})
    - {'type': 'String', 'value': 'Bob Smith'} (age {'value': 25, 'type': 'Integer'})

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - {'value': 'Carol White', 'type': 'String'}: {'value': 'Manager', 'type': 'String'}

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ['id', 'status', 'avatar_url', 'api_key', 'password', 'email', 'bio', 'name', 'age', 'user_role', 'secret_token', 'created_at']
  Projected query:
    - 3 fields per record
    - Fields: ['name', 'email', 'id']
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!

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
Inserted: map[id:3cDtgJ8YImcqpoNzRSaL1jXzcwjFwdhJ44mVVUN_cW0sLSD1yEP7re6yct1pFy5o42n3aR6nyFcDP-b2gjbrPw]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:3cDtgJ8YImcqpoNzRSaL1jXzcwjFwdhJ44mVVUN_cW0sLSD1yEP7re6yct1pFy5o42n3aR6nyFcDP-b2gjbrPw name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found documents: [map[active:map[type:Boolean value:true] id:3cDtgJ8YImcqpoNzRSaL1jXzcwjFwdhJ44mVVUN_cW0sLSD1yEP7re6yct1pFy5o42n3aR6nyFcDP-b2gjbrPw name:map[type:String value:Test Record] value:map[type:Integer value:42]]]

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:3cDtgJ8YImcqpoNzRSaL1jXzcwjFwdhJ44mVVUN_cW0sLSD1yEP7re6yct1pFy5o42n3aR6nyFcDP-b2gjbrPw name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ simple_crud.go completed successfully

=== Running simple_websocket.go ===
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: afrAOAVaIEvOpvRrw6pyCxkAyU5wKGa-XFEQoI0IY3KuDbP4nWD5GVPFbV_fpfIObaSWUuC5Dkdie_pjXlscpw

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1789566411971623000",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "afrAOAVaIEvOpvRrw6pyCxkAyU5wKGa-XFEQoI0IY3KuDbP4nWD5GVPFbV_fpfIObaSWUuC5Dkdie_pjXlscpw",
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

✅ Function saved: YsG6KMZ-hTWinlpD4a0elRYnIcReSpigV_zuV2hmfPZC8hvHBLvnzR2Be1hPDnBDCxR1EMX1Idhs08JBImGCaA
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: erbMBGMKAYZDW7GRbuzHGDWwwABomK5QzMQzfCrczew8Q4N907mT-QJ8Z-rtcmRWDUcDYm7wD4ZY_tBbjKiP4A
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: -2gey2scpc8ohr_-N1HRpQqpK4_0khQHl4ABhwCXieSItRaPbQvCuQmn3ag3FnYrMnhjG5OALJFa8gKJweayaw
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
Collection created with first record: xdawGWVX7U75PRcn2v5xVvayVu2XzZco5qWZ9h0Z8dOmosJsF5VRhKHOIGWNvgy6DKToDovOf9DUbzcaRO5wEg

=== List Collections ===
Total collections: 13
Sample collections: [chat_agent_configs__ek0_testing agent_function_versions__ek0_testing schedules__ek0_testing functions__ek0_testing chat_configurations__ek0_testing]

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ collection_management.go completed successfully

=== Running transactions.go ===
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: kDqCp7255ht4xT6yl5MyA6UqMOyng1E7bhBGei1VT8BjX0LOC27xGclB-3doPjw_u0z2aLAD5tZo-pkfSNGhSg
Created Bob: $500 - ID: uSzfWL_ISVDKl268p120F8d4zaFEJvTT18jb3DBN_4ZNw4sEhqSbJxU7O7jdUM1G43Y7ZaN9VpNe-2EL55oAiw

=== Example 1: Begin Transaction ===
Transaction ID: 625bf002-a208-45fe-856d-86bc3928ff4f

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
New transaction: 700b95c2-05de-45e1-aebd-bb263d905b8d
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
   ✅ Function saved: WQ7QGzNt8Bi15nqhRwCJoEW7X8ANXQD6pT40arjLmSfxsLJg-yrlC8JMe2qJrGpqapfjtM2VZNQn9t815NAQDA

2️⃣ Calling function (Insert + Verify)...
   ✅ function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: ZjgY4Q8XTpUtNLukT5-8q2f-rUDs-Jx7e0eEkioJpa8Y2nwqTg_5sx4fwpxHbWHRvsIek3fqxIEWtVFyWjymcw
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: 6kH5sL2JPPHor0HtxY8gHRMyUpO_MEco6XPf2N6hdPJL65P3kp7HfjCtnaHph9XcnElre6Mcl0w1Ns5bzqDUQg

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
   ✅ Function saved: 2Tkp_K83vTpVN_vrIVYsgsOqYm8KW9nr520vmRITPHzBoNpqGTvZOLF27kAn8jj0N2FhdTbAPenMtLL9QSLh6A

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
   ✅ Function saved: qaw4Ey-cS-AU6kw0q23uY5e-4K30XbiU6vr3DmnSSa-RYqEA9kx4OQK2Ev7oLekAfX25smbMM3hdKS7U0qurGQ

2️⃣ Calling function (Query + Delete + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: WQ7QGzNt8Bi15nqhRwCJ...
   ✅ Deleted script: 6kH5sL2JPPHor0HtxY8g...
   ✅ Deleted script: 2Tkp_K83vTpVN_vrIVYs...
   ✅ Deleted script: qaw4Ey-cS-AU6kw0q23u...
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
  Output: Document ID = UMiDiPCaKwweKMB_xN2ODrvOVt4upxVr6AggqfAVEBqJZL8tvcuya2-_R9s8gDFJlkIUjaHxMEmsrnbH1ewIoQ
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: FindByID(UMiDiPCaKwweKMB_xN2ODrvOVt4upxVr6AggqfAVEBqJZL8tvcuya2-_R9s8gDFJlkIUjaHxMEmsrnbH1ewIoQ)
  Output: Found document with name = map[type:String value:TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: FindByID(UMiDiPCaKwweKMB_xN2ODrvOVt4upxVr6AggqfAVEBqJZL8tvcuya2-_R9s8gDFJlkIUjaHxMEmsrnbH1ewIoQ)
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
  Output: Document ID = wsm9w_8rgFq8WiUiSHGkTw6xRi0Z1iYNAcNZ04pH9XuDGgiGwqxCzEgujKiLX0PhPiaajPOJ04NBOgHVX8kmJw
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: FindByID(wsm9w_8rgFq8WiUiSHGkTw6xRi0Z1iYNAcNZ04pH9XuDGgiGwqxCzEgujKiLX0PhPiaajPOJ04NBOgHVX8kmJw)
  Output: Found document with name = map[type:String value:WS TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: FindByID(wsm9w_8rgFq8WiUiSHGkTw6xRi0Z1iYNAcNZ04pH9XuDGgiGwqxCzEgujKiLX0PhPiaajPOJ04NBOgHVX8kmJw)
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
bash: warning: setlocale: LC_ALL: cannot change locale (C.UTF-8): No such file or directory
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Advanced CRUD Integration Example (Go) ===

--- Inserting base record ---
Inserted record: xOwqTqSHkEiyLyyIpX1Gf9F_KuIphdvSOKdYE72YMsGTL9GutA2aIakCbIJjFvy8yUQSRHR5IsrJfERuyiUoOg

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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: Orz6u4lxCXBs0kRKIYjIGJHWiFRlecFy95GcnXCJI2cefsz6gEkeAksb1VAD_8oecMjrQKgrBvl36lddHQNDhQ

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you have any further questions or need more information, feel free to ask!

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
✓ Created second session: SKCgd5SHIA8LS45JEViLXuwdzsK4Akm6duSgSNKHgab7ipLee0ai-rTBphn6adJA4qaEnG6l1n5u6Ft9VBTuOw
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: SKCgd5SHIA8LS45JEViLXuwdzsK4Akm6duSgSNKHgab7ipLee0ai-rTBphn6adJA4qaEnG6l1n5u6Ft9VBTuOw
✓ Deleted chat session: Orz6u4lxCXBs0kRKIYjIGJHWiFRlecFy95GcnXCJI2cefsz6gEkeAksb1VAD_8oecMjrQKgrBvl36lddHQNDhQ
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: Ck1wc4wBqz7Nbp2CFll00gDXl6yusrAxyv7Kmb94ZCJM4MhTdIMbg0KPkvokmT-OXfKZAWZ1_H1HLZMrINtnyg

=== Sending Chat Message ===
Message ID: ez99c2APV1vR78cSJUK4_y_miaKAmo8oSJAdAbOmLvdoySiFRVo1KoYSC1Tij3IQpyRb9OgwC_uDQhSHFpHIQA

=== AI Response ===
The available products and their prices are as follows:

1. **ekoDB Cloud**: $499
   Description: Fully managed cloud database service product

2. **ekoDB**: $99
   Description: A high-performance database product with AI capabilities

3. **ekoDB Pro**: $299
   Description: Enterprise edition product with advanced features

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:pI8UKPTEZI2lkVPUmV7ADppA8_Ewb6HzPUQf0eCVpNj4pAKCcAMjYXnbB-cLA5Di8Z19PSPsXTDDN6BzzyZMEQ name:ekoDB Cloud price:499] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:AhdcahG8y964lJ33ZrYxyxBiNqmZ_z1Zv8_WmzZugcYPBLA8PErb4qKLQxDJcB9pff67ZaVcbwstu6mR8RnsnA name:ekoDB price:99] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:DoDFzC9Mq6d9T0-eic1xQstpyNiG-6lmUltCAIfERR5Mww_Bhgb2NGT-gDO8CuQe5JT-ak426Iznglc5UhvimQ name:ekoDB Pro price:299] score:0.1111111111111111]

Execution Time: 2347ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 82
Total tokens: 3495

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Chat Message Stream (SSE) Example (Go) ===

Created session: UkjY6eQn4Q9zX_3U2qQul6gXGRfMncWmzBJIMQP5JDmAFBbXQE3fhMt3KQ3IaWQixobjLndxGCJVeOsZtqOwGA

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}EkoDB is a high-performance, distributed database designed for applications that require low-latency and high throughput. It is particularly suited for use cases such as time-series data, event streaming, and large-scale real-time analytics.

Key features of ekoDB may include:

1. **Scalability**: EkoDB is designed to scale horizontally, allowing it to handle increasing amounts of data and requests by adding more servers to the cluster.

2. **Performance**: The database engine is optimized for fast read and write operations, making it ideal for high-load environments.

3. **Distributed Architecture**: EkoDB typically employs a distributed architecture, ensuring data redundancy and fault tolerance, which enhances reliability.

4. **Support for Various Data Models**: EkoDB might support different data models such as key-value, document, or column-family stores, catering to a range of application needs.

5. **Efficient Query Mechanisms**: It may offer indexing and querying features that improve the efficiency of data retrieval.

6. **Real-time Analytics**: Suitable for applications requiring real-time data processing and insights.

While EkoDB may not be as widely known as some other databases, it serves specific niches and use cases where performance and scalability are critical. If you have specific questions about EkoDB or need information on how to implement or use it, feel free to ask!

--- Stream complete ---
Message ID: dXm6qVlohpVc3kmwuen-Sgvj5FTxgcOaSUSsrpWZlFp9gPf8WmE_kBdaNMk2r-0dwI8lyJzdNYqSvckHIIY1jg
Execution time: 2369ms
Context window: 128000 tokens

✓ Chat message stream example completed
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

=== Get All Chat Models ===
OpenAI models: [text-embedding-ada-002 whisper-1 gpt-3.5-turbo tts-1 gpt-3.5-turbo-16k gpt-4-0613 gpt-4 davinci-002 babbage-002 gpt-3.5-turbo-instruct gpt-3.5-turbo-instruct-0914 gpt-3.5-turbo-1106 tts-1-hd tts-1-1106 tts-1-hd-1106 text-embedding-3-small text-embedding-3-large gpt-3.5-turbo-0125 gpt-4-turbo gpt-4-turbo-2024-04-09 gpt-4o gpt-4o-2024-05-13 gpt-4o-mini-2024-07-18 gpt-4o-mini gpt-4o-2024-08-06 omni-moderation-latest omni-moderation-2024-09-26 o1-2024-12-17 o1 o3-mini o3-mini-2025-01-31 gpt-4o-2024-11-20 gpt-4o-mini-search-preview-2025-03-11 gpt-4o-mini-search-preview gpt-4o-transcribe gpt-4o-mini-transcribe o1-pro-2025-03-19 o1-pro gpt-4o-mini-tts o3-2025-04-16 o4-mini-2025-04-16 o3 o4-mini gpt-4.1-2025-04-14 gpt-4.1 gpt-4.1-mini-2025-04-14 gpt-4.1-mini gpt-4.1-nano-2025-04-14 gpt-4.1-nano gpt-image-1 o4-mini-deep-research gpt-4o-transcribe-diarize o4-mini-deep-research-2025-06-26 gpt-5-chat-latest gpt-5-2025-08-07 gpt-5 gpt-5-mini-2025-08-07 gpt-5-mini gpt-5-nano-2025-08-07 gpt-5-nano gpt-audio-2025-08-28 gpt-realtime gpt-realtime-2025-08-28 gpt-audio gpt-5-codex gpt-image-1-mini gpt-5-pro-2025-10-06 gpt-5-pro gpt-audio-mini gpt-audio-mini-2025-10-06 gpt-5-search-api gpt-realtime-mini sora-2 sora-2-pro gpt-5-search-api-2025-10-14 gpt-5.1-chat-latest gpt-5.1-2025-11-13 gpt-5.1 gpt-5.1-codex gpt-5.1-codex-mini gpt-5.1-codex-max gpt-image-1.5 gpt-5.2-2025-12-11 gpt-5.2 gpt-5.2-pro-2025-12-11 gpt-5.2-pro gpt-5.2-chat-latest gpt-4o-mini-transcribe-2025-12-15 gpt-4o-mini-transcribe-2025-03-20 gpt-4o-mini-tts-2025-03-20 gpt-4o-mini-tts-2025-12-15 gpt-realtime-mini-2025-12-15 gpt-audio-mini-2025-12-15 chatgpt-image-latest gpt-5.2-codex gpt-5.3-codex gpt-realtime-1.5 gpt-audio-1.5 gpt-4o-search-preview gpt-4o-search-preview-2025-03-11 gpt-5.3-chat-latest gpt-5.4-2026-03-05 gpt-5.4-pro gpt-5.4-pro-2026-03-05 gpt-5.4 gpt-5.4-nano-2026-03-17 gpt-5.4-nano gpt-5.4-mini-2026-03-17 gpt-5.4-mini gpt-image-2 gpt-image-2-2026-04-21 gpt-5.5 gpt-5.5-2026-04-23 gpt-5.5-pro gpt-5.5-pro-2026-04-23 chat-latest gpt-realtime-translate gpt-realtime-2 gpt-realtime-whisper gpt-5.6-sol gpt-5.6-terra gpt-5.6-luna gpt-realtime-2.1 gpt-realtime-2.1-mini gpt-transcribe gpt-live-transcribe gpt-6-astra gpt-image-2.5-flare gpt-image-2.5-sunburst gpt-image-2.5-flare-2026-09-08 gpt-image-2.5-sunburst-2026-09-08 gpt-live-1]
Anthropic models: [claude-fable-5-1 claude-opus-5 claude-sonnet-5 claude-fable-5 claude-opus-4-8 claude-opus-4-7 claude-sonnet-4-6 claude-opus-4-6 claude-opus-4-5-20251101 claude-haiku-4-5-20251001 claude-sonnet-4-5-20250929]
Perplexity models: []
Gemini models: []
  anthropic: ok 11 models
  gemini: not_configured (unverified) No Gemini API Key
  openai: ok 132 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get OpenAI Models ===
OpenAI models: [text-embedding-ada-002 whisper-1 gpt-3.5-turbo tts-1 gpt-3.5-turbo-16k gpt-4-0613 gpt-4 davinci-002 babbage-002 gpt-3.5-turbo-instruct gpt-3.5-turbo-instruct-0914 gpt-3.5-turbo-1106 tts-1-hd tts-1-1106 tts-1-hd-1106 text-embedding-3-small text-embedding-3-large gpt-3.5-turbo-0125 gpt-4-turbo gpt-4-turbo-2024-04-09 gpt-4o gpt-4o-2024-05-13 gpt-4o-mini-2024-07-18 gpt-4o-mini gpt-4o-2024-08-06 omni-moderation-latest omni-moderation-2024-09-26 o1-2024-12-17 o1 o3-mini o3-mini-2025-01-31 gpt-4o-2024-11-20 gpt-4o-mini-search-preview-2025-03-11 gpt-4o-mini-search-preview gpt-4o-transcribe gpt-4o-mini-transcribe o1-pro-2025-03-19 o1-pro gpt-4o-mini-tts o3-2025-04-16 o4-mini-2025-04-16 o3 o4-mini gpt-4.1-2025-04-14 gpt-4.1 gpt-4.1-mini-2025-04-14 gpt-4.1-mini gpt-4.1-nano-2025-04-14 gpt-4.1-nano gpt-image-1 o4-mini-deep-research gpt-4o-transcribe-diarize o4-mini-deep-research-2025-06-26 gpt-5-chat-latest gpt-5-2025-08-07 gpt-5 gpt-5-mini-2025-08-07 gpt-5-mini gpt-5-nano-2025-08-07 gpt-5-nano gpt-audio-2025-08-28 gpt-realtime gpt-realtime-2025-08-28 gpt-audio gpt-5-codex gpt-image-1-mini gpt-5-pro-2025-10-06 gpt-5-pro gpt-audio-mini gpt-audio-mini-2025-10-06 gpt-5-search-api gpt-realtime-mini sora-2 sora-2-pro gpt-5-search-api-2025-10-14 gpt-5.1-chat-latest gpt-5.1-2025-11-13 gpt-5.1 gpt-5.1-codex gpt-5.1-codex-mini gpt-5.1-codex-max gpt-image-1.5 gpt-5.2-2025-12-11 gpt-5.2 gpt-5.2-pro-2025-12-11 gpt-5.2-pro gpt-5.2-chat-latest gpt-4o-mini-transcribe-2025-12-15 gpt-4o-mini-transcribe-2025-03-20 gpt-4o-mini-tts-2025-03-20 gpt-4o-mini-tts-2025-12-15 gpt-realtime-mini-2025-12-15 gpt-audio-mini-2025-12-15 chatgpt-image-latest gpt-5.2-codex gpt-5.3-codex gpt-realtime-1.5 gpt-audio-1.5 gpt-4o-search-preview gpt-4o-search-preview-2025-03-11 gpt-5.3-chat-latest gpt-5.4-2026-03-05 gpt-5.4-pro gpt-5.4-pro-2026-03-05 gpt-5.4 gpt-5.4-nano-2026-03-17 gpt-5.4-nano gpt-5.4-mini-2026-03-17 gpt-5.4-mini gpt-image-2 gpt-image-2-2026-04-21 gpt-5.5 gpt-5.5-2026-04-23 gpt-5.5-pro gpt-5.5-pro-2026-04-23 chat-latest gpt-realtime-translate gpt-realtime-2 gpt-realtime-whisper gpt-5.6-sol gpt-5.6-terra gpt-5.6-luna gpt-realtime-2.1 gpt-realtime-2.1-mini gpt-transcribe gpt-live-transcribe gpt-6-astra gpt-image-2.5-flare gpt-image-2.5-sunburst gpt-image-2.5-flare-2026-09-08 gpt-image-2.5-sunburst-2026-09-08 gpt-live-1]

=== Get Anthropic Models ===
Anthropic models: [claude-fable-5-1 claude-opus-5 claude-sonnet-5 claude-fable-5 claude-opus-4-8 claude-opus-4-7 claude-sonnet-4-6 claude-opus-4-6 claude-opus-4-5-20251101 claude-haiku-4-5-20251001 claude-sonnet-4-5-20250929]

✓ Chat Models API example complete
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: v1GI3TjyeOkvS8w70JCCznf05Lidxen9dmR7upuJjJ3GXVOCtDMhEIYqlT1yElBbx5QGzszCIr9meVX3px5GFA

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** A high-performance database product
- **Price:** $99

✓ Message 2 sent
  Response: The price of the product ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 8oHfwZxZ7sR1qhyCo9TUVqnbw1ivkFPnqJZxNGa1f6YzuRnXX26t1z3Im9S1_iD64YA3xiaXX04_L4-QW3qYyw
  Parent: v1GI3TjyeOkvS8w70JCCznf05Lidxen9dmR7upuJjJ3GXVOCtDMhEIYqlT1yElBbx5QGzszCIr9meVX3px5GFA

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: 8oHfwZxZ7sR1qhyCo9TUVqnbw1ivkFPnqJZxNGa1f6YzuRnXX26t1z3Im9S1_iD64YA3xiaXX04_L4-QW3qYyw (Untitled)
  Session 2: v1GI3TjyeOkvS8w70JCCznf05Lidxen9dmR7upuJjJ3GXVOCtDMhEIYqlT1yElBbx5QGzszCIr9meVX3px5GFA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 8oHfwZxZ7sR1qhyCo9TUVqnbw1ivkFPnqJZxNGa1f6YzuRnXX26t1z3Im9S1_iD64YA3xiaXX04_L4-QW3qYyw

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: WmhrsLRQB0JDP_TIkWVD6HoXZS1nvGsGc2DjQ67CN9kyJ3x8RW8EPET6rRc2UZj1eIP9wJeMBawCdCsIo2heCQ

=== List Collections ===
Total collections: 13
Sample collections: [chat_agent_configs__ek0_testing agent_function_versions__ek0_testing schedules__ek0_testing functions__ek0_testing chat_configurations__ek0_testing]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Convenience Methods Example ===

=== Native Map Creation ===
✓ Created record with native map: map[id:da9PiNm4qCV6BU450Jo2pltC6sPijFDvlxN-V-AG1W7XwMMdqbjB8jdQVsjR02yAPhnJcOflwGfeh4Jn5GKdkw]

=== Upsert Operation ===
✓ First upsert (update): map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:da9PiNm4qCV6BU450Jo2pltC6sPijFDvlxN-V-AG1W7XwMMdqbjB8jdQVsjR02yAPhnJcOflwGfeh4Jn5GKdkw name:map[type:String value:Alice Johnson]]
✓ Second upsert (insert): map[id:i6Kad4urTBRiuvV6jH_6-1ZLgyRUyikfv_O-wRSOg3LX72_yW7y4dvazLUFe_3s_NhDuKJ6S7fCYAFbiFChJfA]

=== Find One Operation ===
✓ Found user by email: map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:da9PiNm4qCV6BU450Jo2pltC6sPijFDvlxN-V-AG1W7XwMMdqbjB8jdQVsjR02yAPhnJcOflwGfeh4Jn5GKdkw name:map[type:String value:Alice Johnson]]
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: bsGAJ83cwsNFbEK0raV8qTr59Yvrx4tPnCPM7jDEbwd-AG2pjJP8F7VZyHeNOztNOzpTQJStOiqkE3oiqDhZZQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: dh3_9iUW8irkPt2XEz2AR9893Vf8t5GYXRIEqd7zKI06BILhLCQxWi2b9KLsUbj1lueqXUxWO-z0KEWUBN437w

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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB as Edge Cache - Simple Example ===

Setting up edge cache collection...
✓ Cache entry created

Creating edge cache lookup script...
✓ Edge cache script created: VNlV3w48vk2MXVpaTNce--C9NLXnZ4eN64_Vpkh-pOj9RtgIRkKnV8ZzW_09CJbq7WHSHgA0h2_6rs2L-1bwAA

Call 1: Cache lookup
Response time: 2ms
Found 1 cached entries

Call 2: Cache lookup (connection warm)
Response time: 4ms
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
   ⏱️  Duration: 94.590709ms
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
   ⏱️  Duration: 4.78075ms
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
   🚀 Cache speedup: 23.5x faster!

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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
client_function_contract: ok
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
🚀 ekoDB Functions Example (Go Client)

✅ Client initialized

🧹 Cleaning up...
✅ Deleted collection
✅ Deleted test scripts

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: LvxDU4er-O37PM-tnPPtYkhWpGC70j09XUVeK5NuqjzxLBxEwKGWuBQt4Db-JkT8CHQ5fxvuE9BlClpLUxrkDw
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
🚀 ekoDB Go Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: Tl9QminPFEn8gTBUxcPPNWxnJRENWuemGwd4hCdpxzX3-6kpNuvEq-yEasf30sb90_8ot_xuxrl-iFyX3iEe6A
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
🚀 ekoDB Go KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: IMn-iJ5WNMOtRnnKJO12_iNpTXbJN6k5EyrcSFlvgroJvOm1EXiDt1bhJNYTrzXnid5LOhZIIB5u302htDj-Ig
✅ Inserted 2 products with wrapped types

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: Awew1BkW0kSPEz8gSA2pJW8HpAXhNUv6WwtQ21mM7GLIMgX06nDCOztL6qTyOyYDa-h5GgNFfdoy_TeSFC8Vgw
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: map[type:Object value:map[role:admin userId:user_abc]]
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session
📝 Example 4: KV Operations in Functions

✅ Function saved: Cr1sE_Lmw6_bwzFlNKKaiVxqzTJAub9Y68VkWvfeVwjXSsP7NqFvoDiRCTyF_7df1Z1WFR8pD22qk8u76ZIPWA
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: ECcbcqW3FBKzBVJ2GDUvhDn00XiotiQh29IikRqxasen4LrRJqP2IQfNVf-gwj_DSWuhQ2k9PNlcjKd6zaE22w
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

perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
🚀 ekoDB Go Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. map[type:String value:Introduction to Machine Learning] (map[type:String value:AI])
   2. map[type:String value:Getting Started with ekoDB] (map[type:String value:Database])
   3. map[type:String value:Natural Language Processing] (map[type:String value:AI])
   4. map[type:String value:Vector Databases Explained] (map[type:String value:Database])
   5. map[type:String value:Database Design Principles] (map[type:String value:Database])
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Goal Template CRUD Example (Go) ===

--- Creating goal template ---
Created template: Data Migration (id: O-cvT9FX3IFdZW3GSCwdF8H61LnF4CbQFXocgRIpghNqH3CJuBIYYiT7G81XQ68rf8tXDPcoI9NjnbCNsP7oZA)

--- Listing templates ---
Templates: map[count:1 items:[map[description:map[type:String value:Template for migrating data between schemas] id:O-cvT9FX3IFdZW3GSCwdF8H61LnF4CbQFXocgRIpghNqH3CJuBIYYiT7G81XQ68rf8tXDPcoI9NjnbCNsP7oZA steps:map[type:Array value:[map[description:Analyze source schema] map[description:Create target schema] map[description:Migrate records] map[description:Validate results]]] title:map[type:String value:Data Migration]]]]

--- Getting template ---
Fetched: Data Migration

--- Updating template ---
Updated description: Updated: comprehensive data migration workflow

--- Deleting template ---
Template deleted successfully

✓ Goal template CRUD example completed
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Goals, Tasks & Agents Integration Example (Go) ===

--- Creating goal ---
Created goal: Deploy v2.0 (id: 37fO5DgzYGGoGG434f6_ux4MjyedtMdwZ2Yi8wxDMtKKjGe5mhkg70hJ13q5NqLFNNlHUL-T9jbitwPHn55W2g)

--- Listing goals ---
Goals response: map[count:1 goals:[map[created_at:2026-09-16T13:48:05.712236+00:00 description:Ship the v2.0 release to production id:37fO5DgzYGGoGG434f6_ux4MjyedtMdwZ2Yi8wxDMtKKjGe5mhkg70hJ13q5NqLFNNlHUL-T9jbitwPHn55W2g status:pending steps:[{"description":"Run integration tests"},{"description":"Build release artifacts"},{"description":"Deploy to staging"}] title:Deploy v2.0 updated_at:2026-09-16T13:48:05.712236+00:00]]]

--- Getting goal ---
Fetched goal: Deploy v2.0

--- Updating goal ---
Updated description: Ship the v2.0 release to production (with hotfix)

--- Searching goals ---
Search result: map[count:1 items:[map[_score:12.870000000000001 created_at:map[type:DateTime value:2026-09-16T13:48:05.712236+00:00] description:map[type:String value:Ship the v2.0 release to production (with hotfix)] id:37fO5DgzYGGoGG434f6_ux4MjyedtMdwZ2Yi8wxDMtKKjGe5mhkg70hJ13q5NqLFNNlHUL-T9jbitwPHn55W2g status:map[type:String value:pending] steps:map[type:String value:[{"description":"Run integration tests"},{"description":"Build release artifacts"},{"description":"Deploy to staging"}]] title:map[type:String value:Deploy v2.0] updated_at:map[type:DateTime value:2026-09-16T13:48:05.730398+00:00]]]]

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
Created task: Run benchmarks (id: V8H3q9l2f4LpzQdSsAMEyXDI8KqMMPXYW6C5XXZVs8GTsKNfzl-GSwr0X_JGeCYGTUp53owlr52By0IuU8iVBw)

--- Listing tasks ---
Tasks: map[count:1 items:[map[description:map[type:String value:Execute YCSB benchmarks against staging] due_at:map[type:DateTime value:2026-09-17T13:48:05+00:00] id:V8H3q9l2f4LpzQdSsAMEyXDI8KqMMPXYW6C5XXZVs8GTsKNfzl-GSwr0X_JGeCYGTUp53owlr52By0IuU8iVBw name:map[type:String value:Run benchmarks]]]]

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
Created agent: benchmark-runner (id: Tzn_HTMPQ8o3A3P7ITfgC3qefh3qD46AgbxGSIAatWdsm9BH3Z0Chcrga5KrTwqOKm5PEBAQFW2zFyYpNXyKbA)

--- Listing agents ---
Agents: map[count:1 items:[map[deployment_id:map[type:String value:deploy_prod_1] description:map[type:String value:Runs periodic benchmarks] id:Tzn_HTMPQ8o3A3P7ITfgC3qefh3qD46AgbxGSIAatWdsm9BH3Z0Chcrga5KrTwqOKm5PEBAQFW2zFyYpNXyKbA llm_model:map[type:String value:gpt-4.1] name:map[type:String value:benchmark-runner]]]]

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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB KV Links Integration Example (Go) ===

--- Inserting documents ---
Inserted doc1: UzoRgHjEkSjYEH-AI3v7jCBTsJj4jKpP5HJxEAnWmYPHk19uO5K9dTlp9LCKyPnJbLLPA6CU7fR4s4mXwr6nGg
Inserted doc2: HW0V0XdbI_Do72FPsLifFpLDl_aq1wUdLvHF0ed8-_pzq9uiU7V-auzA7GEH7RzTXm3Tk3ULfRAhUnUjItLkcQ

--- Setting KV entry ---
KV entry set: team:engineering:go

--- Linking KV key to documents ---
Linked doc1: map[]
Linked doc2: map[]

--- Getting links ---
Links for team:engineering:go: [map[collection:kv_links_example_go created_at:2026-09-16T13:48:07.859124Z document_id:HW0V0XdbI_Do72FPsLifFpLDl_aq1wUdLvHF0ed8-_pzq9uiU7V-auzA7GEH7RzTXm3Tk3ULfRAhUnUjItLkcQ field_path:<nil> last_accessed:2026-09-16T13:48:07.860351Z metadata:map[]] map[collection:kv_links_example_go created_at:2026-09-16T13:48:07.857642Z document_id:UzoRgHjEkSjYEH-AI3v7jCBTsJj4jKpP5HJxEAnWmYPHk19uO5K9dTlp9LCKyPnJbLLPA6CU7fR4s4mXwr6nGg field_path:<nil> last_accessed:2026-09-16T13:48:07.860351Z metadata:map[]]]

--- Unlinking doc2 ---
Unlink result: map[]

--- Verifying links after unlink ---
Links after unlink: [map[collection:kv_links_example_go created_at:2026-09-16T13:48:07.857642Z document_id:UzoRgHjEkSjYEH-AI3v7jCBTsJj4jKpP5HJxEAnWmYPHk19uO5K9dTlp9LCKyPnJbLLPA6CU7fR4s4mXwr6nGg field_path:<nil> last_accessed:2026-09-16T13:48:07.863334Z metadata:map[]]]

--- Cleanup ---
Cleaned up all resources

=== All KV link operations completed successfully ===
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Schedules Integration Example (Go) ===

--- Creating schedule ---
Created schedule: nightly-backup (id: e9fb0210-7e2d-4f82-85bc-b65ea61d94ac)

--- Listing schedules ---
Schedules: map[count:1 schedules:[map[created_at:2026-09-16T13:48:14.297211Z cron_expression:0 0 0 * * * description:Runs a database backup every night at midnight enabled:true function_label:schedule_noop_go_17740 id:e9fb0210-7e2d-4f82-85bc-b65ea61d94ac last_execution:<nil> name:nightly-backup next_execution:2026-09-17T00:00:00Z parameters:map[] stats:map[avg_execution_time_ms:0 failed_executions:0 last_error:<nil> successful_executions:0 total_executions:0] timezone:UTC updated_at:2026-09-16T13:48:14.297211Z]]]

--- Getting schedule ---
Fetched schedule: nightly-backup (cron: 0 0 0 * * *)

--- Updating schedule ---
Updated cron: 0 0 2 * * *

--- Triggering schedule ---
Trigger response: map[schedule_id:e9fb0210-7e2d-4f82-85bc-b65ea61d94ac status:triggered]

--- Pausing schedule ---
Schedule enabled: false

--- Resuming schedule ---
Schedule enabled: true

--- Deleting schedule ---
Schedule deleted

=== All schedule operations completed successfully ===
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
  1. Score: 0.763
  2. Score: 0.754
  3. Score: 0.725

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.490
  2. Score: 0.901
  3. Score: 0.305

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200

9. Vector search with a metadata pre-filter (category = ml):
Found 2 documents in category "ml" (NLP excluded)
  1. Deep Learning Fundamentals (category: ml)
  2. Introduction to Machine Learning (category: ml)


✅ Search examples completed!
=== Cleanup ===
✅ Deleted test collections
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: map[id:3Cnc85FwZxs4bHBvdTJfrphlFFZRS8WrgVqyepp9ojwZktlSruuQ_8QhtYp2l3vUrq8GaedZUVUmrx2-Hk2rPA]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-09-16T13:48:16+00:00] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:3Cnc85FwZxs4bHBvdTJfrphlFFZRS8WrgVqyepp9ojwZktlSruuQ_8QhtYp2l3vUrq8GaedZUVUmrx2-Hk2rPA metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Test Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:42]]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-09-16 13:48:16 +0000 +0000
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-09-16T13:48:16+00:00 data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:3Cnc85FwZxs4bHBvdTJfrphlFFZRS8WrgVqyepp9ojwZktlSruuQ_8QhtYp2l3vUrq8GaedZUVUmrx2-Hk2rPA metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-09-16T13:48:16+00:00] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:3Cnc85FwZxs4bHBvdTJfrphlFFZRS8WrgVqyepp9ojwZktlSruuQ_8QhtYp2l3vUrq8GaedZUVUmrx2-Hk2rPA metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: 8sFz8JMhr2jjeXGkVLiFRH7uCgPwO-NxyH3dUJ08H1H1yDuRfpLLev4d2GrcVGyZ8oXvOIfY5T0KNyn9lwSojg

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
✓ Created native SWR script: github_user_native_go (ZGSeOq7CasfCxG--WXoIiciwkFE6Nl_h2Z5HMCrg3THqQB-OnzXvhsZya3D-cSvOxbdLQzk4cH4yGqa2ygXkrA)

First call (cache miss - will fetch from GitHub API):
  Response time: 152ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 6ms
  Speedup: 25.3x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_go (ZDujMSaPljFsHYD85eMCypuZQClT78VC8Kk1dG5xb9Siyp4vBUCm_SAuZ-CS37oiR7DGWnGefHJ-WwOlqU7NcA)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_go' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_go (e33qmG9teCuzqQu96ooISHxc9BCDvucFbqA__IfddCT9ij5eevy1UyQ_kM_qWd6cLZjFwQXxj1DicC49sNYX-A)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_go' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_go (6mNRgEGoBDullLfbQNfnAt8FMsqh7Fyxrv99Lb60528nMAosj-OK7tL6p1DDBjVDJ_CcE5x0h3XKpFbLj2UJug)
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_go (4V02qL5UJCRSPg8Qy2UwmUTWPusKjEGPlZIKlvIgKeD9yORLkHMGWc9yBHmePIo6cjY72xkFjVEJROK_gk4uqw)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "cached_at": {
        "type": "DateTime",
        "value": "2026-09-16T13:48:18+00:00"
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
Response time: 11ms (served from cache)
✓ Lightning fast cache hit

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
🧹 Cleaning up...
✓ Cleanup complete

perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 6mggmyV-oZ-BVccvy6WU3S30W2JHrc4he5ZV-Evjnc2yY4k-Bm4q1WwucmGbO9BNZHX5odCTU3RqOmWZ2wByBw
Created Bob: $500 - ID: nlucW5_NZ5J09HtTVjveeiSkKnassnl9Jlz52M078b1ceGCbV6Dd5eBvIwqwlP95e_8G7rI7p2W1CVD781qfOw

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): e1b12090-6804-4e5b-8374-f41d68e6c6f6

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
New transaction: 37eb9f64-99d2-4ae1-af22-969f35fd10b3
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

=== Create User Function ===
Created user function with ID: c_34Wf2QFxDhdj7Z4iQ722YTn9Rvj1unU_HT-aP1eNsT2P44uU3OCW0IDJsOkGlZn5TAJvYtdzIZ_G6d0-FH3A

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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== WebSocket Chat Streaming Example (Go) ===

Created chat session: lvcuTU5UBxu38FIoCpOmdVRDzP403ZdoDPeZ9laHWCBLPVLm5Wl_saEIcVAjlmojXLo_DvIPZ2_CWdNFwePgPg

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: wX_-3uEe4PfROazYY4PrYTUDo0vjtiCeJI2jtw7Ik99tI9BskmWMXDYws5MlDV0j3i2FHCI604WHjx-g19t1IQ
Execution time: 903ms
Token usage: {"completion_tokens":8,"prompt_tokens":15,"total_tokens":23}

Full response: {"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}The capital of France is Paris....
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== WebSocket Subscription Example (Go) ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_go' ===
✓ Subscribed (subscription_id: sub_b2472f68d7c0421cb874561766c088e2)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: 4XZq4jfDFJzhnR7y1bWXihSUqWARu_dYI6CHrvhK8yTSSPd8USCgJKTbmTiYVEou_d7juEGIthO7g-Iv_A8qbA

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_go
     Record IDs: [4XZq4jfDFJzhnR7y1bWXihSUqWARu_dYI6CHrvhK8yTSSPd8USCgJKTbmTiYVEou_d7juEGIthO7g-Iv_A8qbA]
     Timestamp:  2026-09-16T13:48:21.959103+00:00

Inserting record 2...
✓ Inserted: _SmKrZVPjDoULNEBvafzR0avAVJXfco0sVp2O7HPoXa73XTJqN6q6G_HLWsbicTV0iffvY6JZ-ZNG5bnlUPJvQ

  📡 Notification received:
     Event:      insert
     Record IDs: [_SmKrZVPjDoULNEBvafzR0avAVJXfco0sVp2O7HPoXa73XTJqN6q6G_HLWsbicTV0iffvY6JZ-ZNG5bnlUPJvQ]

=== Unsubscribing ===
✓ Unsubscribed

✓ WebSocket subscription example completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Rif2Nxfnh5ffyKF294tYL6IIbYqR8MV7NFQjK45XU_vb65QlBGlJYgaHzYQCNQS-hDBpcEVmDJV8y0gxZmJ8Eg

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
Inserted with ripple: map[id:hWncVRBtB2er72vYulF0q1E1gQdxzvLZ89_iGE_lV_Nq45Q_-SBne78KuoeWwP-yvcCkjOEBAiegpyI85FFC3A]
Inserted with bypass_ripple: map[id:M9u4w1dfFKlouqlC2WnR9Nh1FEFTsZ6H-WKV0gxX2YqlcvJbdzzynRaJ-cR6gwGwJtfxI3j5kWFSKplJXy-lXg]
Inserted with TTL and bypass_ripple: map[id:CEz6TDnfxA1z6S6rjFPwquA3nP3a2vk8kwAG2r8hIpB9CdWYU-jxjxGUEhKf1i7_6u9tt6GzfI8TpcQOQ6j7xg]
Updated with bypass_ripple: map[id:hWncVRBtB2er72vYulF0q1E1gQdxzvLZ89_iGE_lV_Nq45Q_-SBne78KuoeWwP-yvcCkjOEBAiegpyI85FFC3A name:map[type:String value:Product 1] price:map[type:Integer value:150]]
Deleted with bypass_ripple
Batch inserted with bypass_ripple: 2 records
Upserted with bypass_ripple: map[id:yjSK26YSF_uDAHtIOfF-ibzm2Zxz0cutJQt3UtQvkoZw-hgOfhGHHx58yKlkLCTDyUkjqBsAxeW4q8m_Og9u0A]
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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

📦 Building TypeScript client library...

> @ekodb/ekodb-client@0.26.4 prepare
> npm run build


> @ekodb/ekodb-client@0.26.4 build
> tsc


up to date, audited 46 packages in 576ms

12 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.26.4 build
> tsc

✅ TypeScript client built!

added 1 package, removed 1 package, and audited 13 packages in 390ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
bash: warning: setlocale: LC_ALL: cannot change locale (C.UTF-8): No such file or directory
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Advanced CRUD Example (TypeScript) ===

--- Inserting base record ---
Inserted: YTKswpeH0xQcnZMsWXaBICQf--0Up8AXP3tvjt6Fw0VJCYF7V5ZFwCbhWITpqVFKjgtFPKHeHvi5iQ_ch-sB2w

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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: gF6YiDd7ia8MzrNsNkVsHqxzxqJekYhUDyBh0VPIeKUY3R4gEHcV4nBxKUQM965t6dThIQa0pzuW8YiTKl4Vpg

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you have any further questions or need more information, feel free to ask!

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
✓ Created second session: T_NfF1iplssUw35I_WE9NWEexhij3Ibw0AlQsWaJ8ZaFnyxBjbRRxcWs2zG9IxvrsdJbbYERP34L1343LkyIYw
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: T_NfF1iplssUw35I_WE9NWEexhij3Ibw0AlQsWaJ8ZaFnyxBjbRRxcWs2zG9IxvrsdJbbYERP34L1343LkyIYw
✓ Deleted chat session: gF6YiDd7ia8MzrNsNkVsHqxzxqJekYhUDyBh0VPIeKUY3R4gEHcV4nBxKUQM965t6dThIQa0pzuW8YiTKl4Vpg
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: L5AA1VCt9I1tJU3s5nWJLq3Y_whSqweKWv3rbvo_1vWOWt7CidfyYXWnhEJcHBOFekoze2r2itAA48KNXLPoCA

=== Sending Chat Message ===
Message ID: PhdxgKf1JE0PaCfqrJl_QSsSX1ksZwQ8JaUkDWD7i7qsXWoz9WDMI5SilwXrwVj71gOlIWtDiMXqFt-nT8U6IQ

=== AI Response ===
Here are the available products along with their prices:

1. **ekoDB**
   - Price: $99
   - Description: A high-performance database product with AI capabilities.

2. **ekoDB Cloud**
   - Price: $499
   - Description: Fully managed cloud database service product.

3. **ekoDB Pro**
   - Price: $299
   - Description: Enterprise edition product with advanced features.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'CtaTJoqvQ3TyVfcIGDiJZ_E4teTAcKJnmmjcPm8rttxqdpNrMuQeGN3R0slYgOGBkwAEF18qFjvquRn1EdfuVg',
    description: 'A high-performance database product with AI capabilities',
    price: 99,
    name: 'ekoDB'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Fully managed cloud database service product',
    id: 'cMcOp6cansv6llaoPH4gm2mVxk3FJmxEXN1MU0IfX5pcw4qg_MT_WN3oZwXJHB8Qweok0ShxywiNj7E_sqUfBg',
    price: 499,
    name: 'ekoDB Cloud'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'gRANOxcM_j0n1kErwj2-y9j8wpwyPqbYe5ZrdiJPDyfHuNfgQVurtjGVPI-HyrIe91t-s52DYTedmFx4f2hV5A',
    description: 'Enterprise edition product with advanced features',
    name: 'ekoDB Pro',
    price: 299
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 6509ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 91
Total tokens: 3504

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Chat Message Stream (SSE) Example (TypeScript) ===

Created session: YsEi5x23cRPYqrrDlR6osum7XCXbYsR2W7Yo6LfbdoAgLePGGzuah0jd_qHNGuLhoK0PTWDcw-ipAHq7fm61ig

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}**ekoDB** (Environmental Knowledge Organization DataBase) is a term that may refer to different resources or databases that catalog information related to ecosystems, environmental knowledge, or eco-organisms, depending on the context. However, as of June 2024, the most widely referenced **ekoDB** is an online database and resource platform specializing in the study of non-coding RNAs (ncRNAs) associated with bacterial and archaeal hosts, especially those involved in defense mechanisms such as CRISPR-Cas systems.

### ekoDB in the Context of Prokaryotic RNA Defense Systems
- **ekoDB** stands for **"endogenous prokaryotic RNA defense sYstem DataBase."**
- It was designed to help researchers explore, analyze, and compare **prokaryotic Argonaute proteins** and other defense-related non-coding RNAs.
- The database provides curated information on sequences, structures, functions, and evolutionary relationships among various non-coding RNAs involved in prokaryotic immune systems.
- Resources include:
  - Genome annotations
  - Experimental data
  - Bioinformatic analysis tools
  - Cross-references to related databases (such as CRISPRdb, anti-CRISPRdb, and others)

### Possible Broader Uses
- In some contexts, "ekoDB" might generally refer to databases cataloging ecological or environmental data, but this is less common and not standardized. If your context is ecology, please clarify for a more tailored answer.

### Key References
- **ekoDB:** [ekoDB Database—RNA Defense System in Prokaryotes](https://ekodb.genomics.cn/) (frequently cited in bioinformatics and microbiome research literature)
- Example publication: **Zhang J, et al. (2021). ekoDB: A comprehensive database of prokaryotic RNA defense systems. Nucleic Acids Research.**

---

**In summary:**
**ekoDB** is primarily known as a bioinformatics database for studying RNA-based defense systems in prokaryotes, especially those involving Argonaute and CRISPR-Cas systems, though the term could sometimes refer to other environmental or knowledge-organization databases depending on the context. If you meant a different field or a specific application, please specify for a more precise explanation!

--- Stream complete ---
Message ID: WEMudlpJWmx3DwX2k6x8O68qQRhYz2WI3leEJDyT3VPH4D8TH_9SAVbxRtGVgLlG51EQjFEZikEoVH1DAfGUNQ
Execution time: 7248ms
Context window: 1000000 tokens

✓ Chat message stream example completed
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
    - gpt-image-2.5-flare
    - gpt-image-2.5-sunburst
    - gpt-image-2.5-flare-2026-09-08
    - gpt-image-2.5-sunburst-2026-09-08
    - gpt-live-1
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
  openai: ok 132 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get Specific Provider Models ===
OpenAI models: text-embedding-ada-002, whisper-1, gpt-3.5-turbo, tts-1, gpt-3.5-turbo-16k, gpt-4-0613, gpt-4, davinci-002, babbage-002, gpt-3.5-turbo-instruct, gpt-3.5-turbo-instruct-0914, gpt-3.5-turbo-1106, tts-1-hd, tts-1-1106, tts-1-hd-1106, text-embedding-3-small, text-embedding-3-large, gpt-3.5-turbo-0125, gpt-4-turbo, gpt-4-turbo-2024-04-09, gpt-4o, gpt-4o-2024-05-13, gpt-4o-mini-2024-07-18, gpt-4o-mini, gpt-4o-2024-08-06, omni-moderation-latest, omni-moderation-2024-09-26, o1-2024-12-17, o1, o3-mini, o3-mini-2025-01-31, gpt-4o-2024-11-20, gpt-4o-mini-search-preview-2025-03-11, gpt-4o-mini-search-preview, gpt-4o-transcribe, gpt-4o-mini-transcribe, o1-pro-2025-03-19, o1-pro, gpt-4o-mini-tts, o3-2025-04-16, o4-mini-2025-04-16, o3, o4-mini, gpt-4.1-2025-04-14, gpt-4.1, gpt-4.1-mini-2025-04-14, gpt-4.1-mini, gpt-4.1-nano-2025-04-14, gpt-4.1-nano, gpt-image-1, o4-mini-deep-research, gpt-4o-transcribe-diarize, o4-mini-deep-research-2025-06-26, gpt-5-chat-latest, gpt-5-2025-08-07, gpt-5, gpt-5-mini-2025-08-07, gpt-5-mini, gpt-5-nano-2025-08-07, gpt-5-nano, gpt-audio-2025-08-28, gpt-realtime, gpt-realtime-2025-08-28, gpt-audio, gpt-5-codex, gpt-image-1-mini, gpt-5-pro-2025-10-06, gpt-5-pro, gpt-audio-mini, gpt-audio-mini-2025-10-06, gpt-5-search-api, gpt-realtime-mini, sora-2, sora-2-pro, gpt-5-search-api-2025-10-14, gpt-5.1-chat-latest, gpt-5.1-2025-11-13, gpt-5.1, gpt-5.1-codex, gpt-5.1-codex-mini, gpt-5.1-codex-max, gpt-image-1.5, gpt-5.2-2025-12-11, gpt-5.2, gpt-5.2-pro-2025-12-11, gpt-5.2-pro, gpt-5.2-chat-latest, gpt-4o-mini-transcribe-2025-12-15, gpt-4o-mini-transcribe-2025-03-20, gpt-4o-mini-tts-2025-03-20, gpt-4o-mini-tts-2025-12-15, gpt-realtime-mini-2025-12-15, gpt-audio-mini-2025-12-15, chatgpt-image-latest, gpt-5.2-codex, gpt-5.3-codex, gpt-realtime-1.5, gpt-audio-1.5, gpt-4o-search-preview, gpt-4o-search-preview-2025-03-11, gpt-5.3-chat-latest, gpt-5.4-2026-03-05, gpt-5.4-pro, gpt-5.4-pro-2026-03-05, gpt-5.4, gpt-5.4-nano-2026-03-17, gpt-5.4-nano, gpt-5.4-mini-2026-03-17, gpt-5.4-mini, gpt-image-2, gpt-image-2-2026-04-21, gpt-5.5, gpt-5.5-2026-04-23, gpt-5.5-pro, gpt-5.5-pro-2026-04-23, chat-latest, gpt-realtime-translate, gpt-realtime-2, gpt-realtime-whisper, gpt-5.6-sol, gpt-5.6-terra, gpt-5.6-luna, gpt-realtime-2.1, gpt-realtime-2.1-mini, gpt-transcribe, gpt-live-transcribe, gpt-6-astra, gpt-image-2.5-flare, gpt-image-2.5-sunburst, gpt-image-2.5-flare-2026-09-08, gpt-image-2.5-sunburst-2026-09-08, gpt-live-1

=== Get Anthropic Models ===
Anthropic models: claude-fable-5-1, claude-opus-5, claude-sonnet-5, claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 404: {"error":"Unknown provider 'nonexistent_provider_xyz'","error_kind":"unknown_provider"}

✓ Chat Models example complete
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: EnuGFcCEp5WX4Zp0-1-NWNuz9wbFdMH1BqUI8LfoBMHcPrX4aOwfU7xQ0UYW22OxxmfD7BFaXWJhp-suER_maA

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the database context, the available product is:

- **Name:** ekoDB
- **Description:** A high-performance database product
- **Price:** $99

If you need more information or specifics about this product, feel free to ask!

✓ Message 2 sent
  Response: The price of the product **ekoDB** is **$99**. If you have any other questions or need further details, let me know!

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 1XFz__JkGzeOfEFIHxh6CUmX4ZkOntrR63XnaHNRsfShUVRAEJQ2uAHzygZpM5nHkeYI2j0jZlLJnDaRopwuhQ
  Parent: EnuGFcCEp5WX4Zp0-1-NWNuz9wbFdMH1BqUI8LfoBMHcPrX4aOwfU7xQ0UYW22OxxmfD7BFaXWJhp-suER_maA

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: 1XFz__JkGzeOfEFIHxh6CUmX4ZkOntrR63XnaHNRsfShUVRAEJQ2uAHzygZpM5nHkeYI2j0jZlLJnDaRopwuhQ (Untitled)
  Session 2: EnuGFcCEp5WX4Zp0-1-NWNuz9wbFdMH1BqUI8LfoBMHcPrX4aOwfU7xQ0UYW22OxxmfD7BFaXWJhp-suER_maA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 1XFz__JkGzeOfEFIHxh6CUmX4ZkOntrR63XnaHNRsfShUVRAEJQ2uAHzygZpM5nHkeYI2j0jZlLJnDaRopwuhQ

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: sNpbVQkDtJe4rFxEZ-CUPyPUkyty4Z3-2mnaq2kOHozzjwbeL2zpD-_ytKc9VZyNm5VciZh6y2joS0rYYjBNfQ

=== List Collections ===
Total collections: 21
Sample collections: chat_agent_configs__ek0_testing,agent_function_versions__ek0_testing,schedules__ek0_testing,schema_employees_client_go,schema_products_client_go

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created
✓ conc_demo_pay saved
✓ conc_demo_rl_fail saved
✓ conc_demo_rl_skip saved
✓ conc_demo_lock saved

Invoke them like:
  POST /api/functions/conc_demo_pay_ts_43664_1789567012579 { "idempotency_key": "...", "amount": 100 }
  POST /api/functions/conc_demo_rl_fail_ts_43664_1789567012579 { "user_id": 42 }
  POST /api/functions/conc_demo_rl_skip_ts_43664_1789567012579 { "user_id": 42 }
  POST /api/functions/conc_demo_lock_ts_43664_1789567012579 { "resource": "queue:drain" }

✓ Cleaned up demo functions
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: '4pUsgk8BBBUWa6m_STPjcuuMEbZpeBkgEC5rDRoFdPwR0D8N08n4Sn6LSf6uHdpPDsQV6b5SssV6gj-Clv-ViA'
}

=== Upsert Operation ===
✓ First upsert (update): {
  id: '4pUsgk8BBBUWa6m_STPjcuuMEbZpeBkgEC5rDRoFdPwR0D8N08n4Sn6LSf6uHdpPDsQV6b5SssV6gj-Clv-ViA',
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  age: { type: 'Integer', value: 29 },
  active: { value: true, type: 'Boolean' },
  name: { type: 'String', value: 'Alice Johnson' }
}
✓ Second upsert (insert): {
  id: 'uLdv1LQyQmeg-P5ACj44wbO7IDpFP9I5Sk12EBA0dMI7r0mATdiS-ZOWsymA6DyXTv62qDVlf5bfacckf94_VA'
}

=== Find One Operation ===
✓ Found user by email: {
  name: { type: 'String', value: 'Alice Johnson' },
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  id: '4pUsgk8BBBUWa6m_STPjcuuMEbZpeBkgEC5rDRoFdPwR0D8N08n4Sn6LSf6uHdpPDsQV6b5SssV6gj-Clv-ViA',
  active: { type: 'Boolean', value: true },
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: Pyd9xz2Eu9iHHD9aoIWr1iqFzuCNheix1LA5lA28bxEMFUamJeJUxOGH0tTzjlsr7GuaH86cGBxhNktOea4BKQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: RqIJtIhnBGLutZw60JXgtjBGYR5QwqBgtNn6LkuhywQMLjW2LWsvbe_u6WqDB9k4kwLJblDXYJREkNcYLUxfdQ

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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB as Edge Cache - Simple Example ===

Creating edge cache function...
✓ Edge cache script created: XocyYnJ25whFdU1a94R4pZnVvlSbw5EKInYHaWUCWDL1ndawB_jjgG8Dwbzw79ZnM3Gd_Y6hNqwenX6XrrfDIQ

Call 1: Cache miss (fetches from API)
Response time: 116ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "address": {
            "street": "Kulas Light",
            "suite": "Apt. 556",
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "zipcode": "92998-3874"
          },
          "email": "Sincere@april.biz",
          "phone": "1-770-736-8031 x56442",
          "username": "Bret",
          "id": 1,
          "name": "Leanne Graham",
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
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

Call 2: Cache hit (served from ekoDB)
Response time: 9ms (12.9x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "address": {
            "street": "Kulas Light",
            "suite": "Apt. 556",
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "zipcode": "92998-3874"
          },
          "email": "Sincere@april.biz",
          "phone": "1-770-736-8031 x56442",
          "username": "Bret",
          "id": 1,
          "name": "Leanne Graham",
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
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

=== The Magic ===
- Your DATABASE is your EDGE
- No Redis needed
- No CDN needed
- No cache invalidation logic needed (TTL handles it)
- With ripples: All nodes auto-sync cache
- One service: Database + Cache + Edge Functions

✓ Example complete!

perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
   ⏱️  Duration: 106ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "phone": "1-770-736-8031 x56442",
      "address": {
        "geo": {
          "lng": "81.1496",
          "lat": "-37.3159"
        },
        "suite": "Apt. 556"...

Second call (cache hit - from cache):
   ⏱️  Duration: 6ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "phone": "1-770-736-8031 x56442",
      "address": {
        "geo": {
          "lng": "81.1496",
          "lat": "-37.3159"
        },
     ...
   🚀 Cache speedup: 17.7x faster!

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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
client_function_contract: ok
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
🚀 ekoDB Functions Example (TypeScript)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: hepoXMT9m0PENfDrq6D2YaurJivbC8dMhClqekabo9-pZDTK1EMkQBCzICLW_jxX1QgbVXyqxIZDJct1wANdlg
📊 Found 5 active users

📝 Example 2: Parameterized Function

✅ Function saved: 1sRyK92uMJgIg5nfHZCKEIKn5-af_5F1dkn8skWvUtZctqqONrXZomYNTfJf3jz6yV8XywMs0E9kTFULSWEfcQ
📊 Found 3 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: fRc0u3FVNOk_gDADTL_ZTF4UWwPGVNHo3oUEjNUxqJ-SgxQHhYm4NyVrcg8JAZH7dsO7eZYUE0GKPdIRIrxOjg
📊 Statistics: 2 groups
   {"count":{"type":"Integer","value":5},"avg_score":{"type":"Float","value":50},"status":{"value":"inactive","type":"String"}}
   {"count":{"value":5,"type":"Integer"},"avg_score":{"value":60,"type":"Float"},"status":{"value":"active","type":"String"}}

📝 Example 4: UserFunction Management

📋 Total scripts: 11
🔍 Retrieved script: Get Active Users
✏️  function updated
🗑️  function deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed!
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
   {"count":{"value":3,"type":"Integer"},"category":{"type":"String","value":"Furniture"},"avg_price":{"type":"Float","value":365.6666666666667}}
   {"avg_price":{"type":"Float","value":367},"category":{"type":"String","value":"Electronics"},"count":{"value":5,"type":"Integer"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
🚀 ekoDB TypeScript AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer several benefits:

1. **Efficient Similarity Search**: They excel in high-dimensional data retrieval, enabling fast similarity searches based on vector representations, like embeddings.

2. **Scalability**: Designed to handle large-scale datasets, they efficiently manage and index vast amounts of vector data.

3. **Real-time Querying**: Support for real-time access allows for quick responses to queries, which is crucial for applications like recommendation systems.

4. **Enhanced Machine Learning Integration**: They easily integrate with ML models to store and retrieve embeddings, facilitating features like semantic search.

5. **Support for Complex Data Types**: Capable of handling various types of unstructured data, including images, text, and audio.

6. **Metadata Storage**: Many vector databases allow the storage of associated metadata alongside vectors, providing context to the data retrieved.

7. **Versatile Applications**: Useful in various fields, including natural language processing, image recognition, and anomaly detection.

8. **Flexibility**: Can accommodate various distance metrics (e.g., cosine similarity, Euclidean distance), allowing for tailored search functionality.

These advantages make vector databases a powerful tool for modern data-intensive applications.
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
🚀 ekoDB TypeScript Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: LS8AwvDPDoarY-1wNXS1QFZik_sSrIkiztTWNZDzHWrfh614pf8u5QYZzyJkd_6andMxMT7PZGeYapr6qI-OUA
📊 Found 2 product groups
   {"count":{"value":2,"type":"Integer"},"avg_price":{"type":"Float","value":474},"category":{"value":"Furniture","type":"String"}}
   {"category":{"value":"Electronics","type":"String"},"count":{"type":"Integer","value":3},"avg_price":{"type":"Float","value":575.6666666666666}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"category":{"type":"String","value":"Electronics"},"count":{"value":3,"type":"Integer"}}
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
   {"role":{"value":"user","type":"String"},"avg_score":{"type":"Float","value":70},"count":{"type":"Integer","value":7}}
   {"role":{"type":"String","value":"admin"},"avg_score":{"type":"Float","value":20},"count":{"value":3,"type":"Integer"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
🚀 ekoDB TypeScript KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: exVQGRG-8-zXXe6a52fvSaHbhaO7VAZenRXq5ACimpJu41UIna8aZmVJaKpepd4SxeN5oFIfkkCZ068WxQHnFQ
✅ Inserted 2 products with wrapped types

📝 Example 2: UserFunction with Wrapped Type Parameters

✅ Function saved: EGAING45NQKEEwYEo5BIXkQnOXTwSSeQfCgXUZC1iGZBLc7u8j_TK02QKw3tOQte1vLQWxjdDwWN2nqM9IfYPw
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"type":"Object","value":{"userId":"user_abc","role":"admin"}}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: lRn6fi1YNrGPPuCHPSqOZq64QRIrkddPbb3o-SPgxltesmtAcom9QbO0JV0ls6hZBu8n_NEWusKQoNsCla3wvA
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: QeS0eqNfn39ejnbElSf7w3yWhBfoGmSqFamvfieKk3Dz4Yy3N2AceF9ytSKdTCUKPbRWAdvO8J38KSx4JGc-KA
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
🚀 ekoDB TypeScript Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Getting Started with ekoDB (Database)
   2. Database Design Principles (Database)
   3. Natural Language Processing (AI)
   4. Introduction to Machine Learning (AI)
   5. Vector Databases Explained (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"category":{"type":"String","value":"Database"},"count":{"type":"Integer","value":3}}
   {"category":{"type":"String","value":"AI"},"count":{"value":2,"type":"Integer"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Goal Template CRUD Example (TypeScript) ===

--- Creating goal template ---
Created template: Data Migration (id: ytkpWUsrmbJzWVFcnphz1UySkvVRWC0LN1mdL_L9cyz_uSEu11CEQ4LY7QWlFCLHRK7g-CpdGV1FND476BbEew)

--- Listing templates ---
Templates: {
  count: 1,
  items: [
    {
      description: [Object],
      id: 'ytkpWUsrmbJzWVFcnphz1UySkvVRWC0LN1mdL_L9cyz_uSEu11CEQ4LY7QWlFCLHRK7g-CpdGV1FND476BbEew',
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Goals, Tasks & Agents Example (TypeScript) ===

--- Creating goal ---
Created goal: Deploy v2.0 (id: HN6d44HfdlxMEb7ZGLowl8DWb0ZQj6tPasBZfODcazMi790fFRi6gp3HwpUQhPTJ-KC_zy22a3yEIsuM2tsFbA)

--- Listing goals ---
Goals: {
  "count": 1,
  "goals": [
    {
      "created_at": "2026-09-16T13:57:03.725190+00:00",
      "description": "Ship version 2.0 to production",
      "id": "HN6d44HfdlxMEb7ZGLowl8DWb0ZQj6tPasBZfODcazMi790fFRi6gp3HwpUQhPTJ-KC_zy22a3yEIsuM2tsFbA",
      "status": "pending",
      "steps": "[{\"description\":\"Run test suite\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]",
      "title": "Deploy v2.0",
      "updated_at": "2026-09-16T13:57:03.725190+00:00"
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
        "value": "2026-09-16T13:57:03.725190+00:00"
      },
      "description": {
        "type": "String",
        "value": "Ship version 2.0 with hot-fix patches"
      },
      "id": "HN6d44HfdlxMEb7ZGLowl8DWb0ZQj6tPasBZfODcazMi790fFRi6gp3HwpUQhPTJ-KC_zy22a3yEIsuM2tsFbA",
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
        "value": "2026-09-16T13:57:03.746062+00:00"
      }
    }
  ]
}

--- Goal step: start step 0 ---
Step 0 started on goal HN6d44HfdlxMEb7ZGLowl8DWb0ZQj6tPasBZfODcazMi790fFRi6gp3HwpUQhPTJ-KC_zy22a3yEIsuM2tsFbA
--- Goal step: complete step 0 ---
Step 0 completed on goal HN6d44HfdlxMEb7ZGLowl8DWb0ZQj6tPasBZfODcazMi790fFRi6gp3HwpUQhPTJ-KC_zy22a3yEIsuM2tsFbA
--- Goal step: fail step 1 ---
Step 1 failed on goal HN6d44HfdlxMEb7ZGLowl8DWb0ZQj6tPasBZfODcazMi790fFRi6gp3HwpUQhPTJ-KC_zy22a3yEIsuM2tsFbA

--- Completing goal ---
Goal status: pending_review
--- Approving goal ---
Goal status after approve: in_progress

--- Creating goal to reject ---
--- Rejecting goal ---
Goal status after reject: failed


--- Creating task ---
Created task: Hourly Health Check (id: JCkZP3yeHoq8JtDgfwGNUuryq-5Dzz85SpwkciXEpJotYx5-VikImfIlH59_1N_OKiD_RPZiMNk_QgAiD6toqQ)

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
      "id": "JCkZP3yeHoq8JtDgfwGNUuryq-5Dzz85SpwkciXEpJotYx5-VikImfIlH59_1N_OKiD_RPZiMNk_QgAiD6toqQ",
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
Created agent: SupportBot (id: 9n0Tg_h7UolilyOfJSMYsi13sf89X3l9NuZu7YX6E9mHUQEFBIfQOKXznUeCpNG3gGST8W3dmze5NZx69fWARg)

--- Listing agents ---
Agents: {
  "count": 1,
  "items": [
    {
      "deployment_id": {
        "type": "String",
        "value": "deploy_prod_1"
      },
      "id": "9n0Tg_h7UolilyOfJSMYsi13sf89X3l9NuZu7YX6E9mHUQEFBIfQOKXznUeCpNG3gGST8W3dmze5NZx69fWARg",
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
By name: SupportBot (id: 9n0Tg_h7UolilyOfJSMYsi13sf89X3l9NuZu7YX6E9mHUQEFBIfQOKXznUeCpNG3gGST8W3dmze5NZx69fWARg)

--- Updating agent ---
Updated agent: SupportBot

--- Getting agents by deployment ---
Agents in deployment: {
  "count": 0,
  "items": []
}
WARNING: agents-by-deployment omitted created agent 9n0Tg_h7UolilyOfJSMYsi13sf89X3l9NuZu7YX6E9mHUQEFBIfQOKXznUeCpNG3gGST8W3dmze5NZx69fWARg; TODO: check/fix the server-side deployment lookup

--- Deleting agent ---
Agent deleted successfully

--- Cleanup: deleting goals ---
Goals deleted successfully

=== All goals, tasks & agents operations completed ===
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created
✓ ts_users_register saved
✓ ts_users_login saved
✓ ts_users_verify_token saved

=== Auth flow defined as pure stored functions ===
Call them like:
  POST /api/functions/jwt_register_ts_44229_1789567025055 { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/jwt_login_ts_44229_1789567025055 { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/jwt_verify_ts_44229_1789567025055 { "token": "<jwt>" }

Set JWT_SECRET in ekoDB's environment_vars whitelist before invoking.

✓ Cleaned up demo functions
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB KV Document Linking Example (TypeScript) ===

--- Setup: creating KV key and documents ---
Set KV key: session:admin
Inserted document 1: yd6IIWRlX2kl3zX5pPTLxLvb3cYnQarkqZre4qsJ0_H8qXKACr76AnSvSwNlsroQ5bxui6tCg0aMENKH4U-n9A
Inserted document 2: 57ciVJwpuOb_vhrWYNBdxcWkdUQIyce2Qmwm-z-rR-SjH8pfalkpgIDmMh3VYKzihGGoqFkkOtStDUX01LwhBA

--- Linking documents to KV key ---
Linked doc yd6IIWRlX2kl3zX5pPTLxLvb3cYnQarkqZre4qsJ0_H8qXKACr76AnSvSwNlsroQ5bxui6tCg0aMENKH4U-n9A: null
Linked doc 57ciVJwpuOb_vhrWYNBdxcWkdUQIyce2Qmwm-z-rR-SjH8pfalkpgIDmMh3VYKzihGGoqFkkOtStDUX01LwhBA: null

--- Getting links for KV key ---
Links: [
  {
    "collection": "kv_links_example_ts_44269_1789567025406",
    "document_id": "57ciVJwpuOb_vhrWYNBdxcWkdUQIyce2Qmwm-z-rR-SjH8pfalkpgIDmMh3VYKzihGGoqFkkOtStDUX01LwhBA",
    "field_path": null,
    "created_at": "2026-09-16T13:57:05.497356Z",
    "last_accessed": "2026-09-16T13:57:05.499629Z",
    "metadata": {}
  },
  {
    "collection": "kv_links_example_ts_44269_1789567025406",
    "document_id": "yd6IIWRlX2kl3zX5pPTLxLvb3cYnQarkqZre4qsJ0_H8qXKACr76AnSvSwNlsroQ5bxui6tCg0aMENKH4U-n9A",
    "field_path": null,
    "created_at": "2026-09-16T13:57:05.494021Z",
    "last_accessed": "2026-09-16T13:57:05.499629Z",
    "metadata": {}
  }
]

--- Unlinking document ---
Unlinked doc 57ciVJwpuOb_vhrWYNBdxcWkdUQIyce2Qmwm-z-rR-SjH8pfalkpgIDmMh3VYKzihGGoqFkkOtStDUX01LwhBA: null

--- Verifying remaining links ---
Remaining links: [
  {
    "collection": "kv_links_example_ts_44269_1789567025406",
    "document_id": "yd6IIWRlX2kl3zX5pPTLxLvb3cYnQarkqZre4qsJ0_H8qXKACr76AnSvSwNlsroQ5bxui6tCg0aMENKH4U-n9A",
    "field_path": null,
    "created_at": "2026-09-16T13:57:05.494021Z",
    "last_accessed": "2026-09-16T13:57:05.502709Z",
    "metadata": {}
  }
]

=== All KV linking operations completed ===

--- Cleanup ---
Cleanup complete
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: { type: 'Object', value: { username: 'john_doe', userId: 123 } }

=== KV Batch Set ===
✓ Batch set 3 keys
  kv_ops_ts_44298_1789567025865:cache:product:1: success
  kv_ops_ts_44298_1789567025865:cache:product:2: success
  kv_ops_ts_44298_1789567025865:cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  kv_ops_ts_44298_1789567025865:cache:product:1: { name: 'Product 1', price: 29.99 }
  kv_ops_ts_44298_1789567025865:cache:product:2: { price: 39.99, name: 'Product 2' }
  kv_ops_ts_44298_1789567025865:cache:product:3: { name: 'Product 3', price: 49.99 }

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
  kv_ops_ts_44298_1789567025865:cache:product:1: deleted
  kv_ops_ts_44298_1789567025865:cache:product:2: deleted
  kv_ops_ts_44298_1789567025865:cache:product:3: deleted

✓ All KV operations completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
    "name": "Jupiter",
    "diameter_km": 139820
  }
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello! I hope you're having a wonderful day.

=== Done ===
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Schedule Management Example (TypeScript) ===

--- Creating schedule ---
Created schedule: Nightly Database Backup (id: b4c3c035-310d-4b3e-9a23-0deec07b44b1, cron: 0 0 2 * * *)

--- Listing schedules ---
Schedules: {
  "count": 1,
  "schedules": [
    {
      "created_at": "2026-09-16T13:57:10.081119Z",
      "cron_expression": "0 0 2 * * *",
      "description": null,
      "enabled": true,
      "function_label": "schedule_noop_typescript_44451_1789567030044",
      "id": "b4c3c035-310d-4b3e-9a23-0deec07b44b1",
      "last_execution": null,
      "name": "Nightly Database Backup",
      "next_execution": "2026-09-17T02:00:00Z",
      "parameters": {},
      "stats": {
        "avg_execution_time_ms": 0,
        "failed_executions": 0,
        "last_error": null,
        "successful_executions": 0,
        "total_executions": 0
      },
      "timezone": "UTC",
      "updated_at": "2026-09-16T13:57:10.081119Z"
    }
  ]
}

--- Getting schedule ---
Fetched: Nightly Database Backup (cron: 0 0 2 * * *)

--- Updating schedule ---
Updated: Nightly Full Backup (new cron: 0 0 3 * * *)

--- Triggering schedule ---
Trigger response: {
  "schedule_id": "b4c3c035-310d-4b3e-9a23-0deec07b44b1",
  "status": "triggered"
}

--- Pausing schedule ---
Schedule enabled after pause: false

--- Resuming schedule ---
Schedule enabled after resume: true

--- Deleting schedule ---
Schedule deleted successfully

=== All schedule operations completed ===
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
  1. Score: 0.773, Matched:
  2. Score: 0.743, Matched:
  3. Score: 0.730, Matched:

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.509, Matched: title, content
  2. Score: 0.897, Matched: content, title
  3. Score: 0.292, Matched:

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: bio, skills

9. Vector search with a metadata pre-filter (category = ml):
Found 2 documents in category "ml" (NLP excluded)
  1. Introduction to Machine Learning (category: ml)
  2. Deep Learning Fundamentals (category: ml)


✅ Search examples completed!
=== Cleanup ===
✅ Deleted test collections
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: '6uPvvKVzFjh_rV8M4z8nSfJujtBcV0jkKBuku-mrViwy3E_ix8siVZ_ySTLX9kg6rLJCGkMwgpta-ZcBalDSig'
}

=== Find by ID ===
Found: {
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  name: { type: 'String', value: 'Test Record' },
  price: { value: 99.99, type: 'Float' },
  metadata: { type: 'Object', value: { key: 'value', nested: [Object] } },
  active: { type: 'Boolean', value: true },
  value: { type: 'Integer', value: 42 },
  created_at: { value: '2026-09-16T13:57:11.521+00:00', type: 'DateTime' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  id: '6uPvvKVzFjh_rV8M4z8nSfJujtBcV0jkKBuku-mrViwy3E_ix8siVZ_ySTLX9kg6rLJCGkMwgpta-ZcBalDSig'
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-09-16T13:57:11.521Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { key: 'value', nested: { deep: true } }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  name: 'Test Record',
  price: 99.99,
  metadata: { key: 'value', nested: { deep: true } },
  active: true,
  value: 42,
  created_at: '2026-09-16T13:57:11.521+00:00',
  tags: [ 'tag1', 'tag2', 'tag3' ],
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  categories: [ 'electronics', 'computers' ],
  data: 'aGVsbG8gd29ybGQ=',
  id: '6uPvvKVzFjh_rV8M4z8nSfJujtBcV0jkKBuku-mrViwy3E_ix8siVZ_ySTLX9kg6rLJCGkMwgpta-ZcBalDSig'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  value: { value: 100, type: 'Integer' },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  id: '6uPvvKVzFjh_rV8M4z8nSfJujtBcV0jkKBuku-mrViwy3E_ix8siVZ_ySTLX9kg6rLJCGkMwgpta-ZcBalDSig',
  active: { value: true, type: 'Boolean' },
  metadata: { value: { key: 'value', nested: [Object] }, type: 'Object' },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  name: { value: 'Updated Record', type: 'String' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  price: { type: 'Float', value: 99.99 },
  created_at: { type: 'DateTime', value: '2026-09-16T13:57:11.521+00:00' },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: 1e3c0qoAn3EcbJ0T-vP_sk1akMc5ZCksB5OTpdyBFlKzdRedIyl_ZfZMpS6l2gszuvSzJfZ5m_YqsDuaKcH8kQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
✓ Created native SWR script: github_user_native_ts (rF7Co0dMr7y5ZPXvkAWjgd9-5g2ZnbjczF3rXpzckpfojICxG-F6KjX79yVrPZkfisf2j4qzDbghL3aS4dRCVQ)

First call (cache miss - will fetch from GitHub API):
  Response time: 174ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 5ms
  Speedup: 34.8x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_ts (7vHIO0IpfPdBYxdheHiYgfo6Dchr6KA2F2-oMFVV12MgkXymvbLU1F_DzYg5Ziit8oukluO-nJtqEQGfjBYhjQ)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_ts' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_ts (YkpH-3izGy9ESfBr5tcZik5FmMZGcm2S7JyVbEiD6RGWQF2pO1FajLgBtJams63VHl89GspGYomlsDg8MIJ_Vw)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_ts' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_ts (ZjZwehFwX-NcY4B5PNNs-1djiXI77f_Hv7hgpgTtZ4jWwno58qv-lBKfGCZxAbMUBYAhUL5UF3YDEASB3RCHEQ)
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_ts_44654_1789567033797 (C4aA0eLf1hBeFUm0J_ERRpbszRx-G9G_NCan1KV-iQ1aPor2-52ZQECBM4pVvgwKCZbt1PUg8Y4QmiWtmOrgKA)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "username": "Bret",
          "name": "Leanne Graham",
          "phone": "1-770-736-8031 x56442",
          "website": "hildegard.org",
          "email": "Sincere@april.biz",
          "id": 1,
          "address": {
            "suite": "Apt. 556",
            "zipcode": "92998-3874",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
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
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 8ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "username": "Bret",
          "name": "Leanne Graham",
          "phone": "1-770-736-8031 x56442",
          "website": "hildegard.org",
          "email": "Sincere@april.biz",
          "id": 1,
          "address": {
            "suite": "Apt. 556",
            "zipcode": "92998-3874",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
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
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_reviews_ts_44654_1789567033797 (9GM0HCuwR1ivD2uQ9eMyydEJbYUgO_522x5CaIHNLMOMxkGbfFSfOhOXCH9rTRHv2dx5eThSftw4bLdM4EXi-w)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "category": "beauty",
          "stock": 99,
          "reviews": [
            {
              "rating": 3,
              "comment": "Would not recommend!",
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerName": "Eleanor Collins",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com"
            },
            {
              "reviewerName": "Lucas Gordon",
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerEmail": "lucas.gordon@x.dummyjson.com",
              "comment": "Very satisfied!",
              "rating": 4
            },
            {
              "reviewerName": "Eleanor Collins",
              "comment": "Highly impressed!",
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "rating": 5
            }
          ],
          "price": 9.99,
          "sku": "BEA-ESS-ESS-001",
          "dimensions": {
            "depth": 22.99,
            "height": 13.08,
            "width": 15.14
          },
          "shippingInformation": "Ships in 3-5 business days",
          "id": 1,
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "warrantyInformation": "1 week warranty",
          "minimumOrderQuantity": 48,
          "brand": "Essence",
          "availabilityStatus": "In Stock",
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "discountPercentage": 10.48,
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "rating": 2.56,
          "meta": {
            "updatedAt": "2026-05-23T11:27:41.868Z",
            "createdAt": "2025-10-09T14:47:01.588Z",
            "barcode": "5784719087687",
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png"
          },
          "returnPolicy": "No return policy",
          "tags": [
            "beauty",
            "mascara"
          ],
          "title": "Essence Mascara Lash Princess",
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

perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: l0t13w8Bc4PT2zMBHo1AFUumOu1sFUwAomyDicvNbxA9FkYFKKEPIfs2US_zth50yad4bHn3c4nXOHLizu59Tg
Created Bob: $500 - ID: n6a6xjh10AiKr1vibZGSzcaMUG0et7s9t1Vokkylye8zV9JTtpmlw63oYVSnWaiSGFKSydowehMeNvSkNEmQ1A

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 703dded5-cf26-4f0c-bfee-a1a636e21028

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
New transaction: becf42c6-cde9-4439-bfc3-f07b19ca41d9
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

=== Create User Function ===
Created user function with ID: t5Cs3dhQH9reoGHcdmUSctND3NldctK0NB7ztdKpDkQPiF9CX3FbIKAGQrpyNvP3kL9Z-7oNz6cCf6wgY74Qrg

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 11 user functions:
  - conc_demo_rl_fail_ts_30955_1789566761407: Rate-limit (fail mode)
  - conc_demo_rl_skip_ts_43664_1789567012579: Rate-limit (skip mode)
  - conc_demo_rl_skip_ts_30955_1789566761407: Rate-limit (skip mode)
  - conc_demo_rl_fail_ts_19088_1789566533301: Rate-limit (fail mode)
  - conc_demo_rl_fail_ts_43664_1789567012579: Rate-limit (fail mode)
  - get_active_users_ts: Get Active Users
  - get_active_users_client_ts_updated: Get Active Users (Updated)
  - fetch_product_reviews_ts_32495_1789566782714: Fetch Product with Reviews (Multi-API)
  - fetch_product_reviews_ts_44654_1789567033797: Fetch Product with Reviews (Multi-API)
  - conc_demo_rl_skip_ts_19088_1789566533301: Rate-limit (skip mode)
  - avg_score_by_role_ts_19467_1789566544643: Average Score by Role

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== WebSocket Chat Streaming Example (TypeScript) ===

Created chat session: 55eVXkoOfGX8HW1fCOZbqEC4HP7MHmAuTTT2zNEcwkn8sg1uIjD-QLc7FROp8RrcWAy7Fj4sRuVx83HUw4OFcw

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: oeTGyOd-IEst_e64cXenHWlW2apIKAcb5ntf1VQIQghxvDpfwI7WdTDWqypW-oageUm7Uhm6QqvdPLISN0Bd2g
Execution time: 513ms
Token usage: {"completion_tokens":8,"prompt_tokens":15,"total_tokens":23}

Full response: {"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris....
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== WebSocket Subscription Example ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_ts' ===
✓ Subscribed (subscription_id: sub_8014a7a55c2446a796e88a5008925e01)

=== Performing mutations to trigger notifications ===
Inserting a record...
✓ Inserted record: 4xm8tj3LhYAYPvY4Ad5d8AT95dz3NFkPbq97CFPUgtECjAmBcJ30bqqbKSG4k1gZXONXo3FHznmMdjaXOp6OFQ
  📡 Notification received for 4xm8tj3LhYAYPvY4Ad5d8AT95dz3NFkPbq97CFPUgtECjAmBcJ30bqqbKSG4k1gZXONXo3FHznmMdjaXOp6OFQ

Inserting another record...
✓ Inserted record: U6uw4WnOPK2IV9a57c-Vd5nwhFJ9HHGIoqFALwJF6mSMakUFQVgEicxvbUss-eD8XB5T6TNHiMRaXzo8E-2pyw
  📡 Notification received for U6uw4WnOPK2IV9a57c-Vd5nwhFJ9HHGIoqFALwJF6mSMakUFQVgEicxvbUss-eD8XB5T6TNHiMRaXzo8E-2pyw

=== Unsubscribing ===
✓ Unsubscribed: {"collection":"ws_subscribe_example_ts","found":true,"unsubscribed":true}

✓ WebSocket subscription example completed successfully
✓ Deleted collection 'ws_subscribe_example_ts'
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: X4Mv0G4FlDcPOX7EizLypZMulLe37eUVo6jeD4ARSpPXT41kboNNNDBb5fQZxr_-GkndC-_jMsnpmLHU8Vzvpg

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: {"id":"lpwUJJ0Zdyfln0VIf2q-L1LXJe3BF6bwNkiIHNo7l9KEYtghfkkJM94hJaNWH_hY-FrfvqtO6HK275he3rj6NA"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"FAnu5BvYFg3qt_c26mQA_M2TbjFxQUjcZ19od3_53zdtW581SKr8d6zRoIifgjwYHyDE3GbdICxJU9AuaqX4Bg"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"id":"lpwUJJ0Zdyfln0VIf2q-L1LXJe3BF6bwNkiIHNo7l9KEYtghfkkJM94hJaNWH_hY-FrfvqtO6HK275he3rj6NA","price":{"value":150,"type":"Integer"},"name":{"type":"String","value":"Product 1"}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"id":"oVupjWL0yw8kwtMHQljrEPBdYEtnWlghP_dVa1v_qpveo4wS-tLVIUIeNIjRrJ4bOdeaBE2HGo-Qqmx_kKEVBg"}

✅ All bypass_ripple operations completed successfully!
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ["id","name","email"]
  First user: Alice Johnson <alice@example.com>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["created_at","email","user_role","bio","avatar_url","age","id","name","status"]

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
    - Fields: ["password","secret_token","age","email","created_at","id","api_key","status","user_role","bio","avatar_url","name"]
  Projected query:
    - 3 fields per record
    - Fields: ["name","id","email"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!

🧪 Running JavaScript examples (direct HTTP/WebSocket)...

added 1 package, removed 1 package, and audited 9 packages in 3s

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
  id: 'F8fDcJ4m7sZHXQmlukN3Y6trYGCjr8kj5-zd1jsO5davj6UmHT1cyay0sTDApPXrIBg-XSK_3tCySK549PS3vw'
}

=== Find by ID ===
Found: {
  name: { type: 'String', value: 'Test Record' },
  active: { type: 'Boolean', value: true },
  id: 'F8fDcJ4m7sZHXQmlukN3Y6trYGCjr8kj5-zd1jsO5davj6UmHT1cyay0sTDApPXrIBg-XSK_3tCySK549PS3vw',
  value: { value: 42, type: 'Integer' }
}

=== Find with Query ===
Found documents: [
  {
    active: { value: true, type: 'Boolean' },
    name: { type: 'String', value: 'Test Record' },
    id: 'F8fDcJ4m7sZHXQmlukN3Y6trYGCjr8kj5-zd1jsO5davj6UmHT1cyay0sTDApPXrIBg-XSK_3tCySK549PS3vw',
    value: { value: 42, type: 'Integer' }
  }
]

=== Update Document ===
Updated: {
  value: { type: 'Integer', value: 100 },
  id: 'F8fDcJ4m7sZHXQmlukN3Y6trYGCjr8kj5-zd1jsO5davj6UmHT1cyay0sTDApPXrIBg-XSK_3tCySK549PS3vw',
  name: { type: 'String', value: 'Updated Record' },
  active: { value: true, type: 'Boolean' }
}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ simple_crud.js completed successfully

=== Running ekoDB/ekodb-client/examples/javascript/simple_websocket.js ===
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: tuqqPo65s9NE1l4L0kSO8amhdoidyx2Fk_IEq-_EyOMswmorSeE9YZZrvwb4IIhcmvprhg5ypVrxZSnCdUYkdg

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "type": "Success",
  "payload": {
    "data": [
      {
        "id": "tuqqPo65s9NE1l4L0kSO8amhdoidyx2Fk_IEq-_EyOMswmorSeE9YZZrvwb4IIhcmvprhg5ypVrxZSnCdUYkdg",
        "active": {
          "value": true,
          "type": "Boolean"
        },
        "value": {
          "value": 42,
          "type": "Integer"
        },
        "name": {
          "value": "WebSocket Test Record",
          "type": "String"
        }
      }
    ]
  },
  "messageId": "1789495144348"
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

✅ Function saved: 8WvIDGgAtJWxUpiDWtSh-pVCGanOohyLVCHP94_jGQwVBklGMVlO0pYGt0U6QzqZ5_RiKX4AVbVoJbV-IlHArA
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: eU4Vxjhng9slQoKuArxowH6bM3t1299gyCsfF6aRd3midRztkjqWUrNcae7zKx637cX7whDJtmpXlUL_h9Q17A
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: OGllAFnMS3I4LOCc8XA579bDAYajW3vG3GJPu65CkpZ7UNAsZ0L8D-4bRcXfEIurso_p6EX_NyGobRULdZ7Dsw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"status":{"value":"inactive","type":"String"},"avg_score":{"type":"Float","value":50},"max_score":{"type":"Integer","value":90},"count":{"value":5,"type":"Integer"}}
   {"count":{"type":"Integer","value":5},"avg_score":{"type":"Float","value":60},"status":{"type":"String","value":"active"},"max_score":{"type":"Integer","value":100}}

📝 Example 4: Function Management

📋 Total functions: 14
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
Retrieved value: { type: 'Object', value: { userId: 123, username: 'john_doe' } }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { value: { name: 'Product 1', price: 29.99 }, type: 'Object' }
cache:product:2: {
  type: 'Object',
  value: { name: 'Product 2', price: 39.989999999999995 }
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
Collection created with first record: Rnfa4g453_cS0g9ZqFa67BEVsYG-XKlFPj7SnCEbrZacCNmH7WvUZwoJbgdSa_PQcDvV-iUUg0AftHqwa_QXFA

=== List Collections ===
Total collections: 33
Sample collections: [
  'schema_employees_client_ts',
  'schema_products_client_go',
  'agent_function_versions__ek0_testing',
  'chat_agent_configs__ek0_testing',
  'chat_goal_templates__ek0_testing'
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
Created Alice: $1000 - ID: 954rkVxajrQ5KMhClBx2c-hQgH-misT_OnJMU1CLivB6zmFjz1grHlIchxTgd-JK0VIw35Yjlw1CV_TyYTZziQ
Created Bob: $500 - ID: PQd6SZUNfeHHyoHwnu_2f1hy9NiCYmXC2RCdwgHQYGAOMX4jBn8ykOfnlBL6rHtD4lca3jUTQ6tR5STud5lbHw

=== Example 1: Begin Transaction ===
Transaction ID: 2ce6b13e-de85-46bc-9e33-6651f29b35bd

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
New transaction: 846e0741-efd5-4614-8d1b-5b49f67fdba1
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
   ✅ Function saved: vRhdtTd7Oo38hyjV6gZvLj70OU4wi5brcnaifzm2hF17--vspedeC2SfjfeF4FVcB2GxTAKM2j3PEfPw_-tJPQ

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
   ✅ Function saved: 9c-4ayaieVe_9pFO3hek4gTBuMEBoFy0cpQVoJnfafEyTYG_2BSto9OK9WyRCJB6ypvFImajElh0uD12qZMKEw

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
   ✅ Function saved: 0Thm3ncO1j_IGYGtopU1f52gKJJpynW8bXNceXQam2rMnnl3B81wZDaVTLlPjYnvzL4tlGmI13eNk4xRzMazzw

2️⃣ Calling Function (Query + Update Credits + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"value":100,"type":"Integer"}
   📋 Status: {"value":"active","type":"String"}
   📋 Name: {"type":"String","value":"Alice Smith"}

============================================================
📝 Function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: gRiNk7jdsd11SFuqWL0TRz1segjWtAiSdldg3ajhX8deKZIKojRtxjfxQt1tJzj1evWT4EjW7vqeDO-BSu0YDg

2️⃣ Calling Function (Query + Delete + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted function: vRhdtTd7Oo38hyjV6gZv...
   ✅ Deleted function: 9c-4ayaieVe_9pFO3hek...
   ✅ Deleted function: 0Thm3ncO1j_IGYGtopU1...
   ✅ Deleted function: gRiNk7jdsd11SFuqWL0T...
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
  Output: Document ID = sygB9G5o9JyKciMLRHKcSbo_UYajm4R6XkeitKOs08EXOQGGbV-U1ew1DBqpRMpD2NS4yggRnK2QvcMGT60O0g
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(sygB9G5o9JyKciMLRHKcSbo_UYajm4R6XkeitKOs08EXOQGGbV-U1ew1DBqpRMpD2NS4yggRnK2QvcMGT60O0g)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(sygB9G5o9JyKciMLRHKcSbo_UYajm4R6XkeitKOs08EXOQGGbV-U1ew1DBqpRMpD2NS4yggRnK2QvcMGT60O0g)
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
  Output: Document ID = UnGbqnoSPZ-1akQnWs16JFPPxKVV1IbI1ZiSX8B2Bs76YwYmYyVymKscS8IQdp1ZqR6La6y8EQTN7ASgAT0KwA
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(UnGbqnoSPZ-1akQnWs16JFPPxKVV1IbI1ZiSX8B2Bs76YwYmYyVymKscS8IQdp1ZqR6La6y8EQTN7ASgAT0KwA)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(UnGbqnoSPZ-1akQnWs16JFPPxKVV1IbI1ZiSX8B2Bs76YwYmYyVymKscS8IQdp1ZqR6La6y8EQTN7ASgAT0KwA)
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

> @ekodb/ekodb-client@0.26.4 prepare
> npm run build


> @ekodb/ekodb-client@0.26.4 build
> tsc


up to date, audited 46 packages in 4s

12 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.26.4 build
> tsc

✅ TypeScript client built!

added 1 package, removed 1 package, and audited 9 packages in 3s

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities

added 1 package, removed 1 package, and audited 13 packages in 3s

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> ekodb-typescript-examples@1.0.0 build
> tsc

bash: warning: setlocale: LC_ALL: cannot change locale (C.UTF-8): No such file or directory
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
✓ Created session: 7C-N9dULIr9A-vLIwmI4GH2gp9TftHWmSOYLC1noqeuc1u4tZ3HhpUZR8ZjFFHzMaORwHDR7DEQ9fetcJ_Yptg

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you have any more questions or need further information, feel free to ask!

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
✓ Created second session: hCkoSQHcG22ihQqUV0v7VbGtTJXM0VbX73mN9kqlbZez8zmP9iT7VahYxJLfzOXBuJdqIYQzWS6H6PJTvyU3kg
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
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: z-5DbLxhrjnqZOcmcj8j_0Wdsj-ytze8jY0rqjCVibULNicquu8OHgdWxkuvTeSidmVl4exznBdI0l-AgvhXHQ

=== Sending Chat Message ===
Message ID: RwxVWoGms3JdiSa77MncQNBQBG59DkVBrcx44sH6MYWK40NwA-br2UCw3tmRZ27EmmOWeQkFArZf1p7mKvJ46Q

=== AI Response ===
Here are the available products along with their prices:

1. **ekoDB Cloud**
   - Price: $499
   - Description: Fully managed cloud database service product

2. **ekoDB**
   - Price: $99
   - Description: A high-performance database product with AI capabilities

3. **ekoDB Pro**
   - Price: $299
   - Description: Enterprise edition product with advanced features

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    price: 499,
    name: 'ekoDB Cloud',
    description: 'Fully managed cloud database service product',
    id: 'eWArQCVvbmm-PHKBCjpXnrfqDV2KScsHeaacktY3lCS3KciyIWFJ3uR5OtGIchqyybMFsM7lavsWMwmJ1z1yUw'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    price: 99,
    id: '0FbZA3OMJjK0bsViUXHNNOZfB_ZQ4QPN_VCjd1AKAAL1YiI5UEwfVCpNHO0AQCtap4Ev_cFWq8kQoJYR3f6Eow',
    name: 'ekoDB',
    description: 'A high-performance database product with AI capabilities'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB Pro',
    description: 'Enterprise edition product with advanced features',
    id: 'Aez4fHwJYOknfEStP3fvVzbTBvFZzPWalHBIfWa6KPC5mFfZnPFQcgEhVXEIeevRFchglDu3McmB91OkvVCDmg',
    price: 299
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 6681ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 91
Total tokens: 3504

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
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
    - gpt-image-2.5-flare
    - gpt-image-2.5-sunburst
    - gpt-image-2.5-flare-2026-09-08
    - gpt-image-2.5-sunburst-2026-09-08
    - gpt-live-1
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
  openai: ok 132 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get Specific Provider Models ===
OpenAI models: text-embedding-ada-002, whisper-1, gpt-3.5-turbo, tts-1, gpt-3.5-turbo-16k, gpt-4-0613, gpt-4, davinci-002, babbage-002, gpt-3.5-turbo-instruct, gpt-3.5-turbo-instruct-0914, gpt-3.5-turbo-1106, tts-1-hd, tts-1-1106, tts-1-hd-1106, text-embedding-3-small, text-embedding-3-large, gpt-3.5-turbo-0125, gpt-4-turbo, gpt-4-turbo-2024-04-09, gpt-4o, gpt-4o-2024-05-13, gpt-4o-mini-2024-07-18, gpt-4o-mini, gpt-4o-2024-08-06, omni-moderation-latest, omni-moderation-2024-09-26, o1-2024-12-17, o1, o3-mini, o3-mini-2025-01-31, gpt-4o-2024-11-20, gpt-4o-mini-search-preview-2025-03-11, gpt-4o-mini-search-preview, gpt-4o-transcribe, gpt-4o-mini-transcribe, o1-pro-2025-03-19, o1-pro, gpt-4o-mini-tts, o3-2025-04-16, o4-mini-2025-04-16, o3, o4-mini, gpt-4.1-2025-04-14, gpt-4.1, gpt-4.1-mini-2025-04-14, gpt-4.1-mini, gpt-4.1-nano-2025-04-14, gpt-4.1-nano, gpt-image-1, o4-mini-deep-research, gpt-4o-transcribe-diarize, o4-mini-deep-research-2025-06-26, gpt-5-chat-latest, gpt-5-2025-08-07, gpt-5, gpt-5-mini-2025-08-07, gpt-5-mini, gpt-5-nano-2025-08-07, gpt-5-nano, gpt-audio-2025-08-28, gpt-realtime, gpt-realtime-2025-08-28, gpt-audio, gpt-5-codex, gpt-image-1-mini, gpt-5-pro-2025-10-06, gpt-5-pro, gpt-audio-mini, gpt-audio-mini-2025-10-06, gpt-5-search-api, gpt-realtime-mini, sora-2, sora-2-pro, gpt-5-search-api-2025-10-14, gpt-5.1-chat-latest, gpt-5.1-2025-11-13, gpt-5.1, gpt-5.1-codex, gpt-5.1-codex-mini, gpt-5.1-codex-max, gpt-image-1.5, gpt-5.2-2025-12-11, gpt-5.2, gpt-5.2-pro-2025-12-11, gpt-5.2-pro, gpt-5.2-chat-latest, gpt-4o-mini-transcribe-2025-12-15, gpt-4o-mini-transcribe-2025-03-20, gpt-4o-mini-tts-2025-03-20, gpt-4o-mini-tts-2025-12-15, gpt-realtime-mini-2025-12-15, gpt-audio-mini-2025-12-15, chatgpt-image-latest, gpt-5.2-codex, gpt-5.3-codex, gpt-realtime-1.5, gpt-audio-1.5, gpt-4o-search-preview, gpt-4o-search-preview-2025-03-11, gpt-5.3-chat-latest, gpt-5.4-2026-03-05, gpt-5.4-pro, gpt-5.4-pro-2026-03-05, gpt-5.4, gpt-5.4-nano-2026-03-17, gpt-5.4-nano, gpt-5.4-mini-2026-03-17, gpt-5.4-mini, gpt-image-2, gpt-image-2-2026-04-21, gpt-5.5, gpt-5.5-2026-04-23, gpt-5.5-pro, gpt-5.5-pro-2026-04-23, chat-latest, gpt-realtime-translate, gpt-realtime-2, gpt-realtime-whisper, gpt-5.6-sol, gpt-5.6-terra, gpt-5.6-luna, gpt-realtime-2.1, gpt-realtime-2.1-mini, gpt-transcribe, gpt-live-transcribe, gpt-6-astra, gpt-image-2.5-flare, gpt-image-2.5-sunburst, gpt-image-2.5-flare-2026-09-08, gpt-image-2.5-sunburst-2026-09-08, gpt-live-1

=== Get Anthropic Models ===
Anthropic models: claude-fable-5-1, claude-opus-5, claude-sonnet-5, claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 404: {"error":"Unknown provider 'nonexistent_provider_xyz'","error_kind":"unknown_provider"}

✓ Chat Models example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: lXOYCcqcKWvHw5gX-4Njcz1Z7aDg_4hAtIeYtj6yqWejyAuCJsCYqeFiBc7y8H5S--XeDs3I4sDPe8otKzh8SA

=== Sending Messages ===
✓ Message 1 sent
  Response: Currently, the available product is:

- **Name:** ekoDB
- **Description:** A high-performance database product
- **Price:** $99

If you need more information or details about other products, please let me know!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: bK3Jeh_ye6jDStQmcUt2VHZB2DbEV-oUUnX-50nF1Gs7qWngtlK-DT57QTvJZLCZen2D9IQI5RUVGV63x-s2pA
  Parent: lXOYCcqcKWvHw5gX-4Njcz1Z7aDg_4hAtIeYtj6yqWejyAuCJsCYqeFiBc7y8H5S--XeDs3I4sDPe8otKzh8SA

=== Listing Sessions ===
✓ Found 7 sessions
  Session 1: bK3Jeh_ye6jDStQmcUt2VHZB2DbEV-oUUnX-50nF1Gs7qWngtlK-DT57QTvJZLCZen2D9IQI5RUVGV63x-s2pA (Untitled)
  Session 2: lXOYCcqcKWvHw5gX-4Njcz1Z7aDg_4hAtIeYtj6yqWejyAuCJsCYqeFiBc7y8H5S--XeDs3I4sDPe8otKzh8SA (Untitled)
  Session 3: hCkoSQHcG22ihQqUV0v7VbGtTJXM0VbX73mN9kqlbZez8zmP9iT7VahYxJLfzOXBuJdqIYQzWS6H6PJTvyU3kg (Untitled)
  Session 4: vuKbaGHsQC94yLa7_2ro5Vdy1xg9A2RA5KvKSyzx1oJLcCAi4D6DZ8eKRvWcmz0zxkBszT3iMxPubOmpZYLlXw (Untitled)
  Session 5: YUYboJLJZ3yqwxN1Cb5yCHopx8VzkWngmGBfDuMzwk5ocLrAA3onhZxGs9kFbV_RmSc9V1V2HhdynIh_f9G3Ww (Untitled)
  Session 6: MAxsP25SeTa3zmDskKituh1NEf7CdZJC9f2ywIhfXYYV748h-w5M7KwY6MKOrpZK8nSLu8xnE7EilU_pKoIjUg (Untitled)
  Session 7: SflIr8vmbb7jS_W_FfaYgsH9mYpw2-RnHYV0wV1SwX1f-j6HhxF5WJTlBhQmO8kpxpJki7WWPq2VM9MJBc5OQg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: bK3Jeh_ye6jDStQmcUt2VHZB2DbEV-oUUnX-50nF1Gs7qWngtlK-DT57QTvJZLCZen2D9IQI5RUVGV63x-s2pA

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: 1sMzGLznO_2yNqKHYMYewkCuEKNbD78UNdOTeQ3obu1y5esYPWAitTUjVhaiRjRYpd_bcSvbFKNnNeoxplygYA

=== List Collections ===
Total collections: 33
Sample collections: schema_employees_client_ts,schema_products_client_go,agent_function_versions__ek0_testing,chat_agent_configs__ek0_testing,chat_goal_templates__ek0_testing

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: QmzH-QI5oYOfghWmXH0pi14rdqVeZFDGIm4DVaJoSKUWmv6NkAT7Y9IDcWckL1B57us9PaY3z87KwosMUtbxEQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: PCfTFvc3d3NeJ9hqgUdP_rBgTDAuh4R8KUqhF2iFee34zOmt7Fbiqd1-FOq2ybyo9xPLIKICnjLTFATs8uus5g

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
✓ Edge cache script created: wV_6d37BMQkiluixPD6uxsnVvChxDV3u0w2I0eYsPIzPQmZn3KzuhHBzp-o8ovDDl-5V4dQ0m5BTO59qVvB0oQ

Call 1: Cache miss (fetches from API)
Response time: 454ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "timezone_abbreviation": "GMT",
          "current_units": {
            "interval": "seconds",
            "time": "iso8601",
            "temperature_2m": "°C"
          },
          "utc_offset_seconds": 0,
          "generationtime_ms": 0.035762786865234375,
          "longitude": -73.99308,
          "latitude": 40.710335,
          "elevation": 32,
          "timezone": "GMT",
          "current": {
            "temperature_2m": 22.3,
            "interval": 900,
            "time": "2026-09-15T18:00"
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
Response time: 14ms (32.4x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "timezone_abbreviation": "GMT",
          "current_units": {
            "interval": "seconds",
            "time": "iso8601",
            "temperature_2m": "°C"
          },
          "utc_offset_seconds": 0,
          "generationtime_ms": 0.035762786865234375,
          "longitude": -73.99308,
          "latitude": 40.710335,
          "elevation": 32,
          "timezone": "GMT",
          "current": {
            "temperature_2m": 22.3,
            "interval": 900,
            "time": "2026-09-15T18:00"
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

ℹ️  Function 'fetch_user' already existed — updated instead
✅ Saved reusable function: fetch_user
ℹ️  Function 'get_user_wrapper' already existed — updated instead
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"type":"String","value":"User 1"}
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
   ⏱️  Duration: 163ms
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
   ⏱️  Duration: 64ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "company": {
        "name": "Romaguera-Crona",
        "bs": "harness real-time e-markets",
        "catchPhrase": "Multi-layered client-serv...
   🚀 Cache speedup: 2.5x faster!

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
🚀 ekoDB Functions Example (JavaScript Client)

✅ Client initialized (token exchange automatic)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: __zCG9k_e6iC2dlKBVqyj6gAKFms1hp5fEr7JV0vnhZ8r6cBdqJVBHlusXY6kU7Gk2KWXsE-ralYbMI9pv-hAA
📊 Found 5 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

✅ Function saved
📊 Found 3 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 2 groups
   {"avg_score":{"value":60,"type":"Float"},"count":{"value":5,"type":"Integer"},"status":{"type":"String","value":"active"}}
   {"count":{"type":"Integer","value":5},"avg_score":{"type":"Float","value":50},"status":{"value":"inactive","type":"String"}}
⏱️  Execution time: 0ms

📝 Example 4: Function Management

📋 Total functions: 15
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
   1. Laptop Pro - $1299 (⭐4.8)
   2. Webcam HD - $119 (⭐4.5)
   3. Desk Chair - $349 (⭐4.2)
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
   1. Higher precision: Vector databases use mathematical points, lines, and polygons to represent spatial data, hence provide draw precision compared to raster databases.

2. Scalability: The data can be scaled up or down without losing quality, making it suitable for mapping operations on various scales.

3. Less storage: Vector databases tend to use less storage space compared to raster databases because they only need to store information about the vertices and paths of the geometric figures.

4. Detailed attributes: They allow the attachment of extensive attributes to individual objects, which provides comprehensive information for each spatial feature.

5. Better Analysis: Vector data support a wide spectrum of analysis like topological and network analysis which might not be supported in a raster database.

6. Easier editing: It is generally easier and more versatile to edit vector objects compared to raster objects.

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
🚀 ekoDB JavaScript Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: -kWyfj0OJJq-oqirBP7d_0noSgjU4hzfEj9MPEgVbvrTzglQGvQHt0qpJB9UJgy0H33LLhtMbnbtZ3J5tggZhg
📊 Found 2 product groups
   {"category":{"value":"Electronics","type":"String"},"avg_price":{"type":"Float","value":575.6666666666666},"count":{"type":"Integer","value":3}}
   {"avg_price":{"value":474,"type":"Float"},"category":{"type":"String","value":"Furniture"},"count":{"type":"Integer","value":2}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"category":{"value":"Electronics","type":"String"},"count":{"type":"Integer","value":3}}
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
   1. User 7 - Score: 70
   2. User 6 - Score: 60
   3. User 5 - Score: 50
   4. User 4 - Score: 40
   5. User 8 - Score: 80
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
🚀 ekoDB JavaScript KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: mRy1cjtAKIz-00hEiZb_3UC1w-tI9Q-viqDxPQUPmzmpeBSGruINI3XkrFjMEw-ZUxzsiyGsXlnokWr9CjvrnA
✅ Inserted 2 products with wrapped types

📝 Example 2: Function with Wrapped Type Parameters

✅ Function saved: ICaVe7UPSxr45L0n6JW0eAYmnBlbYviaevg5HfBn12ieLEHuR4Da9rqCcu9bcTyrUMujFgA--gBcNKRi_rhF2g
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"value":{"role":"admin","userId":"user_abc"},"type":"Object"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: ZW6XSLEWzNWFj2O3AXv-CDadGsa1MfE4w02LT4ZC22oZdmVwX7MkmXJpYsP6jwzGFLY-aXEXZ5R-SSxJntNf9g
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: FpLJ_ZAPzhgO64eVTNnf9naIwRFt7mdRLU8-N4bJ_X7UHPvoZYoJWGZVvlZg6-jOaQjau-ccnDLgUsCQcmSqew
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
   3. Database Design Principles (Database)
   4. Natural Language Processing (AI)
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
   1. Introduction to Machine Learning
   2. Getting Started with ekoDB
   3. Database Design Principles
   4. Natural Language Processing
   5. Vector Databases Explained
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Function saved
📊 Projected documents (showing first 3):
   1. Introduction to Machine Learning
   2. Getting Started with ekoDB
   3. Database Design Principles
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
Retrieved value: { type: 'Object', value: { userId: 123, username: 'john_doe' } }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { value: { name: 'Product 1', price: 29.99 }, type: 'Object' }
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
Total keys in store: 9

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
  1. Score: 13.200, Matched: title, bio
  2. Score: 13.200, Matched: title, bio
  3. Score: 13.200, Matched: bio, title
  4. Score: 13.200, Matched: bio, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400, Matched: bio, title
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
  1. Score: 0.774, Matched:
  2. Score: 0.752, Matched:
  3. Score: 0.742, Matched:

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.501, Matched: content, title
  2. Score: 0.897, Matched: title, content
  3. Score: 0.310, Matched:

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: skills, bio

=== Cleanup ===
✅ Deleted test collections

✅ Search examples completed!
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: '4-MurzqSw7MnkPrGv4sdvEF9RQMHE8AGa2bU7o3ywT-ix5adnxaY7tLHCaQWObIjxsHIjREUP-WbTfOUxgVGnA'
}

=== Find by ID ===
Found: {
  active: { value: true, type: 'Boolean' },
  value: { type: 'Integer', value: 42 },
  name: { type: 'String', value: 'Test Record' },
  id: '4-MurzqSw7MnkPrGv4sdvEF9RQMHE8AGa2bU7o3ywT-ix5adnxaY7tLHCaQWObIjxsHIjREUP-WbTfOUxgVGnA'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  id: '4-MurzqSw7MnkPrGv4sdvEF9RQMHE8AGa2bU7o3ywT-ix5adnxaY7tLHCaQWObIjxsHIjREUP-WbTfOUxgVGnA',
  active: { type: 'Boolean', value: true },
  name: { type: 'String', value: 'Updated Record' },
  value: { type: 'Integer', value: 100 }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: t4BsuG2ZgFq1W93NHcepGe_a-DIwpFNonCkRKf3Se7omjY5EEJxj5WkqBgEV5QLfR9RCtbHrMQ5Pr1Ohi4hsRQ

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
✓ Created native SWR script: github_user_native_js (Q5DHB75F-D-dmS97FXTNy7nDq6y793GZ6BmM1qDKKqtScyu-HaQSkbwqVbpWHAwR9A-kGXwGJFHdcslkd2nmHw)

First call (cache miss - will fetch from GitHub API):
  Response time: 207ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 14ms
  Speedup: 14.8x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_js (dERZX2aScueMliv68ihQNW-wKmWvI_qh8PiGm25ay4l9OvuLn13wYYmla6zo22IUJ6YCiTwQYDyP0oF_Hksahg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_js' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_js (2Wp-Mafbmyp4yf48_I9ryO8E3bH296Jqy3v06z1qYjhSF-f2LNhGLHUY5VFv-yE0z6xP7Rx-lu8ESgO4hjMiAQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_js' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_js (z3ATGw-bdEdN4GbJe_kW6hEdz1SJ0gqXKHgdaohQCWvi_pvR8b8GS9h0GNVwQxSBMMMiWUrM3AtFe3w092bfDA)
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
✓ Created SWR script: fetch_api_user_js (Pebyn8mG_MOexJFUp93R8ePk3POogPYF_dWqtq7afbJ0oNqWLU7QOJnOlTDlKgZSyVmYOa5E0yueXSnPxnlUvg)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "value": {
          "website": "hildegard.org",
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
          "name": "Leanne Graham",
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "email": "Sincere@april.biz",
          "username": "Bret",
          "id": 1,
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
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 6ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: e1OV4akIF_yUMFU02CS48N4xX_kkRsWC8OKzGfL_qnylfH8G89wGQJjlcETQms7IPsRA-mBI6QSwQxqKWW-uMg
Created Bob: $500 - ID: 1IZQUMZVOjYVS9omoyBHBdsx-QxVbelov81SZwXbUxUOcqzzya0ybRB3t3FdrOt-6KW08N4sryJca5TfocfjKA

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 4768cd4e-f3a3-4d5a-a8f0-98d577547dfc

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 59731f6c-5c40-4d69-85dc-d185cd7faec3
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: IZ6ashqF2X4Ed5cepsuBmGy66ghWSsXpicZdnBNqLASpgDdU1coorWWGYMGkjDlbf0pFt_XeJwRRWP1KnpGDlA

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
Inserted: Vdm3UXvLqusN1ECKcbfpY3P2nwSSDxkIMAQNA9LbAXbmnHn8F9Z5k2-Y7O7PVJPYOJd1jioGIc_F9JwZoFT5hA

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
✓ Created session: -NKOl2jyh_bSKHzclmHE1qHD2mW39pGpxIs6pzc42UpG8jdNk0A0f1k77831PyACssQnI0BH0KmYDRLNTYYGOQ

=== Sending Initial Message ===
✓ Message sent
  Response: It seems there are currently no active products available in the database. If you were looking for specific details about a product or if there's anything else you need, please let me know!

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
✓ Created second session: 8nmIAEuBvl3MZ2ga6cs9wDiH9-Lt73UPAMuab3OlVB_feqbh_jI7KTOo_62LHb2pxuALcwtL4F1iUdrJPtOD8Q
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
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: kYP1pDtSGKqcDF8jaFLNfsgyfWfH8fc0yJc2055X-6e81m7HhcFoDyIew54XwDeAVIHTf58psnhQg481Q3XucA

=== Sending Chat Message ===
Message ID: tSArvan9t5COeYs-pWQ4T3DzV5-yrUHoDx4wuy8VPeEOq4-yYyYqvMesCZz9YySmNsElMhlop4qYH32yopw6EQ

=== AI Response ===
Here are the products available along with their prices:

1. **ekoDB Pro**
   - Price: $299
   - Description: Enterprise edition product with advanced features

2. **ekoDB**
   - Price: $99
   - Description: A high-performance database product with AI capabilities

3. **ekoDB Cloud**
   - Price: $499
   - Description: Fully managed cloud database service product

If you need more information about any specific product, feel free to ask!

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Enterprise edition product with advanced features',
    id: '8i2l7aBZY01JlLfYlqInxz--ab5mjEHbHPyTBao-Fvmntq6CvNWVGBNjfaeRuPV6yfT0052tjKyNA4v1J4g9qw',
    price: 299,
    name: 'ekoDB Pro'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'NJcru8_gOXbWSvdvWnAG1eez7ZnZqQfhZDVwWJs5XxD1nyqitFoLWIseRm_tr_A0qz8aT_W50RygUUvK8CGtFQ',
    name: 'ekoDB',
    price: 99,
    description: 'A high-performance database product with AI capabilities'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Fully managed cloud database service product',
    price: 499,
    id: 'MjJ3QT7wxmv0LSCoj03ChR51Ykj_ZhquJ1ZRx5wUW6xngr266I7EImb2BwqPiX95_Shhxln0vJ2o0ziG2ICA1A',
    name: 'ekoDB Cloud'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 6024ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 110
Total tokens: 3523

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Message Stream (SSE) Example (TypeScript) ===

Created session: _W8uLBnKKl4FSapuSEKXVEjQtNlv7chSv9Geup_Imi-5S1jeviukqbx8o6fjmjA3L7KzTP85Z18-H2RIJqGgCQ

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}**ekoDB** is a curated knowledgebase dedicated to the field of **cancer ecosystems**. It integrates and organizes large-scale multi-omic datasets, computational tools, and literature, focusing on the complex cellular and molecular interactions within tumor microenvironments (TMEs). The goal of ekoDB is to facilitate cancer research by providing researchers with user-friendly access to annotated data and analytical resources relating to:

- Immune landscapes and immune cell infiltration in tumors
- Cell-cell communication within the TME
- Spatial organization and phenotypic diversity of tumor and immune cells
- Tumor heterogeneity and ecosystem dynamics

**Key features of ekoDB include**:
- Aggregated and harmonized single-cell, spatial transcriptomics, and bulk sequencing datasets from various cancer studies.
- Visualization tools for exploring the composition and dynamics of cancer ecosystems.
- Downloadable data and analysis results for further research.

ekoDB is designed to support the research community's efforts in unraveling the complexity of tumor ecology, improving biomarker discovery, and developing new therapeutic strategies.

**References:**
- [ekoDB official website](https://ekodb.dp.tech)
- Zhu, X. et al., *ekoDB: a comprehensive molecular ecology knowledgebase for cancer ecosystem research*, Nucleic Acids Research, 2024. [Link](https://academic.oup.com/nar/article/52/D1/D952/7374524)

--- Stream complete ---
Message ID: iYjHofZ1YX8NtxH3R0-ymEQHLb4d568EX-i-XExKOqcV1ZsiKCjB4XWUkFvuSIs-JeUiBy6unXFrpQfmDu5R7g
Execution time: 4465ms
Context window: 1000000 tokens

✓ Chat message stream example completed
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
    - gpt-image-2.5-flare
    - gpt-image-2.5-sunburst
    - gpt-image-2.5-flare-2026-09-08
    - gpt-image-2.5-sunburst-2026-09-08
    - gpt-live-1
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
  openai: ok 132 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get Specific Provider Models ===
OpenAI models: text-embedding-ada-002, whisper-1, gpt-3.5-turbo, tts-1, gpt-3.5-turbo-16k, gpt-4-0613, gpt-4, davinci-002, babbage-002, gpt-3.5-turbo-instruct, gpt-3.5-turbo-instruct-0914, gpt-3.5-turbo-1106, tts-1-hd, tts-1-1106, tts-1-hd-1106, text-embedding-3-small, text-embedding-3-large, gpt-3.5-turbo-0125, gpt-4-turbo, gpt-4-turbo-2024-04-09, gpt-4o, gpt-4o-2024-05-13, gpt-4o-mini-2024-07-18, gpt-4o-mini, gpt-4o-2024-08-06, omni-moderation-latest, omni-moderation-2024-09-26, o1-2024-12-17, o1, o3-mini, o3-mini-2025-01-31, gpt-4o-2024-11-20, gpt-4o-mini-search-preview-2025-03-11, gpt-4o-mini-search-preview, gpt-4o-transcribe, gpt-4o-mini-transcribe, o1-pro-2025-03-19, o1-pro, gpt-4o-mini-tts, o3-2025-04-16, o4-mini-2025-04-16, o3, o4-mini, gpt-4.1-2025-04-14, gpt-4.1, gpt-4.1-mini-2025-04-14, gpt-4.1-mini, gpt-4.1-nano-2025-04-14, gpt-4.1-nano, gpt-image-1, o4-mini-deep-research, gpt-4o-transcribe-diarize, o4-mini-deep-research-2025-06-26, gpt-5-chat-latest, gpt-5-2025-08-07, gpt-5, gpt-5-mini-2025-08-07, gpt-5-mini, gpt-5-nano-2025-08-07, gpt-5-nano, gpt-audio-2025-08-28, gpt-realtime, gpt-realtime-2025-08-28, gpt-audio, gpt-5-codex, gpt-image-1-mini, gpt-5-pro-2025-10-06, gpt-5-pro, gpt-audio-mini, gpt-audio-mini-2025-10-06, gpt-5-search-api, gpt-realtime-mini, sora-2, sora-2-pro, gpt-5-search-api-2025-10-14, gpt-5.1-chat-latest, gpt-5.1-2025-11-13, gpt-5.1, gpt-5.1-codex, gpt-5.1-codex-mini, gpt-5.1-codex-max, gpt-image-1.5, gpt-5.2-2025-12-11, gpt-5.2, gpt-5.2-pro-2025-12-11, gpt-5.2-pro, gpt-5.2-chat-latest, gpt-4o-mini-transcribe-2025-12-15, gpt-4o-mini-transcribe-2025-03-20, gpt-4o-mini-tts-2025-03-20, gpt-4o-mini-tts-2025-12-15, gpt-realtime-mini-2025-12-15, gpt-audio-mini-2025-12-15, chatgpt-image-latest, gpt-5.2-codex, gpt-5.3-codex, gpt-realtime-1.5, gpt-audio-1.5, gpt-4o-search-preview, gpt-4o-search-preview-2025-03-11, gpt-5.3-chat-latest, gpt-5.4-2026-03-05, gpt-5.4-pro, gpt-5.4-pro-2026-03-05, gpt-5.4, gpt-5.4-nano-2026-03-17, gpt-5.4-nano, gpt-5.4-mini-2026-03-17, gpt-5.4-mini, gpt-image-2, gpt-image-2-2026-04-21, gpt-5.5, gpt-5.5-2026-04-23, gpt-5.5-pro, gpt-5.5-pro-2026-04-23, chat-latest, gpt-realtime-translate, gpt-realtime-2, gpt-realtime-whisper, gpt-5.6-sol, gpt-5.6-terra, gpt-5.6-luna, gpt-realtime-2.1, gpt-realtime-2.1-mini, gpt-transcribe, gpt-live-transcribe, gpt-6-astra, gpt-image-2.5-flare, gpt-image-2.5-sunburst, gpt-image-2.5-flare-2026-09-08, gpt-image-2.5-sunburst-2026-09-08, gpt-live-1

=== Get Anthropic Models ===
Anthropic models: claude-fable-5-1, claude-opus-5, claude-sonnet-5, claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 404: {"error":"Unknown provider 'nonexistent_provider_xyz'","error_kind":"unknown_provider"}

✓ Chat Models example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: hm5aw9utC9F2v7PS_NrYOqDbK2eZGIjDigYqh5qDKz6u1j-upqHAEMw59MVOxT2Q9Upj04O-zFJ9imVa1GK1wA

=== Sending Messages ===
✓ Message 1 sent
  Response: It appears that there are no products available in the queried database. However, I can confirm that we have a product called **ekoDB** with the following details:

- **Description:** A high-performance database product
- **Price:** $99

If you need further assistance or want more information, feel free to ask!

✓ Message 2 sent
  Response: The price of **ekoDB** is **$99**. If you have any more questions or need further information, feel free to ask!

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 1egLKgFpPARPm4C5xVrm7Z3hQlI4eaDhmRhovvZZciai8VxNT5wPR4aSQqssZDzj2hdT5kWQSdfxdJKHk4jaIA
  Parent: hm5aw9utC9F2v7PS_NrYOqDbK2eZGIjDigYqh5qDKz6u1j-upqHAEMw59MVOxT2Q9Upj04O-zFJ9imVa1GK1wA

=== Listing Sessions ===
✓ Found 8 sessions
  Session 1: 1egLKgFpPARPm4C5xVrm7Z3hQlI4eaDhmRhovvZZciai8VxNT5wPR4aSQqssZDzj2hdT5kWQSdfxdJKHk4jaIA (Untitled)
  Session 2: hm5aw9utC9F2v7PS_NrYOqDbK2eZGIjDigYqh5qDKz6u1j-upqHAEMw59MVOxT2Q9Upj04O-zFJ9imVa1GK1wA (Untitled)
  Session 3: 8nmIAEuBvl3MZ2ga6cs9wDiH9-Lt73UPAMuab3OlVB_feqbh_jI7KTOo_62LHb2pxuALcwtL4F1iUdrJPtOD8Q (Untitled)
  Session 4: hCkoSQHcG22ihQqUV0v7VbGtTJXM0VbX73mN9kqlbZez8zmP9iT7VahYxJLfzOXBuJdqIYQzWS6H6PJTvyU3kg (Untitled)
  Session 5: vuKbaGHsQC94yLa7_2ro5Vdy1xg9A2RA5KvKSyzx1oJLcCAi4D6DZ8eKRvWcmz0zxkBszT3iMxPubOmpZYLlXw (Untitled)
  Session 6: YUYboJLJZ3yqwxN1Cb5yCHopx8VzkWngmGBfDuMzwk5ocLrAA3onhZxGs9kFbV_RmSc9V1V2HhdynIh_f9G3Ww (Untitled)
  Session 7: MAxsP25SeTa3zmDskKituh1NEf7CdZJC9f2ywIhfXYYV748h-w5M7KwY6MKOrpZK8nSLu8xnE7EilU_pKoIjUg (Untitled)
  Session 8: SflIr8vmbb7jS_W_FfaYgsH9mYpw2-RnHYV0wV1SwX1f-j6HhxF5WJTlBhQmO8kpxpJki7WWPq2VM9MJBc5OQg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 1egLKgFpPARPm4C5xVrm7Z3hQlI4eaDhmRhovvZZciai8VxNT5wPR4aSQqssZDzj2hdT5kWQSdfxdJKHk4jaIA

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: nOlF3SdXSfWt1Xn4qLCnzQNINRhwtcPYNpeC7JvOW1o6D54d2Sih45k9bnW9GRiPlI21gzG5t0no3A60cOOCag

=== List Collections ===
Total collections: 37
Sample collections: schema_employees_client_ts,schema_products_client_go,agent_function_versions__ek0_testing,chat_agent_configs__ek0_testing,chat_goal_templates__ek0_testing

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
  POST /api/functions/conc_demo_pay        { "idempotency_key": "...", "amount": 100 }
  POST /api/functions/conc_demo_rl_fail    { "user_id": 42 }
  POST /api/functions/conc_demo_rl_skip    { "user_id": 42 }
  POST /api/functions/conc_demo_lock       { "resource": "queue:drain" }

✓ Cleaned up demo functions
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'XzXPnN9tq7E4RmyD3jGTraCLVRBvzsNFGOJucKgX0PJ4nMI-1Xfx4pDr39oJFydDni1boHXr3YQTkonY0T-hnA'
}

=== Upsert Operation ===
✓ First upsert (update): {
  id: 'XzXPnN9tq7E4RmyD3jGTraCLVRBvzsNFGOJucKgX0PJ4nMI-1Xfx4pDr39oJFydDni1boHXr3YQTkonY0T-hnA',
  active: { type: 'Boolean', value: true },
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  age: { type: 'Integer', value: 29 },
  name: { type: 'String', value: 'Alice Johnson' }
}
✓ Second upsert (insert): {
  id: 'cpTn0L_dLmX_aj7LKXTMUXlsZ2s_knHGCHhQTBKrmLFGz7QCgPyhtOSOOoD1jz_IOHEyylWOBLtZNq5kVfhgOQ'
}

=== Find One Operation ===
✓ Found user by email: {
  id: 'XzXPnN9tq7E4RmyD3jGTraCLVRBvzsNFGOJucKgX0PJ4nMI-1Xfx4pDr39oJFydDni1boHXr3YQTkonY0T-hnA',
  name: { type: 'String', value: 'Alice Johnson' },
  age: { type: 'Integer', value: 29 },
  active: { type: 'Boolean', value: true },
  email: { value: 'alice.j@newdomain.com', type: 'String' }
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
✓ Inserted document: FYkvoG21Ko59MH_SuncSj068k1FBIPf4bEWQbvV7YowV32OhGrUCeF4MIgzsATQgwzZ-hnAa1CELeTkPl75T0w

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: diOffO3UE6A0lwMZtDzMCVY3T1pzUGrDTrCtzVy3QG2y2KB_zpiR7CTwAGiIImIZ6eBE9s3Zay_KbvOz67nrjg

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
ℹ️  Function 'cache_api_call' already existed — updated instead
✓ Edge cache script updated: KGn5p2Nthgt1SmGcncaj0NQoxs8yMjFgT0pLDSua-8IZSkDxxui2PSux4X3CojN7N9BjFUyKA5dA92LRD6n_zA

Call 1: Cache miss (fetches from API)
Response time: 6ms
Result: {
  "records": [
    {
      "value": {
        "value": {
          "phone": "1-770-736-8031 x56442",
          "website": "hildegard.org",
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "id": 1,
          "name": "Leanne Graham",
          "username": "Bret",
          "address": {
            "suite": "Apt. 556",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "zipcode": "92998-3874",
            "city": "Gwenborough",
            "street": "Kulas Light"
          },
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

Call 2: Cache hit (served from ekoDB)
Response time: 4ms (1.5x faster!)
Result: {
  "records": [
    {
      "value": {
        "value": {
          "phone": "1-770-736-8031 x56442",
          "website": "hildegard.org",
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "id": 1,
          "name": "Leanne Graham",
          "username": "Bret",
          "address": {
            "suite": "Apt. 556",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "zipcode": "92998-3874",
            "city": "Gwenborough",
            "street": "Kulas Light"
          },
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
   ⏱️  Duration: 6ms
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
   ⏱️  Duration: 4ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "company": {
        "name": "Romaguera-Crona",
        "bs": "harness real-time e-markets",
        "catchPhrase": "Multi-layered client-server neural-net"
      }...
   🚀 Cache speedup: 1.5x faster!

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
client_function_contract: ok
🚀 ekoDB Functions Example (TypeScript)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: uow8mTDdXdxqpArVGPwwCawr1omf79ydyKb3pofRcfJhX7iPDRA-TRUu4SJVvG73X520OKslCIHY6W9VszIC_Q
📊 Found 5 active users

📝 Example 2: Parameterized Function

✅ Function saved: mE4YtxMMeYXm8IbKB-epC7m3VOSY6OHNkJ9Fi1j5K7xl4Q4Lh85vuEC-3K-X5mnSnU_-Ofe9Is_FDoC7Mm8CEw
📊 Found 3 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: 58rzK48Xrb_ee7HXHc44RGUtV4-DjhyzHLaIz7wP7UuHGj5dPzBaUzi6gwCv04lQV7vtFlX_vWxGkqlYSVDMKw
📊 Statistics: 2 groups
   {"status":{"value":"inactive","type":"String"},"avg_score":{"value":50,"type":"Float"},"count":{"value":5,"type":"Integer"}}
   {"status":{"type":"String","value":"active"},"count":{"value":5,"type":"Integer"},"avg_score":{"value":60,"type":"Float"}}

📝 Example 4: UserFunction Management

📋 Total scripts: 15
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
   {"avg_price":{"type":"Float","value":367},"count":{"value":5,"type":"Integer"},"category":{"value":"Electronics","type":"String"}}
   {"avg_price":{"type":"Float","value":365.6666666666667},"count":{"value":3,"type":"Integer"},"category":{"type":"String","value":"Furniture"}}
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

1. **High-Dimensional Data Handling**: Efficiently manage and search vast amounts of high-dimensional data, such as embeddings from machine learning models.

2. **Fast Similarity Search**: Enable rapid nearest neighbor search, which is crucial for applications like recommendation systems and image/video retrieval.

3. **Scalability**: Designed to handle large-scale datasets with horizontal scalability, allowing for growth without performance degradation.

4. **Flexibility in Data Types**: Support various data types, including images, text, and audio, by representing them as vectors.

5. **Real-Time Processing**: Facilitate near-real-time analytics, which is valuable for dynamic applications that require immediate results.

6. **Integration with AI/ML**: Seamless integration with machine learning workflows for tasks like clustering, classification, and anomaly detection.

7. **Enhanced Performance**: Optimized for performance, reducing latency in queries compared to traditional databases when dealing with unstructured data.

8. **Support for Advanced Algorithms**: Enable the implementation of complex algorithms for tasks such as semantic search and natural language processing.
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
🚀 ekoDB TypeScript Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: Uli0Kih7kT2kCQhwmKNEWrENEkLXl4DcNnN29LVwWoA1Zces7YYEH3n_6Ox0tCQilHMwtzxI4RqTxhxY3IO9sw
📊 Found 2 product groups
   {"category":{"type":"String","value":"Electronics"},"count":{"type":"Integer","value":3},"avg_price":{"type":"Float","value":575.6666666666666}}
   {"count":{"type":"Integer","value":2},"avg_price":{"type":"Float","value":474},"category":{"value":"Furniture","type":"String"}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"count":{"type":"Integer","value":3},"category":{"type":"String","value":"Electronics"}}
   {"category":{"type":"String","value":"Furniture"},"count":{"type":"Integer","value":2}}
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
   {"role":{"value":"user","type":"String"},"count":{"type":"Integer","value":7},"avg_score":{"type":"Float","value":70}}
   {"role":{"value":"admin","type":"String"},"count":{"type":"Integer","value":3},"avg_score":{"type":"Float","value":20}}
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

✅ Inserted order: 1RQT20Gfm8tVO9Smx-sR0h335fFhnFkY29PH_X-VPb4729z7FVUSpFSmTkNeqK07D7QnLPq9ycG_n2H1lwDagw
✅ Inserted 2 products with wrapped types

📝 Example 2: UserFunction with Wrapped Type Parameters

✅ Function saved: q-rBmMyzrwQJdiUD0WmBuZolg5eRJ7Or8D91vzoZMRz9LdjHJBvld9gvwTHisfhgfkgXEGDlC16o_6xpepeFoQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"type":"Object","value":{"role":"admin","userId":"user_abc"}}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: LLaoMfSW-UdNREUsRHs-b8yiBZW-y2SlmzRXwNIg7cMTxxXiXGORGq3c5CWFdeIbGrtDgYMVmfaw73YGZL1HcQ
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: RKfuuzYpeptF4btu-6d_vsFJHigB_f_-ksqB1qxKA50eRyWjrFNRcLhrL3ewCH8UMLkIMToXPrZSBtzp0XjihQ
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
   1. Vector Databases Explained (Database)
   2. Natural Language Processing (AI)
   3. Database Design Principles (Database)
   4. Introduction to Machine Learning (AI)
   5. Getting Started with ekoDB (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"count":{"type":"Integer","value":3},"category":{"value":"Database","type":"String"}}
   {"count":{"value":2,"type":"Integer"},"category":{"type":"String","value":"AI"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Goal Template CRUD Example (TypeScript) ===

--- Creating goal template ---
Created template: Data Migration (id: 0z6N03TZl0aBV61eKixki01R-OZ66ZVwc2GSNOLjmOA5uqKN7L_q7TH2_enT-YNNSFlL2TsV8S1Zv8xiL2AWgg)

--- Listing templates ---
Templates: {
  count: 1,
  items: [
    {
      description: [Object],
      id: '0z6N03TZl0aBV61eKixki01R-OZ66ZVwc2GSNOLjmOA5uqKN7L_q7TH2_enT-YNNSFlL2TsV8S1Zv8xiL2AWgg',
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
Created goal: Deploy v2.0 (id: P4a5f0CnS_RKc4TyqfkljHHjMt5Ij2muAfrDNKRvRGH8L2nLNjcpPJxcJH9NQL2TPIZ9Dp0Twr3BVRwGRKlLcw)

--- Listing goals ---
Goals: {
  "count": 1,
  "goals": [
    {
      "created_at": "2026-09-15T18:02:27.754712+00:00",
      "description": "Ship version 2.0 to production",
      "id": "P4a5f0CnS_RKc4TyqfkljHHjMt5Ij2muAfrDNKRvRGH8L2nLNjcpPJxcJH9NQL2TPIZ9Dp0Twr3BVRwGRKlLcw",
      "status": "pending",
      "steps": "[{\"description\":\"Run test suite\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]",
      "title": "Deploy v2.0",
      "updated_at": "2026-09-15T18:02:27.754712+00:00"
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
        "value": "2026-09-15T18:02:27.754712+00:00"
      },
      "description": {
        "type": "String",
        "value": "Ship version 2.0 with hot-fix patches"
      },
      "id": "P4a5f0CnS_RKc4TyqfkljHHjMt5Ij2muAfrDNKRvRGH8L2nLNjcpPJxcJH9NQL2TPIZ9Dp0Twr3BVRwGRKlLcw",
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
        "value": "2026-09-15T18:02:27.793574+00:00"
      }
    }
  ]
}

--- Goal step: start step 0 ---
Step 0 started on goal P4a5f0CnS_RKc4TyqfkljHHjMt5Ij2muAfrDNKRvRGH8L2nLNjcpPJxcJH9NQL2TPIZ9Dp0Twr3BVRwGRKlLcw
--- Goal step: complete step 0 ---
Step 0 completed on goal P4a5f0CnS_RKc4TyqfkljHHjMt5Ij2muAfrDNKRvRGH8L2nLNjcpPJxcJH9NQL2TPIZ9Dp0Twr3BVRwGRKlLcw
--- Goal step: fail step 1 ---
Step 1 failed on goal P4a5f0CnS_RKc4TyqfkljHHjMt5Ij2muAfrDNKRvRGH8L2nLNjcpPJxcJH9NQL2TPIZ9Dp0Twr3BVRwGRKlLcw

--- Completing goal ---
Goal status: pending_review
--- Approving goal ---
Goal status after approve: in_progress

--- Creating goal to reject ---
--- Rejecting goal ---
Goal status after reject: failed


--- Creating task ---
Created task: Hourly Health Check (id: 2nXjDzbkhuqALahCrpOM2MUJgmMOvTfvhKky6nrCRKN4dkFnq9gB3lbryZu_8ltt8wucBX8MSmSdkIK4z_FDVg)

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
      "id": "2nXjDzbkhuqALahCrpOM2MUJgmMOvTfvhKky6nrCRKN4dkFnq9gB3lbryZu_8ltt8wucBX8MSmSdkIK4z_FDVg",
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
Created agent: SupportBot (id: cEUR6saBaw1-7czwBdmZk_TPbivSgnx6UfPRPZK-nNmbkY2hWd6pRQ6JGDtXr3EKh5DEZ3LRU0hzddCpUEgDew)

--- Listing agents ---
Agents: {
  "count": 1,
  "items": [
    {
      "deployment_id": {
        "type": "String",
        "value": "deploy_prod_1"
      },
      "id": "cEUR6saBaw1-7czwBdmZk_TPbivSgnx6UfPRPZK-nNmbkY2hWd6pRQ6JGDtXr3EKh5DEZ3LRU0hzddCpUEgDew",
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
By name: SupportBot (id: cEUR6saBaw1-7czwBdmZk_TPbivSgnx6UfPRPZK-nNmbkY2hWd6pRQ6JGDtXr3EKh5DEZ3LRU0hzddCpUEgDew)

--- Updating agent ---
Updated agent: SupportBot

--- Getting agents by deployment ---
Agents in deployment: {
  "count": 0,
  "items": []
}

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

=== Cleanup ===
✅ Deleted test collections

✅ Join operations examples completed!
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
=== ekoDB KV Document Linking Example (TypeScript) ===

--- Setup: creating KV key and documents ---
Set KV key: session:admin
Inserted document 1: -Hxy_4IWvmjoEZIZPoZ4ZA8DksKby--3yyQydrD7YkVOT_2SdN93coIbOdjqUsppuiPDgcg8o29oSiqNtbMEZQ
Inserted document 2: doHlyt5UcRbAirtrkgnXleT-HFrepHFmWOpyYMxGHN2F_0nb3V4SgL7Rof6Wuc7BslFrU9RbLqQOLQxghK31lw

--- Linking documents to KV key ---
Linked doc -Hxy_4IWvmjoEZIZPoZ4ZA8DksKby--3yyQydrD7YkVOT_2SdN93coIbOdjqUsppuiPDgcg8o29oSiqNtbMEZQ: null
Linked doc doHlyt5UcRbAirtrkgnXleT-HFrepHFmWOpyYMxGHN2F_0nb3V4SgL7Rof6Wuc7BslFrU9RbLqQOLQxghK31lw: null

--- Getting links for KV key ---
Links: [
  {
    "collection": "kv_links_example_ts",
    "document_id": "-Hxy_4IWvmjoEZIZPoZ4ZA8DksKby--3yyQydrD7YkVOT_2SdN93coIbOdjqUsppuiPDgcg8o29oSiqNtbMEZQ",
    "field_path": null,
    "created_at": "2026-09-15T18:02:30.575586Z",
    "last_accessed": "2026-09-15T18:02:30.594565Z",
    "metadata": {}
  },
  {
    "collection": "kv_links_example_ts",
    "document_id": "doHlyt5UcRbAirtrkgnXleT-HFrepHFmWOpyYMxGHN2F_0nb3V4SgL7Rof6Wuc7BslFrU9RbLqQOLQxghK31lw",
    "field_path": null,
    "created_at": "2026-09-15T18:02:30.590204Z",
    "last_accessed": "2026-09-15T18:02:30.594565Z",
    "metadata": {}
  }
]

--- Unlinking document ---
Unlinked doc doHlyt5UcRbAirtrkgnXleT-HFrepHFmWOpyYMxGHN2F_0nb3V4SgL7Rof6Wuc7BslFrU9RbLqQOLQxghK31lw: null

--- Verifying remaining links ---
Remaining links: [
  {
    "collection": "kv_links_example_ts",
    "document_id": "-Hxy_4IWvmjoEZIZPoZ4ZA8DksKby--3yyQydrD7YkVOT_2SdN93coIbOdjqUsppuiPDgcg8o29oSiqNtbMEZQ",
    "field_path": null,
    "created_at": "2026-09-15T18:02:30.575586Z",
    "last_accessed": "2026-09-15T18:02:30.605193Z",
    "metadata": {}
  }
]

--- Cleanup ---
Cleanup complete

=== All KV linking operations completed ===
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
  cache:product:2: { price: 39.99, name: 'Product 2' }
  cache:product:3: { price: 49.99, name: 'Product 3' }

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
Blocking response: Hello! I hope you're having a great day.

=== Done ===
=== ekoDB Schedule Management Example (TypeScript) ===

--- Creating schedule ---
Created schedule: Nightly Database Backup (id: 23a55926-be7e-4fde-aea4-e3fd9ef9d7d7, cron: 0 0 2 * * *)

--- Listing schedules ---
Schedules: {
  "count": 1,
  "schedules": [
    {
      "created_at": "2026-09-15T18:02:42.477409Z",
      "cron_expression": "0 0 2 * * *",
      "description": null,
      "enabled": true,
      "function_label": "schedule_noop_typescript_24355",
      "id": "23a55926-be7e-4fde-aea4-e3fd9ef9d7d7",
      "last_execution": null,
      "name": "Nightly Database Backup",
      "next_execution": "2026-09-16T02:00:00Z",
      "parameters": {},
      "stats": {
        "avg_execution_time_ms": 0,
        "failed_executions": 0,
        "last_error": null,
        "successful_executions": 0,
        "total_executions": 0
      },
      "timezone": "UTC",
      "updated_at": "2026-09-15T18:02:42.477409Z"
    }
  ]
}

--- Getting schedule ---
Fetched: Nightly Database Backup (cron: 0 0 2 * * *)

--- Updating schedule ---
Updated: Nightly Full Backup (new cron: 0 0 3 * * *)

--- Triggering schedule ---
Trigger response: {
  "schedule_id": "23a55926-be7e-4fde-aea4-e3fd9ef9d7d7",
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
  4. Score: 13.200, Matched: title, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400, Matched: bio, title
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
  1. Score: 0.774, Matched:
  2. Score: 0.772, Matched:
  3. Score: 0.754, Matched:

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.501, Matched: content, title
  2. Score: 0.910, Matched: content, title
  3. Score: 0.309, Matched:

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: bio, skills

9. Vector search with a metadata pre-filter (category = ml):
Found 2 documents in category "ml" (NLP excluded)
  1. Deep Learning Fundamentals (category: ml)
  2. Introduction to Machine Learning (category: ml)

=== Cleanup ===
✅ Deleted test collections

✅ Search examples completed!
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'PNb5NiQaihw5YEsEQvgXEp66q5EnkRNIjIBe61RuRJQOhI-CM08uNJ8BipBQMCv1N7u9eCvjIlSCDd0lz5IfyA'
}

=== Find by ID ===
Found: {
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  created_at: { value: '2026-09-15T18:02:44.118+00:00', type: 'DateTime' },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  value: { type: 'Integer', value: 42 },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  name: { type: 'String', value: 'Test Record' },
  metadata: { value: { key: 'value', nested: [Object] }, type: 'Object' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  price: { value: 99.99, type: 'Float' },
  active: { value: true, type: 'Boolean' },
  id: 'PNb5NiQaihw5YEsEQvgXEp66q5EnkRNIjIBe61RuRJQOhI-CM08uNJ8BipBQMCv1N7u9eCvjIlSCDd0lz5IfyA',
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-09-15T18:02:44.118Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { key: 'value', nested: { deep: true } }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  categories: [ 'electronics', 'computers' ],
  created_at: '2026-09-15T18:02:44.118+00:00',
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  value: 42,
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  name: 'Test Record',
  metadata: { key: 'value', nested: { deep: true } },
  data: 'aGVsbG8gd29ybGQ=',
  price: 99.99,
  active: true,
  id: 'PNb5NiQaihw5YEsEQvgXEp66q5EnkRNIjIBe61RuRJQOhI-CM08uNJ8BipBQMCv1N7u9eCvjIlSCDd0lz5IfyA',
  tags: [ 'tag1', 'tag2', 'tag3' ]
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  value: { value: 100, type: 'Integer' },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  price: { value: 99.99, type: 'Float' },
  id: 'PNb5NiQaihw5YEsEQvgXEp66q5EnkRNIjIBe61RuRJQOhI-CM08uNJ8BipBQMCv1N7u9eCvjIlSCDd0lz5IfyA',
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  created_at: { value: '2026-09-15T18:02:44.118+00:00', type: 'DateTime' },
  name: { type: 'String', value: 'Updated Record' },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  active: { value: true, type: 'Boolean' },
  metadata: { type: 'Object', value: { key: 'value', nested: [Object] } },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: MaN-CtgMKxFWHZBpOJMCd9LxkfYIR-YVoLHya-ovllpr4aPKYTJy88i2o4rL6aKlZsO_kgILkiQcN8RGTlOLhA

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
✓ Created native SWR script: github_user_native_ts (LslJm_UHmfQOtd0JrTFARkPC43uX6x0DMcHlxewviYcv2bwAGBPtslCQ1jJyOTU5rLarJqed9PLDOaaHGi-eQw)

First call (cache miss - will fetch from GitHub API):
  Response time: 164ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 9ms
  Speedup: 18.2x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_ts (TDNOxApZx_oKRuTckyVGOjz1cz-zApEED7VSoaObHzK4F7ZlEkjBz7cPqJWhn-T1B7GzqLvWGzF867kxma1v9g)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_ts' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_ts (Ws6-Ub0UvvsWH_I6iVTBFc-7hEu5NxPkXWRzb8j1t99zwDvgIE5dc_Jc3LcryDONbHDF3MC3payQGztZBJcwmg)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_ts' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_ts (HLQUphEszTPSgZceyyqEFQLTdoR35BudMELYJOqKtPCvJmWSCuO8iMO-OP0C0mSmCepMEHyk61UfqWx4JSgu5g)
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
Function 'fetch_api_user' already existed — updated instead
✓ Created SWR script: fetch_api_user (YpdpS_Zhn0VlrPNEiJiImqRS1dZGpJK573hvdEVDVwpHHnxDd0VDq4wKUZm3fy6_xdxP_5VCzkfLiKyBgDCATw)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "website": "hildegard.org",
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
          "name": "Leanne Graham",
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "email": "Sincere@april.biz",
          "username": "Bret",
          "id": 1,
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
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 11ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "value": {
          "website": "hildegard.org",
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
          "name": "Leanne Graham",
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "email": "Sincere@april.biz",
          "username": "Bret",
          "id": 1,
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
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
Function 'fetch_product_with_reviews' already existed — updated instead
✓ Created enrichment script: fetch_product_with_reviews (nTIKJdn9imY6WSIgQJDxYhjBSivzUkUw-X262i-BTncmU_MuenQg9QpB_aGTjo36ThoHhSz3ttSCjvbhOge_FQ)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "weight": 4,
          "discountPercentage": 10.48,
          "dimensions": {
            "height": 13.08,
            "width": 15.14,
            "depth": 22.99
          },
          "meta": {
            "updatedAt": "2026-05-23T11:27:41.868Z",
            "barcode": "5784719087687",
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
            "createdAt": "2025-10-09T14:47:01.588Z"
          },
          "minimumOrderQuantity": 48,
          "shippingInformation": "Ships in 3-5 business days",
          "stock": 99,
          "id": 1,
          "brand": "Essence",
          "tags": [
            "beauty",
            "mascara"
          ],
          "title": "Essence Mascara Lash Princess",
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "rating": 2.56,
          "availabilityStatus": "In Stock",
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "price": 9.99,
          "category": "beauty",
          "reviews": [
            {
              "comment": "Would not recommend!",
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerName": "Eleanor Collins",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "rating": 3
            },
            {
              "reviewerEmail": "lucas.gordon@x.dummyjson.com",
              "date": "2025-04-30T09:41:02.053Z",
              "comment": "Very satisfied!",
              "reviewerName": "Lucas Gordon",
              "rating": 4
            },
            {
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "comment": "Highly impressed!",
              "rating": 5,
              "reviewerName": "Eleanor Collins",
              "date": "2025-04-30T09:41:02.053Z"
            }
          ],
          "sku": "BEA-ESS-ESS-001",
          "returnPolicy": "No return policy",
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

✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: XoF3jmpJn3IIqK1TcDEFtFWQNd6UWSeCZ3OxwSpz8lohn5IL-E0e2NEQiHa5SPrbafI8wi2NsRuHxO15nE9pxQ
Created Bob: $500 - ID: jvfuWwW-ZzapctLHXEombm_Z_F7AAADGWqsU4Ms29mnBNxLQE4h-ys6g28RbqOD0DUhKVroku6GL1xxMwPHoow

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): a8ddf2b4-16d6-4ba3-805f-ed51b423b496

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: ba3b29a2-5ec4-4e44-bff6-097b7764ca4b
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✓ Client created

=== Create User Function ===
Created user function with ID: gb4JooUhfekWqzW4h92Is8WqTM3Kz5Pk12CUE19cRczyZkHrLmg3XEiYtBE60LnTU6CFMOdpArxye3m_LqksIA

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 13 user functions:
  - fetch_slim_user: Validate and slim down user
  - cache_api_call_js: Cache External API Call
  - get_verified_user: Get verified and validated user
  - cache_api_call: Cache External API Call
  - cache_api_call_py: Cache External API Call
  - fetch_api_user: Fetch User with Cache
  - fetch_user: Fetch user by code
  - get_user_wrapper: Wrapper that calls fetch_user
  - fetch_product_with_reviews: Fetch Product with Reviews (Multi-API)
  - swr_user: SWR pattern for user data (KV-based)
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_active_users_ts: Get Active Users
  - validate_user: Check if user exists

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
=== WebSocket Chat Streaming Example (TypeScript) ===

Created chat session: rglSiBmBMU_HxuLekpGiQo_xtbNA5ZVNpgx6wSVeVgdVb25tlvZWsXfVvP6NpUKnhG5Wrce1-lKcjAKKzza7bg

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: 0PIvbkMF-1R1wy6G0RhYD76xF8KySpLKQAONIaTaFz0B5thQPRYsZ6KD0Cgi2KwgaMtPIH4qABJBLVcAp3kO3Q
Execution time: 2337ms
Token usage: {"completion_tokens":8,"prompt_tokens":15,"total_tokens":23}

Full response: {"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris....
=== WebSocket Subscription Example ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_ts' ===
✓ Subscribed (subscription_id: sub_5b24034195d94f40a653ce3d0f541012)

=== Listening for mutation notifications ===

Inserting a record...

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_ts
     Record IDs: 4J6iUOIBJ7i9BTR0V0Rd_BNqxkXk_E3g5Yx_sLhLFsAZPW9vuhz_YEK7lRm3msey3FwPWjb8lrYH6_3yrL2McQ
     Timestamp:  2026-09-15T18:02:50.167327+00:00
     Records:    {"active":{"type":"Boolean","value":true},"id":"4J6iUOIBJ7i9BTR0V0Rd_BNqxkXk_E3g5Yx_sLhLFsAZPW9vuhz_...
✓ Inserted record: 4J6iUOIBJ7i9BTR0V0Rd_BNqxkXk_E3g5Yx_sLhLFsAZPW9vuhz_YEK7lRm3msey3FwPWjb8lrYH6_3yrL2McQ

Inserting another record...

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_ts
     Record IDs: 7AEK2CX9m2Tix_br0R97krh3UgB0gGPMIQSPGxdEkqTysuyZ2T8AQkjtxbpCxTbuQHQQ-bjTApBxmXTGwi1aOA
     Timestamp:  2026-09-15T18:02:51.182832+00:00
     Records:    {"active":{"type":"Boolean","value":true},"id":"7AEK2CX9m2Tix_br0R97krh3UgB0gGPMIQSPGxdEkqTysuyZ2T8A...

=== Unsubscribing ===
✓ Unsubscribed: {"collection":"ws_subscribe_example_ts","found":true,"unsubscribed":true}

✓ WebSocket subscription example completed successfully
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: 9AXlTOQRmHl75WM0EUZbMmk8i6SDBp7ZJGtgISRjnP6Kh4GcAyh0EiaJtIHcD-z-D4k2L1kFksZjNUg_lZlh7g

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
   Inserted with ripple: {"id":"W0-pSo3f2l8efNozPwQtsrvWpHYvrnWt5lCWXsEP-clHb5dDVMVk6BDJZgCMXifs3yIFOwxanLSnWXVFl7bkLw"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"pMljFpQuuXqXhBdzcavkFJntVtrNuFOIxnx2jVEpDB_bHgHdHHecoNXWWrSId3zQg6s0lk71-2XyyFRLqgTd_Q"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"id":"W0-pSo3f2l8efNozPwQtsrvWpHYvrnWt5lCWXsEP-clHb5dDVMVk6BDJZgCMXifs3yIFOwxanLSnWXVFl7bkLw","price":{"value":150,"type":"Integer"},"name":{"type":"String","value":"Product 1"}}

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
  Fields returned: ["name","id","email"]
  First user: Bob Smith <bob@example.com>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["age","created_at","user_role","status","bio","avatar_url","email","name","id"]

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
    - Fields: ["id","avatar_url","secret_token","age","name","user_role","status","api_key","email","bio","password","created_at"]
  Projected query:
    - 3 fields per record
    - Fields: ["name","id","email"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!

🟣 Building Kotlin client library...
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
> Task :compileTestKotlin UP-TO-DATE
> Task :compileTestJava NO-SOURCE
> Task :processTestResources NO-SOURCE
> Task :testClasses UP-TO-DATE
> Task :test
> Task :runKtlintCheckOverTestSourceSet
> Task :ktlintTestSourceSetCheck
> Task :check
> Task :build

[Incubating] Problems report is available at: file://ekoDB/ekodb-client/ekodb-client-kt/build/reports/problems/problems-report.html

Deprecated Gradle features were used in this build, making it incompatible with Gradle 10.

You can use '--warning-mode all' to show the individual deprecation warnings and determine if they come from your own scripts or plugins.

For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/command_line_interface.html#sec:command_line_warnings in the Gradle documentation.

BUILD SUCCESSFUL in 13s
11 actionable tasks: 3 executed, 8 up-to-date
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
✅ Kotlin client built!
bash: warning: setlocale: LC_ALL: cannot change locale (C.UTF-8): No such file or directory
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
Inserted: 8UbsmV80IkDd1tN1BM-oZq7BZQfMOrzIleI2iS5tSVqPXAq3dz8MvRDZNoFZpO0PLjiJ8oVI0-7f2bku6SatHA

--- updateWithAction: increment views ---
After increment: EkoRecord(fields={views=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=15)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=kotlin), StringValue(value=example)])}), id=StringValue(value=8UbsmV80IkDd1tN1BM-oZq7BZQfMOrzIleI2iS5tSVqPXAq3dz8MvRDZNoFZpO0PLjiJ8oVI0-7f2bku6SatHA), name=ObjectValue(value={value=StringValue(value=Counter Record), type=StringValue(value=String)}), score=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=100.0)})})

--- updateWithAction: push to tags ---
After push: EkoRecord(fields={score=ObjectValue(value={value=FloatValue(value=100.0), type=StringValue(value=Float)}), views=ObjectValue(value={value=IntegerValue(value=15), type=StringValue(value=Integer)}), id=StringValue(value=8UbsmV80IkDd1tN1BM-oZq7BZQfMOrzIleI2iS5tSVqPXAq3dz8MvRDZNoFZpO0PLjiJ8oVI0-7f2bku6SatHA), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Counter Record)}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=kotlin), StringValue(value=example), StringValue(value=advanced)]), type=StringValue(value=Array)})})

--- updateWithActionSequence: multiple operations ---
After sequence: EkoRecord(fields={score=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=75.0)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Counter Record)}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=kotlin), StringValue(value=example), StringValue(value=advanced), StringValue(value=sequenced)]), type=StringValue(value=Array)}), views=ObjectValue(value={value=IntegerValue(value=115), type=StringValue(value=Integer)}), id=StringValue(value=8UbsmV80IkDd1tN1BM-oZq7BZQfMOrzIleI2iS5tSVqPXAq3dz8MvRDZNoFZpO0PLjiJ8oVI0-7f2bku6SatHA)})

--- Cleanup ---
Deleted collection: kotlin_advanced_crud_example

=== Example Complete ===

BUILD SUCCESSFUL in 8s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
? Inserted 5 records
  IDs: J7u8ChqppBfuKEBeYqzNDUe3ek-0pPIgxdRUR770NB0VlhyNEP_dLo2DyDTiVGdydETM0QxZJ-UcwIvFJiTBOw, KTcWmXfGA9kbbetmJ25pMJl04btfIBSXYBD0JDLhx-uygL5K3y9XbsXNM9ITpBi4Fbh5ht9qI0IV6xePTS4N5g, gyjNbyqJCizfh0OU9PdldARIQKw994Rv4hcyK7WKTarxbXau3As08sfDefS0wf6AmImyzfXkXBTLNRGZqT17mg...

=== Batch Update ===
? Updated 3 records

=== Batch Delete ===
? Deleted 2 records

=== Cleanup ===
? Deleted collection: kotlin_batch_example

=== Example Complete ===

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
? Inserted sample product

=== Creating Chat Session ===
? Created session: 4I0UdfqNcdqtoUkD2pTT0Lg80yq8iMn3Fi0yuonQzTaTnn8nqG_cBsb_jku4YXLB2uVhrZhYqtsUPYR1gqQHng

=== Sending Initial Message ===
? Message sent
  Responses: ["The available product is:\n\n- **Name:** ekoDB\n- **Description:** High-performance database product\n- **Price:** $99\n\nIf you need more information or have other questions, feel free to ask!"]

? Second message sent

=== Regenerating AI Response ===
? AI response regenerated
  New responses: ["The price of ekoDB is $99. If you have any other questions or need more information, feel free to ask!"]

=== Updating Message ===
? Updated message content

=== Toggling Forgotten Status ===
? Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
? Created second session: kyZrtql3xoKunqJ4L8bITuiX46MMo9wmXuXx7R01Cn07BtB259BtnQ-rxfIzYzOmVi6Ddi2Yn1_8ZEIjz_gfrg

=== Merging Sessions ===
? Merged sessions
  Total messages in merged session: 5

=== Deleting Message ===
? Deleted message

=== Cleanup ===
? Deleted chat session: kyZrtql3xoKunqJ4L8bITuiX46MMo9wmXuXx7R01Cn07BtB259BtnQ-rxfIzYzOmVi6Ddi2Yn1_8ZEIjz_gfrg
? Deleted chat session: 4I0UdfqNcdqtoUkD2pTT0Lg80yq8iMn3Fi0yuonQzTaTnn8nqG_cBsb_jku4YXLB2uVhrZhYqtsUPYR1gqQHng
? Deleted collection: kotlin_chat_advanced_example

? Advanced chat features example completed successfully

BUILD SUCCESSFUL in 14s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
? Inserted 3 sample documents

=== Creating Chat Session ===
? Created session: kIsbXtyCWRFJtyKoIXIXv6AmzmyH5frO9CjGaNbf74cLZX-xknROnusfswvCLzVIN3LDDNTkWsIFJEe-UAbKow

=== Sending Chat Message ===
? Chat response:
  Message ID: "ZZSaE2WgA-t6zm3XjcAAWhpkGyRwW-4pjuy4aDe-9WZKYu7_Zgeu5emvEBM_xQlLMn1zLGhbunWDbhsjKMvGQg"
  Responses: ["ekoDB is a high-performance database that integrates intelligent caching and real-time capabilities, along with AI integration. It allows users to query the database using natural language, providing AI-powered responses enriched with relevant context. \n\nKey features of ekoDB include:\n\n1. **AI Chat Integration**: Users can interact with the database through a chat feature that facilitates querying using natural language.\n   \n2. **Search Capabilities**: ekoDB supports multiple search types, including full-text search, vector search, and hybrid search, which automatically retrieves context relevant to the queries.\n\nIn summary, ekoDB offers a user-friendly interface powered by AI, along with robust search functionalities that enhance the database querying experience."]

=== Cleanup ===
? Deleted chat session
? Deleted collection: kotlin_chat_basic_example

? Basic chat example completed successfully

BUILD SUCCESSFUL in 10s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
Created session: hLMzYUbPX-WRBVW9QJs37_qky1BqyGJxS1odkg48aGebdrFOLdj3EUK9qux99C44n9-JdrBD191FnMLEdBpVLw

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}EkoDB is a high-performance, in-memory key-value database designed for fast data retrieval and manipulation. It is often used in environments requiring low-latency access to data, such as real-time analytics, gaming, or caching scenarios.

EkoDB typically emphasizes ease of use, scalability, and robust performance, allowing developers to efficiently manage data without the overhead associated with traditional disk-based databases. It may support various data structures and provide features like persistence, replication, and transactions, depending on the specific implementation.

However, as of my last knowledge update in October 2023, EkoDB does not have widespread recognition compared to other databases like Redis, Memcached, or traditional relational databases. If you are looking for more specific information or have a particular context in mind about EkoDB, feel free to provide additional details!

--- Stream complete ---
Message ID: UG9V73G6O8rkeGFmUh2-pD8lBW1y1V6hcAsubmZ-EDTHUVcWHIGwpS4nzlXrOWs8cPj6B_cUu6UYx13BV5IAJQ
Execution time: 1977ms
Context window: 128000 tokens

? Chat message stream example completed

BUILD SUCCESSFUL in 10s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
    - gpt-image-2.5-flare
    - gpt-image-2.5-sunburst
    - gpt-image-2.5-flare-2026-09-08
    - gpt-image-2.5-sunburst-2026-09-08
    - gpt-live-1
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
  openai: ok 132 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get OpenAI Models ===
OpenAI models: text-embedding-ada-002, whisper-1, gpt-3.5-turbo, tts-1, gpt-3.5-turbo-16k, gpt-4-0613, gpt-4, davinci-002, babbage-002, gpt-3.5-turbo-instruct, gpt-3.5-turbo-instruct-0914, gpt-3.5-turbo-1106, tts-1-hd, tts-1-1106, tts-1-hd-1106, text-embedding-3-small, text-embedding-3-large, gpt-3.5-turbo-0125, gpt-4-turbo, gpt-4-turbo-2024-04-09, gpt-4o, gpt-4o-2024-05-13, gpt-4o-mini-2024-07-18, gpt-4o-mini, gpt-4o-2024-08-06, omni-moderation-latest, omni-moderation-2024-09-26, o1-2024-12-17, o1, o3-mini, o3-mini-2025-01-31, gpt-4o-2024-11-20, gpt-4o-mini-search-preview-2025-03-11, gpt-4o-mini-search-preview, gpt-4o-transcribe, gpt-4o-mini-transcribe, o1-pro-2025-03-19, o1-pro, gpt-4o-mini-tts, o3-2025-04-16, o4-mini-2025-04-16, o3, o4-mini, gpt-4.1-2025-04-14, gpt-4.1, gpt-4.1-mini-2025-04-14, gpt-4.1-mini, gpt-4.1-nano-2025-04-14, gpt-4.1-nano, gpt-image-1, o4-mini-deep-research, gpt-4o-transcribe-diarize, o4-mini-deep-research-2025-06-26, gpt-5-chat-latest, gpt-5-2025-08-07, gpt-5, gpt-5-mini-2025-08-07, gpt-5-mini, gpt-5-nano-2025-08-07, gpt-5-nano, gpt-audio-2025-08-28, gpt-realtime, gpt-realtime-2025-08-28, gpt-audio, gpt-5-codex, gpt-image-1-mini, gpt-5-pro-2025-10-06, gpt-5-pro, gpt-audio-mini, gpt-audio-mini-2025-10-06, gpt-5-search-api, gpt-realtime-mini, sora-2, sora-2-pro, gpt-5-search-api-2025-10-14, gpt-5.1-chat-latest, gpt-5.1-2025-11-13, gpt-5.1, gpt-5.1-codex, gpt-5.1-codex-mini, gpt-5.1-codex-max, gpt-image-1.5, gpt-5.2-2025-12-11, gpt-5.2, gpt-5.2-pro-2025-12-11, gpt-5.2-pro, gpt-5.2-chat-latest, gpt-4o-mini-transcribe-2025-12-15, gpt-4o-mini-transcribe-2025-03-20, gpt-4o-mini-tts-2025-03-20, gpt-4o-mini-tts-2025-12-15, gpt-realtime-mini-2025-12-15, gpt-audio-mini-2025-12-15, chatgpt-image-latest, gpt-5.2-codex, gpt-5.3-codex, gpt-realtime-1.5, gpt-audio-1.5, gpt-4o-search-preview, gpt-4o-search-preview-2025-03-11, gpt-5.3-chat-latest, gpt-5.4-2026-03-05, gpt-5.4-pro, gpt-5.4-pro-2026-03-05, gpt-5.4, gpt-5.4-nano-2026-03-17, gpt-5.4-nano, gpt-5.4-mini-2026-03-17, gpt-5.4-mini, gpt-image-2, gpt-image-2-2026-04-21, gpt-5.5, gpt-5.5-2026-04-23, gpt-5.5-pro, gpt-5.5-pro-2026-04-23, chat-latest, gpt-realtime-translate, gpt-realtime-2, gpt-realtime-whisper, gpt-5.6-sol, gpt-5.6-terra, gpt-5.6-luna, gpt-realtime-2.1, gpt-realtime-2.1-mini, gpt-transcribe, gpt-live-transcribe, gpt-6-astra, gpt-image-2.5-flare, gpt-image-2.5-sunburst, gpt-image-2.5-flare-2026-09-08, gpt-image-2.5-sunburst-2026-09-08, gpt-live-1

=== Get Anthropic Models ===
Anthropic models: claude-fable-5-1, claude-opus-5, claude-sonnet-5, claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Request failed with status 404: {"error":"Unknown provider 'nonexistent_provider_xyz'","error_kind":"unknown_provider"}

=== Chat Models Example Complete ===

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
? Inserted sample product

=== Creating Chat Session ===
? Created session: kt47Tl9LW95YFmu0_P_Q2lWT2XBjMwM512abXuUtxnRyNR1YQw18hcS3g1loxGE7RjmWjdTD4HL8N5olIiAh7Q

=== Sending Messages ===
? Message 1 sent
  Responses: ["The available product is:\n\n- **Product**: ekoDB\n- **Description**: A high-performance database product with AI capabilities\n- **Price**: $99\n\nIf you need more information or have any other questions, feel free to ask!"]

? Message 2 sent
  Responses: ["The price of the product ekoDB is $99."]

=== Getting Message History ===
? Retrieved message history
  Total messages: [{"chat_id":{"type":"String","value":"kt47Tl9LW95YFmu0_P_Q2lWT2XBjMwM512abXuUtxnRyNR1YQw18hcS3g1loxGE7RjmWjdTD4HL8N5olIiAh7Q"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"J-QZq3RjgoPld842BzRy3A3h1DYLUp3pzDM-uAPlrv_KOwtxW3ct0sCZazA1TOVogwmFiYlbkIMazuPG_DBhlQ","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-09-15T18:47:31.268160+00:00"},"id":"1RkW57DsL52uIH78R8rvB03lhRW-JPIZm1Wt-pCOQirLu86lniSfdOQz2Myezk_gYElA4wbSzZWWF72M2zLqnQ","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":87,"prompt_tokens":6740,"total_tokens":6827}},"updated_at":{"type":"DateTime","value":"2026-09-15T18:47:31.268160+00:00"}},{"chat_id":{"type":"String","value":"kt47Tl9LW95YFmu0_P_Q2lWT2XBjMwM512abXuUtxnRyNR1YQw18hcS3g1loxGE7RjmWjdTD4HL8N5olIiAh7Q"},"content":{"type":"String","value":"The available product is:\n\n- **Product**: ekoDB\n- **Description**: A high-performance database product with AI capabilities\n- **Price**: $99\n\nIf you need more information or have any other questions, feel free to ask!"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"J-QZq3RjgoPld842BzRy3A3h1DYLUp3pzDM-uAPlrv_KOwtxW3ct0sCZazA1TOVogwmFiYlbkIMazuPG_DBhlQ","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-09-15T18:47:31.286+00:00"},"id":"OORWYCZwiY2JbR8igbUYNARC6bJ0Xj63uTfCF9PFNrSbafqOqUs4BAAfbfbNy7aY8j4Pk3_mS-p0N3NWdVMoaA","llm_model":{"type":"String","value":"gpt-4o-mini"},"llm_provider":{"type":"String","value":"openai"},"role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":87,"prompt_tokens":6740,"total_tokens":6827}},"tool_call_count":{"type":"Number","value":2},"tool_call_history":{"type":"Object","value":{"iterations":2,"tool_calls":[{"arguments":{"collection":"kotlin_chat_sessions_example","filter":{"content":{"field":"product","operator":"Ne","value":""},"type":"Condition"}},"id":"call_QeGb0sRTrxyy0bJnpYPdUOGe","name":"query_collection"}],"tool_results":[{"error":null,"result":{"count":1,"records":[{"description":"A high-performance database product with AI capabilities","id":"J-QZq3RjgoPld842BzRy3A3h1DYLUp3pzDM-uAPlrv_KOwtxW3ct0sCZazA1TOVogwmFiYlbkIMazuPG_DBhlQ","price":99,"product":"ekoDB"}]},"success":true,"tool_call_id":"call_QeGb0sRTrxyy0bJnpYPdUOGe","tool_name":"query_collection"}]}},"updated_at":{"type":"DateTime","value":"2026-09-15T18:47:31.286+00:00"}},{"chat_id":{"type":"String","value":"kt47Tl9LW95YFmu0_P_Q2lWT2XBjMwM512abXuUtxnRyNR1YQw18hcS3g1loxGE7RjmWjdTD4HL8N5olIiAh7Q"},"content":{"type":"String","value":"What is the price?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description","product","price"],"record":{"description":"A high-performance database product with AI capabilities","id":"J-QZq3RjgoPld842BzRy3A3h1DYLUp3pzDM-uAPlrv_KOwtxW3ct0sCZazA1TOVogwmFiYlbkIMazuPG_DBhlQ","price":99,"product":"ekoDB"},"score":2.0999999999999996}]},"created_at":{"type":"DateTime","value":"2026-09-15T18:47:32.480620+00:00"},"id":"eL1iyirvuDp5XuUcDOCqSS62mo-En1KrQpojhgbkVUU0Uu7ge4kSrcMujOMHsayhGMe0CL2qesYJ75LclseC0w","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":9,"prompt_tokens":3386,"total_tokens":3395}},"updated_at":{"type":"DateTime","value":"2026-09-15T18:47:32.480620+00:00"}},{"chat_id":{"type":"String","value":"kt47Tl9LW95YFmu0_P_Q2lWT2XBjMwM512abXuUtxnRyNR1YQw18hcS3g1loxGE7RjmWjdTD4HL8N5olIiAh7Q"},"content":{"type":"String","value":"The price of the product ekoDB is $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description","product","price"],"record":{"description":"A high-performance database product with AI capabilities","id":"J-QZq3RjgoPld842BzRy3A3h1DYLUp3pzDM-uAPlrv_KOwtxW3ct0sCZazA1TOVogwmFiYlbkIMazuPG_DBhlQ","price":99,"product":"ekoDB"},"score":2.0999999999999996}]},"created_at":{"type":"DateTime","value":"2026-09-15T18:47:32.508340+00:00"},"id":"I8wFL6UVDy5INpDj-lbTDBZEFnbJdRG2vOy_LiMuVB64-SpsDBxwMi3hKENgTYk7_uZ6FYSEQxcz15NT0muIFA","llm_model":{"type":"String","value":"gpt-4o-mini"},"llm_provider":{"type":"String","value":"openai"},"role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":9,"prompt_tokens":3386,"total_tokens":3395}},"updated_at":{"type":"DateTime","value":"2026-09-15T18:47:32.508340+00:00"}}]

=== Updating Session ===
? Updated session system prompt

=== Listing Sessions ===
? Total sessions: 7

=== Branching Session ===
? Created branched session: x1MfXtI-2NL_d5CAuzXjhHVxvb1gpFPZlcxW_P7BL4Sn-Ua7SHYi5fJyzok7LRt3fAg6WAuLgc7e0X0oYvoQ9Q

=== Cleanup ===
? Deleted chat sessions
? Deleted collection: kotlin_chat_sessions_example

? Chat session management example completed successfully

BUILD SUCCESSFUL in 14s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
? Found 36 collections
  - schema_employees_client_ts
  - schema_products_client_go
  - agent_function_versions__ek0_testing
  - chat_agent_configs__ek0_testing
  - chat_goal_templates__ek0_testing

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
? Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
? Schema: {"fields":{"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2026-09-15T18:47:42.018463Z","last_modified":"2026-09-15T18:47:42.018464Z","bypass_ripple":false,"primary_key_alias":"id"}

=== Cleanup ===
? Deleted collection: kotlin_collection_example

=== Example Complete ===

BUILD SUCCESSFUL in 8s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
Collection metadata: {"analytics":["collection_utils_test_kt",{"manifest_backed":false,"record_count":5,"resident_record_ids":5,"total_size":675}],"collection":{"bypass_ripple":false,"created_at":"2026-09-15T18:47:51.727908Z","fields":{"index":{"default":null,"enums":[],"field_type":"Integer","max":null,"min":null,"regex":null,"required":false,"unique":false},"name":{"default":null,"enums":[],"field_type":"String","max":null,"min":null,"regex":null,"required":false,"unique":false}},"last_modified":"2026-09-15T18:47:51.728757Z","primary_key_alias":"id","version":1}}

=== List Collections ===
All collections (37):
  - schema_employees_client_ts
  - schema_products_client_go
  - agent_function_versions__ek0_testing
  - chat_agent_configs__ek0_testing
  - chat_goal_templates__ek0_testing
  - schedules__ek0_testing
  - schema_users_client_py
  - schema_employees_client_py
  - schema_products_client_js
  - test_accounts
  - users
  - ws_subscribe_example_rs
  - schema_products_client_ts
  - ws_subscribe_example_go
  - chat_messages__ek0_testing
  - schema_products_client_py
  - chat_raw_completions__ek0_testing
  - chat_tasks__ek0_testing
  - schema_documents_client_ts
  - ws_subscribe_example_ts
  - chat_goals__ek0_testing
  - schema_users_client_ts
  - collection_utils_test_kt
  - schema_documents_client_go
  - schema_users_client_js
  - schema_employees_client_js
  - test_collection
  - chat_configurations__ek0_testing
  - schema_documents_client_js
  - audit__ek0_testing
  - chat_turns__ek0_testing
  - ttl_cache
  - functions__ek0_testing
  - schema_documents_client_py
  - ws_subscribe_example_py
  - schema_users_client_go
  - schema_employees_client_go

=== Check Non-Existent Collection ===
Collection 'nonexistent_collection_xyz_304132701624750' exists: false

=== Cleanup ===
Deleted collection 'collection_utils_test_kt'

=== Collection Utilities Example Complete ===

BUILD SUCCESSFUL in 9s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
? Client created
? conc_demo_pay saved
? conc_demo_rl_fail saved
? conc_demo_rl_skip saved
? conc_demo_lock saved

Invoke them like:
  POST /api/functions/conc_demo_pay        { "idempotency_key": "...", "amount": 100 }
  POST /api/functions/conc_demo_rl_fail    { "user_id": 42 }
  POST /api/functions/conc_demo_rl_skip    { "user_id": 42 }
  POST /api/functions/conc_demo_lock       { "resource": "queue:drain" }

? Cleaned up demo functions

BUILD SUCCESSFUL in 10s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
? Created record: EkoRecord(fields={id=StringValue(value=dnEhhLEAo7Yn8V9-ewQZYd3hd06vHkRCdMMhodUdnKoTpa29Ljm_uAkFNFyJXrIZQysluXYWFHwTtPr_NwItMQ)})

=== Upsert Operation ===
? First upsert (update): EkoRecord(fields={age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), email=ObjectValue(value={value=StringValue(value=alice.j@newdomain.com), type=StringValue(value=String)}), id=StringValue(value=dnEhhLEAo7Yn8V9-ewQZYd3hd06vHkRCdMMhodUdnKoTpa29Ljm_uAkFNFyJXrIZQysluXYWFHwTtPr_NwItMQ), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)})})
? Second upsert (insert): EkoRecord(fields={id=StringValue(value=0RhSR5nksqnLzx9oIancnBdAlL6AZY-_aJ64PmcOO4nq4hEz4xVKrU3T4OLl5DhL-rPdj6HNvMklV8xQ195QFg)})

=== Find One Operation ===
? Found user by email: EkoRecord(fields={age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), email=ObjectValue(value={value=StringValue(value=alice.j@newdomain.com), type=StringValue(value=String)}), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), id=StringValue(value=dnEhhLEAo7Yn8V9-ewQZYd3hd06vHkRCdMMhodUdnKoTpa29Ljm_uAkFNFyJXrIZQysluXYWFHwTtPr_NwItMQ)})
? User not found (as expected)

=== Exists Check ===
? Record exists: true
? Fake record exists: false (should be false)

=== Pagination ===
? Inserted 25 records for pagination
? Page 1: 10 records (expected 10)
? Page 2: 10 records (expected 10)
? Page 3: 7 records (expected ~7)

=== Cleanup ===
? Deleted collection

? All convenience methods demonstrated successfully!

BUILD SUCCESSFUL in 8s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
? Client created
? crypto_demo_hmac saved
? crypto_demo_aes saved
? crypto_demo_uuid saved
? crypto_demo_totp saved
? crypto_demo_encoding saved

Invoke them with:
  POST /api/functions/crypto_demo_hmac     { "payload": "hi" }
  POST /api/functions/crypto_demo_aes      { "plaintext": "secret" }
  POST /api/functions/crypto_demo_uuid
  POST /api/functions/crypto_demo_totp
  POST /api/functions/crypto_demo_encoding { "title": "H?llo World" }

? Cleaned up demo functions

BUILD SUCCESSFUL in 6s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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

BUILD SUCCESSFUL in 6s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
? Inserted document with 10s TTL
  Document ID: yjmuX9E4nl2YPHzvQ6_pS4IJbU8ppfn8kkzyXb6F2k8qPjiXYQWlH368eRYOk4ngTWt4-sio_Lt7dF5wo-Jpew

=== Verify Document Exists ===
? Document found: user_id, created_at, session_id, id, ttl

=== Insert with Longer TTL ===
? Inserted document with 1h TTL
  Document ID: 1J7UPQFGNoO0QgBMyQkZsNZjl7hR-30TcVkiST3xf3-V2-AleYV_Nr8dPTg2YVDoQbTbok88lZVYaqJGmSN9Ew

=== TTL Expiration ===
? Document will automatically expire after 10 seconds

=== Verify Long TTL Document ===
? Long TTL document still exists: id, value, cache_key, ttl

=== Delete Document ===
? Deleted document

=== Cleanup ===
? Deleted collection: kotlin_ttl_example

=== Example Complete ===

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
? Cache entry created

Creating edge cache lookup function...
? Edge cache function created: JLsCg7zHjtA4y3S2cCnG_b1T6kQAsVZCRQAeef6LHJ5dEBkhPFswW3BkiW6Loth6G6UHoU8q2dRHVHxKXcpydg

Call 1: Cache lookup
Found 1 cached entries
Response time: 24ms

Call 2: Cache lookup (connection warm)
Found 1 cached entries
Response time: 14ms

? Cleaning up...
? Cleanup complete

=== The Magic ===
- Your DATABASE is your EDGE
- No Redis needed
- No CDN needed
- No cache invalidation logic needed (TTL handles it)
- With ripples: All nodes auto-sync cache
- One service: Database + Cache + Edge Functions

? Example complete!

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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

? Setting up test data...

? Test data ready

? Example 1: Basic Function Composition

Building reusable functions that call each other...

? Saved reusable function: fetch_user
? Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

? Result from composed function:
   Records: 1
   Name: {"value":"User 1","type":"String"}
   Department: {"value":"engineering","type":"String"}

? Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

? Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

? Saved reusable function: fetch_and_store_user (uses KV)
? Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ??  Duration: 118ms
   ? Records: 1

Second call (cache hit - from cache):
   ??  Duration: 14ms
   ? Records: 1
   ? Cache speedup: 8.4x faster!

? Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

? Level 1 function: validate_user
? Level 2 function: fetch_slim_user (calls validate_user)
? Level 3 function: get_verified_user (calls fetch_slim_user)

? Result from 3-level nested composition:
   Records: 1
   Name: {"value":"User 1","type":"String"}
   Department: {"type":"String","value":"engineering"}

? Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - get_verified_user: Specific workflow

? Cleaning up...
? Cleanup complete

? All composition examples completed!

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
? ekoDB Functions Example (Kotlin Client)

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
? Client initialized

? Setting up test data...
? Test data ready

? Example 1: Simple Query Function

? Function saved: 8buUeI05rP5_2OgeBRt97ZyW6plRfeLVgWy7jX8xYpZyl2Yop00jRGsU_UCT8OQevxC99drW8SZkdoHak183kg
? Found 5 records
??  Execution time: 0ms

? Example 2: Parameterized Function

? Function saved
? Found 3 users (limited)
??  Execution time: 0ms

? Example 3: Aggregation Function

? Function saved
? Statistics: 2 groups
   {"avg_score":{"value":60.0,"type":"Float"},"status":{"value":"active","type":"String"},"count":{"value":5,"type":"Integer"}}
   {"avg_score":{"type":"Float","value":50.0},"count":{"value":5,"type":"Integer"},"status":{"value":"inactive","type":"String"}}
??  Execution time: 0ms

? Example 4: function Management

? Total functions: 15
? Retrieved function: Get Active Users
??  function updated
??  function deleted

??  Note: GET/UPDATE/DELETE operations require the encrypted ID
??  Only CALL can use either ID or label

? Example 5: Multi-Stage Pipeline

? Multi-stage function saved
? Pipeline executed 2 stages
??  Total execution time: 0ms
? Stage breakdown:

? Example 6: Count Users

? Count function saved
? Total user count: {"type":"Integer","value":10}
??  Execution time: 0ms

? Cleaning up...
? Deleted test functions
? Deleted collection

? All examples completed successfully!

? Key Advantages of Using the Client:
   ? Automatic token management
   ? Type-safe Stage builders
   ? Built-in error handling

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
? ekoDB Kotlin Advanced Functions Example

? Setting up test data...
? Created 8 products

? Example 1: List All Products

? Function saved
? Found 8 products
??  Execution time: 0ms

? Example 2: Group Products by Category

? Function saved
? Category breakdown:
   {"category":{"value":"Electronics","type":"String"},"count":{"type":"Integer","value":5},"avg_price":{"type":"Float","value":367.0}}
   {"category":{"value":"Furniture","type":"String"},"count":{"type":"Integer","value":3},"avg_price":{"type":"Float","value":365.6666666666667}}
??  Execution time: 0ms

? Cleaning up...
? Cleanup complete

? All advanced function examples finished!

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
? ekoDB Kotlin AI Functions Example

? Setting up test data...
? Created 2 articles

? Example 1: Simple Chat Completion

? Chat function saved
? AI Response:
   Vector databases offer several benefits, including:

1. **Efficient Similarity Search**: They enable fast retrieval of similar items through vector representations, making them ideal for applications like recommendation systems and image searches.

2. **Handling High-Dimensional Data**: They effectively manage and query high-dimensional data, which is common in machine learning and AI applications.

3. **Scalability**: Vector databases can scale to handle large datasets, accommodating growth in data volume and complexity.

4. **Real-time Querying**: They support real-time data processing and querying, essential for dynamic applications.

5. **Integration with AI/ML Models**: Vector databases can efficiently store and query embeddings from deep learning models, facilitating AI-driven applications.

6. **Complex Data Understanding**: They allow for capturing and querying complex relationships in data, improving the understanding of contextual information.

7. **Reduced Latency**: By optimizing data retrieval through indexing techniques, they significantly reduce query response times.

8. **Versatile Use Cases**: Suitable for a wide range of applications, including natural language processing, computer vision, and anomaly detection.

Overall, vector databases enhance performance and usability for modern applications relying on data relationships and similarity.
??  Execution time: 0ms

? Example 2: Generate Embeddings

? Embed function saved
? Generated 2 embeddings
   Dimensions: 1536
??  Execution time: 0ms

? Cleaning up...
? Cleanup complete

? All AI function examples finished!

? This example demonstrates:
   ? Chat completions with system/user messages
   ? Embedding generation for text

BUILD SUCCESSFUL in 10s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
? ekoDB Kotlin Complete Functions Example

? Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

? Setting up complete test data...
? Created 5 products

? Example 1: FindAll + Group (Simple Aggregation)

? Function saved: hZuSe1cn1dvdYzth2CXcF_KmiFlmNhJtIasRFhEafLS_sA5K0JCOnPEf--F6N3Mn97IsFRyeFTBMuRyo1nmcuw
? Found 2 category groups

? Example 2: Simple Product Listing

? Function saved
? Found 5 products

? Example 3: Count by Category

? Function saved
? Found 2 categories

? Example 4: Multi-Stage Pipeline (FindAll ? Group ? Count)

? Function saved
? Pipeline executed with 1 results

? Cleaning up...
? Cleanup complete

? All complete function examples finished!

? This example demonstrates ekoDB's function system:
   ? FindAll operations
   ? Group aggregations (Count, Average)
   ? Multi-stage pipelines (FindAll ? Group ? Count)
   ? Function management (save, call, delete)

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
? ekoDB Kotlin CRUD Functions Example

? Setting up test data...
? Created 10 test users

? Example 1: List All Users

? Function saved
? Found 10 users
??  Execution time: 0ms

? Example 2: Count Users by Status

? Function saved
? User counts by status:
   {"count":{"type":"Integer","value":7},"status":{"type":"String","value":"active"}}
   {"status":{"value":"inactive","type":"String"},"count":{"value":3,"type":"Integer"}}
??  Execution time: 0ms

? Cleaning up...
? Cleanup complete

? All CRUD function examples finished!

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :processResources NO-SOURCE
> Task :compileKotlin
> Task :compileJava NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
? ekoDB Kotlin KV Store & Wrapped Types Example

? Demonstrates:
   ? Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   ? KV store operations (get, set, delete, exists, query)
   ? KV operations within functions
   ? Combined wrapped types + KV workflows

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
? Client initialized

? Example 1: Inserting Records with Wrapped Types

? Inserted order: StringValue(value=bBff1zdFPvHAK9e-hj0POymIgKBv75zOe1d1qFZoinVxm0jhJqSichZEj7hZnvdXXMvA6BvxWleWGyceDNM4HA)
? Inserted 2 products

? Example 2: function with Wrapped Type Parameters

? Function saved: TOTtKjeNUez1WupoE_2BCdwRrTR4sxQ2q4BM4BgRSOgMxoDBO60d4H7LAFW2zbxplVsWzLUrfKsb4KvFwbsVYQ
? Created order via function
??  Execution time: 0ms

? Example 3: Basic KV Store Operations

? Set session data
? Retrieved session: {"type":"Object","value":{"userId":"user_abc","role":"admin"}}
? Set cached data with 1 hour TTL
??  Deleted session

? Example 4: KV Operations in Functions

? Function saved: jrwKLNg89_-d5EulebV86tzUHLwO6LaIHhhs14pPAcLr9bT3QqkiLdwfR5RceD3PhDxRnVK21iz3-FxJxtRxXg
? Cached and retrieved product data
??  Execution time: 0ms

? Example 5: Combined Wrapped Types + KV Function

? Function saved: Ue2UStGQXyz_1x1OZ5oMcuB7oZDFdh3_xonDH4UT7upL56aKqRiOkalYvEDrnqbPUmkT7EBggIFsk3jQOdTt5w
? Processed order with caching
??  Stages executed: 3
??  Execution time: 0ms

? Cleaning up...
? Cleanup complete

? All KV & Wrapped Types examples completed!

? Key takeaways:
   ? Use field* helpers for type-safe wrapped values
   ? fieldDecimal() preserves precision (no floating point errors)
   ? KV store is great for caching and quick lookups
   ? FunctionStageConfig.Kv* classes work within functions

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
? ekoDB Kotlin Search Functions Example

? Setting up test data...
? Inserted 5 documents

? Example 1: List All Documents

? Function saved
? Found 5 documents
   1. {"value":"Vector Databases Explained","type":"String"} ({"type":"String","value":"Database"})
   2. {"value":"Database Design Principles","type":"String"} ({"type":"String","value":"Database"})
   3. {"type":"String","value":"Natural Language Processing"} ({"type":"String","value":"AI"})
   4. {"type":"String","value":"Getting Started with ekoDB"} ({"type":"String","value":"Database"})
   5. {"type":"String","value":"Introduction to Machine Learning"} ({"value":"AI","type":"String"})
??  Execution time: 0ms

? Example 2: Count Documents by Category

? Function saved
? Documents by category:
   {"count":{"value":3,"type":"Integer"},"category":{"type":"String","value":"Database"}}
   {"category":{"type":"String","value":"AI"},"count":{"value":2,"type":"Integer"}}
??  Execution time: 0ms

? Cleaning up...
? Cleanup complete

? All search function examples finished!

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
Created template: Data Migration (id: W67MA6Yphp4XmBUV_FocBA-TXt4bwLIfTzufpqZbQUYv1Qxv6XO_SQeWlRcD7XJIEclFxrGMqro_9yMjb7DiOg)

--- Listing templates ---
Templates: {"count":1,"items":[{"description":{"type":"String","value":"Template for migrating data between schemas"},"id":"W67MA6Yphp4XmBUV_FocBA-TXt4bwLIfTzufpqZbQUYv1Qxv6XO_SQeWlRcD7XJIEclFxrGMqro_9yMjb7DiOg","steps":{"type":"Array","value":[{"description":"Analyze source schema"},{"description":"Create target schema"},{"description":"Migrate records"},{"description":"Validate results"}]},"title":{"type":"String","value":"Data Migration"}}]}

--- Getting template ---
Fetched: {"type":"String","value":"Data Migration"}

--- Updating template ---
Updated description: {"type":"String","value":"Updated: comprehensive data migration workflow"}

--- Deleting template ---
Template deleted successfully

? Goal template CRUD example completed

BUILD SUCCESSFUL in 6s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
Created goal: 80s7Jc2zfe6x_2_88pB2BctIAD0zzn-CsuywRFYooL4LVgIqfJtnr7y6sQBlU4REmL0ucUzxYT4QnipaHgFlrA

--- Listing goals ---
Goals: {"count":1,"goals":[{"created_at":"2026-09-15T18:50:13.044861+00:00","description":"Ship the next major release","id":"80s7Jc2zfe6x_2_88pB2BctIAD0zzn-CsuywRFYooL4LVgIqfJtnr7y6sQBlU4REmL0ucUzxYT4QnipaHgFlrA","status":"pending","steps":"[{\"description\":\"Run test suite\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to production\"}]","title":"Deploy v2.0","updated_at":"2026-09-15T18:50:13.044861+00:00"}]}

--- Getting goal ---
Fetched: {"type":"String","value":"Deploy v2.0"}

--- Updating goal ---
Updated description: {"type":"String","value":"Ship v2.0 with full test coverage"}

--- Searching goals ---
Search results: {"count":1,"items":[{"_score":12.870000000000001,"created_at":{"type":"DateTime","value":"2026-09-15T18:50:13.044861+00:00"},"description":{"type":"String","value":"Ship v2.0 with full test coverage"},"id":"80s7Jc2zfe6x_2_88pB2BctIAD0zzn-CsuywRFYooL4LVgIqfJtnr7y6sQBlU4REmL0ucUzxYT4QnipaHgFlrA","status":{"type":"String","value":"pending"},"steps":{"type":"String","value":"[{\"description\":\"Run test suite\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to production\"}]"},"title":{"type":"String","value":"Deploy v2.0"},"updated_at":{"type":"DateTime","value":"2026-09-15T18:50:13.101196+00:00"}}]}

--- Goal step lifecycle ---
Step 0 started: "80s7Jc2zfe6x_2_88pB2BctIAD0zzn-CsuywRFYooL4LVgIqfJtnr7y6sQBlU4REmL0ucUzxYT4QnipaHgFlrA"
Step 0 completed: "80s7Jc2zfe6x_2_88pB2BctIAD0zzn-CsuywRFYooL4LVgIqfJtnr7y6sQBlU4REmL0ucUzxYT4QnipaHgFlrA"
Step 1 started: "80s7Jc2zfe6x_2_88pB2BctIAD0zzn-CsuywRFYooL4LVgIqfJtnr7y6sQBlU4REmL0ucUzxYT4QnipaHgFlrA"
Step 1 failed: "80s7Jc2zfe6x_2_88pB2BctIAD0zzn-CsuywRFYooL4LVgIqfJtnr7y6sQBlU4REmL0ucUzxYT4QnipaHgFlrA"

--- Completing goal ---
Goal status after complete: {"type":"String","value":"pending_review"}

--- Approving goal ---
Goal status after approve: {"type":"String","value":"in_progress"}

--- Creating goal for rejection ---
Goal status after reject: {"type":"String","value":"failed"}

--- Creating task ---
Created task: CqXNegRwOpwwtYyhQPukbSOcnCYSfRhS0_h9zHm6nGY6jpLCNVtQZLTC9cCIPccQK6BTy1AtECzS7lGRZqqnag

--- Listing tasks ---
Tasks: {"count":1,"items":[{"cron":{"type":"String","value":"0 2 * * *"},"description":{"type":"String","value":"Full database backup every night at 2 AM"},"id":"CqXNegRwOpwwtYyhQPukbSOcnCYSfRhS0_h9zHm6nGY6jpLCNVtQZLTC9cCIPccQK6BTy1AtECzS7lGRZqqnag","name":{"type":"String","value":"Nightly Backup"}}]}

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
Created agent: l-KzWeoKPFHpOY662VLZuJPyMnwJOqU8hOzgdW5KHHIyMDAIrLapTTpXthE_xN2ffacDNv6VHdRpd0vha0G4KA ? null

--- Listing agents ---
Agents: {"count":1,"items":[{"deployment_id":{"type":"String","value":"deploy_kt_example"},"id":"l-KzWeoKPFHpOY662VLZuJPyMnwJOqU8hOzgdW5KHHIyMDAIrLapTTpXthE_xN2ffacDNv6VHdRpd0vha0G4KA","llm_model":{"type":"String","value":"gpt-4.1"},"name":{"type":"String","value":"DataBot"},"system_prompt":{"type":"String","value":"You are a data analysis assistant."}}]}

--- Getting agent by ID ---
Agent: {"type":"String","value":"DataBot"}

--- Getting agent by name ---
By name: {"type":"String","value":"DataBot"}

--- Updating agent ---
Updated agent system_prompt

--- Agents by deployment ---
Agents for deployment: {"count":0,"items":[]}

--- Deleting agent ---
Agent deleted

--- Cleanup: deleting goals ---
Goals deleted

=== Example Complete ===

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
? Sample data created

=== Example 1: Single collection join (users with departments) ===
? Found 2 users with department data
  - Bob Smith: Sales
  - Alice Johnson: Engineering

=== Example 2: Join with filtering ===
? Found 1 users in Engineering
  - Alice Johnson: Building A

=== Example 3: Join with user profiles ===
? Found 2 users with profile data
  - Bob Smith: Sales Manager
  - Alice Johnson: Senior Software Engineer

=== Example 4: Join orders with user data ===
? Found 2 completed orders
  - Laptop ($1200) by Alice Johnson
  - Mouse ($25) by Alice Johnson

=== Example 5: Complex join with multiple conditions ===
? Found 2 users with example.com emails
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
? Deleted test collections

? Join operations example completed successfully

BUILD SUCCESSFUL in 8s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
? Client created
? kt_users_register saved
? kt_users_login saved
? kt_users_verify_token saved

=== Auth flow defined as pure stored functions ===
Call them like:
  POST /api/functions/kt_users_register { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/kt_users_login    { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/kt_users_verify_token { "token": "<jwt>" }

? Cleaned up demo functions

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
Links for user:alice: [{"collection":"invoices","document_id":"inv_100","field_path":null,"created_at":"2026-09-15T18:50:38.896396Z","last_accessed":"2026-09-15T18:50:38.902537Z","metadata":{}},{"collection":"orders","document_id":"order_001","field_path":null,"created_at":"2026-09-15T18:50:38.877175Z","last_accessed":"2026-09-15T18:50:38.902537Z","metadata":{}},{"collection":"orders","document_id":"order_002","field_path":null,"created_at":"2026-09-15T18:50:38.886981Z","last_accessed":"2026-09-15T18:50:38.902537Z","metadata":{}}]

--- Unlinking document ---
Unlinked order_002: null

--- Verifying remaining links ---
Remaining links: [{"collection":"invoices","document_id":"inv_100","field_path":null,"created_at":"2026-09-15T18:50:38.896396Z","last_accessed":"2026-09-15T18:50:38.915969Z","metadata":{}},{"collection":"orders","document_id":"order_001","field_path":null,"created_at":"2026-09-15T18:50:38.877175Z","last_accessed":"2026-09-15T18:50:38.915969Z","metadata":{}}]

--- Cleanup ---
Unlinked remaining documents
Deleted key: user:alice

=== Example Complete ===

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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
? Set key: user:123

=== KV Get ===
? Retrieved value: {"type":"Object","value":{"name":"Alice","email":"alice@example.com","role":"admin"}}

=== KV Set with TTL ===
? Set key with 10s TTL: session:abc123

=== Verify TTL Key ===
? Session value: {"value":{"user_id":"123","created_at":1789498246239},"type":"Object"}
  (Will expire in 10 seconds)

=== KV Batch Set ===
? Batch set 3 keys
  config:db: success
  config:cache: success
  config:api: success

=== KV Batch Get ===
? Batch retrieved 3 values
  config:db: {"value":{"host":"localhost","port":5432}}
  config:cache: {"value":{"ttl":3600,"enabled":true}}
  config:api: {"value":{"retries":3,"timeout":30}}

=== KV Exists ===
? Key exists: true

=== KV Find (Pattern Query) ===
? Found 3 keys matching 'config:.*'

=== KV Query (Alias for Find) ===
? Total keys in store: 5

=== KV Delete ===
? Deleted key: user:123

=== Verify Deletion ===
? Key exists after delete: false

=== KV Batch Delete ===
? Batch deleted 3 keys
  config:db: deleted
  config:cache: deleted
  config:api: deleted

=== Example Complete ===

BUILD SUCCESSFUL in 6s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/9.7.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
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

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
Consider enabling configuration cache to speed up this build: https://docs.gradle.org/9.7.1/userguide/configuration_cache_enabling.html

🐍 Building Python client package...
🔧 Ensuring maturin is available in .venv...
🔨 Building wheel...
🍹 Building a mixed python/rust project
🐍 Found CPython 3.11 at /Library/Frameworks/Python.framework/Versions/3.11/bin/python3
🔗 Found pyo3 bindings with abi3-py3.8 support
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.22s
📦 Built wheel for abi3 Python ≥ 3.8 to ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.26.4-cp38-abi3-macosx_11_0_arm64.whl
📦 Installing Python wheel into .venv...
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.26.4-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.26.4
    Uninstalling ekodb_client-0.26.4:
      Successfully uninstalled ekodb_client-0.26.4
Successfully installed ekodb-client-0.26.4
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
    ✓ Generated embedding: 1536 dimensions in 360.470875ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 300.273083ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 404.269167ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 262.820667ms
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 295.301959ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 305.134333ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 364.563667ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 327.223667ms
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 277.821916ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 262.9575ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 429.75675ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 251.396625ms
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
    ✓ Generated embedding: 1536 dimensions in 249.024958ms
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 76.088458ms

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

Writing memory-safe and high-performance database code involves several strategies that leverage best practices in programming, database design, and performance optimization. Here’s a comprehensive approach that combines these aspects:

### 1. Use Appropriate Database Technology
- **Choose the Right DBMS:** Assess your application needs to choose between SQL (for structured, relational data) and NoSQL (for unstructured or semi-structured data). Use NoSQL for flexible schemas and horizontal scaling.
- **Proper Indexing:** Use indexes on frequently queried fields to improve lookup times. Monitor and optimize existing indexes.

### 2. Memory Safety
- **Use Memory-Safe Programming Languages:** Opt for languages that enforce memory safety through built-in features (e.g., Rust, Java) when possible.
- **Handling Pointers Carefully:** In languages like C/C++, ensure pointers are handled with care to avoid memory leaks or segmentation faults.
- **Avoid Buffer Overflows:** Implement bounds-checking and utilize safe libraries for string handling.
- **Garbage Collection:** In languages with garbage collection (e.g., Java, C#), ensure you manage object lifetimes correctly to avoid memory leaks.

### 3. Optimizing Database Queries
- **Optimize Query Logic:** Write efficient, well-structured queries. Use `EXPLAIN` to analyze query plans and find bottlenecks.
- **Batch Operations:** Use batch inserts/updates instead of individual operations to reduce transaction overhead.
- **Connection Pooling:** Reuse database connections instead of opening a new one for each request to decrease latency.

### 4. Error Handling
- **Graceful Degradation:** Implement proper error handling to ensure that your application can recover or gracefully handle unexpected database errors without crashing.
- **Transactions:** Use transactions to ensure data integrity and rollback mechanisms during failures.

### 5. Data Manipulation
- **Prepared Statements:** Use prepared statements to prevent SQL injection and improve performance by allowing the database to cache execution plans.
- **Normalization:** Normalize your database to eliminate redundancy, which can help reduce memory usage and improve data integrity.

### 6. Caching Strategies
- **Query Caching:** Cache frequently accessed data to reduce load on the database. Use in-memory storage (like Redis) for caching.
- **Materialized Views:** Create materialized views for complex queries that are costly to execute.

### 7. Profiling and Monitoring
- **Performance Profiling:** Continuously profile your database interactions to identify slow queries and optimize them.
- **Monitoring Tools:** Use database monitoring tools to keep an eye on memory usage, query performance, and database health.

### 8. Scaling Considerations
- **Partitioning:** For heavily load-balanced applications, consider sharding the database or partitioning data across multiple databases.
- **Load Balancing:** Distribute database load across multiple replicas or instances to enhance response times and reduce latency.

### Example Snippet
Here’s an example of using prepared statements in Python with a PostgreSQL database to ensure memory safety and performance:

```python
import psycopg2
from psycopg2 import sql

# Connection pooling
from psycopg2 import pool
db_pool = pool.SimpleConnectionPool(minconn=1, maxconn=10, user='user', password='password', host='127.0.0.1', port='5432', database='test_db')

def insert_user(name, email):
    conn = db_pool.getconn()
    try:
        with conn.cursor() as cursor:
            cursor.execute("INSERT INTO users (name, email) VALUES (%s, %s)", (name, email))
            conn.commit()  # Ensure atomicity
    except Exception as e:
        print(f"Error occurred: {e}")
        conn.rollback()
    finally:
        db_pool.putconn(conn)  # Return the connection to the pool
```

### In Summary
Maintain a balance between memory safety and performance by choosing the right tools, optimizing your database interactions, and constantly monitoring performance metrics. This multifaceted approach can significantly enhance the reliability and efficiency of your database code.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 274.832083ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 4154 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 316.989375ms
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
  ✓ Text search completed in 44.164792ms

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
=== ekoDB RAG Conversation System ===

This example shows how ekoDB can power a self-improving AI system
that learns from its own conversation history.

=== Step 1: Building Conversation History ===
Storing previous conversations with embeddings...

  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 34 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.261s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.297s
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
    ✓ Generated embedding: 1536 dimensions in 0.572s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.300s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.323s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.238s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.282s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.344s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.406s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.337s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.266s
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
    ✓ Generated embedding: 1536 dimensions in 0.292s
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.068s

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

Writing memory-safe, high-performance database code involves several best practices and approaches that span the design of your application, the choice of the database, and the handling of data within your code. Below are some key strategies to consider:

### 1. **Choose the Right Database**

- **NoSQL vs. SQL**: Depending on your needs, choose a suitable database type. SQL databases are great for complex queries and relationships, while NoSQL databases excel in horizontal scaling and working with unstructured data.
- **Database Optimization**: Take advantage of built-in database features such as indexing, caching, and partitioning to improve performance.

### 2. **Connection Management**

- **Use Connection Pools**: Instead of opening a new connection for each request, use connection pooling to manage database connections efficiently.
- **Limit Connection Lifespan**: Occasionally refresh connections to avoid stale connections that can cause memory leaks.

### 3. **Efficient Query Writing**

- **Prepared Statements**: Use prepared statements to prevent SQL injection attacks and reduce the overhead of query parsing.
- **Optimize Queries**: Assess query execution plans and use the right indexes (e.g., composite indexes for multi-field filters) to speed up data access.
- **Batch Operations**: Use batch insert/update queries to reduce round trips to the database.

### 4. **Memory Management**

- **Limit Result Set Size**: Only fetch the necessary data by applying pagination or limits to your queries.
- **Lazy Loading**: Consider implementing lazy loading patterns where related data is only fetched when needed.
- **Free Unused Resources**: Ensure that cursors and result sets are properly closed and cleaned up after use to free memory.

### 5. **Concurrency Handling**

- **Transactions**: Use database transactions to ensure data consistency and integrity, especially when performing multiple related operations.
- **Optimistic Concurrency Control**: Implement optimistic concurrency controls where applicable to handle updates without lengthy locks.

### 6. **Error Handling**

- **Graceful Error Handling**: Implement robust error handling to capture and log exceptions without crashing the application.
- **Graceful Degradation**: Ensure that when failures occur, the system degrades gracefully rather than becoming entirely unresponsive.

### 7. **Performance Monitoring**

- **Evaluate Performance**: Regularly monitor performance metrics using built-in database tools and custom logging to discover slow queries or high resource usage.
- **Profile Database Calls**: Use profiling tools to analyze database calls and identify bottlenecks in your application.

### 8. **Use of Database Tools and Libraries**

- Leverage ORM (Object-Relational Mapping) libraries that support memory safety. Some ORMs automatically handle connection pooling, memory management, and query optimization for you while maintaining a focus on high performance.

### Example (Using ekoDB)

If you are using ekoDB, which provides a full backend platform, you can follow these design principles effectively. For instance:

- Utilize `create_function` to create optimized stored procedures to encapsulate complex operations.
- Employ `query_explain` before executing queries to ensure they are optimized and use indexes appropriately.
- Manage connection lifecycles with provided tools seamlessly without leaking connections.

### Conclusion

By combining efficient database design, robust code practices, and performance monitoring, you can create memory-safe and high-performance database code. Always consider the unique needs of your application and system architecture when adopting these practices.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.288s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 3706 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.251s
    • Function auto-cleaned up by client
✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...

✓ Found 3 messages mentioning ownership:
  1. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  2. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  3. From conv_new_question: Writing memory-safe, high-performance database code involves several best practices and approaches that span the design of your application, the choice of the database, and the handling of data within your code. Below are some key strategies to consider:

### 1. **Choose the Right Database**

- **NoSQL vs. SQL**: Depending on your needs, choose a suitable database type. SQL databases are great for complex queries and relationships, while NoSQL databases excel in horizontal scaling and working with unstructured data.
- **Database Optimization**: Take advantage of built-in database features such as indexing, caching, and partitioning to improve performance.

### 2. **Connection Management**

- **Use Connection Pools**: Instead of opening a new connection for each request, use connection pooling to manage database connections efficiently.
- **Limit Connection Lifespan**: Occasionally refresh connections to avoid stale connections that can cause memory leaks.

### 3. **Efficient Query Writing**

- **Prepared Statements**: Use prepared statements to prevent SQL injection attacks and reduce the overhead of query parsing.
- **Optimize Queries**: Assess query execution plans and use the right indexes (e.g., composite indexes for multi-field filters) to speed up data access.
- **Batch Operations**: Use batch insert/update queries to reduce round trips to the database.

### 4. **Memory Management**

- **Limit Result Set Size**: Only fetch the necessary data by applying pagination or limits to your queries.
- **Lazy Loading**: Consider implementing lazy loading patterns where related data is only fetched when needed.
- **Free Unused Resources**: Ensure that cursors and result sets are properly closed and cleaned up after use to free memory.

### 5. **Concurrency Handling**

- **Transactions**: Use database transactions to ensure data consistency and integrity, especially when performing multiple related operations.
- **Optimistic Concurrency Control**: Implement optimistic concurrency controls where applicable to handle updates without lengthy locks.

### 6. **Error Handling**

- **Graceful Error Handling**: Implement robust error handling to capture and log exceptions without crashing the application.
- **Graceful Degradation**: Ensure that when failures occur, the system degrades gracefully rather than becoming entirely unresponsive.

### 7. **Performance Monitoring**

- **Evaluate Performance**: Regularly monitor performance metrics using built-in database tools and custom logging to discover slow queries or high resource usage.
- **Profile Database Calls**: Use profiling tools to analyze database calls and identify bottlenecks in your application.

### 8. **Use of Database Tools and Libraries**

- Leverage ORM (Object-Relational Mapping) libraries that support memory safety. Some ORMs automatically handle connection pooling, memory management, and query optimization for you while maintaining a focus on high performance.

### Example (Using ekoDB)

If you are using ekoDB, which provides a full backend platform, you can follow these design principles effectively. For instance:

- Utilize `create_function` to create optimized stored procedures to encapsulate complex operations.
- Employ `query_explain` before executing queries to ensure they are optimized and use indexes appropriately.
- Manage connection lifecycles with provided tools seamlessly without leaking connections.

### Conclusion

By combining efficient database design, robust code practices, and performance monitoring, you can create memory-safe and high-performance database code. Always consider the unique needs of your application and system architecture when adopting these practices.

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
    ✓ Generated embedding: 1536 dimensions in 0.290s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.326s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.298s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.280s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.277s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.288s
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
    ✓ Generated embedding: 1536 dimensions in 0.278s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.261s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.233s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.258s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.231s
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
    ✓ Generated embedding: 1536 dimensions in 0.215s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.079s
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

To write memory-safe high-performance database code, consider the following strategies that combine best practices in database management, programming languages, and system architectures:

### 1. **Use Prepared Statements and Parameterized Queries**
   - **Prevention of SQL Injection**: Ensure that queries are safe from injection attacks by using prepared statements, which separate SQL code from data.
   - **Performance**: Prepared statements can be pre-parsed and compiled, which can lead to performance improvements, especially for repeated queries.

### 2. **Connection Pooling**
   - **Efficient Resource Management**: Utilize connection pooling to manage database connections efficiently. This reduces the overhead of establishing new connections and helps maintain performance.
   - **Concurrency Handling**: Pooling allows multiple requests to share connections, enhancing scalability in applications with high concurrency demands.

### 3. **Memory Management**
   - **Garbage Collection**: In languages like Java or C#, rely on garbage collection for automatic memory management, but also monitor memory usage to avoid leaks.
   - **Manual Memory Management**: In languages like C or C++, ensure to free allocated memory appropriately to prevent memory leaks, utilizing smart pointers where applicable.

### 4. **Optimize Queries**
   - **Indexing**: Create appropriate indexes on database tables to speed up query performance. Analyze query performance using explain plans to understand how indexes are utilized.
   - **Normalization**: Normalize database schemas to reduce redundancy and improve data integrity, but also consider denormalization for read-heavy applications to minimize JOIN operations.

### 5. **Batch Processing**
   - **Bulk Inserts and Updates**: Use batch operations for inserts and updates to reduce the number of database transactions. This is more efficient than executing many individual queries.
   - **Transaction Management**: Wrap multiple operations in transactions to ensure atomicity and to minimize the overhead of commit operations.

### 6. **Asynchronous I/O Operations**
   - **Non-Blocking Calls**: If supported by your database and programming environment, use asynchronous I/O to avoid blocking application threads while waiting for database responses, improving overall application responsiveness.

### 7. **Caching Mechanisms**
   - **In-Memory Caching**: Utilize in-memory caching strategies (like Redis or Memcached) to reduce the load on your database and speed up access to frequently queried data.
   - **Application-Level Caching**: Implement caching at the application level to store results of expensive queries for quick retrieval.

### 8. **Profiling and Monitoring**
   - **Database Monitoring Tools**: Use tools to monitor query performance and system resource usage to identify bottlenecks (e.g., slow queries, high CPU, or memory consumption).
   - **Profile Your Code**: Use profiling tools in your development environment to analyze memory usage and identify performance hotspots.

### 9. **Utilize Connection Options Wisely**
   - **Timeouts and Retries**: Set appropriate connection timeouts and retry policies to handle transient failures gracefully without crashing the application.
   - **Transaction Isolation Levels**: Choose the correct isolation level based on the use case to balance between data integrity and performance.

### 10. **Concurrency Control**
   - **Optimistic Locking**: Implement optimistic locking to avoid blocking database rows during updates, which can improve performance in high-write environments.
   - **Pessimistic Locking**: In scenarios demanding strong consistency, use pessimistic locking judiciously.

By integrating these strategies into your coding practices, you can create database interactions that are both memory-safe and performance-oriented. Always ensure to test under load and review code regularly for performance improvements.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.269s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 3950 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.246s
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
  ✓ Text search completed in 0.044s
✓ Found 3 messages mentioning ownership:
  1. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  2. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  3. From conv_new_question: To write memory-safe high-performance database code, consider the following strategies that combine best practices in database management, programming languages, and system architectures:

### 1. **Use Prepared Statements and Parameterized Queries**
   - **Prevention of SQL Injection**: Ensure that queries are safe from injection attacks by using prepared statements, which separate SQL code from data.
   - **Performance**: Prepared statements can be pre-parsed and compiled, which can lead to performance improvements, especially for repeated queries.

### 2. **Connection Pooling**
   - **Efficient Resource Management**: Utilize connection pooling to manage database connections efficiently. This reduces the overhead of establishing new connections and helps maintain performance.
   - **Concurrency Handling**: Pooling allows multiple requests to share connections, enhancing scalability in applications with high concurrency demands.

### 3. **Memory Management**
   - **Garbage Collection**: In languages like Java or C#, rely on garbage collection for automatic memory management, but also monitor memory usage to avoid leaks.
   - **Manual Memory Management**: In languages like C or C++, ensure to free allocated memory appropriately to prevent memory leaks, utilizing smart pointers where applicable.

### 4. **Optimize Queries**
   - **Indexing**: Create appropriate indexes on database tables to speed up query performance. Analyze query performance using explain plans to understand how indexes are utilized.
   - **Normalization**: Normalize database schemas to reduce redundancy and improve data integrity, but also consider denormalization for read-heavy applications to minimize JOIN operations.

### 5. **Batch Processing**
   - **Bulk Inserts and Updates**: Use batch operations for inserts and updates to reduce the number of database transactions. This is more efficient than executing many individual queries.
   - **Transaction Management**: Wrap multiple operations in transactions to ensure atomicity and to minimize the overhead of commit operations.

### 6. **Asynchronous I/O Operations**
   - **Non-Blocking Calls**: If supported by your database and programming environment, use asynchronous I/O to avoid blocking application threads while waiting for database responses, improving overall application responsiveness.

### 7. **Caching Mechanisms**
   - **In-Memory Caching**: Utilize in-memory caching strategies (like Redis or Memcached) to reduce the load on your database and speed up access to frequently queried data.
   - **Application-Level Caching**: Implement caching at the application level to store results of expensive queries for quick retrieval.

### 8. **Profiling and Monitoring**
   - **Database Monitoring Tools**: Use tools to monitor query performance and system resource usage to identify bottlenecks (e.g., slow queries, high CPU, or memory consumption).
   - **Profile Your Code**: Use profiling tools in your development environment to analyze memory usage and identify performance hotspots.

### 9. **Utilize Connection Options Wisely**
   - **Timeouts and Retries**: Set appropriate connection timeouts and retry policies to handle transient failures gracefully without crashing the application.
   - **Transaction Isolation Levels**: Choose the correct isolation level based on the use case to balance between data integrity and performance.

### 10. **Concurrency Control**
   - **Optimistic Locking**: Implement optimistic locking to avoid blocking database rows during updates, which can improve performance in high-write environments.
   - **Pessimistic Locking**: In scenarios demanding strong consistency, use pessimistic locking judiciously.

By integrating these strategies into your coding practices, you can create database interactions that are both memory-safe and performance-oriented. Always ensure to test under load and review code regularly for performance improvements.

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
    ✓ Generated embedding: 1536 dimensions in 0.307s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.228s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.269s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.318s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.307s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.271s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.252s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.273s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.302s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.276s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.242s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.260s
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
    ✓ Generated embedding: 1536 dimensions in 0.244s
    • Function auto-cleaned up by client

→ Executing HybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.055s

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

Writing memory-safe, high-performance database code involves combining best practices in both database interactions and programming language constructs. Here’s a comprehensive approach to achieve this:

### 1. Use Efficient Querying Techniques
- **Prepared Statements**: Use prepared statements to enhance performance by pre-compiling SQL queries. This reduces the overhead of query parsing and execution.
- **Batch Operations**: When inserting or updating multiple records, use batch operations to minimize the number of database calls.
- **Proper Indexing**: Ensure that your database tables are appropriately indexed to speed up query performance. Analyze query plans to identify the need for indexes.

### 2. Optimize Database Design
- **Normalization**: Normalize your database schema to reduce redundancy, but also consider denormalization for read-heavy applications where performance is critical and data integrity can be managed.
- **Use Proper Data Types**: Choose efficient data types for your fields to reduce memory usage and increase performance (e.g., using INTEGER instead of BIGINT if possible).

### 3. Connection Management
- **Connection Pooling**: Utilize connection pooling to manage database connections efficiently, reducing the overhead of establishing database connections for each request.
- **Close Connections**: Ensure that all database connections are closed or returned to the pool promptly to avoid resource leaks.

### 4. Error Handling
- **Graceful Handling**: Implement proper error handling strategies (such as try-catch blocks) to manage exceptions without crashing the application and ensure resources are freed.
- **Use Transactions**: Use transactions to ensure that a series of database operations are completed successfully or rolled back in case of errors, maintaining data integrity.

### 5. Memory Management
- **Pooling Resources**: For languages like C/C++, use memory pool allocators to minimize fragmentation and allocation overhead.
- **Automatic Garbage Collection**: If using languages with garbage collection (e.g., Java, C#), ensure large datasets are processed in small chunks to allow for effective garbage collection.

### 6. Asynchronous Operations
- **Non-blocking I/O**: Consider using non-blocking I/O operations where possible to prevent blocking threads while waiting for database responses, especially in web applications.

### 7. Profiling and Monitoring
- **Performance Monitoring**: Use monitoring tools to analyze query performance, system resource utilization, and identify bottlenecks in the application's interaction with the database.
- **Regular Profiling**: Profile your code regularly to understand memory usage patterns and optimize hotspots.

### 8. Security Considerations
- **Sanitize Inputs**: Always validate and sanitize user inputs to prevent SQL injection attacks and other malicious behaviors.
- **Limit Permissions**: Operate with the least privileges necessary for database accounts to minimize security risks.

### 9. Use of Frameworks and Libraries
- **ORMs**: If using an Object-Relational Mapping (ORM) framework, ensure it’s optimized for performance. Some ORMs can generate inefficient queries if not configured properly.
- **Data Access Layers**: Implement a data access layer that abstracts the database interactions, allowing for better maintenance and reusability.

### Conclusion
Combining these practices will help you write database code that is both memory-safe and performs well. Always keep performance considerations in mind during the design and implementation phases, and continuously evaluate your code as the application and database grow.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.250s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 3652 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.291s
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
  ✓ Text search completed in 0.041s

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

  ? Calling ekoDB embed() helper...
    ? Using model: text-embedding-3-small
    ? Text length: 34 characters
    ? Behind the scenes: Creating temp Function with Embed operation
    ? Generated embedding: 1536 dimensions in 0.313s
    ? Function auto-cleaned up by client
  ? Calling ekoDB embed() helper...
    ? Using model: text-embedding-3-small
    ? Text length: 169 characters
    ? Behind the scenes: Creating temp Function with Embed operation
    ? Generated embedding: 1536 dimensions in 0.297s
    ? Function auto-cleaned up by client
  ? Calling ekoDB embed() helper...
    ? Using model: text-embedding-3-small
    ? Text length: 33 characters
    ? Behind the scenes: Creating temp Function with Embed operation
    ? Generated embedding: 1536 dimensions in 0.248s
    ? Function auto-cleaned up by client
  ? Calling ekoDB embed() helper...
    ? Using model: text-embedding-3-small
    ? Text length: 230 characters
    ? Behind the scenes: Creating temp Function with Embed operation
    ? Generated embedding: 1536 dimensions in 0.256s
    ? Function auto-cleaned up by client
? Stored Rust programming conversation (4 messages)
  ? Calling ekoDB embed() helper...
    ? Using model: text-embedding-3-small
    ? Text length: 31 characters
    ? Behind the scenes: Creating temp Function with Embed operation
    ? Generated embedding: 1536 dimensions in 0.244s
    ? Function auto-cleaned up by client
  ? Calling ekoDB embed() helper...
    ? Using model: text-embedding-3-small
    ? Text length: 217 characters
    ? Behind the scenes: Creating temp Function with Embed operation
    ? Generated embedding: 1536 dimensions in 0.296s
    ? Function auto-cleaned up by client
  ? Calling ekoDB embed() helper...
    ? Using model: text-embedding-3-small
    ? Text length: 33 characters
    ? Behind the scenes: Creating temp Function with Embed operation
    ? Generated embedding: 1536 dimensions in 0.226s
    ? Function auto-cleaned up by client
  ? Calling ekoDB embed() helper...
    ? Using model: text-embedding-3-small
    ? Text length: 232 characters
    ? Behind the scenes: Creating temp Function with Embed operation
    ? Generated embedding: 1536 dimensions in 0.285s
    ? Function auto-cleaned up by client
? Stored database design conversation (4 messages)
  ? Calling ekoDB embed() helper...
    ? Using model: text-embedding-3-small
    ? Text length: 36 characters
    ? Behind the scenes: Creating temp Function with Embed operation
    ? Generated embedding: 1536 dimensions in 0.296s
    ? Function auto-cleaned up by client
  ? Calling ekoDB embed() helper...
    ? Using model: text-embedding-3-small
    ? Text length: 178 characters
    ? Behind the scenes: Creating temp Function with Embed operation
    ? Generated embedding: 1536 dimensions in 0.281s
    ? Function auto-cleaned up by client
  ? Calling ekoDB embed() helper...
    ? Using model: text-embedding-3-small
    ? Text length: 37 characters
    ? Behind the scenes: Creating temp Function with Embed operation
    ? Generated embedding: 1536 dimensions in 0.272s
    ? Function auto-cleaned up by client
  ? Calling ekoDB embed() helper...
    ? Using model: text-embedding-3-small
    ? Text length: 213 characters
    ? Behind the scenes: Creating temp Function with Embed operation
    ? Generated embedding: 1536 dimensions in 0.245s
    ? Function auto-cleaned up by client
? Stored performance optimization conversation (4 messages)

=== Step 2: New User Question with Context Retrieval ===
User asks: "How do I write memory-safe high-performance database code?"

=== Step 3: Searching Related Context ===
Using hybrid search to find relevant messages from all conversations...


? Generating embedding for user question...
  ? Calling ekoDB embed() helper...
    ? Using model: text-embedding-3-small
    ? Text length: 58 characters
    ? Behind the scenes: Creating temp Function with Embed operation
    ? Generated embedding: 1536 dimensions in 0.288s
    ? Function auto-cleaned up by client

? Executing hybridSearch()...
  ? Collection: rag_messages
  ? Query text: "How do I write memory-safe high-performance database code?"
  ? Vector dimensions: 1536
  ? Limit: 5 results
  ? Search type: Semantic (vector) + Keyword (text)
  ? Server combines both scores for relevance ranking
  ? Search completed in 0.093s

? Found 5 related messages across all conversations:
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
? Context prepared from search results
? AI would use this context to generate comprehensive response

=== Step 5: Storing New Conversation ===
  ? Calling ekoDB embed() helper...
    ? Using model: text-embedding-3-small
    ? Text length: 58 characters
    ? Behind the scenes: Creating temp Function with Embed operation
    ? Generated embedding: 1536 dimensions in 0.265s
    ? Function auto-cleaned up by client
? New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...


? Executing textSearch()...
  ? Collection: rag_messages
  ? Query: "ownership system"
  ? Limit: 3 results
  ? Search method: Full-text with fuzzy matching & stemming
  ? No vector embeddings needed - pure keyword search
  ? Text search completed in 0.044s

? Found 3 messages mentioning ownership:
  1. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  2. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  3. From conv_rust_programming: The borrow checker enforces Rust's ownership rules at compile time. It ensures that references don't outlive the data they point to and prevents data races by allowing either multiple immutable references or one mutable reference.

=== System Statistics ===

? Querying database statistics...
  ? Using findAllWithLimit() helper - simplified query API

? Database Statistics:
  ? Total conversations: 4
  ? Total messages stored: 13
  ? All messages indexed for vector search ?
  ? All messages indexed for text search ?
  ? All messages queryable by metadata ?

=== Step 8: Dynamic Search Configuration ===
Each conversation can have its own search config...

? Conversations can store custom search configurations:
  ? Search type: hybrid, text, or vector
  ? Relevance thresholds
  ? Filter by tags or metadata
  ? Collection-specific settings
  ? Per-conversation AI behavior

This enables context-aware search tuned to each conversation's needs!

=== Cleanup ===
? Cleanup complete


=== ? Summary: What This Example Showed ===

? ekoDB Native Capabilities Used:
  ? Functions with Embed operation (AI integration)
  ? Hybrid Search (text + vector combined)
  ? Text Search (full-text with stemming)
  ? Automatic embedding generation
  ? Cross-collection queries

? New Client Helper Methods:
  ? client.embed(text, model) - Generate embeddings
  ? client.hybridSearch() - Semantic + keyword search
  ? client.textSearch() - Full-text search
  ? client.findAllWithLimit() - Query all documents

? Key Takeaways:
  1. ekoDB handles AI Functions natively - no external services needed
  2. One-line embedding generation with auto-cleanup
  3. Hybrid search combines semantic understanding + keyword matching
  4. Perfect for RAG: store, search, and retrieve context
  5. All AI capabilities accessible through simple client methods

? Build production RAG systems with ekoDB!
   ? Set OPENAI_API_KEY in your ekoDB server environment
   ? Use these client helpers to make AI integration simple
   ? Scale to millions of documents with native indexing


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
