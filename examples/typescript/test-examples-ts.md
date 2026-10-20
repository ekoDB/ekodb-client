make test-examples-typescript
📦 Building TypeScript client library...

> @ekodb/ekodb-client@0.30.1 prepare
> npm run build


> @ekodb/ekodb-client@0.30.1 build
> tsc


up to date, audited 44 packages in 838ms

12 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.30.1 build
> tsc

✅ TypeScript client built!

added 1 package, removed 1 package, and audited 13 packages in 474ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
Admin permissions read successfully
=== ekoDB Advanced CRUD Example (TypeScript) ===

--- Inserting base record ---
Inserted: CuHLqz3sakZqp74DHOkndnFyScACJq_SJH_gKpVvd4iZiFz6oeR7oApto6iAFQnAzfZGM7dVnNIXES9Kkr8Xpg

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
✓ Created session: q7tV2d87MyEowLbaBgTW5yCm4xOyR3qJY5XhPF03vnt_uY9Xl5FHH7_sp6DoyyZ_BnI-o4GSsbKv3DkvfwgnLA

=== Sending Initial Message ===
✓ Message sent
  Response: It looks like there are currently no available products listed in the database. The only product context I have is the "High-performance database product" named "ekoDB" with an ID of ZOF0HOhlD2ZXllOEQq5FxdIfx92DwjhU-kp4DKQIS-QRTdUdq87joeYeuKXrGx9Vy4xe4t6ABKwrqPJPKzMkEQ, priced at $99.

If you would like more details or need to perform another action, please let me know!

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
✓ Created second session: pXlwy_3n7NHjsqyUQFlPOk9t2r-szPyGt4Agljkt-l4m26IFFxdzWrBRI6vbaMFixw23U1vcqZv38irlG_gsjw
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: pXlwy_3n7NHjsqyUQFlPOk9t2r-szPyGt4Agljkt-l4m26IFFxdzWrBRI6vbaMFixw23U1vcqZv38irlG_gsjw
✓ Deleted chat session: q7tV2d87MyEowLbaBgTW5yCm4xOyR3qJY5XhPF03vnt_uY9Xl5FHH7_sp6DoyyZ_BnI-o4GSsbKv3DkvfwgnLA
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: VYt2UhwMm2tqFT_cqWCKpiGGEdDCgWnQXvV7BKKNZFNXOEm9DgbermPQq-e0inKQ7hUxSaUrj7_GOZN5T_T75A

=== Sending Chat Message ===
Message ID: De_j6nLki9caiBTx4BC_wq7PtkA6DmNjl-Q4-6nfsZefrQUS8750hljSqPK7thdde9pypzXAes00ZJ6O80VfSA

=== AI Response ===
Here are the products available along with their prices:

1. **ekoDB Pro**
   - Price: $299
   - Description: Enterprise edition product with advanced features

2. **ekoDB Cloud**
   - Price: $499
   - Description: Fully managed cloud database service product

3. **ekoDB**
   - Price: $99
   - Description: A high-performance database product with AI capabilities

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 299,
    name: 'ekoDB Pro',
    description: 'Enterprise edition product with advanced features',
    id: 'N4m7Umfa6LB3yM8aC_s3u5oQvrh99q-zQouGwUysF_UipT-lDFaiVi3fvtzYUO2VgZOx162yf1Px-EOETIwiYw'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'gjGsoHsUc9531yqTBfklqg_EBk6xZTmI-rz17sKMqvVJ74kouCE5zWxziF5vcIJ86bD-aET9TFLLE3B6E8T7zg',
    price: 499,
    description: 'Fully managed cloud database service product',
    name: 'ekoDB Cloud'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'A high-performance database product with AI capabilities',
    price: 99,
    id: 'oFTUoNTAdK1W_YfcdPwpKSJRa8s7orTaoP1GJZJS5n9IsyR_bHwKNDx_1qhw0Ra1107elPIObAlfJdxrMUWqow',
    name: 'ekoDB'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2980ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 91
Total tokens: 3504

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Message Stream (SSE) Example (TypeScript) ===

Created session: 4lT_7FnzchMBOPc6uf0jqjC1qanH3RzHjuCHXNxq3YMB5FF9WFoBbXaqwrWtzdNOoTPL33nNFmBjktT7tAcQkQ

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}**ekoDB** can refer to a few different things, but the most common contexts are:

1. **ekoDB (Ecological Databases):**
   - This is the broader and more common use. Here, "ekoDB" is shorthand for **ecological database(s)**—collections of data related to environmental science, species, habitats, or ecological phenomena. These databases are used by researchers for biodiversity studies, conservation planning, and environmental monitoring.

2. **ekoDB (Specific Projects):**
   - In some academic or open-source projects, "ekoDB" is used as the proper name for a particular ecological database or data management system. For example:
     - There may be a database or web portal called "ekoDB" devoted to storing and sharing ecological, zoological, or biodiversity data within a certain research community or geographic region.

3. **ekoDB (Bioinformatics/Genomics):**
   - In genetics and bioinformatics, ekoDB is sometimes mentioned as a database or platform for omics data, like gene expression or protein studies, especially involving model organisms.

**Do you have a specific context or region in mind?** (For example, a particular scientific field, a web resource, or a type of data?) If so, please provide more details so I can give you the most accurate answer.

--- Stream complete ---
Message ID: nvJE2JaazmSa0b8K1Gx5wOD8WnEGCJA2c2lWaksMgHvdGFteDKFWTopkrGjD0dI4DIph5FmVs0M1Wmx3jRy1_Q
Execution time: 2629ms
Context window: 1000000 tokens

✓ Chat message stream example completed
✓ Client created

