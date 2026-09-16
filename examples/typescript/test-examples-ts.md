make test-examples-typescript
📦 Building TypeScript client library...

> @ekodb/ekodb-client@0.26.4 prepare
> npm run build


> @ekodb/ekodb-client@0.26.4 build
> tsc


up to date, audited 46 packages in 839ms

12 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.26.4 build
> tsc

✅ TypeScript client built!

added 1 package, removed 1 package, and audited 13 packages in 383ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
=== ekoDB Advanced CRUD Example (TypeScript) ===

--- Inserting base record ---
Inserted: 6u5dKOi6Jx9rpwHmj-UaR3qk6coAYm31kBafX9x0ADSvTYLfJjXKPPykoXFEqv3Vgsh8Fr1qLlp7RJz3TX2MFQ

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
✓ Created session: sEvd-GaxG-EXsSvlh7c0K1eoO5TLBFNjqTcg5pHKtPfaMb5g34yHnqmck-XdtKZm6FW2rx7HiXJOr06Xae9MKg

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you need more information or have further questions, feel free to ask!

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
✓ Created second session: lQrBJD7Qx2fC7oaU7IO8ho19QoF9IhCpcmQHeKdKuJ8nrY75z-Vzv9azTnx042jc-8HjW02dyPuHj96u0IbEnw
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: sEvd-GaxG-EXsSvlh7c0K1eoO5TLBFNjqTcg5pHKtPfaMb5g34yHnqmck-XdtKZm6FW2rx7HiXJOr06Xae9MKg
✓ Deleted chat session: lQrBJD7Qx2fC7oaU7IO8ho19QoF9IhCpcmQHeKdKuJ8nrY75z-Vzv9azTnx042jc-8HjW02dyPuHj96u0IbEnw
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: h6NlubxlUOT5IF5q_I3NFOc-dKJ7xSBaKuvIfbPIP2DjBB_2f-exyWH9Jwdspk8CROLphtqG7g1TnQVoJPQfEg

=== Sending Chat Message ===
Message ID: mZHmK9ekxFnQdaJzChuDxo5hIdrQ1mbXHQ5uzlFXFA7ARqEYu5uosOX2hOdNcppLmC_KobP2ph-m3-7tdUHZ2A

=== AI Response ===
The available products and their prices are as follows:

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
  collection: 'client_chat_basic_ts',
  record: {
    id: 'QvcVXlHeVaX0gQ0Ra54CeX8yOt2vHauUr-NjmIb2C1IXIqTr4bByL4hLBEz9ZEwcOH5rXGG4owqpvhjUIV--KQ',
    name: 'ekoDB Cloud',
    description: 'Fully managed cloud database service product',
    price: 499
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 99,
    description: 'A high-performance database product with AI capabilities',
    id: 'm4mKzQj58y94_KlbE3leWMTPXFAVO8UZ6k6gj7GUSMaLymo1izl7gRAkrT4HB9PczvincH0sGNfumzezYHeIFQ',
    name: 'ekoDB'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'B3bg5mY6KDdBx914vxlFW3gLwOj2iylbAC7liEO95sGMxXNJTjyaFBivN4GUo11dZhssGRB2iYZIKEdSE7a8MQ',
    description: 'Enterprise edition product with advanced features',
    price: 299,
    name: 'ekoDB Pro'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 5624ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 90
Total tokens: 3503

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Message Stream (SSE) Example (TypeScript) ===

Created session: 67v1SoYBya0n_s2YcBLEGTxPq4ZMWr10ZQt5h78cjZusv4re5F73SLVWL-tnX_suG4yP9WFkYEujy6DIJe8i_w

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}**ekoDB** is a curated ecological database developed to support ecological research, data analysis, and modeling. Although there may be several uses of the term, in the context of ecology and environmental sciences, **ekoDB** generally refers to:

- **A Comprehensive Ecological Data Repository**: ekoDB collects, organizes, and provides access to a wide range of ecological datasets. These may include information on species distributions, trait data, biodiversity, community structure, environmental variables, biotic interactions, and more.

- **A Tool for Data Analysis and Modeling**: ekoDB often includes tools or an API that lets users easily integrate ecological data into their research workflows, facilitating tasks such as ecological modeling, conservation planning, or macroecological analysis.

- **Curated and Standardized Data**: Data in ekoDB is curated and standardized, which helps prevent issues with data quality, format inconsistencies, and facilitates reproducibility in science.

