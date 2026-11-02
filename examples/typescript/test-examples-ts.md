make test-examples-typescript
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