=== List Chat Models ===
Available chat models by provider:
  openai:
    - gpt-4-turbo-2024-04-09
    - gpt-realtime-2.1-mini
    - gpt-realtime-2025-08-28
    - gpt-4
    - tts-1-hd
    - gpt-realtime-1.5
    - gpt-realtime
    - gpt-5.2-pro
    - gpt-5-chat-latest
    - gpt-3.5-turbo-0125
    - gpt-5.4-2026-03-05
    - gpt-5-nano
    - gpt-5.4-mini
    - o1-pro
    - gpt-5.1-codex
    - tts-1
    - gpt-5-mini
    - gpt-5-mini-2025-08-07
    - omni-moderation-2024-09-26
    - gpt-live-transcribe
    - gpt-5.6-luna
    - omni-moderation-latest
    - gpt-5-2025-08-07
    - o4-mini-2025-04-16
    - gpt-4o-2024-11-20
    - gpt-5.1
    - whisper-1
    - gpt-5-pro
    - gpt-realtime-whisper
    - gpt-5.5-pro
    - gpt-audio-2025-08-28
    - gpt-4o-mini-transcribe-2025-03-20
    - gpt-realtime-2
    - gpt-5-search-api
    - gpt-5.5-2026-04-23
    - chat-latest
    - gpt-realtime-translate
    - gpt-5.4-pro-2026-03-05
    - gpt-5.4-mini-2026-03-17
    - babbage-002
    - gpt-4o-2024-05-13
    - text-embedding-3-large
    - gpt-4o-mini-transcribe-2025-12-15
    - tts-1-1106
    - gpt-5.5-pro-2026-04-23
    - gpt-audio-mini-2025-10-06
    - gpt-5.2-pro-2025-12-11
    - gpt-4-turbo
    - gpt-4o-2024-08-06
    - o3
    - gpt-4o-mini-search-preview-2025-03-11
    - o3-mini-2025-01-31
    - gpt-audio-1.5
    - gpt-image-1.5
    - gpt-5-pro-2025-10-06
    - gpt-5.6-sol
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
    - gpt-4.1-mini
    - gpt-4o-mini
    - gpt-5.4-pro
    - gpt-4o-mini-tts
    - gpt-realtime-mini-2025-12-15
    - gpt-5.1-codex-max
    - gpt-realtime-mini
    - gpt-image-1-mini
    - gpt-5.6-terra
    - gpt-3.5-turbo-instruct-0914
    - o1-2024-12-17
    - sora-2
    - tts-1-hd-1106
    - gpt-4o-mini-tts-2025-12-15
    - gpt-image-1
    - gpt-audio
    - gpt-5.3-codex
    - gpt-5-search-api-2025-10-14
    - gpt-5
    - gpt-4o-mini-transcribe
    - gpt-image-2-2026-04-21
    - gpt-5.2-chat-latest
    - sora-2-pro
    - gpt-3.5-turbo-instruct
    - gpt-transcribe
    - gpt-4.1-nano-2025-04-14
    - o4-mini-deep-research
    - gpt-5.4
    - gpt-4o-search-preview-2025-03-11
    - text-embedding-3-small
    - gpt-5.1-chat-latest
    - gpt-4o-mini-2024-07-18
    - gpt-4.1-2025-04-14
    - gpt-5.3-chat-latest
    - o3-2025-04-16
    - gpt-5.2-2025-12-11
    - chatgpt-image-latest
    - gpt-3.5-turbo
    - gpt-5.2-codex
    - gpt-4o
    - davinci-002
    - gpt-5.4-nano
    - o3-mini
    - o4-mini-deep-research-2025-06-26
    - gpt-audio-mini-2025-12-15
    - o1-pro-2025-03-19
    - gpt-5-nano-2025-08-07
    - gpt-4o-transcribe
    - o4-mini
    - gpt-5.1-codex-mini
    - gpt-3.5-turbo-1106
    - gpt-4o-mini-tts-2025-03-20
    - gpt-5.2
    - gpt-5.4-nano-2026-03-17
    - o1
    - gpt-image-2.5-sunburst-2026-09-08
    - gpt-6-luna
    - gpt-audio-mini
    - gpt-4.1-mini-2025-04-14
    - gpt-image-2.5-flare
    - gpt-4o-mini-search-preview
    - gpt-6-sol
    - gpt-6-astra
    - gpt-6.1-sol
    - gpt-live-1
    - gpt-image-2.5-sunburst
    - gpt-image-2.5-flare-2026-09-08
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
OpenAI models: gpt-4-turbo-2024-04-09, gpt-realtime-2.1-mini, gpt-realtime-2025-08-28, gpt-4, tts-1-hd, gpt-realtime-1.5, gpt-realtime, gpt-5.2-pro, gpt-5-chat-latest, gpt-3.5-turbo-0125, gpt-5.4-2026-03-05, gpt-5-nano, gpt-5.4-mini, o1-pro, gpt-5.1-codex, tts-1, gpt-5-mini, gpt-5-mini-2025-08-07, omni-moderation-2024-09-26, gpt-live-transcribe, gpt-5.6-luna, omni-moderation-latest, gpt-5-2025-08-07, o4-mini-2025-04-16, gpt-4o-2024-11-20, gpt-5.1, whisper-1, gpt-5-pro, gpt-realtime-whisper, gpt-5.5-pro, gpt-audio-2025-08-28, gpt-4o-mini-transcribe-2025-03-20, gpt-realtime-2, gpt-5-search-api, gpt-5.5-2026-04-23, chat-latest, gpt-realtime-translate, gpt-5.4-pro-2026-03-05, gpt-5.4-mini-2026-03-17, babbage-002, gpt-4o-2024-05-13, text-embedding-3-large, gpt-4o-mini-transcribe-2025-12-15, tts-1-1106, gpt-5.5-pro-2026-04-23, gpt-audio-mini-2025-10-06, gpt-5.2-pro-2025-12-11, gpt-4-turbo, gpt-4o-2024-08-06, o3, gpt-4o-mini-search-preview-2025-03-11, o3-mini-2025-01-31, gpt-audio-1.5, gpt-image-1.5, gpt-5-pro-2025-10-06, gpt-5.6-sol, gpt-4.1-nano, gpt-image-2, gpt-3.5-turbo-16k, text-embedding-ada-002, gpt-realtime-2.1, gpt-4.1, gpt-5.5, gpt-4o-transcribe-diarize, gpt-5-codex, gpt-4-0613, gpt-5.1-2025-11-13, gpt-4o-search-preview, gpt-4.1-mini, gpt-4o-mini, gpt-5.4-pro, gpt-4o-mini-tts, gpt-realtime-mini-2025-12-15, gpt-5.1-codex-max, gpt-realtime-mini, gpt-image-1-mini, gpt-5.6-terra, gpt-3.5-turbo-instruct-0914, o1-2024-12-17, sora-2, tts-1-hd-1106, gpt-4o-mini-tts-2025-12-15, gpt-image-1, gpt-audio, gpt-5.3-codex, gpt-5-search-api-2025-10-14, gpt-5, gpt-4o-mini-transcribe, gpt-image-2-2026-04-21, gpt-5.2-chat-latest, sora-2-pro, gpt-3.5-turbo-instruct, gpt-transcribe, gpt-4.1-nano-2025-04-14, o4-mini-deep-research, gpt-5.4, gpt-4o-search-preview-2025-03-11, text-embedding-3-small, gpt-5.1-chat-latest, gpt-4o-mini-2024-07-18, gpt-4.1-2025-04-14, gpt-5.3-chat-latest, o3-2025-04-16, gpt-5.2-2025-12-11, chatgpt-image-latest, gpt-3.5-turbo, gpt-5.2-codex, gpt-4o, davinci-002, gpt-5.4-nano, o3-mini, o4-mini-deep-research-2025-06-26, gpt-audio-mini-2025-12-15, o1-pro-2025-03-19, gpt-5-nano-2025-08-07, gpt-4o-transcribe, o4-mini, gpt-5.1-codex-mini, gpt-3.5-turbo-1106, gpt-4o-mini-tts-2025-03-20, gpt-5.2, gpt-5.4-nano-2026-03-17, o1, gpt-image-2.5-sunburst-2026-09-08, gpt-6-luna, gpt-audio-mini, gpt-4.1-mini-2025-04-14, gpt-image-2.5-flare, gpt-4o-mini-search-preview, gpt-6-sol, gpt-6-astra, gpt-6.1-sol, gpt-live-1, gpt-image-2.5-sunburst, gpt-image-2.5-flare-2026-09-08