**Example:**
One prominent example of "ekodb" is the R package [`ekodb`](https://cran.r-project.org/web/packages/ekodb/index.html), which is designed as a general-purpose ecological knowledge base for species interactions, traits, and ecological knowledge. It provides an infrastructure for storing, accessing, and querying ecological data in R.

**In summary:**
ekoDB is an ecological knowledge database or platform that makes it easier to access, share, and analyze ecological data, usually through a software library or an online portal, and is particularly valuable for ecologists, conservationists, and data scientists working with ecological or biodiversity data.

**If you have a specific context or platform in mind related to ekoDB, please provide more details for a more targeted answer.**

--- Stream complete ---
Message ID: hjMriqwwY7GH0vK-h6eq4_R23_hHpRMET3cUT7OlVRvGetR-F1Tzgy9RUGY8BrLas5loSLfECjRSosqQNBfchA
Execution time: 3480ms
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
✓ Created session: fFrkYi28bme2YC4I365Ly17eYYh_hk5D1ICdEQpNRE8lC8E5bB1nAzzMiowmMvJvcUaKr1v_XrfoUwciBA-o-w

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** A high-performance database product
- **Price:** $99

If you need more information or additional products, let me know!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 2I6hbzf3jTTo2g4IwfrZLEa6ucytZ_haxfXWMjCGZnr9vbIvyqpgTnqocWiw4auX-AMXVaaSrv4X9Jzy0KGJ6A
  Parent: fFrkYi28bme2YC4I365Ly17eYYh_hk5D1ICdEQpNRE8lC8E5bB1nAzzMiowmMvJvcUaKr1v_XrfoUwciBA-o-w

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: 2I6hbzf3jTTo2g4IwfrZLEa6ucytZ_haxfXWMjCGZnr9vbIvyqpgTnqocWiw4auX-AMXVaaSrv4X9Jzy0KGJ6A (Untitled)
  Session 2: fFrkYi28bme2YC4I365Ly17eYYh_hk5D1ICdEQpNRE8lC8E5bB1nAzzMiowmMvJvcUaKr1v_XrfoUwciBA-o-w (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 2I6hbzf3jTTo2g4IwfrZLEa6ucytZ_haxfXWMjCGZnr9vbIvyqpgTnqocWiw4auX-AMXVaaSrv4X9Jzy0KGJ6A

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: t0LKdxvj-jI-9qfNdjX8wpNoXnw0Jo0wLPZgdKrotd__l15luj82SANe5pivvbvkk_ZU9emTgVjFcdy-EBm-xA

=== List Collections ===
Total collections: 17
Sample collections: chat_goals__ek0_testing,schedules__ek0_testing,client_collection_management_ts,schema_products_client_go,agent_function_versions__ek0_testing

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
  POST /api/functions/conc_demo_pay_ts_67690_1789591578851 { "idempotency_key": "...", "amount": 100 }
  POST /api/functions/conc_demo_rl_fail_ts_67690_1789591578851 { "user_id": 42 }
  POST /api/functions/conc_demo_rl_skip_ts_67690_1789591578851 { "user_id": 42 }
  POST /api/functions/conc_demo_lock_ts_67690_1789591578851 { "resource": "queue:drain" }

✓ Cleaned up demo functions
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'XDdF0edDtfgx1U9ZWxPmYB--fRFc5nJrLVy0LCzKAToUEwTLN5R_IRvEXg9iM9eUZF-5VnQWwpZ2HH1S8qDigw'
}

=== Upsert Operation ===
✓ First upsert (update): {
  id: 'XDdF0edDtfgx1U9ZWxPmYB--fRFc5nJrLVy0LCzKAToUEwTLN5R_IRvEXg9iM9eUZF-5VnQWwpZ2HH1S8qDigw',
  age: { value: 29, type: 'Integer' },
  active: { type: 'Boolean', value: true },
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  name: { type: 'String', value: 'Alice Johnson' }
}
✓ Second upsert (insert): {
  id: 'rV2nJW3CwBCh8v-Qjw3pfxGQ9sdnQJPdZUIB376yvcQsJBArS54YeHUVfDePDaxvFNd3fQEtxvsVLI9bUdmBaw'
}

=== Find One Operation ===
✓ Found user by email: {
  name: { value: 'Alice Johnson', type: 'String' },
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  id: 'XDdF0edDtfgx1U9ZWxPmYB--fRFc5nJrLVy0LCzKAToUEwTLN5R_IRvEXg9iM9eUZF-5VnQWwpZ2HH1S8qDigw',
  active: { value: true, type: 'Boolean' },
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
✓ Inserted document: 97Bb4GtSSe_toM5d7V53AqbZWyssHzmOXBMN5g6_3tyQtSyaIGD5SC1KBo_c2ZO_ddOcMrRQb-tqpPZCfc4WwA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: IEpvfCotH_J1uuT4pMulXG_lakbDX1xEyfcLDG7-kn5v3B9yJ4zBudRj0IgNiK8D_RE-Ipb8UPHJwwslNftOsw

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
✓ Edge cache script created: KjnfpDI5ey3lt2wmkVIi3rNg1K214w77g7SSEbkCFHh5_NteFaxlLjpDbLteEobwv8eFFxXjW0LqHf6m70l7Gw

Call 1: Cache miss (fetches from API)
Response time: 69ms
Result: {
  "records": [
    {
      "value": {
        "value": {
          "phone": "1-770-736-8031 x56442",
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets"
          },
          "name": "Leanne Graham",
          "id": 1,
          "address": {
            "zipcode": "92998-3874",
            "suite": "Apt. 556",
            "street": "Kulas Light",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "city": "Gwenborough"
          },
          "website": "hildegard.org",
          "username": "Bret",
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
Response time: 3ms (23x faster!)
Result: {
  "records": [
    {
      "value": {
        "value": {
          "phone": "1-770-736-8031 x56442",
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets"
          },
          "name": "Leanne Graham",
          "id": 1,
          "address": {
            "zipcode": "92998-3874",
            "suite": "Apt. 556",
            "street": "Kulas Light",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "city": "Gwenborough"
          },
          "website": "hildegard.org",
          "username": "Bret",
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
   ⏱️  Duration: 69ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "email": "Sincere@april.biz",
      "website": "hildegard.org",
      "company": {
        "bs": "harness real-time e-markets",
        "name"...

Second call (cache hit - from cache):
   ⏱️  Duration: 2ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "email": "Sincere@april.biz",
      "website": "hildegard.org",
      "company": {
        "bs": "harness real-time e-markets",
        "name"...
   🚀 Cache speedup: 34.5x faster!

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

✅ Function saved: cD2V9m9BoKZAsVE7mGlyM6KFhZvhX-mU9g8tpgbLJ1loQGn4nlXG-Y8rDZnn8ouy9bwVabmYb_Z3F-LedHkmaA
📊 Found 5 active users

📝 Example 2: Parameterized Function

✅ Function saved: x7LrG8YLGsJGts9vb5BTdq6XZMGv8NdyxiEfP3pGgw0HONzMCU8S_aR5LYp4EH3ZSwj2gXFhmELdn2NTI_WShA
📊 Found 3 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: 2_g4cBQO5H9eBMEntPQ3V0JlLSyhImt7R7YX4numVvcutrTHczXYvFEH9yxsFUZ2qj43SVW2ovnAxKgKFXt63Q
📊 Statistics: 2 groups
   {"count":{"type":"Integer","value":5},"status":{"type":"String","value":"active"},"avg_score":{"type":"Float","value":60}}
   {"count":{"value":5,"type":"Integer"},"avg_score":{"type":"Float","value":50},"status":{"value":"inactive","type":"String"}}

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
   {"count":{"value":3,"type":"Integer"},"avg_price":{"value":365.6666666666667,"type":"Float"},"category":{"value":"Furniture","type":"String"}}
   {"avg_price":{"type":"Float","value":367},"category":{"value":"Electronics","type":"String"},"count":{"type":"Integer","value":5}}
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

1. **Efficient Similarity Search**: Enable fast retrieval of similar items based on vector embeddings, ideal for applications like recommendation systems and image search.

2. **High Dimensionality**: Handle high-dimensional data effectively, which is common in machine learning and natural language processing.

3. **Scalability**: Designed to scale with large datasets, allowing for real-time querying and processing.

4. **Flexible Data Types**: Support various data types such as text, images, and audio, making them versatile for diverse applications.

5. **Enhanced Performance**: Optimize queries using techniques like approximate nearest neighbor search to improve speed and reduce resource consumption.

6. **Integration with AI**: Easily integrate with machine learning frameworks, facilitating workflow for AI applications.

7. **Rich Metadata Support**: Often allow for the inclusion of additional metadata alongside vector data, enhancing context for searches.

8. **Semantic Understanding**: Enable understanding of semantic relationships and meanings in data through embeddings.

Overall, they are essential for modern AI-driven applications requiring efficient and effective data retrieval.
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

✅ Function saved: 2ihKmqVtRqN2X7jeY33UOOYl6_Pu-T6uZpICs7d1dFFuP4Wfzhm8_AMg0GCFT2tl3MW0mqRaP3PZK_xu5y0buA
📊 Found 2 product groups
   {"count":{"value":3,"type":"Integer"},"category":{"type":"String","value":"Electronics"},"avg_price":{"value":575.6666666666666,"type":"Float"}}
   {"category":{"type":"String","value":"Furniture"},"count":{"value":2,"type":"Integer"},"avg_price":{"type":"Float","value":474}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"category":{"value":"Electronics","type":"String"},"count":{"type":"Integer","value":3}}
   {"count":{"type":"Integer","value":2},"category":{"type":"String","value":"Furniture"}}
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
   {"count":{"value":7,"type":"Integer"},"role":{"value":"user","type":"String"},"avg_score":{"type":"Float","value":70}}
   {"count":{"type":"Integer","value":3},"avg_score":{"value":20,"type":"Float"},"role":{"type":"String","value":"admin"}}
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

✅ Inserted order: GmPSy5NyqSkMXjS0Thga5IsnReQIMZfj9_aBRRMGemE2VxNwM0WfSoG92XCBjU4GIhqKQsg7DIYW8SwgTtnCxA
✅ Inserted 2 products with wrapped types

📝 Example 2: UserFunction with Wrapped Type Parameters

✅ Function saved: oY1R8jcpjTO19vt_gopCkdOnejYZSKEAPXRtQAQWa7x5JSwNy0F3c_b4dtZ5J_LNELUnmdfGYU03X6dgAV_RfQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"value":{"userId":"user_abc","role":"admin"},"type":"Object"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: fwc-RuU06ehUMrOpCmxsqztB9EqKqvtk3QBP-CjMtBIVkIYqbjwPyjwD9c9JSPloEtaIIkoHJUXnjStSSdgPdw
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: 3bLLmFbdSiqdGipgtGvhUIWxYPKlszZSys7-Y7eUZSQmPigDOd_d_Jo9-WJwoaK-r9vCLmTMRziAXmgk2tDHqw
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
   2. Vector Databases Explained (Database)
   3. Introduction to Machine Learning (AI)
   4. Natural Language Processing (AI)
   5. Database Design Principles (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"category":{"value":"AI","type":"String"},"count":{"type":"Integer","value":2}}
   {"category":{"type":"String","value":"Database"},"count":{"value":3,"type":"Integer"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Goal Template CRUD Example (TypeScript) ===

--- Creating goal template ---
Created template: Data Migration (id: MCB2oHFWRBO5pKhcZHb723rnobhDBgJDhiwlPXCVpq9-1C02XdIJJzOmTnvViLfT9YjLBlAZi04sdN3Avy3MNA)

--- Listing templates ---
Templates: {
  count: 1,
  items: [
    {
      description: [Object],
      id: 'MCB2oHFWRBO5pKhcZHb723rnobhDBgJDhiwlPXCVpq9-1C02XdIJJzOmTnvViLfT9YjLBlAZi04sdN3Avy3MNA',
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
Created goal: Deploy v2.0 (id: 7L3h1mT9ecVvIf_eDJR4-KVf39jQrC6Dx1CZBuaz0swOJtxXYo0MOYpukgrLzeiGLoHER48hrg75c0QmeFkWsA)

--- Listing goals ---
Goals: {
  "count": 1,
  "goals": [
    {
      "created_at": "2026-09-16T20:46:28.990550+00:00",
      "description": "Ship version 2.0 to production",
      "id": "7L3h1mT9ecVvIf_eDJR4-KVf39jQrC6Dx1CZBuaz0swOJtxXYo0MOYpukgrLzeiGLoHER48hrg75c0QmeFkWsA",
      "status": "pending",
      "steps": "[{\"description\":\"Run test suite\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]",
      "title": "Deploy v2.0",
      "updated_at": "2026-09-16T20:46:28.990550+00:00"
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
        "value": "2026-09-16T20:46:28.990550+00:00"
      },
      "description": {
        "type": "String",
        "value": "Ship version 2.0 with hot-fix patches"
      },
      "id": "7L3h1mT9ecVvIf_eDJR4-KVf39jQrC6Dx1CZBuaz0swOJtxXYo0MOYpukgrLzeiGLoHER48hrg75c0QmeFkWsA",
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
        "value": "2026-09-16T20:46:29.001495+00:00"
      }
    }
  ]
}

--- Goal step: start step 0 ---
Step 0 started on goal 7L3h1mT9ecVvIf_eDJR4-KVf39jQrC6Dx1CZBuaz0swOJtxXYo0MOYpukgrLzeiGLoHER48hrg75c0QmeFkWsA
--- Goal step: complete step 0 ---
Step 0 completed on goal 7L3h1mT9ecVvIf_eDJR4-KVf39jQrC6Dx1CZBuaz0swOJtxXYo0MOYpukgrLzeiGLoHER48hrg75c0QmeFkWsA
--- Goal step: fail step 1 ---
Step 1 failed on goal 7L3h1mT9ecVvIf_eDJR4-KVf39jQrC6Dx1CZBuaz0swOJtxXYo0MOYpukgrLzeiGLoHER48hrg75c0QmeFkWsA

--- Completing goal ---
Goal status: pending_review
--- Approving goal ---
Goal status after approve: in_progress

--- Creating goal to reject ---
--- Rejecting goal ---
Goal status after reject: failed


--- Creating task ---
Created task: Hourly Health Check (id: CedjLvDx8PbBzQzeCrbe_PBfKbDpscDCFgLA6m-g4yjs1DYwlsD8uRLbZ9WEfcmsV6nQoTvxTnlbpiB6kLee7g)

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
      "id": "CedjLvDx8PbBzQzeCrbe_PBfKbDpscDCFgLA6m-g4yjs1DYwlsD8uRLbZ9WEfcmsV6nQoTvxTnlbpiB6kLee7g",
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
Created agent: SupportBot (id: NE_HAsIqkS4ZID1xOnX3rKHosUA1Y3v8eNnNQemqPM7K1wqQS7pxqwhnS_PzegT44yl8M-BVvV8WjPGxmtPduw)

--- Listing agents ---
Agents: {
  "count": 1,
  "items": [
    {
      "deployment_id": {
        "type": "String",
        "value": "deploy_prod_1"
      },
      "id": "NE_HAsIqkS4ZID1xOnX3rKHosUA1Y3v8eNnNQemqPM7K1wqQS7pxqwhnS_PzegT44yl8M-BVvV8WjPGxmtPduw",
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
By name: SupportBot (id: NE_HAsIqkS4ZID1xOnX3rKHosUA1Y3v8eNnNQemqPM7K1wqQS7pxqwhnS_PzegT44yl8M-BVvV8WjPGxmtPduw)

--- Updating agent ---
Updated agent: SupportBot

--- Getting agents by deployment ---
Agents in deployment: {
  "count": 0,
  "items": []
}
WARNING: agents-by-deployment omitted created agent NE_HAsIqkS4ZID1xOnX3rKHosUA1Y3v8eNnNQemqPM7K1wqQS7pxqwhnS_PzegT44yl8M-BVvV8WjPGxmtPduw; TODO: check/fix the server-side deployment lookup

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
  POST /api/functions/jwt_register_ts_68440_1789591590076 { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/jwt_login_ts_68440_1789591590076 { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/jwt_verify_ts_68440_1789591590076 { "token": "<jwt>" }

Set JWT_SECRET in ekoDB's environment_vars whitelist before invoking.

✓ Cleaned up demo functions
=== ekoDB KV Document Linking Example (TypeScript) ===

--- Setup: creating KV key and documents ---
Set KV key: session:admin
Inserted document 1: 39KI33T5Crwilgsy_XPOuPqUzZGCcjmK5GUMwlmEd-toDjZbZym9rk3y7KCIBu-i-ctmdinAQJZWn_7w9bHU-A
Inserted document 2: 24eEE4gU81mj0ddsoULx6RR86R3CqCWXx6q5ks-NgqV-UcvBAStdPqoLoIKmtj6SfBZmYRTGehb0Sh6UUQGsWA

--- Linking documents to KV key ---
Linked doc 39KI33T5Crwilgsy_XPOuPqUzZGCcjmK5GUMwlmEd-toDjZbZym9rk3y7KCIBu-i-ctmdinAQJZWn_7w9bHU-A: null
Linked doc 24eEE4gU81mj0ddsoULx6RR86R3CqCWXx6q5ks-NgqV-UcvBAStdPqoLoIKmtj6SfBZmYRTGehb0Sh6UUQGsWA: null

--- Getting links for KV key ---
Links: [
  {
    "collection": "kv_links_example_ts_68475_1789591590426",
    "document_id": "39KI33T5Crwilgsy_XPOuPqUzZGCcjmK5GUMwlmEd-toDjZbZym9rk3y7KCIBu-i-ctmdinAQJZWn_7w9bHU-A",
    "field_path": null,
    "created_at": "2026-09-16T20:46:30.521535Z",
    "last_accessed": "2026-09-16T20:46:30.523812Z",
    "metadata": {}
  },
  {
    "collection": "kv_links_example_ts_68475_1789591590426",
    "document_id": "24eEE4gU81mj0ddsoULx6RR86R3CqCWXx6q5ks-NgqV-UcvBAStdPqoLoIKmtj6SfBZmYRTGehb0Sh6UUQGsWA",
    "field_path": null,
    "created_at": "2026-09-16T20:46:30.522765Z",
    "last_accessed": "2026-09-16T20:46:30.523812Z",
    "metadata": {}
  }
]

--- Unlinking document ---
Unlinked doc 24eEE4gU81mj0ddsoULx6RR86R3CqCWXx6q5ks-NgqV-UcvBAStdPqoLoIKmtj6SfBZmYRTGehb0Sh6UUQGsWA: null

--- Verifying remaining links ---
Remaining links: [
  {
    "collection": "kv_links_example_ts_68475_1789591590426",
    "document_id": "39KI33T5Crwilgsy_XPOuPqUzZGCcjmK5GUMwlmEd-toDjZbZym9rk3y7KCIBu-i-ctmdinAQJZWn_7w9bHU-A",
    "field_path": null,
    "created_at": "2026-09-16T20:46:30.521535Z",
    "last_accessed": "2026-09-16T20:46:30.525977Z",
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
  kv_ops_ts_68505_1789591590884:cache:product:1: success
  kv_ops_ts_68505_1789591590884:cache:product:2: success
  kv_ops_ts_68505_1789591590884:cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  kv_ops_ts_68505_1789591590884:cache:product:1: { name: 'Product 1', price: 29.99 }
  kv_ops_ts_68505_1789591590884:cache:product:2: { name: 'Product 2', price: 39.99 }
  kv_ops_ts_68505_1789591590884:cache:product:3: { price: 49.99, name: 'Product 3' }

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
  kv_ops_ts_68505_1789591590884:cache:product:1: deleted
  kv_ops_ts_68505_1789591590884:cache:product:2: deleted
  kv_ops_ts_68505_1789591590884:cache:product:3: deleted

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
Blocking response: Hello! I hope you’re having a wonderful day.

=== Done ===
=== ekoDB Schedule Management Example (TypeScript) ===

--- Creating schedule ---
Created schedule: Nightly Database Backup (id: 093b4f3d-7f60-4f0a-bf48-e017478b8056, cron: 0 0 2 * * *)

--- Listing schedules ---
Schedules: {
  "count": 1,
  "schedules": [
    {
      "created_at": "2026-09-16T20:46:35.315401Z",
      "cron_expression": "0 0 2 * * *",
      "description": null,
      "enabled": true,
      "function_label": "schedule_noop_typescript_68668_1789591595284",
      "id": "093b4f3d-7f60-4f0a-bf48-e017478b8056",
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
      "updated_at": "2026-09-16T20:46:35.315401Z"
    }
  ]
}

--- Getting schedule ---
Fetched: Nightly Database Backup (cron: 0 0 2 * * *)

--- Updating schedule ---
Updated: Nightly Full Backup (new cron: 0 0 3 * * *)

--- Triggering schedule ---
Trigger response: {
  "schedule_id": "093b4f3d-7f60-4f0a-bf48-e017478b8056",
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
  1. Score: 0.773, Matched:
  2. Score: 0.731, Matched:
  3. Score: 0.720, Matched:

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.488, Matched: title, content
  2. Score: 0.893, Matched: title, content
  3. Score: 0.309, Matched:

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: skills, bio

9. Vector search with a metadata pre-filter (category = ml):
Found 2 documents in category "ml" (NLP excluded)
  1. Deep Learning Fundamentals (category: ml)
  2. Introduction to Machine Learning (category: ml)


✅ Search examples completed!
=== Cleanup ===
✅ Deleted test collections
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'hyFbi-v9QQDMg8NPP5Hhlqzn74H6GFJ6gwy5PWhIPHmxyTt5VWdJVPqOkbomNsOSM1vJdOu1IEHdsw3c-Iu5jA'
}

=== Find by ID ===
Found: {
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  name: { type: 'String', value: 'Test Record' },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  created_at: { type: 'DateTime', value: '2026-09-16T20:46:36.660+00:00' },
  value: { value: 42, type: 'Integer' },
  id: 'hyFbi-v9QQDMg8NPP5Hhlqzn74H6GFJ6gwy5PWhIPHmxyTt5VWdJVPqOkbomNsOSM1vJdOu1IEHdsw3c-Iu5jA',
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  price: { type: 'Float', value: 99.99 },
  metadata: { value: { nested: [Object], key: 'value' }, type: 'Object' },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  active: { type: 'Boolean', value: true }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-09-16T20:46:36.660Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { nested: { deep: true }, key: 'value' }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  data: 'aGVsbG8gd29ybGQ=',
  name: 'Test Record',
  categories: [ 'electronics', 'computers' ],
  created_at: '2026-09-16T20:46:36.660+00:00',
  value: 42,
  id: 'hyFbi-v9QQDMg8NPP5Hhlqzn74H6GFJ6gwy5PWhIPHmxyTt5VWdJVPqOkbomNsOSM1vJdOu1IEHdsw3c-Iu5jA',
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  tags: [ 'tag1', 'tag2', 'tag3' ],
  price: 99.99,
  metadata: { nested: { deep: true }, key: 'value' },
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  active: true
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  active: { type: 'Boolean', value: true },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  id: 'hyFbi-v9QQDMg8NPP5Hhlqzn74H6GFJ6gwy5PWhIPHmxyTt5VWdJVPqOkbomNsOSM1vJdOu1IEHdsw3c-Iu5jA',
  price: { type: 'Float', value: 99.99 },
  value: { value: 100, type: 'Integer' },
  name: { value: 'Updated Record', type: 'String' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  metadata: { value: { nested: [Object], key: 'value' }, type: 'Object' },
  created_at: { value: '2026-09-16T20:46:36.660+00:00', type: 'DateTime' },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: YLBLGDcd36AtEOFXkM3Tvun6C20hA_oFFqAa-hHl59PE8qYlU235zCzLks1eCNoSXAai5wqQjGlyel0RZIuGRQ

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
✓ Created native SWR script: github_user_native_ts (SfyfnK1wf2Vy4baTcbrWmZsVhnYvpYwz9D9QTnuWWx-HkUaAdOFrl5Ky0lZdLdM78XCJnGWmFMAja57uD8UtMw)

First call (cache miss - will fetch from GitHub API):
  Response time: 482ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 3ms
  Speedup: 160.7x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_ts (HhegY1bBBiAxgqW9jTaB2qtmwJGodkDkjSK6rLlQHxgqxCaM-SyTBbTLvUdNT60kQCvvnyD_e7YvvAbYPFTrcQ)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_ts' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_ts (72M6S_phU2tIOPpleluuHFjPe0caosFzFPaDtHUDfAnK_cCBQ--MJe0p6yDu-r4DRg6DTtFOgGxKxYo8NtWUtw)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_ts' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_ts (aJbRwRO-rd4laSbrHvsRsEYm1MCL8s6wmj9EFJfZ5uWA-UUyTde2zDj_ExbhSGMa04ooyoDRikDKYxm_odiaqg)
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
✓ Created SWR script: fetch_api_user_ts_68859_1789591599098 (oi1XBJRnnQTprjMZi1w8s9A1yPFzaoLTC46geZXgHaBm7Snm9w4t__4nxUFuxMymNr8QC1I7UE8EVO4hFaABVg)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "email": "Sincere@april.biz",
          "website": "hildegard.org",
          "company": {
            "name": "Romaguera-Crona",
            "catchPhrase": "Multi-layered client-server neural-net",
            "bs": "harness real-time e-markets"
          },
          "username": "Bret",
          "phone": "1-770-736-8031 x56442",
          "id": 1,
          "address": {
            "city": "Gwenborough",
            "zipcode": "92998-3874",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "street": "Kulas Light",
            "suite": "Apt. 556"
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
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 2ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "value": {
          "email": "Sincere@april.biz",
          "website": "hildegard.org",
          "company": {
            "name": "Romaguera-Crona",
            "catchPhrase": "Multi-layered client-server neural-net",
            "bs": "harness real-time e-markets"
          },
          "username": "Bret",
          "phone": "1-770-736-8031 x56442",
          "id": 1,
          "address": {
            "city": "Gwenborough",
            "zipcode": "92998-3874",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "street": "Kulas Light",
            "suite": "Apt. 556"
          },
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
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_reviews_ts_68859_1789591599098 (1qiuv4ebCd21HKRj9rxOAySdFShIrJoyUAaBzQTlEzYjbryjjoNVZaBz5O6nds-Qa_l8BC6ejE5MCihGRrOcxw)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "value": {
          "weight": 4,
          "warrantyInformation": "1 week warranty",
          "category": "beauty",
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "returnPolicy": "No return policy",
          "stock": 99,
          "brand": "Essence",
          "dimensions": {
            "width": 15.14,
            "depth": 22.99,
            "height": 13.08
          },
          "meta": {
            "barcode": "5784719087687",
            "createdAt": "2025-10-09T14:47:01.588Z",
            "updatedAt": "2026-05-23T11:27:41.868Z",
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png"
          },
          "minimumOrderQuantity": 48,
          "rating": 2.56,
          "discountPercentage": 10.48,
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "shippingInformation": "Ships in 3-5 business days",
          "availabilityStatus": "In Stock",
          "title": "Essence Mascara Lash Princess",
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "tags": [
            "beauty",
            "mascara"
          ],
          "price": 9.99,
          "id": 1,
          "sku": "BEA-ESS-ESS-001",
          "reviews": [
            {
              "date": "2025-04-30T09:41:02.053Z",
              "comment": "Would not recommend!",
              "rating": 3,
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "reviewerName": "Eleanor Collins"
            },
            {
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerEmail": "lucas.gordon@x.dummyjson.com",
              "rating": 4,
              "reviewerName": "Lucas Gordon",
              "comment": "Very satisfied!"
            },
            {
              "comment": "Highly impressed!",
              "reviewerName": "Eleanor Collins",
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "rating": 5
            }
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

✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: I2eaEoX_PYiUyPacFGhYmNnB8JeS9Hy4HbH52jAW4jvnMf1C84yBeHftQ2VW36EemT6EUnojk6i8tsO3zo_EXQ
Created Bob: $500 - ID: hEjGdzSCMTBGAcP1sifrBtNTCaEZ5jQigeUPCgDENoSJV-3hyTJYV6VybjDAbc0f8WigCB1-elG_EJPwsiZxaQ

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 73f926ec-75bf-4b64-b399-d0598476f8eb

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
New transaction: b426bb99-2b0f-4e49-84f8-42aa51507cd2
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Create User Function ===
Created user function with ID: 667ouiTuRngOR-9_3I7Jct8RC36Ml7YnT_wBvVJxjXeTIdIe-0BTC1myF4HYrFXtsJ9JeX-q1mwJ6hMgWg8Kwg

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 5 user functions:
  - fetch_product_reviews_ts_68859_1789591599098: Fetch Product with Reviews (Multi-API)
  - conc_demo_rl_skip_ts_67690_1789591578851: Rate-limit (skip mode)
  - get_active_users_ts: Get Active Users
  - conc_demo_rl_fail_ts_67690_1789591578851: Rate-limit (fail mode)
  - get_active_users_client_ts_updated: Get Active Users (Updated)

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
=== WebSocket Chat Streaming Example (TypeScript) ===

Created chat session: 16cYPTcV5eqr2k1LL0FMGGhXs72oVy6yisJCVDN0kKpkLWwC7PrysYUpdggVesFOgsV5B3FTa2EyAV3KlAiHuA

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: jGDWC8yq5ktqJdE4-UZO-WqoC3u3wuyQMp9Ek-2FM6AUiDu7iTBNx-kB0JYOUjxES2lqMU_ec8cG3JQmtzd_lg
Execution time: 543ms
Token usage: {"completion_tokens":8,"prompt_tokens":15,"total_tokens":23}

Full response: {"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris....
=== WebSocket Subscription Example ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_ts' ===
✓ Subscribed (subscription_id: sub_b21b03b58b784e9c85b5366094b86c5a)

=== Performing mutations to trigger notifications ===
Inserting a record...
✓ Inserted record: zQK6JbCAGDGb_5p-TOfMM6ZJ9MeVIgtZf3Ehj_zPrUcW0Td2BIbYau009T6kR93Wawavi62g0i8vcKTn02BzmA
  📡 Notification received for zQK6JbCAGDGb_5p-TOfMM6ZJ9MeVIgtZf3Ehj_zPrUcW0Td2BIbYau009T6kR93Wawavi62g0i8vcKTn02BzmA

Inserting another record...
✓ Inserted record: BG91MUi138Z2RCOsuQ59hDAUZv64wx7jcsLB_2TQJo7xMQUfoAEMpReYa2t9bzrZxtY8V1ntWNOSf3TuefiekA
  📡 Notification received for BG91MUi138Z2RCOsuQ59hDAUZv64wx7jcsLB_2TQJo7xMQUfoAEMpReYa2t9bzrZxtY8V1ntWNOSf3TuefiekA

=== Unsubscribing ===
✓ Unsubscribed: {"collection":"ws_subscribe_example_ts","found":true,"unsubscribed":true}

✓ WebSocket subscription example completed successfully
✓ Deleted collection 'ws_subscribe_example_ts'
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: 3nJHvaPt9Jbwy8Z-u7_--mUd4SnoqGQkc11VlrsI0jfyZCGPewxRalbmPeHoA2uuEvszlgsByj1CQOFYa42yKg

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
   Inserted with ripple: {"id":"12-xM16Gbq7NcRLRsFEYsbBAwzXT8PREgWmqnNScD7LlDl2UTRCLub_T24paR1s7ywqDRy8ZHpoTt5BVcOBXwQ"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"Zxq5teP2yRWzZRO3sc3cZfNBgfo9SHuBVlhubFGvIY8rTGLPQQBG894-9IX9eL7KCBTVUzBcTA45fTWbLNZuPg"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"name":{"type":"String","value":"Product 1"},"price":{"type":"Integer","value":150},"id":"12-xM16Gbq7NcRLRsFEYsbBAwzXT8PREgWmqnNScD7LlDl2UTRCLub_T24paR1s7ywqDRy8ZHpoTt5BVcOBXwQ"}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"id":"m8Rk4ojaO2dFda0-s5SAFWkZDADVDk24JzQ8XYZMY79rO-pCcO7x09bitea8ajDsQFjAoVuOq57izKuj1cBEhg"}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ["name","email","id"]
  First user: Alice Johnson <alice@example.com>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["avatar_url","id","created_at","email","age","user_role","name","status","bio"]

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
    - Fields: ["created_at","name","email","age","secret_token","avatar_url","bio","user_role","password","id","api_key","status"]
  Projected query:
    - 3 fields per record
    - Fields: ["name","email","id"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
