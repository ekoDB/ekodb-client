make test-examples-typescript
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