=== Get Anthropic Models ===
Anthropic models: claude-haiku-5-5, claude-sonnet-5-5, claude-opus-5-5, claude-fable-5-1, claude-opus-5, claude-sonnet-5, claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 404: {"error":"Unknown provider 'nonexistent_provider_xyz'","error_kind":"unknown_provider"}

✓ Chat Models example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: G8fukzJuvGyVk-OheQ6_zEjXmZUy7qTCWx-WXGTOclN9WG97E9Sa6p5vyGG8t-jjQZY36O5MiU6PW1l3FQbfrA

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99

If you need information about more products or anything else, let me know!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 1tbAC-SLuAC-XVHXwss8ucSPkiMKDk0qnNPUuvzNUdKFe11_vsMpB0TFL5Y0n3xCKQA6QqQTLpUN0Y4A-80H5A
  Parent: G8fukzJuvGyVk-OheQ6_zEjXmZUy7qTCWx-WXGTOclN9WG97E9Sa6p5vyGG8t-jjQZY36O5MiU6PW1l3FQbfrA

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: 1tbAC-SLuAC-XVHXwss8ucSPkiMKDk0qnNPUuvzNUdKFe11_vsMpB0TFL5Y0n3xCKQA6QqQTLpUN0Y4A-80H5A (Untitled)
  Session 2: G8fukzJuvGyVk-OheQ6_zEjXmZUy7qTCWx-WXGTOclN9WG97E9Sa6p5vyGG8t-jjQZY36O5MiU6PW1l3FQbfrA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 1tbAC-SLuAC-XVHXwss8ucSPkiMKDk0qnNPUuvzNUdKFe11_vsMpB0TFL5Y0n3xCKQA6QqQTLpUN0Y4A-80H5A

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: f8_rOjtoCUX_85JvsFgcdwMNBjm2G5U8q2YqdscAGXMfEtklkkfsLkVazbUl-CiLmSaU95YNGmrdEYEbbiWBmg

=== List Collections ===
Total collections: 17
Sample collections: chat_goals__ek0_testing,chat_agent_configs__ek0_testing,chat_tasks__ek0_testing,schema_employees_client_go,schema_users_client_go

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
  POST /api/functions/conc_demo_pay_ts_12240_1792526703280 { "idempotency_key": "...", "amount": 100 }
  POST /api/functions/conc_demo_rl_fail_ts_12240_1792526703280 { "user_id": 42 }
  POST /api/functions/conc_demo_rl_skip_ts_12240_1792526703280 { "user_id": 42 }
  POST /api/functions/conc_demo_lock_ts_12240_1792526703280 { "resource": "queue:drain" }

✓ Cleaned up demo functions
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'ya-cDEFl57qAIZxMB9s_eSMNmo0GYUu_KnU3iYteWQoJXz-JXrRziQa0Nz-4iScBYFRpCRq0wo1jLsLIxceEhA'
}

=== Upsert Operation ===
✓ First upsert (update): {
  id: 'ya-cDEFl57qAIZxMB9s_eSMNmo0GYUu_KnU3iYteWQoJXz-JXrRziQa0Nz-4iScBYFRpCRq0wo1jLsLIxceEhA',
  age: { type: 'Integer', value: 29 },
  name: { value: 'Alice Johnson', type: 'String' },
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  active: { type: 'Boolean', value: true }
}
✓ Second upsert (insert): { id: 'new-user-id' }

