make test-examples-typescript
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