=== Find One Operation ===
✓ Found user by email: {
  id: 'ya-cDEFl57qAIZxMB9s_eSMNmo0GYUu_KnU3iYteWQoJXz-JXrRziQa0Nz-4iScBYFRpCRq0wo1jLsLIxceEhA',
  age: { type: 'Integer', value: 29 },
  active: { type: 'Boolean', value: true },
  name: { type: 'String', value: 'Alice Johnson' },
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
✓ Inserted document: OMZQDI7BqCnbXeRd48A95FKJ0AHZd3tBWepZJ6SuSHgPHufib4CTKd5VhDc2JUwkDlVAX7-yDCiycr4deboxqw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 4xSDj3W6_Kbl8vdZsHp6HCxW1_BbdWLm3pO3sICl966rJIsVaz1ujOoVhYkqSKNK2ZuRpxWr7Cp2Bd5GkRGmSg

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
✓ Edge cache script created: pBy_KOUqwTTI5i22zCR3sqfNB8BMvxP0uMEzD8f2VPyD5689GvOD8pURh7r7fHmCHyqnJYUkgN7IARcCKwP2Ow

Call 1: Cache miss (fetches from API)
Response time: 66ms
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
          "email": "Sincere@april.biz",
          "id": 1,
          "website": "hildegard.org",
          "phone": "1-770-736-8031 x56442",
          "address": {
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "street": "Kulas Light",
            "suite": "Apt. 556",
            "city": "Gwenborough",
            "zipcode": "92998-3874"
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
Response time: 1ms (66x faster!)
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
          "email": "Sincere@april.biz",
          "id": 1,
          "website": "hildegard.org",
          "phone": "1-770-736-8031 x56442",
          "address": {
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "street": "Kulas Light",
            "suite": "Apt. 556",
            "city": "Gwenborough",
            "zipcode": "92998-3874"
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
   Department: {"value":"engineering","type":"String"}

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
      "phone": "1-770-736-8031 x56442",
      "email": "Sincere@april.biz",
      "username": "Bret",
      "company": {
        "name": "Romaguera-Crona",
        "catch...

Second call (cache hit - from cache):
   ⏱️  Duration: 1ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "phone": "1-770-736-8031 x56442",
      "email": "Sincere@april.biz",
      "username": "Bret",
      "company": {
        "name": "Romaguera-...
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
🚀 ekoDB Functions Example (TypeScript)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: p1LYynbxigwk2I1vwoN0brvKfK-GQF2PRxZybVRdQsQYCmpjaD_TACzAT3Zeil3hIeGIXkzHj3r77phoUu5boQ
📊 Found 5 active users

📝 Example 2: Parameterized Function

✅ Function saved: iL2pjppkc_sSphS22J3--BJ07yLNmrYXkIMV_pCOa-7ZtiPTDqaVjJJOyjlu5jxnHhXJ8NxYH7VSNtEOP9cNfg
📊 Found 3 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: mdJP0Lj6xPFaIKuPxB2p34Zr7brkM-4shP6EfSt6wkXhc29Z8HbO9nQdANRcEFXnZRR6VXb9-WXtn7aExjWCTA
📊 Statistics: 2 groups
   {"status":{"value":"inactive","type":"String"},"avg_score":{"value":50,"type":"Float"},"count":{"type":"Integer","value":5}}
   {"count":{"type":"Integer","value":5},"status":{"type":"String","value":"active"},"avg_score":{"type":"Float","value":60}}

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
   {"avg_price":{"type":"Float","value":365.6666666666667},"category":{"type":"String","value":"Furniture"},"count":{"type":"Integer","value":3}}
   {"count":{"value":5,"type":"Integer"},"avg_price":{"value":367,"type":"Float"},"category":{"type":"String","value":"Electronics"}}
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

1. **Efficient Similarity Searches**: They excel at high-dimensional similarity searches, enabling fast retrieval of similar items based on vector representations.

2. **Scalability**: Designed to handle large volumes of vector data, scaling efficiently as data size increases.

3. **Real-Time Performance**: Support for real-time querying, making them suitable for applications like recommendation systems and personalization.

4. **Multimodal Data Handling**: Capable of processing various data types (text, images, audio) by converting them into vector formats.

5. **Advanced Machine Learning Integration**: Seamless integration with machine learning models to leverage embeddings for enhanced data analysis.

6. **Dimensionality Reduction**: Techniques like PCA or t-SNE can be easily applied to manage high-dimensional data.

7. **Built-In Indexing**: Specialized indexing methods (e.g., HNSW, Annoy) for efficient nearest-neighbor searches.

8. **Data Analysis and Insights**: Facilitates complex analytics and can uncover patterns through clustering and classifications based on vector similarities.

These features make vector databases especially advantageous for AI-driven applications and data science projects.
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

✅ Function saved: nK-XJgiDod29kH3KD7oCuOAFI6XwO_JFavYf4qkWc2GETY_3yJCn1mfXZPdiyza6CjcyY7QdtbWSNkeRUu7tIQ
📊 Found 2 product groups
   {"avg_price":{"type":"Float","value":575.6666666666666},"count":{"type":"Integer","value":3},"category":{"type":"String","value":"Electronics"}}
   {"avg_price":{"type":"Float","value":474},"count":{"value":2,"type":"Integer"},"category":{"type":"String","value":"Furniture"}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"count":{"value":2,"type":"Integer"},"category":{"value":"Furniture","type":"String"}}
   {"count":{"type":"Integer","value":3},"category":{"value":"Electronics","type":"String"}}
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
   {"avg_score":{"type":"Float","value":70},"count":{"value":7,"type":"Integer"},"role":{"type":"String","value":"user"}}
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

✅ Inserted order: PgkU4Q5ggcXpSsTzSmYfQNCx8P9g3ZvmuTEMXvd6ZxJ8a78x81KwjmZIyNvPejOU2G8GeJG1yWg4eKORiwLfaQ
✅ Inserted 2 products with wrapped types

📝 Example 2: UserFunction with Wrapped Type Parameters

✅ Function saved: VGh8CGVcC2br8xsczjckQwALtLDA9Ggzx4065wzY0JfCZqFdsIumS1K9H8zzJjc8q9YRmSP_kn403nvKL6ISkA
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"type":"Object","value":{"role":"admin","userId":"user_abc"}}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: YyC__lOsXYkfDiR_bKsTCNXVbogmFg2z4WmILb48Cg8UyPeqrSkrsIMeh0AWHRY2ZHyDLrM1koXLZI6NwyXysw
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: IEcjs6zFNiaF5dz4_pk_OJDMUoFw2AMfUBop8ce8LlOgE18P1aazrPGiae0JMEuPEnkQD2jhQEOCOEnzBhWsQw
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
   2. Getting Started with ekoDB (Database)
   3. Natural Language Processing (AI)
   4. Database Design Principles (Database)
   5. Vector Databases Explained (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"category":{"type":"String","value":"Database"},"count":{"value":3,"type":"Integer"}}
   {"category":{"value":"AI","type":"String"},"count":{"value":2,"type":"Integer"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Goal Template CRUD Example (TypeScript) ===

--- Creating goal template ---
Created template: Data Migration (id: 7j4fUJOogQZVXROb0TsLhWLEPT1qk6mDsRarGoJ3bHb420OKycbR7ztCjV1l__ovA3QNFf7Zmj0A-Ut-V8oAUw)

--- Listing templates ---
Templates: {
  count: 1,
  items: [
    {
      description: [Object],
      id: '7j4fUJOogQZVXROb0TsLhWLEPT1qk6mDsRarGoJ3bHb420OKycbR7ztCjV1l__ovA3QNFf7Zmj0A-Ut-V8oAUw',
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
Created goal: Deploy v2.0 (id: JZfz8m68k-8Bx4qx5t7KMHoIWuIOPlOxwq4y-JvtZhucJFZ5oDPOLnKQvI1Kk1lv3GRecXYZJkDju70ufF2Faw)

--- Listing goals ---
Goals: {
  "count": 1,
  "goals": [
    {
      "created_at": "2026-10-20T20:05:14.526432+00:00",
      "description": "Ship version 2.0 to production",
      "id": "JZfz8m68k-8Bx4qx5t7KMHoIWuIOPlOxwq4y-JvtZhucJFZ5oDPOLnKQvI1Kk1lv3GRecXYZJkDju70ufF2Faw",
      "status": "pending",
      "steps": "[{\"description\":\"Run test suite\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]",
      "title": "Deploy v2.0",
      "updated_at": "2026-10-20T20:05:14.526432+00:00"
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
        "value": "2026-10-20T20:05:14.526432+00:00"
      },
      "description": {
        "type": "String",
        "value": "Ship version 2.0 with hot-fix patches"
      },
      "id": "JZfz8m68k-8Bx4qx5t7KMHoIWuIOPlOxwq4y-JvtZhucJFZ5oDPOLnKQvI1Kk1lv3GRecXYZJkDju70ufF2Faw",
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
        "value": "2026-10-20T20:05:14.536369+00:00"
      }
    }
  ]
}

--- Goal step: start step 0 ---
Step 0 started on goal JZfz8m68k-8Bx4qx5t7KMHoIWuIOPlOxwq4y-JvtZhucJFZ5oDPOLnKQvI1Kk1lv3GRecXYZJkDju70ufF2Faw
--- Goal step: complete step 0 ---
Step 0 completed on goal JZfz8m68k-8Bx4qx5t7KMHoIWuIOPlOxwq4y-JvtZhucJFZ5oDPOLnKQvI1Kk1lv3GRecXYZJkDju70ufF2Faw
--- Goal step: fail step 1 ---
Step 1 failed on goal JZfz8m68k-8Bx4qx5t7KMHoIWuIOPlOxwq4y-JvtZhucJFZ5oDPOLnKQvI1Kk1lv3GRecXYZJkDju70ufF2Faw

--- Completing goal ---
Goal status: pending_review
--- Approving goal ---
Goal status after approve: in_progress

--- Creating goal to reject ---
--- Rejecting goal ---
Goal status after reject: failed


--- Creating task ---
Created task: Hourly Health Check (id: kZAbOEigj9W7ny6DuGa2suFqg9t5NZjZ2ROdU0bkdMwSWpHgDDNiOVR5XmnTUvw-jmsIt-cylw4wN_dyKKeOjA)

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
      "id": "kZAbOEigj9W7ny6DuGa2suFqg9t5NZjZ2ROdU0bkdMwSWpHgDDNiOVR5XmnTUvw-jmsIt-cylw4wN_dyKKeOjA",
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
Created agent: SupportBot (id: fxZlidia5jCZ_bg5Pc5PNjEGH3vCV9A0791-pF4AClVyO9lNGfevHlYfufctdGoogBOaimwHHamZ6_Oda7L_hQ)

--- Listing agents ---
Agents: {
  "count": 1,
  "items": [
    {
      "deployment_id": {
        "type": "String",
        "value": "deploy_prod_1"
      },
      "id": "fxZlidia5jCZ_bg5Pc5PNjEGH3vCV9A0791-pF4AClVyO9lNGfevHlYfufctdGoogBOaimwHHamZ6_Oda7L_hQ",
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
By name: SupportBot (id: fxZlidia5jCZ_bg5Pc5PNjEGH3vCV9A0791-pF4AClVyO9lNGfevHlYfufctdGoogBOaimwHHamZ6_Oda7L_hQ)

--- Updating agent ---
Updated agent: SupportBot

--- Getting agents by deployment ---
Agents in deployment: {
  "count": 0,
  "items": []
}
WARNING: agents-by-deployment omitted created agent fxZlidia5jCZ_bg5Pc5PNjEGH3vCV9A0791-pF4AClVyO9lNGfevHlYfufctdGoogBOaimwHHamZ6_Oda7L_hQ; TODO: check/fix the server-side deployment lookup

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
  POST /api/functions/jwt_register_ts_27477_1792526715644 { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/jwt_login_ts_27477_1792526715644 { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/jwt_verify_ts_27477_1792526715644 { "token": "<jwt>" }

Set JWT_SECRET in ekoDB's environment_vars whitelist before invoking.

✓ Cleaned up demo functions
=== ekoDB KV Document Linking Example (TypeScript) ===

--- Setup: creating KV key and documents ---
Set KV key: session:admin
Inserted document 1: w-p6nky54MLX7LPjzECR566xlbkv3rt_SKFS3nYD0YExS_mDqKLk7QKbpQdtvwlNRwlKz7iTMZ4DMVqIN6cIpg
Inserted document 2: E8ZCRxcula1VT0kPL65443zsBc-hEvSLqwC0890Ki8rZxeNCpc413x0_Yn-0cmBcg-bsrlrTwaX79hzOpNTm8A

--- Linking documents to KV key ---
Linked doc w-p6nky54MLX7LPjzECR566xlbkv3rt_SKFS3nYD0YExS_mDqKLk7QKbpQdtvwlNRwlKz7iTMZ4DMVqIN6cIpg: null
Linked doc E8ZCRxcula1VT0kPL65443zsBc-hEvSLqwC0890Ki8rZxeNCpc413x0_Yn-0cmBcg-bsrlrTwaX79hzOpNTm8A: null

--- Getting links for KV key ---
Links: [
  {
    "collection": "kv_links_example_ts_28130_1792526716109",
    "document_id": "w-p6nky54MLX7LPjzECR566xlbkv3rt_SKFS3nYD0YExS_mDqKLk7QKbpQdtvwlNRwlKz7iTMZ4DMVqIN6cIpg",
    "field_path": null,
    "created_at": "2026-10-20T20:05:16.194817Z",
    "last_accessed": "2026-10-20T20:05:16.199492Z",
    "metadata": {}
  },
  {
    "collection": "kv_links_example_ts_28130_1792526716109",
    "document_id": "E8ZCRxcula1VT0kPL65443zsBc-hEvSLqwC0890Ki8rZxeNCpc413x0_Yn-0cmBcg-bsrlrTwaX79hzOpNTm8A",
    "field_path": null,
    "created_at": "2026-10-20T20:05:16.197684Z",
    "last_accessed": "2026-10-20T20:05:16.199492Z",
    "metadata": {}
  }
]

--- Unlinking document ---
Unlinked doc E8ZCRxcula1VT0kPL65443zsBc-hEvSLqwC0890Ki8rZxeNCpc413x0_Yn-0cmBcg-bsrlrTwaX79hzOpNTm8A: null

--- Verifying remaining links ---
Remaining links: [
  {
    "collection": "kv_links_example_ts_28130_1792526716109",
    "document_id": "w-p6nky54MLX7LPjzECR566xlbkv3rt_SKFS3nYD0YExS_mDqKLk7QKbpQdtvwlNRwlKz7iTMZ4DMVqIN6cIpg",
    "field_path": null,
    "created_at": "2026-10-20T20:05:16.194817Z",
    "last_accessed": "2026-10-20T20:05:16.204739Z",
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
  kv_ops_ts_28771_1792526716637:cache:product:1: success
  kv_ops_ts_28771_1792526716637:cache:product:2: success
  kv_ops_ts_28771_1792526716637:cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  kv_ops_ts_28771_1792526716637:cache:product:1: { name: 'Product 1', price: 29.99 }
  kv_ops_ts_28771_1792526716637:cache:product:2: { name: 'Product 2', price: 39.99 }
  kv_ops_ts_28771_1792526716637:cache:product:3: { name: 'Product 3', price: 49.99 }

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
  kv_ops_ts_28771_1792526716637:cache:product:1: deleted
  kv_ops_ts_28771_1792526716637:cache:product:2: deleted
  kv_ops_ts_28771_1792526716637:cache:product:3: deleted

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
    "name": "Mars",
    "diameter_km": 6779
  }
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello! I hope you're having a wonderful day.

=== Done ===
=== ekoDB Schedule Management Example (TypeScript) ===

--- Creating schedule ---
Created schedule: Nightly Database Backup (id: d572e6fd-bc28-4b5e-9f43-f43f10a68ada, cron: 0 0 2 * * *)

--- Listing schedules ---
Schedules: {
  "count": 1,
  "schedules": [
    {
      "created_at": "2026-10-20T20:05:20.925150Z",
      "cron_expression": "0 0 2 * * *",
      "description": null,
      "enabled": true,
      "function_label": "schedule_noop_typescript_33914_1792526720895",
      "id": "d572e6fd-bc28-4b5e-9f43-f43f10a68ada",
      "last_execution": null,
      "name": "Nightly Database Backup",
      "next_execution": "2026-10-21T02:00:00Z",
      "parameters": {},
      "stats": {
        "avg_execution_time_ms": 0,
        "failed_executions": 0,
        "last_error": null,
        "successful_executions": 0,
        "total_executions": 0
      },
      "timezone": "UTC",
      "updated_at": "2026-10-20T20:05:20.925150Z"
    }
  ]
}

--- Getting schedule ---
Fetched: Nightly Database Backup (cron: 0 0 2 * * *)

--- Updating schedule ---
Updated: Nightly Full Backup (new cron: 0 0 3 * * *)

--- Triggering schedule ---
Trigger response: {
  "schedule_id": "d572e6fd-bc28-4b5e-9f43-f43f10a68ada",
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
  3. Score: 13.200, Matched: bio, title
  4. Score: 13.200, Matched: title, bio

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
  1. Score: 0.765, Matched:
  2. Score: 0.747, Matched:
  3. Score: 0.735, Matched:

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.494, Matched: content, title
  2. Score: 0.899, Matched: content, title
  3. Score: 0.306, Matched:

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
  id: 'mTcEmNiwyazoLzlYQlJET1o96BlvEA-sUK7pn-q9PKZaV8zBWk4Fd_AUi5xv79RKl500sz2gAPPg7IJvEHlk8Q'
}

=== Find by ID ===
Found: {
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  value: { type: 'Integer', value: 42 },
  price: { value: 99.99, type: 'Float' },
  name: { type: 'String', value: 'Test Record' },
  metadata: { type: 'Object', value: { key: 'value', nested: [Object] } },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  active: { value: true, type: 'Boolean' },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  created_at: { type: 'DateTime', value: '2026-10-20T20:05:22.353+00:00' },
  id: 'mTcEmNiwyazoLzlYQlJET1o96BlvEA-sUK7pn-q9PKZaV8zBWk4Fd_AUi5xv79RKl500sz2gAPPg7IJvEHlk8Q'
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-10-20T20:05:22.353Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { key: 'value', nested: { deep: true } }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  categories: [ 'electronics', 'computers' ],
  tags: [ 'tag1', 'tag2', 'tag3' ],
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  value: 42,
  price: 99.99,
  name: 'Test Record',
  metadata: { key: 'value', nested: { deep: true } },
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  active: true,
  data: 'aGVsbG8gd29ybGQ=',
  created_at: '2026-10-20T20:05:22.353+00:00',
  id: 'mTcEmNiwyazoLzlYQlJET1o96BlvEA-sUK7pn-q9PKZaV8zBWk4Fd_AUi5xv79RKl500sz2gAPPg7IJvEHlk8Q'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  value: { type: 'Integer', value: 100 },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  price: { value: 99.99, type: 'Float' },
  name: { type: 'String', value: 'Updated Record' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  created_at: { value: '2026-10-20T20:05:22.353+00:00', type: 'DateTime' },
  metadata: { type: 'Object', value: { key: 'value', nested: [Object] } },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  active: { value: true, type: 'Boolean' },
  id: 'mTcEmNiwyazoLzlYQlJET1o96BlvEA-sUK7pn-q9PKZaV8zBWk4Fd_AUi5xv79RKl500sz2gAPPg7IJvEHlk8Q'
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: a5fpQ5T9ZN8aJ4T65DKD3ZooqoTw3ArNcpX6vSBlgZjner5IQOO8dvujBuZyFBvMOnKCPgtxeyRmxUERXTpn1g

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
✓ Created native SWR script: github_user_native_ts (fiyiDKmnZVylD461jBPJ8P-1ZyHlb9K4K9mCrS8OHdlnTt43A4ok7Y4hm2ZvYS4iwEnKIQ_hgOZo_KOYhNMK3Q)

First call (cache miss - will fetch from GitHub API):
  Response time: 299ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 8ms
  Speedup: 37.4x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_ts (t0_PVwB_sKINvgPBo4cM_9pri6eWcG8_jj56_tCXsBGzZ9ANvvb7sxGbO6RQ3ctzRShXc5CwdoaTvY3uCaK6eA)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_ts' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_ts (HIZDiqda1iZJ02IOEgP771NR8wLUy_g4kL75P6rKmJH0a7rrwQcukBUhfrn8XEQ5FeAH1Wx2Yz0YuWsq3P3mWQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_ts' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_ts (VV4-ZFTXZAGGEKSpFsbLJhm-oUzFHpkcIqx-R4PY-pQ5mD5i0BPQGnFcHhvDHZsoAQNrfHbKIhpzE6kIFygFIg)
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
✓ Created SWR script: fetch_api_user_ts_38404_1792526724530 (kVEqjuDd7VdnBqwJg2ZGoA0J-teQrLcooxUZUSXdxWyvFySYo5YmEyy69yGgeiYdBlJjRdD8DgQ99U2HXi0H7g)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "phone": "1-770-736-8031 x56442",
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "bs": "harness real-time e-markets",
            "name": "Romaguera-Crona"
          },
          "id": 1,
          "email": "Sincere@april.biz",
          "website": "hildegard.org",
          "name": "Leanne Graham",
          "username": "Bret",
          "address": {
            "street": "Kulas Light",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "city": "Gwenborough",
            "zipcode": "92998-3874",
            "suite": "Apt. 556"
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
Response time: 1ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "value": {
          "phone": "1-770-736-8031 x56442",
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "bs": "harness real-time e-markets",
            "name": "Romaguera-Crona"
          },
          "id": 1,
          "email": "Sincere@april.biz",
          "website": "hildegard.org",
          "name": "Leanne Graham",
          "username": "Bret",
          "address": {
            "street": "Kulas Light",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "city": "Gwenborough",
            "zipcode": "92998-3874",
            "suite": "Apt. 556"
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
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_reviews_ts_38404_1792526724530 (Y_KAfXG_WAldViy4NpOMS67xvuVVIXzXa6eRB-drCwtpgVZkHo8zGJkF0z4km5COImEMCDZlPn_9MTV4oQXrFw)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "value": {
          "sku": "BEA-ESS-ESS-001",
          "title": "Essence Mascara Lash Princess",
          "weight": 4,
          "minimumOrderQuantity": 48,
          "discountPercentage": 10.48,
          "availabilityStatus": "In Stock",
          "meta": {
            "createdAt": "2025-10-09T14:47:01.588Z",
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
            "barcode": "5784719087687",
            "updatedAt": "2026-05-23T11:27:41.868Z"
          },
          "rating": 2.56,
          "reviews": [
            {
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "reviewerName": "Eleanor Collins",
              "date": "2025-04-30T09:41:02.053Z",
              "comment": "Would not recommend!",
              "rating": 3
            },
            {
              "reviewerEmail": "lucas.gordon@x.dummyjson.com",
              "comment": "Very satisfied!",
              "reviewerName": "Lucas Gordon",
              "date": "2025-04-30T09:41:02.053Z",
              "rating": 4
            },
            {
              "comment": "Highly impressed!",
              "rating": 5,
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerName": "Eleanor Collins"
            }
          ],
          "warrantyInformation": "1 week warranty",
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "category": "beauty",
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "tags": [
            "beauty",
            "mascara"
          ],
          "price": 9.99,
          "returnPolicy": "No return policy",
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "id": 1,
          "shippingInformation": "Ships in 3-5 business days",
          "dimensions": {
            "width": 15.14,
            "depth": 22.99,
            "height": 13.08
          },
          "brand": "Essence",
          "stock": 99
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
Created Alice: $1000 - ID: lIzPCAYvvSOARVuKe9U-_2VAa3u5RhdDDmTOUp2YV74zZcjqWlOtWj_hxgT_UTuU8FvzMgDfZuVp7Hj26V-KkQ
Created Bob: $500 - ID: Qy_F97-5pLSWilJPzKYOcWYeovr2fFBrUp9XNKW0iBJVNbwQ82hVC6t8xrjx4L1MWM0LTMt_YyJ5I2MzyzVXdw

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 39177b08-c39a-4c28-8453-7874830c9ee5

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
New transaction: 9ff11377-ab76-485b-81db-a39da6354fe6
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Create User Function ===
Created user function with ID: l6mm4a6QAdL8dEIoII7qrX0u_xab8mYiniT21f9DHoB7kP3VP4MT8IXmBK4YIxRI9VEpP3ID-VgdzJh6tw1GxA

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 5 user functions:
  - conc_demo_rl_skip_ts_12240_1792526703280: Rate-limit (skip mode)
  - conc_demo_rl_fail_ts_12240_1792526703280: Rate-limit (fail mode)
  - fetch_product_reviews_ts_38404_1792526724530: Fetch Product with Reviews (Multi-API)
  - get_active_users_ts: Get Active Users
  - get_active_users_client_ts_updated: Get Active Users (Updated)

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
=== WebSocket Chat Streaming Example (TypeScript) ===

Created chat session: FbM3o0MAL3UUPt1faS5KgZC4hNb6aQ90kK_j5dqMn5GTr44d_kPu4Nb1hvS-pn87hyAl7QeyQIwAtXeKHmz5Kw

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: T99qeabXYwU6p7-nLuJa-ZcyqsgnlfmLj710_xfGvDtAx5BzkaCJOdHk6wfLpRW9BlkdXUdRQ_6W9EBvXpL9Nw
Execution time: 811ms
Token usage: {"completion_tokens":8,"prompt_tokens":15,"total_tokens":23}

Full response: {"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris....
=== WebSocket Subscription Example ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_ts' ===
✓ Subscribed (subscription_id: sub_e416ef6fc3a1458586c1ddb969a73bcd)

=== Performing mutations to trigger notifications ===
Inserting a record...
✓ Inserted record: zov7PEHGPoGNoJz3K0k0v-lvm38dC0W3nvs7e3L2gZEGk40wHesR74KU9qbtTR7ie6sBmTsZiKftg72bGKsJPw
  📡 Notification received for zov7PEHGPoGNoJz3K0k0v-lvm38dC0W3nvs7e3L2gZEGk40wHesR74KU9qbtTR7ie6sBmTsZiKftg72bGKsJPw

Inserting another record...
✓ Inserted record: 3TjUDoHG9dVuerDn_4_26KfHYRm2MUXPlfT8Y7RXZnz3TUrCwDtCnXeTyL36eqUDcr-Rg8TSDtdjhgitjygi2Q
  📡 Notification received for 3TjUDoHG9dVuerDn_4_26KfHYRm2MUXPlfT8Y7RXZnz3TUrCwDtCnXeTyL36eqUDcr-Rg8TSDtdjhgitjygi2Q

=== Unsubscribing ===
✓ Unsubscribed: {"collection":"ws_subscribe_example_ts","found":true,"unsubscribed":true}

✓ WebSocket subscription example completed successfully
✓ Deleted collection 'ws_subscribe_example_ts'
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: IQIoEVmyrNNCbnvjacWViUJYpt2tBcBj3sJ4Wx2UIATbOxVE5cjKspTzbkS-EPk_BhBG--FmmhEnJsGwdM4xWQ

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
   Inserted with ripple: {"id":"khR4VQ2Ju1xgdEwS5qvOoJ0G-KfaebaEyHf0r3fafoIR-KVqg8thJC-yxOCysQaDlCqqTaGW0pUszj2tI9Iu6w"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"PqXR7Fjjy5UG2GZfo3v2xsI32VQdnrq3mr_kjUYg0xum2AJPRkH2MP4QDtv-8HY9SfPztdoDmk-gNXpmf9SplQ"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"id":"khR4VQ2Ju1xgdEwS5qvOoJ0G-KfaebaEyHf0r3fafoIR-KVqg8thJC-yxOCysQaDlCqqTaGW0pUszj2tI9Iu6w","price":{"value":150,"type":"Integer"},"name":{"type":"String","value":"Product 1"}}

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
  Fields returned: ["name","email","id"]
  First user: Bob Smith <bob@example.com>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["created_at","user_role","bio","avatar_url","email","age","id","name","status"]

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
    - Fields: ["age","email","password","name","id","avatar_url","status","created_at","secret_token","bio","api_key","user_role"]
  Projected query:
    - 3 fields per record
    - Fields: ["id","name","email"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
